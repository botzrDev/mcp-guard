# HTTP API Reference

This document describes all HTTP endpoints exposed by mcp-guard.

## Base URL

```
http://{host}:{port}
```

Default: `http://127.0.0.1:3000`

## Authentication

Protected endpoints require a Bearer token in the Authorization header:

```http
Authorization: Bearer <token>
```

Supported token types:
- **API Key**: Raw API key string
- **JWT**: Signed JWT token (HS256 or RS256/ES256 via JWKS)
- **OAuth**: Access token from OAuth 2.1 flow

## Common Response Headers

All responses include:

| Header | Description |
|--------|-------------|
| `X-Content-Type-Options` | `nosniff` |
| `X-Frame-Options` | `DENY` |
| `Content-Security-Policy` | `default-src 'none'; frame-ancestors 'none'` |
| `X-XSS-Protection` | `1; mode=block` |
| `X-Trace-ID` | Trace ID for request correlation (if tracing enabled) |

Rate-limited endpoints also include:

| Header | Description |
|--------|-------------|
| `x-ratelimit-limit` | Maximum requests per second |
| `x-ratelimit-remaining` | Remaining requests in current window |
| `x-ratelimit-reset` | Unix timestamp when limit resets |

---

## Health Endpoints

### GET /health

Comprehensive health check with version and uptime.

**Authentication**: None required

**Response**: `200 OK`

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_secs": 3600
}
```

### GET /live

Kubernetes liveness probe. Returns 200 if the process is running.

**Authentication**: None required

**Response**: `200 OK`

```json
{
  "status": "alive"
}
```

### GET /ready

Kubernetes readiness probe. Returns 200 if ready to serve traffic, 503 if not ready.

**Authentication**: None required

**Response**: `200 OK` (ready)

```json
{
  "ready": true,
  "version": "1.0.0"
}
```

**Response**: `503 Service Unavailable` (not ready)

```json
{
  "ready": false,
  "version": "1.0.0"
}
```

---

## Metrics Endpoint

### GET /metrics

Prometheus-formatted metrics.

**Authentication**: None required

**Response**: `200 OK`
**Content-Type**: `text/plain; charset=utf-8`

```prometheus
# Minimal example metrics response
# See docs/observability.md for full list of metrics and labels
mcp_guard_requests_total{method="POST",status="200"} 1523
mcp_guard_active_identities 25
```

For a detailed breakdown of all available metrics, labels, and usage examples, refer to the **[Observability Guide](../observability.md#metrics)**.

---

## OAuth Endpoints

### GET /oauth/authorize

Initiates OAuth 2.1 authorization code flow with PKCE.

**Authentication**: None required

**Query Parameters**:

| Parameter | Required | Description |
|-----------|----------|-------------|
| `redirect_uri` | Yes | Client's callback URL |
| `state` | No | Opaque state value (passed back to client) |

**Response**: `302 Found`

Redirects to the OAuth provider's authorization endpoint with PKCE challenge.

**Example**:

```bash
curl -v "http://localhost:3000/oauth/authorize?redirect_uri=http://localhost:8080/callback&state=abc123"
```

### GET /oauth/callback

Handles OAuth authorization code callback.

**Authentication**: None required

**Query Parameters**:

| Parameter | Required | Description |
|-----------|----------|-------------|
| `code` | Yes | Authorization code from provider |
| `state` | Yes | State value from initial request |

**Response**: `302 Found`

Redirects back to client's `redirect_uri` with access token:

```
{redirect_uri}?access_token={token}&token_type=Bearer&expires_in=3600&state={state}
```

**Error Response**: `400 Bad Request`

```json
{
  "error": "invalid_grant",
  "error_description": "Authorization code expired or invalid"
}
```

---

## Routes Endpoint

### GET /routes

Lists configured server routes (multi-server mode only).

**Authentication**: None required

**Response**: `200 OK`

```json
{
  "count": 2,
  "routes": ["github", "filesystem"]
}
```

**Response**: `404 Not Found` (single-server mode)

```json
{
  "error": "Multi-server routing not enabled"
}
```

---

## MCP Endpoints

### POST /mcp

Forward an MCP JSON-RPC request to the upstream server (single-server mode).

**Authentication**: Required (Bearer token)

**Request Body**: JSON-RPC 2.0 request

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "read_file",
    "arguments": {
      "path": "/tmp/test.txt"
    }
  }
}
```

**Response**: `200 OK`

