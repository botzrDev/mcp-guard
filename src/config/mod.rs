//! Configuration types and parsing for mcp-guard
//!
//! This module provides strongly-typed configuration for all mcp-guard features:
//! - Server settings (host, port, TLS)
//! - Authentication (API keys, JWT, OAuth 2.1, mTLS)
//! - Rate limiting (per-identity token bucket)
//! - Audit logging (file, stdout, HTTP export)
//! - Tracing (OpenTelemetry/OTLP)
//! - Upstream routing (single server or multi-server)
//!
//! Configuration can be loaded from TOML or YAML files via [`Config::from_file`].

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ============================================================================
// Error Types
// ============================================================================

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

// ============================================================================
// Core Configuration
// ============================================================================

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

    /// Maximum request body size in bytes (default: 1MB)
    /// Requests exceeding this size will receive 413 Payload Too Large
    #[serde(default = "default_max_request_size")]
    pub max_request_size: usize,

    /// CORS configuration
    #[serde(default)]
    pub cors: CorsConfig,

    /// Enable TLS
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_request_size: default_max_request_size(),
            cors: CorsConfig::default(),
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

fn default_max_request_size() -> usize {
    1024 * 1024 // 1MB default
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS (default: false for API-only use)
    #[serde(default)]
    pub enabled: bool,

    /// Allowed origins (default: none - same-origin only when enabled)
    /// Use ["*"] for permissive mode (not recommended for production)
    #[serde(default)]
    pub allowed_origins: Vec<String>,

    /// Allowed methods (default: GET, POST, OPTIONS)
    #[serde(default = "default_cors_methods")]
    pub allowed_methods: Vec<String>,

    /// Allowed headers (default: Authorization, Content-Type)
    #[serde(default = "default_cors_headers")]
    pub allowed_headers: Vec<String>,

    /// Max age for preflight cache in seconds (default: 3600)
    #[serde(default = "default_cors_max_age")]
    pub max_age: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: vec![],
            allowed_methods: default_cors_methods(),
            allowed_headers: default_cors_headers(),
            max_age: default_cors_max_age(),
        }
    }
}

fn default_cors_methods() -> Vec<String> {
    vec!["GET".into(), "POST".into(), "OPTIONS".into()]
}

fn default_cors_headers() -> Vec<String> {
    vec!["Authorization".into(), "Content-Type".into()]
}

fn default_cors_max_age() -> u64 {
    3600 // 1 hour
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Path to server certificate (PEM format)
    pub cert_path: PathBuf,
    /// Path to server private key (PEM format)
    pub key_path: PathBuf,
    /// Path to CA certificate for client certificate validation (mTLS)
    /// If set, client certificates will be required and validated against this CA
    pub client_ca_path: Option<PathBuf>,
}

/// mTLS authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtlsConfig {
    /// Whether to enable mTLS authentication
    #[serde(default)]
    pub enabled: bool,
    /// Claim to extract user ID from (CN or SAN)
    /// Default: "cn" (Common Name)
    #[serde(default = "default_mtls_identity_source")]
    pub identity_source: MtlsIdentitySource,
    /// Allowed tools for mTLS-authenticated identities (empty means all)
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    /// Custom rate limit for mTLS-authenticated identities
    #[serde(default)]
    pub rate_limit: Option<u32>,
    /// Trusted proxy IP addresses/CIDR ranges that are allowed to set mTLS headers
    /// SECURITY: If empty, mTLS header authentication is DISABLED to prevent header spoofing
    /// You MUST configure this when using mTLS with a reverse proxy.
    /// Example: ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16", "127.0.0.1"]
    #[serde(default)]
    pub trusted_proxy_ips: Vec<String>,
}

impl Default for MtlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            identity_source: default_mtls_identity_source(),
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        }
    }
}

/// Source for extracting identity from client certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MtlsIdentitySource {
    /// Extract from Common Name (CN)
    Cn,
    /// Extract from Subject Alternative Name (SAN) - DNS name
    SanDns,
    /// Extract from Subject Alternative Name (SAN) - Email
    SanEmail,
}

fn default_mtls_identity_source() -> MtlsIdentitySource {
    MtlsIdentitySource::Cn
}

