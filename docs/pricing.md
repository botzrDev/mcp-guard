# mcp-guard Pricing & Tiers

mcp-guard offers three tiers to match your needs, from hobbyist projects to enterprise deployments.

## Quick Comparison

| | Free | Pro | Enterprise |
|---|:---:|:---:|:---:|
| **Price** | $0 | $12/mo | $29 + $8/seat/mo |
| **License** | AGPL-3.0 | Commercial | Commercial |
| **Support** | Community | Email (48h) | Priority (4h) |

## Free Tier

**Perfect for**: Individual developers, hobbyists, and open-source projects.

### What's Included

**Authentication**
- API Key authentication
- JWT HS256 (simple mode with shared secret)

**Transport**
- Stdio transport (single upstream MCP server)

**Security**
- Per-tool authorization (allowed_tools per API key)
- Tools/list filtering
- Global rate limiting

**Observability**
- Prometheus metrics endpoint
- Health check endpoints (/health, /live, /ready)
- Audit logging to file or console

### Installation

```bash
cargo install mcp-guard
```

---

## Pro Tier ($12/month)

**Perfect for**: Small teams and production applications needing OAuth or HTTP transports.

### Everything in Free, Plus

**Authentication**
- OAuth 2.1 with PKCE (GitHub, Google, Okta, custom providers)
- JWT JWKS mode (RS256/ES256 with auto-refresh)

**Transport**
- HTTP transport (connect to upstream via HTTP POST)
- SSE transport (Server-Sent Events for streaming)

**Security**
- Per-identity rate limiting (each user gets their own quota)

### Installation

```bash
curl -fsSL https://mcp-guard.io/install-pro.sh | bash
export MCP_GUARD_LICENSE_KEY="pro_xxx..."
```

### License Validation

Pro licenses are validated cryptographically (offline) or via our licensing server. The license key is set via the `MCP_GUARD_LICENSE_KEY` environment variable.

---

## Enterprise Tier ($29+/user/month)

**Perfect for**: Large teams with compliance requirements, multi-server deployments, and SIEM integration needs.

### Everything in Pro, Plus

**Authentication**
- mTLS client certificate authentication
- Certificate-based identity extraction (CN, SAN)

**Transport**
- Multi-server routing with path prefixes
- Route requests to different upstreams by path

**Observability**
- OpenTelemetry distributed tracing (Jaeger, Tempo, etc.)
- Audit log shipping to SIEM (Splunk, Datadog, etc.)
- Configurable batch size and flush intervals

**Administration**
- Guard tools API for runtime management
- Key management via MCP tools (guard/keys/*)
- Audit log queries via MCP tools (guard/audit/*)
- Configuration validation via MCP tools (guard/config/*)

### Installation

```bash
curl -fsSL https://mcp-guard.io/install-enterprise.sh | bash
export MCP_GUARD_LICENSE_KEY="ent_xxx..."
```

### License Validation

Enterprise licenses are validated online with 30-day offline caching. This ensures compliance while supporting air-gapped deployments.

---

## Feature Matrix

| Feature | Free | Pro | Enterprise |
|---------|:----:|:---:|:----------:|
| **Authentication** | | | |
| API Key | ✅ | ✅ | ✅ |
| JWT HS256 | ✅ | ✅ | ✅ |
| JWT JWKS (RS256/ES256) | | ✅ | ✅ |
| OAuth 2.1 + PKCE | | ✅ | ✅ |
| mTLS Client Certs | | | ✅ |
| **Transport** | | | |
| Stdio | ✅ | ✅ | ✅ |
| HTTP | | ✅ | ✅ |
| SSE | | ✅ | ✅ |
| Multi-Server Routing | | | ✅ |
| **Rate Limiting** | | | |
| Global | ✅ | ✅ | ✅ |
| Per-Identity | | ✅ | ✅ |
| Per-Tool | | | ✅ |
| **Audit** | | | |
| File/Console Logging | ✅ | ✅ | ✅ |
| SIEM Shipping | | | ✅ |
| **Observability** | | | |
| Prometheus Metrics | ✅ | ✅ | ✅ |
| Health Endpoints | ✅ | ✅ | ✅ |
| OpenTelemetry Tracing | | | ✅ |
| **Guard Tools API** | | | |
| guard/health | ✅ | ✅ | ✅ |
| guard/metrics | ✅ | ✅ | ✅ |
| guard/version | ✅ | ✅ | ✅ |
| guard/keys/* | | | ✅ |
| guard/audit/* | | | ✅ |
| guard/config/* | | | ✅ |

---

## Upgrade Prompts

When you try to use a feature that requires an upgrade, mcp-guard provides a helpful error message:

```
Error: HTTP transport requires a Pro license.

The free tier supports stdio transport only.

Upgrade to Pro for $12/month:
→ https://mcp-guard.io/pricing

Or use stdio transport:
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@your/mcp-server"]
```

---

## FAQ

### Can I use the Free tier commercially?

Yes, but the Free tier is licensed under AGPL-3.0. This means if you distribute the software or run it as a service, you must make your source code available. For proprietary use, upgrade to Pro or Enterprise.

### How does license validation work?

- **Pro**: Cryptographic offline validation (no network required)
- **Enterprise**: Online validation with 30-day offline cache

### What happens if my license expires?

The gateway will continue running but will refuse to start new instances until the license is renewed. Existing connections are not disrupted.

### Can I try Pro/Enterprise features before purchasing?

Contact us at sales@mcp-guard.io for a 14-day trial license.

---

## Get Started

→ [Purchase a license at mcp-guard.io/pricing](https://mcp-guard.io/pricing)

→ [Quick Start Guide](quickstart.md)

→ [Contact Sales](mailto:sales@mcp-guard.io)
