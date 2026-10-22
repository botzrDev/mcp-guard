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

    /// Local tool policy (used by `mcp-guard serve`, the stdio proxy mode)
    #[serde(default)]
    pub policy: PolicyConfig,

    /// Flight recorder: tamper-evident per-session recording (serve mode)
    #[serde(default)]
    pub recorder: RecorderConfig,

    /// Database URL for persistent storage
    pub database_url: Option<String>,

    /// Stripe secret key for billing
    pub stripe_secret_key: Option<String>,
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

    /// HTTP export URL for SIEM integration (e.g., `<https://siem.example.com/logs>`)
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
    /// Uses Rust regex syntax: <https://docs.rs/regex>
    pub pattern: String,

    /// Replacement text (default: `"[REDACTED]"`)
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
// Local Policy Configuration (stdio proxy mode)
// ============================================================================

/// What to do when a pinned tool definition changes or a new tool appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PinMismatchAction {
    /// Hide the tool from `tools/list` and reject calls to it until re-pinned.
    Deny,
    /// Allow the tool but log a warning and an audit event.
    Warn,
}

/// Local tool policy applied by `mcp-guard serve` (the stdio proxy mode).
///
/// In serve mode there is no network caller to authenticate: the MCP client
/// launching mcp-guard *is* the user. What matters is which upstream tools the
/// agent may use and whether the upstream's tool definitions are still the
/// ones that were reviewed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Tools the agent may call. Glob patterns are supported (`read_*`, `fs/*`).
    /// `["*"]` allows every tool. Defaults to all tools.
    #[serde(default = "default_allow_all")]
    pub allowed_tools: Vec<String>,

    /// Tools the agent may never call, checked before `allowed_tools`.
    /// Glob patterns are supported (`delete_*`, `execute_*`).
    #[serde(default)]
    pub denied_tools: Vec<String>,

    /// Pin upstream tool definitions (name, description, input schema).
    ///
    /// On first use the definitions are written to the lockfile. Afterwards,
    /// any tool whose definition changed, or that was not present when pinned,
    /// is handled according to `on_pin_mismatch`. This defeats tool poisoning
    /// and rug-pull attacks where a server silently rewrites its tools.
    #[serde(default = "default_true")]
    pub pin_tools: bool,

    /// Path of the tool lockfile. Defaults to `<config>.tools.lock.json`
    /// next to the configuration file.
    #[serde(default)]
    pub tools_lock: Option<PathBuf>,

    /// Action for tools that are new or changed since pinning.
    #[serde(default = "default_pin_mismatch_action")]
    pub on_pin_mismatch: PinMismatchAction,

    /// `enforce` (default) blocks what the policy denies. `observe` allows
    /// everything but records what *would* have been denied, so a policy can
    /// be tried, or generated with `mcp-guard policy suggest`, before it bites.
    #[serde(default)]
    pub mode: PolicyMode,

    /// Argument-level rules, evaluated in order on `tools/call` after
    /// `denied_tools` and `allowed_tools`. The first rule whose `tools` match
    /// and whose `when` conditions all hold decides; no match means allow.
    #[serde(default)]
    pub rules: Vec<PolicyRule>,

    /// Human-in-the-loop settings for rules with `action = "hold"`.
    #[serde(default)]
    pub holds: HoldsConfig,
}

/// Whether policy decisions are enforced or only recorded.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyMode {
    #[default]
    Enforce,
    Observe,
}

/// What a matching rule does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Deny,
    /// Pause the call until a person approves it with `mcp-guard approve`,
    /// or deny it when nobody does within `policy.holds.timeout_secs`.
    Hold,
}

/// Settings for held calls (human-in-the-loop approval).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldsConfig {
    /// Directory where pending holds and their approvals live.
    /// Default: `<config stem>.holds/` next to the config file.
    #[serde(default)]
    pub dir: Option<PathBuf>,

    /// How long a held call waits for approval before it is denied.
    #[serde(default = "default_hold_timeout_secs")]
    pub timeout_secs: u64,

    /// Show a desktop notification when a call is held (macOS `osascript`,
    /// Linux `notify-send`, Windows PowerShell toast). Best effort.
    #[serde(default = "default_true")]
    pub notify: bool,

    /// Command to run when a call is held, instead of the desktop notification.
    /// Placeholders: `{id}`, `{tool}`, `{reason}`, `{arguments}`, `{approve}`.
    #[serde(default)]
    pub notify_command: Option<Vec<String>>,
}

