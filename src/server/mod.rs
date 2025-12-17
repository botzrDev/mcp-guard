//! Axum server and middleware for mcp-guard

use axum::{
    body::Body,
    extract::{ConnectInfo, Query, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use dashmap::DashMap;
use metrics_exporter_prometheus::PrometheusHandle;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing_opentelemetry::OpenTelemetrySpanExt;

// ============================================================================
// Constants
// ============================================================================

/// OAuth state expiry time for PKCE flow.
/// 10 minutes allows users time to complete the OAuth flow (login, consent)
/// while limiting the window for state token reuse attacks.
const OAUTH_STATE_EXPIRY_SECS: u64 = 600;

/// Maximum number of pending OAuth states to prevent DoS attacks.
/// An attacker flooding /oauth/authorize could cause memory exhaustion without this limit.
/// 10,000 concurrent OAuth flows is generous for legitimate use but prevents resource exhaustion.
const MAX_PENDING_OAUTH_STATES: usize = 10_000;

use crate::audit::AuditLogger;
use crate::auth::{AuthProvider, ClientCertInfo, Identity, MtlsAuthProvider, OAuthAuthProvider};
use crate::authz::{filter_tools_list_response, is_tools_list_request};
use crate::config::Config;
use crate::observability::{record_auth, record_rate_limit, record_request, set_active_identities};
use crate::rate_limit::RateLimitService;
use crate::router::ServerRouter;
use crate::transport::{Message, Transport};
use std::net::IpAddr;

/// PKCE state entry for OAuth flow
///
/// SECURITY: Includes client IP binding to prevent state fixation attacks.
/// The client IP that initiated the OAuth flow must match the callback IP.
pub struct PkceState {
    /// PKCE code verifier
    pub code_verifier: String,
    /// When the state was created
    pub created_at: Instant,
    /// Client IP that initiated the OAuth flow (for binding validation)
    pub client_ip: IpAddr,
}

/// OAuth state storage (state -> PKCE verifier)
pub type OAuthStateStore = Arc<DashMap<String, PkceState>>;

/// Create a new OAuth state store
pub fn new_oauth_state_store() -> OAuthStateStore {
    Arc::new(DashMap::new())
}

/// Application state shared across all request handlers
///
/// This struct contains all the shared resources needed to process MCP requests,
/// including authentication, rate limiting, transport connections, and metrics.
pub struct AppState {
    /// Loaded configuration (immutable after server start)
    pub config: Config,
    /// Primary authentication provider (may be MultiProvider for fallback auth)
    pub auth_provider: Arc<dyn AuthProvider>,
    /// Per-identity rate limiter with token bucket algorithm
    pub rate_limiter: RateLimitService,
    /// Audit logger for security event tracking
    pub audit_logger: Arc<AuditLogger>,
    /// Transport for single-server mode; None when using multi-server routing
    pub transport: Option<Arc<dyn Transport>>,
    /// Router for multi-server mode; None when using single-server mode
    pub router: Option<Arc<ServerRouter>>,
    /// Prometheus metrics handle for rendering /metrics endpoint
    pub metrics_handle: PrometheusHandle,
    /// OAuth provider for authorization code flow with PKCE (optional)
    pub oauth_provider: Option<Arc<OAuthAuthProvider>>,
    /// PKCE state storage mapping state tokens to code verifiers
    pub oauth_state_store: OAuthStateStore,
    /// Server startup timestamp for calculating uptime in /health
    pub started_at: Instant,
    /// Readiness flag for /ready endpoint (false until transport initialized)
    pub ready: Arc<RwLock<bool>>,
    /// mTLS provider for client certificate auth via reverse proxy headers
    pub mtls_provider: Option<Arc<MtlsAuthProvider>>,
}

/// Health check response (detailed)
#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    uptime_secs: u64,
}

/// Liveness check response (minimal)
#[derive(serde::Serialize)]
struct LiveResponse {
    status: &'static str,
}

/// Readiness check response
#[derive(serde::Serialize)]
struct ReadyResponse {
    ready: bool,
    version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// Health check handler - returns detailed status
async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let uptime = state.started_at.elapsed();
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
        uptime_secs: uptime.as_secs(),
    })
}

/// Liveness check handler - minimal check for container orchestration
/// Returns 200 if the server is running
async fn live() -> Json<LiveResponse> {
    Json(LiveResponse { status: "alive" })
}

/// Readiness check handler - checks if the server can handle requests
/// Returns 200 if ready, 503 if not ready
async fn ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let is_ready = *state.ready.read().await;

    if is_ready {
        (
            StatusCode::OK,
            Json(ReadyResponse {
                ready: true,
                version: env!("CARGO_PKG_VERSION"),
                reason: None,
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadyResponse {
                ready: false,
                version: env!("CARGO_PKG_VERSION"),
                reason: Some("Transport not initialized".to_string()),
            }),
        )
    }
}

