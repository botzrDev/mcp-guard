# MCP-Guard Pro

**Professional tier security gateway for Model Context Protocol (MCP) servers**

[![License: Commercial](https://img.shields.io/badge/license-Commercial-red.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-mcp--guard.io-blue.svg)](https://docs.mcp-guard.io)

---

## Features Included

MCP-Guard Pro extends the free tier with enterprise-grade authentication and transport options:

### Authentication

- ✅ **OAuth 2.1 + PKCE** - Industry-standard OAuth with Proof Key for Code Exchange
  - Support for GitHub, Google, Okta, and custom providers
  - Token introspection and UserInfo validation
  - Automatic token caching to reduce API calls
  - Scope-to-tool mapping for fine-grained permissions

- ✅ **JWT JWKS Mode** - Auto-refreshing JWKS for RS256/ES256 tokens
  - Automatic key rotation support
  - Background JWKS refresh (1hr default TTL)
  - Compatible with Auth0, Keycloak, Okta, and other identity providers

### Transport

- ✅ **HTTP Transport** - Connect to upstream MCP servers over HTTP
  - POST JSON-RPC to any HTTP endpoint
  - Automatic retries with exponential backoff
  - Connection pooling and reuse

- ✅ **SSE Transport** - Server-Sent Events for streaming responses
  - Real-time streaming from upstream servers
  - Automatic reconnection on disconnect
  - Low latency for interactive applications

### Rate Limiting

- ✅ **Per-Identity Rate Limiting** - Rate limits tied to authenticated users
  - Custom rate limits per user/API key
  - Token bucket algorithm with configurable burst
  - Graceful degradation with `Retry-After` headers

---

## Pricing

**$12/month** - Includes all Pro features listed above

[Get your Pro license →](https://mcp-guard.io/pricing)

---

## Installation

### Prerequisites

- Valid Pro license key (starts with `pro_`)
- Rust 1.75+ (if building from source)

### Option 1: Pre-built Binary (Recommended)

```bash
# Download Pro binary
curl -fsSL https://mcp-guard.io/install-pro.sh | bash

# Verify installation
mcp-guard version
# Should show: Tier: Pro
```

### Option 2: Build from Source

```bash
# Clone repository
git clone https://github.com/botzrdev/mcp-guard.git
cd mcp-guard

# Build with Pro features
cargo build --release --features pro

# Binary will be at: target/release/mcp-guard
```

---

## Quick Start

### 1. Set Your License Key

```bash
export MCP_GUARD_LICENSE_KEY="pro_xxx..."
```

**Tip:** Add to your `~/.bashrc` or `~/.zshrc` to persist across sessions.

### 2. Generate Configuration

```bash
mcp-guard init
```

This creates `mcp-guard.toml` with example configurations.

### 3. Configure OAuth (Example: GitHub)

Edit `mcp-guard.toml`:

```toml
[auth.oauth]
provider = "github"
client_id = "your_github_client_id"
client_secret = "your_github_client_secret"  # Or use env var
redirect_uri = "http://localhost:3000/oauth/callback"
scopes = ["read:user"]

# Map OAuth scopes to MCP tools
[auth.oauth.scope_tool_mapping]
"read:user" = ["filesystem/read", "filesystem/list"]
"write:user" = ["filesystem/write", "filesystem/delete"]
```

### 4. Start the Server

```bash
mcp-guard run
```

### 5. Test OAuth Flow

```bash
# Initiate OAuth flow
curl http://localhost:3000/oauth/authorize

# Follow the URL, authenticate, and get your access token
# Then use the token for MCP requests
curl -H "Authorization: Bearer <access-token>" http://localhost:3000/health
```

---

## Configuration Examples

### JWT with JWKS (Auth0)

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://your-tenant.auth0.com/.well-known/jwks.json"
issuer = "https://your-tenant.auth0.com/"
audience = "https://your-api.example.com"

# Map JWT scopes to tools
[auth.jwt.scope_tool_mapping]
"read:files" = ["filesystem/read", "filesystem/list"]
"write:files" = ["filesystem/write", "filesystem/delete"]
```

### HTTP Transport to Upstream

```toml
[upstream]
transport = "http"
url = "http://your-mcp-server:8080/mcp"
```

### SSE Transport for Streaming

```toml
[upstream]
transport = "sse"
url = "http://your-mcp-server:8080/mcp/stream"
```

### Per-Identity Rate Limiting

```toml
# Global default
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 20

# Override for specific API key
[[auth.api_keys]]
key_hash = "sha256_hash_here"
user_id = "premium_user"
rate_limit = 500  # Higher limit for premium users
```

---

## Support

### Documentation

- [Full Documentation](https://docs.mcp-guard.io)
- [OAuth Setup Guide](https://docs.mcp-guard.io/auth/oauth)
- [JWT Configuration](https://docs.mcp-guard.io/auth/jwt)
- [Troubleshooting](https://docs.mcp-guard.io/troubleshooting)

### Email Support

**Response Time:** 48 hours for Pro tier

Contact: [austin@botzr.dev](mailto:austin@botzr.dev)

Include:
- License key (first 10 characters)
- MCP-Guard version (`mcp-guard version`)
- Configuration file (redacted secrets)
- Error logs

### License Management

- Renew license: [mcp-guard.io/account](https://mcp-guard.io/account)
- Upgrade to Enterprise: [mcp-guard.io/pricing](https://mcp-guard.io/pricing)
- Billing questions: [austin@botzr.dev](mailto:austin@botzr.dev)

---

## Troubleshooting

### License Validation Fails

```
Error: Pro license validation failed: Invalid signature
```

**Solutions:**
1. Verify license key is correct: `echo $MCP_GUARD_LICENSE_KEY`
2. Check license hasn't expired
3. Ensure you're using Pro binary, not free tier
4. Contact support if issue persists

### OAuth Redirect Not Working

```
Error: OAuth error: Invalid redirect_uri
```

**Solutions:**
1. Ensure `redirect_uri` in config matches OAuth provider settings
2. Check that MCP-Guard is accessible at the redirect URL
3. Verify firewall/network allows incoming connections

### JWKS Refresh Failing

```
Warning: JWKS refresh failed: Connection timeout
```

**Solutions:**
1. Check network connectivity to JWKS URL
2. Verify `jwks_url` is correct and accessible
3. Check firewall allows outbound HTTPS
4. Contact your identity provider if JWKS endpoint is down

---

## Upgrade to Enterprise

Need advanced features like mTLS, multi-server routing, or SIEM integration?

**Enterprise tier includes:**
- Everything in Pro
- mTLS client certificate authentication
- Multi-server routing
- SIEM audit log shipping (Splunk, Datadog, etc.)
- OpenTelemetry distributed tracing
- Priority support (4hr SLA for critical issues)

[Upgrade to Enterprise →](https://mcp-guard.io/pricing)

---

## License

This software is licensed under a commercial license and requires a valid Pro license key for use.

See [LICENSE](LICENSE) for full terms.

**Copyright © 2025 Austin Green / Botzr**
