# mcp-guard Alpha Testing Guide

## Overview

This document provides structured testing scenarios to validate mcp-guard v1.0.0 in real-world development conditions before public release.

**Test Environment Requirements:**
- Linux/macOS/WSL2
- Rust 1.75+
- Node.js 18+ (for MCP test servers)
- curl or httpie
- Optional: Docker (for JWKS/OAuth provider testing)

---

## Test Matrix

| Scenario | Priority | Status | Notes |
|----------|----------|--------|-------|
| 1. Build & Install | P0 | ⬜ | |
| 2. CLI Commands | P0 | ⬜ | |
| 3. API Key Auth | P0 | ⬜ | |
| 4. JWT Simple Mode | P0 | ⬜ | |
| 5. Rate Limiting | P0 | ⬜ | |
| 6. Audit Logging | P0 | ⬜ | |
| 7. Health Endpoints | P0 | ⬜ | |
| 8. Prometheus Metrics | P1 | ⬜ | |
| 9. Tool Authorization | P1 | ⬜ | |
| 10. HTTP Transport | P1 | ⬜ | |
| 11. Multi-Server Routing | P2 | ⬜ | |
| 12. Error Handling | P1 | ⬜ | |
| 13. Performance | P1 | ⬜ | |
| 14. Graceful Shutdown | P1 | ⬜ | |

---

## Scenario 1: Build & Install

### 1.1 Clean Build
```bash
# Clean and rebuild from scratch
cargo clean
cargo build --release

# Expected: Build succeeds without errors
# Record: Build time, binary size
ls -lh target/release/mcp-guard
```

**Expected Results:**
- [ ] Build completes without errors
- [ ] Binary size < 15MB
- [ ] No warnings (except allowed dead_code)

**Actual Results:**
```
Build time: ___
Binary size: ___
Errors/Warnings: ___
```

### 1.2 Test Suite
```bash
cargo test
```

**Expected Results:**
- [ ] All tests pass (80+ tests)
- [ ] No test timeouts

**Actual Results:**
```
Tests passed: ___/___
Failed tests: ___
```

---

## Scenario 2: CLI Commands

### 2.1 Help & Version
```bash
./target/release/mcp-guard --help
./target/release/mcp-guard version
```

**Expected Results:**
- [ ] Help shows all commands (init, validate, keygen, run, version, check-upstream)
- [ ] Version shows build info

### 2.2 Config Generation
```bash
rm -f test-config.toml
./target/release/mcp-guard init --config test-config.toml
cat test-config.toml
```

**Expected Results:**
- [ ] Config file created
- [ ] Contains valid TOML with all sections

### 2.3 Key Generation
```bash
./target/release/mcp-guard keygen --user-id test-user --rate-limit 50
```

**Expected Results:**
- [ ] Outputs API key (only shown once)
- [ ] Outputs key hash for config
- [ ] Shows rate limit setting

**Actual Results:**
```
API Key: ___
Key Hash: ___
```

### 2.4 Config Validation
```bash
./target/release/mcp-guard validate --config test-config.toml
```

**Expected Results:**
- [ ] Validation passes or shows specific errors

---

## Scenario 3: API Key Authentication

### 3.1 Setup Test Server

Create `test-api-key.toml`:
```toml
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-everything"]

[[auth.api_keys]]
id = "test-client"
key_hash = "PASTE_HASH_FROM_KEYGEN"
allowed_tools = ["*"]
rate_limit = 100

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true
```

### 3.2 Start Server
```bash
./target/release/mcp-guard run --config test-api-key.toml
```

### 3.3 Test Authentication

**Valid API Key:**
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected:** 200 OK with tools list

**Invalid API Key:**
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer wrong-key" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected:** 401 Unauthorized

**Missing Authorization:**
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected:** 401 Unauthorized

**Actual Results:**
```
Valid key response: ___
Invalid key response: ___
Missing auth response: ___
```

---

## Scenario 4: JWT Simple Mode

### 4.1 Setup JWT Config

Create `test-jwt.toml`:
```toml
[server]
host = "127.0.0.1"
port = 3001

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-everything"]

[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-for-testing-purposes-only"
algorithms = ["HS256"]
issuer = "test-issuer"
audience = "mcp-guard"

[auth.jwt.scope_tool_mapping]
"read:tools" = ["echo", "add"]
"admin" = ["*"]

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true
```

### 4.2 Generate Test JWT

