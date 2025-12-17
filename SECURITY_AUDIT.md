# Security & Architectural Audit Report for mcp-guard v1.0

> **Audit Date:** 2025-12-15
> **Auditor:** Claude Code Security Review
> **Codebase Version:** v1.0.0 (commit 41e573d)

---

## Executive Summary

Overall, the codebase demonstrates good security practices with proper use of Rust's type system, async patterns, and defense-in-depth measures. However, this audit identified **7 critical/high-severity issues** and **9 medium/low-severity issues** that should be addressed before open-source release.

### Quick Stats
- **Critical/High Issues:** 7
- **Medium Issues:** 6
- **Low Issues:** 5
- **Dependencies with Advisories:** 1 (unmaintained)

---

## Issue Tracking

### Legend
- [ ] Not started
- [x] Fixed
- 🔴 Critical/High
- 🟡 Medium
- 🟢 Low

---

## 🔴 Critical / High Severity Issues

### Issue #1: SSRF Vulnerability in HTTP/SSE Transport
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 Critical
- **Location:** `src/transport/mod.rs:311-336`
- **Category:** Server-Side Request Forgery

**Description:**
The `HttpTransport` and `SseTransport` accept arbitrary URLs from configuration without validation. An attacker with config access could:
- Make requests to internal services (169.254.169.254 for cloud metadata)
- Probe internal network infrastructure
- Bypass firewall restrictions

**Vulnerable Code:**
```rust
// src/transport/mod.rs:311-312
pub fn new(url: String) -> Self {
    Self {
        client: reqwest::Client::new(),
        url,  // No validation!
```

**Recommendation:**
- Add URL allowlisting/blocklisting configuration
- Reject private IP ranges by default (10.x, 172.16-31.x, 192.168.x, 169.254.x)
- Validate URL scheme is http/https only
- Consider DNS rebinding protection

**Fix Checklist:**
- [x] Add `validate_url_for_ssrf()` function with private IP/cloud metadata blocking
- [x] Validate URLs in `HttpTransport::new()` and `SseTransport::connect()`
- [x] Add `new_unchecked()` variants for trusted/test configurations
- [x] Add 6 SSRF prevention tests

**Implementation Notes:**
- Added `validate_url_for_ssrf()` function in `src/transport/mod.rs`
- Blocks RFC 1918 private ranges (10.x, 172.16-31.x, 192.168.x)
- Blocks loopback (127.x.x.x), link-local (169.254.x.x), and cloud metadata endpoints
- Performs DNS resolution check for hostnames to prevent rebinding attacks
- `HttpTransport::new()` and `SseTransport::connect()` now return `Result<Self, TransportError>`
- Added `new_unchecked()` variants for tests and trusted configurations

---

### Issue #2: Command Injection Risk in StdioTransport
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 Critical
- **Location:** `src/transport/mod.rs:163-169`
- **Category:** Command Injection

**Description:**
The `StdioTransport::spawn()` passes user-controlled `command` and `args` directly to `Command::new()`. A malicious config could execute arbitrary commands.

**Vulnerable Code:**
```rust
// src/transport/mod.rs:163-169
let mut child = Command::new(command)
    .args(args)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::inherit())
    .spawn()?;
```

**Recommendation:**
- Validate commands against an allowlist
- Sanitize arguments (reject shell metacharacters)
- Consider using absolute paths only
- Add config option for command allowlist

