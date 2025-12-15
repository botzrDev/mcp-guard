# MCP-Guard v1.0 Release Checklist

## Overview

This document outlines all tasks required to make mcp-guard production-ready for a v1.0 release. Tasks are prioritized as **P0** (must have), **P1** (should have), or **P2** (nice to have).

---

## 1. Remaining Architectural Fixes

### P0: Graceful Shutdown Coordination
**Files:** `src/main.rs`, `src/audit/mod.rs`, `src/auth/jwt.rs`

Background tasks don't participate in graceful shutdown:
- JWKS refresh task runs forever without cancellation
- Audit shipper may lose buffered logs on exit
- Transport tasks have no shutdown signal

**Implementation:**
```rust
// Add to main.rs
use tokio_util::sync::CancellationToken;

let shutdown_token = CancellationToken::new();

// Pass to background tasks
let jwks_shutdown = shutdown_token.clone();
tokio::spawn(async move {
    tokio::select! {
        _ = jwks_shutdown.cancelled() => {},
        _ = refresh_loop => {},
    }
});

// Handle SIGTERM/SIGINT
tokio::signal::ctrl_c().await?;
shutdown_token.cancel();
audit_handle.shutdown().await;
```

**Tests needed:**
- Test that SIGTERM flushes audit logs
- Test that JWKS refresh stops on shutdown
- Test graceful transport closure

---

### P1: AppState Refactoring
**File:** `src/server/mod.rs`

The `AppState` struct has 12 fields violating Single Responsibility. Split into:

```rust
pub struct AuthState {
    pub auth_provider: Arc<dyn AuthProvider>,
    pub oauth_provider: Option<Arc<OAuthAuthProvider>>,
    pub oauth_state_store: OAuthStateStore,
    pub mtls_provider: Option<Arc<MtlsAuthProvider>>,
}

pub struct TransportState {
    pub transport: Option<Arc<dyn Transport>>,
    pub router: Option<Arc<ServerRouter>>,
}

pub struct ObservabilityState {
    pub metrics_handle: PrometheusHandle,
    pub audit_logger: Arc<AuditLogger>,
}

pub struct AppState {
    pub config: Arc<Config>,
    pub auth: AuthState,
    pub transport: TransportState,
    pub observability: ObservabilityState,
    pub rate_limiter: RateLimitService,
    pub started_at: Instant,
    pub ready: Arc<RwLock<bool>>,
}
```

---

## 2. Security Hardening

### P0: Input Validation
**Files:** `src/config/mod.rs`, `src/auth/jwt.rs`, `src/auth/oauth.rs`

Add validation for:
- [ ] JWKS URL must use HTTPS in production
- [ ] OAuth redirect_uri must be valid URL
- [ ] API key minimum length (32 chars recommended)
- [ ] Rate limit values must be positive
- [ ] Server port must be valid (1-65535)

```rust
// Example validation in config/mod.rs
impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(ref jwt) = self.auth.jwt {
            if let Some(ref url) = jwt.jwks_url {
                if !url.starts_with("https://") && !cfg!(debug_assertions) {
                    return Err(ConfigError::Validation(
                        "JWKS URL must use HTTPS in production".into()
                    ));
                }
            }
        }
        // ... more validations
        Ok(())
    }
}
```

### P0: Replace Panicking unwrap() Calls
**Files:** Multiple (see grep results)

Production code locations with `unwrap()`:
- `src/rate_limit/mod.rs:88-89` - NonZeroU32 creation
- `src/observability/mod.rs:65` - tracing config
- `src/auth/oauth.rs:431` - SystemTime conversion
- `src/router/mod.rs:200` - best_match comparison

Replace with proper error handling or `expect()` with clear messages.

### P1: Security Headers
**File:** `src/server/mod.rs`

Add security headers middleware:
```rust
use tower_http::set_header::SetResponseHeaderLayer;

.layer(SetResponseHeaderLayer::overriding(
    header::X_CONTENT_TYPE_OPTIONS,
    HeaderValue::from_static("nosniff"),
))
.layer(SetResponseHeaderLayer::overriding(
    header::X_FRAME_OPTIONS,
    HeaderValue::from_static("DENY"),
))
```

### P1: Rate Limit Response Headers
**File:** `src/server/mod.rs`

Add standard rate limit headers:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`

---

## 3. Testing

### P0: Missing Test Coverage

**End-to-end tests needed:**
- [ ] Full OAuth flow (authorize -> callback -> token)
- [ ] Multi-server routing with different transport types
- [ ] Rate limiting across requests
- [ ] Audit log export to HTTP endpoint
- [ ] JWKS refresh with key rotation

**Concurrent operation tests:**
- [ ] OAuth state store cleanup under load
- [ ] Rate limiter with many concurrent identities
- [ ] Token cache eviction under pressure

**Error handling tests:**
- [ ] Upstream server crashes mid-request
- [ ] Network timeout handling
- [ ] Invalid JSON from upstream
- [ ] JWKS endpoint unavailable

### P1: Benchmark Validation
**File:** `benches/performance.rs`

Validate performance targets from PRD:
- [ ] Latency overhead <2ms p99
- [ ] Throughput >5,000 RPS
- [ ] Memory <50MB RSS

```bash
cargo bench
# Add CI job to fail if benchmarks regress >10%
```

### P1: Fuzzing
Add cargo-fuzz targets for:
- JSON-RPC message parsing
- Config file parsing
- JWT token validation

---

## 4. Documentation

### P0: API Documentation
- [ ] Add `#![deny(missing_docs)]` to lib.rs
- [ ] Document all public types and functions
- [ ] Generate and host rustdoc