/// Metrics endpoint handler - returns Prometheus format metrics
async fn metrics_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Update the active identities gauge before rendering
    set_active_identities(state.rate_limiter.tracked_identities());

    let metrics = state.metrics_handle.render();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        metrics,
    )
}

/// MCP message handler with tools/list filtering (FR-AUTHZ-03)
/// Used for single-server mode
async fn handle_mcp_message(
    State(state): State<Arc<AppState>>,
    axum::Extension(identity): axum::Extension<Identity>,
    Json(message): Json<Message>,
) -> Result<Json<Message>, AppError> {
    // Get the transport (single-server mode)
    let transport = state.transport.as_ref().ok_or_else(|| {
        AppError::internal("No transport configured (use multi-server routing?)")
    })?;

    // Check if this is a tools/list request (for later filtering)
    let is_tools_list = is_tools_list_request(&message);

    // Forward to upstream transport
    transport.send(message).await?;

    // Wait for response
    let response = transport.receive().await?;

    // Filter tools/list response to only show authorized tools
    let response = if is_tools_list {
        filter_tools_list_response(response, &identity)
    } else {
        response
    };

    Ok(Json(response))
}

/// MCP message handler for multi-server routing (FR-AUTHZ-03 applies here too)
/// Routes requests to different upstreams based on the server name in the path
async fn handle_routed_mcp_message(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(server_name): axum::extract::Path<String>,
    axum::Extension(identity): axum::Extension<Identity>,
    Json(message): Json<Message>,
) -> Result<Json<Message>, AppError> {
    // Get the router (multi-server mode)
    let router = state.router.as_ref().ok_or_else(|| {
        AppError::internal("No router configured (use single-server mode?)")
    })?;

    // Build path for routing
    let path = format!("/{}", server_name);

    // Get the transport for this path
    let transport = router.get_transport(&path).ok_or_else(|| {
        AppError::not_found(format!("No server route for path: {}", path))
    })?;

    tracing::debug!(
        server = %server_name,
        route = ?router.get_route_name(&path),
        "Routing MCP message"
    );

    // Check if this is a tools/list request (for later filtering)
    let is_tools_list = is_tools_list_request(&message);

    // Forward to upstream transport
    transport.send(message).await?;

    // Wait for response
    let response = transport.receive().await?;

    // Filter tools/list response to only show authorized tools
    let response = if is_tools_list {
        filter_tools_list_response(response, &identity)
    } else {
        response
    };

    Ok(Json(response))
}

// ============================================================================
// OAuth 2.1 Authorization Code Flow with PKCE (FR-AUTH-05)
// ============================================================================

/// Generate a cryptographically secure random string using OsRng and base64url encoding.
///
/// SECURITY: Uses OsRng (operating system's cryptographic RNG) instead of thread_rng
/// for better entropy. Base64url encoding provides ~6 bits per character (vs ~5.95
/// for charset-based approach), resulting in higher entropy per character.
fn generate_random_string(len: usize) -> String {
    use base64::Engine;
    use rand::RngCore;
    use rand::rngs::OsRng;

    // Calculate bytes needed: base64 encodes 3 bytes to 4 chars
    // We need enough bytes to produce at least `len` characters
    let bytes_needed = (len * 3 + 3) / 4;
    let mut bytes = vec![0u8; bytes_needed];
    OsRng.fill_bytes(&mut bytes);

    // Encode with URL-safe base64 and truncate to desired length
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
    encoded[..len].to_string()
}

/// Generate PKCE code verifier and challenge
fn generate_pkce() -> (String, String) {
    use sha2::{Digest, Sha256};

    // Generate a random 43-128 character code verifier
    let code_verifier = generate_random_string(64);

    // Create SHA-256 hash and base64url encode it
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let code_challenge = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        hash,
    );

    (code_verifier, code_challenge)
}

/// Clean up expired OAuth states (older than 10 minutes)
fn cleanup_expired_oauth_states(store: &OAuthStateStore) {
    let expiry = Duration::from_secs(OAUTH_STATE_EXPIRY_SECS);
    store.retain(|_, state| state.created_at.elapsed() < expiry);
}

