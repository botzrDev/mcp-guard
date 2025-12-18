# Configuration Reference

Complete reference for all MCP Guard configuration options.

## Overview

MCP Guard uses a configuration file to define server settings, authentication providers, rate limiting, audit logging, and upstream connections.

### Supported Formats

- **TOML** (recommended): `mcp-guard.toml`
- **YAML**: `mcp-guard.yaml` or `mcp-guard.yml`

### File Locations

1. Explicit path: `mcp-guard run --config /path/to/config.toml`
2. Current directory: `./mcp-guard.toml`

### Validation

Always validate configuration before deployment:

```bash
mcp-guard validate
mcp-guard validate --config production.toml
```

---

## [server] Section

Server and TLS configuration.

### Basic Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | string | `"127.0.0.1"` | Host to bind to |
| `port` | integer | `3000` | Port to listen on (1-65535) |

**Example:**

```toml
[server]
host = "0.0.0.0"  # Listen on all interfaces
port = 3000
```

### TLS Configuration

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tls.cert_path` | string | Yes (for TLS) | Path to server certificate (PEM) |
| `tls.key_path` | string | Yes (for TLS) | Path to server private key (PEM) |
| `tls.client_ca_path` | string | No | Path to CA for client cert validation (enables mTLS) |

**Example: HTTPS Server**

```toml
[server]
host = "0.0.0.0"
port = 443

[server.tls]
cert_path = "/etc/ssl/server.crt"
key_path = "/etc/ssl/server.key"
```

**Example: mTLS (Mutual TLS)**

```toml
[server]
host = "0.0.0.0"
port = 443

[server.tls]
cert_path = "/etc/ssl/server.crt"
key_path = "/etc/ssl/server.key"
client_ca_path = "/etc/ssl/client-ca.crt"  # Validates client certificates
```

---

## [auth] Section

Authentication configuration. Multiple providers can be enabled simultaneously.

### API Keys [[auth.api_keys]]

Pre-shared API keys with SHA-256 hashing.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier for the key holder |
| `key_hash` | string | Yes | Base64-encoded SHA-256 hash of the API key |
| `allowed_tools` | array | No | List of allowed tool names (empty = all) |
| `rate_limit` | integer | No | Custom rate limit (requests/second) |

**Generate keys:**

```bash
mcp-guard keygen --user-id alice
```

**Example:**

```toml
[[auth.api_keys]]
id = "service-a"
key_hash = "abc123..."
allowed_tools = ["read_file", "list_directory"]
rate_limit = 100

[[auth.api_keys]]
id = "admin-service"
key_hash = "def456..."
# No allowed_tools = all tools allowed
rate_limit = 1000
```

**Client Usage:**

```bash
curl -H "Authorization: Bearer mcp_YOUR_API_KEY" http://localhost:3000/mcp
```

---

### JWT [auth.jwt]

JSON Web Token authentication with two modes: simple (HS256) and JWKS (RS256/ES256).

#### Common Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | string | Required | `"simple"` or `"jwks"` |
| `issuer` | string | Required | Expected `iss` claim value |
| `audience` | string | Required | Expected `aud` claim value |
| `user_id_claim` | string | `"sub"` | Claim to extract user ID from |
| `scopes_claim` | string | `"scope"` | Claim to extract scopes from |
| `leeway_secs` | integer | `0` | Clock skew tolerance in seconds |

#### Simple Mode (HS256)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret` | string | Yes | Shared secret for HMAC-SHA256 (min 32 chars recommended) |

**Example:**

```toml
[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-here-minimum-32-characters"
issuer = "https://your-app.com"
audience = "mcp-guard"
leeway_secs = 60  # Allow 60s clock skew
```

#### JWKS Mode (RS256/ES256)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `jwks_url` | string | Required | URL to JWKS endpoint (HTTPS required in production) |
| `algorithms` | array | `["RS256", "ES256"]` | Allowed signing algorithms |
| `cache_duration_secs` | integer | `3600` | JWKS cache TTL in seconds |

**Example: Auth0**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://YOUR_DOMAIN.auth0.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://YOUR_DOMAIN.auth0.com/"
audience = "mcp-guard"
scopes_claim = "permissions"  # Auth0 uses 'permissions' for RBAC
```

**Example: Keycloak**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://keycloak.example.com/realms/YOUR_REALM/protocol/openid-connect/certs"
algorithms = ["RS256"]
issuer = "https://keycloak.example.com/realms/YOUR_REALM"
audience = "mcp-guard"
user_id_claim = "preferred_username"
```

