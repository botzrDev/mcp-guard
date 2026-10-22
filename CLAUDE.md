# CLAUDE.md - MCP Guard Project Context

> **Last Updated:** 2026-10-22
> **Project Status:** v1.0.0 shipped (gateway). Local mode, argument-level policy with shadow mode, and the flight recorder landed; roadmap below is the plan from here.

---

## Project Overview

**mcp-guard** is a security layer for Model Context Protocol (MCP) servers, written in Rust. One binary, one config file, no infrastructure.

### Positioning
> The policy and evidence layer between an AI agent and its tools: what may the agent do, did the tools change, what did it actually do.

Authentication of MCP servers is table stakes (the spec mandates OAuth, every API gateway bolted on MCP). mcp-guard competes one level up: tool policy, tool pinning, audit evidence, and a zero-infrastructure install on the developer's own machine.

### Two modes
| Mode | Command | Who launches it | What it adds |
|---|---|---|---|
| **Local** | `mcp-guard serve` | The MCP client (Claude Desktop, Claude Code, Cursor, Windsurf, VS Code) via `mcp-guard wrap` | `[policy]` allow/deny, tool pinning lockfile, audit log. No auth: the user is the caller |
| **Gateway** | `mcp-guard run` | Operator, as a network service | API key / JWT / OAuth 2.1 / mTLS auth, per-identity authz and rate limits, metrics, tracing, SIEM shipping, multi-server routing |

---

## Repository Layout (actual)

```
mcp-guard/                       Cargo workspace, version 1.0.0, AGPL-3.0
├── crates/
│   ├── mcp-guard-core/          Library (everything below is in here)
│   │   ├── src/config/          Config types incl. PolicyConfig, UpstreamConfig, tiers
│   │   ├── src/auth/            AuthProvider trait, ApiKey, JWT (HS256/JWKS), OAuth, mTLS, MultiProvider
│   │   ├── src/authz/           Tool-name authorization, tools/list filtering
│   │   ├── src/clients.rs       MCP client config discovery, wrap/unwrap/doctor logic
│   │   ├── src/pin.rs           Tool lockfile: canonical JSON hashing, diff, load/save
│   │   ├── src/policy.rs        Policy evaluation: tool globs + argument rules -> Decision
│   │   ├── src/recorder.rs      Flight recorder: hash-chained JSONL sessions, redaction, verify, timeline
│   │   ├── src/mcp_server/      Stdio proxy (`serve`): handshake forwarding, policy, pinning, audit
│   │   ├── src/guard_tools/     guard/health, guard/metrics, guard/version tools
│   │   ├── src/server/          Axum HTTP gateway, middleware, OAuth endpoints, health, dashboard/billing routes
│   │   ├── src/router/          Multi-server path routing
│   │   ├── src/rate_limit/      Per-identity + per-tool token buckets (governor + DashMap)
│   │   ├── src/audit/           Channel-based audit logger, redaction, rotation, HTTP shipping
│   │   ├── src/transport/       Transport trait, Stdio/Http/Sse, SSRF + shell-injection validation
│   │   ├── src/observability/   Prometheus metrics, OpenTelemetry tracing
│   │   ├── src/db/              Postgres users/api_keys (sqlx) for the hosted control plane
│   │   ├── src/tier.rs          Feature gating by compile-time tier
│   │   ├── src/cli/             clap definitions (Commands enum)
│   │   ├── benches/             Criterion benchmarks
│   │   └── tests/               Integration tests (auth, oauth, jwt, audit, lifecycle)
│   ├── mcp-guard-cli/           The `mcp-guard` binary (src/main.rs has all command handlers)
│   │   └── tests/               E2E tests incl. local_mode_tests.rs (self-reinvoking mock MCP server)
│   ├── mcp-guard-pro/           Ed25519 license verification for Pro (feature "pro")
│   └── mcp-guard-enterprise/    Enterprise license + guard/keys, guard/audit, guard/config tools
├── docs/                        User + dev docs (local-mode.md, cli.md, configuration.md, ...)
├── templates/mcp-guard.toml     Annotated config template (includes [policy])
├── examples/                    api-key, jwt-auth0, oauth-github, multi-server, kubernetes
├── landing/                     Angular site (pricing, signup, dashboard). Not needed to use mcp-guard
├── workers/                     Cloudflare Workers: sign-license, stripe-webhook, download-binary
└── .github/workflows/           ci.yml (check, fmt, clippy -D warnings, test, docs, coverage, audit, deny), release-tiers.yml
```

