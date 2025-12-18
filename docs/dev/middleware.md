# Middleware Chain

This guide explains the middleware architecture in mcp-guard and how to add new middleware.

## Overview

mcp-guard uses Axum's middleware system, which is built on Tower. Middleware wraps handlers and can modify requests/responses, short-circuit requests, or add data to the request context.

## Current Middleware Chain

Requests flow through middleware in this order:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      Incoming Request                                │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│ 1. Security Headers Layer                                            │
│    Adds: X-Content-Type-Options, X-Frame-Options, CSP, X-XSS        │
│    Source: server/mod.rs:904                                        │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│ 2. Trace Context Layer (if tracing enabled)                         │
│    Extracts: traceparent, tracestate headers                        │
│    Creates: New span with trace ID                                  │
│    Source: server/mod.rs:844                                        │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│ 3. Request Duration Layer                                            │
│    Records: Request timing for metrics                              │
│    Source: server/mod.rs:773                                        │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│ 4. Authentication Layer (protected routes only)                      │
│    Extracts: Bearer token from Authorization header                 │
│    Validates: Via AuthProvider                                      │
│    Stores: Identity in request extensions                           │
│    Source: server/mod.rs:552                                        │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Route Handler                                │
└─────────────────────────────────────────────────────────────────────┘
```

## Middleware Details

### Security Headers Layer

```rust
// src/server/mod.rs:904-930

async fn security_headers_layer<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Prevent MIME type sniffing
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );

    // Prevent clickjacking
    headers.insert(
        "X-Frame-Options",
        HeaderValue::from_static("DENY"),
    );

    // Content Security Policy
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
    );

    // XSS Protection (legacy browsers)
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    response
}
```

### Trace Context Layer

Propagates W3C trace context for distributed tracing:

```rust
// src/server/mod.rs:844-900

async fn trace_context_layer<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // Extract incoming trace context
    let traceparent = request.headers()
        .get("traceparent")
        .and_then(|v| v.to_str().ok());

    let tracestate = request.headers()
        .get("tracestate")
        .and_then(|v| v.to_str().ok());

    // Create span with extracted context
    let span = if let Some(tp) = traceparent {
        tracing::info_span!(
            "http_request",
            traceparent = %tp,
            tracestate = tracestate.unwrap_or("")
        )
    } else {
        tracing::info_span!("http_request")
    };

    // Execute request within span
    async move {
        let mut response = next.run(request).await;

        // Propagate trace context in response
        if let Some(trace_id) = current_trace_id() {
            response.headers_mut().insert(
                "X-Trace-ID",
                HeaderValue::from_str(&trace_id).unwrap_or_else(|_| HeaderValue::from_static("")),
            );
        }

        response
    }
    .instrument(span)
    .await
}
```

### Request Duration Layer

Records timing metrics for Prometheus:

```rust
// src/server/mod.rs:773-810

async fn request_duration_layer<B>(
    State(state): State<Arc<AppState>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().to_string();
    let start = std::time::Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status().as_u16();

    // Record to Prometheus
    record_request(&method, status, duration);

    // Update active identities gauge
    set_active_identities(state.rate_limiter.tracked_identities());

    response
}
```

### Authentication Layer

The most complex middleware - handles token extraction and validation:

```rust
// src/server/mod.rs:552-770

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, AppError> {
    // 1. Check for mTLS authentication first (if configured)
    if let Some(ref mtls) = state.mtls_provider {
        if let Some(cert_info) = extract_client_cert_info(&request) {
            // Validate against trusted proxy IPs
            if let Ok(identity) = mtls.authenticate_cert(&cert_info).await {
                request.extensions_mut().insert(identity);
                return Ok(next.run(request).await);
            }
        }
    }

    // 2. Extract Bearer token
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            state.audit_logger.log_auth_failure("Missing credentials");
            AppError::Unauthorized("Missing or invalid Authorization header".into())
        })?;

    // 3. Authenticate via provider(s)
    let identity = state.auth_provider.authenticate(token).await.map_err(|e| {
        state.audit_logger.log_auth_failure(&format!("{:?}", e));
        record_auth(state.auth_provider.name(), false);
        AppError::Unauthorized(format!("Authentication failed: {:?}", e))
    })?;

    // 4. Record success and store identity
    state.audit_logger.log_auth_success(&identity.id);
    record_auth(state.auth_provider.name(), true);
    request.extensions_mut().insert(identity);

    // 5. Continue to handler
    Ok(next.run(request).await)
}
```

## Request Extensions

Middleware communicates with handlers via request extensions:

```rust
// In middleware: Store identity
request.extensions_mut().insert(identity);

