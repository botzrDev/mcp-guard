//! MCP Guard - Security gateway for MCP servers

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use mcp_guard::{
    audit::AuditLogger,
    auth::{ApiKeyProvider, AuthProvider, JwtProvider, MtlsAuthProvider, MultiProvider, OAuthAuthProvider},
    cli::{generate_api_key, generate_config, hash_api_key, Cli, Commands},
    config::Config,
    observability::{init_metrics, init_tracing},
    rate_limit::RateLimitService,
    router::ServerRouter,
    server::{self, new_oauth_state_store, AppState},
    transport::{HttpTransport, SseTransport, StdioTransport},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();

    match cli.command {
        Commands::Init { format, force } => {
            // Initialize basic tracing for CLI commands
            let _guard = init_tracing(cli.verbose, None);

            let filename = if format == "yaml" {
                "mcp-guard.yaml"
            } else {
                "mcp-guard.toml"
            };

            let path = std::path::Path::new(filename);
            if path.exists() && !force {
                eprintln!(
                    "Error: {} already exists. Use --force to overwrite.",
                    filename
                );
                std::process::exit(1);
            }

            let config = generate_config(&format);
            std::fs::write(filename, config)?;
            println!("Created configuration file: {}", filename);
        }

        Commands::Validate => {
            // Initialize basic tracing for CLI commands
            let _guard = init_tracing(cli.verbose, None);

            match Config::from_file(&cli.config) {
                Ok(_) => {
                    println!("Configuration is valid: {}", cli.config.display());
                }
                Err(e) => {
                    eprintln!("Configuration error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Keygen {
            user_id,
            rate_limit,
            tools,
        } => {
            // Initialize basic tracing for CLI commands
            let _guard = init_tracing(cli.verbose, None);

            let key = generate_api_key();
            let hash = hash_api_key(&key);

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

            if let Some(tools_str) = tools {
                let tool_list: Vec<&str> = tools_str.split(',').map(|s| s.trim()).collect();
                println!("  allowed_tools = {:?}", tool_list);
            }
        }

        Commands::HashKey { key } => {
            // No tracing needed for simple hash operation
            let hash = hash_api_key(&key);
            println!("{}", hash);
        }

        Commands::Version => {
            println!("mcp-guard {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Build Information:");
            println!("  Package:     {}", env!("CARGO_PKG_NAME"));
            println!("  Version:     {}", env!("CARGO_PKG_VERSION"));
            println!("  Description: {}", env!("CARGO_PKG_DESCRIPTION"));
            println!("  License:     {}", env!("CARGO_PKG_LICENSE"));
            println!("  Repository:  {}", env!("CARGO_PKG_REPOSITORY"));
            println!();
            println!("Features:");
            println!("  Auth providers: API Key, JWT (HS256/JWKS), OAuth 2.1 (PKCE), mTLS");
            println!("  Transports:     Stdio, HTTP, SSE");
            println!("  Rate limiting:  Per-identity, token bucket");
            println!("  Observability:  Prometheus metrics, OpenTelemetry tracing");
        }

        Commands::CheckUpstream { timeout } => {
            // Initialize basic tracing for CLI commands
            let _guard = init_tracing(cli.verbose, None);

            // Load configuration
            let config = match Config::from_file(&cli.config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error loading config: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Checking upstream connectivity...");
            println!();

            match &config.upstream.transport {
                mcp_guard::config::TransportType::Stdio => {
                    let command = config
                        .upstream
                        .command
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("stdio transport requires 'command' in config"))?;

                    println!("Transport: stdio");
                    println!("Command:   {}", command);
                    println!("Args:      {:?}", config.upstream.args);
                    println!();

                    // Try to spawn the process and send a test message
                    let timeout_duration = std::time::Duration::from_secs(timeout);
                    match tokio::time::timeout(
                        timeout_duration,
                        check_stdio_upstream(command, &config.upstream.args),
                    )
                    .await
                    {
                        Ok(Ok(())) => {
                            println!("✓ Upstream is reachable and responding");
                        }
                        Ok(Err(e)) => {
                            eprintln!("✗ Upstream check failed: {}", e);
                            std::process::exit(1);
                        }
                        Err(_) => {
                            eprintln!("✗ Upstream check timed out after {}s", timeout);
                            std::process::exit(1);
                        }
                    }
                }
                mcp_guard::config::TransportType::Http => {
                    let url = config
                        .upstream
                        .url
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("HTTP transport requires 'url' in config"))?;

                    println!("Transport: HTTP");
                    println!("URL:       {}", url);
                    println!();

                    let timeout_duration = std::time::Duration::from_secs(timeout);
                    match tokio::time::timeout(timeout_duration, check_http_upstream(url)).await {
                        Ok(Ok(())) => {
                            println!("✓ Upstream is reachable");
                        }
                        Ok(Err(e)) => {
                            eprintln!("✗ Upstream check failed: {}", e);
                            std::process::exit(1);
                        }
                        Err(_) => {
                            eprintln!("✗ Upstream check timed out after {}s", timeout);
                            std::process::exit(1);
                        }
                    }
                }
                mcp_guard::config::TransportType::Sse => {
                    let url = config
                        .upstream
                        .url
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("SSE transport requires 'url' in config"))?;

                    println!("Transport: SSE");
                    println!("URL:       {}", url);
                    println!();

                    let timeout_duration = std::time::Duration::from_secs(timeout);
                    match tokio::time::timeout(timeout_duration, check_sse_upstream(url)).await {
                        Ok(Ok(())) => {
                            println!("✓ Upstream is reachable");
                        }
                        Ok(Err(e)) => {
                            eprintln!("✗ Upstream check failed: {}", e);
                            std::process::exit(1);
                        }
                        Err(_) => {
                            eprintln!("✗ Upstream check timed out after {}s", timeout);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }

        Commands::Run { host, port } => {
            // Load configuration first so we can use tracing config
            let mut config = Config::from_file(&cli.config)?;

            // Override with CLI args
            if let Some(h) = host {
                config.server.host = h;
            }
            if let Some(p) = port {
                config.server.port = p;
            }

            // Initialize tracing with OpenTelemetry (if configured)
            let _tracing_guard = init_tracing(cli.verbose, Some(&config.tracing));

            // Log tracing configuration
            if config.tracing.enabled {
                tracing::info!(
                    service_name = %config.tracing.service_name,
                    otlp_endpoint = ?config.tracing.otlp_endpoint,
                    sample_rate = %config.tracing.sample_rate,
                    "OpenTelemetry tracing enabled"
                );
            }

            // Initialize Prometheus metrics
            let metrics_handle = init_metrics();

            // Set up OAuth provider (separate from MultiProvider for auth code flow)
            let oauth_provider: Option<Arc<OAuthAuthProvider>> =
                if let Some(oauth_config) = config.auth.oauth.clone() {
                    tracing::info!("Enabling OAuth 2.1 authentication (provider: {:?})", oauth_config.provider);
                    Some(Arc::new(
                        OAuthAuthProvider::new(oauth_config)
                            .map_err(|e| anyhow::anyhow!("Failed to initialize OAuth provider: {}", e))?
                    ))
                } else {
                    None
                };

            // Set up authentication provider(s)
            let auth_provider: Arc<dyn AuthProvider> = {
                let mut providers: Vec<Arc<dyn AuthProvider>> = Vec::new();

                // Add API key provider if configured
                if !config.auth.api_keys.is_empty() {
                    tracing::info!("Enabling API key authentication ({} keys)", config.auth.api_keys.len());
                    providers.push(Arc::new(ApiKeyProvider::new(config.auth.api_keys.clone())));
                }

                // Add JWT provider if configured
                if let Some(jwt_config) = &config.auth.jwt {
                    tracing::info!("Enabling JWT authentication");
                    let jwt_provider = Arc::new(
                        JwtProvider::new(jwt_config.clone())
                            .map_err(|e| anyhow::anyhow!("Failed to initialize JWT provider: {}", e))?
                    );
                    // Start background refresh for JWKS mode
                    jwt_provider.start_background_refresh();
                    providers.push(jwt_provider);
                }

                // Add OAuth provider for token validation (shares with oauth_provider)
                if let Some(ref oauth_prov) = oauth_provider {
                    providers.push(oauth_prov.clone());
                }

                // Select appropriate provider setup
                if providers.is_empty() {
                    tracing::warn!("No authentication providers configured - all requests will be rejected");
                    Arc::new(ApiKeyProvider::new(vec![])) // Deny all
                } else if providers.len() == 1 {
                    // Safe: we just checked length is exactly 1
                    providers.into_iter().next().unwrap_or_else(|| {
                        Arc::new(ApiKeyProvider::new(vec![]))
                    })
                } else {
                    tracing::info!("Using multi-provider authentication");
                    Arc::new(MultiProvider::new(providers))
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
            let (audit_logger, _audit_handle) = AuditLogger::with_tasks(&config.audit)?;
            let audit_logger = Arc::new(audit_logger);

            // Set up transport/router based on configuration
            let (transport, router): (Option<Arc<dyn mcp_guard::transport::Transport>>, Option<Arc<ServerRouter>>) =
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
                    let transport: Arc<dyn mcp_guard::transport::Transport> = match &config.upstream.transport
                    {
                        mcp_guard::config::TransportType::Stdio => {
                            let command = config
                                .upstream
                                .command
                                .as_ref()
                                .ok_or_else(|| anyhow::anyhow!("stdio transport requires 'command' in config"))?;
                            tracing::info!(command = %command, "Using stdio transport");
                            Arc::new(StdioTransport::spawn(command, &config.upstream.args).await?)
                        }
                        mcp_guard::config::TransportType::Http => {
                            let url = config
                                .upstream
                                .url
                                .as_ref()
                                .ok_or_else(|| anyhow::anyhow!("HTTP transport requires 'url' in config"))?
                                .clone();
                            tracing::info!(url = %url, "Using HTTP transport");
                            Arc::new(HttpTransport::new(url))
                        }
                        mcp_guard::config::TransportType::Sse => {
                            let url = config
                                .upstream
                                .url
                                .as_ref()
                                .ok_or_else(|| anyhow::anyhow!("SSE transport requires 'url' in config"))?
                                .clone();
                            tracing::info!(url = %url, "Using SSE transport");
                            Arc::new(SseTransport::connect(url).await?)
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
            });

            // Run server
            server::run(state).await?;
        }
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

    let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
    let stdout = child.stdout.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdout"))?;
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
                    server_info.get("name").and_then(|v| v.as_str()).unwrap_or("unknown"),
                    server_info.get("version").and_then(|v| v.as_str()).unwrap_or("unknown")
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
        println!("Content-Type: {}", content_type.to_str().unwrap_or("unknown"));
    }

    Ok(())
}
