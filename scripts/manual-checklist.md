# Manual Testing Checklist for mcp-guard v1.0

**Tester:** ________________
**Date:** ________________
**Environment:** ________________
**Tier:** Free / Pro / Enterprise

---

## Pre-Release Testing (30-45 minutes)

### 1. Installation & Setup

- [ ] `cargo install mcp-guard` works (or download binary from releases)
- [ ] `mcp-guard version` shows correct version (1.0.0)
- [ ] `mcp-guard version` shows correct tier (Free/Pro/Enterprise)
- [ ] `mcp-guard init` creates valid config file (TOML format)
- [ ] `mcp-guard init -f yaml` creates valid YAML config
- [ ] `mcp-guard validate` passes on generated config
- [ ] `mcp-guard keygen --user-id test` generates valid API key
- [ ] `mcp-guard keygen` shows helpful error when user-id missing
- [ ] `mcp-guard hash-key` hashes API keys correctly
- [ ] `mcp-guard check-upstream` validates stdio connectivity
- [ ] `mcp-guard help` shows all commands

**Notes:**
```





```

---

### 2. Server Startup

- [ ] `mcp-guard run` starts without errors
- [ ] Server binds to configured port (default 3000)
- [ ] Server prints startup message with version
- [ ] Server shows "Server ready" message
- [ ] Graceful shutdown with Ctrl+C works
- [ ] Shutdown message shows "Shutting down gracefully"
- [ ] Config validation errors show helpful messages

**Notes:**
```





```

---

### 3. Authentication

#### API Key Authentication
- [ ] Valid API key → 200 OK
- [ ] Missing Authorization header → 401 Unauthorized
- [ ] Wrong API key → 401 Unauthorized
- [ ] Malformed Authorization header → 401
- [ ] Case-insensitive "Bearer" token → works

#### JWT Authentication (if configured)
- [ ] Generate JWT token with correct claims
- [ ] Valid JWT token → 200 OK
- [ ] Expired JWT token → 401
- [ ] Invalid signature → 401
- [ ] Wrong issuer → 401
- [ ] Wrong audience → 401
- [ ] Missing required claims → 401

#### OAuth 2.1 (if configured - Pro tier)
- [ ] Visit `/oauth/authorize` → redirects to provider
- [ ] Authorize with provider → redirects to callback
- [ ] Callback receives code and exchanges for token
- [ ] Use access token for API requests → works
- [ ] Invalid OAuth token → 401

#### mTLS (if configured - Enterprise tier)
- [ ] Client with valid certificate → authenticated
- [ ] Client without certificate → rejected
- [ ] Client with invalid/expired certificate → rejected

**Notes:**
```





```

---

### 4. Authorization

- [ ] User with `allowed_tools = ["read_file"]` can call `read_file`
- [ ] User with restricted tools cannot call forbidden tools
- [ ] User with `allowed_tools = ["*"]` can call any tool
- [ ] User with no `allowed_tools` (default) can call any tool
- [ ] Tools/list response only shows authorized tools
- [ ] Scope-to-tool mapping works (JWT/OAuth)

**Notes:**
```





```

---

### 5. Rate Limiting

- [ ] Rapid requests hit limit and return 429
- [ ] 429 includes Retry-After header with seconds
- [ ] 429 includes X-RateLimit-Limit header
- [ ] 429 includes X-RateLimit-Remaining header (0)
- [ ] 429 includes X-RateLimit-Reset header
- [ ] Different users have isolated limits
- [ ] Per-identity rate limits work (custom limits per user)
- [ ] Rate limits reset after time window
- [ ] Burst capacity allows temporary spikes

**Test script:**
```bash
# Send 100 rapid requests
for i in {1..100}; do
  curl -s -o /dev/null -w "%{http_code}\n" \
    -H "Authorization: Bearer YOUR_KEY" \
    http://localhost:3000/mcp
done | sort | uniq -c
```

**Notes:**
```





```

---

### 6. Health & Observability

#### Health Endpoints
- [ ] `/health` returns 200 with version and uptime
- [ ] `/health` response includes `{"status": "healthy", "version": "1.0.0", "uptime_secs": N}`
- [ ] `/live` returns 200 (Kubernetes liveness probe)
- [ ] `/ready` returns 200 after startup
- [ ] `/ready` returns 503 if not ready

#### Metrics
- [ ] `/metrics` returns Prometheus format (text/plain)
- [ ] Contains `mcp_guard_requests_total` counter
- [ ] Contains `mcp_guard_request_duration_seconds` histogram
- [ ] Contains `mcp_guard_auth_total` counter
- [ ] Contains `mcp_guard_rate_limit_total` counter
- [ ] Contains `mcp_guard_active_identities` gauge
- [ ] Metrics update after requests