Use jwt.io or a script to create a token:
```json
{
  "sub": "test-user",
  "iss": "test-issuer",
  "aud": "mcp-guard",
  "scope": "read:tools",
  "exp": 1999999999
}
```

Sign with HS256 using the secret from config.

### 4.3 Test JWT Auth
```bash
./target/release/mcp-guard run --config test-jwt.toml &

curl -X POST http://localhost:3001/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected Results:**
- [ ] Valid JWT returns 200 OK
- [ ] Expired JWT returns 401
- [ ] Wrong issuer returns 401
- [ ] Wrong audience returns 401

---

## Scenario 5: Rate Limiting

### 5.1 Burst Test
```bash
# Send 60 requests rapidly (above 50 burst limit)
for i in {1..60}; do
  curl -s -o /dev/null -w "%{http_code}\n" \
    -X POST http://localhost:3000/mcp \
    -H "Authorization: Bearer YOUR_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
done | sort | uniq -c
```

**Expected Results:**
- [ ] First ~50 requests return 200
- [ ] Remaining requests return 429
- [ ] 429 responses include `Retry-After` header

### 5.2 Check Rate Limit Headers
```bash
curl -i -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected Headers:**
- [ ] `X-RateLimit-Limit: 100`
- [ ] `X-RateLimit-Remaining: <number>`
- [ ] `X-RateLimit-Reset: <timestamp>`

---

## Scenario 6: Audit Logging

### 6.1 File Logging

Create config with file logging:
```toml
[audit]
enabled = true
file = "/tmp/mcp-guard-audit.log"
stdout = false
```

Run server and make requests, then check:
```bash
cat /tmp/mcp-guard-audit.log | jq .
```

**Expected Results:**
- [ ] Log entries are valid JSON
- [ ] Each entry has timestamp, event_type, identity_id
- [ ] Auth success/failure events logged
- [ ] Tool calls logged

### 6.2 Log Rotation

Configure rotation:
```toml
[audit]
enabled = true
file = "/tmp/mcp-guard-audit.log"

[audit.rotation]
enabled = true
max_size_bytes = 10000  # 10KB for testing
max_backups = 3
compress = true
```

Generate enough logs to trigger rotation:
```bash
for i in {1..500}; do
  curl -s -X POST http://localhost:3000/mcp \
    -H "Authorization: Bearer YOUR_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
done

ls -la /tmp/mcp-guard-audit*
```

**Expected Results:**
- [ ] Multiple log files created
- [ ] Older logs compressed (.gz)
- [ ] Only max_backups kept

---

## Scenario 7: Health Endpoints

### 7.1 Health Check
```bash
curl http://localhost:3000/health | jq .
```

**Expected:**
```json
{"status": "healthy", "version": "1.0.0", "uptime_secs": <number>}
```

### 7.2 Liveness Probe
```bash
curl http://localhost:3000/live | jq .
```

**Expected:**
```json
{"status": "alive"}
```

### 7.3 Readiness Probe
```bash
curl -w "\n%{http_code}" http://localhost:3000/ready | jq .
```

**Expected:**
- [ ] Returns 200 when ready
- [ ] Returns 503 when transport not initialized
- [ ] Response includes version

---

## Scenario 8: Prometheus Metrics

```bash
curl http://localhost:3000/metrics
```

**Expected Metrics:**
- [ ] `mcp_guard_requests_total{method,status}`
- [ ] `mcp_guard_request_duration_seconds{method}`
- [ ] `mcp_guard_auth_total{provider,result}`
- [ ] `mcp_guard_rate_limit_total{allowed}`
- [ ] `mcp_guard_active_identities`

**Verify Counters Increment:**
```bash
# Before requests
curl -s http://localhost:3000/metrics | grep mcp_guard_requests_total

# Make some requests
curl -s -X POST http://localhost:3000/mcp ...

# After requests
curl -s http://localhost:3000/metrics | grep mcp_guard_requests_total
```

---

## Scenario 9: Tool Authorization

### 9.1 Allowed Tools Only

Config with restricted tools:
```toml
[[auth.api_keys]]
id = "restricted-client"
key_hash = "..."
allowed_tools = ["echo"]  # Only echo allowed
```

Test calling allowed tool:
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer RESTRICTED_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"echo","arguments":{}}}'
```

**Expected:** 200 OK

Test calling forbidden tool:
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer RESTRICTED_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"add","arguments":{}}}'
```

