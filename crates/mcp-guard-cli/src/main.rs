// Copyright (c) 2025 Austin Green
// SPDX-License-Identifier: AGPL-3.0
//
// This file is part of MCP-Guard.
//
// MCP-Guard is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// MCP-Guard is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with MCP-Guard. If not, see <https://www.gnu.org/licenses/>.
//! MCP Guard - Security gateway for MCP servers

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use mcp_guard_core::{
    audit::{AuditLogger, AuditLoggerHandle},
    auth::{
        ApiKeyProvider, AuthProvider, DatabaseAuthProvider, JwtProvider, MtlsAuthProvider,
        MultiProvider, OAuthAuthProvider,
    },
    cli::{
        apply_key_to_config, apply_policy_suggestion, generate_api_key,
        generate_config_with_demo_key, hash_api_key, Cli, Commands, PolicyAction,
    },
    clients::{
        self, ClientKind, DiscoveryEnv, LoadedClientConfig, Severity, UnwrapAction, WrapAction,
        WrapOptions,
    },
    config::{Config, TransportType},
    mcp_server::{fetch_upstream_tools, McpServer, McpServerConfig},
    observability::{init_metrics, init_tracing},
    pin::ToolLockfile,
    policy::{self, Decision},
    rate_limit::RateLimitService,
    recorder::{self, Recorder, Session, SessionInfo},
    router::ServerRouter,
    server::{self, new_oauth_state_store, AppState},
    transport::{HttpTransport, SseTransport, StdioTransport, Transport},
};

/// Result of bootstrapping the server state.
/// Contains all handles needed for graceful shutdown.
pub struct BootstrapResult {
    pub state: Arc<AppState>,
    pub audit_handle: AuditLoggerHandle,
    pub shutdown_token: CancellationToken,
}

