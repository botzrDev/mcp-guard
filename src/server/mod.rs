//! Axum server and middleware for mcp-guard

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::Arc;
use std::time::Instant;
use tower_http::trace::TraceLayer;

use crate::audit::AuditLogger;
use crate::auth::AuthProvider;
use crate::config::Config;
use crate::observability::{record_auth, record_rate_limit, record_request, set_active_identities};
use crate::rate_limit::RateLimitService;
use crate::transport::{Message, Transport};

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub rate_limiter: RateLimitService,
    pub audit_logger: Arc<AuditLogger>,
    pub transport: Arc<dyn Transport>,
    pub metrics_handle: PrometheusHandle,
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

    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler))
        .merge(protected_routes)
        .layer(middleware::from_fn(metrics_middleware))
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