// ============================================================================
// Authentication Configuration
// ============================================================================

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

    /// mTLS client certificate authentication
    #[serde(default)]
    pub mtls: Option<MtlsConfig>,
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

    /// Token cache TTL in seconds (default: 300 = 5 minutes)
    ///
    /// SECURITY NOTE: Revoked tokens remain valid in the cache until they expire.
    /// Lower values provide faster revocation detection but increase OAuth provider load.
    /// Set to 0 to disable caching (not recommended for production).
    #[serde(default = "default_token_cache_ttl")]
    pub token_cache_ttl_secs: u64,
}

fn default_token_cache_ttl() -> u64 {
    300 // 5 minutes
}

fn default_redirect_uri() -> String {
    "http://localhost:3000/oauth/callback".to_string()
}

fn default_oauth_scopes() -> Vec<String> {
    vec!["openid".to_string(), "profile".to_string()]
}

// ============================================================================
// Rate Limiting Configuration
// ============================================================================

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

    /// Per-tool rate limits (optional)
    /// Apply stricter limits to specific tools matched by glob patterns
    #[serde(default)]
    pub tool_limits: Vec<ToolRateLimitConfig>,
}

/// Per-tool rate limit configuration
///
/// Allows applying stricter rate limits to expensive or dangerous operations
/// using glob patterns to match tool names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRateLimitConfig {
    /// Glob pattern to match tool names (e.g., "execute_*", "write_*", "delete_*")
    pub tool_pattern: String,

    /// Maximum requests per second for matched tools
    pub requests_per_second: u32,

    /// Burst size for matched tools
    #[serde(default = "default_tool_burst")]
    pub burst_size: u32,
}

fn default_tool_burst() -> u32 {
    5 // Conservative burst for per-tool limits
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: default_rps(),
            burst_size: default_burst(),
            tool_limits: Vec::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_rps() -> u32 {
    25 // Conservative default - 25 RPS per identity
}

fn default_burst() -> u32 {
    10 // Conservative default burst size
}

// ============================================================================
// Audit Configuration
// ============================================================================

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

    /// HTTP export URL for SIEM integration (e.g., "https://siem.example.com/logs")
    /// If set, audit logs will be batched and sent to this endpoint
    #[serde(default)]
    pub export_url: Option<String>,

    /// Number of logs to batch before sending (default: 100)
    #[serde(default = "default_export_batch_size")]
    pub export_batch_size: usize,

    /// Interval in seconds to flush logs even if batch is not full (default: 30)
    #[serde(default = "default_export_interval_secs")]
    pub export_interval_secs: u64,

    /// Additional headers to include in export requests (e.g., for authentication)
    #[serde(default)]
    pub export_headers: HashMap<String, String>,

    /// Secret redaction rules to prevent sensitive data from being logged
    /// Each rule defines a regex pattern and replacement text
    #[serde(default)]
    pub redaction_rules: Vec<RedactionRule>,

    /// Log rotation configuration
    #[serde(default)]
    pub rotation: Option<LogRotationConfig>,
}

/// Secret redaction rule for audit logs
///
/// Matches sensitive data using regex patterns and replaces with safe text.
/// Patterns are applied in order, so more specific patterns should come first.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionRule {
    /// Rule name for logging/debugging (e.g., "bearer_tokens", "api_keys")
    pub name: String,

    /// Regex pattern to match sensitive data
    /// Uses Rust regex syntax: https://docs.rs/regex
    pub pattern: String,

    /// Replacement text (default: "[REDACTED]")
    #[serde(default = "default_redaction_replacement")]
    pub replacement: String,
}

fn default_redaction_replacement() -> String {
    "[REDACTED]".to_string()
}

/// Log rotation configuration
///
/// Prevents audit log files from growing indefinitely by rotating
/// based on size and/or age.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Enable log rotation
    #[serde(default)]
    pub enabled: bool,

    /// Maximum file size in bytes before rotation (e.g., 104857600 = 100MB)
    #[serde(default)]
    pub max_size_bytes: Option<u64>,

    /// Maximum age in seconds before rotation (e.g., 86400 = 1 day)
    #[serde(default)]
    pub max_age_secs: Option<u64>,

    /// Number of backup files to keep (default: 10)
    #[serde(default = "default_max_backups")]
    pub max_backups: usize,

    /// Compress rotated files with gzip
    #[serde(default)]
    pub compress: bool,
}

fn default_max_backups() -> usize {
    10
}

fn default_export_batch_size() -> usize {
    100
}

fn default_export_interval_secs() -> u64 {
    30
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file: None,
            // SECURITY: Default to false to prevent accidental PII exposure in logs.
            // Users should explicitly configure their log destination.
            stdout: false,
            export_url: None,
            export_batch_size: default_export_batch_size(),
            export_interval_secs: default_export_interval_secs(),
            export_headers: HashMap::new(),
            redaction_rules: Vec::new(),
            rotation: None,
        }
    }
}

