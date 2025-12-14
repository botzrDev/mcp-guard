# Continue MCP Guard Development: Sprint 6 - Polish & Launch

We're building mcp-guard, a security gateway for MCP servers. Sprint 5 (Enterprise Features) is complete.

## Current State
- `cargo build && cargo test && cargo clippy` all pass (61 tests)
- 3 authentication providers: API Key, JWT (HS256/JWKS), OAuth 2.1 (PKCE)
- 3 transport types: Stdio, HTTP, SSE
- Prometheus metrics at /metrics
- OpenTelemetry tracing with W3C trace context propagation
- Per-identity rate limiting with Retry-After headers
- Tools/list response filtering based on identity permissions (FR-AUTHZ-03)

## Completed Sprints
- Sprint 1: Foundation (CLI, config, stdio transport, Axum server)
- Sprint 2: Core Security (JWT, rate limiting, Prometheus metrics)
- Sprint 3: OAuth 2.1 (PKCE, GitHub/Google/Custom providers)
- Sprint 4: OpenTelemetry (OTLP export, W3C trace context, correlation IDs)
- Sprint 5: Enterprise Features (HTTP/SSE transport, tools/list filtering)

## Sprint 6 Goal: Polish & Launch

Candidates from PRD (prioritize based on value):

### High Priority
1. **mTLS Client Certificate Validation** - Enterprise security requirement
   - Add `client_ca_path` to TlsConfig
   - Validate client certificates in middleware
   - Extract identity from certificate CN/SAN

2. **Documentation Polish**
   - Update README.md with HTTP/SSE transport docs
   - Add architecture diagram
   - Create quick-start guide
   - Add troubleshooting section

3. **Performance Benchmarking**
   - Create benchmark suite
   - Verify <2ms p99 latency target
   - Verify >5,000 RPS throughput target
   - Memory profiling (<50MB RSS)

### Medium Priority
4. **Multi-Server Routing** - Route to different upstreams based on path
   - Add `servers` array to UpstreamConfig
   - Path-based routing (e.g., /github/* -> github-mcp, /files/* -> fs-mcp)
   - Load balancing support

5. **Redis-Backed Rate Limiting** - For horizontal scaling
   - Add Redis connection config
   - Implement distributed rate limiter
   - Fallback to in-memory when Redis unavailable

6. **Audit Log Shipping** - HTTP export for SIEM integration
   - Add `export_url` to AuditConfig
   - Batch and ship logs to HTTP endpoint
   - Retry logic with backoff

### Lower Priority
7. **Health Check Improvements**
   - Add /ready endpoint (checks upstream connectivity)
   - Add /live endpoint (basic liveness)
   - Include version, uptime in health response

8. **CLI Improvements**
   - Add `mcp-guard check-upstream` command
   - Add `mcp-guard version` command
   - Colored output support

## Key Files
- `src/transport/mod.rs` - Transport trait, Stdio/HTTP/SSE implementations
- `src/authz/mod.rs` - Authorization logic, tools/list filtering
- `src/config/mod.rs` - All config types
- `src/server/mod.rs` - Axum server, middleware, OAuth endpoints
- `CLAUDE.md` - Project context and progress tracking
- `context/PRD.md` - Product Requirements Document

## Commands
```bash
cargo build                    # Build
cargo test                     # Run tests (61 total)
cargo clippy -- -D warnings    # Lint
cargo run -- --help            # CLI help
```

## Instructions
1. Read CLAUDE.md for full project context
2. Pick 2-3 high-value items from Sprint 6 candidates
3. Update CLAUDE.md sprint status before starting
4. Implement features with tests
5. Update CLAUDE.md with completion notes

Start by reading CLAUDE.md, then propose which Sprint 6 items to tackle first.