/// OAuth authorize endpoint - initiates the OAuth flow
/// Initiate OAuth authorization flow with PKCE.
///
/// SECURITY: Binds the OAuth state to the client IP to prevent state fixation attacks.
/// Also enforces a limit on pending states to prevent DoS attacks.
async fn oauth_authorize(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
) -> Result<impl IntoResponse, AppError> {
    let oauth_provider = state
        .oauth_provider
        .as_ref()
        .ok_or_else(|| AppError::internal("OAuth not configured"))?;

    // SECURITY: Cleanup expired states first, then check the limit
    cleanup_expired_oauth_states(&state.oauth_state_store);

    // SECURITY: Prevent DoS by limiting the number of pending OAuth states
    if state.oauth_state_store.len() >= MAX_PENDING_OAUTH_STATES {
        tracing::warn!(
            current_count = state.oauth_state_store.len(),
            max_allowed = MAX_PENDING_OAUTH_STATES,
            "OAuth state store at capacity - possible DoS attack"
        );
        // Return rate limited with a 60 second retry-after
        return Err(AppError::rate_limited(Some(60)));
    }

    // Generate PKCE code verifier and challenge
    let (code_verifier, code_challenge) = generate_pkce();

    // Generate random state parameter
    let oauth_state = generate_random_string(32);

    // SECURITY: Bind the state to the client IP to prevent state fixation attacks
    let client_ip = addr.ip();

    // Store the code verifier with the state and client IP binding
    state.oauth_state_store.insert(
        oauth_state.clone(),
        PkceState {
            code_verifier,
            created_at: Instant::now(),
            client_ip,
        },
    );

    // Build authorization URL
    let auth_url = oauth_provider.get_authorization_url(&oauth_state, Some(&code_challenge));

    tracing::info!(
        client_ip = %client_ip,
        pending_states = state.oauth_state_store.len(),
        "Initiating OAuth flow with state: {}",
        oauth_state
    );

    Ok(Redirect::temporary(&auth_url))
}

/// Query parameters for OAuth callback
#[derive(Debug, serde::Deserialize)]
pub struct OAuthCallbackParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// OAuth token response
#[derive(Debug, serde::Serialize)]
struct OAuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

/// OAuth callback endpoint - exchanges authorization code for tokens.
///
/// SECURITY: Validates that the client IP matches the IP that initiated the OAuth flow
/// to prevent state fixation attacks.
async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Query(params): Query<OAuthCallbackParams>,
) -> Result<impl IntoResponse, AppError> {
    // Check for errors from OAuth provider
    if let Some(error) = params.error {
        let description = params.error_description.unwrap_or_default();
        tracing::warn!("OAuth error: {} - {}", error, description);
        return Err(AppError::unauthorized(format!(
            "OAuth error: {} - {}",
            error, description
        )));
    }

    // Validate state parameter
    let oauth_state = params
        .state
        .ok_or_else(|| AppError::unauthorized("Missing state parameter"))?;

    // Retrieve and remove PKCE state
    let pkce_state = state
        .oauth_state_store
        .remove(&oauth_state)
        .map(|(_, v)| v)
        .ok_or_else(|| AppError::unauthorized("Invalid or expired state"))?;

    // Validate state hasn't expired (10 minute limit)
    if pkce_state.created_at.elapsed() > Duration::from_secs(OAUTH_STATE_EXPIRY_SECS) {
        return Err(AppError::unauthorized("OAuth state expired"));
    }

    // SECURITY: Validate client IP binding to prevent state fixation attacks
    let callback_ip = addr.ip();
    if pkce_state.client_ip != callback_ip {
        tracing::warn!(
            expected_ip = %pkce_state.client_ip,
            actual_ip = %callback_ip,
            "OAuth callback IP mismatch - possible state fixation attack"
        );
        return Err(AppError::unauthorized("OAuth state binding mismatch"));
    }

    // Get authorization code
    let code = params
        .code
        .ok_or_else(|| AppError::unauthorized("Missing authorization code"))?;

    // Get OAuth provider
    let oauth_provider = state
        .oauth_provider
        .as_ref()
        .ok_or_else(|| AppError::internal("OAuth not configured"))?;

    // Exchange code for tokens
    let tokens = exchange_code_for_tokens(
        &state.config,
        oauth_provider,
        &code,
        &pkce_state.code_verifier,
    )
    .await?;

    tracing::info!("OAuth code exchange successful");

    Ok(Json(tokens))
}

/// Exchange authorization code for tokens
async fn exchange_code_for_tokens(
    config: &Config,
    oauth_provider: &OAuthAuthProvider,
    code: &str,
    code_verifier: &str,
) -> Result<OAuthTokenResponse, AppError> {
    let oauth_config = config
        .auth
        .oauth
        .as_ref()
        .ok_or_else(|| AppError::internal("OAuth not configured"))?;

    // Build token request
    let client = reqwest::Client::new();
    let mut form = vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", &oauth_config.redirect_uri),
        ("client_id", &oauth_config.client_id),
        ("code_verifier", code_verifier),
    ];

    // Add client_secret for confidential clients
    let client_secret;
    if let Some(ref secret) = oauth_config.client_secret {
        client_secret = secret.clone();
        form.push(("client_secret", &client_secret));
    }

    let response = client
        .post(oauth_provider.token_url())
        .header("Accept", "application/json")
        .form(&form)
        .send()
        .await
        .map_err(|e| AppError::internal(format!("Token exchange request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::error!("Token exchange failed: {} - {}", status, body);
        return Err(AppError::unauthorized(format!(
            "Token exchange failed: {}",
            status
        )));
    }

    let token_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::internal(format!("Failed to parse token response: {}", e)))?;

    let access_token = token_response
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::internal("No access_token in response"))?
        .to_string();

    let token_type = token_response
        .get("token_type")
        .and_then(|v| v.as_str())
        .unwrap_or("Bearer")
        .to_string();

    let expires_in = token_response
        .get("expires_in")
        .and_then(|v| v.as_u64());

    let refresh_token = token_response
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .map(String::from);

    let scope = token_response
        .get("scope")
        .and_then(|v| v.as_str())
        .map(String::from);

    Ok(OAuthTokenResponse {
        access_token,
        token_type,
        expires_in,
        refresh_token,
        scope,
    })
}

