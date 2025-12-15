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
| mTLS Client Certificate Auth | ✅ |
| Per-Identity Rate Limiting | ✅ |
| Per-Tool Authorization | ✅ |
| Tools/List Filtering | ✅ |
| Prometheus Metrics | ✅ |
| OpenTelemetry Tracing | ✅ |
| W3C Trace Context Propagation | ✅ |
| Audit Logging | ✅ |
| Audit Log Shipping (SIEM integration) | ✅ |
| Stdio Transport | ✅ |
| HTTP Transport | ✅ |
| SSE Transport | ✅ |
| Multi-Server Routing | ✅ |
| Health Check Endpoints (/health, /live, /ready) | ✅ |

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

### HTTP Transport

Connect to an HTTP-based MCP server:

```toml
[upstream]
transport = "http"
url = "http://localhost:8080/mcp"
```

### SSE Transport

Connect via Server-Sent Events for streaming responses:

```toml
[upstream]
transport = "sse"
url = "http://localhost:8080/mcp/stream"
```

### mTLS Client Certificate Authentication

For enterprise service-to-service authentication via reverse proxy:

```toml
# Enable mTLS via reverse proxy headers
[auth.mtls]
enabled = true
identity_source = "cn"  # Extract identity from: cn, san_dns, san_email
allowed_tools = []      # Empty = all tools allowed
rate_limit = 1000       # Optional custom rate limit

# Configure your reverse proxy (nginx) to forward cert info:
# proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
# proxy_set_header X-Client-Cert-Verified $ssl_client_verify;
```

### Multi-Server Routing

Route requests to different MCP servers based on path prefix:

```toml
[upstream]
transport = "stdio"  # Default (ignored when servers configured)
command = "echo"     # Default (ignored when servers configured)

# Define multiple upstream servers
[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[upstream.servers]]
name = "database"
path_prefix = "/db"
transport = "http"
url = "http://localhost:8080/mcp"

[[upstream.servers]]
name = "default"
path_prefix = "/"
transport = "stdio"
command = "node"
args = ["./default-server.js"]
```

Access servers via `/mcp/:server_name` or by path prefix matching. List available routes at `GET /routes`.

### Audit Log Shipping (SIEM Integration)

Export audit logs to HTTP endpoints for SIEM integration:

```toml
[audit]
enabled = true
stdout = true

# Export to Splunk, Datadog, or custom SIEM
export_url = "https://hec.splunk.example.com/services/collector/event"
export_batch_size = 100          # Logs per batch
export_interval_secs = 30        # Max time between flushes
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
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
| `/health` | GET | Health check with version and uptime |
| `/live` | GET | Liveness probe (for Kubernetes) |
| `/ready` | GET | Readiness probe (checks transport) |
| `/metrics` | GET | Prometheus metrics |
| `/mcp` | POST | MCP message handler (requires auth) |
| `/mcp/:server` | POST | Route to specific server (multi-server mode) |
| `/routes` | GET | List available server routes (multi-server mode) |
| `/oauth/authorize` | GET | Start OAuth flow |
| `/oauth/callback` | GET | OAuth callback |

### Health Check Responses

**GET /health** - Detailed health status
```json
{"status": "healthy", "version": "0.1.0", "uptime_secs": 3600}
```

**GET /live** - Kubernetes liveness probe
```json
{"status": "alive"}
```

**GET /ready** - Kubernetes readiness probe (200 if ready, 503 if not)
```json
{"ready": true, "version": "0.1.0"}
```

## CLI Reference

```bash
mcp-guard init [--format toml|yaml] [--force]       # Generate config file
mcp-guard validate                                   # Validate config
mcp-guard keygen --user-id X [--rate-limit N] [--tools T]  # Generate API key
mcp-guard hash-key <key>                             # Hash an existing key
mcp-guard run [--host H] [--port P]                 # Start the gateway
mcp-guard version                                    # Show version and build info
mcp-guard check-upstream [--timeout N]              # Test upstream connectivity
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

## Security Considerations

### Credential Management

