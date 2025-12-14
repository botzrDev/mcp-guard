<div align="center">
  <h1>mcp-guard</h1>
  <h3>Secure your MCP servers in 5 minutes.</h3>
  <p>No Docker. No Kubernetes. No DevOps team.</p>

  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#observability">Observability</a> •
  <a href="#cli-reference">CLI Reference</a>
</div>

---

## The Problem

Model Context Protocol (MCP) is powerful, but most servers are deployed with **zero authentication**. If your agent can access it, so can anyone else.

## The Solution

`mcp-guard` is a lightweight security gateway that wraps any MCP server with production-grade protection:

- **Authentication**: API Keys, JWT (HS256 + JWKS), OAuth 2.1 (PKCE)
- **Authorization**: Per-tool permissions with scope mapping
- **Rate Limiting**: Per-identity limits with configurable burst
- **Observability**: Prometheus metrics + OpenTelemetry tracing
- **Audit Logging**: Track every request with correlation IDs

## Features

| Feature | Status |
|---------|--------|
| API Key Authentication | ✅ |
| JWT Authentication (HS256) | ✅ |
| JWT Authentication (JWKS/RS256/ES256) | ✅ |
| OAuth 2.1 with PKCE | ✅ |
| Per-Identity Rate Limiting | ✅ |
| Per-Tool Authorization | ✅ |
| Prometheus Metrics | ✅ |
| OpenTelemetry Tracing | ✅ |
| W3C Trace Context Propagation | ✅ |
| Audit Logging | ✅ |

## Quick Start

### Build from Source

```bash
git clone https://github.com/botzrdev/mcp-guard
cd mcp-guard
cargo build --release
```

### Generate Configuration

```bash
# Create a config file
./target/release/mcp-guard init

# Generate an API key
./target/release/mcp-guard keygen --user-id my-agent
```

### Run the Gateway

```bash
./target/release/mcp-guard run --config mcp-guard.toml
```

## Configuration

### Basic Example

```toml
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[auth.api_keys]]
id = "my-agent"
key_hash = "YOUR_KEY_HASH_HERE"
allowed_tools = ["read_file", "list_directory"]
rate_limit = 100

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true
```

### JWT Authentication (JWKS)

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://your-idp.com/.well-known/jwks.json"
algorithms = ["RS256", "ES256"]
issuer = "https://your-idp.com/"
audience = "mcp-guard"

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]
```

### OAuth 2.1 (GitHub Example)

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "http://localhost:3000/oauth/callback"
scopes = ["read:user", "repo"]
```

### OpenTelemetry Tracing

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"  # Jaeger/Tempo gRPC
sample_rate = 1.0
propagate_context = true
```

## Observability

### Prometheus Metrics

Metrics are exposed at `GET /metrics`:

| Metric | Type | Description |
|--------|------|-------------|
| `mcp_guard_requests_total` | Counter | Total requests (labels: method, status) |
| `mcp_guard_request_duration_seconds` | Histogram | Request latency (labels: method) |
| `mcp_guard_auth_total` | Counter | Auth attempts (labels: provider, result) |
| `mcp_guard_rate_limit_total` | Counter | Rate limit checks (labels: allowed) |
| `mcp_guard_active_identities` | Gauge | Tracked identity count |

### OpenTelemetry Tracing

When enabled, mcp-guard exports traces via OTLP to collectors like Jaeger or Grafana Tempo:

- **W3C Trace Context**: Extracts `traceparent`/`tracestate` from incoming requests
- **Span propagation**: Trace context is propagated to downstream services
- **Correlation IDs**: Trace IDs are included in logs for debugging

## Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/metrics` | GET | Prometheus metrics |
| `/mcp` | POST | MCP message handler (requires auth) |
| `/oauth/authorize` | GET | Start OAuth flow |
| `/oauth/callback` | GET | OAuth callback |

## CLI Reference

```bash
mcp-guard init [--format toml|yaml] [--force]  # Generate config file
mcp-guard validate                              # Validate config
mcp-guard keygen --user-id X [--rate-limit N]  # Generate API key
mcp-guard hash-key <key>                        # Hash an existing key
mcp-guard run [--host H] [--port P]            # Start the gateway
```

### Global Options

```bash
-c, --config <FILE>    Config file path (default: mcp-guard.toml)
-v, --verbose          Enable verbose logging
-h, --help             Show help
```

## Architecture

```
┌─────────────┐     ┌─────────────────┐     ┌─────────────┐
│   Client    │────▶│   mcp-guard     │────▶│  MCP Server │
│  (Agent)    │◀────│  (Gateway)      │◀────│  (Upstream) │
└─────────────┘     └─────────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    ▼             ▼
              ┌─────────┐   ┌──────────┐
              │Prometheus│   │  Jaeger  │
              │ /metrics │   │  (OTLP)  │
              └─────────┘   └──────────┘
```

## License

AGPL-3.0
