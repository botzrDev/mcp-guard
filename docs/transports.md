# Transport Guide

Deep dive into MCP Guard transport types for connecting to upstream MCP servers.

## Overview

Transports define how MCP Guard communicates with upstream MCP servers. Each transport handles the specifics of process management, network protocols, and message serialization.

### Available Transports

| Transport | Use Case | Communication |
|-----------|----------|---------------|
| **Stdio** | Local processes (npx, python) | stdin/stdout |
| **HTTP** | Remote servers, microservices | POST JSON-RPC |
| **SSE** | Streaming responses | Server-Sent Events |

### Choosing a Transport

| Scenario | Recommended Transport |
|----------|----------------------|
| npx or local MCP servers | Stdio |
| Cloud-hosted MCP servers | HTTP |
| Streaming/real-time responses | SSE |
| Multiple remote servers | HTTP with multi-server routing |

---

## Stdio Transport

### Overview

The stdio transport spawns a local process and communicates via stdin/stdout using newline-delimited JSON-RPC messages.

**Best for:**

- npx-based MCP servers (`@modelcontextprotocol/server-*`)
- Python MCP servers
- Local development
- Single-host deployments

### How It Works

```
MCP Guard ─────────────────────────> MCP Server Process
           stdin (JSON-RPC requests)

MCP Guard <───────────────────────── MCP Server Process
           stdout (JSON-RPC responses)
```

1. MCP Guard spawns the child process
2. Writer task sends JSON-RPC messages to stdin
3. Reader task receives responses from stdout
4. Process lifecycle is managed (health checks, restart on crash)

### Configuration

```toml
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transport` | string | Yes | Must be `"stdio"` |
| `command` | string | Yes | Executable path or command |
| `args` | array | No | Command-line arguments |

### Examples

**Filesystem Server (npx):**

```toml
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/home/user/documents"]
```

**Python MCP Server:**

```toml
[upstream]
transport = "stdio"
command = "python"
args = ["-m", "my_mcp_server", "--config", "/etc/mcp/server.json"]
```

**Custom Binary:**

```toml
[upstream]
transport = "stdio"
command = "/usr/local/bin/my-mcp-server"
args = ["--port", "internal"]
```

**Node.js Server:**

```toml
[upstream]
transport = "stdio"
command = "node"
args = ["/opt/mcp-server/index.js"]
```

### Health Monitoring

MCP Guard monitors stdio transport health:

- **Process exit detection** - Logs warning if process terminates
- **is_healthy() check** - Verifies process is still running
- **Reader/writer task supervision** - Monitors background tasks

Check upstream health:

```bash
mcp-guard check-upstream --timeout 10
```

### Troubleshooting

**"Process failed to start":**

1. Verify command exists: `which npx` or `which python`
2. Check PATH includes the command
3. Try running the command manually:
   ```bash
   npx -y @modelcontextprotocol/server-filesystem /tmp
   ```

**"JSON parsing error":**

1. The upstream server may be outputting non-JSON to stdout
2. Redirect stderr in your command if needed
3. Enable verbose logging: `mcp-guard -v run`

**"Process exited unexpectedly":**

1. Check process logs (stderr may have error messages)
2. Verify all required arguments are provided
3. Check file permissions for data directories

**"Timeout waiting for response":**

1. Increase timeout with `check-upstream --timeout 30`
2. Verify the server is responding to JSON-RPC
3. Check for deadlocks or infinite loops in the server

---

## HTTP Transport

### Overview

The HTTP transport sends JSON-RPC requests as POST requests to an HTTP endpoint and receives JSON-RPC responses in the response body.

**Best for:**

- Cloud-hosted MCP servers
- Microservice architectures
- Load-balanced deployments
- Remote server integration

### How It Works

```
MCP Guard ──── POST /mcp ────> Upstream HTTP Server
               Content-Type: application/json
               {"jsonrpc": "2.0", ...}

MCP Guard <─── 200 OK ──────── Upstream HTTP Server
               Content-Type: application/json
               {"jsonrpc": "2.0", ...}
```

1. MCP Guard receives authenticated request
2. Forwards as HTTP POST to upstream URL
3. Waits for HTTP response
4. Returns response to client

### Configuration

```toml
[upstream]
transport = "http"
url = "http://mcp-server.internal:8080/mcp"
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transport` | string | Yes | Must be `"http"` |
| `url` | string | Yes | Upstream HTTP endpoint |

### Examples

**Remote MCP Server:**

```toml
[upstream]
transport = "http"
url = "https://mcp.example.com/api/v1/mcp"
```

**Internal Service:**

```toml
[upstream]
transport = "http"
url = "http://mcp-server.internal:8080/jsonrpc"
```

**Kubernetes Service:**

```toml
[upstream]
transport = "http"
url = "http://mcp-server.default.svc.cluster.local:8080/mcp"
```

### Connection Behavior

- **Connection pooling** - Reuses connections via keep-alive
- **30-second timeout** - Requests fail after 30 seconds
- **Automatic retries** - No automatic retries (implement at client level)

### SSRF Protection

HTTP transport includes Server-Side Request Forgery (SSRF) protection:

- Private IP ranges blocked (10.x, 172.16-31.x, 192.168.x) unless explicitly configured
- Cloud metadata endpoints blocked (169.254.169.254)
- DNS rebinding protection

**Note:** For internal services, use explicit IP addresses or ensure DNS resolution is trusted.

