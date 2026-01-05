# CLAUDE.md - MCP Guard Project Context

> **Last Updated:** 2025-12-14
> **Current Sprint:** Sprint 6 (Complete!)
> **Project Status:** 🟢 All Sprints Complete - Ready for v1.0 Launch

---

## Project Overview

**mcp-guard** is a lightweight, high-performance security gateway for Model Context Protocol (MCP) servers, written in Rust.

### One-Liner
> "Secure your MCP servers in 5 minutes. No Docker. No Kubernetes. No DevOps team."

### Key Value Props
1. Zero Infrastructure — Single binary, no containers
2. 5-Minute Setup — Install → Configure → Secure
3. Production-Grade — OAuth 2.1, JWT, API keys, per-tool authorization
4. Observable — Audit logs, Prometheus metrics, OpenTelemetry traces
5. Edge-Ready — WASM compilation for Cloudflare Workers

---

## Quick Reference

### Project Structure (Target)
```
mcp-guard/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library root
│   ├── cli/             # CLI commands
│   ├── config/          # Configuration types
│   ├── auth/            # Authentication providers
│   ├── authz/           # Authorization logic
│   ├── rate_limit/      # Rate limiting service
│   ├── audit/           # Audit logging
│   ├── transport/       # MCP transports (stdio, http)
│   ├── server/          # Axum server & middleware
│   └── observability/   # Metrics & tracing
├── templates/           # Config templates
├── tests/               # Integration & E2E tests
├── examples/            # Usage examples
└── docs/                # Documentation
```

### Key Commands
```bash
# Unified Development (start all services)
./scripts/dev.sh start         # Start backend + frontend
./scripts/dev.sh stop          # Stop all services gracefully
./scripts/dev.sh restart       # Restart all services
./scripts/dev.sh status        # Show service status
./scripts/dev.sh logs          # Tail all logs

# Or use make shortcuts
make dev                       # Start all services
make dev-stop                  # Stop all services
make dev-status                # Show status

# Backend Only
cargo build                    # Build debug
cargo test                     # Run tests
cargo clippy -- -D warnings    # Lint

# CLI Usage
mcp-guard init                 # Generate config
mcp-guard validate             # Check config
mcp-guard keygen --user-id X   # Generate API key
mcp-guard run                  # Start server
```


### Performance Targets
| Metric | Target |
|--------|--------|
| Latency overhead | <2ms p99 |
| Throughput | >5,000 RPS |
| Memory | <50MB RSS |
| Binary size | <15MB |

---

## Current Sprint Status

### Sprint 1: Foundation
**Goal:** Get minimal working proxy with basic security infrastructure

| Task | Status | Notes |
|------|--------|-------|
| Initialize Rust project | ✅ Done | cargo init |
| Set up project structure | ✅ Done | Per PRD 2.9 spec |
| Create Cargo.toml with deps | ✅ Done | axum, tokio, governor, etc. |
| Set up GitHub Actions CI | ✅ Done | build, test, clippy, fmt |
| Create lib.rs with modules | ✅ Done | All 9 modules declared |
| Implement CLI structure | ✅ Done | init, validate, keygen, run |
| Create config types | ✅ Done | YAML/TOML parsing |
| Implement stdio transport | ✅ Done | MCP JSON-RPC passthrough |
| Set up Axum server | ✅ Done | With auth middleware |
| Write integration tests | ✅ Done | 18 tests passing |

### Sprint 2: Core Security (Complete ✅)
**Goal:** Production-ready authentication with JWT and enhanced rate limiting

| Task | Status | Notes |
|------|--------|-------|
| Add JWT authentication provider | ✅ Done | HS256 + JWKS modes |
| Implement JWKS cache with auto-refresh | ✅ Done | 1hr default TTL |
| Add scope-to-tool mapping | ✅ Done | FR-AUTHZ-06 |
| Wire up MultiProvider | ✅ Done | API key + JWT |
| Update config templates | ✅ Done | Auth0, Keycloak examples |
| Add JWT unit tests | ✅ Done | 10 tests |
| Implement per-identity rate limiting | ✅ Done | DashMap + Retry-After header |
| Add metrics endpoint (/metrics) | ✅ Done | Prometheus format, 5 metrics |
| Write CLI documentation | ⬜ Pending | Moved to Sprint 4 |

