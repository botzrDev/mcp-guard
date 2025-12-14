//! Axum server and middleware for mcp-guard

use axum::{
    body::Body,
    extract::{Query, State},
    http::{header, HeaderMap, Request, StatusCode},
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
use tower_http::trace::TraceLayer;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::audit::AuditLogger;
use crate::auth::{AuthProvider, OAuthAuthProvider};
use crate::config::Config;
use crate::observability::{record_auth, record_rate_limit, record_request, set_active_identities};
use crate::rate_limit::RateLimitService;
use crate::transport::{Message, Transport};

/// PKCE state entry for OAuth flow
pub struct PkceState {
    /// PKCE code verifier
    pub code_verifier: String,
    /// When the state was created
    pub created_at: Instant,
}

/// OAuth state storage (state -> PKCE verifier)
pub type OAuthStateStore = Arc<DashMap<String, PkceState>>;

/// Create a new OAuth state store
pub fn new_oauth_state_store() -> OAuthStateStore {
    Arc::new(DashMap::new())
}

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub rate_limiter: RateLimitService,
    pub audit_logger: Arc<AuditLogger>,
    pub transport: Arc<dyn Transport>,
    pub metrics_handle: PrometheusHandle,
    /// OAuth provider for authorization code flow (optional)
    pub oauth_provider: Option<Arc<OAuthAuthProvider>>,
    /// OAuth state storage for PKCE
    pub oauth_state_store: OAuthStateStore,
}

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

/// Health check handler
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
    })
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

/// MCP message handler
async fn handle_mcp_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Result<Json<Message>, AppError> {
    // Forward to upstream transport
    state.transport.send(message).await?;

    // Wait for response
    let response = state.transport.receive().await?;

    Ok(Json(response))
}

// ============================================================================
// OAuth 2.1 Authorization Code Flow with PKCE (FR-AUTH-05)
// ============================================================================

/// Generate a cryptographically secure random string
fn generate_random_string(len: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
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
    let expiry = Duration::from_secs(600); // 10 minutes
    store.retain(|_, state| state.created_at.elapsed() < expiry);
}

/// OAuth authorize endpoint - initiates the OAuth flow
async fn oauth_authorize(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let oauth_provider = state
        .oauth_provider
        .as_ref()
        .ok_or_else(|| AppError::Internal("OAuth not configured".into()))?;

    // Generate PKCE code verifier and challenge
    let (code_verifier, code_challenge) = generate_pkce();

    // Generate random state parameter
    let oauth_state = generate_random_string(32);

    // Store the code verifier with the state
    cleanup_expired_oauth_states(&state.oauth_state_store);
    state.oauth_state_store.insert(
        oauth_state.clone(),
        PkceState {
            code_verifier,
            created_at: Instant::now(),
        },
    );

    // Build authorization URL
    let auth_url = oauth_provider.get_authorization_url(&oauth_state, Some(&code_challenge));

    tracing::info!("Initiating OAuth flow with state: {}", oauth_state);

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

/// OAuth callback endpoint - exchanges authorization code for tokens
async fn oauth_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<OAuthCallbackParams>,
) -> Result<impl IntoResponse, AppError> {
    // Check for errors from OAuth provider
    if let Some(error) = params.error {
        let description = params.error_description.unwrap_or_default();
        tracing::warn!("OAuth error: {} - {}", error, description);
        return Err(AppError::Unauthorized(format!(
            "OAuth error: {} - {}",
            error, description
        )));
    }

    // Validate state parameter
    let oauth_state = params
        .state
        .ok_or_else(|| AppError::Unauthorized("Missing state parameter".into()))?;

    // Retrieve and remove PKCE state
    let pkce_state = state
        .oauth_state_store
        .remove(&oauth_state)
        .map(|(_, v)| v)
        .ok_or_else(|| AppError::Unauthorized("Invalid or expired state".into()))?;

    // Validate state hasn't expired (10 minute limit)
    if pkce_state.created_at.elapsed() > Duration::from_secs(600) {
        return Err(AppError::Unauthorized("OAuth state expired".into()));
    }

    // Get authorization code
    let code = params
        .code
        .ok_or_else(|| AppError::Unauthorized("Missing authorization code".into()))?;

    // Get OAuth provider
    let oauth_provider = state
        .oauth_provider
        .as_ref()
        .ok_or_else(|| AppError::Internal("OAuth not configured".into()))?;

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
        .ok_or_else(|| AppError::Internal("OAuth not configured".into()))?;

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
        .map_err(|e| AppError::Internal(format!("Token exchange request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::error!("Token exchange failed: {} - {}", status, body);
        return Err(AppError::Unauthorized(format!(
            "Token exchange failed: {}",
            status
        )));
    }

    let token_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to parse token response: {}", e)))?;

    let access_token = token_response
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Internal("No access_token in response".into()))?
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

/// Authentication middleware with metrics
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from Authorization header
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing authorization header".into()))?;

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
            return Err(AppError::Unauthorized(e.to_string()));
        }
    };

    // Check rate limit (per-identity)
    let rate_limit_result = state.rate_limiter.check(&identity.id, identity.rate_limit);
    record_rate_limit(rate_limit_result.allowed);

    if !rate_limit_result.allowed {
        state.audit_logger.log_rate_limited(&identity.id);
        return Err(AppError::RateLimited {
            retry_after_secs: rate_limit_result.retry_after_secs,
        });
    }

    // Add identity to request extensions
    request.extensions_mut().insert(identity);

    Ok(next.run(request).await)
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

