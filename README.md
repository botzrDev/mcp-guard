<div align="center">
  <h1>mcp-guard</h1>
  <h3>Secure your MCP servers in 5 minutes.</h3>
  <p>No Docker. No Kubernetes. No DevOps team.</p>

  <a href="#features">Features</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#metrics">Metrics</a>
</div>

---

## The Problem

Model Context Protocol (MCP) is powerful, but most servers are deployed with **zero authentication**. If your agent can access it, so can anyone else.

## The Solution

`mcp-guard` is a lightweight security gateway that wraps any MCP server with production-grade protection:

- **Authentication**: API Keys, JWT (HS256 + JWKS)
- **Rate Limiting**: Per-identity limits with configurable burst
- **Observability**: Prometheus metrics endpoint
- **Audit Logging**: Track every request

## Features

| Feature | Status |
|---------|--------|
| API Key Authentication | ✅ |
| JWT Authentication (HS256) | ✅ |
| JWT Authentication (JWKS/RS256) | ✅ |
| Per-Identity Rate Limiting | ✅ |
| Prometheus Metrics | ✅ |
| Audit Logging | ✅ |
| Per-Tool Authorization | ✅ |
| OAuth 2.1 | Coming Soon |

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

Example `mcp-guard.toml`:

```toml
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[auth]
[[auth.api_keys]]
id = "my-agent"
key_hash = "YOUR_KEY_HASH_HERE"
allowed_tools = ["read", "list"]
rate_limit = 100

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
```

## Metrics

Prometheus metrics are exposed at `GET /metrics`:

| Metric | Type | Description |
|--------|------|-------------|
| `mcp_guard_requests_total` | Counter | Total requests (labels: method, status) |
| `mcp_guard_request_duration_seconds` | Histogram | Request latency (labels: method) |
| `mcp_guard_auth_total` | Counter | Auth attempts (labels: provider, result) |
| `mcp_guard_rate_limit_total` | Counter | Rate limit checks (labels: allowed) |
| `mcp_guard_active_identities` | Gauge | Tracked identity count |

## CLI Reference

```bash
mcp-guard init              # Generate config file
mcp-guard validate          # Validate config
mcp-guard keygen --user-id X  # Generate API key
mcp-guard run               # Start the gateway
```

## License

AGPL-3.0
