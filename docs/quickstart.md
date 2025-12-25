# Quick Start Guide

Get from zero to a secured MCP server in under 10 minutes.

## What is MCP Guard?

MCP Guard is a lightweight security gateway that wraps any Model Context Protocol (MCP) server with production-grade protection. It provides authentication, authorization, rate limiting, and observability without requiring Docker, Kubernetes, or a DevOps team.

For a complete feature list, see the [README](../README.md).

## Prerequisites

- **Rust toolchain** (1.70+) for building from source, OR a prebuilt binary
- **An MCP server** to protect (e.g., the filesystem server from `@modelcontextprotocol/server-filesystem`)

> **Note:** First-time setup may take longer if you need to install Rust or if npm packages need to download. Subsequent runs are much faster.

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
mcp-guard keygen --user-id my-agent
```

**Output:**

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
- The **key_hash** goes in your config file (this is safe to commit)

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

## Step 5: Start the Gateway

Start MCP Guard:

```bash
mcp-guard run
```

**Output:**

```
2024-12-15T10:00:00Z  INFO mcp-guard: Starting server on 127.0.0.1:3000
2024-12-15T10:00:00Z  INFO mcp-guard: Authentication: API Key
2024-12-15T10:00:00Z  INFO mcp-guard: Transport: stdio (npx)
```

Verify it's running:

```bash
curl http://localhost:3000/health
```

**Response:**

```json
{"status": "healthy", "version": "0.1.0", "uptime_secs": 5}
```

## Step 6: Test Authentication

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
