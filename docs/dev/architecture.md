# Architecture Overview

This document describes the internal architecture of mcp-guard for developers who want to understand or extend the codebase.

## System Context

```
                         ┌─────────────────────────────────────────────────────────┐
                         │                      mcp-guard                          │
                         │                                                         │
  ┌─────────┐            │  ┌──────────┐   ┌──────────┐   ┌───────────────────┐   │            ┌─────────────┐
  │  Client │────HTTP────│─▶│Middleware│──▶│   Auth   │──▶│    Rate Limit     │───│───────────▶│  Upstream   │
  │ (Claude │            │  │  Chain   │   │ Provider │   │     Service       │   │            │ MCP Server  │
  │  Code)  │◀───────────│──│          │◀──│          │◀──│                   │◀──│────────────│             │
  └─────────┘            │  └──────────┘   └──────────┘   └───────────────────┘   │            └─────────────┘
                         │        │              │                  │             │
                         │        ▼              ▼                  ▼             │
                         │  ┌──────────┐   ┌──────────┐   ┌───────────────────┐   │
                         │  │ Metrics  │   │  Audit   │   │    Transport      │   │
                         │  │Prometheus│   │  Logger  │   │ (stdio/http/sse)  │   │
                         │  └──────────┘   └──────────┘   └───────────────────┘   │
                         │                       │                                │
                         │                       ▼                                │
                         │               ┌──────────────┐                         │
                         │               │ SIEM Export  │                         │
                         │               │   (HTTP)     │                         │
                         │               └──────────────┘                         │
                         └─────────────────────────────────────────────────────────┘
```

## Module Structure

```
src/
├── main.rs           # CLI dispatch, bootstrap, server startup
├── lib.rs            # Library root, module exports, Error enum
├── cli/              # CLI commands (init, validate, keygen, run, etc.)
├── config/           # Configuration types and parsing
├── auth/             # Authentication providers
│   ├── mod.rs        # AuthProvider trait, ApiKeyProvider, MultiProvider
│   ├── jwt.rs        # JwtProvider (HS256, JWKS)
│   ├── oauth.rs      # OAuthAuthProvider (PKCE, introspection)
│   └── mtls.rs       # MtlsAuthProvider (client certificates)
├── authz/            # Authorization logic
│   └── mod.rs        # Tool-level authorization, tools/list filtering
├── rate_limit/       # Rate limiting
│   └── mod.rs        # RateLimitService with DashMap + Governor
├── transport/        # MCP transports
│   └── mod.rs        # Transport trait, Stdio/HTTP/SSE implementations
├── router/           # Multi-server routing
│   └── mod.rs        # ServerRouter for path-based routing
├── server/           # HTTP server
│   └── mod.rs        # Axum router, middleware, endpoints
├── audit/            # Audit logging
│   └── mod.rs        # AuditLogger, AuditShipper for SIEM
└── observability/    # Metrics and tracing
    └── mod.rs        # Prometheus metrics, OpenTelemetry tracing
```

## Request Lifecycle

A request flows through these stages:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Request Processing                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. Connection        Incoming TCP connection on configured host:port       │
│         │                                                                   │
│         ▼                                                                   │
│  2. Security Headers  X-Content-Type-Options, X-Frame-Options, CSP added   │
│         │                                                                   │
│         ▼                                                                   │
│  3. Trace Context     W3C traceparent/tracestate extracted/propagated      │
│         │                                                                   │
│         ▼                                                                   │
│  4. Metrics           Request counter incremented, timer started           │
│         │                                                                   │
│         ▼                                                                   │
│  5. Auth Middleware   Token extracted, provider.authenticate() called      │
│         │             Identity stored in request extensions                │
│         │             Audit: auth_success or auth_failure                  │
│         ▼                                                                   │
│  6. Rate Limit        Per-identity check via RateLimitService             │
│         │             429 with Retry-After if exceeded                     │
│         │             Audit: rate_limited                                  │
│         ▼                                                                   │
│  7. Handler           MCP message parsed, authorization checked            │
│         │             tools/list filtered per identity.allowed_tools       │
│         ▼                                                                   │
│  8. Transport         Message forwarded to upstream via transport         │
│         │             Response received and returned to client            │
│         ▼                                                                   │
│  9. Response          Status recorded in metrics, duration histogram      │
│                       X-RateLimit-* headers added                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Concurrency Model

mcp-guard uses an async/await concurrency model built on Tokio:

### Runtime

- **Tokio multi-threaded runtime**: Default for production
- **Async handlers**: All HTTP handlers are async functions
- **Non-blocking I/O**: File and network operations never block worker threads

### Background Tasks

Several subsystems spawn background tasks:

| Task | Purpose | Coordination |
|------|---------|--------------|
| Audit Writer | Writes to file/stdout asynchronously | `mpsc` channel, shutdown signal |
| Audit Shipper | Batches and ships logs to SIEM | `mpsc` channel, flushes on drop |
| JWKS Refresher | Periodically refreshes JWKS keys | `CancellationToken` |
| Stdio Reader | Reads from subprocess stdout | `mpsc` channel |
| Stdio Writer | Writes to subprocess stdin | `mpsc` channel |

### Channel-Based Communication

```rust
// Audit logging uses non-blocking channels
let (tx, rx) = mpsc::channel::<AuditMessage>(AUDIT_CHANNEL_SIZE);

// Non-blocking send (drops if full)
tx.try_send(AuditMessage::Entry(json))?;

// Background task receives and writes
while let Some(msg) = rx.recv().await {
    // Write to file/stdout
}
```