/// Bootstrap the server state from configuration.
/// This extracts all initialization logic to make it testable.
pub async fn bootstrap(config: Config) -> anyhow::Result<BootstrapResult> {
    // Create shutdown token for graceful shutdown coordination
    let shutdown_token = CancellationToken::new();

    // Initialize Prometheus metrics
    let metrics_handle = init_metrics();

    // Set up database connection
    let db = if let Some(url) = &config.database_url {
        tracing::info!("Initializing database connection");
        Some(
            mcp_guard_core::db::Database::new(url)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?,
        )
    } else {
        None
    };

    // Set up OAuth provider (separate from MultiProvider for auth code flow)
    let oauth_provider: Option<Arc<OAuthAuthProvider>> =
        if let Some(oauth_config) = config.auth.oauth.clone() {
            tracing::info!(
                "Enabling OAuth 2.1 authentication (provider: {:?})",
                oauth_config.provider
            );
            Some(Arc::new(OAuthAuthProvider::new(oauth_config).map_err(
                |e| anyhow::anyhow!("Failed to initialize OAuth provider: {}", e),
            )?))
        } else {
            None
        };

    // Set up authentication provider(s)
    let (auth_provider, jwt_provider_arc): (Arc<dyn AuthProvider>, Option<Arc<JwtProvider>>) = {
        let mut providers: Vec<Arc<dyn AuthProvider>> = Vec::new();
        let mut jwt_provider_arc = None;

        // Add API key provider if configured
        if !config.auth.api_keys.is_empty() {
            tracing::info!(
                "Enabling API key authentication ({} keys)",
                config.auth.api_keys.len()
            );
            providers.push(Arc::new(ApiKeyProvider::new(config.auth.api_keys.clone())));
        }

        // Add Database provider if configured
        if let Some(database) = &db {
            tracing::info!("Enabling database authentication");
            providers.push(Arc::new(DatabaseAuthProvider::new(database.api_keys())));
        }

        // Add JWT provider if configured
        if let Some(jwt_config) = &config.auth.jwt {
            tracing::info!("Enabling JWT authentication");
            let jwt_provider = Arc::new(
                JwtProvider::new(jwt_config.clone())
                    .map_err(|e| anyhow::anyhow!("Failed to initialize JWT provider: {}", e))?,
            );
            // Start background refresh for JWKS mode with shutdown coordination
            jwt_provider.start_background_refresh(shutdown_token.clone());
            providers.push(jwt_provider.clone());
            jwt_provider_arc = Some(jwt_provider);
        }

        // Add OAuth provider for token validation (shares with oauth_provider)
        if let Some(ref oauth_prov) = oauth_provider {
            providers.push(oauth_prov.clone());
        }

        // Select appropriate provider setup
        if providers.is_empty() {
            tracing::warn!(
                "No authentication providers configured - all requests will be rejected"
            );
            (Arc::new(ApiKeyProvider::new(vec![])), jwt_provider_arc) // Deny all
        } else if providers.len() == 1 {
            // Safe: we just checked length is exactly 1
            let provider = providers
                .into_iter()
                .next()
                .unwrap_or_else(|| Arc::new(ApiKeyProvider::new(vec![])));
            (provider, jwt_provider_arc)
        } else {
            tracing::info!("Using multi-provider authentication");
            (Arc::new(MultiProvider::new(providers)), jwt_provider_arc)
        }
    };

    // Create OAuth state store for PKCE
    let oauth_state_store = new_oauth_state_store();

    // Set up mTLS provider if configured
    let mtls_provider: Option<Arc<MtlsAuthProvider>> =
        if let Some(mtls_config) = config.auth.mtls.clone() {
            if mtls_config.enabled {
                tracing::info!("Enabling mTLS client certificate authentication");
                Some(Arc::new(MtlsAuthProvider::new(mtls_config)))
            } else {
                None
            }
        } else {
            None
        };

    // Set up rate limiter
    let rate_limiter = RateLimitService::new(&config.rate_limit);

    // Set up audit logger with background tasks for non-blocking I/O
    let (audit_logger, audit_handle) = AuditLogger::with_tasks(&config.audit)?;
    let audit_logger = Arc::new(audit_logger);

    // Set up transport/router based on configuration
    let (transport, router): (Option<Arc<dyn Transport>>, Option<Arc<ServerRouter>>) =
        if config.is_multi_server() {
            // Multi-server routing mode
            tracing::info!(
                routes = config.upstream.servers.len(),
                "Initializing multi-server routing"
            );
            for server in &config.upstream.servers {
                tracing::info!(
                    name = %server.name,
                    path_prefix = %server.path_prefix,
                    transport = ?server.transport,
                    "Configuring server route"
                );
            }
            let server_router = ServerRouter::new(config.upstream.servers.clone())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to initialize router: {}", e))?;
            (None, Some(Arc::new(server_router)))
        } else {
            // Single-server mode
            let transport: Arc<dyn Transport> = match &config.upstream.transport {
                mcp_guard_core::config::TransportType::Stdio => {
                    let command = config.upstream.command.as_ref().ok_or_else(|| {
                        anyhow::anyhow!("stdio transport requires 'command' in config")
                    })?;
                    tracing::info!(command = %command, "Using stdio transport");
                    Arc::new(StdioTransport::spawn(command, &config.upstream.args).await?)
                }
                mcp_guard_core::config::TransportType::Http => {
                    let url = config
                        .upstream
                        .url
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("HTTP transport requires 'url' in config"))?
                        .clone();
                    tracing::info!(url = %url, "Using HTTP transport");
                    #[cfg(test)]
                    let transport = HttpTransport::new_unchecked(url);
                    #[cfg(not(test))]
                    let transport = HttpTransport::new(url)
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to create HTTP transport: {}", e))?;
                    Arc::new(transport)
                }
                mcp_guard_core::config::TransportType::Sse => {
                    let url = config
                        .upstream
                        .url
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("SSE transport requires 'url' in config"))?
                        .clone();
                    tracing::info!(url = %url, "Using SSE transport");
                    #[cfg(test)]
                    let transport = SseTransport::connect_unchecked(url).await?;
                    #[cfg(not(test))]
                    let transport = SseTransport::connect(url).await?;
                    Arc::new(transport)
                }
            };
            (Some(transport), None)
        };

    // Create readiness state (set to true since transport is initialized)
    let ready = Arc::new(RwLock::new(true));

    // Create application state
    let state = Arc::new(AppState {
        config,
        auth_provider,
        rate_limiter,
        audit_logger,
        transport,
        router,
        metrics_handle,
        oauth_provider,
        oauth_state_store,
        started_at: Instant::now(),
        ready,
        mtls_provider,
        jwt_provider: jwt_provider_arc,
        db: db.clone(),
    });

    Ok(BootstrapResult {
        state,
        audit_handle,
        shutdown_token,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let cli = Cli::parse_args();
    if let Err(e) = run_cli(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

async fn run_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Init { format, force } => handle_init(&format, force, cli.verbose),
        Commands::Validate => handle_validate(&cli.config, cli.verbose),
        Commands::Keygen {
            user_id,
            rate_limit,
            tools,
            apply_to_config,
        } => handle_keygen(
            &cli.config,
            &user_id,
            rate_limit,
            tools.as_deref(),
            apply_to_config,
            cli.verbose,
        ),
        Commands::HashKey { key } => handle_hash_key(&key),
        Commands::Version => handle_version(),
        Commands::CheckUpstream { timeout } => {
            handle_check_upstream(&cli.config, timeout, cli.verbose).await
        }
        Commands::Run { host, port } => handle_run(&cli.config, host, port, cli.verbose).await,
        Commands::Serve => handle_serve(&cli.config, cli.verbose).await,
        Commands::Wrap {
            client,
            servers,
            allow,
            deny,
            no_pin,
            dry_run,
            config_dir,
            guard_bin,
        } => handle_wrap(
            &client, &servers, allow, deny, !no_pin, dry_run, config_dir, guard_bin,
        ),
        Commands::Unwrap {
            client,
            servers,
            dry_run,
        } => handle_unwrap(&client, &servers, dry_run),
        Commands::Doctor {
            clients,
            json,
            strict,
        } => handle_doctor(&clients, json, strict),
        Commands::Pin {
            accept,
            show,
            timeout,
        } => handle_pin(&cli.config, accept, show, timeout, cli.verbose).await,
        Commands::Replay {
            session,
            dir,
            list,
            denied,
            tool,
            json,
        } => handle_replay(&cli.config, &session, dir, list, denied, tool, json),
        Commands::Verify { session, dir, json } => handle_verify(&cli.config, &session, dir, json),
        Commands::Policy { action } => match action {
            PolicyAction::Test { tool, args } => handle_policy_test(&cli.config, &tool, &args),
            PolicyAction::Suggest { apply, audit_log } => {
                handle_policy_suggest(&cli.config, apply, audit_log)
            }
        },
    }
}

// ============================================================================
// CLI Command Handlers
// ============================================================================

/// Handle the `init` command: create a new configuration file with a demo API key.
fn handle_init(format: &str, force: bool, verbose: bool) -> anyhow::Result<()> {
    let _guard = init_tracing(verbose, None);

    let filename = if format == "yaml" {
        "mcp-guard.yaml"
    } else {
        "mcp-guard.toml"
    };

    let path = std::path::Path::new(filename);
    if path.exists() && !force {
        anyhow::bail!("{} already exists. Use --force to overwrite.", filename);
    }

    let (config, demo_key) = generate_config_with_demo_key(format);
    std::fs::write(filename, &config)?;

    println!("Created configuration file: {}", filename);
    println!();
    println!("Demo API key (for testing only - replace in production):");
    println!("  {}", demo_key);
    println!();
    println!("Test with:");
    println!("  mcp-guard run");
    println!(
        "  curl -H \"Authorization: Bearer {}\" http://localhost:3000/health",
        demo_key
    );
    println!();
    println!("Generate production keys with:");
    println!("  mcp-guard keygen --user-id <name>");

    Ok(())
}

/// Handle the `validate` command: validate a configuration file.
fn handle_validate(config_path: &std::path::PathBuf, verbose: bool) -> anyhow::Result<()> {
    let _guard = init_tracing(verbose, None);

    match Config::from_file(config_path) {
        Ok(_) => {
            println!("Configuration is valid: {}", config_path.display());
            Ok(())
        }
        Err(e) => {
            anyhow::bail!("Configuration error: {}", e);
        }
    }
}

/// Handle the `keygen` command: generate a new API key.
fn handle_keygen(
    config_path: &std::path::Path,
    user_id: &str,
    rate_limit: Option<u32>,
    tools: Option<&str>,
    apply_to_config: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let _guard = init_tracing(verbose, None);

    let key = generate_api_key();
    let hash = hash_api_key(&key);

    // Parse tools if provided
    let tool_list: Option<Vec<String>> =
        tools.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());

    if apply_to_config {
        // Check config file exists
        if !config_path.exists() {
            anyhow::bail!(
                "Config file not found: {}\n\
                 Run 'mcp-guard init' first, or specify --config <path>",
                config_path.display()
            );
        }

        // Apply key to config file
        apply_key_to_config(
            config_path,
            user_id,
            &hash,
            rate_limit,
            tool_list.as_deref(),
        )
        .map_err(|e| anyhow::anyhow!("Failed to apply key to config: {}", e))?;

        println!(
            "✓ API key for '{}' added to {}",
            user_id,
            config_path.display()
        );
        println!();
        println!("API Key (save this, shown only once):");
        println!("  {}", key);
        println!();
        println!("Next steps:");
        println!("  mcp-guard validate");
        println!("  mcp-guard run");
    } else {
        // Original behavior - print to stdout
        println!("Generated API key for '{}':", user_id);
        println!();
        println!("  API Key (save this, shown only once):");
        println!("    {}", key);
        println!();
        println!("  Add to your config file:");
        println!();
        println!("  [[auth.api_keys]]");
        println!("  id = \"{}\"", user_id);
        println!("  key_hash = \"{}\"", hash);

        if let Some(limit) = rate_limit {
            println!("  rate_limit = {}", limit);
        }

        if let Some(ref tools_list) = tool_list {
            println!("  allowed_tools = {:?}", tools_list);
        }
    }

    Ok(())
}

/// Handle the `hash-key` command: hash an existing API key.
fn handle_hash_key(key: &str) -> anyhow::Result<()> {
    let hash = hash_api_key(key);
    println!("{}", hash);
    Ok(())
}