// ============================================================================
// Tracing Configuration
// ============================================================================

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
    // SECURITY: Default to 10% sampling to avoid performance impact and cost
    // in production. Users can increase to 1.0 for development/debugging.
    0.1
}

// ============================================================================
// Upstream Configuration
// ============================================================================

/// Upstream MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    /// Transport type (used for single-server mode)
    pub transport: TransportType,

    /// Command to run (for stdio transport)
    pub command: Option<String>,

    /// Arguments for the command
    #[serde(default)]
    pub args: Vec<String>,

    /// URL for HTTP transport
    pub url: Option<String>,

    /// Multiple server routes (if configured, path-based routing is enabled)
    /// Requests are routed based on path prefix matching
    #[serde(default)]
    pub servers: Vec<ServerRouteConfig>,
}

/// Server route configuration for multi-server routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRouteConfig {
    /// Unique name for this server
    pub name: String,

    /// Path prefix to match (e.g., "/github", "/filesystem")
    /// Requests with this prefix are routed to this server
    pub path_prefix: String,

    /// Transport type for this server
    pub transport: TransportType,

    /// Command to run (for stdio transport)
    pub command: Option<String>,

    /// Arguments for the command
    #[serde(default)]
    pub args: Vec<String>,

    /// URL for HTTP/SSE transport
    pub url: Option<String>,

    /// Strip the path prefix when forwarding requests
    /// If true, "/github/repos" becomes "/repos" when sent to the server
    #[serde(default)]
    pub strip_prefix: bool,
}

/// Transport type for upstream connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    Stdio,
    Http,
    Sse,
}

