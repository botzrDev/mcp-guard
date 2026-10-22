<div align="center">
  <h1>mcp-guard</h1>
  <p><strong>The policy and evidence layer between your AI agent and its tools.</strong></p>
  <p>One binary. One config file. No Docker, no Kubernetes, no DevOps team.</p>

  <p>
    <a href="https://github.com/botzrDev/mcp-guard/actions/workflows/ci.yml"><img src="https://github.com/botzrDev/mcp-guard/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License"></a>
    <a href="https://crates.io/crates/mcp-guard"><img src="https://img.shields.io/crates/v/mcp-guard.svg" alt="crates.io"></a>
  </p>

  <p>
    <a href="#quick-start">Quick Start</a> &bull;
    <a href="#two-ways-to-run-it">Modes</a> &bull;
    <a href="#features">Features</a> &bull;
    <a href="#roadmap">Roadmap</a> &bull;
    <a href="docs/">Documentation</a>
  </p>
</div>

<br>

## The Problem

Model Context Protocol (MCP) servers give AI agents real power: files, shells, databases, SaaS APIs. Most of them run with no policy at all.

- A server advertises whatever tools it likes, and the agent calls them. Nobody said which tools were allowed.
- A server can silently rewrite its tool descriptions after you approved it. The model reads those descriptions as instructions.
- When something goes wrong, there is no record of what the agent actually did.
- Remote servers are often exposed with no authentication whatsoever.

## The Solution

`mcp-guard` sits between the agent and every MCP server it uses, and answers three questions: **what may the agent do, did the tools change, and what did it actually do?**

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   Agent     │─────▶│  mcp-guard  │─────▶│ MCP Server  │
│ (Claude,    │      │             │      │  (yours)    │
│  Cursor...) │◀─────│             │◀─────│             │
└─────────────┘      └─────────────┘      └─────────────┘
                            │
                     Tool policy (allow / deny)
                     Tool pinning (lockfile)
                     Audit log of every call
                     Auth + rate limits (gateway mode)
```

<br>

## Quick Start

Build the binary (there are no prebuilt binaries or Homebrew tap yet):

```bash
cargo install mcp-guard
# or: git clone https://github.com/botzrDev/mcp-guard && cd mcp-guard && cargo build --release
```

Then see what your agents are running unprotected:

```bash
mcp-guard doctor
```

```
Scanned 2 client config file(s):
  Claude Desktop   /Users/you/Library/Application Support/Claude/claude_desktop_config.json
  Cursor           /Users/you/.cursor/mcp.json

Servers: 4 found, 0 guarded by mcp-guard

WARN  Claude Desktop / filesystem
      `npx -y @modelcontextprotocol/server-filesystem /Users/you` runs with full access to
      every tool it offers, no audit log, and no pinning of its tool definitions
      fix: mcp-guard wrap --client claude-desktop --server "filesystem"
