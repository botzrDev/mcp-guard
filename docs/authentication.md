# Authentication Guide

Deep dive into MCP Guard's authentication providers, including setup, configuration, and troubleshooting.

## Overview

MCP Guard supports four authentication providers that can be used individually or combined:

| Provider | Use Case | Token Type |
|----------|----------|------------|
| **API Keys** | Service-to-service, CLI tools | Pre-shared key |
| **JWT** | Enterprise SSO, existing IdPs | JSON Web Token |
| **OAuth 2.1** | User authentication, third-party apps | Access token |
| **mTLS** | High-security, zero-trust | Client certificate |

### How Authentication Works

1. Client sends request with credentials
2. MCP Guard extracts credentials (mTLS headers or Bearer token)
3. Credentials are validated against configured providers
4. On success, an `Identity` is created with user info and permissions
5. Identity is used for authorization and rate limiting

### The Identity Model

Every authenticated request produces an `Identity`:

```rust
pub struct Identity {
    pub id: String,                        // Unique identifier
    pub name: Option<String>,              // Display name
    pub allowed_tools: Option<Vec<String>>, // Authorized tools
    pub rate_limit: Option<u32>,           // Custom rate limit (RPS)
    pub claims: HashMap<String, Value>,    // Additional metadata
}
```

**allowed_tools behavior:**
- `None` → All tools allowed
- `Some([])` → No tools allowed
- `Some(["read_file", ...])` → Only listed tools allowed

### Multi-Provider Support

When multiple providers are configured, they're tried in order:

1. **mTLS** (if configured and headers present)
2. **Bearer token** → tries API Key → JWT → OAuth

The first successful authentication wins. If all fail, the most informative error is returned.

---

## API Key Authentication

### Overview

API keys are pre-shared secrets for simple, secure authentication. They're ideal for:

- Service-to-service communication
- CLI tools and scripts
- Simple deployments without an IdP

### Security Model

- Keys are SHA-256 hashed before storage
- Constant-time comparison prevents timing attacks
- Keys cannot be recovered from configuration (hash only)

### Configuration

```toml
[[auth.api_keys]]
id = "my-service"
key_hash = "base64-encoded-sha256-hash"
allowed_tools = ["read_file", "list_directory"]  # Optional
rate_limit = 100                                  # Optional
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier |
| `key_hash` | string | Yes | Base64-encoded SHA-256 hash |
| `allowed_tools` | array | No | Authorized tools (empty = all) |
| `rate_limit` | integer | No | Custom rate limit (RPS) |

### Step-by-Step Setup

**1. Generate a new API key:**

```bash
mcp-guard keygen --user-id my-service
```

**Output:**

```
Generated API key for 'my-service':

  API Key (save this, shown only once):
    mcp_AbCdEf123456789XYZ...

  Add to your config file:

  [[auth.api_keys]]
  id = "my-service"
  key_hash = "abc123def456..."