**Expected:** 403 Forbidden

### 9.2 Tools List Filtering

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer RESTRICTED_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected:** Only shows tools the user is authorized to use (not all tools)

---

## Scenario 10: HTTP Transport

### 10.1 Connect to HTTP MCP Server

If you have an HTTP-based MCP server running:
```toml
[upstream]
transport = "http"
url = "http://localhost:8080/mcp"
```

**Test:**
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected Results:**
- [ ] Requests proxied to HTTP upstream
- [ ] Responses returned correctly

---

## Scenario 11: Multi-Server Routing

### 11.1 Setup Multi-Server Config
```toml
[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[upstream.servers]]
name = "everything"
path_prefix = "/everything"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-everything"]
```

### 11.2 Test Routing
```bash
# List routes
curl http://localhost:3000/routes | jq .

# Access specific server
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer YOUR_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'

curl -X POST http://localhost:3000/mcp/everything \
  -H "Authorization: Bearer YOUR_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected Results:**
- [ ] /routes lists all configured servers
- [ ] Each server accessible at /mcp/:name
- [ ] Different tools available per server

---

## Scenario 12: Error Handling

### 12.1 Invalid JSON
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d 'not json'
```

**Expected:** 400 Bad Request with error_id

### 12.2 Missing Content-Type
```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_KEY" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

**Expected:** Appropriate error or works anyway

### 12.3 Upstream Failure

Stop the upstream server and try a request:
**Expected:** 502 Bad Gateway or appropriate error

### 12.4 Security Headers

```bash
curl -I http://localhost:3000/health
```

**Expected Headers:**
- [ ] `X-Content-Type-Options: nosniff`
- [ ] `X-Frame-Options: DENY`
- [ ] `Content-Security-Policy: default-src 'none'`

---

## Scenario 13: Performance

### 13.1 Latency Test
```bash
# Measure request latency
for i in {1..100}; do
  curl -s -w "%{time_total}\n" -o /dev/null \
    -X POST http://localhost:3000/mcp \
    -H "Authorization: Bearer YOUR_KEY" \
    -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
done | awk '{sum+=$1; count++} END {print "Avg:", sum/count*1000, "ms"}'
```

**Expected:** < 2ms p99 overhead (excluding upstream time)

### 13.2 Memory Usage
```bash
# Start server and check memory
./target/release/mcp-guard run --config test-api-key.toml &
PID=$!
sleep 5

# Check RSS
ps -o rss= -p $PID | awk '{print $1/1024 " MB"}'

# After load test
for i in {1..1000}; do
  curl -s -X POST http://localhost:3000/mcp \
    -H "Authorization: Bearer YOUR_KEY" \
    -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
done

ps -o rss= -p $PID | awk '{print $1/1024 " MB"}'
```

**Expected:** < 50MB RSS

### 13.3 Run Benchmarks
```bash
cargo bench
```

**Record Results:**
```
api_key_auth: ___
jwt_validation: ___
rate_limit_check: ___
```

---

## Scenario 14: Graceful Shutdown

### 14.1 SIGTERM Handling
```bash
./target/release/mcp-guard run --config test-api-key.toml &
PID=$!
sleep 2

# Send SIGTERM
kill $PID

# Check exit code
wait $PID
echo "Exit code: $?"
```

**Expected Results:**
- [ ] Clean shutdown message in logs
- [ ] Exit code 0
- [ ] No zombie processes

### 14.2 SIGINT Handling (Ctrl+C)
Run interactively and press Ctrl+C.

**Expected:** Clean shutdown

---

## Test Summary

After completing all tests, fill in this summary:

| Category | Pass | Fail | Notes |
|----------|------|------|-------|
| Build & Install | | | |
| CLI Commands | | | |
| API Key Auth | | | |
| JWT Auth | | | |
| Rate Limiting | | | |
| Audit Logging | | | |
| Health Endpoints | | | |
| Metrics | | | |
| Authorization | | | |
| HTTP Transport | | | |
| Multi-Server | | | |
| Error Handling | | | |
| Performance | | | |
| Graceful Shutdown | | | |

**Overall Status:** ⬜ Ready for Release / ⬜ Needs Fixes

**Blocking Issues:**
1.
2.
3.

**Non-Blocking Issues:**
1.
2.
3.

**Tester:** _______________
**Date:** _______________
**Environment:** _______________
