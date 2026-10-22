# CLI Reference

Complete reference for all MCP Guard command-line commands and options.

## Overview

MCP Guard provides a command-line interface for configuration, key management, and server operation.

### Installation Verification

```bash
mcp-guard version
```

### Global Options

These options can be used with any command:

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--config` | `-c` | `mcp-guard.toml` | Path to configuration file |
| `--verbose` | `-v` | false | Enable verbose logging output |
| `--help` | `-h` | - | Show help for command |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (invalid config, auth failure, etc.) |

---

## Commands

### init

Generate a new configuration file template.

**Usage:**

```bash
mcp-guard init [OPTIONS]
```

**Options:**

| Option | Default | Description |
|--------|---------|-------------|
| `--format` | `toml` | Output format: `toml` or `yaml` |
| `--force`, `-f` | false | Overwrite existing file |

**Examples:**

```bash
# Generate TOML config (default)
mcp-guard init

# Generate YAML config
mcp-guard init --format yaml

# Overwrite existing config
mcp-guard init --force
```

**Output:**

Creates `mcp-guard.toml` (or `.yaml`) in the current directory with commented examples for all configuration sections.

---

### validate

Parse and validate a configuration file without starting the server.

**Usage:**

```bash
mcp-guard validate [OPTIONS]
```

**Options:**

Uses global `--config` option to specify which file to validate.

**Validation Checks:**

- File exists and is readable
- Valid TOML/YAML syntax
- Required fields present
- Field values in valid ranges (port 1-65535, sample_rate 0.0-1.0)
- URL formats valid (HTTPS required for JWKS in production)

**Examples:**

```bash
# Validate default config
mcp-guard validate

# Validate custom config
mcp-guard validate --config production.toml

# Validate with verbose output
mcp-guard -v validate
```

**Output:**

```
Configuration is valid: mcp-guard.toml
```

Or on error:

```
Configuration error: invalid port value '0': must be between 1 and 65535
```

---

### keygen

Generate a new API key with its hash for use in configuration.

**Usage:**

```bash
mcp-guard keygen --user-id <ID> [OPTIONS]
```

**Required Arguments:**

| Argument | Description |
|----------|-------------|
| `--user-id` | Unique identifier for the user or service |

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--rate-limit` | u32 | Custom rate limit in requests per second |
| `--tools` | string | Comma-separated list of allowed tools |
| `--apply-to-config` | flag | Automatically add the key to the config file |

**Examples:**

```bash
# Basic key generation (prints TOML to copy)
mcp-guard keygen --user-id alice

# Auto-add key to config file (recommended)
mcp-guard keygen --user-id alice --apply-to-config

# With custom rate limit
mcp-guard keygen --user-id bob --rate-limit 500

# With tool restrictions
mcp-guard keygen --user-id readonly --tools "read_file,list_directory"

# Full example with auto-apply
mcp-guard keygen --user-id admin --rate-limit 1000 --tools "read_file,write_file" --apply-to-config
```

**Output (without --apply-to-config):**

```
Generated API key for 'alice':

  API Key (save this, shown only once):
    mcp_AbCdEf123456789XYZ...

  Add to your config file:

  [[auth.api_keys]]
  id = "alice"
  key_hash = "abc123def456..."
```

> **Note**: `rate_limit` and `allowed_tools` are shown only when specified via `--rate-limit` and `--tools` options.

**Output (with --apply-to-config):**

```
✓ API key for 'alice' added to mcp-guard.toml

API Key (save this, shown only once):
  mcp_AbCdEf123456789XYZ...

Next steps:
  mcp-guard validate
  mcp-guard run
```

**Key Format:**

- Prefix: `mcp_`
- Body: 32 bytes of cryptographically random data, base64url encoded
- Total length: ~47 characters

**Security Notes:**

- The API key is shown **only once** and cannot be recovered
- Only the SHA-256 hash is stored in configuration
- Store API keys securely (secrets manager, environment variables)
- Use separate keys for each client/service

---

### hash-key

Hash an existing API key for use in configuration.

**Usage:**

```bash
mcp-guard hash-key <KEY>
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `<KEY>` | The API key to hash |

**Examples:**

```bash
# Hash a key
mcp-guard hash-key "mcp_AbCdEf123456..."