```

**2. Add the hash to your config:**

```toml
[[auth.api_keys]]
id = "my-service"
key_hash = "abc123def456..."
```

**3. Give the API key to your client (securely)**

**4. Restart MCP Guard**

### Client Usage

Include the API key in the `Authorization` header:

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer mcp_AbCdEf123456789XYZ..." \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

### Best Practices

1. **Use unique keys per client** - Easier to revoke and audit
2. **Set appropriate rate limits** - Prevent abuse
3. **Restrict tools when possible** - Principle of least privilege
4. **Rotate keys periodically** - Generate new keys, update clients, remove old
5. **Store keys securely** - Use secrets managers, not environment variables in logs

### Troubleshooting

**"Invalid API key" error:**

1. Verify you're using the full key (starts with `mcp_`)
2. Check the hash matches: `mcp-guard hash-key "mcp_YOUR_KEY"`
3. Ensure no whitespace or newlines in the key

**Key not working after restart:**

1. Verify config file was saved
2. Run `mcp-guard validate` to check syntax
3. Check for duplicate `id` values

---

## JWT Authentication

### Overview

JWT (JSON Web Token) authentication integrates with existing identity providers like Auth0, Keycloak, Okta, and AWS Cognito. It supports two modes:

- **Simple (HS256)**: Symmetric key, shared secret
- **JWKS (RS256/ES256)**: Asymmetric keys from IdP

### Simple Mode (HS256)

Use simple mode for:
- Development and testing
- Internal services with shared secrets
- Scenarios without an external IdP

**Configuration:**

```toml
[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-here-minimum-32-characters"
issuer = "https://your-app.com"
audience = "mcp-guard"
user_id_claim = "sub"
scopes_claim = "scope"
leeway_secs = 60
```

**Security Considerations:**

- Use a strong random secret (32+ characters)
- Rotate secrets periodically
- Both client and server must have the secret

**Creating Test Tokens:**

Using Node.js:

```javascript
const jwt = require('jsonwebtoken');

const token = jwt.sign(
  {
    sub: 'user123',
    scope: 'read:files write:files',
    name: 'Test User'
  },
  'your-256-bit-secret-key-here-minimum-32-characters',
  {
    issuer: 'https://your-app.com',
    audience: 'mcp-guard',
    expiresIn: '1h'
  }
);

console.log(token);
```

### JWKS Mode (RS256/ES256)

Use JWKS mode for production with identity providers. Public keys are fetched automatically from the IdP's JWKS endpoint.

**Configuration:**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://your-idp.com/.well-known/jwks.json"
algorithms = ["RS256", "ES256"]
cache_duration_secs = 3600
issuer = "https://your-idp.com/"
audience = "mcp-guard"
```

**JWKS Cache Behavior:**

- Keys are cached for `cache_duration_secs` (default: 1 hour)
- Background refresh at 75% of cache duration
- 10-second timeout for JWKS endpoint calls
- Graceful fallback to cached keys on fetch failure

### Claims Mapping

Configure how JWT claims map to Identity:

| Config Field | Default | Description |
|--------------|---------|-------------|
| `user_id_claim` | `"sub"` | Claim for user ID |
| `scopes_claim` | `"scope"` | Claim for scopes |

**Scope formats accepted:**

```json
// Space-separated string
{"scope": "read:files write:files"}

// Array of strings
{"scope": ["read:files", "write:files"]}
```

### Scope-to-Tool Mapping

Map JWT scopes to MCP tools:

```toml
[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]  # Wildcard = all tools
```

**Mapping Logic:**

1. No mapping configured → All tools allowed
2. User has scope mapping to `["*"]` → All tools allowed
3. Otherwise → Union of tools from all matched scopes

### Auth0 Setup

**1. Create an API in Auth0:**

- Go to **Applications** → **APIs** → **Create API**
- Name: `mcp-guard`
- Identifier (audience): `mcp-guard`
- Signing Algorithm: RS256

**2. Configure Permissions (Scopes):**

- Go to your API → **Permissions** tab
- Add permissions: `read:files`, `write:files`, `admin`

**3. Get JWKS URL:**

Your JWKS URL is: `https://YOUR_DOMAIN.auth0.com/.well-known/jwks.json`

**4. Configure MCP Guard:**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://YOUR_DOMAIN.auth0.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://YOUR_DOMAIN.auth0.com/"
audience = "mcp-guard"
scopes_claim = "permissions"  # Auth0 uses 'permissions' for RBAC

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file"]
"admin" = ["*"]
```

**5. Get a Test Token:**

```bash
curl -X POST "https://YOUR_DOMAIN.auth0.com/oauth/token" \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "YOUR_CLIENT_ID",
    "client_secret": "YOUR_CLIENT_SECRET",
    "audience": "mcp-guard",
    "grant_type": "client_credentials"
  }'
```

**6. Test Authentication:**

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

### Keycloak Setup

**1. Create a Realm (if needed):**

- Go to **Master** dropdown → **Create Realm**
- Name: `mcp`

**2. Create a Client:**

- Go to **Clients** → **Create client**
- Client ID: `mcp-guard`
- Client Protocol: `openid-connect`
- Access Type: `confidential` (or `public` for PKCE-only)

**3. Configure Client Scopes:**

- Go to **Client Scopes** → **Create client scope**
- Create: `read:files`, `write:files`, `admin`
- Assign to your client: **Clients** → `mcp-guard` → **Client scopes** → **Add**

**4. Get JWKS URL:**

```
https://YOUR_KEYCLOAK/realms/YOUR_REALM/protocol/openid-connect/certs
```

**5. Configure MCP Guard:**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://keycloak.example.com/realms/mcp/protocol/openid-connect/certs"
algorithms = ["RS256"]
issuer = "https://keycloak.example.com/realms/mcp"
audience = "mcp-guard"
user_id_claim = "preferred_username"  # Keycloak uses preferred_username
scopes_claim = "scope"

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file"]
"admin" = ["*"]
```

**6. Get a Test Token:**

```bash
curl -X POST "https://keycloak.example.com/realms/mcp/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "client_id=mcp-guard" \
  -d "client_secret=YOUR_SECRET" \
  -d "grant_type=client_credentials" \
  -d "scope=read:files write:files"
```

### Other Identity Providers

