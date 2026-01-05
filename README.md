<div align="center">
  <img src="https://mcpg.botzr.com/logo.svg" alt="mcp-guard" width="64" height="64">
  <h1>mcp-guard</h1>
  <p><strong>MCP security without the infrastructure tax.</strong></p>
  <p>One binary. One config file. Production-ready in 5 minutes.</p>

  <p>
    <a href="https://crates.io/crates/mcp-guard"><img src="https://img.shields.io/crates/v/mcp-guard.svg" alt="Crates.io"></a>
    <a href="https://github.com/mcp-guard/mcp-guard/actions/workflows/ci.yml"><img src="https://github.com/mcp-guard/mcp-guard/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License"></a>
    <a href="https://github.com/mcp-guard/mcp-guard/releases"><img src="https://img.shields.io/github/v/release/mcp-guard/mcp-guard" alt="Release"></a>
  </p>

  <p>
    <a href="#quick-start">Quick Start</a> &bull;
    <a href="#features">Features</a> &bull;
    <a href="#pricing">Pricing</a> &bull;
    <a href="https://mcpg.botzr.com/docs">Documentation</a>
  </p>
</div>

<br>

## The Problem

Model Context Protocol (MCP) servers are powerful. Most are deployed with **zero authentication**.

If your AI agent can access it, so can anyone else.

## The Solution

`mcp-guard` is a security gateway that wraps any MCP server with production-grade protection.

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   Client    │─────▶│  mcp-guard  │─────▶│ MCP Server  │
│  (Claude)   │      │   Gateway   │      │  (yours)    │
└─────────────┘      └─────────────┘      └─────────────┘
                            │
                     Authentication
                     Authorization
                     Rate Limiting
                     Audit Logging
```

<br>

## Quick Start

Three commands. That's it.

```bash
# 1. Install
curl -fsSL https://mcpg.botzr.com/install.sh | sh

# 2. Configure
mcp-guard init

# 3. Run
mcp-guard run
```

Test that it works:

```bash
curl -H "Authorization: Bearer mcp_YOUR_KEY" http://localhost:3000/health
```

<details>
<summary><strong>Alternative installation methods</strong></summary>

```bash
# From crates.io (requires Rust)
cargo install mcp-guard

# Homebrew (macOS/Linux)
brew install mcp-guard/tap/mcp-guard

# Download binary directly
curl -fsSL https://github.com/mcp-guard/mcp-guard/releases/latest/download/mcp-guard-$(uname -s)-$(uname -m).tar.gz | tar -xz
```

</details>

<br>

## Features

### Authentication

| Method | Free | Pro | Enterprise |
|--------|:----:|:---:|:----------:|
| API Keys | ✓ | ✓ | ✓ |
| JWT (HS256) | ✓ | ✓ | ✓ |
| JWT (JWKS/RS256/ES256) | | ✓ | ✓ |
| OAuth 2.1 + PKCE | | ✓ | ✓ |
| mTLS Client Certificates | | | ✓ |

### Transport

| Type | Free | Pro | Enterprise |
|------|:----:|:---:|:----------:|
| Stdio | ✓ | ✓ | ✓ |
| HTTP | | ✓ | ✓ |
| SSE | | ✓ | ✓ |
| Multi-Server Routing | | | ✓ |

### Security & Observability

| Feature | Free | Pro | Enterprise |
|---------|:----:|:---:|:----------:|
| Per-Tool Authorization | ✓ | ✓ | ✓ |
| Tools Filtering | ✓ | ✓ | ✓ |
| Global Rate Limiting | ✓ | ✓ | ✓ |
| Per-Identity Rate Limiting | | ✓ | ✓ |
| Prometheus Metrics | ✓ | ✓ | ✓ |
| Health Endpoints | ✓ | ✓ | ✓ |
| Audit Logs (file/console) | ✓ | ✓ | ✓ |
| OpenTelemetry Tracing | | | ✓ |
| SIEM Log Shipping | | | ✓ |

<br>

## Configuration

`mcp-guard init` generates a config file with sensible defaults:

```toml
# mcp-guard.toml

