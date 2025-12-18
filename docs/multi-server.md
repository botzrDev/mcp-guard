# Multi-Server Routing Guide

Configure MCP Guard to route requests to multiple upstream MCP servers based on path prefixes.

## Overview

Multi-server routing allows a single MCP Guard instance to proxy requests to multiple upstream MCP servers. Each server is identified by a unique name and path prefix.

### Benefits

- **Single gateway** - One entry point for multiple MCP servers
- **Unified authentication** - Single auth config for all servers
- **Centralized rate limiting** - Consistent rate limits across servers
- **Consolidated audit logs** - All activity in one place
- **Mixed transports** - Combine stdio, HTTP, and SSE servers

### Architecture

```
                          ┌─────────────────┐
                          │   MCP Guard     │
                          │                 │
Client ─── /fs/* ────────>│  Path Router    │────> Filesystem Server (stdio)
                          │                 │
Client ─── /github/* ────>│                 │────> GitHub Server (http)
                          │                 │
Client ─── /api/* ───────>│                 │────> API Server (sse)
                          └─────────────────┘
```

---

## Configuration

### Basic Structure

```toml
[upstream]
transport = "stdio"  # Required but ignored when servers configured
command = "echo"

[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[upstream.servers]]
name = "github"
path_prefix = "/github"
transport = "http"
url = "https://github-mcp.example.com/api"
```

### Server Entry Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Unique server identifier |
| `path_prefix` | string | Yes | Path prefix to match (must start with `/`) |
| `transport` | string | Yes | `"stdio"`, `"http"`, or `"sse"` |
| `command` | string | For stdio | Command to execute |
| `args` | array | No | Command arguments |
| `url` | string | For http/sse | Upstream URL |
| `strip_prefix` | boolean | No | Remove prefix when forwarding |

### Validation Rules

- Each `name` must be unique
- Each `path_prefix` must start with `/`
- At least one server must be configured
- Transport-specific fields required (command for stdio, url for http/sse)

---

## Routing Behavior

### Path Matching

MCP Guard uses **longest prefix match** to select the target server.

**Example configuration:**

```toml
[[upstream.servers]]
name = "api"
path_prefix = "/api"

[[upstream.servers]]
name = "api-v2"
path_prefix = "/api/v2"

[[upstream.servers]]
name = "default"
path_prefix = "/"
```

**Request routing:**

| Request Path | Matched Server | Matched Prefix |
|--------------|----------------|----------------|
| `/api/users` | api | `/api` |
| `/api/v2/users` | api-v2 | `/api/v2` |
| `/health` | default | `/` |

### Request Path Transformation

By default, the full path is forwarded to the upstream server. Use `strip_prefix = true` to remove the matched prefix.

**Without strip_prefix (default):**

```toml
[[upstream.servers]]
name = "api"
path_prefix = "/api"
url = "http://backend:8080"
```

Request to `/api/users` → Forwarded as `/api/users`

**With strip_prefix:**

```toml
[[upstream.servers]]
name = "api"
path_prefix = "/api"
url = "http://backend:8080"
strip_prefix = true
```

Request to `/api/users` → Forwarded as `/users`

---

## API Endpoints

### POST /mcp/:server_name

Route a request to a specific server by name.

**Request:**

```bash
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer mcp_YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Response (200 OK):**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {"name": "read_file", "description": "Read a file..."}
    ]
  }
}
```

**Error (404 Not Found):**

```json
{
  "error": "Server not found: unknown-server"
}
```

### GET /routes

List all available server routes.

**Request:**

```bash
curl http://localhost:3000/routes
```

**Response:**

```json
{
  "routes": [
    {
      "name": "filesystem",
      "path_prefix": "/fs",
      "transport": "stdio"
    },
    {
      "name": "github",
      "path_prefix": "/github",
      "transport": "http"
    }
  ]
}
```

---

## Examples

### Two Local Servers (Stdio)

Run two npx-based MCP servers with different capabilities:

```toml
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "echo"

[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/home/user/documents"]

[[upstream.servers]]
name = "memory"
path_prefix = "/memory"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-memory"]

[[auth.api_keys]]
id = "developer"
key_hash = "..."
```

**Usage:**