use crate::rate_limit::RateLimitResult;

/// Authentication middleware with metrics
///
/// Supports multiple authentication methods in order of preference:
/// 1. mTLS: Client certificate info from headers (X-Client-Cert-CN, etc.)
///    SECURITY: Only accepted from trusted proxy IPs configured in `trusted_proxy_ips`
/// 2. Bearer token: Authorization header with Bearer token (API key, JWT, OAuth)
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Try mTLS authentication first (if configured and headers present)
    if let Some(ref mtls_provider) = state.mtls_provider {
        // SECURITY: Use the secure method that validates client IP
        let client_ip = addr.ip();
        if let Some(cert_info) = ClientCertInfo::from_headers_if_trusted(
            request.headers(),
            &client_ip,
            mtls_provider,
        ) {
            if cert_info.verified || cert_info.common_name.is_some() {
                match mtls_provider.extract_identity(&cert_info) {
                    Ok(identity) => {
                        record_auth("mtls", true);
                        state.audit_logger.log_auth_success(&identity.id);

                        // Check rate limit
                        let rate_limit_result =
                            state.rate_limiter.check(&identity.id, identity.rate_limit);
                        record_rate_limit(rate_limit_result.allowed);

                        if !rate_limit_result.allowed {
                            state.audit_logger.log_rate_limited(&identity.id);
                            return Err(AppError::rate_limited_with_info(rate_limit_result));
                        }

                        request.extensions_mut().insert(identity);
                        let mut response = next.run(request).await;
                        add_rate_limit_headers_from_result(&mut response, &rate_limit_result);
                        return Ok(response);
                    }
                    Err(e) => {
                        record_auth("mtls", false);
                        tracing::debug!("mTLS auth failed, falling back to bearer: {}", e);
                        // Fall through to bearer token auth
                    }
                }
            }
        }
    }

    // Fall back to Bearer token authentication
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Missing authorization header"))?;

    // Get provider name for metrics
    let provider_name = state.auth_provider.name().to_string();

    // Authenticate
    let identity = match state.auth_provider.authenticate(token).await {
        Ok(identity) => {
            record_auth(&provider_name, true);
            state.audit_logger.log_auth_success(&identity.id);
            identity
        }
        Err(e) => {
            record_auth(&provider_name, false);
            state.audit_logger.log_auth_failure(&e.to_string());
            return Err(AppError::unauthorized(e.to_string()));
        }
    };

    // Check rate limit (per-identity)
    let rate_limit_result = state.rate_limiter.check(&identity.id, identity.rate_limit);
    record_rate_limit(rate_limit_result.allowed);

    if !rate_limit_result.allowed {
        state.audit_logger.log_rate_limited(&identity.id);
        return Err(AppError::rate_limited_with_info(rate_limit_result));
    }

    // Add identity to request extensions
    request.extensions_mut().insert(identity);

    // Run the request and add rate limit headers to response
    let mut response = next.run(request).await;
    add_rate_limit_headers_from_result(&mut response, &rate_limit_result);
    Ok(response)
}

/// Add rate limit headers to a response
///
/// Headers added (per RFC 6585 and draft-ietf-httpapi-ratelimit-headers):
/// - `X-RateLimit-Limit`: The maximum number of requests allowed per second
/// - `X-RateLimit-Remaining`: Approximate remaining requests in current window
/// - `X-RateLimit-Reset`: Unix timestamp when the rate limit resets
fn add_rate_limit_headers_from_result(response: &mut Response, rate_limit: &RateLimitResult) {
    let headers = response.headers_mut();

    if let Ok(limit) = HeaderValue::from_str(&rate_limit.limit.to_string()) {
        headers.insert(HeaderName::from_static("x-ratelimit-limit"), limit);
    }
    if let Ok(remaining) = HeaderValue::from_str(&rate_limit.remaining.to_string()) {
        headers.insert(HeaderName::from_static("x-ratelimit-remaining"), remaining);
    }
    if let Ok(reset) = HeaderValue::from_str(&rate_limit.reset_at.to_string()) {
        headers.insert(HeaderName::from_static("x-ratelimit-reset"), reset);
    }
}

