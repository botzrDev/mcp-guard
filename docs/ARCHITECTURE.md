# MCP Guard Architecture

## Overview

mcp-guard is a security gateway for Model Context Protocol (MCP) servers. It intercepts requests between MCP clients and servers, providing authentication, authorization, rate limiting, and observability.

```
┌─────────────────────────────────────────────────────────────────────┐
│                          mcp-guard                                   │
│  ┌─────────────────────────────────────────────────────────────────┐│
│  │                    Request Pipeline                              ││
│  │                                                                  ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ ││
│  │  │ Security │→│   Auth   │→│   Rate   │→│     Transport      │ ││
│  │  │ Headers  │  │Middleware│  │  Limit   │  │   (send/recv)    │ ││
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────────────┘ ││
│  │       ↓              ↓             ↓                ↓           ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ ││
│  │  │ Metrics  │  │  Audit   │  │ Identity │  │  Tools Filtering │ ││
│  │  │ Record   │  │  Logger  │  │  Store   │  │  (FR-AUTHZ-03)   │ ││
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────────────┘ ││
│  └─────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

## Module Structure

```
src/
├── main.rs           # Entry point, CLI dispatch
├── lib.rs            # Library exports
├── cli/              # CLI commands (init, validate, keygen, run)
├── config/           # Configuration types and validation
├── auth/             # Authentication providers
│   ├── mod.rs        # AuthProvider trait, ApiKeyProvider, MultiProvider
│   ├── jwt.rs        # JwtProvider (HS256/JWKS)
│   ├── oauth.rs      # OAuthAuthProvider (PKCE)
│   └── mtls.rs       # MtlsAuthProvider
├── authz/            # Authorization logic and tools filtering
├── rate_limit/       # Per-identity rate limiting
├── audit/            # Audit logging and SIEM export
├── transport/        # MCP transports (stdio, HTTP, SSE)
├── router/           # Multi-server routing
├── server/           # Axum server, middleware, handlers
└── observability/    # Metrics and tracing
```

## Key Components

### Authentication (`src/auth/`)

The `AuthProvider` trait defines the authentication interface:

```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<Identity, AuthError>;
    fn name(&self) -> &str;
}
```

**Providers:**

| Provider | Mode | Use Case |
|----------|------|----------|
| `ApiKeyProvider` | SHA256 hash comparison | Service-to-service, simple setups |
| `JwtProvider` | HS256 (simple) or JWKS (RS256/ES256) | Enterprise SSO integration |
| `OAuthAuthProvider` | Token introspection + UserInfo | User authentication |
| `MtlsAuthProvider` | Client certificate headers | High-security environments |
| `MultiProvider` | Tries providers in order | Mixed authentication |

### Rate Limiting (`src/rate_limit/`)

Token bucket algorithm with per-identity limits:

```rust
pub struct RateLimitService {
    default_rps: u32,
    default_burst: u32,
    identity_limiters: DashMap<String, RateLimitEntry>,
    entry_ttl: Duration,  // 1 hour default
}
```

**Features:**
- Lazy limiter creation per identity
- TTL-based cleanup (prevents unbounded memory growth)
- Custom rate limits per identity
- Retry-After header on 429 responses

### Transport (`src/transport/`)

The `Transport` trait abstracts MCP server communication:

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: Message) -> Result<(), TransportError>;
    async fn receive(&self) -> Result<Message, TransportError>;
}
```

**Implementations:**

| Transport | Protocol | Use Case |
|-----------|----------|----------|
| `StdioTransport` | stdin/stdout | Local MCP processes |
| `HttpTransport` | HTTP POST | Remote MCP servers |
| `SseTransport` | Server-Sent Events | Streaming responses |

### Router (`src/router/`)

Multi-server routing with longest-prefix matching:

```rust
pub struct ServerRouter {
    routes: Vec<Route>,  // Sorted by prefix length (longest first)
}

impl ServerRouter {
    pub fn get_transport(&self, path: &str) -> Option<Arc<dyn Transport>>;
}
```