#### Scope-to-Tool Mapping

Map JWT scopes to allowed MCP tools.

```toml
[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]  # Wildcard = all tools
```

**Mapping Logic:**
- No mapping configured → All tools allowed
- Scope maps to `["*"]` → All tools allowed
- Otherwise → Only tools from matched scopes

For detailed JWT setup, see the [Authentication Guide](authentication.md#jwt-authentication).

---

### OAuth [auth.oauth]

OAuth 2.1 authentication with PKCE support.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `provider` | string | Required | `"github"`, `"google"`, `"okta"`, or `"custom"` |
| `client_id` | string | Required | OAuth client ID |
| `client_secret` | string | No | OAuth client secret |
| `redirect_uri` | string | `"http://localhost:3000/oauth/callback"` | Callback URL |
| `scopes` | array | `["openid", "profile"]` | OAuth scopes to request |
| `user_id_claim` | string | `"sub"` | Claim to extract user ID from |

**Custom Provider Fields (required when `provider = "custom"`):**

| Field | Type | Description |
|-------|------|-------------|
| `authorization_url` | string | Authorization endpoint |
| `token_url` | string | Token endpoint |
| `introspection_url` | string | Token introspection endpoint (optional) |
| `userinfo_url` | string | UserInfo endpoint (optional) |

**Example: GitHub**

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["read:user", "repo"]
user_id_claim = "id"  # GitHub uses numeric ID

[auth.oauth.scope_tool_mapping]
"read:user" = ["read_file"]
"repo" = ["read_file", "write_file"]
```

**Example: Google**

```toml
[auth.oauth]
provider = "google"
client_id = "your-client-id.apps.googleusercontent.com"
client_secret = "your-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["openid", "profile", "email"]
```

**Example: Custom Provider**

```toml
[auth.oauth]
provider = "custom"
client_id = "your-client-id"
client_secret = "your-client-secret"
authorization_url = "https://your-idp.com/oauth/authorize"
token_url = "https://your-idp.com/oauth/token"
introspection_url = "https://your-idp.com/oauth/introspect"
userinfo_url = "https://your-idp.com/userinfo"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["openid", "profile"]
```

For detailed OAuth setup, see the [Authentication Guide](authentication.md#oauth-21-authentication).

---

### mTLS [auth.mtls]

Client certificate authentication via reverse proxy headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `false` | Enable mTLS authentication |
| `identity_source` | string | `"cn"` | Certificate field for identity: `"cn"`, `"san_dns"`, or `"san_email"` |
| `allowed_tools` | array | `[]` | Allowed tools (empty = all) |
| `rate_limit` | integer | None | Custom rate limit (requests/second) |
| `trusted_proxy_ips` | array | `[]` | **REQUIRED**: Trusted proxy IP addresses/CIDR ranges |

**Security Critical:** You **must** configure `trusted_proxy_ips` when enabling mTLS to prevent header spoofing attacks.

**Example:**

```toml
[auth.mtls]
enabled = true
identity_source = "cn"
allowed_tools = ["read_file", "write_file"]
rate_limit = 1000
trusted_proxy_ips = ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"]
```

**nginx Configuration:**

```nginx
ssl_client_certificate /etc/nginx/client-ca.pem;
ssl_verify_client on;

proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
proxy_set_header X-Client-Cert-SAN-DNS $ssl_client_s_dn_san_dns_0;
proxy_set_header X-Client-Cert-Verified $ssl_client_verify;
```

For detailed mTLS setup, see the [Authentication Guide](authentication.md#mtls-authentication).

---

## [rate_limit] Section

Per-identity rate limiting using token bucket algorithm.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable rate limiting |
| `requests_per_second` | integer | `100` | Default rate limit (must be > 0) |
| `burst_size` | integer | `50` | Burst allowance (must be > 0) |

**Example:**

```toml
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50
```

**Per-Identity Overrides:**

Rate limits can be overridden per identity:

```toml
[[auth.api_keys]]
id = "high-volume-service"
key_hash = "..."
rate_limit = 1000  # Override default 100 RPS
```

**Response Headers:**

Successful requests include:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 47
X-RateLimit-Reset: 1702656789
```

Rate-limited requests (429) include:

```
Retry-After: 1
```

**Memory Management:**

Rate limiter entries expire after 1 hour of inactivity to prevent unbounded memory growth.

---

## [audit] Section

Audit logging configuration with file, stdout, and HTTP export options.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable audit logging |
| `stdout` | boolean | `false` | Log to stdout |
| `file` | string | None | File path for audit logs |
| `export_url` | string | None | HTTP endpoint for SIEM integration |
| `export_batch_size` | integer | `100` | Logs per batch (1-10000) |
| `export_interval_secs` | integer | `30` | Max seconds between flushes |
| `export_headers` | table | `{}` | Custom headers for HTTP export |

**Security Note:** `stdout` defaults to `false` to prevent accidental PII exposure in container logs.

**Example: File Logging**

```toml
[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
```

**Example: Splunk HEC**

```toml
[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 15
export_headers = { "Authorization" = "Splunk YOUR_HEC_TOKEN" }
```

**Example: Datadog**

```toml
[audit]
enabled = true
export_url = "https://http-intake.logs.datadoghq.com/api/v2/logs"
export_batch_size = 50
export_interval_secs = 10
export_headers = { "DD-API-KEY" = "your-datadog-api-key" }
```

**Example: Multiple Outputs**

```toml
[audit]
enabled = true
stdout = true
file = "/var/log/mcp-guard/audit.log"
export_url = "https://siem.example.com/api/logs"
export_headers = { "Authorization" = "Bearer token" }
```

**Audit Event Types:**

- Authentication success/failure
- Authorization denied
- Rate limit exceeded
- Tool calls with duration

---

## [tracing] Section

OpenTelemetry distributed tracing with W3C trace context propagation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `false` | Enable OpenTelemetry tracing |
| `service_name` | string | `"mcp-guard"` | Service name in traces |
| `otlp_endpoint` | string | Required (if enabled) | OTLP gRPC endpoint |
| `sample_rate` | float | `0.1` | Sampling rate (0.0-1.0) |
| `propagate_context` | boolean | `true` | Extract/inject W3C traceparent headers |

**Security Note:** `sample_rate` defaults to 0.1 (10%) for production safety. Use 1.0 for development.

**Example: Jaeger**

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://localhost:4317"  # gRPC port
sample_rate = 1.0  # 100% for development
propagate_context = true
```

**Example: Grafana Tempo**

```toml
[tracing]
enabled = true
service_name = "mcp-guard"
otlp_endpoint = "http://tempo.monitoring.svc:4317"
sample_rate = 0.1  # 10% for production
propagate_context = true
```

**W3C Headers:**

When `propagate_context = true`:
- Extracts `traceparent` and `tracestate` from incoming requests
- Injects trace context into upstream requests
- Includes `trace_id` in audit logs

---

## [upstream] Section

Upstream MCP server configuration. Supports single-server or multi-server routing.

### Single-Server Mode

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transport` | string | Yes | `"stdio"`, `"http"`, or `"sse"` |
| `command` | string | For stdio | Command to execute |
| `args` | array | No | Command arguments |
| `url` | string | For http/sse | Upstream URL |

**Example: Stdio Transport**

```toml
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
```

**Example: Python Server**

```toml
[upstream]
transport = "stdio"
command = "python"
args = ["-m", "my_mcp_server"]
```

**Example: HTTP Transport**

```toml
[upstream]
transport = "http"
url = "http://localhost:8080/mcp"
```

**Example: SSE Transport**

```toml
[upstream]
transport = "sse"
url = "http://localhost:8080/mcp/stream"
```

### Multi-Server Routing Mode

When `[[upstream.servers]]` is configured, path-based routing is enabled.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Unique server identifier |
| `path_prefix` | string | Yes | Path prefix to match (must start with `/`) |
| `transport` | string | Yes | `"stdio"`, `"http"`, or `"sse"` |
| `command` | string | For stdio | Command to execute |
| `args` | array | No | Command arguments |
| `url` | string | For http/sse | Upstream URL |
| `strip_prefix` | boolean | No | Strip prefix when forwarding |

**Example: Multiple Servers**

```toml
[upstream]
transport = "stdio"  # Default (ignored when servers configured)
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

[[upstream.servers]]
name = "database"
path_prefix = "/db"
transport = "http"
url = "http://localhost:8081/mcp"
```

**Routing Algorithm:**

- Longest prefix match wins
- Case-sensitive matching
- Use `GET /routes` to list available routes

**Accessing Servers:**

```bash
# By server name
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer ..." \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'

# List available routes
curl http://localhost:3000/routes
```

---

## Complete Examples

### Minimal Development Configuration

```toml
[server]
host = "127.0.0.1"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

[[auth.api_keys]]
id = "dev"
key_hash = "YOUR_HASH"

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true
```

### Production Configuration with JWT

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "http"
url = "http://mcp-server.internal:8080/mcp"

[auth.jwt]
mode = "jwks"
jwks_url = "https://auth.example.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://auth.example.com/"
audience = "mcp-guard"

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file"]
"admin" = ["*"]

[rate_limit]
enabled = true
requests_per_second = 500
burst_size = 100

[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
export_url = "https://siem.example.com/logs"
export_headers = { "Authorization" = "Bearer siem-token" }

[tracing]
enabled = true
service_name = "mcp-guard-prod"
otlp_endpoint = "http://tempo.monitoring:4317"
sample_rate = 0.1
```

### Enterprise Configuration with OAuth + mTLS

```toml
[server]
host = "0.0.0.0"
port = 443

[server.tls]
cert_path = "/etc/ssl/server.crt"
key_path = "/etc/ssl/server.key"
client_ca_path = "/etc/ssl/client-ca.crt"

[upstream]
transport = "http"
url = "http://mcp-cluster.internal:8080/mcp"

# API keys for automated services
[[auth.api_keys]]
id = "ci-pipeline"
key_hash = "..."
allowed_tools = ["read_file", "list_directory"]
rate_limit = 1000

# OAuth for user authentication
[auth.oauth]
provider = "custom"
client_id = "mcp-guard"
client_secret = "..."
authorization_url = "https://idp.example.com/oauth/authorize"
token_url = "https://idp.example.com/oauth/token"
introspection_url = "https://idp.example.com/oauth/introspect"
redirect_uri = "https://mcp.example.com/oauth/callback"

# mTLS for internal services
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.0/8"]
rate_limit = 5000

[rate_limit]
enabled = true
requests_per_second = 1000
burst_size = 200

[audit]
enabled = true
export_url = "https://splunk.example.com:8088/services/collector/event"
export_batch_size = 100
export_interval_secs = 10
export_headers = { "Authorization" = "Splunk HEC-TOKEN" }

[tracing]
enabled = true
service_name = "mcp-guard-enterprise"
otlp_endpoint = "http://otel-collector.monitoring:4317"
sample_rate = 0.05  # 5% sampling for high-volume
```

### Multi-Tenant Configuration with Routing

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "stdio"
command = "echo"

[[upstream.servers]]
name = "tenant-a"
path_prefix = "/tenant-a"
transport = "http"
url = "http://mcp-tenant-a.internal:8080/mcp"

[[upstream.servers]]
name = "tenant-b"
path_prefix = "/tenant-b"
transport = "http"
url = "http://mcp-tenant-b.internal:8080/mcp"

[[upstream.servers]]
name = "shared"
path_prefix = "/"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-shared"]

[[auth.api_keys]]
id = "tenant-a-service"
key_hash = "..."

[[auth.api_keys]]
id = "tenant-b-service"
key_hash = "..."

[rate_limit]
enabled = true
requests_per_second = 500
burst_size = 100

[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
```

---

## Validation Rules Summary

| Field | Rule |
|-------|------|
| `server.port` | Must be 1-65535 |
| `auth.jwt.jwks_url` | HTTPS required in production |
| `auth.jwt.secret` | Minimum 32 characters recommended |
| `auth.oauth.redirect_uri` | Valid HTTP(S) URL |
| `auth.mtls.trusted_proxy_ips` | Required when mTLS enabled |
| `rate_limit.requests_per_second` | Must be > 0 |
| `rate_limit.burst_size` | Must be > 0 |
| `tracing.sample_rate` | Must be 0.0-1.0 |
| `audit.export_batch_size` | Must be 1-10000 |
| `upstream.path_prefix` | Must start with `/` |

---

## See Also

- [Quick Start Guide](quickstart.md) - Get started in 5 minutes
- [CLI Reference](cli.md) - Command-line options
- [Authentication Guide](authentication.md) - Detailed auth provider setup