### Sprint 3: Auth Expansion (Complete ✅)
**Goal:** OAuth 2.1 authentication provider with PKCE support

| Task | Status | Notes |
|------|--------|-------|
| Add OAuthConfig with provider enum | ✅ Done | GitHub, Google, Okta, Custom |
| Create OAuthAuthProvider struct | ✅ Done | src/auth/oauth.rs |
| Implement token validation | ✅ Done | Introspection + UserInfo fallback |
| Add PKCE support | ✅ Done | S256 code challenge |
| Add /oauth/authorize endpoint | ✅ Done | Initiates OAuth flow |
| Add /oauth/callback endpoint | ✅ Done | Exchanges code for tokens |
| Wire into MultiProvider | ✅ Done | API key + JWT + OAuth |
| Update config template | ✅ Done | GitHub, Google, Custom examples |
| Add OAuth unit tests | ✅ Done | 11 tests |

### Sprint 4: Advanced Observability (Complete ✅)
**Goal:** OpenTelemetry distributed tracing with W3C trace context

| Task | Status | Notes |
|------|--------|-------|
| Add TracingConfig to config | ✅ Done | service_name, otlp_endpoint, sample_rate |
| Add OpenTelemetry dependencies | ✅ Done | opentelemetry 0.27, tracing-opentelemetry 0.28 |
| Implement OTLP exporter | ✅ Done | Supports Jaeger, Tempo, etc. |
| Create trace context middleware | ✅ Done | W3C traceparent/tracestate propagation |
| Wire trace_id into logs | ✅ Done | FR-AUDIT-06 correlation ID |
| Add TracingGuard for shutdown | ✅ Done | Proper cleanup on exit |
| Update config template | ✅ Done | Jaeger, Tempo examples |
| Add tracing tests | ✅ Done | 49 tests passing |

### Sprint 5: Enterprise Features (Complete ✅)
**Goal:** HTTP/SSE transport and advanced authorization

| Task | Status | Notes |
|------|--------|-------|
| Implement HTTP transport | ✅ Done | POST JSON-RPC to upstream URL |
| Implement SSE transport | ✅ Done | Server-Sent Events for streaming |
| Add tools/list filtering | ✅ Done | FR-AUTHZ-03 |
| Update config template | ✅ Done | HTTP/SSE examples |
| Add transport tests | ✅ Done | 61 tests passing |

### Sprint 6: Polish & Launch (Complete ✅)
**Goal:** Production polish, enterprise features, and documentation

| Task | Status | Notes |
|------|--------|-------|
| Add /health endpoint improvements | ✅ Done | version, uptime_secs |
| Add /live liveness endpoint | ✅ Done | Kubernetes probe |
| Add /ready readiness endpoint | ✅ Done | 503 when not ready |
| Add mTLS config types | ✅ Done | MtlsConfig, MtlsIdentitySource |
| Create MtlsAuthProvider | ✅ Done | Header-based cert extraction |
| Update auth middleware for mTLS | ✅ Done | Checks headers before Bearer |
| Update README with HTTP/SSE docs | ✅ Done | Transport examples |
| Update README with health endpoints | ✅ Done | Response formats |
| Update README with mTLS docs | ✅ Done | Nginx example |
| Update config template | ✅ Done | mTLS section |
| Add mTLS tests | ✅ Done | 8 new tests |
| Add `version` CLI command | ✅ Done | Shows build info and features |
| Add `check-upstream` CLI command | ✅ Done | Tests upstream connectivity |
| Create performance benchmark suite | ✅ Done | Criterion benchmarks for auth, rate limit, authz |
| Add CLI tests | ✅ Done | 4 new tests (77 total) |
| **Multi-Server Routing** | ✅ Done | Path-based routing to multiple upstreams |
| **Audit Log Shipping** | ✅ Done | HTTP export with batching for SIEM integration |

### Next Actions
1. [x] Write CLI documentation - Added to README.md
2. [x] Start Sprint 5: Enterprise Features
3. [x] Start Sprint 6: Polish & Launch
4. [x] Implement Multi-Server Routing
5. [x] Implement Audit Log Shipping

---

## Sprint Roadmap