#### Audit Logging
- [ ] Audit logs written to configured file
- [ ] Audit logs in JSON Lines format
- [ ] Each log has timestamp, user_id, method, tool_name
- [ ] Sensitive data is redacted (if configured)
- [ ] Audit export to HTTP endpoint works (Enterprise)

**Notes:**
```





```

---

### 7. Transport Integration

#### Stdio Transport
- [ ] Connect to local MCP server (e.g., filesystem server)
- [ ] Send `tools/list` request → receives tool list
- [ ] Send `tools/call` request → receives response
- [ ] Server crashes are handled gracefully
- [ ] Invalid JSON from upstream → error response

#### HTTP Transport (Pro tier)
- [ ] Connect to HTTP MCP server
- [ ] Send requests via HTTP POST
- [ ] Receive JSON responses
- [ ] Network timeouts handled gracefully

#### SSE Transport (Pro tier)
- [ ] Connect to SSE MCP server
- [ ] Receive streaming responses
- [ ] Handle connection interruptions

**Notes:**
```





```

---

### 8. Multi-Server Routing (Enterprise)

- [ ] Configure multiple servers with different path prefixes
- [ ] GET `/routes` lists all available servers
- [ ] Request to `/mcp/server1` routes to server1
- [ ] Request to `/mcp/server2` routes to server2
- [ ] Different transports per server work
- [ ] Unknown server path returns 404

**Notes:**
```





```

---

### 9. Integration Testing

#### With Claude Desktop
- [ ] Configure Claude Desktop to use mcp-guard
- [ ] Claude can list tools
- [ ] Claude can call tools
- [ ] Authentication works
- [ ] Rate limiting doesn't block normal usage

#### With Popular MCP Servers
- [ ] @modelcontextprotocol/server-filesystem
- [ ] @modelcontextprotocol/server-git (if available)
- [ ] Other community MCP servers

**Test Commands:**
```bash
# Start mcp-guard with filesystem server
mcp-guard run &

# Test with curl
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Notes:**
```





```

---

### 10. Error Handling & Edge Cases

- [ ] Invalid config file → helpful error message
- [ ] Missing config file → helpful error message
- [ ] Port already in use → helpful error message
- [ ] Upstream server unavailable → graceful error
- [ ] Invalid JSON request → error response
- [ ] Very large requests → handled or rejected gracefully
- [ ] Concurrent requests → no race conditions
- [ ] Memory usage stays stable under load

**Notes:**
```





```

---

### 11. Performance

- [ ] Latency overhead acceptable (<2ms p99 target)
- [ ] Throughput adequate (>5k RPS target)
- [ ] Memory usage reasonable (<50MB RSS target)
- [ ] Binary size acceptable (<15MB target)

**Test Commands:**
```bash
# Check binary size
ls -lh target/release/mcp-guard

# Simple load test (requires apache bench or similar)
ab -n 10000 -c 100 -H "Authorization: Bearer YOUR_KEY" \
  http://localhost:3000/health
```

**Measured Results:**
- Binary size: _______ MB
- p99 latency: _______ ms
- Throughput: _______ RPS
- Memory usage: _______ MB

---

### 12. Documentation & User Experience

- [ ] README.md is clear and accurate
- [ ] Quickstart guide works end-to-end
- [ ] Error messages are helpful (no cryptic errors)
- [ ] Config examples in templates/ work
- [ ] CLI help text is clear
- [ ] All referenced docs exist and are accurate

**Notes:**
```





```

---

## Platform Testing

Test on multiple platforms:

- [ ] Linux x86_64 (Ubuntu/Debian)
- [ ] Linux x86_64 (RHEL/CentOS)
- [ ] macOS x86_64 (Intel)
- [ ] macOS ARM64 (Apple Silicon)
- [ ] Windows x86_64
- [ ] Docker container

**Notes per platform:**
```





```

---

## Security Testing

- [ ] No secrets in logs
- [ ] No secrets in error messages
- [ ] API keys are hashed in config
- [ ] Audit logs redact sensitive data (if configured)
- [ ] HTTPS enforcement for JWKS URLs (in production)
- [ ] Security headers present in responses
- [ ] No obvious injection vulnerabilities

**Notes:**
```





```

---

## Final Checks

- [ ] All automated tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Security audit passes (`cargo audit`)
- [ ] License check passes (`cargo deny check`)
- [ ] Documentation builds (`cargo doc`)
- [ ] Release notes in CHANGELOG.md are accurate
- [ ] Version in Cargo.toml is correct (1.0.0)

---

## Issues Found

| # | Severity | Description | Status |
|---|----------|-------------|--------|
| 1 |          |             |        |
| 2 |          |             |        |
| 3 |          |             |        |

**Severity:** Critical / High / Medium / Low

---

## Sign-off

- [ ] I have completed all applicable tests
- [ ] All critical/high severity issues have been resolved
- [ ] I recommend this build for release

**Tester Signature:** ________________
**Date:** ________________

---

## Notes

Use this space for any additional observations, suggestions, or concerns:

```











```
