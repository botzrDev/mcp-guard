//! Configuration types and parsing for mcp-guard

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration error type
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Read(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Main configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    #[serde(default)]
    pub server: ServerConfig,

    /// Authentication configuration
    #[serde(default)]
    pub auth: AuthConfig,

    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limit: RateLimitConfig,

    /// Audit logging configuration
    #[serde(default)]
    pub audit: AuditConfig,

    /// OpenTelemetry tracing configuration
    #[serde(default)]
    pub tracing: TracingConfig,

    /// Upstream MCP server configuration
    pub upstream: UpstreamConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,

    /// Port to listen on
    #[serde(default = "default_port")]
    pub port: u16,

    /// Enable TLS
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            tls: None,
        }
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

/// Authentication configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthConfig {
    /// API key authentication
    #[serde(default)]
    pub api_keys: Vec<ApiKeyConfig>,

    /// JWT authentication
    #[serde(default)]
    pub jwt: Option<JwtConfig>,

    /// OAuth 2.1 configuration
    #[serde(default)]
    pub oauth: Option<OAuthConfig>,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// User/service identifier
    pub id: String,

    /// The hashed API key
    pub key_hash: String,

    /// Allowed tools (empty means all)
    #[serde(default)]
    pub allowed_tools: Vec<String>,

    /// Custom rate limit (overrides global)
    #[serde(default)]
    pub rate_limit: Option<u32>,
}

/// JWT authentication mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum JwtMode {
    /// Simple mode: HS256 with local secret
    Simple {
        /// Shared secret for HS256 signing (min 32 characters recommended)
        secret: String,
    },
    /// JWKS mode: RS256/ES256 with remote JWKS endpoint
    Jwks {
        /// JWKS endpoint URL
        jwks_url: String,
        /// Allowed algorithms (default: ["RS256", "ES256"])
        #[serde(default = "default_jwks_algorithms")]
        algorithms: Vec<String>,
        /// JWKS cache duration in seconds (default: 3600 = 1 hour)
        #[serde(default = "default_cache_duration")]
        cache_duration_secs: u64,
    },
}

/// JWT configuration supporting both simple and JWKS modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT validation mode (simple or jwks)
    #[serde(flatten)]
    pub mode: JwtMode,

    /// Expected issuer (iss claim) - required for validation
    pub issuer: String,

    /// Expected audience (aud claim) - required for validation
    pub audience: String,

    /// Claim to extract user ID from (default: "sub")
    #[serde(default = "default_user_id_claim")]
    pub user_id_claim: String,

    /// Claim to extract scopes from (default: "scope")
    #[serde(default = "default_scopes_claim")]
    pub scopes_claim: String,

    /// Mapping from scopes to allowed tools
    /// e.g., {"read:files": ["read_file", "list_files"], "admin": ["*"]}
    #[serde(default)]
    pub scope_tool_mapping: HashMap<String, Vec<String>>,

    /// Leeway in seconds for exp/nbf validation (default: 0)
    #[serde(default)]
    pub leeway_secs: u64,
}

fn default_jwks_algorithms() -> Vec<String> {
    vec!["RS256".to_string(), "ES256".to_string()]
}

fn default_cache_duration() -> u64 {
    3600 // 1 hour
}

fn default_user_id_claim() -> String {
    "sub".to_string()
}

fn default_scopes_claim() -> String {
    "scope".to_string()
}

/// OAuth 2.1 provider type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    /// GitHub OAuth
    GitHub,
    /// Google OAuth
    Google,
    /// Okta OAuth
    Okta,
    /// Custom OAuth provider
    Custom,
}

/// OAuth 2.1 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// OAuth provider type
    pub provider: OAuthProvider,

    /// Client ID
    pub client_id: String,

    /// Client secret (for confidential clients)
    pub client_secret: Option<String>,

    /// Authorization endpoint URL (required for custom, auto-derived for known providers)
    pub authorization_url: Option<String>,

    /// Token endpoint URL (required for custom, auto-derived for known providers)
    pub token_url: Option<String>,

    /// Token introspection endpoint URL (for validating opaque tokens)
    pub introspection_url: Option<String>,

    /// User info endpoint URL (fallback if no introspection)
    pub userinfo_url: Option<String>,

    /// Redirect URI for authorization code flow
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,

    /// OAuth scopes to request
    #[serde(default = "default_oauth_scopes")]
    pub scopes: Vec<String>,

    /// Claim to extract user ID from (default: "sub")
    #[serde(default = "default_user_id_claim")]
    pub user_id_claim: String,

    /// Mapping from scopes to allowed tools (same as JWT)
    #[serde(default)]
    pub scope_tool_mapping: HashMap<String, Vec<String>>,
}

fn default_redirect_uri() -> String {
    "http://localhost:3000/oauth/callback".to_string()
}

fn default_oauth_scopes() -> Vec<String> {
    vec!["openid".to_string(), "profile".to_string()]
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Requests per second
    #[serde(default = "default_rps")]
    pub requests_per_second: u32,

    /// Burst size
    #[serde(default = "default_burst")]
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: default_rps(),
            burst_size: default_burst(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_rps() -> u32 {
    100
}

fn default_burst() -> u32 {
    50
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Log file path
    #[serde(default)]
    pub file: Option<PathBuf>,

    /// Log to stdout
    #[serde(default)]
    pub stdout: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file: None,
            stdout: true,
        }
    }
}

/// OpenTelemetry tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable OpenTelemetry distributed tracing
    #[serde(default)]
    pub enabled: bool,

    /// Service name for traces (default: "mcp-guard")
    #[serde(default = "default_service_name")]
    pub service_name: String,

    /// OTLP exporter endpoint (e.g., "http://localhost:4317" for gRPC)
    /// If not set, traces are only logged locally
    pub otlp_endpoint: Option<String>,

    /// Sample rate (0.0 to 1.0, default: 1.0 = sample all)
    #[serde(default = "default_sample_rate")]
    pub sample_rate: f64,

    /// Propagate W3C trace context headers (traceparent, tracestate)
    #[serde(default = "default_true")]
    pub propagate_context: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            service_name: default_service_name(),
            otlp_endpoint: None,
            sample_rate: default_sample_rate(),
            propagate_context: true,
        }
    }
}

fn default_service_name() -> String {
    "mcp-guard".to_string()
}

fn default_sample_rate() -> f64 {
    1.0
}

/// Upstream MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    /// Transport type
    pub transport: TransportType,

    /// Command to run (for stdio transport)
    pub command: Option<String>,

    /// Arguments for the command
    #[serde(default)]
    pub args: Vec<String>,

    /// URL for HTTP transport
    pub url: Option<String>,
}

/// Transport type for upstream connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    Stdio,
    Http,
    Sse,
}

impl Config {
    /// Load configuration from a file
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;

        let config: Config = if path.extension().map(|e| e == "yaml" || e == "yml").unwrap_or(false) {
            serde_yaml::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))?
        } else {
            toml::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))?
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        match self.upstream.transport {
            TransportType::Stdio => {
                if self.upstream.command.is_none() {
                    return Err(ConfigError::Validation(
                        "stdio transport requires 'command' to be set".to_string(),
                    ));
                }
            }
            TransportType::Http | TransportType::Sse => {
                if self.upstream.url.is_none() {
                    return Err(ConfigError::Validation(
                        "http/sse transport requires 'url' to be set".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}
