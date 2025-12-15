# MCP-Guard v1.0 Release Checklist

## Overview

This document outlines all tasks required to make mcp-guard production-ready for a v1.0 release. Tasks are prioritized as **P0** (must have), **P1** (should have), or **P2** (nice to have).

---

## 1. Remaining Architectural Fixes

### P0: Graceful Shutdown Coordination ✅ COMPLETE
**Files:** `src/main.rs`, `src/audit/mod.rs`, `src/auth/jwt.rs`

**Status:** Implemented with CancellationToken. JWKS refresh, audit shipper, and transport
tasks all participate in graceful shutdown via tokio::select! with cancellation.

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

### P0: Input Validation ✅ COMPLETE
**Files:** `src/config/mod.rs`, `src/auth/jwt.rs`, `src/auth/oauth.rs`

**Status:** All validation implemented in `Config::validate()`:
- [x] JWKS URL must use HTTPS in production
- [x] OAuth redirect_uri must be valid URL
- [x] Rate limit values must be positive
- [x] Server port must be valid (1-65535)
- [x] Audit export URL validation
- [x] Tracing sample rate validation (0.0-1.0)

### P0: Replace Panicking unwrap() Calls ✅ COMPLETE
**Status:** Production code uses `unwrap_or()` with defaults or proper error handling.
Const unwraps for NonZeroU32 are compile-time safe.

### P1: Security Headers ✅ COMPLETE
**File:** `src/server/mod.rs`

**Status:** `security_headers_middleware` adds:
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- X-XSS-Protection: 1; mode=block
- Content-Security-Policy: default-src 'none'

### P1: Rate Limit Response Headers ✅ COMPLETE
**File:** `src/server/mod.rs`

**Status:** All responses include:
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
- [ ] Add `#![deny(missing_docs)]` to lib.rs (deferred - requires extensive doc comments)
- [x] Document all public types and functions (partial - key types documented)
- [x] Generate and host rustdoc (CI runs `cargo doc`)

### P0: README Completeness ✅ COMPLETE
All sections added:
- [x] Troubleshooting guide
- [x] Security considerations
- [x] Performance tuning
- [x] Comparison with alternatives

### P1: Examples Directory ✅ COMPLETE
Created `examples/` with:
- [x] `simple-api-key/` - Basic API key setup with README
- [x] `oauth-github/` - GitHub OAuth integration with README
- [x] `jwt-auth0/` - Auth0 JWT integration with README
- [x] `multi-server/` - Multi-server routing with README
- [x] `kubernetes/` - K8s deployment manifests with README

### P1: Architecture Documentation ✅ COMPLETE
- [x] `docs/ARCHITECTURE.md` - System design with module structure
- [x] `docs/SECURITY.md` - Security model and threat analysis

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

### P0: Version Bump ✅ COMPLETE
- [x] Update `Cargo.toml` version to `1.0.0`
- [x] Create `CHANGELOG.md` with comprehensive release notes
- [ ] Tag release in git (when ready to release)

### P0: Binary Releases ✅ COMPLETE
GitHub Actions workflow (`.github/workflows/release.yml`) builds for:
- [x] Linux x86_64 (gnu and musl static binary)
- [x] macOS x86_64
- [x] macOS aarch64 (Apple Silicon)
- [x] Windows x86_64
- [x] Automatic checksums generation
- [x] GitHub Release creation with artifacts

### P1: Container Image
- [ ] Create minimal `Dockerfile`
- [ ] Publish to Docker Hub / GHCR
- [ ] Multi-arch image support

### P1: Package Managers
- [ ] Homebrew formula
- [ ] AUR package
- [x] Cargo publish (crates.io) - release workflow includes this

---

## 7. CI/CD ✅ COMPLETE

### P0: Required Checks ✅ COMPLETE
All checks in `.github/workflows/ci.yml`:
- [x] `cargo test` (all tests pass)
- [x] `cargo clippy -- -D warnings` (no warnings)
- [x] `cargo fmt -- --check` (formatted)
- [x] `cargo audit` (via rustsec/audit-check)
- [x] `cargo deny check` (via EmbarkStudios/cargo-deny-action)

### P1: Additional Checks
- [x] Code coverage (cargo-llvm-cov with Codecov upload)
- [ ] Benchmark regression check
- [x] Binary size check - current: 8.6MB (target: <15MB) ✅
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