/// Middleware for recording request duration metrics
pub async fn metrics_middleware(request: Request<Body>, next: Next) -> Response {
    let method = request.method().to_string();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status().as_u16();
    record_request(&method, status, duration);

    response
}

/// Header extractor for W3C trace context propagation
struct HeaderExtractor<'a>(&'a HeaderMap);

impl opentelemetry::propagation::Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

/// Header injector for W3C trace context propagation
struct HeaderInjector<'a>(&'a mut HeaderMap);

impl opentelemetry::propagation::Injector for HeaderInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        if let Ok(header_name) = header::HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(header_value) = header::HeaderValue::from_str(&value) {
                self.0.insert(header_name, header_value);
            }
        }
    }
}

/// Middleware for W3C trace context propagation (FR-OBS-03)
///
/// Extracts W3C traceparent and tracestate headers from incoming requests
/// and sets them on the current tracing span. Also propagates trace context
/// to downstream requests.
pub async fn trace_context_middleware(request: Request<Body>, next: Next) -> Response {
    // Extract trace context from incoming headers
    let propagator = TraceContextPropagator::new();
    let parent_context = propagator.extract(&HeaderExtractor(request.headers()));

    // Create a new span for this request with the extracted context
    let span = tracing::info_span!(
        "http_request",
        method = %request.method(),
        uri = %request.uri(),
        trace_id = tracing::field::Empty,
    );

    // Set the parent context on the span
    span.set_parent(parent_context);

    // Record trace_id in the span (for logs)
    if let Some(trace_id) = crate::observability::current_trace_id() {
        span.record("trace_id", &trace_id);
    }

    // Execute the request within the span
    let _guard = span.enter();
    let mut response = next.run(request).await;

    // Optionally inject trace context into response headers (for debugging)
    // This allows clients to correlate their requests with our traces
    let current_span = tracing::Span::current();
    let context = current_span.context();
    propagator.inject_context(&context, &mut HeaderInjector(response.headers_mut()));

    response
}

/// Middleware that adds security headers to all responses.
///
/// Headers added:
/// - `X-Content-Type-Options: nosniff` - Prevents MIME-sniffing attacks
/// - `X-Frame-Options: DENY` - Prevents clickjacking via iframe embedding
/// - `X-XSS-Protection: 1; mode=block` - Enables browser XSS filtering (legacy browsers)
/// - `Content-Security-Policy: default-src 'none'` - Strict CSP for API responses
///
/// These headers provide defense-in-depth for security even though
/// mcp-guard is primarily an API server (not serving HTML).
pub async fn security_headers_middleware(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();

    // Prevent MIME-sniffing attacks
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // Prevent clickjacking via iframe embedding
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );

    // Enable browser XSS filtering (for legacy browsers)
    headers.insert(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );

    // Strict Content-Security-Policy for API responses
    // Since we don't serve HTML, we use the strictest possible policy
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'none'"),
    );

    response
}

/// Application error type with unique error ID for correlation
#[derive(Debug)]
pub struct AppError {
    /// Unique error ID for correlation across logs and responses
    pub error_id: String,
    /// The actual error kind
    pub kind: AppErrorKind,
}

/// Application error variants
#[derive(Debug)]
pub enum AppErrorKind {
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    RateLimited {
        retry_after_secs: Option<u64>,
        limit: Option<u32>,
        remaining: Option<u32>,
        reset_at: Option<u64>,
    },
    Transport(crate::transport::TransportError),
    Internal(String),
}

impl AppError {
    /// Create a new error with a unique ID
    fn new(kind: AppErrorKind) -> Self {
        let error_id = uuid::Uuid::new_v4().to_string();
        Self { error_id, kind }
    }

    /// Create an Unauthorized error
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::new(AppErrorKind::Unauthorized(msg.into()))
    }

    /// Create a Forbidden error
    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::new(AppErrorKind::Forbidden(msg.into()))
    }

    /// Create a NotFound error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::new(AppErrorKind::NotFound(msg.into()))
    }

    /// Create a RateLimited error
    pub fn rate_limited(retry_after_secs: Option<u64>) -> Self {
        Self::new(AppErrorKind::RateLimited {
            retry_after_secs,
            limit: None,
            remaining: None,
            reset_at: None,
        })
    }

    /// Create a RateLimited error with full rate limit info
    pub fn rate_limited_with_info(rate_limit: RateLimitResult) -> Self {
        Self::new(AppErrorKind::RateLimited {
            retry_after_secs: rate_limit.retry_after_secs,
            limit: Some(rate_limit.limit),
            remaining: Some(rate_limit.remaining),
            reset_at: Some(rate_limit.reset_at),
        })
    }

    /// Create a Transport error
    pub fn transport(e: crate::transport::TransportError) -> Self {
        Self::new(AppErrorKind::Transport(e))
    }

    /// Create an Internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::new(AppErrorKind::Internal(msg.into()))
    }
}