### Audit Logging (`src/audit/`)

Non-blocking audit logging with SIEM export:

```rust
pub struct AuditLogger {
    file_sender: Option<Sender<AuditEntry>>,
    stdout_sender: Option<Sender<AuditEntry>>,
    export_sender: Option<Sender<AuditEntry>>,
}
```

**Features:**
- Channel-based async I/O (non-blocking)
- Multiple outputs (file, stdout, HTTP)
- HTTP batching with exponential backoff
- Configurable batch size and flush interval

## Request Flow

### Single-Server Mode

```
1. Client Request → POST /mcp
2. Security Headers Middleware (adds X-Content-Type-Options, etc.)
3. Trace Context Middleware (W3C traceparent extraction)
4. Metrics Middleware (records duration)
5. Auth Middleware:
   a. Check mTLS headers (if configured)
   b. Extract Bearer token
   c. Authenticate via provider(s)
   d. Check rate limit
   e. Add Identity to request extensions
6. MCP Handler:
   a. Forward to transport.send()
   b. Wait for transport.receive()
   c. Filter tools/list response (if applicable)
7. Add x-ratelimit-* headers
8. Return response
```

### Multi-Server Mode

```
1. Client Request → POST /mcp/:server_name
2. ... (same middleware stack)
6. MCP Handler:
   a. router.get_transport(path)
   b. Forward to selected transport
   c. ...
```

## Concurrency Model

- **Async Runtime**: Tokio multi-threaded
- **Shared State**: `Arc<AppState>` with interior mutability where needed
- **Rate Limiters**: `DashMap` for lock-free concurrent access
- **Background Tasks**:
  - JWKS refresh (with CancellationToken)
  - Audit log shipping (channel-based)
  - OAuth state cleanup (periodic)

## Configuration

Configuration is loaded from TOML or YAML with validation:

```rust
impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        // Parse file
        // Validate (port, URLs, rate limits, etc.)
        Ok(config)
    }
}
```

**Validation rules:**
- Port must be 1-65535
- JWKS URLs must use HTTPS in production
- OAuth redirect_uri must be valid URL
- Rate limit values must be positive
- Tracing sample rate must be 0.0-1.0

## Error Handling

All errors include a unique `error_id` for correlation:

```rust
pub struct AppError {
    pub error_id: String,  // UUID v4
    pub kind: AppErrorKind,
}
```

**Error sanitization:**
- Transport errors don't expose internal paths
- Internal errors return generic message to client
- Full errors logged server-side with error_id

## Observability

### Metrics (Prometheus)

| Metric | Type | Labels |
|--------|------|--------|
| `mcp_guard_requests_total` | Counter | method, status |
| `mcp_guard_request_duration_seconds` | Histogram | method |
| `mcp_guard_auth_total` | Counter | provider, result |
| `mcp_guard_rate_limit_total` | Counter | allowed |
| `mcp_guard_active_identities` | Gauge | - |

### Tracing (OpenTelemetry)

- W3C trace context propagation
- OTLP gRPC export
- Configurable sampling
- Trace ID in logs

## Security Design

### Defense in Depth

1. **Transport Security**: TLS via reverse proxy
2. **Authentication**: Multiple provider support
3. **Authorization**: Per-tool permissions
4. **Rate Limiting**: Per-identity limits
5. **Input Validation**: Config validation, URL validation
6. **Output Sanitization**: Error message filtering

### Credential Handling

- API keys: Only hashes stored
- JWT secrets: JWKS preferred (no local secrets)
- OAuth: Token cache with LRU eviction

## SOLID Principles

- **S (Single Responsibility)**: Each module has one job
- **O (Open/Closed)**: AuthProvider trait allows extension
- **L (Liskov Substitution)**: All auth providers interchangeable
- **I (Interface Segregation)**: Separate Transport, AuthProvider traits
- **D (Dependency Inversion)**: AppState depends on abstractions