### Graceful Shutdown

```rust
// CancellationToken coordinates shutdown
let shutdown_token = CancellationToken::new();

// Background tasks check for cancellation
tokio::select! {
    _ = do_work() => {}
    _ = shutdown_token.cancelled() => {
        // Cleanup and exit
    }
}

// Main triggers shutdown
shutdown_token.cancel();
audit_handle.shutdown().await;
```

## Error Handling

### Error Types

The crate defines a unified error hierarchy in `lib.rs`:

```rust
pub enum Error {
    Config(ConfigError),      // Configuration parsing/validation
    Auth(AuthError),          // Authentication failures
    Authz(String),            // Authorization denied
    RateLimited,              // Rate limit exceeded
    Transport(TransportError),// Upstream communication
    Router(RouterError),      // Multi-server routing
    Server(String),           // HTTP server errors
    Io(std::io::Error),       // File I/O
    Other(String),            // Catch-all
}
```

### HTTP Error Responses

`AppError` in `server/mod.rs` maps errors to HTTP responses:

| Variant | Status Code | Response |
|---------|-------------|----------|
| `Unauthorized` | 401 | `{"error": "...", "error_id": "uuid"}` |
| `Forbidden` | 403 | `{"error": "...", "error_id": "uuid"}` |
| `NotFound` | 404 | `{"error": "...", "error_id": "uuid"}` |
| `RateLimited` | 429 | Includes `Retry-After` header |
| `Transport` | 502 | Sanitized message (no internal details) |
| `Internal` | 500 | Generic message (details logged) |

### Security Considerations

- **Sanitized errors**: Transport errors never expose internal paths or commands
- **Error IDs**: Each error has a UUID for log correlation
- **Truncated bodies**: External response bodies truncated to 200 chars in logs

## Design Decisions

### Why Axum?

- **Async-first**: Built on Tokio, natural fit for async handlers
- **Type-safe extractors**: Request extensions for passing Identity
- **Composable middleware**: Tower middleware stack
- **Performance**: Zero-cost abstractions, minimal overhead

### Why Governor for Rate Limiting?

- **Token bucket algorithm**: Industry standard, predictable behavior
- **No external dependencies**: In-memory, no Redis required
- **Per-identity isolation**: DashMap provides O(1) lookup
- **Burst handling**: Configurable burst size for traffic spikes

### Why Channel-Based Audit Logging?

- **Non-blocking**: `try_send` never blocks async runtime
- **Backpressure handling**: Drops entries when buffer full (logs warning)
- **Clean shutdown**: Flushes pending entries before exit
- **Decoupled I/O**: File writes happen in dedicated task

### Why SSRF Validation?

HTTP/SSE transports validate URLs to prevent Server-Side Request Forgery:

```rust
// Blocked: private IPs, cloud metadata, non-HTTP schemes
validate_url_for_ssrf("http://169.254.169.254/...")?;  // Error
validate_url_for_ssrf("http://10.0.0.1/...")?;         // Error
validate_url_for_ssrf("file:///etc/passwd")?;          // Error
validate_url_for_ssrf("https://api.example.com")?;     // OK
```

### Why Command Validation?

Stdio transport validates commands to prevent shell injection:

```rust
// Blocked: shell metacharacters, direct shell execution
validate_command_for_injection("echo; rm -rf /")?;  // Error
validate_command_for_injection("bash")?;             // Error
validate_command_for_injection("node")?;             // OK
```

## State Management

### AppState

All shared state lives in `Arc<AppState>`:

```rust
pub struct AppState {
    pub config: Config,                           // Immutable config
    pub auth_provider: Arc<dyn AuthProvider>,     // Auth chain
    pub rate_limiter: RateLimitService,           // Per-identity limits
    pub audit_logger: Arc<AuditLogger>,           // Audit logging
    pub transport: Option<Arc<dyn Transport>>,    // Single-server mode
    pub router: Option<Arc<ServerRouter>>,        // Multi-server mode
    pub metrics_handle: PrometheusHandle,         // Metrics rendering
    pub oauth_provider: Option<Arc<OAuthAuthProvider>>,
    pub oauth_state_store: OAuthStateStore,       // PKCE states
    pub started_at: Instant,                      // For uptime
    pub ready: Arc<RwLock<bool>>,                 // Readiness flag
    pub mtls_provider: Option<Arc<MtlsAuthProvider>>,
}
```

### Thread Safety

- `Arc<T>`: Shared ownership across tasks
- `DashMap`: Concurrent hash map for rate limiters and PKCE states
- `RwLock`: Read-heavy access patterns (readiness flag)
- `Mutex`: Exclusive access (transport receive)

## Extension Points

### Adding a New Auth Provider

See [AuthProvider Trait Guide](./auth-provider.md).

### Adding a New Transport

See [Transport Trait Guide](./transport.md).

### Adding New Middleware

See [Middleware Chain](./middleware.md).

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs:30` | `bootstrap()` function - initialization |
| `src/server/mod.rs:967` | `build_router()` - route setup |
| `src/server/mod.rs:552` | `auth_middleware()` - authentication |
| `src/rate_limit/mod.rs:176` | `check()` - rate limit logic |
| `src/transport/mod.rs:133` | `validate_url_for_ssrf()` |
| `src/authz/mod.rs:95` | `filter_tools_list_response()` |