impl From<crate::transport::TransportError> for AppError {
    fn from(e: crate::transport::TransportError) -> Self {
        AppError::transport(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_id = self.error_id.clone();

        match self.kind {
            AppErrorKind::Unauthorized(msg) => {
                tracing::warn!(error_id = %error_id, error = %msg, "Authentication failed");
                let body = serde_json::json!({
                    "error": msg,
                    "error_id": error_id
                });
                (StatusCode::UNAUTHORIZED, Json(body)).into_response()
            }
            AppErrorKind::Forbidden(msg) => {
                tracing::warn!(error_id = %error_id, error = %msg, "Authorization denied");
                let body = serde_json::json!({
                    "error": msg,
                    "error_id": error_id
                });
                (StatusCode::FORBIDDEN, Json(body)).into_response()
            }
            AppErrorKind::NotFound(msg) => {
                tracing::debug!(error_id = %error_id, error = %msg, "Resource not found");
                let body = serde_json::json!({
                    "error": msg,
                    "error_id": error_id
                });
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
            AppErrorKind::RateLimited { retry_after_secs, limit, remaining, reset_at } => {
                let retry_after = retry_after_secs.unwrap_or(1);
                tracing::debug!(error_id = %error_id, retry_after = retry_after, "Rate limit exceeded");
                let body = serde_json::json!({
                    "error": "Rate limit exceeded",
                    "retry_after": retry_after,
                    "error_id": error_id
                });

                // Build response with all rate limit headers (FR-RATE-05 + P1 enhancements)
                let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(body)).into_response();
                let headers = response.headers_mut();

                // Retry-After header (required by RFC 6585)
                if let Ok(val) = HeaderValue::from_str(&retry_after.to_string()) {
                    headers.insert(header::RETRY_AFTER, val);
                }

                // X-RateLimit-* headers (draft-ietf-httpapi-ratelimit-headers)
                if let Some(l) = limit {
                    if let Ok(val) = HeaderValue::from_str(&l.to_string()) {
                        headers.insert(HeaderName::from_static("x-ratelimit-limit"), val);
                    }
                }
                if let Some(r) = remaining {
                    if let Ok(val) = HeaderValue::from_str(&r.to_string()) {
                        headers.insert(HeaderName::from_static("x-ratelimit-remaining"), val);
                    }
                }
                if let Some(reset) = reset_at {
                    if let Ok(val) = HeaderValue::from_str(&reset.to_string()) {
                        headers.insert(HeaderName::from_static("x-ratelimit-reset"), val);
                    }
                }

                response
            }
            AppErrorKind::Transport(e) => {
                // Log the full error internally for debugging, but return sanitized message
                tracing::error!(
                    error_id = %error_id,
                    error = %e,
                    "Upstream transport error"
                );
                // Sanitize: don't expose internal paths, commands, or detailed error messages
                let sanitized_msg = match &e {
                    crate::transport::TransportError::Timeout => "Upstream request timed out",
                    crate::transport::TransportError::ConnectionClosed => "Upstream connection closed",
                    crate::transport::TransportError::ProcessExited => "Upstream process unavailable",
                    _ => "Upstream communication error",
                };
                let body = serde_json::json!({
                    "error": sanitized_msg,
                    "error_id": error_id
                });
                (StatusCode::BAD_GATEWAY, Json(body)).into_response()
            }
            AppErrorKind::Internal(msg) => {
                // Log the full message internally but return generic message to client
                tracing::error!(error_id = %error_id, error = %msg, "Internal server error");
                let body = serde_json::json!({
                    "error": "Internal server error",
                    "error_id": error_id
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

/// Build the application router
pub fn build_router(state: Arc<AppState>) -> Router {
    // Determine if we're in multi-server mode
    let is_multi_server = state.router.is_some();

    // Build protected routes based on mode
    let protected_routes = if is_multi_server {
        // Multi-server mode: route to /mcp/:server_name
        Router::new()
            .route("/mcp/:server_name", post(handle_routed_mcp_message))
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
    } else {
        // Single-server mode: route to /mcp
        Router::new()
            .route("/mcp", post(handle_mcp_message))
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
    };

    // OAuth routes (only added if OAuth is configured)
    let mut router = Router::new()
        .route("/health", get(health))
        .route("/live", get(live))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics_handler));

    // Add routes endpoint for multi-server mode (lists available servers)
    if is_multi_server {
        router = router.route("/routes", get(list_routes));
    }

    if state.oauth_provider.is_some() {
        router = router
            .route("/oauth/authorize", get(oauth_authorize))
            .route("/oauth/callback", get(oauth_callback));
    }

    // Build the router with middleware layers
    // Layer order (bottom to top): SecurityHeaders -> TraceContext -> Metrics -> TraceLayer
    // Security headers are applied first (outermost) to ensure all responses get them
    router
        .merge(protected_routes)
        .layer(middleware::from_fn(metrics_middleware))
        .layer(middleware::from_fn(trace_context_middleware))
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// List available server routes (multi-server mode only)
async fn list_routes(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Some(ref router) = state.router {
        let routes: Vec<_> = router.route_names().iter().map(|s| s.to_string()).collect();
        let body = serde_json::json!({
            "routes": routes,
            "count": routes.len()
        });
        (StatusCode::OK, Json(body))
    } else {
        let body = serde_json::json!({
            "routes": [],
            "count": 0,
            "note": "Single-server mode, no routes configured"
        });
        (StatusCode::OK, Json(body))
    }
}

/// Run the server
pub async fn run(state: Arc<AppState>) -> Result<(), crate::Error> {
    let addr = format!("{}:{}", state.config.server.host, state.config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("MCP Guard listening on {}", addr);

    let app = build_router(state);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .map_err(|e| crate::Error::Server(e.to_string()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use opentelemetry::propagation::Extractor;
    use tower::ServiceExt;

    // ------------------------------------------------------------------------
    // AppError Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_app_error_unauthorized() {
        let err = AppError::unauthorized("Invalid token");
        assert!(matches!(err.kind, AppErrorKind::Unauthorized(_)));
        assert!(!err.error_id.is_empty());
    }

    #[test]
    fn test_app_error_forbidden() {
        let err = AppError::forbidden("Access denied");
        assert!(matches!(err.kind, AppErrorKind::Forbidden(_)));
    }

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::not_found("Route not found");
        assert!(matches!(err.kind, AppErrorKind::NotFound(_)));
    }

    #[test]
    fn test_app_error_rate_limited() {
        let err = AppError::rate_limited(Some(5));
        match err.kind {
            AppErrorKind::RateLimited { retry_after_secs, .. } => {
                assert_eq!(retry_after_secs, Some(5));
            }
            _ => panic!("Expected RateLimited"),
        }
    }

    #[test]
    fn test_app_error_internal() {
        let err = AppError::internal("Something went wrong");
        assert!(matches!(err.kind, AppErrorKind::Internal(_)));
    }

    #[tokio::test]
    async fn test_app_error_unauthorized_response() {
        let err = AppError::unauthorized("Test unauthorized");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_app_error_forbidden_response() {
        let err = AppError::forbidden("Test forbidden");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_app_error_not_found_response() {
        let err = AppError::not_found("Test not found");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_app_error_rate_limited_response() {
        let err = AppError::rate_limited(Some(10));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(response.headers().get(header::RETRY_AFTER).is_some());
    }

    #[tokio::test]
    async fn test_app_error_internal_response() {
        let err = AppError::internal("Internal error");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_app_error_transport_timeout_response() {
        let err = AppError::transport(crate::transport::TransportError::Timeout);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    }

    #[tokio::test]
    async fn test_app_error_transport_connection_closed_response() {
        let err = AppError::transport(crate::transport::TransportError::ConnectionClosed);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    }

    // ------------------------------------------------------------------------
    // PKCE & OAuth State Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_generate_random_string() {
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_ne!(s1, s2); // Should be different each time
    }

    #[test]
    fn test_generate_pkce() {
        let (verifier, challenge) = generate_pkce();
        assert_eq!(verifier.len(), 64);
        assert!(!challenge.is_empty());
        // Challenge should be base64url encoded SHA-256 (43 chars without padding)
        assert_eq!(challenge.len(), 43);
    }

    #[test]
    fn test_pkce_consistency() {
        // Verify that verifier and challenge are correctly related
        use sha2::{Digest, Sha256};
        
        let (verifier, challenge) = generate_pkce();
        
        // Manually compute expected challenge
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let hash = hasher.finalize();
        let expected_challenge = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hash,
        );
        
        assert_eq!(challenge, expected_challenge);
    }

    #[test]
    fn test_new_oauth_state_store() {
        let store = new_oauth_state_store();
        assert!(store.is_empty());
    }

    #[test]
    fn test_cleanup_expired_oauth_states() {
        let store = new_oauth_state_store();

        // Add a fresh state with client IP binding
        store.insert("fresh".to_string(), PkceState {
            code_verifier: "verifier".to_string(),
            created_at: Instant::now(),
            client_ip: "127.0.0.1".parse().unwrap(),
        });

        // Cleanup should keep fresh state
        cleanup_expired_oauth_states(&store);
        assert!(store.contains_key("fresh"));
    }

    #[test]
    fn test_generate_random_string_entropy() {
        // Test that generated strings are unique (high entropy)
        let s1 = generate_random_string(32);
        let s2 = generate_random_string(32);
        let s3 = generate_random_string(32);

        assert_eq!(s1.len(), 32);
        assert_eq!(s2.len(), 32);
        assert_eq!(s3.len(), 32);

        // All should be different (with overwhelming probability)
        assert_ne!(s1, s2);
        assert_ne!(s2, s3);
        assert_ne!(s1, s3);

        // Should only contain URL-safe base64 characters
        for c in s1.chars() {
            assert!(c.is_ascii_alphanumeric() || c == '-' || c == '_');
        }
    }

    #[test]
    fn test_oauth_state_store_limit_constant() {
        // Verify the constant is set to a reasonable value
        assert!(MAX_PENDING_OAUTH_STATES >= 1000); // At least 1000 for legitimate use
        assert!(MAX_PENDING_OAUTH_STATES <= 100_000); // Not too high to be useless
    }

    #[test]
    fn test_oauth_state_store_capacity_check() {
        // This test verifies the store can be checked for capacity
        let store = new_oauth_state_store();

        // Fill to near capacity (we don't actually fill to max to avoid test slowness)
        for i in 0..100 {
            store.insert(format!("state_{}", i), PkceState {
                code_verifier: "verifier".to_string(),
                created_at: Instant::now(),
                client_ip: "127.0.0.1".parse().unwrap(),
            });
        }

        // Verify we can check the length
        assert_eq!(store.len(), 100);

        // Verify the max constant is accessible
        assert!(store.len() < MAX_PENDING_OAUTH_STATES);
    }

    // ------------------------------------------------------------------------
    // Response Types Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy",
            version: "1.0.0",
            uptime_secs: 100,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("1.0.0"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_live_response_serialization() {
        let response = LiveResponse { status: "alive" };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("alive"));
    }

    #[test]
    fn test_ready_response_ready() {
        let response = ReadyResponse {
            ready: true,
            version: "1.0.0",
            reason: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(!json.contains("reason")); // Should be skipped when None
    }

    #[test]
    fn test_ready_response_not_ready() {
        let response = ReadyResponse {
            ready: false,
            version: "1.0.0",
            reason: Some("Transport not initialized".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("false"));
        assert!(json.contains("Transport not initialized"));
    }

    // ------------------------------------------------------------------------
    // Security Headers Middleware Test
    // ------------------------------------------------------------------------

    #[tokio::test]
    async fn test_security_headers_middleware() {
        use axum::routing::get;

        async fn dummy_handler() -> &'static str {
            "OK"
        }

        let app = Router::new()
            .route("/test", get(dummy_handler))
            .layer(middleware::from_fn(security_headers_middleware));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        
        // Check security headers are present
        assert_eq!(
            response.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(),
            "nosniff"
        );
        assert_eq!(
            response.headers().get(header::X_FRAME_OPTIONS).unwrap(),
            "DENY"
        );
        assert_eq!(
            response.headers().get("x-xss-protection").unwrap(),
            "1; mode=block"
        );
        assert_eq!(
            response.headers().get(header::CONTENT_SECURITY_POLICY).unwrap(),
            "default-src 'none'"
        );
    }

    // ------------------------------------------------------------------------
    // Header Extractor/Injector Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_header_extractor() {
        let mut headers = HeaderMap::new();
        headers.insert("traceparent", HeaderValue::from_static("00-abc-def-01"));
        
        let extractor = HeaderExtractor(&headers);
        assert_eq!(extractor.get("traceparent"), Some("00-abc-def-01"));
        assert_eq!(extractor.get("missing"), None);
    }

    #[test]
    fn test_header_extractor_keys() {
        let mut headers = HeaderMap::new();
        headers.insert("x-custom", HeaderValue::from_static("value"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let extractor = HeaderExtractor(&headers);
        let keys = extractor.keys();
        assert!(keys.contains(&"x-custom"));
        assert!(keys.contains(&"content-type"));
    }

    #[test]
    fn test_header_injector() {
        use opentelemetry::propagation::Injector;
        
        let mut headers = HeaderMap::new();
        {
            let mut injector = HeaderInjector(&mut headers);
            injector.set("x-trace-id", "12345".to_string());
        }
        
        assert_eq!(headers.get("x-trace-id").unwrap(), "12345");
    }

    #[test]
    fn test_app_error_response_codes() {
        // Forbidden
        let err = AppError::forbidden("access denied");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        
        // Not Found
        let err = AppError::not_found("resource missing");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        
        // Transport error
        let err = AppError::transport(crate::transport::TransportError::Timeout);
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
        
        // Internal
        let err = AppError::internal("boom");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_trace_context_middleware() {
        use tower::ServiceExt;
        
        async fn handler() -> &'static str { "ok" }
        
        let app = Router::new()
             .route("/", get(handler))
             .layer(middleware::from_fn(trace_context_middleware));
             
        let req = Request::builder()
            .uri("/")
            .header("traceparent", "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
            .body(Body::empty())
            .unwrap();
            
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