...
Next step: mcp-guard wrap --client claude-desktop
```

Put the guard in front of them. The client's config is backed up first, and the original command, arguments and environment are preserved:

```bash
mcp-guard wrap --client claude-desktop --deny "delete_*,execute_*"
```

Restart the client. Every stdio server now runs behind mcp-guard with a per-server config under `~/.config/mcp-guard/`, an audit log, and its tool definitions pinned on first use. Edit `[policy]` in the generated config to tighten access, and run `mcp-guard doctor` again.

<br>

## Two ways to run it

### Local mode: `mcp-guard serve`

For the servers your own agent launches on your machine. The MCP client runs `mcp-guard serve`, which launches the real server as a subprocess and proxies the protocol. There is nothing to authenticate here: you are the user. What you get is control and evidence.

| What it does | How |
|---|---|
| Restrict which tools the agent may call | `[policy] allowed_tools` and `denied_tools`, with globs |
| Detect tool poisoning and rug pulls | Tool definitions are hashed into a lockfile on first use. Changed or new tools are hidden and rejected until you review them with `mcp-guard pin` |
| Record what happened | Every tool call and every denial is written to an audit log |
| Prove what happened | The flight recorder keeps every request, response and decision in a hash-chained, redacted session file. `replay` renders it, `verify` checks it |
| Keep secrets out of guard configs | Environment variables stay on the client's entry and are inherited by the upstream |
| Get in and out safely | `mcp-guard wrap` backs up the client config; `mcp-guard unwrap` restores it |

Works with Claude Desktop, Claude Code, Cursor, Windsurf and VS Code, or any JSON file with an `mcpServers` or `servers` map. See [docs/local-mode.md](docs/local-mode.md).

### Gateway mode: `mcp-guard run`

For servers other people or other systems reach over the network. mcp-guard listens on HTTP, authenticates callers, applies per-identity tool permissions and rate limits, and forwards to the upstream over stdio, HTTP or SSE.

```bash
mcp-guard init        # generate mcp-guard.toml with a demo API key
mcp-guard run         # listen on :3000
curl -H "Authorization: Bearer mcp_YOUR_KEY" http://localhost:3000/health
```

<br>

## Features

### Local mode (free, all tiers)

- Tool policy with allow and deny globs
- Argument-level rules (`write_file` only under the workspace, `http_request` only `GET`), tested with `mcp-guard policy test`
- Shadow mode: observe first, then `mcp-guard policy suggest --apply` writes a least-privilege policy from what the agent actually used
- Tool pinning with a reviewable JSON lockfile (`mcp-guard pin`)
- Flight recorder: tamper-evident, redacted per-session recording; `mcp-guard replay` shows what the agent did, `mcp-guard verify` proves nobody edited it
- Per-server audit log
- `doctor`, `wrap`, `unwrap` for Claude Desktop, Claude Code, Cursor, Windsurf, VS Code
- `guard/health`, `guard/metrics`, `guard/version` tools exposed to the agent
- Transparent forwarding of the `initialize` handshake and of resources, prompts and notifications

### Gateway mode

| Authentication | Free | Pro | Enterprise |
|--------|:----:|:---:|:----------:|
| API Keys | ✓ | ✓ | ✓ |
| JWT (HS256) | ✓ | ✓ | ✓ |
| JWT (JWKS/RS256/ES256) | | ✓ | ✓ |
| OAuth 2.1 + PKCE | | ✓ | ✓ |
| mTLS Client Certificates | | | ✓ |

| Transport | Free | Pro | Enterprise |
|------|:----:|:---:|:----------:|
| Stdio | ✓ | ✓ | ✓ |
| HTTP | | ✓ | ✓ |
| SSE | | ✓ | ✓ |
| Multi-Server Routing | | | ✓ |

| Security & Observability | Free | Pro | Enterprise |
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

Pro and Enterprise builds are not on sale yet. The tier split is under review; see [Roadmap](#roadmap).

<br>

## Configuration

`mcp-guard init` generates a config file with sensible defaults. The full annotated template is at [templates/mcp-guard.toml](templates/mcp-guard.toml).

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

# Used by `mcp-guard serve` (local mode)
[policy]
mode = "enforce"           # or "observe" to record without blocking
allowed_tools = ["*"]
denied_tools = ["delete_*", "execute_*"]
pin_tools = true
on_pin_mismatch = "deny"   # or "warn"

[[policy.rules]]
name = "writes stay in the workspace"
tools = ["write_file", "edit_file"]
action = "deny"
[policy.rules.when]
path = { not_starts_with = "/home/me/project/" }
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
[[upstream.servers]]
name = "filesystem"
path_prefix = "/fs"
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "./"]

[[upstream.servers]]
name = "database"
path_prefix = "/db"
transport = "http"
url = "http://localhost:8080"
```

</details>

<br>

## Protocol support

mcp-guard speaks MCP revisions 2024-11-05 through 2025-11-25 and passes the upstream's negotiated version through unchanged. It is transparent for resources, prompts, logging and progress notifications. Server-initiated requests (sampling, elicitation, roots) are not yet relayed in local mode and are answered with a method-not-found error so the upstream does not hang. Support for the 2026-07-28 stateless revision is on the roadmap.

<br>

## Performance

Targets, checked with the Criterion suite in `crates/mcp-guard-core/benches` (`cargo bench`):