/// Application error type
#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    Forbidden(String),
    RateLimited { retry_after_secs: Option<u64> },
    Transport(crate::transport::TransportError),
    Internal(String),
}

impl From<crate::transport::TransportError> for AppError {
    fn from(e: crate::transport::TransportError) -> Self {
        AppError::Transport(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Unauthorized(msg) => {
                let body = serde_json::json!({ "error": msg });
                (StatusCode::UNAUTHORIZED, Json(body)).into_response()
            }
            AppError::Forbidden(msg) => {
                let body = serde_json::json!({ "error": msg });
                (StatusCode::FORBIDDEN, Json(body)).into_response()
            }
            AppError::RateLimited { retry_after_secs } => {
                let retry_after = retry_after_secs.unwrap_or(1);
                let body = serde_json::json!({
                    "error": "Rate limit exceeded",
                    "retry_after": retry_after
                });
                // FR-RATE-05: Return 429 with Retry-After header
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    [(header::RETRY_AFTER, retry_after.to_string())],
                    Json(body),
                )
                    .into_response()
            }
            AppError::Transport(e) => {
                let body = serde_json::json!({ "error": e.to_string() });
                (StatusCode::BAD_GATEWAY, Json(body)).into_response()
            }
            AppError::Internal(msg) => {
                let body = serde_json::json!({ "error": msg });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

/// Build the application router
pub fn build_router(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/mcp", post(handle_mcp_message))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // OAuth routes (only added if OAuth is configured)
    let mut router = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler));

    if state.oauth_provider.is_some() {
        router = router
            .route("/oauth/authorize", get(oauth_authorize))
            .route("/oauth/callback", get(oauth_callback));
    }

    // Build the router with middleware layers
    // Layer order (bottom to top): TraceContext -> Metrics -> TraceLayer
    // This ensures trace context is available for all subsequent layers
    router
        .merge(protected_routes)
        .layer(middleware::from_fn(metrics_middleware))
        .layer(middleware::from_fn(trace_context_middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Run the server
pub async fn run(state: Arc<AppState>) -> Result<(), crate::Error> {
    let addr = format!("{}:{}", state.config.server.host, state.config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("MCP Guard listening on {}", addr);

    let app = build_router(state);
    axum::serve(listener, app)
        .await
        .map_err(|e| crate::Error::Server(e.to_string()))
}