/// Handle the `version` command: print version information.
fn handle_version() -> anyhow::Result<()> {
    use mcp_guard_core::tier;

    println!("mcp-guard {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Build Information:");
    println!("  Package:     {}", env!("CARGO_PKG_NAME"));
    println!("  Version:     {}", env!("CARGO_PKG_VERSION"));
    println!("  Tier:        {}", tier::current_tier());
    println!("  Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("  License:     {}", env!("CARGO_PKG_LICENSE"));
    println!("  Repository:  {}", env!("CARGO_PKG_REPOSITORY"));
    println!();

    // Show features based on current tier
    println!("Available Features:");

    // Free tier features (always available)
    println!("  [Free]");
    println!("    - API Key authentication");
    println!("    - JWT HS256 (simple mode)");
    println!("    - Stdio transport");
    println!("    - Global rate limiting");
    println!("    - File/console audit logging");
    println!("    - Prometheus metrics");

    // Pro tier features
    if tier::is_feature_available("oauth") {
        println!("  [Pro]");
        println!("    - OAuth 2.1 + PKCE authentication");
        println!("    - JWT JWKS mode (RS256/ES256)");
        println!("    - HTTP/SSE transports");
        println!("    - Per-identity rate limiting");
    } else {
        println!("  [Pro] (upgrade at https://mcpg.botzr.com/pricing)");
        println!("    - OAuth 2.1 + PKCE authentication");
        println!("    - JWT JWKS mode (RS256/ES256)");
        println!("    - HTTP/SSE transports");
    }

    // Enterprise tier features
    if tier::is_feature_available("mtls") {
        println!("  [Enterprise]");
        println!("    - mTLS client certificate authentication");
        println!("    - Multi-server routing");
        println!("    - SIEM audit log shipping");
        println!("    - OpenTelemetry tracing");
        println!("    - Per-tool rate limiting");
        println!("    - Admin guard tools");
    } else {
        println!("  [Enterprise] (upgrade at https://mcpg.botzr.com/pricing)");
        println!("    - mTLS client certificate authentication");
        println!("    - Multi-server routing");
        println!("    - SIEM audit log shipping");
        println!("    - OpenTelemetry tracing");
    }

    Ok(())
}

/// Print startup summary with configuration details
fn print_startup_summary(config: &Config) {
    let version = env!("CARGO_PKG_VERSION");

    // Header box
    println!();
    println!("╭──────────────────────────────────────────────╮");
    println!("│  MCP Guard v{:<33}│", version);
    println!("╰──────────────────────────────────────────────╯");
    println!();

    // Server address
    println!(
        "✓ Server:     http://{}:{}",
        config.server.host, config.server.port
    );

    // Auth providers
    let mut providers = Vec::new();
    if !config.auth.api_keys.is_empty() {
        providers.push(format!("API Keys ({})", config.auth.api_keys.len()));
    }
    if config.auth.jwt.is_some() {
        providers.push("JWT".to_string());
    }
    if config.auth.oauth.is_some() {
        providers.push("OAuth 2.1".to_string());
    }
    if config
        .auth
        .mtls
        .as_ref()
        .map(|m| m.enabled)
        .unwrap_or(false)
    {
        providers.push("mTLS".to_string());
    }

    if providers.is_empty() {
        println!("✗ Auth:       None (all requests will be rejected)");
    } else {
        println!("✓ Auth:       {}", providers.join(", "));
    }

    // Transport
    if config.is_multi_server() {
        println!(
            "✓ Transport:  Multi-server ({} routes)",
            config.upstream.servers.len()
        );
    } else {
        match &config.upstream.transport {
            TransportType::Stdio => {
                let cmd = config.upstream.command.as_deref().unwrap_or("?");
                println!("✓ Transport:  stdio → {}", cmd);
            }
            TransportType::Http => {
                let url = config.upstream.url.as_deref().unwrap_or("?");
                println!("✓ Transport:  HTTP → {}", url);
            }
            TransportType::Sse => {
                let url = config.upstream.url.as_deref().unwrap_or("?");
                println!("✓ Transport:  SSE → {}", url);
            }
        }
    }

    // Rate limiting
    if config.rate_limit.enabled {
        println!(
            "✓ Rate Limit: {} req/s, burst {}",
            config.rate_limit.requests_per_second, config.rate_limit.burst_size
        );
    } else {
        println!("✗ Rate Limit: Disabled");
    }

    // Audit
    if config.audit.enabled {
        println!("✓ Audit:      Enabled");
    } else {
        println!("✗ Audit:      Disabled");
    }

    println!();
    println!("Ready for requests!");
    println!();
    println!("Test with:");
    println!(
        "  curl http://{}:{}/health",
        config.server.host, config.server.port
    );
    println!();
}

/// Handle the `check-upstream` command: check upstream server connectivity.
async fn handle_check_upstream(
    config_path: &std::path::PathBuf,
    timeout: u64,
    verbose: bool,
) -> anyhow::Result<()> {
    let _guard = init_tracing(verbose, None);

    let config = Config::from_file(config_path)
        .map_err(|e| anyhow::anyhow!("Error loading config: {}", e))?;

    println!("Checking upstream connectivity...");
    println!();

    let timeout_duration = std::time::Duration::from_secs(timeout);

    match &config.upstream.transport {
        mcp_guard_core::config::TransportType::Stdio => {
            let command =
                config.upstream.command.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("stdio transport requires 'command' in config")
                })?;

            println!("Transport: stdio");
            println!("Command:   {}", command);
            println!("Args:      {:?}", config.upstream.args);
            println!();

            match tokio::time::timeout(
                timeout_duration,
                check_stdio_upstream(command, &config.upstream.args),
            )
            .await
            {
                Ok(Ok(())) => println!("✓ Upstream is reachable and responding"),
                Ok(Err(e)) => anyhow::bail!("✗ Upstream check failed: {}", e),
                Err(_) => anyhow::bail!("✗ Upstream check timed out after {}s", timeout),
            }
        }
        mcp_guard_core::config::TransportType::Http => {
            let url = config
                .upstream
                .url
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("HTTP transport requires 'url' in config"))?;

            println!("Transport: HTTP");
            println!("URL:       {}", url);
            println!();

            match tokio::time::timeout(timeout_duration, check_http_upstream(url)).await {
                Ok(Ok(())) => println!("✓ Upstream is reachable"),
                Ok(Err(e)) => anyhow::bail!("✗ Upstream check failed: {}", e),
                Err(_) => anyhow::bail!("✗ Upstream check timed out after {}s", timeout),
            }
        }
        mcp_guard_core::config::TransportType::Sse => {
            let url = config
                .upstream
                .url
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("SSE transport requires 'url' in config"))?;

            println!("Transport: SSE");
            println!("URL:       {}", url);
            println!();

            match tokio::time::timeout(timeout_duration, check_sse_upstream(url)).await {
                Ok(Ok(())) => println!("✓ Upstream is reachable"),
                Ok(Err(e)) => anyhow::bail!("✗ Upstream check failed: {}", e),
                Err(_) => anyhow::bail!("✗ Upstream check timed out after {}s", timeout),
            }
        }
    }

    Ok(())
}

/// Validate that the user has a valid license for the features they're using
///
/// This is the CRITICAL security boundary that prevents users from bypassing licensing
/// by compiling with `--features pro,enterprise` without paying.
///
/// This function is called BEFORE starting the server to ensure license compliance.
#[allow(unused_variables)] // config unused in free tier (no pro/enterprise features)
fn validate_license_for_config(config: &Config) -> anyhow::Result<()> {
    // SECURITY: Allow bypassing license check in development mode
    if std::env::var("MCP_GUARD_SKIP_LICENSE_CHECK").is_ok() {
        tracing::warn!("Bypassing license check (MCP_GUARD_SKIP_LICENSE_CHECK is set)");
        return Ok(());
    }

    // Check if Pro features are being used
    #[cfg(feature = "pro")]
    if config.requires_pro_features() {
        use mcp_guard_pro::license::ProLicense;

        let license_key = std::env::var("MCP_GUARD_LICENSE_KEY").map_err(|_| {
            anyhow::anyhow!(
                "Pro features detected but no license key found.\n\n\
                 Your configuration uses Pro tier features:\n\
                 - OAuth 2.1 authentication\n\
                 - JWT JWKS mode\n\
                 - HTTP/SSE transports\n\n\
                 Set your license key:\n\
                   export MCP_GUARD_LICENSE_KEY=\"pro_xxx...\"\n\n\
                 Don't have a license? Get one at:\n\
                   https://mcpg.botzr.com/pricing\n\
                   Pro tier: $12/month"
            )
        })?;

        let license = ProLicense::validate(&license_key).map_err(|e| {
            anyhow::anyhow!(
                "Pro license validation failed: {}\n\n\
                 Your license key may be:\n\
                 - Expired\n\
                 - Invalid\n\
                 - For a different tier\n\n\
                 To renew or upgrade:\n\
                   https://mcpg.botzr.com/account\n\
                 For support:\n\
                   austin@botzr.com",
                e
            )
        })?;

        // Log successful validation
        tracing::info!(
            licensee = %license.payload.licensee,
            expires_at = %license.payload.expires_at.format("%Y-%m-%d"),
            "Pro license validated successfully"
        );

        // Warn if license is expiring soon
        if license.is_expiring_soon() {
            tracing::warn!(
                days_remaining = license.days_until_expiry(),
                "Pro license expires soon! Renew at https://mcpg.botzr.com/account"
            );
        }
    }

    // Check if Enterprise features are being used
    #[cfg(feature = "enterprise")]
    if config.requires_enterprise_features() {
        use mcp_guard_enterprise::license::EnterpriseLicense;

        // Enterprise licenses are validated via keygen.sh (online with offline cache)
        let license = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { EnterpriseLicense::validate().await })
        })
        .map_err(|e| {
            anyhow::anyhow!(
                "Enterprise license validation failed: {}\n\n\
                 Your configuration uses Enterprise tier features:\n\
                 - mTLS authentication\n\
                 - Multi-server routing\n\
                 - SIEM audit log shipping\n\
                 - OpenTelemetry tracing\n\n\
                 Set your license key:\n\
                   export MCP_GUARD_LICENSE_KEY=\"ent_xxx...\"\n\n\
                 Don't have a license? Get one at:\n\
                   https://mcpg.botzr.com/pricing\n\
                   Enterprise tier: $29/user/month\n\n\
                 For support:\n\
                   austin@botzr.com",
                e
            )
        })?;

        // Log successful validation
        tracing::info!(
            licensee = %license.data.licensee,
            seats = ?license.data.max_seats,
            from_cache = license.from_cache,
            "Enterprise license validated successfully"
        );
    }

    Ok(())
}

