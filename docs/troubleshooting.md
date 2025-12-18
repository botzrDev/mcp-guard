# Troubleshooting Guide

Diagnose and resolve common issues with MCP Guard.

## Diagnostic Tools

### mcp-guard validate

Check configuration syntax and validity:

```bash
mcp-guard validate
mcp-guard validate --config /path/to/config.toml
```

**Output (success):**

```
Configuration is valid: mcp-guard.toml
```

**Output (error):**

```
Configuration error: invalid port value '0': must be between 1 and 65535
```

### mcp-guard check-upstream

Test upstream MCP server connectivity:

```bash
mcp-guard check-upstream
mcp-guard check-upstream --timeout 30
```

**Output (success):**

```
Checking upstream connectivity...

Transport: stdio
Command:   npx

Server: Filesystem v1.0.0
✓ Upstream is reachable and responding
```

**Output (failure):**

```
✗ Upstream check failed: command not found
```

### mcp-guard version

Show version and feature information:

```bash
mcp-guard version
```

Include this output in bug reports.

### Verbose Logging

Enable detailed logging:

```bash
mcp-guard -v run
```

Or set environment variable:

```bash
RUST_LOG=debug mcp-guard run
```

### Health Endpoints

Check server status:

```bash
# Detailed health
curl http://localhost:3000/health

# Liveness (is it running?)
curl http://localhost:3000/live

# Readiness (is it accepting requests?)
curl http://localhost:3000/ready
```

---

## Startup Issues

### "Config file not found"

**Symptom:**

```
Error: Config file not found: mcp-guard.toml
```

**Solutions:**

1. Check file exists in current directory:
   ```bash
   ls -la mcp-guard.toml
   ```

2. Specify explicit path:
   ```bash
   mcp-guard --config /etc/mcp-guard/config.toml run
   ```

3. Generate new config:
   ```bash
   mcp-guard init
   ```

### "Invalid configuration"

**Symptom:**

```
Configuration error: missing field 'transport'
```

**Solutions:**

1. Run validation for detailed errors:
   ```bash
   mcp-guard validate
   ```

2. Check TOML syntax (missing quotes, brackets):
   ```toml
   # Wrong
   transport = stdio

   # Correct
   transport = "stdio"
   ```

3. Verify required fields are present

### "Address already in use"

**Symptom:**

```
Error: Address already in use (os error 98)
```

**Solutions:**

1. Find process using the port:
   ```bash
   lsof -i :3000
   # or
   ss -tlnp | grep 3000
   ```

2. Stop the existing process or use a different port:
   ```bash
   mcp-guard run --port 3001
   ```

### "Permission denied"

**Symptom:**

```
Error: Permission denied (os error 13)
```

**Solutions:**

1. Ports below 1024 require root (use port 3000+ instead)

2. Check file permissions:
   ```bash
   ls -la /etc/mcp-guard/config.toml
   ls -la /var/log/mcp-guard/
   ```

3. Run as correct user:
   ```bash
   sudo -u mcp-guard mcp-guard run
   ```

### "TLS certificate error"

**Symptom:**

```
Error: Failed to load TLS certificate: No such file or directory
```

**Solutions:**

1. Verify certificate paths exist:
   ```bash
   ls -la /etc/ssl/server.crt
   ls -la /etc/ssl/server.key
   ```

2. Check certificate format (must be PEM):
   ```bash
   openssl x509 -in /etc/ssl/server.crt -text -noout
   ```

3. Verify key matches certificate:
   ```bash
   openssl x509 -noout -modulus -in server.crt | md5sum
   openssl rsa -noout -modulus -in server.key | md5sum
   # Should match
   ```

---

## Authentication Errors

### HTTP 401 Unauthorized

**Symptom:**

```json
{"error": "Unauthorized", "error_id": "..."}
```

**Common causes and solutions:**

#### Missing Authorization Header

```bash
# Wrong - no header
curl -X POST http://localhost:3000/mcp ...

# Correct
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer mcp_YOUR_KEY" ...
```

#### Invalid API Key Format

```bash
# Wrong - missing prefix
curl -H "Authorization: Bearer AbCdEf123..." ...

# Correct - includes mcp_ prefix
curl -H "Authorization: Bearer mcp_AbCdEf123..." ...
```

#### Hash Mismatch

Verify key hash:

```bash
mcp-guard hash-key "mcp_YOUR_KEY"
# Compare output to key_hash in config
```

#### Expired JWT

Check token expiration:

```bash
# Decode JWT (without verification) at jwt.io
# or use jq:
echo "YOUR_JWT" | cut -d. -f2 | base64 -d | jq .exp
```