// ============================================================================
// Implementation
// ============================================================================

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
        // Validate server port (must be 1-65535, not 0)
        if self.server.port == 0 {
            return Err(ConfigError::Validation(
                "server.port must be between 1 and 65535".to_string(),
            ));
        }

        // Validate rate limit settings
        if self.rate_limit.enabled {
            if self.rate_limit.requests_per_second == 0 {
                return Err(ConfigError::Validation(
                    "rate_limit.requests_per_second must be greater than 0".to_string(),
                ));
            }
            if self.rate_limit.burst_size == 0 {
                return Err(ConfigError::Validation(
                    "rate_limit.burst_size must be greater than 0".to_string(),
                ));
            }
        }

        // Validate JWT configuration
        if let Some(ref jwt_config) = self.auth.jwt {
            if let JwtMode::Jwks { ref jwks_url, .. } = jwt_config.mode {
                // JWKS URL must use HTTPS in production (allow HTTP in debug builds for local testing)
                #[cfg(not(debug_assertions))]
                if !jwks_url.starts_with("https://") {
                    return Err(ConfigError::Validation(
                        "jwt.jwks_url must use HTTPS in production".to_string(),
                    ));
                }
                // Validate URL format
                if !jwks_url.starts_with("http://") && !jwks_url.starts_with("https://") {
                    return Err(ConfigError::Validation(
                        "jwt.jwks_url must be a valid HTTP(S) URL".to_string(),
                    ));
                }
            }
        }

        // Validate OAuth configuration
        if let Some(ref oauth_config) = self.auth.oauth {
            // Validate redirect_uri is a valid URL
            if !oauth_config.redirect_uri.starts_with("http://")
                && !oauth_config.redirect_uri.starts_with("https://")
            {
                return Err(ConfigError::Validation(
                    "oauth.redirect_uri must be a valid HTTP(S) URL".to_string(),
                ));
            }
            // SECURITY: Warn about HTTP redirect_uri in production (allow in debug for local testing)
            #[cfg(not(debug_assertions))]
            if oauth_config.redirect_uri.starts_with("http://") {
                tracing::warn!(
                    "SECURITY WARNING: oauth.redirect_uri uses HTTP instead of HTTPS. \
                     This is insecure in production and may allow authorization code interception."
                );
            }
        }

        // Validate audit export configuration
        if let Some(ref export_url) = self.audit.export_url {
            // Validate URL format
            if !export_url.starts_with("http://") && !export_url.starts_with("https://") {
                return Err(ConfigError::Validation(
                    "audit.export_url must be a valid HTTP(S) URL".to_string(),
                ));
            }
            // Validate batch size
            if self.audit.export_batch_size == 0 {
                return Err(ConfigError::Validation(
                    "audit.export_batch_size must be greater than 0".to_string(),
                ));
            }
            if self.audit.export_batch_size > 10000 {
                return Err(ConfigError::Validation(
                    "audit.export_batch_size must be less than or equal to 10000".to_string(),
                ));
            }
            // Validate flush interval
            if self.audit.export_interval_secs == 0 {
                return Err(ConfigError::Validation(
                    "audit.export_interval_secs must be greater than 0".to_string(),
                ));
            }
        }

        // Validate mTLS configuration
        if let Some(ref mtls_config) = self.auth.mtls {
            if mtls_config.enabled && mtls_config.trusted_proxy_ips.is_empty() {
                // SECURITY: mTLS without trusted proxy IPs allows header spoofing
                return Err(ConfigError::Validation(
                    "auth.mtls.trusted_proxy_ips must be configured when mTLS is enabled. \
                     Without trusted proxy IPs, attackers could spoof client certificate headers."
                        .to_string(),
                ));
            }
        }

        // Validate tracing sample rate
        if self.tracing.enabled
            && (self.tracing.sample_rate < 0.0 || self.tracing.sample_rate > 1.0)
        {
            return Err(ConfigError::Validation(
                "tracing.sample_rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        // If multi-server routing is configured, validate each server
        if !self.upstream.servers.is_empty() {
            for server in &self.upstream.servers {
                server.validate()?;
            }
            return Ok(());
        }

        // Single-server mode validation
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

    /// Check if multi-server routing is enabled
    pub fn is_multi_server(&self) -> bool {
        !self.upstream.servers.is_empty()
    }
}

impl ServerRouteConfig {
    /// Validate the server route configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.name.is_empty() {
            return Err(ConfigError::Validation(
                "Server route 'name' cannot be empty".to_string(),
            ));
        }

        if self.path_prefix.is_empty() {
            return Err(ConfigError::Validation(format!(
                "Server route '{}' path_prefix cannot be empty",
                self.name
            )));
        }

        if !self.path_prefix.starts_with('/') {
            return Err(ConfigError::Validation(format!(
                "Server route '{}' path_prefix must start with '/'",
                self.name
            )));
        }

        match self.transport {
            TransportType::Stdio => {
                if self.command.is_none() {
                    return Err(ConfigError::Validation(format!(
                        "Server route '{}' with stdio transport requires 'command' to be set",
                        self.name
                    )));
                }
            }
            TransportType::Http | TransportType::Sse => {
                if self.url.is_none() {
                    return Err(ConfigError::Validation(format!(
                        "Server route '{}' with http/sse transport requires 'url' to be set",
                        self.name
                    )));
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_config() -> Config {
        Config {
            server: ServerConfig::default(),
            auth: AuthConfig::default(),
            rate_limit: RateLimitConfig::default(),
            audit: AuditConfig::default(),
            tracing: TracingConfig::default(),
            upstream: UpstreamConfig {
                transport: TransportType::Http,
                command: None,
                args: vec![],
                url: Some("http://localhost:8080".to_string()),
                servers: vec![],
            },
        }
    }

    // ------------------------------------------------------------------------
    // Default Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_server_config_defaults() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.tls.is_none());
    }

    #[test]
    fn test_rate_limit_config_defaults() {
        let config = RateLimitConfig::default();
        assert!(config.enabled);
        // SECURITY: Conservative defaults (25 RPS, burst 10) to limit abuse
        assert_eq!(config.requests_per_second, 25);
        assert_eq!(config.burst_size, 10);
    }

    #[test]
    fn test_audit_config_defaults() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert!(config.file.is_none());
        // SECURITY: stdout defaults to false to prevent accidental PII exposure
        assert!(!config.stdout);
        assert!(config.export_url.is_none());
        assert_eq!(config.export_batch_size, 100);
        assert_eq!(config.export_interval_secs, 30);
    }

    #[test]
    fn test_tracing_config_defaults() {
        let config = TracingConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.service_name, "mcp-guard");
        assert!(config.otlp_endpoint.is_none());
        // SECURITY: sample_rate defaults to 0.1 (10%) for production safety
        assert_eq!(config.sample_rate, 0.1);
        assert!(config.propagate_context);
    }

    #[test]
    fn test_mtls_config_defaults() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(matches!(config.identity_source, MtlsIdentitySource::Cn));
        assert!(config.allowed_tools.is_empty());
        assert!(config.rate_limit.is_none());
    }

    // ------------------------------------------------------------------------
    // Validation Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_config_validation_success() {
        let config = create_valid_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_rate_limit_zero_rps() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.requests_per_second = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_rate_limit_zero_burst() {
        let mut config = create_valid_config();
        config.rate_limit.enabled = true;
        config.rate_limit.burst_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_stdio_missing_command() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Stdio;
        config.upstream.command = None;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_http_missing_url() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Http;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_sse_missing_url() {
        let mut config = create_valid_config();
        config.upstream.transport = TransportType::Sse;
        config.upstream.url = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_jwt_invalid_jwks_url() {
        let mut config = create_valid_config();
        config.auth.jwt = Some(JwtConfig {
            mode: JwtMode::Jwks {
                jwks_url: "invalid-url".to_string(),
                algorithms: default_jwks_algorithms(),
                cache_duration_secs: 3600,
            },
            issuer: "https://issuer.example.com".to_string(),
            audience: "mcp-guard".to_string(),
            user_id_claim: "sub".to_string(),
            scopes_claim: "scope".to_string(),
            scope_tool_mapping: HashMap::new(),
            leeway_secs: 0,
        });
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_oauth_invalid_redirect_uri() {
        let mut config = create_valid_config();
        config.auth.oauth = Some(OAuthConfig {
            provider: OAuthProvider::GitHub,
            client_id: "test".to_string(),
            client_secret: None,
            authorization_url: None,
            token_url: None,
            introspection_url: None,
            userinfo_url: None,
            redirect_uri: "invalid-uri".to_string(),
            scopes: vec![],
            user_id_claim: "sub".to_string(),
            scope_tool_mapping: HashMap::new(),
            token_cache_ttl_secs: 300,
        });
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_audit_invalid_export_url() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("not-a-url".to_string());
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_audit_batch_size_zero() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_batch_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_audit_batch_size_too_large() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_batch_size = 10001;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_audit_interval_zero() {
        let mut config = create_valid_config();
        config.audit.export_url = Some("http://siem.example.com".to_string());
        config.audit.export_interval_secs = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_tracing_invalid_sample_rate() {
        let mut config = create_valid_config();
        config.tracing.enabled = true;
        config.tracing.sample_rate = 1.5;
        assert!(config.validate().is_err());

        config.tracing.sample_rate = -0.1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_mtls_requires_trusted_proxy_ips() {
        let mut config = create_valid_config();
        // mTLS enabled without trusted_proxy_ips should fail
        config.auth.mtls = Some(MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![], // Empty = security risk
        });
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("trusted_proxy_ips"));

        // mTLS enabled with trusted_proxy_ips should succeed
        config.auth.mtls = Some(MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.0/8".to_string()],
        });
        assert!(config.validate().is_ok());

        // mTLS disabled without trusted_proxy_ips should succeed (not used)
        config.auth.mtls = Some(MtlsConfig {
            enabled: false,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec![],
        });
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_is_multi_server() {
        let mut config = create_valid_config();
        assert!(!config.is_multi_server());

        config.upstream.servers.push(ServerRouteConfig {
            name: "test".to_string(),
            path_prefix: "/test".to_string(),
            transport: TransportType::Http,
            command: None,
            args: vec![],
            url: Some("http://localhost:8080".to_string()),
            strip_prefix: false,
        });
        assert!(config.is_multi_server());
    }

    // ------------------------------------------------------------------------
    // ConfigError Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::Parse("invalid TOML".to_string());
        assert!(format!("{}", err).contains("invalid TOML"));

        let err = ConfigError::Validation("port must be > 0".to_string());
        assert!(format!("{}", err).contains("port must be > 0"));
    }

    // ------------------------------------------------------------------------
    // Transport Type Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_transport_type_serialization() {
        let json = serde_json::to_string(&TransportType::Stdio).unwrap();
        assert!(json.contains("stdio"));

        let json = serde_json::to_string(&TransportType::Http).unwrap();
        assert!(json.contains("http"));

        let json = serde_json::to_string(&TransportType::Sse).unwrap();
        assert!(json.contains("sse"));
    }

    // ------------------------------------------------------------------------
    // OAuth Provider Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_oauth_provider_serialization() {
        let provider = OAuthProvider::GitHub;
        let json = serde_json::to_string(&provider).unwrap();
        assert!(json.contains("github"));

        let provider = OAuthProvider::Google;
        let json = serde_json::to_string(&provider).unwrap();
        assert!(json.contains("google"));
    }
}