| Sprint | Weeks | Goal | Status |
|--------|-------|------|--------|
| Sprint 1 | 1-2 | Foundation (working proxy) | ✅ Complete |
| Sprint 2 | 3-4 | Core Security (JWT + rate limit + metrics) | ✅ Complete |
| Sprint 3 | 5-6 | Auth Expansion (OAuth 2.1) | ✅ Complete |
| Sprint 4 | 7-8 | Advanced Observability (OpenTelemetry) | ✅ Complete |
| Sprint 5 | 9-10 | Enterprise Features | ✅ Complete |
| Sprint 6 | 11-12 | Polish & Launch | ✅ Complete |

---

## Technical Decisions Log

### Architecture
- **Framework:** Axum 0.7 (async web framework)
- **Runtime:** Tokio (async runtime)
- **Rate Limiting:** Governor crate (token bucket)
- **Auth:** jsonwebtoken for JWT, oauth2 for OAuth 2.1
- **License:** AGPL-3.0 (core) / Commercial (enterprise)

### Design Principles (SOLID)
- **S:** Each module has single responsibility
- **O:** AuthProvider trait allows extension without modification
- **L:** All auth providers are interchangeable
- **I:** Separate traits for Transport, AuthProvider
- **D:** AppState depends on abstractions (Arc<dyn AuthProvider>)

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point, CLI dispatch, provider wiring, metrics init |
| `src/lib.rs` | Library root, module exports |
| `src/config/mod.rs` | Config types: `JwtConfig`, `JwtMode`, `OAuthConfig`, `AuthConfig`, `TracingConfig`, `MtlsConfig`, `ServerRouteConfig` |
| `src/auth/mod.rs` | `AuthProvider` trait, `ApiKeyProvider`, `MultiProvider` |
| `src/auth/jwt.rs` | `JwtProvider` with HS256/JWKS support |
| `src/auth/oauth.rs` | `OAuthAuthProvider` with PKCE, introspection, userinfo |
| `src/auth/mtls.rs` | `MtlsAuthProvider` for client certificate auth via headers |
| `src/server/mod.rs` | Axum server, auth/metrics middleware, OAuth endpoints, health checks, /routes endpoint |
| `src/router/mod.rs` | `ServerRouter` for multi-server routing based on path prefix |
| `src/rate_limit/mod.rs` | Per-identity rate limiting with DashMap |
| `src/transport/mod.rs` | `Transport` trait, `StdioTransport`, `HttpTransport`, `SseTransport` |
| `src/audit/mod.rs` | Audit logging with HTTP export for SIEM integration, `AuditShipper` |
| `src/observability/mod.rs` | Prometheus metrics + OpenTelemetry tracing, TracingGuard |
| `templates/mcp-guard.toml` | Config template with JWT + OAuth + Tracing + mTLS + Multi-Server + Audit Export examples |
| `tests/integration_tests.rs` | Integration test suite |
| `benches/performance.rs` | Criterion performance benchmarks (auth, rate limit, authz, JSON) |
| `context/PRD.md` | Product Requirements Document |

---

## Memory & Progress Tracking

### Completed Milestones
- [x] 2025-12-14: Project planning complete, implementation plan approved
- [x] 2025-12-14: Sprint 1 Foundation complete - working Rust project with:
  - Full project structure (9 modules)
  - CLI with init/validate/keygen/run commands
  - API key authentication provider
  - Rate limiting with Governor
  - Stdio transport for MCP passthrough
  - Axum server with auth middleware
  - Audit logging infrastructure
  - GitHub Actions CI pipeline
  - 18 integration tests passing
- [x] 2025-12-14: JWT Authentication complete (Sprint 2 partial):
  - Dual-mode JWT: Simple (HS256) + JWKS (RS256/ES256)
  - JWKS cache with auto-refresh (1hr default TTL)
  - Scope-to-tool mapping (FR-AUTHZ-06)
  - MultiProvider for API key + JWT authentication
  - Config templates with Auth0, Keycloak examples
  - 10 new JWT unit tests (28 total tests passing)
- [x] 2025-12-14: Per-Identity Rate Limiting complete:
  - DashMap for per-identity rate limiter storage
  - Lazy limiter creation on first request per identity
  - Custom rate limit support from Identity.rate_limit
  - RateLimitResult struct with retry_after_secs
  - HTTP 429 responses include Retry-After header (FR-RATE-05)
  - 7 new rate limit unit tests (33 total tests passing)
