//! MCP Guard - Security gateway for MCP servers

use std::sync::Arc;

use mcp_guard::{
    audit::AuditLogger,
    auth::{ApiKeyProvider, AuthProvider, JwtProvider, MultiProvider},
    cli::{generate_api_key, generate_config, hash_api_key, Cli, Commands},
    config::Config,
    observability::init_tracing,
    rate_limit::RateLimitService,
    server::{self, AppState},
    transport::StdioTransport,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_args();

    // Initialize tracing
    init_tracing(cli.verbose);

    match cli.command {
        Commands::Init { format, force } => {
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
            let hash = hash_api_key(&key);
            println!("{}", hash);
        }

        Commands::Run { host, port } => {
            // Load configuration
            let mut config = Config::from_file(&cli.config)?;

            // Override with CLI args
            if let Some(h) = host {
                config.server.host = h;
            }
            if let Some(p) = port {
                config.server.port = p;
            }

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

                // Select appropriate provider setup
                match providers.len() {
                    0 => {
                        tracing::warn!("No authentication providers configured - all requests will be rejected");
                        Arc::new(ApiKeyProvider::new(vec![])) // Deny all
                    }
                    1 => providers.pop().unwrap(),
                    _ => {
                        tracing::info!("Using multi-provider authentication");
                        Arc::new(MultiProvider::new(providers))
                    }
                }
            };

            // Set up rate limiter
            let rate_limiter = RateLimitService::new(&config.rate_limit);

            // Set up audit logger
            let audit_logger = Arc::new(AuditLogger::new(&config.audit)?);

            // Set up transport to upstream MCP server
            let transport: Arc<dyn mcp_guard::transport::Transport> = match &config.upstream.transport
            {
                mcp_guard::config::TransportType::Stdio => {
                    let command = config
                        .upstream
                        .command
                        .as_ref()
                        .expect("command required for stdio transport");
                    Arc::new(StdioTransport::spawn(command, &config.upstream.args).await?)
                }
                _ => {
                    anyhow::bail!("HTTP/SSE transport not yet implemented");
                }
            };

            // Create application state
            let state = Arc::new(AppState {
                config,
                auth_provider,
                rate_limiter,
                audit_logger,
                transport,
            });

            // Run server
            server::run(state).await?;
        }
    }

    Ok(())
}