fn default_hold_timeout_secs() -> u64 {
    120
}

impl Default for HoldsConfig {
    fn default() -> Self {
        Self {
            dir: None,
            timeout_secs: default_hold_timeout_secs(),
            notify: true,
            notify_command: None,
        }
    }
}

impl HoldsConfig {
    /// Resolve the holds directory for a given config file path.
    pub fn dir_for(&self, config_path: &std::path::Path) -> PathBuf {
        resolve_sibling_dir(self.dir.as_deref(), config_path, "holds")
    }
}

/// Resolve `explicit` relative to the config file's directory, or build
/// `<config stem>.<suffix>/` next to the config file.
pub(crate) fn resolve_sibling_dir(
    explicit: Option<&std::path::Path>,
    config_path: &std::path::Path,
    suffix: &str,
) -> PathBuf {
    let parent = config_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf());
    if let Some(explicit) = explicit {
        if explicit.is_absolute() {
            return explicit.to_path_buf();
        }
        return match parent {
            Some(p) => p.join(explicit),
            None => explicit.to_path_buf(),
        };
    }
    let stem = config_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("mcp-guard");
    let dir = format!("{}.{}", stem, suffix);
    match parent {
        Some(p) => p.join(dir),
        None => PathBuf::from(dir),
    }
}

/// One argument-level rule.
///
/// ```toml
/// [[policy.rules]]
/// name = "writes stay inside the workspace"
/// tools = ["write_file", "edit_file"]
/// action = "deny"
/// [policy.rules.when]
/// path = { not_starts_with = "/home/me/project/" }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Optional label, shown in denials and audit entries.
    #[serde(default)]
    pub name: Option<String>,

    /// Tool name globs this rule applies to. Default: every tool.
    #[serde(default = "default_allow_all")]
    pub tools: Vec<String>,

    /// What to do when the rule matches.
    pub action: RuleAction,

    /// Optional explanation returned to the agent on denial.
    #[serde(default)]
    pub reason: Option<String>,

    /// Conditions on tool arguments, keyed by dotted argument path
    /// (`path`, `options.recursive`, `files.0.name`). All must hold.
    /// An empty map matches every call.
    #[serde(default)]
    pub when: std::collections::BTreeMap<String, ArgCondition>,
}

/// One string or a list of strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            OneOrMany::One(v) => std::slice::from_ref(v).iter(),
            OneOrMany::Many(v) => v.iter(),
        }
    }
}

/// Conditions on one argument value. Every field that is set must hold.
///
/// String operators apply to string arguments only; a non-string value never
/// matches them. Numeric operators apply to numbers. `exists = false` is the
/// only operator that holds when the argument is absent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArgCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<serde_json::Value>,
    #[serde(default, rename = "in", skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<serde_json::Value>>,
    /// Holds when the string starts with any of these.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<OneOrMany<String>>,
    /// Holds when the string starts with none of these.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_starts_with: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_with: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_ends_with: Option<OneOrMany<String>>,
    /// Substring match, any of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_contains: Option<OneOrMany<String>>,
    /// Glob match (`*`, `?`, `[..]`), any of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glob: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_glob: Option<OneOrMany<String>>,
    /// Regular expression (Rust `regex` syntax), unanchored, any of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_regex: Option<OneOrMany<String>>,
    /// Maximum string length in characters (or array length).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
}

fn default_allow_all() -> Vec<String> {
    vec!["*".to_string()]
}

fn default_pin_mismatch_action() -> PinMismatchAction {
    PinMismatchAction::Deny
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            allowed_tools: default_allow_all(),
            denied_tools: Vec::new(),
            pin_tools: true,
            tools_lock: None,
            on_pin_mismatch: default_pin_mismatch_action(),
            mode: PolicyMode::Enforce,
            rules: Vec::new(),
            holds: HoldsConfig::default(),
        }
    }
}