JSON-RPC 2.0 response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "File contents here..."
      }
    ]
  }
}
```

**Filtered Response** (tools/list):

When the authenticated identity has `allowed_tools` restrictions, the `tools/list` response is filtered to only show authorized tools:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "read_file",
        "description": "Read a file from the filesystem",
        "inputSchema": { /* ... */ }
      }
    ]
  }
}
```

### POST /mcp/:server_name

Forward an MCP request to a specific upstream server (multi-server mode).

**Authentication**: Required (Bearer token)

**Path Parameters**:

| Parameter | Description |
|-----------|-------------|
| `server_name` | Name of the target server route |

**Example**:

```bash
curl -X POST http://localhost:3000/mcp/github \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

---

## Error Responses

### 400 Bad Request

Invalid request format or parameters.

```json
{
  "error": "Invalid JSON-RPC request",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 401 Unauthorized

Missing or invalid authentication credentials.

```json
{
  "error": "Authentication failed: InvalidApiKey",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Common causes**:
- Missing `Authorization` header
- Invalid or expired token
- Malformed Bearer token

### 403 Forbidden

Authenticated but not authorized for the requested action.

```json
{
  "error": "Identity 'user-123' is not authorized to call tool 'delete_file'",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Common causes**:
- Tool not in `allowed_tools` list
- Action not permitted for identity

### 404 Not Found

Resource or route not found.

```json
{
  "error": "No route found for path: /unknown",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 429 Too Many Requests

Rate limit exceeded.

**Headers**:
| Header | Value |
|--------|-------|
| `Retry-After` | Seconds until retry allowed |
| `x-ratelimit-limit` | Configured limit |
| `x-ratelimit-remaining` | `0` |
| `x-ratelimit-reset` | Unix timestamp |

```json
{
  "error": "Rate limit exceeded",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 500 Internal Server Error

Unexpected server error.

```json
{
  "error": "Internal server error",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Note**: Detailed error information is logged server-side with the `error_id` for correlation.

### 502 Bad Gateway

Upstream server error or connection failure.

```json
{
  "error": "Upstream server error",
  "error_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Note**: Internal details (paths, URLs) are sanitized for security.

---

## Request/Response Examples

### Complete Authentication Flow

```bash
# 1. Get API key from config (pre-shared)
API_KEY="your-api-key-here"

# 2. Make authenticated request
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/list"
  }'
```

### OAuth Flow

```bash
# 1. Start authorization (opens browser)
open "http://localhost:3000/oauth/authorize?redirect_uri=http://localhost:8080/callback"

# 2. After callback, use the access token
ACCESS_TOKEN="token-from-callback"

curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"ping"}'
```

### Multi-Server Routing

```bash
# List available routes
curl http://localhost:3000/routes

# Call GitHub MCP server
curl -X POST http://localhost:3000/mcp/github \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'

# Call Filesystem MCP server
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"resources/list"}'
```

---

## Rate Limiting

Rate limits are applied per-identity:

| Configuration | Default |
|---------------|---------|
| `requests_per_second` | 100 |
| `burst_size` | 50 |

Identities can have custom limits via config:

```toml
[[auth.api_keys]]
id = "vip-user"
key_hash = "..."
rate_limit = 500  # Custom: 500 RPS
```

### Rate Limit Headers

Successful responses include:

```http
x-ratelimit-limit: 100
x-ratelimit-remaining: 42
x-ratelimit-reset: 1702900000
```

Rate-limited responses (429) include:

```http
Retry-After: 5
x-ratelimit-limit: 100
x-ratelimit-remaining: 0
x-ratelimit-reset: 1702900005
```

---

## Trace Context

When OpenTelemetry tracing is enabled, mcp-guard supports W3C trace context propagation.

### Incoming Context

Include trace context in requests:

```http
traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01
tracestate: congo=t61rcWkgMzE
```

### Outgoing Context

Responses include the trace ID:

```http
X-Trace-ID: 0af7651916cd43dd8448eb211c80319c
```

Use this ID to correlate requests across services and in audit logs.

---

## Content Types

### Request Content Types

| Endpoint | Content-Type |
|----------|--------------|
| `/mcp` | `application/json` |
| `/mcp/:server_name` | `application/json` |

### Response Content Types

| Endpoint | Content-Type |
|----------|--------------|
| `/health`, `/live`, `/ready` | `application/json` |
| `/metrics` | `text/plain; charset=utf-8` |
| `/routes` | `application/json` |
| `/mcp`, `/mcp/:server_name` | `application/json` |
| `/oauth/*` | `application/json` or redirect |

---

## WebSocket Support

Currently not supported. MCP communication uses:
- HTTP POST for request-response
- SSE for server-sent events

WebSocket transport is planned for future releases.
