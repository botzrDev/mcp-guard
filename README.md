<div align="center">
  <h1>mcp-guard</h1>
  <h3>Secure your MCP servers in 5 minutes.</h3>
  <p>No Docker. No Kubernetes. No DevOps team.</p>

  <p>
    <a href="https://crates.io/crates/mcp-guard"><img src="https://img.shields.io/crates/v/mcp-guard.svg" alt="Crates.io"></a>
    <a href="https://github.com/botzrdev/mcp-guard/actions/workflows/ci.yml"><img src="https://github.com/botzrdev/mcp-guard/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License: AGPL-3.0"></a>
    <a href="https://github.com/botzrdev/mcp-guard/releases"><img src="https://img.shields.io/github/v/release/botzrdev/mcp-guard" alt="GitHub Release"></a>
  </p>

  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#configuration">Configuration</a> •
  <a href="docs/quickstart.md">Documentation</a>
</div>

---

## Install

```bash
# From crates.io (requires Rust)
cargo install mcp-guard

# Or download prebuilt binary
curl -fsSL https://github.com/botzrdev/mcp-guard/releases/latest/download/mcp-guard-x86_64-linux.tar.gz | tar -xz

# Or use the install script
curl -fsSL https://raw.githubusercontent.com/botzrdev/mcp-guard/main/install.sh | bash
```

---

## Quick Start

```bash
# Generate config with demo API key
mcp-guard init

# Start the gateway
mcp-guard run

# Test it works (use the demo key printed by init)
curl -H "Authorization: Bearer mcp_YOUR_DEMO_KEY" http://localhost:3000/health
```

For production setup, see the [Quick Start Guide](docs/quickstart.md).

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

## Getting Started

The fastest way to get started is to follow our **[Quick Start Guide](docs/quickstart.md)**, which covers:
- Installation
- Generating configuration
- Setting up your first API key
- Testing with a filesystem MCP server

For advanced setup and production deployment, see the **[Deployment Guide](docs/deployment.md)**.

## Documentation

Explore our comprehensive documentation for detailed guides on every feature:

| Topic | Guide |
|-------|-------|
| **Setup** | [Quick Start](docs/quickstart.md), [Installation & Deployment](docs/deployment.md) |
| **Core Features** | [Authentication](docs/authentication.md), [Authorization](docs/authorization.md), [Rate Limiting](docs/rate-limiting.md) |
| **Connectivity** | [Transports (Stdio/HTTP/SSE)](docs/transports.md), [Multi-Server Routing](docs/multi-server.md) |
| **Integrations** | [Auth0](docs/integrations/auth0.md), [GitHub OAuth](docs/integrations/github-oauth.md), [Splunk](docs/integrations/splunk.md), [Jaeger](docs/integrations/jaeger.md) |
| **Reference** | [Configuration Details](docs/configuration.md), [CLI Reference](docs/cli.md), [HTTP API](docs/api/http.md) |
| **Developer** | [Architecture](docs/dev/architecture.md), [Contributing](docs/dev/contributing.md), [Testing](docs/dev/testing.md) |

---

## Security

Security is our top priority. For details on how we handle credentials, validated inputs, and protect against common attacks like SSRF, please see our **[Security Policy](SECURITY.md)**.

## Troubleshooting

Having trouble? Check our **[Troubleshooting Guide](docs/troubleshooting.md)** for common issues, debugging tips, and diagnostic commands.

## License

AGPL-3.0