- [x] 2025-12-14: Prometheus Metrics Endpoint complete (Sprint 2 DONE!):
  - `/metrics` endpoint with Prometheus format (text/plain)
  - `mcp_guard_requests_total` counter with method, status labels
  - `mcp_guard_request_duration_seconds` histogram with method label
  - `mcp_guard_auth_total` counter with provider, result labels
  - `mcp_guard_rate_limit_total` counter with allowed label
  - `mcp_guard_active_identities` gauge (from rate_limiter.tracked_identities())
  - Request duration middleware for timing all requests
  - Uses `metrics` + `metrics-exporter-prometheus` crates
  - 4 new tests (37 total tests passing)
- [x] 2025-12-14: OAuth 2.1 Implementation complete (Sprint 3 DONE!):
  - `OAuthAuthProvider` with token validation (introspection + userinfo fallback)
  - PKCE support with S256 code challenge generation
  - Known provider endpoints (GitHub, Google) with auto-configuration
  - Custom provider support with configurable URLs
  - `/oauth/authorize` endpoint initiates OAuth flow with PKCE
  - `/oauth/callback` endpoint exchanges code for tokens
  - Token caching to avoid repeated introspection calls
  - Scope-to-tool mapping (same as JWT)
  - Config template with GitHub, Google, Custom examples
  - 11 new OAuth tests (48 total tests passing)
- [x] 2025-12-14: OpenTelemetry Tracing complete (Sprint 4 DONE!):
  - `TracingConfig` with service_name, otlp_endpoint, sample_rate, propagate_context
  - `TracingGuard` for proper OpenTelemetry shutdown on exit
  - OTLP gRPC exporter for Jaeger, Tempo, and other collectors
  - W3C trace context middleware (traceparent/tracestate propagation)
  - Trace ID extraction from spans via `current_trace_id()` helper
  - Configurable sampling (0.0-1.0 ratio, AlwaysOn, AlwaysOff)
  - Config template with Jaeger and Tempo examples
  - 1 new tracing test (49 total tests passing)
- [x] 2025-12-14: HTTP/SSE Transport & Tools Filtering complete (Sprint 5 DONE!):
  - `HttpTransport` for connecting to upstream MCP servers over HTTP POST
  - `SseTransport` for Server-Sent Events streaming responses
  - FR-AUTHZ-03: `filter_tools_list_response` filters tools/list to show only authorized tools
  - `is_tools_list_request` helper function
  - Config template with HTTP/SSE transport examples
  - Automatic transport selection based on config (stdio/http/sse)
  - 7 new tests (61 total tests passing)
- [x] 2025-12-14: Sprint 6 Progress - Health Checks & mTLS:
  - `/health` endpoint with version and uptime_secs
  - `/live` endpoint for Kubernetes liveness probes
  - `/ready` endpoint with 503 status when not ready
  - `MtlsAuthProvider` for client certificate authentication via headers
  - `ClientCertInfo` extraction from reverse proxy headers (X-Client-Cert-CN, etc.)
  - `MtlsConfig` with identity_source (cn/san_dns/san_email), allowed_tools, rate_limit
  - TlsConfig.client_ca_path for mTLS configuration
  - Updated auth middleware to check mTLS headers before Bearer token
  - README updated with HTTP/SSE transport, health endpoints, mTLS docs
  - Config template with mTLS section and nginx examples
  - 12 new tests (73 total tests passing)
- [x] 2025-12-14: CLI Improvements & Benchmarks complete (Sprint 6 continued):
  - `mcp-guard version` command showing build info, version, and features
  - `mcp-guard check-upstream` command for testing upstream connectivity (stdio/http/sse)
  - Criterion benchmark suite in `benches/performance.rs`
  - Benchmarks cover: API key auth, rate limiting, authorization, tools filtering, crypto, JSON parsing
  - 4 new CLI tests (77 total tests passing)
  - Run benchmarks with `cargo bench`