- **API Keys**: Store only hashed keys in config files. Generate keys with `mcp-guard keygen`.
- **OAuth Secrets**: Use environment variables or a secrets manager for `client_secret`.
- **JWT Secrets**: For JWKS mode, no local secrets needed. For simple mode, use a strong random secret (32+ chars).

### Network Security

- **TLS**: Use a reverse proxy (nginx, Caddy) for TLS termination in production.
- **mTLS**: For service-to-service auth, configure your reverse proxy to forward client cert headers.
- **JWKS URLs**: Must use HTTPS in production builds (enforced by config validation).

### Headers & Responses

mcp-guard adds security headers to all responses:
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Content-Security-Policy: default-src 'none'`

Error responses include a unique `error_id` for correlation but never expose:
- Internal file paths or commands
- Database connection strings
- Stack traces or internal errors

### Rate Limiting Headers

Successful requests include standard rate limit headers:
- `X-RateLimit-Limit`: Maximum requests per second
- `X-RateLimit-Remaining`: Approximate remaining requests
- `X-RateLimit-Reset`: Unix timestamp when limit resets

## Performance Tuning

### Targets

mcp-guard is designed to meet these performance targets:

| Metric | Target |
|--------|--------|
| Latency overhead | <2ms p99 |
| Throughput | >5,000 RPS |
| Memory | <50MB RSS |
| Binary size | <15MB |

### Rate Limiting

```toml
[rate_limit]
requests_per_second = 100  # Global default RPS
burst_size = 50            # Allows short traffic spikes
```

Per-identity limits can be set in API key config or JWT `scope_tool_mapping`.

### Memory Management

- Rate limiter entries expire after 1 hour of inactivity
- OAuth token cache uses LRU eviction (max 500 entries)
- Audit log batching reduces memory pressure

### Tracing Overhead

For high-throughput scenarios, reduce tracing overhead:

```toml
[tracing]
sample_rate = 0.1  # Sample 10% of requests
```

### Connection Pooling

For HTTP/SSE transports, mcp-guard uses connection pooling via `reqwest`. No additional configuration needed.

## Troubleshooting

### Common Issues

**Authentication failures**

```
401 Unauthorized: Invalid API key
```

- Verify the key is correct (API keys are shown only once during generation)
- Check the key hash in your config matches `mcp-guard hash-key <your-key>`
- For JWT: verify issuer and audience match your token

**Rate limit errors**

```
429 Too Many Requests
```

- Check `X-RateLimit-*` headers for current limits
- Increase `burst_size` for bursty workloads
- Set per-identity custom limits for high-volume clients

**Upstream connection issues**

```
502 Bad Gateway: Upstream communication error
```

- Test connectivity: `mcp-guard check-upstream`
- For stdio: ensure the command path is correct and executable
- For HTTP/SSE: verify the URL is reachable

**Server not ready**

```
503 Service Unavailable: Transport not initialized
```

- The upstream transport failed to initialize
- Check logs for specific errors
- Verify upstream server is running

### Debugging

Enable verbose logging:

```bash
mcp-guard run -v
```

Check server health:

```bash
curl http://localhost:3000/health
curl http://localhost:3000/ready
```

View metrics:

```bash
curl http://localhost:3000/metrics
```

### Log Correlation

All errors include an `error_id` field. Use this to correlate:
- Client error responses
- Server logs
- Distributed traces (if tracing enabled)

Example:
```json
{"error": "Rate limit exceeded", "error_id": "a1b2c3d4-..."}
```

Search logs: `grep a1b2c3d4 /var/log/mcp-guard.log`

## Comparison with Alternatives

| Feature | mcp-guard | DIY Proxy | Cloud Gateway |
|---------|-----------|-----------|---------------|
| Setup time | 5 minutes | Hours | Varies |
| Infrastructure | Single binary | Containers/VMs | Cloud account |
| Auth providers | 4 (API key, JWT, OAuth, mTLS) | Build yourself | Limited |
| Cost | Free / $12-29/mo | Development time | Usage-based |
| Latency overhead | <2ms | Variable | 10-50ms |
| Self-hosted | Yes | Yes | No |
| Audit logging | Built-in + SIEM export | Build yourself | Usually |

## License

AGPL-3.0
