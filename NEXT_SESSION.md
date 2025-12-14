# Continue MCP Guard Development: Sprint 6 - Final Polish

We're building mcp-guard, a security gateway for MCP servers. Sprint 6 is mostly complete.

## Current State
- `cargo build && cargo test && cargo clippy` all pass (77 tests)
- `cargo bench` runs performance benchmarks
- 4 authentication providers: API Key, JWT (HS256/JWKS), OAuth 2.1 (PKCE), mTLS
- 3 transport types: Stdio, HTTP, SSE
- Prometheus metrics at /metrics
- OpenTelemetry tracing with W3C trace context propagation
- Per-identity rate limiting with Retry-After headers
- Tools/list response filtering based on identity permissions
- Health check endpoints: /health, /live, /ready
- CLI commands: init, validate, keygen, run, hash-key, version, check-upstream

## Sprint 6 Completed
- ✅ Health check improvements (/health with uptime, /live, /ready with 503)
- ✅ mTLS client certificate authentication (via reverse proxy headers)
- ✅ Documentation polish (README updated with all features)
- ✅ CLI improvements (`version`, `check-upstream` commands)
- ✅ Performance benchmark suite (Criterion benchmarks)

## Sprint 6 Remaining (pick 1-2)

### High Value
1. **Multi-Server Routing** - Route to different upstreams based on path
   - Add `servers` array to UpstreamConfig with path patterns
   - Path-based routing (e.g., /github/* -> github-mcp, /filesystem/* -> fs-mcp)
   - Useful for organizations running multiple MCP servers

2. **Redis-Backed Rate Limiting** - For horizontal scaling
   - Add optional Redis connection config to RateLimitConfig
   - Implement distributed rate limiter using Redis
   - Fall back to in-memory when Redis not configured

### Medium Value
3. **Audit Log Shipping** - HTTP export for SIEM integration
   - Add `export_url` and `export_batch_size` to AuditConfig
   - Background task to batch and ship logs to HTTP endpoint
   - Useful for compliance and security monitoring

4. **WASM Compilation** - Edge deployment
   - Add wasm32-wasi target support
   - Cloudflare Workers compatibility
   - May require feature flags for some dependencies

### Nice to Have
5. **Config Hot Reload** - Update config without restart
   - Watch config file for changes
   - Reload API keys, rate limits without dropping connections

## Key Files
- `src/server/mod.rs` - Server, middleware, health endpoints
- `src/auth/` - Authentication providers (api_key, jwt, oauth, mtls)
- `src/config/mod.rs` - All config types
- `src/rate_limit/mod.rs` - Rate limiting service
- `benches/performance.rs` - Performance benchmarks
- `CLAUDE.md` - Project context and progress tracking

## Commands
```bash
cargo build                    # Build
cargo test                     # Run tests (77 total)
cargo clippy -- -D warnings    # Lint
cargo bench                    # Run performance benchmarks
cargo run -- version           # Show version info
cargo run -- check-upstream    # Test upstream connectivity
```

## Instructions

1. Read CLAUDE.md for full project context
2. Pick 1-2 items from remaining Sprint 6 work
3. Implement with tests
4. Update CLAUDE.md with completion notes
