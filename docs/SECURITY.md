# Security Model

## Overview

mcp-guard implements a defense-in-depth security model with multiple layers of protection.

## Authentication

### Provider Chain

Authentication providers are tried in order:

1. **mTLS** (if configured): Checks `X-Client-Cert-*` headers from reverse proxy
2. **Bearer Token**: Tries each configured provider until one succeeds

```
Request → mTLS? → Bearer → ApiKey? → JWT? → OAuth? → Identity
                           ↓         ↓       ↓
                         reject   reject   reject
```

### API Key Authentication

**Storage**: Only SHA256 hashes are stored in config files.

```rust
// Key generation
let key = generate_api_key();  // 32 random bytes, base64
let hash = sha256(key);        // Store this in config
```

**Validation**: Constant-time comparison to prevent timing attacks.

### JWT Authentication

**Simple Mode (HS256)**:
- Symmetric key stored in config
- Suitable for internal services
- Recommended: 32+ character secrets

**JWKS Mode (RS256/ES256)**:
- Public keys fetched from JWKS endpoint
- No local secrets required
- Automatic key rotation via background refresh
- Configurable cache duration (default: 1 hour)

**Validation**:
- Signature verification
- Issuer (`iss`) validation
- Audience (`aud`) validation
- Expiration (`exp`) check with optional leeway
- Not-before (`nbf`) check

### OAuth 2.1

**PKCE Support**: All flows use S256 code challenge.

**Token Validation**:
1. Token introspection (if endpoint configured)
2. UserInfo endpoint fallback
3. Cache validated tokens (LRU, max 500 entries)

## Authorization

### Per-Tool Permissions

Each identity has an `allowed_tools` list:

```rust
pub struct Identity {
    pub id: String,
    pub allowed_tools: Option<Vec<String>>,  // None = all tools
    pub rate_limit: Option<u32>,
}
```

**Wildcard Support**: `["*"]` grants access to all tools.

### Tools/List Filtering (FR-AUTHZ-03)

The `tools/list` response is filtered to only show authorized tools:

```rust
fn filter_tools_list_response(response: Message, identity: &Identity) -> Message {
    // Remove tools not in identity.allowed_tools
}
```

### Scope-to-Tool Mapping

JWT and OAuth tokens can map scopes to tools:

```toml
[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file"]
"admin" = ["*"]
```

## Rate Limiting

### Per-Identity Limits

Each authenticated identity gets its own rate limiter:

- Token bucket algorithm (via Governor crate)
- Default: 100 RPS, 50 burst
- Customizable per-identity
- TTL-based cleanup (1 hour idle expiry)

### Response Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 47
X-RateLimit-Reset: 1702656789
Retry-After: 1  (only on 429)
```

## Input Validation

### Configuration Validation

| Field | Rule |
|-------|------|
| `server.port` | 1-65535 |
| `jwt.jwks_url` | HTTPS required (production) |
| `oauth.redirect_uri` | Valid HTTP(S) URL |
| `rate_limit.requests_per_second` | > 0 |
| `rate_limit.burst_size` | > 0 |
| `tracing.sample_rate` | 0.0-1.0 |

### URL Validation

All external URLs (JWKS, introspection, etc.) are validated:
- Must be valid URL format
- HTTPS enforced in production builds

## Output Sanitization

### Error Responses

Errors include a unique `error_id` for correlation:

```json
{
  "error": "Rate limit exceeded",
  "error_id": "a1b2c3d4-..."
}
```

**Never Exposed**:
- Internal file paths
- Command lines
- Stack traces
- Database errors

### Transport Errors

Transport errors are sanitized:

| Internal Error | Client Message |
|---------------|----------------|
| `Timeout` | "Upstream request timed out" |
| `ConnectionClosed` | "Upstream connection closed" |
| `ProcessExited` | "Upstream process unavailable" |
| `Io(...)` | "Upstream communication error" |

## Network Security

### TLS Termination

TLS should be handled by a reverse proxy (nginx, Caddy):

```
Client → nginx (TLS) → mcp-guard (plaintext) → upstream
```

### mTLS

For client certificate authentication:

```nginx
# nginx config
ssl_client_certificate /path/to/ca.crt;
ssl_verify_client on;

proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
proxy_set_header X-Client-Cert-Verified $ssl_client_verify;
```

### Security Headers

All responses include:

```
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Content-Security-Policy: default-src 'none'
```

## Audit Logging

### Events Logged

- Authentication success/failure
- Authorization denied
- Rate limit exceeded
- Requests (method, status, duration)

### Log Format

Structured JSON with correlation IDs:

```json
{
  "timestamp": "2024-12-15T10:30:00Z",
  "event_type": "auth_success",
  "identity_id": "user123",
  "trace_id": "abc123..."
}
```

### SIEM Integration

Logs can be shipped to SIEM systems via HTTP:

- Batch size: configurable (default 100)
- Flush interval: configurable (default 30s)
- Retry: exponential backoff (3 attempts)
- Authentication: custom headers

## Threat Model

### Protected Against

| Threat | Mitigation |
|--------|-----------|
| Unauthorized access | Authentication required |
| Credential theft | Hash-only storage, JWKS mode |
| Rate abuse | Per-identity rate limiting |
| MITM attacks | HTTPS enforcement |
| Timing attacks | Constant-time comparison |
| Memory exhaustion | TTL-based cache eviction |
| Tool abuse | Per-tool authorization |

### Out of Scope

- Physical security
- Upstream MCP server vulnerabilities
- Client-side security
- Denial of service at network layer

## Recommendations

### Production Deployment

1. Use TLS via reverse proxy
2. Enable mTLS for service-to-service
3. Use JWKS mode for JWT (no local secrets)
4. Set appropriate rate limits
5. Enable audit logging with SIEM export
6. Use read-only root filesystem (containers)
7. Run as non-root user

### Secret Management

- Never commit secrets to version control
- Use environment variables or secrets manager
- Rotate API keys periodically
- Use short-lived JWT tokens