- [x] 2025-12-14: Multi-Server Routing & Audit Log Shipping complete (Sprint 6 DONE!):
  - **Multi-Server Routing:**
    - `ServerRouteConfig` for configuring server routes with path prefixes
    - `ServerRouter` for path-based routing to multiple upstream MCP servers
    - `/routes` endpoint to list available server routes
    - Route to `/mcp/:server_name` in multi-server mode
    - Support for mixed transport types (stdio/http/sse) across servers
    - Longest-prefix matching for route selection
  - **Audit Log Shipping:**
    - `AuditShipper` background task for batching and shipping logs
    - HTTP export to SIEM endpoints (Splunk, Datadog, etc.)
    - Configurable batch size (default: 100) and flush interval (default: 30s)
    - Custom headers for authentication (API keys, tokens)
    - Exponential backoff retry (3 attempts)
    - Non-blocking log submission with channel-based buffering
  - Config template updated with multi-server routing and audit export examples
  - 5 new tests (82 total tests passing)
- [x] 2025-12-14: Architectural Audit & Fixes complete:
  - **Channel-based audit logger**: Replaced blocking `std::sync::Mutex` with async channel I/O
  - **Shared map_scopes_to_tools**: Extracted duplicated function to `src/auth/mod.rs`
  - **TTL-based rate limiter eviction**: Added 1-hour TTL and cleanup_expired() to prevent unbounded growth
  - **OAuth token cache eviction**: Added CACHE_MAX_ENTRIES (500) and LRU-style eviction
  - **MultiProvider error preservation**: Now returns informative errors instead of always `MissingCredentials`
  - **Transport task supervision**: Added JoinHandle storage and is_healthy() method for StdioTransport
  - **Magic numbers to constants**: Extracted timeouts, buffer sizes, TTLs across all modules
  - 2 new rate limit tests

### Blockers & Issues
*None currently*

### Notes for Future Sessions
- Sprint 6 COMPLETE - All core features implemented
- Architectural audit COMPLETE - Major anti-patterns fixed
- Current state: `cargo build && cargo test && cargo clippy` all pass (455 tests)
- 4 authentication providers: API Key, JWT (HS256/JWKS), OAuth 2.1 (PKCE), mTLS
- 3 transport types: Stdio, HTTP, SSE
- Key features for v1.0:
  - Multi-server routing for organizations running multiple MCP servers
  - Audit log shipping for SIEM integration and compliance
  - All health check endpoints (/health, /live, /ready)
  - Performance benchmarks for CI validation
- Key Sprint 6 files:
  - `src/router/mod.rs` - ServerRouter for multi-server routing
  - `src/audit/mod.rs` - AuditShipper for HTTP log export (now channel-based)
  - `src/server/mod.rs` - /routes endpoint, health checks
  - `src/config/mod.rs` - ServerRouteConfig, audit export config
  - `templates/mcp-guard.toml` - comprehensive config examples
- **v1.0 Release**: See `RELEASE_CHECKLIST.md` for remaining P0/P1 tasks

**Future Enhancements (post-v1.0):**
1. **Redis-Backed Rate Limiting** - For horizontal scaling with shared state
2. **Config Hot Reload** - Update config without restart (watch file for changes)
3. **WASM Compilation** - Edge deployment on Cloudflare Workers
4. **Graceful Shutdown Coordination** - CancellationToken for background tasks (see RELEASE_CHECKLIST.md)

---

## Business Context

### Revenue Targets
| Day | Target |
|-----|--------|
| 7 | First sponsor |
| 30 | $500 MRR |
| 90 | $2,000 MRR |

### Pricing Tiers (Founder)
| Tier | Price | Features |
|------|-------|----------|
| Free | $0 | API key + JWT HS256, stdio, global rate limit, prometheus |
| Pro | $12/mo | + OAuth 2.1, JWT JWKS, HTTP/SSE, per-identity rate limit |
| Enterprise | $29 + $8/seat | + mTLS, multi-server, SIEM, OpenTelemetry, admin tools |

---

## How to Use This File

This CLAUDE.md serves as persistent project memory. Update it when:

1. **Completing sprints** — Update Current Sprint Status section
2. **Making technical decisions** — Add to Technical Decisions Log
3. **Hitting milestones** — Add to Completed Milestones
4. **Encountering blockers** — Add to Blockers & Issues
5. **Session handoff** — Add notes to "Notes for Future Sessions"

---

*This file should be kept in sync with project progress. Read this first in each session.*
