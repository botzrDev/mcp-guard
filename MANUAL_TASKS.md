# Manual Tasks Before Production Launch

> **Generated:** 2025-12-23
> **Status:** Review Required

This document lists tasks that require manual intervention before marketing launch.

---

## Completed (Code Changes)

All the following security and production readiness issues have been resolved in this commit:

| Issue | Description | Status |
|-------|-------------|--------|
| Dockerfile | Multi-stage production Dockerfile created | Done |
| docker-compose.yml | Local development compose file added | Done |
| Issue #8 | Authorization check on tools/call | Done |
| Issue #10 | Lowered default rate limits (25 RPS, burst 10) | Done |
| Issue #11 | Request body size limits (1MB default) | Done |
| Issue #12 | Audit log sanitization (injection prevention) | Done |
| Issue #13 | CORS configuration support | Done |
| Issue #15 | Cache-Control headers on OAuth callback | Done |
| Upstream metrics | Added `mcp_guard_upstream_latency_seconds` histogram | Done |

---

## Manual Tasks Required

### P0 - Critical (Before Marketing)

#### 1. Test Docker Build
```bash
# Build the image
docker build -t mcp-guard:latest .

# Test the image runs
docker run --rm mcp-guard:latest --help

# Verify health check works
docker run -d --name mcp-guard-test -p 3000:3000 mcp-guard:latest run --config /path/to/config.toml
curl http://localhost:3000/health
docker stop mcp-guard-test
```

#### 2. Run Full Test Suite
```bash
cargo test --all-features
```

#### 3. Build Release Binary
```bash
cargo build --release
./target/release/mcp-guard --version
```

#### 4. Test with Real MCP Server
- [ ] Test with Claude Desktop
- [ ] Test with a filesystem MCP server
- [ ] Test with a git MCP server
- [ ] Verify tool authorization works (restricted user can't call unauthorized tools)

#### 5. OAuth Flow Testing
- [ ] Test GitHub OAuth flow end-to-end
- [ ] Test Google OAuth flow end-to-end (if applicable)
- [ ] Verify state binding works (IP mismatch is rejected)

---

### P1 - Important (Before Production Deployment)

#### 6. Documentation Updates
The following documentation should be updated:
- [ ] Update README with new CORS configuration options
- [ ] Update README with new `max_request_size` option
- [ ] Document the Dockerfile and container deployment
- [ ] Update rate-limiting.md with new defaults (25 RPS, burst 10)

#### 7. Security Review
- [ ] Review SECURITY_AUDIT.md - Issue #9 (Information Disclosure) is still open
- [ ] Review remaining low-priority issues (#14, #16, #17, #18)
- [ ] Run `cargo audit` to check for new advisories
- [ ] Run `cargo deny check` for license/security

#### 8. Performance Testing
```bash
# Run benchmarks and compare to targets
cargo bench

# Target metrics:
# - Latency overhead: <2ms p99
# - Throughput: >5,000 RPS
# - Memory: <50MB RSS
```

#### 9. CI/CD Verification
- [ ] Ensure GitHub Actions workflows still pass
- [ ] Verify release workflow produces correct artifacts
- [ ] Test the published binary on each platform (Linux, macOS, Windows)

---

### P2 - Nice to Have (Post-Launch)

#### 10. Package Distribution
- [ ] Create Homebrew formula
- [ ] Create AUR package
- [ ] Publish to crates.io

#### 11. Additional Testing
- [ ] Add integration tests for tool authorization
- [ ] Add tests for oversized request rejection
- [ ] Add tests for CORS behavior
- [ ] Add tests for audit log sanitization

#### 12. Remaining Security Issues
From SECURITY_AUDIT.md still open:
- [ ] Issue #9: Sanitize error messages (Information Disclosure)
- [ ] Issue #14: Update oauth2 crate for rustls-pemfile advisory
- [ ] Issue #16: Review test secrets in code
- [ ] Issue #17: Add TLS certificate pinning option
- [ ] Issue #18: Document route prefix collision behavior

---

## Configuration Updates Required

### Update your mcp-guard.toml

Add the new configuration options:

```toml
[server]
# Maximum request body size (default: 1MB)
max_request_size = 1048576

# CORS configuration (disabled by default)
[server.cors]
enabled = false
allowed_origins = []  # Use ["*"] for permissive, or ["https://example.com"]
allowed_methods = ["GET", "POST", "OPTIONS"]
allowed_headers = ["Authorization", "Content-Type"]
max_age = 3600

[rate_limit]
# New defaults: 25 RPS, burst of 10
# Adjust if these are too restrictive for your use case
requests_per_second = 25
burst_size = 10
```

---

## Rollback Plan

If issues are discovered after launch:

1. **Docker**: Roll back to previous image tag
2. **Binary**: Keep previous version in release assets
3. **Config**: New options have safe defaults, no breaking changes

---

## Sign-Off Checklist

| Task | Reviewer | Date | Status |
|------|----------|------|--------|
| Code review | | | [ ] |
| Docker build tested | | | [ ] |
| Full test suite passed | | | [ ] |
| OAuth flow tested | | | [ ] |
| Documentation reviewed | | | [ ] |
| Security audit reviewed | | | [ ] |
| Performance benchmarks | | | [ ] |

---

## Contact

For questions about this audit or the fixes implemented, review:
- `SECURITY_AUDIT.md` - Detailed security issue tracking
- `CHANGELOG.md` - Version history
- `docs/` - Comprehensive documentation