# From environment variable
mcp-guard hash-key "$MY_API_KEY"
```

**Output:**

```
abc123def456789...
```

**Use Cases:**

- Migrating existing keys to MCP Guard
- Verifying a key matches a hash in config
- Sharing keys across multiple deployments

---

### run

Start the MCP Guard server.

**Usage:**

```bash
mcp-guard run [OPTIONS]
```

**Options:**

| Option | Type | Description |
|--------|------|-------------|
| `--host` | string | Override listen host from config |
| `--port` | u16 | Override listen port from config |

**Examples:**

```bash
# Start with config defaults
mcp-guard run

# Listen on all interfaces
mcp-guard run --host 0.0.0.0

# Custom port
mcp-guard run --port 8080

# Override both
mcp-guard run --host 0.0.0.0 --port 8080

# With custom config and verbose logging
mcp-guard -v --config production.toml run
```

**Startup Sequence:**

1. Load and validate configuration
2. Initialize authentication providers (API Key, JWT, OAuth, mTLS)
3. Initialize rate limiter
4. Initialize transport (stdio, HTTP, or SSE)
5. Start audit logger (with optional SIEM export)
6. Initialize Prometheus metrics
7. Initialize OpenTelemetry tracing (if configured)
8. Bind to host:port
9. Ready for requests

**Startup Output:**

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

**Available Endpoints:**

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health status with version and uptime |
| `/live` | GET | Kubernetes liveness probe |
| `/ready` | GET | Kubernetes readiness probe |
| `/metrics` | GET | Prometheus metrics |
| `/mcp` | POST | MCP JSON-RPC handler (auth required) |
| `/mcp/:server` | POST | Route to specific server (multi-server mode) |
| `/routes` | GET | List available routes (multi-server mode) |
| `/oauth/authorize` | GET | Start OAuth flow |
| `/oauth/callback` | GET | OAuth callback |

**Graceful Shutdown:**

Press `Ctrl+C` or send `SIGTERM` to stop the server gracefully:

1. Stop accepting new connections
2. Flush audit logs
3. Export pending traces
4. Close upstream transports
5. Exit

---

### version

Display version, build information, and available features by tier.

**Usage:**

```bash
mcp-guard version
```

**Output:**

```
mcp-guard 1.0.0

Build Information:
  Package:     mcp-guard
  Version:     1.0.0
  Tier:        Free
  Description: A lightweight, high-performance security gateway for MCP servers
  License:     AGPL-3.0
  Repository:  https://github.com/botzrdev/mcp-guard

