# Quick Start Guide

Get from zero to a secured MCP server in under 10 minutes.

## What is MCP Guard?

MCP Guard is a lightweight security gateway that wraps any Model Context Protocol (MCP) server with production-grade protection. It provides authentication, authorization, rate limiting, and observability without requiring Docker, Kubernetes, or a DevOps team.

For a complete feature list, see the [README](../README.md).

## Prerequisites

### Required

- **mcp-guard binary** (see Installation below)

### For the Demo Setup

- **Node.js & npm** - The default config uses `npx` to run the demo MCP filesystem server

> **Don't have Node.js?** Either:
> - Install Node.js from [nodejs.org](https://nodejs.org)
> - Use your own MCP server (edit the `[upstream]` section in config)
> - Use HTTP transport with an existing MCP server URL

### Installation Speed

| Method | Time | Best For |
|--------|------|----------|
| Prebuilt binary | ~30 sec | Fastest setup |
| cargo install | 2-3 min | Rust developers |
| Build from source | 3-5 min | Contributors |

> **Tip**: Download the [prebuilt binary](https://github.com/botzrdev/mcp-guard/releases) for the fastest experience.

## Installation

Choose one of three installation methods:

### Option 1: Install from crates.io

```bash
cargo install mcp-guard
```

### Option 2: Download Prebuilt Binary

Download the latest release for your platform from [GitHub Releases](https://github.com/botzrdev/mcp-guard/releases).

### Option 3: Build from Source

```bash
git clone https://github.com/botzrdev/mcp-guard
cd mcp-guard
cargo build --release
```

The binary will be at `./target/release/mcp-guard`.

### Verify Installation

```bash
mcp-guard version
```

You should see version information and enabled features.

---

## Step 1: Generate Configuration

Create a configuration file template:

```bash
mcp-guard init
```

This creates `mcp-guard.toml` in your current directory with example configurations for all features.

## Step 2: Generate an API Key

Generate an API key for your first client:

```bash
# Option A: Auto-add to config (recommended)
mcp-guard keygen --user-id my-agent --apply-to-config

# Option B: Manual (prints TOML to copy)
mcp-guard keygen --user-id my-agent
```

**Output (with --apply-to-config):**

```
✓ API key for 'my-agent' added to mcp-guard.toml

API Key (save this, shown only once):
  mcp_AbCdEf123456...

Next steps:
  mcp-guard validate
  mcp-guard run
```

**Output (without --apply-to-config):**

```
Generated API key for 'my-agent':

  API Key (save this, shown only once):
    mcp_AbCdEf123456...

  Add to your config file:

  [[auth.api_keys]]
  id = "my-agent"
  key_hash = "abc123def456..."
```

**Important:**
- The **API Key** goes to your client application (store it securely)
- The **key_hash** is stored in the config file (safe to commit)

## Step 3: (Optional) Add JWT Authentication

For services that already use JWT tokens (from Auth0, Keycloak, etc.), add JWT authentication alongside API keys.

Add to your `mcp-guard.toml`:

```toml
[auth.jwt]
mode = "simple"
secret = "your-256-bit-secret-key-here-minimum-32-characters"
issuer = "https://your-app.com"
audience = "mcp-guard"
```

**When to use each:**
- **API Keys**: Service-to-service communication, CLI tools, simple setups
- **JWT**: Integration with existing identity providers, user authentication

For production, consider [JWKS mode](authentication.md#jwks-mode-rs256es256) which uses public key verification from your identity provider.

## Step 4: Configure Upstream

Edit `mcp-guard.toml` to point to your MCP server. Here's a complete minimal configuration:

```toml
[server]
host = "127.0.0.1"
port = 3000

# Your MCP server
[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

# API key authentication
[[auth.api_keys]]
id = "my-agent"
key_hash = "YOUR_KEY_HASH_HERE"  # From Step 2
allowed_tools = []  # Empty = all tools allowed
rate_limit = 100    # Requests per second

# Rate limiting
[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

# Audit logging
[audit]
enabled = true
stdout = true
```

Validate your configuration:

```bash
mcp-guard validate
```

You should see: `Configuration is valid: mcp-guard.toml`

## Step 5: Verify Upstream Connectivity

Before starting the server, verify your MCP upstream is reachable:

```bash
mcp-guard check-upstream
```

**Expected output:**

```
Checking upstream connectivity...

Transport: stdio
Command:   npx
Args:      ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

Server: @modelcontextprotocol/server-filesystem v0.6.2
✓ Upstream is reachable and responding
```

**Common issues:**

| Error | Cause | Solution |
|-------|-------|----------|
| `command not found` | Node.js/npm not installed | Install Node.js (see [Prerequisites](#prerequisites)) |
| `timeout` | Slow connection or unresponsive server | Use `--timeout 30` for more time |
| `connection refused` | Wrong URL or server not running | Check upstream URL/command |

> **Tip**: This step catches configuration issues before the server starts, saving debugging time.

## Step 6: Start the Gateway

Start MCP Guard:

```bash
mcp-guard run
```

**Output:**

```
╭──────────────────────────────────────────────╮
│  MCP Guard v1.0.0                            │
╰──────────────────────────────────────────────╯

✓ Server:     http://127.0.0.1:3000
✓ Auth:       API Keys (1)
✓ Transport:  stdio → npx
✓ Rate Limit: 100 req/s, burst 50
✓ Audit:      Enabled

Ready for requests!

Test with:
  curl http://127.0.0.1:3000/health
```

Verify it's running:

```bash
curl http://localhost:3000/health
```

**Response:**

```json
{"status": "healthy", "version": "1.0.0", "uptime_secs": 5}
```

## Step 7: Test Authentication

### Unauthenticated Request (should fail)

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Response: 401 Unauthorized**

```json
{"error": "Unauthorized", "error_id": "abc123..."}
```

### Authenticated Request (should succeed)

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer mcp_YOUR_API_KEY_HERE" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Response: 200 OK**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {"name": "read_file", "description": "Read a file..."},
      {"name": "write_file", "description": "Write a file..."}
    ]
  }
}
```

### Check Rate Limit Headers

Successful responses include rate limit information:

```
x-ratelimit-limit: 100
x-ratelimit-remaining: 99
x-ratelimit-reset: 1702656789
```

---

## Next Steps

Now that you have a working gateway, explore these topics:

- **[Configuration Reference](configuration.md)** - Complete reference for all configuration options
- **[Authentication Guide](authentication.md)** - Deep dive into JWT, OAuth 2.1, and mTLS
- **[CLI Reference](cli.md)** - All CLI commands and options

### Common Next Tasks

**Add more API keys:**

```bash
mcp-guard keygen --user-id another-service --rate-limit 200
```

**Restrict tools for specific users:**

```toml
[[auth.api_keys]]
id = "read-only-client"
key_hash = "..."
allowed_tools = ["read_file", "list_directory"]
```

**Enable Prometheus metrics:**

```bash
curl http://localhost:3000/metrics
```

**Test upstream connectivity:**

```bash
mcp-guard check-upstream
```

---

## Troubleshooting

### "Config file not found"

Make sure `mcp-guard.toml` exists in your current directory, or specify the path:

```bash
mcp-guard run --config /path/to/config.toml
```

### "Invalid API key"

Verify your key:
1. Check you're using the full API key (starts with `mcp_`)
2. Verify the hash in config matches: `mcp-guard hash-key "mcp_YOUR_KEY"`

### "Upstream communication error"

Test upstream connectivity:

```bash
mcp-guard check-upstream --timeout 30
```

For stdio transports, verify the command is correct:

```bash
npx -y @modelcontextprotocol/server-filesystem /tmp
```

### Enable Verbose Logging

For more details:

```bash
mcp-guard run -v
```