### P0: README Completeness
Missing sections:
- [ ] Troubleshooting guide
- [ ] Upgrade guide
- [ ] Security considerations
- [ ] Performance tuning
- [ ] Comparison with alternatives

### P1: Examples Directory
Create `examples/` with:
- [ ] `simple-api-key/` - Basic API key setup
- [ ] `oauth-github/` - GitHub OAuth integration
- [ ] `jwt-auth0/` - Auth0 JWT integration
- [ ] `multi-server/` - Multi-server routing
- [ ] `kubernetes/` - K8s deployment manifests

### P1: Architecture Documentation
- [ ] `docs/ARCHITECTURE.md` - System design
- [ ] `docs/SECURITY.md` - Security model
- [ ] Sequence diagrams for auth flows

---

## 5. Operational Readiness

### P0: Structured Logging Consistency
**Files:** Multiple

Ensure all log statements use structured fields:
```rust
// Bad
tracing::info!("User {} authenticated", user_id);

// Good
tracing::info!(user_id = %user_id, "User authenticated");
```

Audit all `tracing::*` calls for consistency.

### P0: Error Messages
Review all error messages for:
- [ ] No sensitive data leakage
- [ ] Actionable information for operators
- [ ] Correlation IDs included

### P1: Metrics Completeness
Add missing metrics:
- [ ] `mcp_guard_upstream_latency_seconds` histogram
- [ ] `mcp_guard_cache_hits_total` / `cache_misses_total`
- [ ] `mcp_guard_config_reload_total`
- [ ] `mcp_guard_connection_pool_size` gauge

### P1: Health Check Enhancement
Enhance `/ready` to check:
- [ ] Upstream connectivity
- [ ] JWKS freshness (if configured)
- [ ] Rate limiter state

---

## 6. Release Artifacts

### P0: Version Bump
- [ ] Update `Cargo.toml` version to `1.0.0`
- [ ] Create `CHANGELOG.md`
- [ ] Tag release in git

### P0: Binary Releases
Set up GitHub Actions for:
- [ ] Linux x86_64 (musl static binary)
- [ ] Linux aarch64
- [ ] macOS x86_64
- [ ] macOS aarch64 (Apple Silicon)
- [ ] Windows x86_64

### P1: Container Image
- [ ] Create minimal `Dockerfile`
- [ ] Publish to Docker Hub / GHCR
- [ ] Multi-arch image support

### P1: Package Managers
- [ ] Homebrew formula
- [ ] AUR package
- [ ] Cargo publish (crates.io)

---

## 7. CI/CD

### P0: Required Checks
- [ ] `cargo test` (all tests pass)
- [ ] `cargo clippy -- -D warnings` (no warnings)
- [ ] `cargo fmt -- --check` (formatted)
- [ ] `cargo audit` (no vulnerabilities)
- [ ] `cargo deny check` (license compliance)

### P1: Additional Checks
- [ ] Code coverage (target: 80%)
- [ ] Benchmark regression check
- [ ] Binary size check (<15MB)
- [ ] Memory leak check (valgrind)

---

## 8. Pre-Release Testing

### P0: Manual Testing Checklist
- [ ] Fresh install from binary
- [ ] `mcp-guard init` creates valid config
- [ ] `mcp-guard keygen` generates working keys
- [ ] `mcp-guard validate` catches config errors
- [ ] `mcp-guard run` starts successfully
- [ ] API key authentication works
- [ ] JWT authentication works
- [ ] Rate limiting enforces limits
- [ ] Metrics endpoint returns Prometheus format
- [ ] Health endpoints return correct status

### P0: Integration Testing
- [ ] Test with Claude Desktop
- [ ] Test with popular MCP servers (filesystem, git, etc.)
- [ ] Test OAuth flow with GitHub
- [ ] Test OAuth flow with Google

---

## Task Summary

| Priority | Category | Count |
|----------|----------|-------|
| P0 | Must Have | 18 |
| P1 | Should Have | 16 |
| P2 | Nice to Have | 0 |

**Estimated effort:** 3-5 days for P0, additional 2-3 days for P1

---

## Command to Start

```
Implement the P0 items from RELEASE_CHECKLIST.md to prepare mcp-guard for v1.0 release.
Start with graceful shutdown coordination, then input validation, then replace
panicking unwrap() calls. After each fix, run `cargo test && cargo clippy`.
Update CHANGELOG.md as you go.
```