Increase clock skew tolerance:

```toml
[auth.jwt]
leeway_secs = 60
```

#### Invalid JWT Signature

1. Verify JWKS URL is correct and accessible:
   ```bash
   curl https://your-idp.com/.well-known/jwks.json
   ```

2. Check algorithm matches:
   ```toml
   [auth.jwt]
   algorithms = ["RS256"]  # Must match token's alg header
   ```

3. For simple mode, verify secret matches token signer

### JWT-Specific Issues

#### "Invalid issuer"

```toml
# Check iss claim in JWT
# Must match exactly (including trailing slash)
[auth.jwt]
issuer = "https://auth.example.com/"
```

#### "Invalid audience"

```toml
# Check aud claim in JWT
[auth.jwt]
audience = "mcp-guard"
```

#### "JWKS fetch failed"

1. Check network connectivity:
   ```bash
   curl -v https://your-idp.com/.well-known/jwks.json
   ```

2. Verify HTTPS certificate is valid

3. Check firewall allows outbound HTTPS

#### "Algorithm mismatch"

```toml
# JWKS mode requires RS256/ES256, not HS256
[auth.jwt]
mode = "jwks"
algorithms = ["RS256", "ES256"]
```

### OAuth-Specific Issues

#### "Redirect URI mismatch"

- URI must match **exactly** what's registered with provider
- Check protocol (http vs https)
- Check for trailing slashes

#### "Invalid client credentials"

1. Verify `client_id` and `client_secret`
2. Check for whitespace or newlines
3. Regenerate credentials if needed

#### "Token introspection failed"

1. Verify `introspection_url` is correct
2. Check client has introspection permission
3. Try `userinfo_url` as fallback

---

## Authorization Errors

### HTTP 403 Forbidden

**Symptom:**

```json
{"error": "Forbidden", "error_id": "..."}
```

#### "Tool not authorized"

1. Check identity's `allowed_tools`:
   ```toml
   [[auth.api_keys]]
   id = "my-key"
   allowed_tools = ["read_file"]  # Only read_file allowed
   ```

2. Check scope-to-tool mapping:
   ```toml
   [auth.jwt.scope_tool_mapping]
   "read" = ["read_file"]
   # User needs "read" scope to use read_file
   ```

3. Verify JWT/OAuth scopes include required permissions

#### Debug Authorization

Enable verbose logging to see authorization decisions:

```bash
mcp-guard -v run
```

Look for:

```
DEBUG identity=user123 tool=write_file allowed=false reason="tool not in allowed_tools"
```

---

## Rate Limiting Issues

### HTTP 429 Too Many Requests

**Symptom:**

```json
{"error": "Rate limit exceeded", "retry_after_secs": 1}
```

#### Check Current Limits

```bash
curl -v -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer mcp_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# Check response headers:
# x-ratelimit-limit: 100
# x-ratelimit-remaining: 0
```

#### Increase Limits

```toml
[rate_limit]
requests_per_second = 500  # Increase from default 100
burst_size = 100
```

#### Per-Identity Limits Not Applying

1. Verify identity has custom rate limit:
   ```toml
   [[auth.api_keys]]
   id = "high-volume"
   rate_limit = 1000  # Custom limit
   ```

2. Check identity is being extracted correctly (verbose logging)

#### Respect Retry-After

Clients should wait the specified time:

```python
retry_after = response.headers.get('Retry-After', 1)
time.sleep(int(retry_after))
```

---

## Transport Issues

### Stdio Transport

#### "Process failed to start"

1. Verify command exists:
   ```bash
   which npx
   which python
   ```

2. Check PATH includes the command

3. Test command manually:
   ```bash
   npx -y @modelcontextprotocol/server-filesystem /tmp
   ```

#### "Process exited unexpectedly"

1. Check stderr for error messages
2. Verify all arguments are correct
3. Check file permissions for data directories

#### "JSON parsing error"

1. Upstream may output non-JSON to stdout
2. Check for debug/log output going to stdout
3. Redirect stderr if needed

### HTTP Transport

#### "Connection refused"

1. Verify upstream is running:
   ```bash
   curl -v http://upstream:8080/health
   ```

2. Check URL is correct
3. Verify network connectivity / firewall rules

#### "Request timeout"

1. Upstream may be overloaded
2. Check upstream server logs
3. Consider increasing upstream resources

#### "SSRF validation failed"

1. For internal services, use IP address
2. Ensure DNS resolves to expected addresses

### SSE Transport