/// Handle the `run` command: start the MCP Guard server.
async fn handle_run(
    config_path: &std::path::PathBuf,
    host: Option<String>,
    port: Option<u16>,
    verbose: bool,
) -> anyhow::Result<()> {
    let mut config = Config::from_file(config_path)?;

    // CRITICAL: Validate licenses BEFORE starting server
    // This prevents users from bypassing licensing by compiling with --features
    validate_license_for_config(&config)?;

    // Override with CLI args
    if let Some(h) = host {
        config.server.host = h;
    }
    if let Some(p) = port {
        config.server.port = p;
    }

    // Initialize tracing with OpenTelemetry (if configured)
    let _tracing_guard = init_tracing(verbose, Some(&config.tracing));

    // Log tracing configuration
    if config.tracing.enabled {
        tracing::info!(
            service_name = %config.tracing.service_name,
            otlp_endpoint = ?config.tracing.otlp_endpoint,
            sample_rate = %config.tracing.sample_rate,
            "OpenTelemetry tracing enabled"
        );
    }

    // Bootstrap the server state
    let BootstrapResult {
        state,
        audit_handle,
        shutdown_token,
    } = bootstrap(config.clone()).await?;

    // Print startup summary
    print_startup_summary(&config);

    // Run server with graceful shutdown handling
    tokio::select! {
        result = server::run(state) => {
            // Server exited (error or normal termination)
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received SIGINT, initiating graceful shutdown...");
        }
    }

    // Trigger shutdown for all background tasks
    shutdown_token.cancel();

    // Give background tasks time to complete (e.g., flush audit logs)
    tracing::info!("Shutting down background tasks...");
    audit_handle.shutdown().await;

    tracing::info!("Shutdown complete");
    Ok(())
}

/// Handle the `serve` command: run as an MCP server over stdio.
///
/// This mode is designed for use with Claude Desktop or other MCP clients
/// that spawn mcp-guard as a subprocess and communicate via stdin/stdout.
async fn handle_serve(config_path: &std::path::PathBuf, verbose: bool) -> anyhow::Result<()> {
    let mut config = Config::from_file(config_path)?;

    // CRITICAL: Validate licenses BEFORE starting server
    // This prevents users from bypassing licensing by compiling with --features
    validate_license_for_config(&config)?;

    // Initialize minimal tracing for stdio mode (logs go to stderr to not interfere with MCP)
    let _tracing_guard = init_tracing(verbose, Some(&config.tracing));

    // stdout is the MCP channel. Audit output there would corrupt the protocol stream.
    if config.audit.stdout {
        tracing::warn!("audit.stdout is ignored in serve mode: stdout carries MCP messages");
        config.audit.stdout = false;
    }
    if config.audit.enabled && config.audit.file.is_none() && config.audit.export_url.is_none() {
        tracing::info!("Audit logging is enabled but has no file or export target; set [audit] file to keep a record");
    }

    // Initialize metrics (always available in serve mode)
    let metrics_handle = Some(std::sync::Arc::new(init_metrics()));

    let upstream = connect_upstream(&config).await?;

    let lock_path = if config.policy.pin_tools && upstream.is_some() {
        Some(config.policy.lock_path_for(config_path))
    } else {
        None
    };
    if let Some(path) = &lock_path {
        tracing::info!(lockfile = %path.display(), "Tool pinning enabled");
    }

    let (audit_logger, audit_handle) = AuditLogger::with_tasks(&config.audit)?;

    let recorder = if config.recorder.enabled {
        let dir = config.recorder.dir_for(config_path);
        let info = SessionInfo {
            guard_version: env!("CARGO_PKG_VERSION").to_string(),
            upstream: upstream_label(&config),
            config_path: Some(config_path.display().to_string()),
            policy_mode: Some(format!("{:?}", config.policy.mode).to_lowercase()),
            ..SessionInfo::default()
        };
        match Recorder::start(&config.recorder, &dir, info, &config.audit.redaction_rules) {
            Ok(rec) => {
                tracing::info!(file = %rec.path().display(), signed = config.recorder.signing_key().is_some(), "Flight recorder on");
                Some(Arc::new(rec))
            }
            Err(e) => {
                tracing::error!(error = %e, dir = %dir.display(), "Flight recorder could not start; continuing without it");
                None
            }
        }
    } else {
        None
    };

    let server_config = McpServerConfig {
        policy: config.policy.clone(),
        lock_path,
        upstream_label: upstream_label(&config),
        ..McpServerConfig::default()
    };
    let mut server = McpServer::new(server_config, upstream, metrics_handle)
        .with_audit(std::sync::Arc::new(audit_logger));
    if let Some(rec) = recorder {
        server = server.with_recorder(rec);
    }

    tracing::info!("Starting MCP server in stdio mode");
    let result = server.run().await;
    audit_handle.shutdown().await;
    result?;

    Ok(())
}

/// Connect to the upstream described by the config, if any.
async fn connect_upstream(config: &Config) -> anyhow::Result<Option<Arc<dyn Transport>>> {
    Ok(match config.upstream.transport {
        TransportType::Stdio => {
            if let Some(command) = &config.upstream.command {
                tracing::info!(
                    command = %command,
                    args = ?config.upstream.args,
                    "Connecting to upstream MCP server via stdio"
                );
                let transport = StdioTransport::spawn(command, &config.upstream.args).await?;
                Some(Arc::new(transport))
            } else {
                tracing::warn!("Stdio transport configured but no command specified");
                None
            }
        }
        TransportType::Http => {
            if let Some(url) = &config.upstream.url {
                tracing::info!(url = %url, "Connecting to upstream MCP server via HTTP");
                let transport = HttpTransport::new(url.clone()).await?;
                Some(Arc::new(transport))
            } else {
                tracing::warn!("HTTP transport configured but no URL specified");
                None
            }
        }
        TransportType::Sse => {
            if let Some(url) = &config.upstream.url {
                tracing::info!(url = %url, "Connecting to upstream MCP server via SSE");
                let transport = SseTransport::connect(url.clone()).await?;
                Some(Arc::new(transport))
            } else {
                tracing::warn!("SSE transport configured but no URL specified");
                None
            }
        }
    })
}

/// Human-readable description of the upstream for lockfiles and reports.
fn upstream_label(config: &Config) -> Option<String> {
    match config.upstream.transport {
        TransportType::Stdio => config.upstream.command.as_ref().map(|c| {
            let mut parts = vec![c.clone()];
            parts.extend(config.upstream.args.iter().cloned());
            parts.join(" ")
        }),
        TransportType::Http | TransportType::Sse => config.upstream.url.clone(),
    }
}