impl PolicyConfig {
    /// Resolve the lockfile path for a given config file path.
    pub fn lock_path_for(&self, config_path: &std::path::Path) -> PathBuf {
        if let Some(explicit) = &self.tools_lock {
            if explicit.is_absolute() {
                return explicit.clone();
            }
            if let Some(dir) = config_path.parent() {
                return dir.join(explicit);
            }
            return explicit.clone();
        }
        let stem = config_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("mcp-guard");
        let file = format!("{}.tools.lock.json", stem);
        match config_path.parent() {
            Some(dir) => dir.join(file),
            None => PathBuf::from(file),
        }
    }

    /// Whether a tool is denied by `denied_tools`.
    pub fn is_denied(&self, tool_name: &str) -> bool {
        self.denied_tools.iter().any(|p| glob_match(p, tool_name))
    }

    /// Whether a tool passes `denied_tools` and `allowed_tools`.
    pub fn allows(&self, tool_name: &str) -> bool {
        if self.is_denied(tool_name) {
            return false;
        }
        self.allowed_tools.iter().any(|p| glob_match(p, tool_name))
    }
}

/// Glob match with the same semantics as `authz::authorize_tool_call`.
pub fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == name || pattern == "*" {
        return true;
    }
    if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
        if let Ok(p) = glob::Pattern::new(pattern) {
            return p.matches(name);
        }
    }
    false
}

// ============================================================================
// Flight Recorder Configuration (stdio proxy mode)
// ============================================================================

/// Flight recorder settings. See `docs/flight-recorder.md`.
///
/// Every message the agent sends through the proxy, every response it gets
/// back, and every guard decision is appended to a per-session JSONL file.
/// Each record carries the hash of the previous one, so edits and deletions
/// are detectable with `mcp-guard verify`, and `mcp-guard replay` renders a
/// session as a timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecorderConfig {
    /// Record sessions. Off by default; `mcp-guard wrap` turns it on.
    #[serde(default)]
    pub enabled: bool,

    /// Directory for session files. Default: `<config stem>.recordings/`
    /// next to the config file.
    #[serde(default)]
    pub dir: Option<PathBuf>,

    /// Payloads larger than this (serialized JSON bytes) are stored as a
    /// preview plus the hash of the full payload.
    #[serde(default = "default_max_payload_bytes")]
    pub max_payload_bytes: usize,

    /// Record tool results and other responses, not only requests and decisions.
    #[serde(default = "default_true")]
    pub include_results: bool,

    /// Apply the built-in secret patterns and `[audit] redaction_rules` to
    /// payloads before they are written.
    #[serde(default = "default_true")]
    pub redact: bool,

    /// Environment variable holding an HMAC key. When set, record hashes are
    /// HMAC-SHA256 under that key instead of plain SHA-256, so a recording
    /// cannot be forged without the key. Keep the key away from the machine
    /// that writes the recordings if you need that guarantee.
    #[serde(default)]
    pub signing_key_env: Option<String>,

    /// Delete session files older than this many days when a new session starts.
    #[serde(default)]
    pub retention_days: Option<u32>,

    /// Keep at most this many session files (oldest deleted first) when a new
    /// session starts.
    #[serde(default)]
    pub max_sessions: Option<usize>,
}

fn default_max_payload_bytes() -> usize {
    64 * 1024
}

impl Default for RecorderConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            dir: None,
            max_payload_bytes: default_max_payload_bytes(),
            include_results: true,
            redact: true,
            signing_key_env: None,
            retention_days: None,
            max_sessions: None,
        }
    }
}

impl RecorderConfig {
    /// Resolve the recordings directory for a given config file path.
    pub fn dir_for(&self, config_path: &std::path::Path) -> PathBuf {
        resolve_sibling_dir(self.dir.as_deref(), config_path, "recordings")
    }

    /// The signing key from the environment, if configured and present.
    pub fn signing_key(&self) -> Option<Vec<u8>> {
        let var = self.signing_key_env.as_ref()?;
        let value = std::env::var(var).ok()?;
        if value.is_empty() {
            None
        } else {
            Some(value.into_bytes())
        }
    }
}

// ============================================================================
// Upstream Configuration
// ============================================================================