The `landing/` and `workers/` directories plus `src/db`, `server/billing.rs` and `server/dashboard.rs` are a half-built hosted control plane (users, API keys, Stripe checkout). They compile and are tested lightly; they are not part of the local-mode story.

---

## Key Commands

```bash
cargo build                                        # debug build (binary: target/debug/mcp-guard)
cargo test --workspace                             # all tests
cargo test -p mcp-guard --test local_mode_tests    # local mode e2e (wrap/doctor/pin/serve)
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cargo bench -p mcp-guard-core

# Local mode
mcp-guard doctor [--client cursor] [--json] [--strict]
mcp-guard wrap --client claude-desktop [--server NAME] [--deny "delete_*"] [--dry-run]
mcp-guard unwrap --client claude-desktop
mcp-guard pin --config ~/.config/mcp-guard/claude-desktop/NAME.toml [--accept] [--show]
mcp-guard policy test --config X.toml --tool write_file --args '{"path": "/etc/passwd"}'
mcp-guard policy suggest --config X.toml [--apply]      # from audit log, after mode = "observe"
mcp-guard replay --config X.toml [--list|--denied|--tool T|--json]
mcp-guard verify --config X.toml [--session all]       # exit 1 on a broken chain
mcp-guard serve --config X.toml           # what wrapped clients launch

# Gateway mode
mcp-guard init | validate | keygen --user-id X | run | check-upstream | version
```

CI runs clippy with `-D warnings` on current stable. New clippy lints on toolchain upgrades break CI; fix them rather than allow them.

---

## Design Notes That Matter

- **Serve mode forwards `initialize`.** The client's handshake and `notifications/initialized` go to the upstream unchanged. The upstream's protocol version and capabilities are passed through, with a `tools` capability added. Guard-only serve (no upstream) negotiates among 2024-11-05 .. 2025-11-25.
- **Serve mode preserves client ids** and uses its own id space (from 1_000_000) upstream. Upstream notifications received while waiting are forwarded to the client before the response. Upstream-initiated requests get method-not-found (not relayed yet).
- **Pinning is TOFU.** First `tools/list` with no lockfile writes it. Hash = sha256 over canonical (key-sorted) JSON of name, description, inputSchema, outputSchema, annotations. `on_pin_mismatch = "deny"` hides new/changed tools and rejects calls with -32011; policy denials use -32010. A corrupt lockfile treats every tool as unpinned rather than disabling protection.
- **Wrap keeps secrets out of guard configs.** The client entry's `env` stays on the client entry; mcp-guard inherits it and the upstream inherits it from mcp-guard.
- **stdout is the protocol in serve mode.** All logs go to stderr; `audit.stdout` is forced off.
- **Shell-injection validation** in `transport` rejects commands/args with shell metacharacters. `wrap` skips such entries with a reason instead of generating a config that fails at launch.
- **Policy evaluation order** (`policy::evaluate`): denied_tools, then allowed_tools, then `[[policy.rules]]` in order, first match wins, no match allows. Rules only run on `tools/call` (they need arguments); `tools/list` visibility uses the globs only. An absent argument fails every operator except `exists = false`.
- **Observe mode** (`policy.mode = "observe"`): nothing blocked, `PolicyWouldDeny` audit events written, denied tools stay visible in `tools/list`. Pinning is independent of mode. `policy suggest` parses the audit JSONL (`event_type` = `tool_call` / `policy_would_deny`) and `--apply` edits the config with `toml_edit` so comments survive.
- **Recorder** (`recorder.rs`): one JSONL file per serve session under `recorder.dir_for(config)`. `hash = H(prev + "\n" + canonical_json(record minus hash))`, H = SHA-256 or HMAC-SHA256 when `signing_key_env` is set. Redaction (built-in patterns + audit rules) and truncation happen before hashing; `payload_sha256` is over the original. `McpServer::handle_message` records the incoming message and every outgoing one; `deny()`, observe mode and hidden tools write `decision` records. Recorder failures are logged, never fatal.
- **Config struct literals in tests** must include `policy: Default::default()` and `recorder: Default::default()`.
- **`upstream.transport` defaults to stdio**; multi-server configs may omit `[upstream]` entirely.
- **E2E tests set `MCP_GUARD_SKIP_LICENSE_CHECK`** (in `tests/common/mod.rs`) so pro/enterprise builds start without a key.
- Rate limits are not applied in serve mode (nothing to key them on).