**Okta:**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://YOUR_DOMAIN.okta.com/oauth2/default/v1/keys"
issuer = "https://YOUR_DOMAIN.okta.com/oauth2/default"
audience = "mcp-guard"
```

**AWS Cognito:**

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://cognito-idp.REGION.amazonaws.com/POOL_ID/.well-known/jwks.json"
issuer = "https://cognito-idp.REGION.amazonaws.com/POOL_ID"
audience = "YOUR_APP_CLIENT_ID"
```

### Token Requirements

**Required Claims:**

| Claim | Description |
|-------|-------------|
| `iss` | Issuer (must match config) |
| `aud` | Audience (must match config) |
| `exp` | Expiration time (Unix timestamp) |
| `sub` | Subject (user ID, unless overridden) |

**Optional Claims:**

| Claim | Description |
|-------|-------------|
| `iat` | Issued at |
| `nbf` | Not before |
| `name` | User's display name |
| `scope` | Space-separated scopes or array |

### Troubleshooting

**"Invalid JWT signature":**

1. Verify JWKS URL is accessible: `curl YOUR_JWKS_URL`
2. Check algorithms match: `algorithms = ["RS256"]`
3. Verify the token was issued by the correct IdP

**"Token expired":**

1. Generate a fresh token
2. If clock skew is an issue: `leeway_secs = 60`

**"Invalid issuer":**

1. Check the `iss` claim in your token: `jwt.io`
2. Match exactly in config (including trailing slash)

**"JWKS fetch failed":**

1. Check network connectivity
2. Verify HTTPS certificate is valid
3. Check for firewalls blocking outbound connections

**"Algorithm mismatch":**

1. Check token's `alg` header matches `algorithms` config
2. JWKS mode requires RS256/ES256, not HS256

---

## OAuth 2.1 Authentication

### Overview

OAuth 2.1 enables user authentication through external providers like GitHub and Google. MCP Guard implements:

- Authorization Code flow with PKCE (RFC 7636)
- Token introspection (RFC 7662)
- UserInfo endpoint fallback

### OAuth Flow in MCP Guard

```
1. Client → GET /oauth/authorize
   ↓
2. MCP Guard redirects to OAuth provider with PKCE challenge
   ↓
3. User authenticates at provider
   ↓
4. Provider redirects to GET /oauth/callback?code=...
   ↓
5. MCP Guard exchanges code for token (with PKCE verifier)
   ↓
6. Token validated via introspection or userinfo
   ↓
7. Access token returned to client
```

### PKCE Support

All OAuth flows use Proof Key for Code Exchange (PKCE) with S256 challenge method. This prevents authorization code interception attacks.

**Automatic handling:**
- MCP Guard generates code verifier and challenge
- State parameter prevents CSRF attacks
- No additional configuration needed

### Built-in Providers

#### GitHub

**1. Create OAuth App:**

- Go to **Settings** → **Developer settings** → **OAuth Apps** → **New OAuth App**
- Application name: `MCP Guard`
- Homepage URL: `https://your-domain.com`
- Authorization callback URL: `https://your-domain.com/oauth/callback`

**2. Configure MCP Guard:**

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["read:user", "repo"]
user_id_claim = "id"  # GitHub uses numeric ID

[auth.oauth.scope_tool_mapping]
"read:user" = ["read_file", "list_directory"]
"repo" = ["read_file", "write_file"]
```

**3. Test the flow:**

```bash
# Open in browser
open "http://localhost:3000/oauth/authorize"

# After callback, use the returned token
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

#### Google

**1. Create OAuth Credentials:**