Available Features:
  [Free]
    - API Key authentication
    - JWT HS256 (simple mode)
    - Stdio transport
    - Global rate limiting
    - File/console audit logging
    - Prometheus metrics
  [Pro] (upgrade at https://mcpg.botzr.com/pricing)
    - OAuth 2.1 + PKCE authentication
    - JWT JWKS mode (RS256/ES256)
    - HTTP/SSE transports
  [Enterprise] (upgrade at https://mcpg.botzr.com/pricing)
    - mTLS client certificate authentication
    - Multi-server routing
    - OpenTelemetry tracing
    - SIEM audit log shipping
```

**Use Cases:**

- CI/CD pipelines (verify correct version deployed)
- Troubleshooting (confirm feature availability and tier)
- Support requests (include version and tier info)

---

### check-upstream

Test upstream MCP server connectivity without starting the full gateway.

**Usage:**

```bash
mcp-guard check-upstream [OPTIONS]
```

**Options:**

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--timeout` | `-t` | 10 | Timeout in seconds |

**Examples:**

```bash
# Check with default timeout
mcp-guard check-upstream

# Custom timeout
mcp-guard check-upstream --timeout 30

# With custom config
mcp-guard --config production.toml check-upstream
```

**Behavior by Transport:**

**Stdio Transport:**
- Spawns the configured command
- Sends MCP `initialize` JSON-RPC request
- Validates response is valid JSON-RPC
- Displays server name and version if available

**HTTP Transport:**
- Sends POST request to configured URL
- Checks for valid HTTP response
- Displays HTTP status code

**SSE Transport:**
- Sends GET request with `Accept: text/event-stream`
- Verifies server accepts SSE connections
- Displays HTTP status code

**Output Examples:**

**Stdio Success:**
```
Checking upstream connectivity...

Transport: stdio
Command:   npx
Args:      ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]

Server: Filesystem v1.0.0
✓ Upstream is reachable and responding
```

**HTTP Success:**
```
Checking upstream connectivity...

Transport: HTTP
URL:       http://localhost:8080/mcp

HTTP Status: 200
✓ Upstream is reachable
```

**Failure:**
```
Checking upstream connectivity...

Transport: stdio
Command:   /nonexistent/command

✗ Upstream check failed: command not found
```

**Timeout:**
```
✗ Upstream check timed out after 10s
```

---

## Local mode commands

These commands put mcp-guard between an agent on your machine and the stdio MCP servers it launches. Full guide: [local-mode.md](local-mode.md).

### doctor

Scan MCP client configuration files and report unprotected servers.

**Usage:**

```bash
mcp-guard doctor [--client <name-or-path>]... [--json] [--strict]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--client` | Restrict to a client (`claude-desktop`, `claude-code`, `cursor`, `windsurf`, `vscode`) or a JSON file. Repeatable. Default: every known client |
| `--json` | Machine-readable report |
| `--strict` | Exit 1 when any WARN or HIGH finding exists (for CI) |

**Example:**

```bash
$ mcp-guard doctor
mcp-guard doctor

Scanned 1 client config file(s):
  Claude Desktop   /Users/you/Library/Application Support/Claude/claude_desktop_config.json

Servers: 2 found, 0 guarded by mcp-guard

WARN  Claude Desktop / filesystem
      `npx -y @modelcontextprotocol/server-filesystem /Users/you` runs with full access to every tool it offers, no audit log, and no pinning of its tool definitions
      fix: mcp-guard wrap --client claude-desktop --server "filesystem"

Summary: 0 high, 1 warn, 0 info

Next step: mcp-guard wrap --client claude-desktop   (wraps every stdio server in that client)
```

---

### wrap

Rewrite a client's configuration so its stdio servers launch through `mcp-guard serve`.

**Usage:**

```bash
mcp-guard wrap --client <name-or-path> [OPTIONS]
```

**Options:**

| Option | Description |
|--------|-------------|
| `--client` | Client to modify (required) |
| `--server <name>` | Only wrap this server. Repeatable. Default: every stdio server |
| `--allow <globs>` | Comma-separated tools the agent may call. Default: `*` |
| `--deny <globs>` | Comma-separated tools the agent may never call, e.g. `delete_*,execute_*` |
| `--no-pin` | Do not pin tool definitions |
| `--dry-run` | Print the plan, write nothing |
| `--config-dir <dir>` | Where to put generated configs and logs. Default: `$MCP_GUARD_HOME` or `~/.config/mcp-guard` |
| `--guard-bin <path>` | Binary the client should launch. Default: the running executable |

The client file is backed up to `<file>.bak-<timestamp>` before it is written. Restart the client afterwards.

---

### unwrap

Restore servers previously wrapped.

```bash
mcp-guard unwrap --client <name-or-path> [--server <name>]... [--dry-run]
```

Reads each wrapped entry's guard config and restores the original `command` and `args`. Guard configs and lockfiles are kept.

---

### pin

Pin, review or accept the upstream's tool definitions for one guard config.

**Usage:**

```bash
mcp-guard pin --config <guard config> [--accept] [--show] [--timeout <secs>]
```

**Behaviour:**

- No lockfile yet: fetch `tools/list` from the upstream and write the lockfile. Exit 0.
- Lockfile matches: report "up to date". Exit 0.
- Lockfile differs: print changed, new and removed tools with old and new descriptions. Exit 1.
- `--accept`: write the lockfile with the current definitions. Exit 0.

While the lockfile is out of date, `serve` hides the changed and new tools and rejects calls to them with JSON-RPC error `-32011`.

---

### replay

Show a recorded session as a timeline.

```bash
mcp-guard replay --config <guard config> [--session latest|<id>|<file>] [--list] [--denied] [--tool <name>] [--json]
mcp-guard replay --dir <recordings dir> ...
```

Each client request is one line with its arguments and outcome; denials and guard decisions are marked. The last line reports whether the hash chain is intact. `--list` shows every session with record and decision counts.

---

### verify

Check a recording's hash chain.

```bash
mcp-guard verify --config <guard config> [--session latest|all|<id>|<file>] [--dir <recordings dir>] [--json]
```

Exit 1 if any checked session has been edited, truncated, reordered or has a corrupt line. For signed recordings the signing key is read from the variable named by `[recorder] signing_key_env`; without it, continuity is checked and the result is marked `OK*`.

---

### policy test

Evaluate the config's `[policy]` against one tool call and print the decision.

```bash
mcp-guard policy test --config <guard config> --tool <name> [--args '<json object>']
```

Prints `ALLOW`, `DENY` (exit 1) or, in observe mode, `WOULD DENY`, with the reason and the rule that decided. Use it while writing `[[policy.rules]]`; nothing needs restarting.

---

### policy suggest

Read the audit log and propose a least-privilege policy from what the agent actually did.

```bash
mcp-guard policy suggest --config <guard config> [--audit-log <path>] [--apply]
```

Lists every tool called with counts, every call the current policy denied, and every call it would have denied in observe mode. Prints a `[policy]` block with `mode = "enforce"` and `allowed_tools` set to the observed tools. `--apply` writes that into the config, keeping comments, `denied_tools` and `rules`.

Typical loop: set `mode = "observe"` in the guard config, use the agent for a while, run `policy suggest`, review, run again with `--apply`, restart the client.

---

### serve

Run as a stdio proxy in front of the configured upstream. This is what wrapped client entries launch; you rarely run it by hand.

```bash
mcp-guard serve --config <guard config>
```

Reads JSON-RPC from stdin, writes to stdout, logs to stderr. Applies `[policy]`, pinning and audit logging. `audit.stdout` is ignored in this mode.

---

## Common Workflows

### Initial Setup

```bash
# 1. Generate config template
mcp-guard init

# 2. Generate API key
mcp-guard keygen --user-id my-agent

# 3. Edit mcp-guard.toml with your settings

# 4. Validate configuration
mcp-guard validate

# 5. Test upstream connectivity
mcp-guard check-upstream

# 6. Start server
mcp-guard run
```

### Production Deployment

```bash
# Validate config before deployment
mcp-guard validate --config /etc/mcp-guard/production.toml

# Start with explicit config and listen on all interfaces
mcp-guard --config /etc/mcp-guard/production.toml run --host 0.0.0.0 --port 3000
```

### Adding New Users

```bash
# Generate key for new user
mcp-guard keygen --user-id new-service --rate-limit 200 --tools "read_file,list_directory"

# Add the output to your config file
# Restart the server (or hot-reload if implemented)
```

### Troubleshooting

```bash
# Verify version and features
mcp-guard version

# Validate configuration
mcp-guard validate

# Check upstream connectivity
mcp-guard check-upstream --timeout 30

# Start with verbose logging
mcp-guard -v run

# Verify a key hash
mcp-guard hash-key "mcp_your_key_here"
```

### Key Migration

```bash
# If you have existing API keys, hash them for config:
mcp-guard hash-key "$EXISTING_API_KEY"

# Add the hash to your config:
# [[auth.api_keys]]
# id = "migrated-key"
# key_hash = "<output from above>"
```

---

## Environment Variables

MCP Guard does not currently read configuration from environment variables directly. Use your shell or orchestration system to substitute values:

```bash
# Shell substitution
mcp-guard run --config <(envsubst < config.template.toml)

# Or use a wrapper script
export MCP_GUARD_PORT=8080
mcp-guard run --port $MCP_GUARD_PORT
```

---

## Licensing

mcp-guard is available in three tiers:

| Tier | Price | Key Features |
|------|-------|--------------|
| **Free** | $0 | API Key, JWT HS256, Stdio transport, Prometheus metrics |
| **Pro** | $12/mo | + OAuth 2.1, JWT JWKS, HTTP/SSE transports |
| **Enterprise** | $29+/user/mo | + mTLS, multi-server routing, SIEM, OpenTelemetry |

### Setting Up a License

For Pro and Enterprise tiers, set the `MCP_GUARD_LICENSE_KEY` environment variable:

```bash
export MCP_GUARD_LICENSE_KEY="pro_xxx..."
# or
export MCP_GUARD_LICENSE_KEY="ent_xxx..."
```

### Upgrade Prompts

When you try to use a feature that requires a higher tier, mcp-guard provides a helpful error message with upgrade instructions:

```
Error: HTTP transport requires a Pro license.

The free tier supports stdio transport only.

Upgrade to Pro for $12/month:
→ https://mcpg.botzr.com/pricing
```

For detailed tier comparison, see [Pricing & Tiers](pricing.md).

---

## See Also

- [Quick Start Guide](quickstart.md) - Get started in 5 minutes
- [Configuration Reference](configuration.md) - Complete configuration options
- [Authentication Guide](authentication.md) - Authentication provider details
- [Pricing & Tiers](pricing.md) - Feature comparison and licensing