**Fix Checklist:**
- [x] Add `validate_command_for_injection()` function
- [x] Add `validate_args_for_injection()` function
- [x] Block shell metacharacters (`;`, `|`, `&`, `$`, `` ` ``, `(`, `)`, `{`, `}`, `<`, `>`, `\n`, `\r`)
- [x] Block direct shell execution (sh, bash, zsh, cmd, powershell)
- [x] Add `spawn_unchecked()` for trusted configurations
- [x] Add 8 command injection tests

**Implementation Notes:**
- Added `validate_command_for_injection()` in `src/transport/mod.rs`
- Added `validate_args_for_injection()` for argument validation
- `StdioTransport::spawn()` now validates command and args before execution
- Added `spawn_unchecked()` variant for tests and trusted configurations
- Blocks direct shell invocation (sh, bash, zsh, fish, csh, ksh, dash, cmd, powershell, pwsh)
- Tests updated to use scripts directly (with shebang) instead of `/bin/sh script.sh`

---

### Issue #3: mTLS Header Spoofing Vulnerability
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 High
- **Location:** `src/auth/mtls.rs:138-176`, `src/server/mod.rs:492-523`
- **Category:** Authentication Bypass

**Description:**
The mTLS authentication relies on headers (`X-Client-Cert-CN`, `X-Client-Cert-Verified`) set by a reverse proxy. If traffic reaches the server without going through the proxy, an attacker can spoof these headers.

**Vulnerable Code:**
```rust
// src/auth/mtls.rs:138-144
pub fn from_headers(headers: &axum::http::HeaderMap) -> Option<Self> {
    let verified = headers
        .get(HEADER_CLIENT_CERT_VERIFIED)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("success") || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
```

**Recommendation:**
- Add option to verify requests come from trusted proxy IPs only
- Add a shared secret header that proxy and mcp-guard validate
- Document this risk prominently in README
- Consider adding `trust_proxy_headers` config with explicit opt-in

**Fix Checklist:**
- [x] Add `trusted_proxy_ips` config option
- [x] Validate source IP before trusting mTLS headers
- [x] Add `TrustedProxyValidator` with CIDR range support
- [x] Add `from_headers_if_trusted()` and `from_headers_unchecked()` methods
- [x] Add 8 tests for trusted proxy validation

**Implementation Notes:**
- Added `trusted_proxy_ips: Vec<String>` to `MtlsConfig` in `src/config/mod.rs`
- Created `TrustedProxyValidator` struct in `src/auth/mtls.rs` with CIDR support
- Renamed `from_headers()` to `from_headers_unchecked()` (explicit opt-in to unsafe behavior)
- Added `from_headers_if_trusted()` that validates client IP against trusted proxy list
- Auth middleware in `src/server/mod.rs` now uses `ConnectInfo<SocketAddr>` to get client IP
- Tests cover: single IP, CIDR ranges, IPv6, empty list rejection, trusted/untrusted IPs

---

### Issue #4: JWT Algorithm Confusion Potential
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 High
- **Location:** `src/auth/jwt.rs:269-280`
- **Category:** Cryptographic Weakness

**Description:**
In Simple mode, the algorithm is hardcoded to `HS256`, but the token's `header.alg` is not verified against this. While JWKS mode checks algorithm match, Simple mode doesn't reject tokens claiming different algorithms (e.g., `none` or `RS256`).

**Vulnerable Code:**
```rust
// src/auth/jwt.rs:269-274
JwtMode::Simple { .. } => {
    let key = self.simple_key.as_ref()
        .ok_or_else(|| AuthError::Internal("Simple key not initialized".into()))?;
    (key.clone(), Algorithm::HS256)
    // Note: header.alg not validated!
}
```

**Recommendation:**
- Add explicit algorithm validation in Simple mode
- Reject tokens where `header.alg != HS256`
- Log attempts to use mismatched algorithms

**Fix Checklist:**
- [x] Add algorithm check in Simple mode authenticate()
- [x] Return `InvalidJwt("Algorithm mismatch")` for non-HS256 tokens
- [x] Add test for algorithm confusion attack (RS256, ES256, none)
- [x] Add warning log for algorithm mismatch attempts

**Implementation Notes:**
- Unified algorithm validation for both Simple and JWKS modes in `src/auth/jwt.rs:282-295`
- Algorithm check now happens before token decoding (fail fast)
- Added tracing::warn() for algorithm mismatch attempts (security monitoring)
- Added 3 new tests: RS256 rejection, ES256 rejection, 'none' algorithm rejection
- Tests manually craft JWT tokens with mismatched headers to simulate attacks

---

### Issue #5: OAuth State Token Not Cryptographically Bound
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 High
- **Location:** `src/server/mod.rs:247-257`
- **Category:** Cryptographic Weakness

**Description:**
The `generate_random_string()` uses a limited charset providing less entropy than full base64. More critically, states are not tied to user sessions, allowing state fixation attacks.

**Vulnerable Code:**
```rust
// src/server/mod.rs:248-249
fn generate_random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
```

**Recommendation:**
- Use `rand::rngs::OsRng` with base64url encoding for more entropy
- Bind state to client IP or session identifier
- Consider using a cryptographic MAC (HMAC) for state integrity

**Fix Checklist:**
- [x] Replace charset-based generation with `OsRng` + base64url
- [x] Add client IP binding to `PkceState` struct
- [x] Validate client IP matches on callback
- [x] Add tests for state entropy and binding

**Implementation Notes:**
- Replaced `rand::thread_rng()` with `rand::rngs::OsRng` for cryptographic entropy
- Changed encoding from charset-based (~5.95 bits/char) to base64url (~6 bits/char)
- Added `client_ip: IpAddr` field to `PkceState` struct
- `oauth_authorize` now captures client IP via `ConnectInfo<SocketAddr>`
- `oauth_callback` validates that callback IP matches the IP that initiated the flow
- Mismatch triggers security warning log and returns "OAuth state binding mismatch" error
- Added `test_generate_random_string_entropy` test for entropy validation

---

### Issue #6: API Key Timing Attack
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 High
- **Location:** `src/auth/mod.rs:162-178`
- **Category:** Side-Channel Attack

**Description:**
API key validation uses HashMap lookup which may be susceptible to timing attacks. While SHA-256 hashing mitigates plaintext timing, the hash comparison itself could leak information about valid key prefixes.

**Vulnerable Code:**
```rust
// src/auth/mod.rs:162-165
async fn authenticate(&self, token: &str) -> Result<Identity, AuthError> {
    let hash = Self::hash_key(token);
    self.keys.get(&hash)  // HashMap lookup - not constant time
```

**Recommendation:**
- Use constant-time comparison (`subtle::ConstantTimeEq`) for hash lookups
- Consider iterating all keys with constant-time comparison
- Add the `subtle` crate as dependency

**Fix Checklist:**
- [x] Add `subtle` crate to dependencies
- [x] Implement constant-time key lookup
- [x] Add tests for constant-time comparison
- [x] Document timing attack mitigation

**Implementation Notes:**
- Added `subtle = "2.5"` crate for constant-time comparison primitives
- Changed `ApiKeyProvider` to use `Vec<ApiKeyConfig>` instead of `HashMap`
- Added `constant_time_compare()` using `subtle::ConstantTimeEq`
- Loop iterates through ALL keys without early exit to maintain constant time
- Length comparison done in constant time before byte comparison
- Added 7 new tests: 5 for constant-time comparison, 2 for provider authentication

---

### Issue #7: Unbounded OAuth State Store Growth
- **Status:** [x] Fixed (2025-12-15)
- **Severity:** 🔴 High
- **Location:** `src/server/mod.rs:279-282`
- **Category:** Denial of Service

**Description:**
The OAuth state cleanup only runs when `oauth_authorize` is called. An attacker could flood the `/oauth/authorize` endpoint without completing flows, growing the DashMap unbounded (memory exhaustion DoS).

**Vulnerable Code:**
```rust
// src/server/mod.rs:298-305
cleanup_expired_oauth_states(&state.oauth_state_store);
state.oauth_state_store.insert(
    oauth_state.clone(),
    PkceState {
        code_verifier,
        created_at: Instant::now(),
    },
);
```

**Recommendation:**
- Add periodic background cleanup task
- Implement rate limiting on `/oauth/authorize` endpoint
- Cap maximum number of pending states (e.g., 10,000)
- Add metrics for state store size

**Fix Checklist:**
- [x] Add `MAX_PENDING_OAUTH_STATES` constant (10,000)
- [x] Check limit before inserting new OAuth state
- [x] Return 429 (rate limited) when at capacity
- [x] Add security warning log when limit reached
- [x] Add 2 tests for limit enforcement

**Implementation Notes:**
- Added `MAX_PENDING_OAUTH_STATES = 10_000` constant in `src/server/mod.rs`
- `oauth_authorize` now runs cleanup first, then checks if at capacity
- Returns 429 Too Many Requests with `Retry-After: 60` header when at capacity
- Logs warning with current count when limit is reached for monitoring
- Added `pending_states` to the OAuth flow initiation log for observability
- Added tests for constant validation and capacity checking

---

## 🟡 Medium Severity Issues

### Issue #8: Missing Authorization on tools/call
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** `src/server/mod.rs:167-194`, `src/authz/mod.rs:55-67`
- **Category:** Authorization Bypass

**Description:**
While `authorize_request()` extracts tool names and checks permissions, the actual tool call authorization is never enforced in the MCP message handlers. The code only filters `tools/list` responses but doesn't block unauthorized `tools/call` requests.

**Recommendation:**
- Add `authorize_request()` call before forwarding `tools/call` messages
- Return 403 Forbidden for unauthorized tool calls
- Log authorization denials to audit log

**Fix Checklist:**
- [ ] Add authz check in `handle_mcp_message()`
- [ ] Add authz check in `handle_routed_mcp_message()`
- [ ] Return proper error response for denied calls
- [ ] Add audit logging for authorization denials
- [ ] Add integration tests for tool authorization

---

### Issue #9: Information Disclosure in Error Messages
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** Multiple files
- **Category:** Information Leakage

**Description:**
Several areas leak internal details to clients:
- `src/auth/jwt.rs:148`: JWKS URL in error
- `src/transport/mod.rs:365-368`: Full HTTP error body
- OAuth callback errors expose provider details

**Recommendation:**
- Sanitize all external-facing error messages
- Log detailed errors internally only
- Return generic error messages to clients with error IDs for correlation

**Fix Checklist:**
- [ ] Audit all `AuthError` messages for sensitive data
- [ ] Audit all `TransportError` messages
- [ ] Audit OAuth error responses
- [ ] Add error sanitization helper function
- [ ] Ensure error_id is always included for correlation

---

### Issue #10: Weak Default Rate Limits
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** `src/config/mod.rs:364-368`
- **Category:** Insecure Defaults

**Description:**
Defaults of 100 RPS with burst of 50 may be too permissive for many deployments, especially for a security gateway.

**Recommendation:**
- Document security implications of default values
- Consider lower defaults (10-25 RPS)
- Add warnings in logs when using defaults

**Fix Checklist:**
- [ ] Update default RPS to 25
- [ ] Update default burst to 10
- [ ] Add documentation about rate limit tuning
- [ ] Add startup warning when using defaults

---

### Issue #11: Missing Request Size Limits
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** `src/server/mod.rs` (router configuration)
- **Category:** Denial of Service

**Description:**
No maximum request body size is enforced. Large JSON payloads could cause memory exhaustion.

**Recommendation:**
- Add `tower_http::limit::RequestBodyLimitLayer`
- Make limit configurable (default: 1MB)
- Return 413 Payload Too Large for oversized requests

**Fix Checklist:**
- [ ] Add `max_request_size` config option
- [ ] Add `RequestBodyLimitLayer` to router
- [ ] Add test for oversized request rejection
- [ ] Document request size limits

---

### Issue #12: Audit Log Injection
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** `src/audit/mod.rs:283-314`
- **Category:** Log Injection

**Description:**
Audit entries serialize user-controlled data (identity_id, tool names) directly to JSON. While JSON escaping prevents direct injection, log aggregators parsing these logs could be vulnerable to injection attacks.

**Recommendation:**
- Sanitize/validate identity IDs and tool names
- Limit string lengths in audit entries
- Consider allowlisting characters in identifiers

**Fix Checklist:**
- [ ] Add `sanitize_for_audit()` helper function
- [ ] Limit identity_id length to 256 chars
- [ ] Limit tool name length to 128 chars
- [ ] Strip/escape control characters
- [ ] Add tests for log injection prevention

---

### Issue #13: Missing CORS Configuration
- **Status:** [ ] Not fixed
- **Severity:** 🟡 Medium
- **Location:** `src/server/mod.rs` (router configuration)
- **Category:** Cross-Origin Security

**Description:**
The server doesn't configure CORS. For browser-based MCP clients, this could be an issue. Conversely, overly permissive CORS could enable CSRF-like attacks.

**Recommendation:**
- Add explicit CORS configuration with restrictive defaults
- Make CORS origins configurable
- Default to same-origin only

**Fix Checklist:**
- [ ] Add `cors` config section
- [ ] Add `CorsLayer` to router with restrictive defaults
- [ ] Document CORS configuration options
- [ ] Add tests for CORS behavior

---

## 🟢 Low Severity Issues

### Issue #14: Unmaintained Dependency (rustls-pemfile)
- **Status:** [ ] Not fixed
- **Severity:** 🟢 Low
- **Location:** `Cargo.toml` (transitive via oauth2)
- **Category:** Supply Chain

**Description:**
`cargo audit` reports `rustls-pemfile 1.0.4` is unmaintained (RUSTSEC-2025-0134), pulled in via `oauth2 -> reqwest`.

**Recommendation:**
- Update `oauth2` crate when new version available
- Or pin `reqwest` to version using maintained dependencies

**Fix Checklist:**
- [ ] Check for oauth2 updates
- [ ] Update dependencies
- [ ] Re-run cargo audit

---

### Issue #15: Missing Security Headers for OAuth Callback
- **Status:** [ ] Not fixed
- **Severity:** 🟢 Low
- **Location:** `src/server/mod.rs:335-389`
- **Category:** Security Headers

**Description:**
OAuth callback returns JSON with access tokens but doesn't include cache-control headers to prevent token caching.

**Recommendation:**
- Add `Cache-Control: no-store` header
- Add `Pragma: no-cache` header

**Fix Checklist:**
- [ ] Add no-cache headers to OAuth callback response
- [ ] Add test for cache headers

---

### Issue #16: Test Secret in Code
- **Status:** [ ] Not fixed
- **Severity:** 🟢 Low
- **Location:** `src/auth/jwt.rs:384`
- **Category:** Hardcoded Secrets

**Description:**
Test secret is defined in test code. While in `#[cfg(test)]`, ensure it can't leak to production.

```rust
const TEST_SECRET: &str = "test-secret-key-at-least-32-characters-long";
```

**Recommendation:**
- Verify `#[cfg(test)]` properly excludes from release builds
- Consider generating random test secrets

**Fix Checklist:**
- [ ] Verify test code exclusion in release builds
- [ ] Consider using random secrets in tests

---

### Issue #17: No TLS Certificate Validation Options
- **Status:** [ ] Not fixed
- **Severity:** 🟢 Low
- **Location:** HTTP client configuration
- **Category:** TLS Security

**Description:**
HTTP clients for JWKS/introspection use default TLS but don't offer certificate pinning options for high-security deployments.

**Recommendation:**
- Add optional certificate pinning configuration for JWKS endpoints
- Document TLS security considerations

**Fix Checklist:**
- [ ] Add optional `jwks_ca_cert` config for pinning
- [ ] Document TLS configuration options

---

### Issue #18: Route Prefix Collision
- **Status:** [ ] Not fixed
- **Severity:** 🟢 Low
- **Location:** `src/router/mod.rs:299-303`
- **Category:** Logic Error

**Description:**
The prefix routing allows `/exactnot` to match `/exact` route. This could cause unintended routing.

**Recommendation:**
- Document this behavior clearly
- Consider requiring trailing slash for prefix routes
- Or use path segment matching instead of string prefix

**Fix Checklist:**
- [ ] Document prefix matching behavior
- [ ] Consider adding `exact_match` option for routes
- [ ] Add warning for potentially ambiguous routes

---

## Architectural Concerns

### A. No Graceful Shutdown for Transports
- **Status:** [ ] Not addressed
- **Location:** `src/transport/mod.rs`, `src/main.rs`

**Description:**
While `CancellationToken` is used for JWKS refresh, the `StdioTransport` child process isn't gracefully terminated on shutdown. Background tasks may be orphaned.

**Recommendation:**
- Pass shutdown token to transports
- Send SIGTERM to child processes on shutdown
- Wait for child process exit with timeout

---

### B. Single-Threaded Rate Limiter Cleanup
- **Status:** [ ] Not addressed
- **Location:** `src/rate_limit/mod.rs:128-129`

**Description:**
Rate limiter cleanup runs inline during requests when threshold is reached. Under high load, this could cause latency spikes.

**Recommendation:**
- Move cleanup to background task with periodic execution
- Use separate cleanup threshold from inline check

---

### C. Token Cache Revocation Delay
- **Status:** [ ] Not addressed
- **Location:** `src/auth/oauth.rs:59`

**Description:**
OAuth token cache has 5-minute TTL. Revoked tokens remain valid during this window.

**Recommendation:**
- Document this behavior in security considerations
- Consider adding cache invalidation endpoint
- Add config option for cache TTL

---

## Summary Table

| ID | Severity | Category | Status | Owner | Target Date |
|----|----------|----------|--------|-------|-------------|
| 1 | 🔴 Critical | SSRF | [x] Fixed | | 2025-12-15 |
| 2 | 🔴 Critical | Command Injection | [x] Fixed | | 2025-12-15 |
| 3 | 🔴 High | Auth Bypass | [ ] | | |
| 4 | 🔴 High | JWT Alg Confusion | [ ] | | |
| 5 | 🔴 High | Crypto Weakness | [ ] | | |
| 6 | 🔴 High | Timing Attack | [ ] | | |
| 7 | 🔴 High | DoS | [ ] | | |
| 8 | 🟡 Medium | Authz Bypass | [ ] | | |
| 9 | 🟡 Medium | Info Leak | [ ] | | |
| 10 | 🟡 Medium | Insecure Defaults | [ ] | | |
| 11 | 🟡 Medium | DoS | [ ] | | |
| 12 | 🟡 Medium | Log Injection | [ ] | | |
| 13 | 🟡 Medium | CORS | [ ] | | |
| 14 | 🟢 Low | Supply Chain | [ ] | | |
| 15 | 🟢 Low | Security Headers | [ ] | | |
| 16 | 🟢 Low | Hardcoded Secret | [ ] | | |
| 17 | 🟢 Low | TLS Security | [ ] | | |
| 18 | 🟢 Low | Logic Error | [ ] | | |

---

## Release Recommendations

### P0 - Before Public Release
Fix all Critical/High severity issues (#1-7)

### P1 - Before Production Use
Fix all Medium severity issues (#8-13)

### P2 - Ongoing Maintenance
Address Low severity issues and architectural concerns

---

## Additional Recommendations

1. **Create SECURITY.md** with:
   - Responsible disclosure policy
   - Security contact email
   - Known limitations
   - Security configuration best practices

2. **Add Security CI Checks:**
   - `cargo audit` in CI pipeline
   - `cargo clippy` with security lints
   - Consider `cargo-deny` for license/supply chain

3. **Consider Security Audit:**
   - Professional penetration testing before v1.0 GA
   - Focus on authentication bypass scenarios

---

## Changelog

| Date | Author | Changes |
|------|--------|---------|
| 2025-12-15 | Claude Code | Initial audit report |

---

*This document should be updated as issues are resolved. Mark checkboxes and update status as fixes are implemented.*