- Go to [Google Cloud Console](https://console.cloud.google.com)
- **APIs & Services** → **Credentials** → **Create Credentials** → **OAuth client ID**
- Application type: Web application
- Authorized redirect URIs: `https://your-domain.com/oauth/callback`

**2. Configure MCP Guard:**

```toml
[auth.oauth]
provider = "google"
client_id = "your-client-id.apps.googleusercontent.com"
client_secret = "your-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["openid", "profile", "email"]
user_id_claim = "sub"
```

### Custom Provider Setup

For other OAuth providers (Okta, Azure AD, etc.):

```toml
[auth.oauth]
provider = "custom"
client_id = "your-client-id"
client_secret = "your-client-secret"
authorization_url = "https://your-idp.com/oauth/authorize"
token_url = "https://your-idp.com/oauth/token"
introspection_url = "https://your-idp.com/oauth/introspect"  # Optional
userinfo_url = "https://your-idp.com/userinfo"               # Optional
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["openid", "profile"]
user_id_claim = "sub"
```

**Required endpoints:**
- `authorization_url` - Where users authenticate
- `token_url` - Token exchange endpoint

**Optional endpoints (at least one needed for token validation):**
- `introspection_url` - RFC 7662 token introspection
- `userinfo_url` - OpenID Connect userinfo

### Token Validation

MCP Guard validates tokens using:

1. **Introspection** (preferred) - Checks token at IdP
2. **UserInfo** (fallback) - Gets user info with token

**Token Caching:**
- Valid tokens are cached for 5 minutes
- Cache uses LRU eviction (max 500 entries)
- Reduces load on OAuth provider

### Troubleshooting

**"Redirect URI mismatch":**

1. Verify `redirect_uri` matches exactly what's registered with provider
2. Check protocol (http vs https)
3. Check trailing slashes

**"Invalid client credentials":**

1. Verify `client_id` and `client_secret`
2. Check for whitespace or newlines

**"Token introspection failed":**

1. Verify `introspection_url` is correct
2. Check client credentials have introspection permission
3. Try `userinfo_url` as fallback

**"State mismatch":**

1. Clear browser cookies and try again
2. Check for multiple OAuth flows in progress

---

## mTLS Authentication

### Overview

Mutual TLS (mTLS) authenticates clients using X.509 certificates. It's ideal for:

- Zero-trust architectures
- Service mesh integration
- High-security environments

### Architecture

```
Client → nginx (TLS) → MCP Guard → Upstream
         ↓ validates cert
         ↓ extracts identity
         → forwards headers
```

MCP Guard doesn't terminate TLS directly. Instead, a reverse proxy:
1. Validates client certificates against a CA
2. Extracts certificate info into headers
3. Forwards headers to MCP Guard

### Required Headers

| Header | Description | Example |
|--------|-------------|---------|
| `X-Client-Cert-CN` | Common Name | `my-service` |
| `X-Client-Cert-SAN-DNS` | SAN DNS entries (comma-separated) | `svc.local,api.local` |
| `X-Client-Cert-SAN-Email` | SAN Email entries | `service@example.com` |
| `X-Client-Cert-Verified` | Verification status | `SUCCESS` or `FAILED` |

### Identity Source Options

| Source | Header | Use Case |
|--------|--------|----------|
| `cn` | `X-Client-Cert-CN` | Most common, simple names |
| `san_dns` | `X-Client-Cert-SAN-DNS` | DNS-based identities |
| `san_email` | `X-Client-Cert-SAN-Email` | Email-based identities |

### Configuration

```toml
[auth.mtls]
enabled = true
identity_source = "cn"
allowed_tools = ["read_file", "write_file"]
rate_limit = 1000
trusted_proxy_ips = ["10.0.0.0/8", "172.16.0.0/12"]  # REQUIRED!
```

### trusted_proxy_ips (CRITICAL)

**You MUST configure `trusted_proxy_ips`** when enabling mTLS. Without it, attackers can spoof certificate headers from any IP.

**Accepted formats:**

```toml
trusted_proxy_ips = [
  "10.0.0.1",        # Single IPv4
  "::1",             # Single IPv6
  "10.0.0.0/8",      # IPv4 CIDR
  "fd00::/8"         # IPv6 CIDR
]
```

**Behavior:**

- Empty list → mTLS header auth disabled (prevents spoofing)
- Client IP not in list → mTLS headers ignored
- Client IP matches → mTLS headers accepted

### nginx Configuration

**1. Configure client certificate validation:**

```nginx
server {
    listen 443 ssl;
    server_name mcp.example.com;

    # Server certificate
    ssl_certificate /etc/nginx/ssl/server.crt;
    ssl_certificate_key /etc/nginx/ssl/server.key;

    # Client certificate CA
    ssl_client_certificate /etc/nginx/ssl/client-ca.crt;
    ssl_verify_client on;
    ssl_verify_depth 2;

    location / {
        # Forward certificate info
        proxy_set_header X-Client-Cert-CN $ssl_client_s_dn_cn;
        proxy_set_header X-Client-Cert-SAN-DNS $ssl_client_s_dn_san_dns_0;
        proxy_set_header X-Client-Cert-SAN-Email $ssl_client_s_dn_san_email_0;
        proxy_set_header X-Client-Cert-Verified $ssl_client_verify;

        proxy_pass http://mcp-guard:3000;
    }
}
```

**2. Configure MCP Guard:**

```toml
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.1"]  # nginx server IP
```

### HAProxy Configuration

```haproxy
frontend https
    bind *:443 ssl crt /etc/haproxy/server.pem ca-file /etc/haproxy/client-ca.crt verify required

    http-request set-header X-Client-Cert-CN %[ssl_c_s_dn(cn)]
    http-request set-header X-Client-Cert-Verified %[ssl_c_verify]

    default_backend mcp_guard

backend mcp_guard
    server mcp1 mcp-guard:3000
```

### Envoy Configuration

```yaml
static_resources:
  listeners:
  - name: listener_0
    address:
      socket_address:
        address: 0.0.0.0
        port_value: 443
    filter_chains:
    - filters:
      - name: envoy.filters.network.http_connection_manager
        typed_config:
          "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
          route_config:
            virtual_hosts:
            - name: local_service
              domains: ["*"]
              routes:
              - match: { prefix: "/" }
                route: { cluster: mcp_guard }
                request_headers_to_add:
                - header:
                    key: X-Client-Cert-CN
                    value: "%DOWNSTREAM_PEER_SUBJECT%"
      transport_socket:
        name: envoy.transport_sockets.tls
        typed_config:
          "@type": type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext
          require_client_certificate: true
          common_tls_context:
            validation_context:
              trusted_ca:
                filename: /etc/envoy/client-ca.crt
```

### Troubleshooting

**"Missing mTLS headers":**

1. Verify reverse proxy is forwarding headers
2. Check `ssl_verify_client` is `on` in nginx
3. Test: `curl -v --cert client.crt --key client.key https://...`

**"Client IP not trusted":**

1. Check nginx server IP is in `trusted_proxy_ips`
2. Verify you're using the correct IP (container IP vs host IP)

**"Certificate verification failed":**

1. Check client certificate is signed by the configured CA
2. Verify certificate is not expired
3. Check certificate chain is complete

**mTLS headers ignored:**

1. Ensure `trusted_proxy_ips` is not empty
2. Check client IP matches a trusted range
3. Enable verbose logging: `mcp-guard -v run`

---

## Combining Multiple Providers

### MultiProvider Behavior

When multiple providers are configured, MCP Guard creates a `MultiProvider` that:

1. Tries each provider in order
2. Returns on first successful authentication
3. Returns the most informative error if all fail

### Priority Order

1. **mTLS** (checked first if enabled and headers present)
2. **Bearer Token** providers (tried in order):
   - API Key
   - JWT
   - OAuth

### Configuration Example

```toml
# API keys for automated services
[[auth.api_keys]]
id = "ci-pipeline"
key_hash = "..."
allowed_tools = ["read_file"]

[[auth.api_keys]]
id = "admin-service"
key_hash = "..."
rate_limit = 1000

# JWT for external users (primary)
[auth.jwt]
mode = "jwks"
jwks_url = "https://auth.example.com/.well-known/jwks.json"
issuer = "https://auth.example.com/"
audience = "mcp-guard"

[auth.jwt.scope_tool_mapping]
"read" = ["read_file", "list_directory"]
"write" = ["write_file"]

# OAuth for web users
[auth.oauth]
provider = "github"
client_id = "..."
client_secret = "..."
redirect_uri = "https://mcp.example.com/oauth/callback"

# mTLS for internal services
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.0/8"]
rate_limit = 5000
```

### Use Cases

**API keys for services + OAuth for users:**

```toml
# Services use API keys
[[auth.api_keys]]
id = "backend-service"
key_hash = "..."

# Users authenticate via OAuth
[auth.oauth]
provider = "google"
client_id = "..."
client_secret = "..."
```

**JWT primary + API key fallback:**

```toml
# Primary: JWT from IdP
[auth.jwt]
mode = "jwks"
jwks_url = "https://idp.example.com/.well-known/jwks.json"
issuer = "https://idp.example.com/"
audience = "mcp-guard"

# Fallback: API keys for legacy systems
[[auth.api_keys]]
id = "legacy-system"
key_hash = "..."
```

**mTLS for internal + OAuth for external:**

```toml
# Internal services use mTLS
[auth.mtls]
enabled = true
identity_source = "cn"
trusted_proxy_ips = ["10.0.0.0/8"]

# External users use OAuth
[auth.oauth]
provider = "github"
client_id = "..."
client_secret = "..."
```

### Error Handling

When all providers fail, the error returned follows this priority:

1. Specific errors (TokenExpired, InvalidJwt) preferred over generic
2. Generic errors (MissingCredentials, InvalidApiKey) lower priority

Example: If JWT fails with "token expired" but API key fails with "invalid key", the client sees "token expired" as it's more informative.

---

## See Also

- [Quick Start Guide](quickstart.md) - Get started in 5 minutes
- [Configuration Reference](configuration.md) - Complete configuration options
- [CLI Reference](cli.md) - Command-line tools
- [Security Model](SECURITY.md) - Security architecture details