[server]
listen = "0.0.0.0:3000"

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "./"]

[rate_limit]
requests_per_second = 10
burst_size = 20

[[auth.api_keys]]
key_hash = "sha256:..."
user_id = "developer-1"
allowed_tools = ["read_file", "list_directory"]
```

<details>
<summary><strong>JWT configuration</strong></summary>

```toml
[auth.jwt]
mode = "simple"
secret = "your-secret-key"
issuer = "https://your-issuer.com"
audience = "mcp-guard"

[auth.jwt.scope_mapping]
"read" = ["read_file", "list_directory"]
"write" = ["write_file", "create_directory"]
"admin" = ["*"]
```

</details>

<details>
<summary><strong>OAuth 2.1 configuration</strong></summary>

```toml
[auth.oauth]
provider = "github"  # or "google", "okta", "custom"
client_id = "your-client-id"
client_secret = "your-client-secret"

[auth.oauth.scope_mapping]
"repo" = ["read_file", "write_file"]
"admin:org" = ["*"]
```

</details>

<details>
<summary><strong>Multi-server routing (Enterprise)</strong></summary>

```toml
[[servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "./"]

[[servers]]
name = "database"
path_prefix = "/db"
transport = "http"
url = "http://localhost:8080"
```

</details>

<br>

## Performance

| Metric | Target | Actual |
|--------|--------|--------|
| Latency overhead | <2ms p99 | **<1ms** |
| Binary size | <15MB | **<10MB** |
| Memory usage | <50MB | **~30MB** |
| Throughput | >5,000 RPS | **>10,000 RPS** |

Your agents stay fast. Your infrastructure stays simple.

<br>

## Pricing

| Tier | Price | Best For |
|------|-------|----------|
| **Free** | $0 | Open source, side projects |
| **Pro** | $12/mo | Small teams, production apps |
| **Enterprise** | $29 + $8/seat | Compliance, multi-server |

<sub>Founder pricing: 40% off forever for early adopters. [Lock in your discount →](https://mcpg.botzr.com/pricing)</sub>

<br>

## CLI Reference

```
mcp-guard <command>

Commands:
  init             Generate config file with demo API key
  validate         Check config file for errors
  keygen           Generate a new API key
  run              Start the gateway
  check-upstream   Test upstream server connectivity
  version          Show version and build info

Options:
  -c, --config     Config file path (default: mcp-guard.toml)
  -h, --help       Show help
```

<br>

## Documentation

| Topic | Guide |
|-------|-------|
| Getting Started | [Quick Start](https://mcpg.botzr.com/docs/quickstart) |
| Authentication | [Auth Guide](https://mcpg.botzr.com/docs/auth) |
| Transports | [Stdio/HTTP/SSE](https://mcpg.botzr.com/docs/transports) |
| Rate Limiting | [Rate Limits](https://mcpg.botzr.com/docs/rate-limiting) |
| Observability | [Metrics & Tracing](https://mcpg.botzr.com/docs/observability) |
| Deployment | [Production Guide](https://mcpg.botzr.com/docs/deployment) |
| API Reference | [HTTP API](https://mcpg.botzr.com/docs/api) |

<br>

## Security

Security vulnerabilities should be reported via [support@botzr.com](mailto:support@botzr.com).

See [SECURITY.md](SECURITY.md) for our security policy.

<br>

## Contributing

We welcome contributions. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Clone and build
git clone https://github.com/mcp-guard/mcp-guard
cd mcp-guard
cargo build

# Run tests
cargo test

# Run lints
cargo clippy -- -D warnings
```

<br>

## License

AGPL-3.0. See [LICENSE](LICENSE).

Commercial licenses available for Pro and Enterprise tiers.

---

<div align="center">
  <sub>Built by <a href="https://botzr.dev">botzr</a></sub>
</div>