// In handler: Retrieve identity
async fn handler(
    Extension(identity): Extension<Identity>,
    // ...
) -> Response {
    // Use identity
}
```

## Adding New Middleware

### Step 1: Define the Middleware Function

```rust
use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};

async fn my_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    // Pre-processing
    tracing::info!("Request: {} {}", request.method(), request.uri());

    // Call the next handler
    let response = next.run(request).await;

    // Post-processing
    tracing::info!("Response: {}", response.status());

    response
}
```

### Step 2: Middleware with State

```rust
use axum::extract::State;

async fn stateful_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Access shared state
    let _count = state.rate_limiter.tracked_identities();

    next.run(request).await
}
```

### Step 3: Middleware that Returns Errors

```rust
async fn validating_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Validate something
    if request.headers().get("X-Required-Header").is_none() {
        return Err(AppError::BadRequest("Missing required header".into()));
    }

    Ok(next.run(request).await)
}
```

### Step 4: Wire into Router

```rust
// src/server/mod.rs build_router()

fn build_router(state: Arc<AppState>) -> Router {
    // Public routes (no auth)
    let public_routes = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler));

    // Protected routes (with auth)
    let protected_routes = Router::new()
        .route("/mcp", post(mcp_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        // Add your middleware here
        .layer(middleware::from_fn(my_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // Global middleware (applies to all routes)
        .layer(middleware::from_fn(security_headers_layer))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            request_duration_layer,
        ))
        .with_state(state)
}
```

## Layer Order

**Important**: In Axum, layers are applied in reverse order. The last layer added is the first to process requests.

```rust
Router::new()
    .layer(layer_3)  // Runs 3rd on request, 1st on response
    .layer(layer_2)  // Runs 2nd on request, 2nd on response
    .layer(layer_1)  // Runs 1st on request, 3rd on response
```

For mcp-guard's middleware order:
```rust
Router::new()
    // Auth runs last (innermost), closest to handler
    .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
    // Duration runs before auth
    .layer(middleware::from_fn_with_state(state.clone(), request_duration_layer))
    // Trace context runs before duration
    .layer(middleware::from_fn(trace_context_layer))
    // Security headers runs first (outermost)
    .layer(middleware::from_fn(security_headers_layer))
```

## Error Handling in Middleware

Middleware can return errors that become HTTP responses:

```rust
// Define error type
pub enum AppError {
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    RateLimited { retry_after: u64 },
    Internal(String),
}

// Implement IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::RateLimited { retry_after } => {
                let mut response = (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded".to_string(),
                ).into_response();
                response.headers_mut().insert(
                    "Retry-After",
                    HeaderValue::from_str(&retry_after.to_string()).unwrap(),
                );
                return response;
            }
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_id = uuid::Uuid::new_v4().to_string();
        let body = serde_json::json!({
            "error": message,
            "error_id": error_id,
        });

        (status, Json(body)).into_response()
    }
}
```

## Common Patterns

### Request Logging

```rust
async fn request_logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status.as_u16(),
        duration_ms = %duration.as_millis(),
        "Request completed"
    );

    response
}
```

### Rate Limit Headers

```rust
async fn rate_limit_headers_middleware(
    State(state): State<Arc<AppState>>,
    Extension(identity): Extension<Identity>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Check rate limit
    let result = state.rate_limiter.check(&identity.id, identity.rate_limit);

    if !result.allowed {
        return AppError::RateLimited {
            retry_after: result.retry_after_secs.unwrap_or(1),
        }.into_response();
    }

    let mut response = next.run(request).await;

    // Add rate limit headers
    let headers = response.headers_mut();
    headers.insert("X-RateLimit-Limit", result.limit.into());
    headers.insert("X-RateLimit-Remaining", result.remaining.into());
    headers.insert("X-RateLimit-Reset", result.reset_at.into());

    response
}
```

### Request ID

```rust
async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Generate or extract request ID
    let request_id = request
        .headers()
        .get("X-Request-ID")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Store in extensions for handlers
    request.extensions_mut().insert(RequestId(request_id.clone()));

    let mut response = next.run(request).await;

    // Echo back in response
    response.headers_mut().insert(
        "X-Request-ID",
        HeaderValue::from_str(&request_id).unwrap(),
    );

    response
}

#[derive(Clone)]
struct RequestId(String);
```

## Testing Middleware

```rust
#[tokio::test]
async fn test_security_headers() {
    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .layer(middleware::from_fn(security_headers_layer));

    let response = app
        .oneshot(Request::get("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(
        response.headers().get("X-Content-Type-Options"),
        Some(&HeaderValue::from_static("nosniff"))
    );
    assert_eq!(
        response.headers().get("X-Frame-Options"),
        Some(&HeaderValue::from_static("DENY"))
    );
}
```