---

## Roadmap (agreed direction, 2026-10-22)

1. Result inspection: rules on tool results (secret/PII redaction, size limits), extending the argument rules. Cedar was considered for the policy engine and deferred: the TOML rule format covers the known cases and stays readable in one config file.
2. Recorder in gateway mode (per identity), retention, HTML session report.
3. Protocol shim: 2026-07-28 stateless revision, relay server-initiated requests (sampling/elicitation/roots), expose stdio servers as compliant remote servers.
4. Human-in-the-loop holds for high-risk tools (approve in terminal/Slack before a `delete_*` call proceeds).
5. Prebuilt binaries + Homebrew tap (release-tiers.yml exists; no published artifacts yet).
6. Re-tier: open core complete (all auth, all transports); charge for policy packs, recorder, fleet control plane, support. `tier.rs` gating of HTTP/SSE/JWKS is the thing to remove when this happens.

Done: local mode (wrap/doctor/pin/serve), argument-level policy rules, shadow mode + `policy suggest`, flight recorder (`replay`/`verify`).

---

## Technical Decisions Log

- Axum 0.7 + Tokio; governor for rate limiting; jsonwebtoken + oauth2; sqlx (Postgres) for the hosted plane; Criterion for benches.
- AGPL-3.0 core, commercial Pro/Enterprise licenses verified offline with Ed25519 (`mcp-guard-pro/src/license.rs`).
- Tier gating is compile-time (`--features pro|enterprise`) plus runtime license validation before `run`/`serve`.
- Local mode deliberately has no identity model: `LOCAL_IDENTITY = "local"` in audit entries.
- E2E tests avoid external runtimes: `local_mode_tests.rs` re-invokes the test binary as a mock MCP server (`MCP_GUARD_TEST_MOCK_SERVER=1`).

---

## History (condensed)

- 2025-12: Sprints 1 to 6 built the gateway: auth providers, rate limiting, metrics, OAuth 2.1, OpenTelemetry, HTTP/SSE transports, mTLS, health endpoints, multi-server routing, audit shipping, benchmarks. v1.0.0 tagged 2025-12-26.
- 2026-01: Stripe checkout, GitHub OAuth login, landing site, license-signing workers.
- 2026 (to Oct): CI hardening (Windows build, cargo-deny, security audit), link cleanup, docs pointed at in-repo files.
- 2026-10-22: Strategic review. Local mode shipped: `doctor`, `wrap`, `unwrap`, `pin`, `[policy]`, tool pinning, serve-mode handshake forwarding and audit. README rewritten to match the real workspace. (PR #5)
- 2026-10-22: Argument-level `[[policy.rules]]`, `mode = "observe"`, `policy test`, `policy suggest --apply`. Fixed pre-existing OAuth callback and routing test failures. (PR #6)
- 2026-10-22: Flight recorder: `[recorder]`, hash-chained session files, redaction, `replay`, `verify`, HMAC signing.

---

## How to Use This File

Update this file when you change the layout, add a command, make an architectural decision, or move a roadmap item. Keep it factual: describe what exists, not what was planned. Read it first in each session.