#### "Stream disconnected"

1. Check network stability
2. Verify upstream maintains connection
3. Check upstream timeout settings

#### "Event parsing error"

1. Verify each `data:` line is valid JSON
2. Check response has `Content-Type: text/event-stream`

---

## Multi-Server Routing Issues

### HTTP 404 "Server not found"

1. List available routes:
   ```bash
   curl http://localhost:3000/routes
   ```

2. Verify server name in request matches config

3. Check path prefix:
   ```bash
   # Correct
   curl -X POST http://localhost:3000/mcp/filesystem ...

   # Wrong (missing server name)
   curl -X POST http://localhost:3000/mcp ...
   ```

### Wrong Server Receiving Requests

Check path prefix ordering (longest prefix match):

```toml
# /api/v2 should be listed before /api
[[upstream.servers]]
name = "api-v2"
path_prefix = "/api/v2"

[[upstream.servers]]
name = "api"
path_prefix = "/api"
```

---

## Performance Issues

### High Latency

1. Check upstream latency:
   ```bash
   time curl http://upstream:8080/health
   ```

2. Check metrics:
   ```promql
   histogram_quantile(0.99, rate(mcp_guard_request_duration_seconds_bucket[5m]))
   ```

3. Look for slow auth (JWKS fetch, token introspection):
   ```bash
   mcp-guard -v run
   ```

### Memory Growth

1. Check active identity count:
   ```promql
   mcp_guard_active_identities
   ```

2. Rate limiters expire after 1 hour automatically

3. If identity count grows unbounded, check for:
   - New identity per request (identity leakage)
   - Unstable JWT `sub` claim

### Connection Exhaustion

1. Check open connections:
   ```bash
   ss -s
   ```

2. Verify HTTP keep-alive is working

3. Consider connection pool limits

---

## Observability Issues

### Metrics Not Appearing

1. Verify endpoint responds:
   ```bash
   curl http://localhost:3000/metrics
   ```

2. Check Prometheus scrape config

3. Verify network connectivity

### Traces Not Exporting

1. Verify OTLP endpoint:
   ```bash
   curl http://jaeger:4317/api/traces
   ```

2. Check `sample_rate` is not 0.0

3. Verify tracing is enabled:
   ```toml
   [tracing]
   enabled = true
   ```

### Audit Logs Missing

1. Verify audit enabled:
   ```toml
   [audit]
   enabled = true
   ```

2. Check file permissions for log file

3. For HTTP export, check endpoint and headers

4. Look for retry/failure messages in console

---

## Getting Help

### Collecting Diagnostics

Before reporting an issue, collect:

1. **Version information:**
   ```bash
   mcp-guard version
   ```

2. **Configuration (sanitized):**
   ```bash
   # Remove secrets before sharing
   mcp-guard validate 2>&1
   ```

3. **Logs:**
   ```bash
   mcp-guard -v run 2>&1 | head -100
   ```

4. **Specific error output:**
   - HTTP status code
   - Error response body
   - Error ID (from response)

### Debug Logging

For detailed debugging:

```bash
RUST_LOG=debug mcp-guard run 2>&1 | tee debug.log
```

This captures:

- Authentication decisions
- Authorization checks
- Rate limit operations
- Transport communications

### Filing Issues

Report bugs at: https://github.com/botzrdev/mcp-guard/issues

Include:

1. MCP Guard version (`mcp-guard version`)
2. OS and version
3. Steps to reproduce
4. Expected vs actual behavior
5. Relevant logs (sanitized)
6. Configuration (sanitized)

### Community Support

- GitHub Discussions: https://github.com/botzrdev/mcp-guard/discussions
- Documentation: https://github.com/botzrdev/mcp-guard/tree/main/docs

---

## Quick Reference

| Symptom | Likely Cause | Quick Fix |
|---------|--------------|-----------|
| 401 Unauthorized | Missing/invalid credentials | Check Authorization header |
| 403 Forbidden | Tool not allowed | Check allowed_tools config |
| 429 Too Many Requests | Rate limit hit | Wait for Retry-After |
| 404 Server not found | Wrong server name | Check /routes endpoint |
| 500 Internal Error | Upstream issue | Check upstream with check-upstream |
| Config error | Invalid TOML | Run validate |
| Can't start | Port in use | Change port or stop other process |

---

## See Also

- [Quick Start Guide](quickstart.md) - Initial setup
- [Configuration Reference](configuration.md) - All configuration options
- [Authentication Guide](authentication.md) - Auth troubleshooting
- [CLI Reference](cli.md) - Diagnostic commands
