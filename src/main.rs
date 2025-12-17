//! MCP Guard - Security gateway for MCP servers

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use mcp_guard::{
    audit::{AuditLoggerHandle, AuditLogger},
    auth::{ApiKeyProvider, AuthProvider, JwtProvider, MtlsAuthProvider, MultiProvider, OAuthAuthProvider},
    cli::{generate_api_key, generate_config, hash_api_key, Cli, Commands},
    config::Config,
    observability::{init_metrics, init_tracing},
    rate_limit::RateLimitService,
    router::ServerRouter,
    server::{self, new_oauth_state_store, AppState},
    transport::{HttpTransport, SseTransport, StdioTransport},
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
            // Start background refresh for JWKS mode with shutdown coordination
            jwt_provider.start_background_refresh(shutdown_token.clone());
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
    let (audit_logger, audit_handle) = AuditLogger::with_tasks(&config.audit)?;
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
                    #[cfg(test)]
                    let transport = HttpTransport::new_unchecked(url);
                    #[cfg(not(test))]
                    let transport = HttpTransport::new(url)
                        .map_err(|e| anyhow::anyhow!("Failed to create HTTP transport: {}", e))?;
                    Arc::new(transport)
                }
                mcp_guard::config::TransportType::Sse => {
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
    });

    Ok(BootstrapResult {
        state,
        audit_handle,
        shutdown_token,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();
    if let Err(e) = run_cli(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

async fn run_cli(cli: Cli) -> anyhow::Result<()> {
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
                anyhow::bail!("{} already exists. Use --force to overwrite.", filename);
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
                    anyhow::bail!("Configuration error: {}", e);
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
            let config = Config::from_file(&cli.config)
                .map_err(|e| anyhow::anyhow!("Error loading config: {}", e))?;

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
                            anyhow::bail!("✗ Upstream check failed: {}", e);
                        }
                        Err(_) => {
                            anyhow::bail!("✗ Upstream check timed out after {}s", timeout);
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
                            anyhow::bail!("✗ Upstream check failed: {}", e);
                        }
                        Err(_) => {
                            anyhow::bail!("✗ Upstream check timed out after {}s", timeout);
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
                            anyhow::bail!("✗ Upstream check failed: {}", e);
                        }
                        Err(_) => {
                            anyhow::bail!("✗ Upstream check timed out after {}s", timeout);
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

            // Bootstrap the server state
            let BootstrapResult { state, audit_handle, shutdown_token } = bootstrap(config).await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Helper to create a minimal valid config for testing
    fn create_test_config_http(url: &str) -> Config {
        let config_str = format!(r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false
"#, url);
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

    #[tokio::test]
    async fn test_bootstrap_with_http_transport() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::any;
        
        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"result": {}})))
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
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::any;
        
        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config_str = format!(r#"
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "http"
url = "{}"

[rate_limit]
enabled = false

[[auth.api_keys]]
id = "test-user"
key_hash = "abc123"
"#, mock_server.uri());
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &config_str).unwrap();
        let config = Config::from_file(&temp_file.path().to_path_buf()).unwrap();
        
        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());
        
        let bootstrap_result = result.unwrap();
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }

    #[tokio::test]
    async fn test_bootstrap_with_no_auth() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::any;
        
        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let config = create_test_config_http(&mock_server.uri());
        
        let result = bootstrap(config).await;
        assert!(result.is_ok(), "bootstrap failed: {:?}", result.err());
        
        let bootstrap_result = result.unwrap();
        bootstrap_result.shutdown_token.cancel();
        bootstrap_result.audit_handle.shutdown().await;
    }


    #[tokio::test]
    async fn test_check_http_upstream_success() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::method;
        
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
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::method;
        
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
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::method;
        
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "text/event-stream")
            )
            .mount(&mock_server)
            .await;

        let result = check_sse_upstream(&mock_server.uri()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_sse_upstream_no_content_type() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::method;
        
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
}