// ============================================================================
// Local mode: wrap / unwrap / doctor / pin
// ============================================================================

/// Directory for generated guard configs: $MCP_GUARD_HOME, else a per-user config dir.
fn default_config_dir() -> std::path::PathBuf {
    if let Some(home) = std::env::var_os("MCP_GUARD_HOME") {
        return std::path::PathBuf::from(home);
    }
    if cfg!(windows) {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            return std::path::PathBuf::from(appdata).join("mcp-guard");
        }
    }
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return std::path::PathBuf::from(xdg).join("mcp-guard");
    }
    home.join(".config").join("mcp-guard")
}

fn split_list(value: Option<String>) -> Vec<String> {
    value
        .map(|v| {
            v.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn client_label(file: &clients::ClientConfigFile) -> String {
    match file.kind {
        ClientKind::Custom => file.path.display().to_string(),
        k => k.display_name().to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_wrap(
    client: &str,
    servers: &[String],
    allow: Option<String>,
    deny: Option<String>,
    pin_tools: bool,
    dry_run: bool,
    config_dir: Option<std::path::PathBuf>,
    guard_bin: Option<std::path::PathBuf>,
) -> anyhow::Result<()> {
    let env = DiscoveryEnv::from_system();
    let files = env.resolve(client)?;

    let guard_bin = match guard_bin {
        Some(p) => p,
        None => std::env::current_exe()
            .map_err(|e| anyhow::anyhow!("cannot determine mcp-guard binary path: {}", e))?,
    };
    let mut allowed = split_list(allow);
    if allowed.is_empty() {
        allowed.push("*".to_string());
    }
    let opts = WrapOptions {
        guard_bin,
        config_dir: config_dir.unwrap_or_else(default_config_dir),
        only: servers.to_vec(),
        allowed_tools: allowed,
        denied_tools: split_list(deny),
        pin_tools,
    };

    let mut wrapped_total = 0;
    for file in &files {
        let mut loaded = LoadedClientConfig::load(file)?;
        let actions = clients::plan_wrap(&loaded, &opts);
        println!("{} ({})", client_label(file), file.path.display());
        if actions.is_empty() {
            println!("  no matching servers");
            println!();
            continue;
        }
        for action in &actions {
            match action {
                WrapAction::Wrap {
                    name,
                    guard_config_path,
                    ..
                } => println!(
                    "  {} {:<24} -> mcp-guard serve  ({})",
                    if dry_run { "would wrap" } else { "wrap" },
                    name,
                    guard_config_path.display()
                ),
                WrapAction::Skip { name, reason } => {
                    println!("  skip       {:<24}    {}", name, reason)
                }
            }
        }
        let to_wrap = actions
            .iter()
            .filter(|a| matches!(a, WrapAction::Wrap { .. }))
            .count();
        if to_wrap > 0 && !dry_run {
            let outcome = clients::apply_wrap(&mut loaded, &actions)?;
            println!("  backup: {}", outcome.backup_path.display());
            wrapped_total += outcome.wrapped.len();
        } else if to_wrap > 0 {
            wrapped_total += to_wrap;
        }
        println!();
    }

    if dry_run {
        println!(
            "Dry run: {} server(s) would be wrapped. Re-run without --dry-run to apply.",
            wrapped_total
        );
    } else if wrapped_total > 0 {
        println!(
            "Wrapped {} server(s). Restart the client to pick up the change.",
            wrapped_total
        );
        println!("Tool definitions are pinned on first use; review them with `mcp-guard pin --config <guard config>`.");
        println!("Tighten access by editing [policy] in the generated config, then run `mcp-guard doctor`.");
    } else {
        println!("Nothing wrapped.");
    }
    Ok(())
}

fn handle_unwrap(client: &str, servers: &[String], dry_run: bool) -> anyhow::Result<()> {
    let env = DiscoveryEnv::from_system();
    let files = env.resolve(client)?;
    let mut restored_total = 0;
    for file in &files {
        let mut loaded = LoadedClientConfig::load(file)?;
        let actions = clients::plan_unwrap(&loaded, servers);
        println!("{} ({})", client_label(file), file.path.display());
        for action in &actions {
            match action {
                UnwrapAction::Unwrap {
                    name,
                    guard_config_path,
                    ..
                } => println!(
                    "  {} {:<24}    (guard config kept at {})",
                    if dry_run { "would restore" } else { "restore" },
                    name,
                    guard_config_path.display()
                ),
                UnwrapAction::Skip { name, reason } => {
                    println!("  skip          {:<24} {}", name, reason)
                }
            }
        }
        let to_restore = actions
            .iter()
            .filter(|a| matches!(a, UnwrapAction::Unwrap { .. }))
            .count();
        if to_restore > 0 && !dry_run {
            let outcome = clients::apply_unwrap(&mut loaded, &actions)?;
            println!("  backup: {}", outcome.backup_path.display());
            restored_total += outcome.wrapped.len();
        } else {
            restored_total += to_restore;
        }
        println!();
    }
    if dry_run {
        println!("Dry run: {} server(s) would be restored.", restored_total);
    } else {
        println!(
            "Restored {} server(s). Restart the client to pick up the change.",
            restored_total
        );
    }
    Ok(())
}

fn handle_doctor(client_specs: &[String], json: bool, strict: bool) -> anyhow::Result<()> {
    let env = DiscoveryEnv::from_system();
    let files = if client_specs.is_empty() {
        env.discover(ClientKind::all())
    } else {
        let mut files = Vec::new();
        for spec in client_specs {
            files.extend(env.resolve(spec)?);
        }
        files
    };

    let report = clients::doctor(&files);

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("mcp-guard doctor");
        println!();
        if report.files_scanned.is_empty() {
            println!("No MCP client configuration files found.");
            println!(
                "Looked for Claude Desktop, Claude Code, Cursor, Windsurf and VS Code configs."
            );
            println!("Point at a file directly with --client <path>.");
            return Ok(());
        }
        println!(
            "Scanned {} client config file(s):",
            report.files_scanned.len()
        );
        for file in &report.files_scanned {
            println!("  {:<16} {}", file.kind.display_name(), file.path.display());
        }
        println!();
        println!(
            "Servers: {} found, {} guarded by mcp-guard",
            report.servers_seen, report.guarded
        );
        println!();

        if report.findings.is_empty() {
            println!("No findings. Every server is guarded.");
        } else {
            for f in &report.findings {
                let where_ = match &f.server {
                    Some(s) => format!("{} / {}", f.client.display_name(), s),
                    None => f.client.display_name().to_string(),
                };
                println!("{:<5} {}", f.severity.label(), where_);
                println!("      {}", f.message);
                if let Some(fix) = &f.fix {
                    println!("      fix: {}", fix);
                }
                println!();
            }
            println!(
                "Summary: {} high, {} warn, {} info",
                report.count(Severity::High),
                report.count(Severity::Warn),
                report.count(Severity::Info)
            );
            let unguarded: Vec<&clients::Finding> = report
                .findings
                .iter()
                .filter(|f| f.code == "UNGUARDED_STDIO")
                .collect();
            if let Some(first) = unguarded.first() {
                let client_arg = match first.client {
                    ClientKind::Custom => first.file.display().to_string(),
                    k => k.slug().to_string(),
                };
                println!();
                println!(
                    "Next step: mcp-guard wrap --client {}   (wraps every stdio server in that client)",
                    client_arg
                );
            }
        }
    }

    if strict && (report.count(Severity::High) > 0 || report.count(Severity::Warn) > 0) {
        std::process::exit(1);
    }
    Ok(())
}

// ============================================================================
// Flight recorder: replay / verify
// ============================================================================

fn recordings_dir(
    config_path: &std::path::PathBuf,
    dir: Option<std::path::PathBuf>,
) -> anyhow::Result<(std::path::PathBuf, Option<Config>)> {
    if let Some(d) = dir {
        return Ok((d, None));
    }
    let config = Config::from_file(config_path).map_err(|e| {
        anyhow::anyhow!(
            "cannot load {} ({}); pass --dir to point at a recordings directory",
            config_path.display(),
            e
        )
    })?;
    Ok((config.recorder.dir_for(config_path), Some(config)))
}

#[allow(clippy::too_many_arguments)]
fn handle_replay(
    config_path: &std::path::PathBuf,
    session: &str,
    dir: Option<std::path::PathBuf>,
    list: bool,
    denied_only: bool,
    tool: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let (dir, _) = recordings_dir(config_path, dir)?;

    if list {
        let sessions = recorder::list_sessions(&dir)?;
        if sessions.is_empty() {
            println!("No recordings in {}", dir.display());
            return Ok(());
        }
        if json {
            let rows: Vec<serde_json::Value> = sessions
                .iter()
                .filter_map(|p| Session::load(p).ok())
                .map(|s| {
                    let info = s.info().unwrap_or_default();
                    serde_json::json!({
                        "file": s.path,
                        "session_id": info.session_id,
                        "started_at": info.started_at,
                        "upstream": info.upstream,
                        "records": s.records.len(),
                        "ended": s.ended(),
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&rows)?);
            return Ok(());
        }
        println!("Recordings in {}:", dir.display());
        for path in sessions {
            match Session::load(&path) {
                Ok(s) => {
                    let info = s.info().unwrap_or_default();
                    let decisions = s
                        .records
                        .iter()
                        .filter(|r| r.kind == recorder::RecordKind::Decision)
                        .count();
                    println!(
                        "  {:<14} {}  {:>5} records  {:>3} decisions  {}{}",
                        info.session_id,
                        info.started_at,
                        s.records.len(),
                        decisions,
                        info.upstream.unwrap_or_default(),
                        if s.ended() { "" } else { "  (open)" }
                    );
                }
                Err(e) => println!("  {}  (unreadable: {})", path.display(), e),
            }
        }
        return Ok(());
    }

    let Some(path) = recorder::find_session(&dir, session)? else {
        anyhow::bail!(
            "no session matching '{}' in {} (use --list to see sessions)",
            session,
            dir.display()
        );
    };
    let loaded = Session::load(&path)?;

    if json {
        for r in &loaded.records {
            println!("{}", serde_json::to_string(r)?);
        }
        return Ok(());
    }

    let info = loaded.info().unwrap_or_default();
    println!("Session {}  {}", info.session_id, path.display());
    if let Some(up) = &info.upstream {
        println!("Upstream {}", up);
    }
    println!(
        "Records  {}{}   Policy mode: {}",
        loaded.records.len(),
        if loaded.ended() {
            ""
        } else {
            "  (session still open or cut off)"
        },
        info.policy_mode.unwrap_or_else(|| "?".into())
    );
    println!();

    let entries: Vec<recorder::TimelineEntry> = recorder::timeline(&loaded)
        .into_iter()
        .filter(|e| !denied_only || e.denied || e.label.starts_with("session"))
        .filter(|e| match &tool {
            Some(t) => e.label.ends_with(&format!(" {}", t)) || e.label.starts_with("session"),
            None => true,
        })
        .collect();
    println!("{}", recorder::render_timeline(&entries));
    println!();

    let v = recorder::verify(&loaded, None);
    let chain = if v.ok {
        if v.key_checked {
            "Chain OK".to_string()
        } else {
            "Chain continuity OK (signed recording; run `mcp-guard verify` with the signing key in the environment to check signatures)".to_string()
        }
    } else {
        format!(
            "CHAIN BROKEN at record {} ({})",
            v.first_bad_seq
                .map(|s| s.to_string())
                .unwrap_or_else(|| "?".into()),
            v.problems.first().cloned().unwrap_or_default()
        )
    };
    println!("{}", chain);
    Ok(())
}

fn handle_verify(
    config_path: &std::path::PathBuf,
    session: &str,
    dir: Option<std::path::PathBuf>,
    json: bool,
) -> anyhow::Result<()> {
    let (dir, config) = recordings_dir(config_path, dir)?;
    let key = config.as_ref().and_then(|c| c.recorder.signing_key());

    let paths: Vec<std::path::PathBuf> = if session == "all" {
        recorder::list_sessions(&dir)?
    } else {
        match recorder::find_session(&dir, session)? {
            Some(p) => vec![p],
            None => anyhow::bail!(
                "no session matching '{}' in {} (use `mcp-guard replay --list`)",
                session,
                dir.display()
            ),
        }
    };
    if paths.is_empty() {
        anyhow::bail!("no recordings in {}", dir.display());
    }

    let mut all_ok = true;
    let mut continuity_only = false;
    let mut results = Vec::new();
    for path in &paths {
        let loaded = Session::load(path)?;
        let v = recorder::verify(&loaded, key.as_deref());
        all_ok &= v.ok;
        continuity_only |= v.signed && !v.key_checked;
        if json {
            results.push(serde_json::json!({ "file": path, "result": v }));
        } else {
            let status = if v.ok {
                if v.key_checked {
                    "OK      "
                } else {
                    "OK*     "
                }
            } else {
                "BROKEN  "
            };
            println!("{} {}  ({} records)", status, path.display(), v.records);
            for p in &v.problems {
                println!("           {}", p);
            }
        }
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else if continuity_only {
        println!();
        println!("* signed recording checked for continuity only; set the [recorder] signing_key_env variable to verify signatures");
    }
    if !all_ok {
        std::process::exit(1);
    }
    Ok(())
}

// ============================================================================
// Policy: test / suggest
// ============================================================================

fn handle_policy_test(
    config_path: &std::path::PathBuf,
    tool: &str,
    args: &str,
) -> anyhow::Result<()> {
    let config = Config::from_file(config_path)?;
    let arguments: serde_json::Value = serde_json::from_str(args)
        .map_err(|e| anyhow::anyhow!("--args must be a JSON object: {}", e))?;
    if !arguments.is_object() {
        anyhow::bail!("--args must be a JSON object, e.g. '{{\"path\": \"/tmp/x\"}}'");
    }

    let decision = policy::evaluate(&config.policy, tool, &arguments);
    println!("Config:  {}", config_path.display());
    println!("Mode:    {:?}", config.policy.mode);
    println!("Call:    {} {}", tool, arguments);
    match &decision {
        Decision::Allow => {
            println!("Result:  ALLOW");
        }
        Decision::Deny { reason, rule } => {
            let enforced = config.policy.mode == mcp_guard_core::config::PolicyMode::Enforce;
            println!(
                "Result:  {}  ({})",
                if enforced {
                    "DENY"
                } else {
                    "WOULD DENY (observe mode, call is allowed)"
                },
                reason
            );
            if let Some(rule) = rule {
                println!("Rule:    {}", rule);
            }
            if enforced {
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct AuditSummary {
    entries: usize,
    calls: std::collections::BTreeMap<String, usize>,
    denied: std::collections::BTreeMap<String, usize>,
    would_deny: std::collections::BTreeMap<(String, String), usize>,
    first: Option<String>,
    last: Option<String>,
}

fn summarize_audit_log(path: &std::path::Path) -> anyhow::Result<AuditSummary> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read audit log {}: {}", path.display(), e))?;
    let mut summary = AuditSummary::default();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        summary.entries += 1;
        if let Some(ts) = entry.get("timestamp").and_then(|t| t.as_str()) {
            if summary.first.is_none() {
                summary.first = Some(ts.to_string());
            }
            summary.last = Some(ts.to_string());
        }
        let tool = entry
            .get("tool")
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();
        match entry.get("event_type").and_then(|e| e.as_str()) {
            Some("tool_call") if !tool.is_empty() => {
                *summary.calls.entry(tool).or_default() += 1;
            }
            Some("authz_denied") if !tool.is_empty() => {
                *summary.denied.entry(tool).or_default() += 1;
            }
            Some("policy_would_deny") if !tool.is_empty() => {
                let reason = entry
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();
                *summary.would_deny.entry((tool, reason)).or_default() += 1;
            }
            _ => {}
        }
    }
    Ok(summary)
}

fn handle_policy_suggest(
    config_path: &std::path::PathBuf,
    apply: bool,
    audit_log: Option<std::path::PathBuf>,
) -> anyhow::Result<()> {
    let config = Config::from_file(config_path)?;
    let log_path = match audit_log.or_else(|| config.audit.file.clone()) {
        Some(p) if p.is_absolute() => p,
        Some(p) => config_path.parent().map(|d| d.join(&p)).unwrap_or(p),
        None => anyhow::bail!(
            "no audit log to read: set [audit] file in {} or pass --audit-log",
            config_path.display()
        ),
    };

    let summary = summarize_audit_log(&log_path)?;
    println!(
        "Audit log: {}  ({} entries)",
        log_path.display(),
        summary.entries
    );
    if let (Some(first), Some(last)) = (&summary.first, &summary.last) {
        println!("Window:    {}  to  {}", first, last);
    }
    println!("Mode now:  {:?}", config.policy.mode);
    println!();

    if summary.calls.is_empty() {
        println!(
            "No tool calls recorded yet. Run the server with `mode = \"observe\"` in [policy],"
        );
        println!("use the agent normally for a while, then run this command again.");
        return Ok(());
    }

    println!("Tools the agent actually called:");
    for (tool, n) in &summary.calls {
        println!("  {:<40} {:>6} call(s)", tool, n);
    }
    if !summary.denied.is_empty() {
        println!();
        println!("Calls denied by the current policy:");
        for (tool, n) in &summary.denied {
            println!("  {:<40} {:>6}", tool, n);
        }
    }
    if !summary.would_deny.is_empty() {
        println!();
        println!("Calls the current policy would have denied (observe mode):");
        for ((tool, reason), n) in &summary.would_deny {
            println!("  {:<40} {:>6}   {}", tool, n, reason);
        }
    }

    let allowed: Vec<String> = summary.calls.keys().cloned().collect();
    println!();
    println!("Suggested least-privilege policy (every other tool is hidden and denied):");
    println!();
    println!("[policy]");
    println!("mode = \"enforce\"");
    println!(
        "allowed_tools = [{}]",
        allowed
            .iter()
            .map(|t| format!("\"{}\"", t))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!();

    if apply {
        apply_policy_suggestion(config_path, &allowed)
            .map_err(|e| anyhow::anyhow!("failed to update {}: {}", config_path.display(), e))?;
        println!(
            "Applied to {}. Existing denied_tools and rules were kept. Restart the client to pick it up.",
            config_path.display()
        );
    } else {
        println!(
            "Re-run with --apply to write this into {}.",
            config_path.display()
        );
    }
    Ok(())
}

async fn handle_pin(
    config_path: &std::path::PathBuf,
    accept: bool,
    show: bool,
    timeout: u64,
    verbose: bool,
) -> anyhow::Result<()> {
    let _guard = init_tracing(verbose, None);
    let config = Config::from_file(config_path)?;
    let lock_path = config.policy.lock_path_for(config_path);

    let upstream = connect_upstream(&config)
        .await?
        .ok_or_else(|| anyhow::anyhow!("config has no upstream to pin"))?;
    let tools = tokio::time::timeout(
        std::time::Duration::from_secs(timeout),
        fetch_upstream_tools(upstream.as_ref()),
    )
    .await
    .map_err(|_| anyhow::anyhow!("upstream did not answer tools/list within {}s", timeout))?
    .map_err(|e| anyhow::anyhow!("failed to fetch tools from upstream: {}", e))?;
    let _ = upstream.close().await;

    let current = ToolLockfile::from_tools(&tools, upstream_label(&config));
    println!("Upstream: {}", current.upstream.clone().unwrap_or_default());
    println!("Lockfile: {}", lock_path.display());
    println!();

    if show {
        for (name, pinned) in &current.tools {
            println!("  {:<32} {}", name, pinned.hash);
        }
        println!();
    }

    let existing = ToolLockfile::load(&lock_path)?;
    let (clean, summary) = match &existing {
        None => (
            false,
            format!("No lockfile yet; {} tool(s) advertised.", tools.len()),
        ),
        Some(lock) => {
            let diff = lock.diff(&tools);
            let mut lines = Vec::new();
            for name in &diff.changed {
                let before = lock
                    .tools
                    .get(name)
                    .and_then(|t| t.description.clone())
                    .unwrap_or_default();
                let after = current
                    .tools
                    .get(name)
                    .and_then(|t| t.description.clone())
                    .unwrap_or_default();
                lines.push(format!("  changed  {}", name));
                if before != after {
                    lines.push(format!("           was: {}", before));
                    lines.push(format!("           now: {}", after));
                } else {
                    lines.push("           (input schema or annotations changed)".to_string());
                }
            }
            for name in &diff.new {
                let desc = current
                    .tools
                    .get(name)
                    .and_then(|t| t.description.clone())
                    .unwrap_or_default();
                lines.push(format!("  new      {}", name));
                if !desc.is_empty() {
                    lines.push(format!("           {}", desc));
                }
            }
            for name in &diff.removed {
                lines.push(format!("  removed  {}", name));
            }
            if diff.is_clean() {
                (
                    true,
                    format!(
                        "Lockfile is up to date: {} pinned tool(s){}.",
                        diff.pinned.len(),
                        if diff.removed.is_empty() {
                            String::new()
                        } else {
                            format!(", {} no longer advertised", diff.removed.len())
                        }
                    ),
                )
            } else {
                (
                    false,
                    format!(
                        "{} changed, {} new, {} removed since pinning:\n{}",
                        diff.changed.len(),
                        diff.new.len(),
                        diff.removed.len(),
                        lines.join("\n")
                    ),
                )
            }
        }
    };
    println!("{}", summary);

    if accept || existing.is_none() {
        current.save(&lock_path)?;
        println!();
        println!(
            "Pinned {} tool(s) to {}",
            current.tools.len(),
            lock_path.display()
        );
        return Ok(());
    }
    if !clean {
        println!();
        println!(
            "Review the changes above. Run `mcp-guard pin --accept --config {}` to accept them.",
            config_path.display()
        );
        std::process::exit(1);
    }
    Ok(())
}

/// Check stdio upstream connectivity by spawning the process and sending an initialize request
async fn check_stdio_upstream(command: &str, args: &[String]) -> anyhow::Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::process::Command;

    // Spawn the upstream process
    let mut child = Command::new(command)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("Failed to open stdout"))?;
    let mut reader = BufReader::new(stdout);

    // Send MCP initialize request
    let init_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "mcp-guard-check",
                "version": env!("CARGO_PKG_VERSION")
            }
        }
    });

    let msg = format!("{}\n", serde_json::to_string(&init_request)?);
    stdin.write_all(msg.as_bytes()).await?;
    stdin.flush().await?;

    // Read response
    let mut line = String::new();
    reader.read_line(&mut line).await?;

    if line.is_empty() {
        return Err(anyhow::anyhow!("No response from upstream"));
    }

    // Parse response to verify it's valid JSON-RPC
    let response: serde_json::Value = serde_json::from_str(&line)?;
    if response.get("result").is_some() || response.get("error").is_some() {
        // Valid JSON-RPC response
        if let Some(result) = response.get("result") {
            if let Some(server_info) = result.get("serverInfo") {
                println!(
                    "Server: {} v{}",
                    server_info
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown"),
                    server_info
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                );
            }
        }
        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid JSON-RPC response: {}", line))
    }
}

/// Check HTTP upstream connectivity by sending a simple request
async fn check_http_upstream(url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    // Try to send an empty POST to check if the server is responding
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body("{}")
        .send()
        .await?;

    let status = response.status();
    println!("HTTP Status: {}", status);

    // Any response (even 400/500) means the server is reachable
    Ok(())
}

/// Check SSE upstream connectivity by attempting to connect
async fn check_sse_upstream(url: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    // Try to connect to the SSE endpoint
    let response = client
        .get(url)
        .header("Accept", "text/event-stream")
        .send()
        .await?;

    let status = response.status();
    println!("HTTP Status: {}", status);

    // Check if the content type suggests SSE
    if let Some(content_type) = response.headers().get("content-type") {
        println!(
            "Content-Type: {}",
            content_type.to_str().unwrap_or("unknown")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper to create a minimal valid config for testing
    #[allow(dead_code)] // used only by feature-gated tests
    fn create_test_config_http(url: &str) -> Config {
        let config_str = format!(
            r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#,
            url
        );
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }

    fn create_test_config_stdio() -> Config {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();
        Config::from_file(&temp_file.path().to_path_buf()).unwrap()
    }

    #[tokio::test]
    async fn test_run_cli_hash_key() {
        let cli = Cli {
            config: "config.toml".into(),
            verbose: false,
            command: Commands::HashKey {
                key: "test-key".to_string(),
            },
        };

        let result = run_cli(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_cli_version() {
        let cli = Cli {
            config: "config.toml".into(),
            verbose: false,
            command: Commands::Version,
        };

        let result = run_cli(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_cli_keygen() {
        let cli = Cli {
            config: "config.toml".into(),
            verbose: false,
            command: Commands::Keygen {
                user_id: "test-user".to_string(),
                rate_limit: Some(100),
                tools: Some("read,write".to_string()),
                apply_to_config: false,
            },
        };

        let result = run_cli(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_cli_keygen_no_extras() {
        let cli = Cli {
            config: "config.toml".into(),
            verbose: false,
            command: Commands::Keygen {
                user_id: "simple-user".to_string(),
                rate_limit: None,
                tools: None,
                apply_to_config: false,
            },
        };

        let result = run_cli(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_cli_validate_missing_config() {
        let cli = Cli {
            config: "non-existent-config.toml".into(),
            verbose: false,
            command: Commands::Validate,
        };

        let result = run_cli(cli).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_cli_validate_valid_config() {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_str.as_bytes()).unwrap();

        let cli = Cli {
            config: temp_file.path().to_path_buf(),
            verbose: false,
            command: Commands::Validate,
        };

        let result = run_cli(cli).await;
        assert!(result.is_ok());
    }

    // HTTP transport tests only run with pro feature
    #[cfg(feature = "pro")]
    #[tokio::test]
    async fn test_bootstrap_with_http_transport() {
        use wiremock::matchers::any;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"result": {}})),
            )
            .mount(&mock_server)
            .await;

        let config = create_test_config_http(&mock_server.uri());

        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());

        let bootstrap_result = result.unwrap();
        assert!(bootstrap_result.state.transport.is_some());
        assert!(bootstrap_result.state.router.is_none());

        // Clean up
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_bootstrap_with_api_key_auth() {
        // Use stdio transport which is available in free tier
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false

[[auth.api_keys]]
id = "test-user"
key_hash = "abc123"
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();
        let config = Config::from_file(&temp_file.path().to_path_buf()).unwrap();

        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());

        let bootstrap_result = result.unwrap();
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_bootstrap_with_no_auth() {
        // Use stdio transport which is available in free tier
        let config = create_test_config_stdio();

        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());

        let bootstrap_result = result.unwrap();
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_check_http_upstream_success() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let result = check_http_upstream(&mock_server.uri()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_http_upstream_server_error() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        // Even 500 is "reachable"
        let result = check_http_upstream(&mock_server.uri()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_sse_upstream_success() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).insert_header("content-type", "text/event-stream"),
            )
            .mount(&mock_server)
            .await;

        let result = check_sse_upstream(&mock_server.uri()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_sse_upstream_no_content_type() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let result = check_sse_upstream(&mock_server.uri()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_stdio_upstream_invalid_command() {
        let result = check_stdio_upstream("/nonexistent/command", &[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_cli_check_upstream_missing_config() {
        let cli = Cli {
            config: "nonexistent.toml".into(),
            verbose: false,
            command: Commands::CheckUpstream { timeout: 5 },
        };

        let result = run_cli(cli).await;
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Additional Main.rs Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_generate_config_yaml() {
        let (config, demo_key) = generate_config_with_demo_key("yaml");
        assert!(config.contains("server:"));
        assert!(config.contains("upstream:"));
        assert!(!demo_key.is_empty());
    }

    #[test]
    fn test_generate_config_toml() {
        let (config, demo_key) = generate_config_with_demo_key("toml");
        assert!(config.contains("[server]"));
        assert!(config.contains("[upstream]"));
        assert!(!demo_key.is_empty());
    }

    #[tokio::test]
    async fn test_bootstrap_with_multiple_auth_providers() {
        // Use stdio transport which is available in free tier
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "/bin/echo"
args = []

[rate_limit]
enabled = false

[[auth.api_keys]]
id = "user1"
key_hash = "hash1"

[auth.jwt]
mode = "simple"
secret = "very-long-secret-at-least-32-characters"
issuer = "test"
audience = "test"
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();
        let config = Config::from_file(&temp_file.path().to_path_buf()).unwrap();

        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());

        let bootstrap_result = result.unwrap();
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_bootstrap_invalid_config() {
        // Use stdio transport which is available in free tier
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000
[upstream]
transport = "stdio"
command = "/bin/echo"
args = []
[rate_limit]
enabled = true
requests_per_second = 0
burst_size = 10
"#;

        // This should fail at Config::from_file, protecting bootstrap
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();

        let result = Config::from_file(&temp_file.path().to_path_buf());
        assert!(result.is_err());
    }

    // Multi-server routing tests only run with enterprise feature
    #[cfg(feature = "enterprise")]
    #[tokio::test]
    async fn test_bootstrap_fail_router_init_multi_server() {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000
[rate_limit]
enabled = false
[upstream]
transport = "http"
[[upstream.servers]]
name = "server1"
path_prefix = "/s1"
transport = { type = "http", url = "not-a-valid-url:::" }
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();

        if let Ok(config) = Config::from_file(&temp_file.path().to_path_buf()) {
            let result = bootstrap(config).await;
            assert!(result.is_err());
        }
    }

    // -------------------------------------------------------------------------
    // Tier Validation Tests
    // -------------------------------------------------------------------------

    #[cfg(not(feature = "pro"))]
    #[test]
    fn test_tier_validation_http_requires_pro() {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000
[upstream]
transport = "http"
url = "http://localhost:8080"
[rate_limit]
enabled = false
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();

        let result = Config::from_file(&temp_file.path().to_path_buf());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Pro license"),
            "Expected Pro license error: {}",
            err
        );
    }

    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_tier_validation_multi_server_requires_enterprise() {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000
[rate_limit]
enabled = false
[upstream]
transport = "stdio"
command = "/bin/echo"
[[upstream.servers]]
name = "server1"
path_prefix = "/s1"
transport = "stdio"
command = "echo"
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();

        let result = Config::from_file(&temp_file.path().to_path_buf());
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Enterprise license"),
            "Expected Enterprise license error: {}",
            err
        );
    }

    #[test]
    fn test_tier_validation_stdio_always_allowed() {
        let config_str = r#"
[server]
host = "127.0.0.1"
port = 3000
[upstream]
transport = "stdio"
command = "/bin/echo"
args = []
[rate_limit]
enabled = false
"#;
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), config_str).unwrap();

        let result = Config::from_file(&temp_file.path().to_path_buf());
        assert!(
            result.is_ok(),
            "stdio should always be allowed: {:?}",
            result.err()
        );
    }
}