### Troubleshooting

**"Connection refused":**

1. Verify upstream server is running
2. Check URL is correct: `curl -v http://upstream:8080/mcp`
3. Check network connectivity and firewall rules

**"Request timeout":**

1. Upstream may be overloaded or slow
2. Check upstream server logs
3. Consider increasing resources on upstream

**"SSRF validation failed":**

1. For internal services, use IP address instead of hostname
2. Ensure DNS resolves to expected addresses
3. Check SSRF protection isn't blocking legitimate internal traffic

**"Invalid JSON response":**

1. Upstream may not be an MCP server
2. Check endpoint path is correct
3. Verify upstream returns JSON-RPC responses

---

## SSE Transport

### Overview

The SSE (Server-Sent Events) transport is designed for streaming responses from upstream MCP servers. It sends requests via POST and receives responses as an event stream.

**Best for:**

- Long-running tool operations
- Streaming LLM responses
- Progress updates
- Real-time notifications

### How It Works

```
MCP Guard ──── POST /mcp ────> Upstream SSE Server
               Content-Type: application/json

MCP Guard <─── 200 OK ──────── Upstream SSE Server
               Content-Type: text/event-stream

               data: {"jsonrpc": "2.0", ...}

               data: {"jsonrpc": "2.0", ...}
```

1. MCP Guard sends POST request with JSON-RPC
2. Upstream responds with `Content-Type: text/event-stream`
3. Multiple JSON-RPC messages can be streamed
4. Stream closes when complete

### Configuration

```toml
[upstream]
transport = "sse"
url = "http://mcp-server.internal:8080/mcp/stream"
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transport` | string | Yes | Must be `"sse"` |
| `url` | string | Yes | Upstream SSE endpoint |

### Examples

**Streaming MCP Server:**

```toml
[upstream]
transport = "sse"
url = "https://streaming-mcp.example.com/api/stream"
```

**Local Streaming Server:**

```toml
[upstream]
transport = "sse"
url = "http://localhost:8081/mcp/events"
```

### Event Format

SSE events are expected in standard format:

```
data: {"jsonrpc": "2.0", "id": 1, "result": {"progress": 0.5}}

data: {"jsonrpc": "2.0", "id": 1, "result": {"progress": 1.0, "data": "..."}}
```

Each `data:` line contains a complete JSON-RPC message.

### Use Cases

**Progress Updates:**

```
data: {"jsonrpc": "2.0", "id": 1, "result": {"status": "processing", "progress": 25}}

data: {"jsonrpc": "2.0", "id": 1, "result": {"status": "processing", "progress": 50}}

data: {"jsonrpc": "2.0", "id": 1, "result": {"status": "complete", "data": "..."}}
```

**Streaming Text:**

```
data: {"jsonrpc": "2.0", "id": 1, "result": {"chunk": "Hello"}}

data: {"jsonrpc": "2.0", "id": 1, "result": {"chunk": " World"}}

data: {"jsonrpc": "2.0", "id": 1, "result": {"done": true}}
```

### Troubleshooting

**"Stream disconnected":**

1. Check network stability
2. Verify upstream server maintains connection
3. Look for upstream timeout settings

**"Event parsing error":**

1. Verify each `data:` line is valid JSON
2. Check for missing newlines between events
3. Ensure response has `Content-Type: text/event-stream`

**"No events received":**

1. Upstream may not support SSE
2. Check upstream logs for errors
3. Try HTTP transport as fallback

---

## Transport Comparison

| Feature | Stdio | HTTP | SSE |
|---------|-------|------|-----|
| **Location** | Local only | Remote | Remote |
| **Connection** | Process pipes | HTTP POST | HTTP + SSE |
| **Streaming** | No | No | Yes |
| **Scalability** | Single instance | Load balanced | Load balanced |
| **Latency** | Lowest | Low | Low (streaming) |
| **Complexity** | Simple | Simple | Moderate |
| **Health checks** | Process status | HTTP status | Connection status |

### Performance Characteristics

| Transport | Typical Latency | Throughput | Memory |
|-----------|-----------------|------------|--------|
| Stdio | <1ms | High (local) | Low |
| HTTP | 1-10ms | High | Low |
| SSE | Variable | Moderate | Higher (buffering) |

### Decision Matrix

```
Is the MCP server on the same host?
├── Yes → Use Stdio
└── No
    └── Does it need streaming responses?
        ├── Yes → Use SSE
        └── No → Use HTTP
```

---

## Multi-Transport Routing

MCP Guard supports running multiple transports simultaneously using multi-server routing. Different upstream servers can use different transports.

```toml
[upstream]
transport = "stdio"  # Default (unused when servers configured)
command = "echo"

[[upstream.servers]]
name = "local-fs"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[upstream.servers]]
name = "remote-api"
path_prefix = "/api"
transport = "http"
url = "https://mcp-api.example.com/v1"

[[upstream.servers]]
name = "streaming"
path_prefix = "/stream"
transport = "sse"
url = "https://streaming-mcp.example.com/events"
```

See the [Multi-Server Routing Guide](multi-server.md) for details.

---

## See Also

- [Multi-Server Routing Guide](multi-server.md) - Path-based routing to multiple upstreams
- [Configuration Reference](configuration.md) - Complete configuration options
- [Deployment Guide](deployment.md) - Production deployment patterns
- [Troubleshooting Guide](troubleshooting.md) - Common issues and solutions