| Metric | Target |
|--------|--------|
| Latency overhead | <2ms p99 |
| Binary size | <15MB |
| Memory usage | <50MB RSS |
| Throughput | >5,000 RPS |

<br>

## CLI Reference

```
mcp-guard <command>

Local mode
  doctor           Scan MCP client configs and report unprotected servers
  wrap             Put mcp-guard in front of a client's stdio servers
  unwrap           Restore servers wrapped with `wrap`
  pin              Pin, review or accept the upstream's tool definitions
  policy test      Evaluate the policy against one tool call
  policy suggest   Propose a least-privilege policy from the audit log
  replay           Show a recorded session as a timeline
  verify           Check a recording's hash chain
  serve            Run as a stdio proxy (what wrapped clients launch)

Gateway mode
  init             Generate config file with demo API key
  validate         Check config file for errors
  keygen           Generate a new API key
  hash-key         Hash an existing API key for the config
  run              Start the HTTP gateway
  check-upstream   Test upstream server connectivity

  version          Show version and build info

Options:
  -c, --config     Config file path (default: mcp-guard.toml)
  -v, --verbose    Verbose logging (stderr)
  -h, --help       Show help
```

<br>

## Roadmap

The direction: stop competing on authentication, which the MCP spec and every API gateway now cover, and own the layer above it, **what an agent may do and proof of what it did**. In rough order:

1. **Result inspection.** Rules on what comes back from a server (secret and PII redaction, size limits), extending today's argument rules.
2. **Recorder in gateway mode.** Per-identity recording for `mcp-guard run`, retention, and a self-contained HTML session report.
3. **Protocol shim.** Full support for the 2026-07-28 stateless revision, relaying server-initiated requests, and exposing old stdio servers as compliant remote servers.
4. **Human-in-the-loop holds** for high-risk tools.
5. **Prebuilt binaries and a Homebrew tap.**
6. **Re-tiering.** Make the open core complete (all auth, all transports) and charge for policy packs, the recorder, fleet management and support.

Issues and pull requests on any of these are welcome.

<br>

## Documentation

| Topic | Guide |
|-------|-------|
| Local mode | [wrap, doctor, pin, serve](docs/local-mode.md) |
| Flight recorder | [replay, verify, record format](docs/flight-recorder.md) |
| Getting Started | [Quick Start](docs/quickstart.md) |
| Authentication | [Auth Guide](docs/authentication.md) |
| Transports | [Stdio/HTTP/SSE](docs/transports.md) |
| Configuration | [Reference](docs/configuration.md) |
| Rate Limiting | [Rate Limits](docs/rate-limiting.md) |
| Observability | [Metrics & Tracing](docs/observability.md) |
| Security Model | [Threats and controls](docs/security-model.md) |
| Deployment | [Production Guide](docs/deployment.md) |
| API Reference | [HTTP API](docs/api/http.md) |
| CLI | [Command reference](docs/cli.md) |

<br>

## Repository layout

```
crates/
  mcp-guard-core/        Library: config, auth, authz, policy, pinning, transports, server
  mcp-guard-cli/         The `mcp-guard` binary
  mcp-guard-pro/         Pro license verification (optional feature)
  mcp-guard-enterprise/  Enterprise license verification and admin tools (optional feature)
docs/                    User and developer documentation
examples/                Config examples (API key, JWT with Auth0, GitHub OAuth, multi-server, Kubernetes)
templates/               Annotated config template
landing/, workers/       Website and Cloudflare Workers for licensing (not needed to use mcp-guard)
```

<br>

## Security

Security vulnerabilities should be reported via [support@botzr.com](mailto:support@botzr.com).

See [SECURITY.md](SECURITY.md) for our security policy.

<br>

## Contributing

We welcome contributions. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/botzrDev/mcp-guard
cd mcp-guard
cargo build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

<br>

## License

AGPL-3.0. See [LICENSE](LICENSE).

Commercial licenses available for Pro and Enterprise tiers.

---

<div align="center">
  <sub>Built by <a href="https://botzr.com">botzr</a></sub>
</div>