```bash
# Access filesystem server
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer mcp_KEY" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# Access memory server
curl -X POST http://localhost:3000/mcp/memory \
  -H "Authorization: Bearer mcp_KEY" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

### Mixed Transports

Combine local and remote MCP servers:

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "stdio"
command = "echo"

# Local filesystem server
[[upstream.servers]]
name = "local"
path_prefix = "/local"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# Remote API server (HTTP)
[[upstream.servers]]
name = "api"
path_prefix = "/api"
transport = "http"
url = "https://mcp-api.example.com/v1"

# Streaming server (SSE)
[[upstream.servers]]
name = "stream"
path_prefix = "/stream"
transport = "sse"
url = "https://streaming-mcp.example.com/events"

[[auth.api_keys]]
id = "service"
key_hash = "..."
```

### Microservices Architecture

Multiple HTTP upstreams for different domains:

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "http"
url = "http://default:8080"

[[upstream.servers]]
name = "users"
path_prefix = "/users"
transport = "http"
url = "http://user-service.internal:8080/mcp"

[[upstream.servers]]
name = "orders"
path_prefix = "/orders"
transport = "http"
url = "http://order-service.internal:8080/mcp"

[[upstream.servers]]
name = "inventory"
path_prefix = "/inventory"
transport = "http"
url = "http://inventory-service.internal:8080/mcp"

[auth.jwt]
mode = "jwks"
jwks_url = "https://auth.internal/.well-known/jwks.json"
issuer = "https://auth.internal/"
audience = "mcp-guard"
```

### Tool Segregation by Server

Different servers expose different tools:

```toml
[upstream]
transport = "stdio"
command = "echo"

# Read-only server
[[upstream.servers]]
name = "readonly"
path_prefix = "/readonly"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/data", "--readonly"]

# Full access server
[[upstream.servers]]
name = "admin"
path_prefix = "/admin"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/data"]

# API key with read-only access
[[auth.api_keys]]
id = "reader"
key_hash = "..."
allowed_tools = ["read_file", "list_directory"]

# API key with full access
[[auth.api_keys]]
id = "admin"
key_hash = "..."
# No allowed_tools = all tools allowed
```

---

## Authorization with Multi-Server

### Global Tool Restrictions

The `allowed_tools` setting on identities applies globally across all servers:

```toml
[[auth.api_keys]]
id = "limited-user"
key_hash = "..."
allowed_tools = ["read_file"]  # Can only use read_file on ANY server
```

### Scope-Based Authorization

JWT/OAuth scope mappings also apply globally:

```toml
[auth.jwt.scope_tool_mapping]
"read" = ["read_file", "list_directory"]
"write" = ["write_file"]
```

A user with `read` scope can use `read_file` on any server.

### Server-Specific Access Control

For server-specific authorization, implement at the application level:

1. Use different API keys for different servers
2. Configure tool restrictions per key based on server needs
3. Use separate MCP Guard instances for strict isolation

---

## Monitoring Multi-Server Deployments

### Metrics

Prometheus metrics include server context where applicable:

```
mcp_guard_requests_total{method="POST", status="200"} 1234
```

### Audit Logs

Audit events include the target server:

```json
{
  "event_type": "ToolCall",
  "identity_id": "user123",
  "server": "filesystem",
  "tool": "read_file",
  "success": true
}
```

### Health Checks

Use `check-upstream` to verify all servers:

```bash
mcp-guard check-upstream
```

For multi-server configs, this checks all configured servers.

---

## Troubleshooting

### "Server not found" (404)

1. Verify server name matches configuration:
   ```bash
   curl http://localhost:3000/routes
   ```

2. Check spelling in request path:
   ```bash
   # Correct
   curl -X POST http://localhost:3000/mcp/filesystem ...

   # Wrong (missing server name)
   curl -X POST http://localhost:3000/mcp ...
   ```

### Route Conflicts

If requests go to the wrong server:

1. Check path prefix ordering (longer prefixes should work correctly)
2. Verify prefix doesn't have trailing slash issues
3. Use `/routes` endpoint to see active configuration

### Per-Server Connectivity Issues

1. Check individual server with `check-upstream`:
   ```bash
   mcp-guard check-upstream
   ```

2. Verify transport-specific settings:
   - Stdio: command exists and is executable
   - HTTP: URL is reachable
   - SSE: server supports Server-Sent Events

### Different Responses from Different Servers

This is expected behavior - each server has its own tools and responses. Use `/routes` to understand which server handles which paths.

---

## See Also

- [Transport Guide](transports.md) - Details on stdio, HTTP, and SSE transports
- [Configuration Reference](configuration.md) - Complete configuration options
- [Authentication Guide](authentication.md) - Authentication across servers
- [Observability Guide](observability.md) - Monitoring multi-server deployments
