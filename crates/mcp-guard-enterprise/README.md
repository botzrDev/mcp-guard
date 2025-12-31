# MCP-Guard Enterprise

**Enterprise-grade security and observability for Model Context Protocol (MCP) servers**

[![License: Commercial](https://img.shields.io/badge/license-Commercial-red.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-mcp--guard.io-blue.svg)](https://docs.mcp-guard.io)

---

## Features Included

MCP-Guard Enterprise includes **all Pro features** plus advanced security, observability, and multi-tenancy capabilities:

### Authentication

- ✅ **mTLS Client Certificate Authentication** - Certificate-based authentication
  - Extract identity from client certificates (CN, SAN DNS, SAN Email)
  - Reverse proxy integration (Nginx, Traefik, etc.)
  - Trusted proxy IP validation to prevent header spoofing
  - Per-certificate tool permissions and rate limits

- ✅ **All Pro Authentication** - OAuth 2.1, JWT JWKS, API Keys

### Multi-Tenancy

- ✅ **Multi-Server Routing** - Route to multiple upstream MCP servers
  - Path-based routing (`/github/*` → GitHub MCP, `/slack/*` → Slack MCP)
  - Mixed transport types (stdio, HTTP, SSE) across servers
  - Longest-prefix matching for complex routing rules
  - `/routes` endpoint to list available servers

### Observability

- ✅ **OpenTelemetry Distributed Tracing** - Full request tracing
  - OTLP gRPC export to Jaeger, Tempo, or custom collectors
  - W3C trace context propagation (traceparent/tracestate)
  - Configurable sampling rates (0.0 - 1.0)
  - Trace IDs in audit logs for correlation

- ✅ **SIEM Audit Log Shipping** - Real-time log export
  - HTTP export to Splunk, Datadog, Elastic, or custom endpoints
  - Batching and buffering for high throughput
  - Exponential backoff retry (3 attempts)
  - Custom headers for authentication
  - Non-blocking async I/O

### Advanced Rate Limiting

- ✅ **Per-Tool Rate Limiting** - Fine-grained rate control
  - Different limits per tool (e.g., 10 RPS for writes, 100 RPS for reads)
  - Glob pattern matching (`filesystem/*`)
  - Per-identity overrides
  - Burst control per tool

### Admin Tools

- ✅ **Guard Tools** - CLI utilities for management
  - Bulk API key generation
  - Audit log viewer and analyzer
  - Configuration generator and validator
  - License diagnostics

---

## Pricing

**$29/user/month** (minimum 5 seats) + **$8/seat** for additional users

Includes:
- All features listed above
- Priority email support (4hr SLA for critical issues)
- Dedicated Slack channel (10+ seats)
- Quarterly check-ins
- Custom feature requests (subject to approval)

[Get your Enterprise license →](https://mcp-guard.io/pricing)

---

## Installation

### Prerequisites

- Valid Enterprise license key (starts with `ent_`)
- Rust 1.75+ (if building from source)
- Internet connection for initial license validation

### Option 1: Pre-built Binary (Recommended)

```bash
# Download Enterprise binary
curl -fsSL https://mcp-guard.io/install-enterprise.sh | bash

# Verify installation
mcp-guard version
# Should show: Tier: Enterprise
```

### Option 2: Build from Source

```bash
# Clone repository
git clone https://github.com/botzrdev/mcp-guard.git
cd mcp-guard

# Build with Enterprise features
cargo build --release --features enterprise

# Binary will be at: target/release/mcp-guard
```

---

## Quick Start

### 1. Set Your License Key

```bash
export MCP_GUARD_LICENSE_KEY="ent_xxx..."
```

Enterprise licenses require online validation on first run. After successful validation, the license is cached for 30 days (offline grace period).

### 2. Generate Configuration

```bash
mcp-guard init
```

### 3. Configure Multi-Server Routing

Edit `mcp-guard.toml`:

```toml
# Define multiple upstream MCP servers
[[upstream.servers]]
name = "github"
path_prefix = "/github"
transport = "http"
url = "http://github-mcp:8080"

[[upstream.servers]]
name = "slack"
path_prefix = "/slack"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-slack"]

[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "sse"
url = "http://filesystem-mcp:8080/stream"
```

### 4. Configure OpenTelemetry Tracing

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://jaeger:4317"
sample_rate = 0.1  # Sample 10% of requests
propagate_context = true
```

### 5. Configure SIEM Audit Export

```toml
[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 30

[audit.export_headers]
"Authorization" = "Splunk YOUR_HEC_TOKEN"
"Content-Type" = "application/json"
```

### 6. Start the Server

```bash
mcp-guard run
```

### 7. Access Multi-Server Routes

```bash
# List available routes
curl http://localhost:3000/routes

# Route to GitHub MCP server
curl -H "Authorization: Bearer <token>" \
  http://localhost:3000/github/tools/list

# Route to Slack MCP server
curl -H "Authorization: Bearer <token>" \
  http://localhost:3000/slack/tools/list
```

---

## Configuration Examples

### mTLS with Nginx Reverse Proxy

**Nginx configuration:**

```nginx
server {
    listen 443 ssl;
    server_name mcp.example.com;

    # SSL certificates
    ssl_certificate /etc/nginx/ssl/server.crt;
    ssl_certificate_key /etc/nginx/ssl/server.key;

    # Client certificate validation
    ssl_client_certificate /etc/nginx/ssl/ca.crt;
    ssl_verify_client on;

    location / {
        # Forward client cert info to MCP-Guard
        proxy_set_header X-Client-Cert-Verified $ssl_client_verify;
        proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
        proxy_set_header X-Client-Cert-SAN-DNS $ssl_client_s_dn_san_dns;

        proxy_pass http://mcp-guard:3000;
    }
}
```

**MCP-Guard configuration:**

```toml
[auth.mtls]
enabled = true
identity_source = "cn"  # Extract identity from Common Name
trusted_proxy_ips = ["10.0.0.0/8"]  # Trust internal network
```

### Per-Tool Rate Limiting

```toml
[rate_limit]
enabled = true

# Default: 100 RPS
requests_per_second = 100
burst_size = 20

# Tool-specific limits
[[rate_limit.tool_limits]]
pattern = "filesystem/write"
requests_per_second = 10
burst_size = 5

[[rate_limit.tool_limits]]
pattern = "filesystem/delete"
requests_per_second = 5
burst_size = 2

[[rate_limit.tool_limits]]
pattern = "filesystem/read*"  # Glob pattern
requests_per_second = 200
burst_size = 50
```

### OpenTelemetry with Jaeger

```toml
[tracing]
enabled = true
service_name = "mcp-guard-production"
otlp_endpoint = "http://jaeger-collector:4317"
sample_rate = 1.0  # 100% sampling (adjust for production)
propagate_context = true
```

View traces at: `http://jaeger-ui:16686`

---

## License Validation

### Online Validation

Enterprise licenses use [Keygen.sh](https://keygen.sh) for online validation:

1. **First Run**: Validates license online, caches result for 30 days
2. **Subsequent Runs**: Uses cached validation (offline capable)
3. **After 30 Days**: Must revalidate online

### Offline Grace Period

Enterprise deployments can run **offline for up to 30 days** after successful online validation. This supports:

- Air-gapped environments (with initial activation)
- Temporary network outages
- Travel/remote work scenarios

### Seat Management

Enterprise licenses include a specified number of "seats" (users/instances). Seat usage is tracked via license validation. Exceeding seat limits will result in validation failures.

**To add seats:**
1. Contact [austin@botzr.dev](mailto:austin@botzr.dev)
2. Additional seats: $8/user/month
3. Updated license key will be issued

---

## Support

### Priority Email Support

**Response SLA:**
- Critical issues: 4 hours
- High priority: 24 hours
- Normal priority: 48 hours

Contact: [austin@botzr.dev](mailto:austin@botzr.dev)

### Dedicated Slack Channel

For teams with 10+ seats, we provide a dedicated Slack channel for:
- Real-time support
- Feature discussions
- Direct access to engineering team

Request Slack access: [austin@botzr.dev](mailto:austin@botzr.dev)

### Quarterly Check-ins

Enterprise customers receive quarterly check-in calls to:
- Review usage and performance
- Discuss new features
- Plan upgrades and scaling

### Documentation

- [Enterprise Setup Guide](https://docs.mcp-guard.io/enterprise/setup)
- [Multi-Server Routing](https://docs.mcp-guard.io/enterprise/multi-server)
- [mTLS Configuration](https://docs.mcp-guard.io/enterprise/mtls)
- [Keygen.sh Setup](../../docs/enterprise/keygen-setup.md)
- [SIEM Integration](https://docs.mcp-guard.io/enterprise/siem)

---

## Troubleshooting

### License Validation Fails

```
Error: Enterprise license validation failed: Network error
```

**First-time setup requires internet:**
1. Ensure firewall allows HTTPS to `api.keygen.sh`
2. Check license key is correct: `echo $MCP_GUARD_LICENSE_KEY | cut -c1-10`
3. Contact support if issue persists

### Offline Grace Period Expired

```
Error: Offline grace period expired. Please connect to the internet to revalidate.
```

**Solution:** Connect to internet and restart MCP-Guard. License will revalidate and cache for another 30 days.

### Seat Limit Exceeded

```
Error: License validation failed: Maximum seats exceeded (5/5)
```

**Solutions:**
1. Remove unused instances
2. Purchase additional seats
3. Contact [austin@botzr.dev](mailto:austin@botzr.dev) for seat expansion

### SIEM Export Failing

```
Warning: Audit export failed: Connection refused (attempt 1/3)
```

**Solutions:**
1. Verify SIEM endpoint is accessible
2. Check authentication headers are correct
3. Review SIEM ingestion logs for errors
4. Contact support for SIEM-specific configuration help

---

## Admin Guard Tools

Enterprise tier includes CLI tools for administration:

```bash
# Bulk generate API keys
mcp-guard guard-tools generate-keys --count 100 --output keys.csv

# View audit logs
mcp-guard guard-tools audit-viewer --since "2025-01-01" --filter user_id=alice

# Validate configuration
mcp-guard guard-tools validate-config --config mcp-guard.toml

# License diagnostics
mcp-guard guard-tools license-info
```

(Note: Guard tools may require separate installation or feature flag)

---

## License

This software is licensed under a commercial license and requires a valid Enterprise license key for use.

See [LICENSE](LICENSE) for full terms.

**Copyright © 2025 Austin Green / Botzr**