/// Upstream MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamConfig {
    /// Transport type (used for single-server mode). Defaults to stdio, so
    /// multi-server configs that only list `[[upstream.servers]]` are valid.
    #[serde(default)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    #[default]
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

        let config: Config = if path
            .extension()
            .map(|e| e == "yaml" || e == "yml")
            .unwrap_or(false)
        {
            serde_yaml::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))?
        } else {
            toml::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))?
        };

        // Apply environment variable overrides
        let mut config = config;
        config.apply_env_overrides();

        config.validate()?;
        Ok(config)
    }

    /// Apply environment variable overrides
    pub fn apply_env_overrides(&mut self) {
        use std::env;

        // OAuth overrides
        if let Some(oauth) = &mut self.auth.oauth {
            if let Ok(id) = env::var("MCP_GUARD_AUTH_OAUTH_CLIENT_ID") {
                oauth.client_id = id;
            }
            if let Ok(secret) = env::var("MCP_GUARD_AUTH_OAUTH_CLIENT_SECRET") {
                oauth.client_secret = Some(secret);
            }
            if let Ok(uri) = env::var("MCP_GUARD_AUTH_OAUTH_REDIRECT_URI") {
                oauth.redirect_uri = uri;
            }
        }

        // Database override
        if let Ok(url) = env::var("MCP_GUARD_DATABASE_URL") {
            self.database_url = Some(url);
        }

        // Stripe override
        if let Ok(key) = env::var("STRIPE_SECRET_KEY") {
            self.stripe_secret_key = Some(key);
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // First validate tier/license requirements
        crate::tier::validate_tier(self)?;

        // Then validate individual sections
        self.validate_server()?;
        self.validate_rate_limit()?;
        self.validate_jwt()?;
        self.validate_oauth()?;
        self.validate_audit()?;
        self.validate_mtls()?;
        self.validate_tracing()?;
        self.validate_policy()?;
        self.validate_upstream()
        // Database validation is handled at connection time
    }

    /// Validate the local policy: regexes and globs must compile.
    fn validate_policy(&self) -> Result<(), ConfigError> {
        for (i, rule) in self.policy.rules.iter().enumerate() {
            let label = rule
                .name
                .clone()
                .unwrap_or_else(|| format!("policy.rules[{}]", i));
            for (arg, cond) in &rule.when {
                for pattern in cond
                    .regex
                    .iter()
                    .chain(cond.not_regex.iter())
                    .flat_map(|p| p.iter())
                {
                    regex::Regex::new(pattern).map_err(|e| {
                        ConfigError::Validation(format!(
                            "{}: invalid regex for argument '{}': {}",
                            label, arg, e
                        ))
                    })?;
                }
                for pattern in cond
                    .glob
                    .iter()
                    .chain(cond.not_glob.iter())
                    .flat_map(|p| p.iter())
                {
                    glob::Pattern::new(pattern).map_err(|e| {
                        ConfigError::Validation(format!(
                            "{}: invalid glob for argument '{}': {}",
                            label, arg, e
                        ))
                    })?;
                }
                if *cond == ArgCondition::default() {
                    return Err(ConfigError::Validation(format!(
                        "{}: condition for argument '{}' has no operators",
                        label, arg
                    )));
                }
            }
        }
        Ok(())
    }

    // ========================================================================
    // Validation Helpers
    // ========================================================================

    /// Validate server configuration.
    fn validate_server(&self) -> Result<(), ConfigError> {
        if self.server.port == 0 {
            return Err(ConfigError::Validation(
                "server.port must be between 1 and 65535".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate rate limit configuration.
    fn validate_rate_limit(&self) -> Result<(), ConfigError> {
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
        Ok(())
    }

    /// Validate JWT configuration.
    fn validate_jwt(&self) -> Result<(), ConfigError> {
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
        Ok(())
    }

    /// Validate OAuth configuration.
    fn validate_oauth(&self) -> Result<(), ConfigError> {
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
        Ok(())
    }

    /// Validate audit configuration.
    fn validate_audit(&self) -> Result<(), ConfigError> {
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
        Ok(())
    }

    /// Validate mTLS configuration.
    fn validate_mtls(&self) -> Result<(), ConfigError> {
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
        Ok(())
    }

    /// Validate tracing configuration.
    fn validate_tracing(&self) -> Result<(), ConfigError> {
        if self.tracing.enabled
            && (self.tracing.sample_rate < 0.0 || self.tracing.sample_rate > 1.0)
        {
            return Err(ConfigError::Validation(
                "tracing.sample_rate must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate upstream configuration.
    fn validate_upstream(&self) -> Result<(), ConfigError> {
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

    /// Check if the configuration uses any Pro tier features
    ///
    /// Returns true if ANY of these are configured:
    /// - OAuth 2.1 authentication
    /// - JWT JWKS mode
    /// - HTTP or SSE transport
    pub fn requires_pro_features(&self) -> bool {
        // OAuth 2.1
        if self.auth.oauth.is_some() {
            return true;
        }

        // JWT JWKS mode
        if let Some(ref jwt_config) = self.auth.jwt {
            if matches!(jwt_config.mode, JwtMode::Jwks { .. }) {
                return true;
            }
        }

        // HTTP or SSE transport (single-server mode)
        if self.upstream.servers.is_empty() {
            match self.upstream.transport {
                TransportType::Http | TransportType::Sse => return true,
                TransportType::Stdio => {}
            }
        } else {
            // Multi-server mode - check if any server uses HTTP/SSE
            for server in &self.upstream.servers {
                match server.transport {
                    TransportType::Http | TransportType::Sse => return true,
                    TransportType::Stdio => {}
                }
            }
        }

        false
    }

    /// Check if the configuration uses any Enterprise tier features
    ///
    /// Returns true if ANY of these are configured:
    /// - mTLS client certificate authentication
    /// - Multi-server routing
    /// - SIEM audit log shipping
    /// - OpenTelemetry tracing with OTLP export
    /// - Per-tool rate limiting
    pub fn requires_enterprise_features(&self) -> bool {
        // mTLS authentication
        if let Some(ref mtls_config) = self.auth.mtls {
            if mtls_config.enabled {
                return true;
            }
        }

        // Multi-server routing
        if !self.upstream.servers.is_empty() {
            return true;
        }

        // SIEM audit log shipping
        if self.audit.export_url.is_some() {
            return true;
        }

        // OpenTelemetry with OTLP export
        if self.tracing.enabled && self.tracing.otlp_endpoint.is_some() {
            return true;
        }

        // Per-tool rate limiting
        if !self.rate_limit.tool_limits.is_empty() {
            return true;
        }

        false
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
        // Use stdio transport which is available in free tier
        Config {
            server: ServerConfig::default(),
            auth: AuthConfig::default(),
            rate_limit: RateLimitConfig::default(),
            audit: AuditConfig::default(),
            tracing: TracingConfig::default(),
            upstream: UpstreamConfig {
                transport: TransportType::Stdio,
                command: Some("/bin/echo".to_string()),
                args: vec![],
                url: None,
                servers: vec![],
            },
            policy: Default::default(),
            recorder: Default::default(),
            database_url: None,
            stripe_secret_key: None,
        }
    }

    // Create a config with HTTP transport for Pro tier tests
    #[cfg(feature = "pro")]
    fn create_valid_config_http() -> Config {
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
            policy: Default::default(),
            recorder: Default::default(),
            database_url: None,
            stripe_secret_key: None,
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

    // mTLS tests require Enterprise feature
    #[cfg(feature = "enterprise")]
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

    // Verify mTLS requires Enterprise in free tier
    #[cfg(not(feature = "enterprise"))]
    #[test]
    fn test_config_validation_mtls_requires_enterprise() {
        let mut config = create_valid_config();
        config.auth.mtls = Some(MtlsConfig {
            enabled: true,
            identity_source: MtlsIdentitySource::Cn,
            allowed_tools: vec![],
            rate_limit: None,
            trusted_proxy_ips: vec!["10.0.0.0/8".to_string()],
        });
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Enterprise"));
    }

    #[test]
    fn test_config_is_multi_server() {
        let mut config = create_valid_config();
        assert!(!config.is_multi_server());

        // Just test the method, not validation
        config.upstream.servers.push(ServerRouteConfig {
            name: "test".to_string(),
            path_prefix: "/test".to_string(),
            transport: TransportType::Stdio,
            command: Some("/bin/echo".to_string()),
            args: vec![],
            url: None,
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
