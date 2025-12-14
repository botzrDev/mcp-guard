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
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::audit::AuditLogger;
use crate::auth::AuthProvider;
use crate::config::Config;
use crate::rate_limit::RateLimitService;
use crate::transport::{Message, Transport};

/// Application state shared across handlers
pub struct AppState {
    pub config: Config,
    pub auth_provider: Arc<dyn AuthProvider>,
    pub rate_limiter: RateLimitService,
    pub audit_logger: Arc<AuditLogger>,
    pub transport: Arc<dyn Transport>,
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

/// Authentication middleware
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

    // Authenticate
    let identity = state
        .auth_provider
        .authenticate(token)
        .await
        .map_err(|e| {
            state.audit_logger.log_auth_failure(&e.to_string());
            AppError::Unauthorized(e.to_string())
        })?;

    state.audit_logger.log_auth_success(&identity.id);

    // Check rate limit (per-identity)
    let rate_limit_result = state.rate_limiter.check(&identity.id, identity.rate_limit);
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
        .merge(protected_routes)
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
