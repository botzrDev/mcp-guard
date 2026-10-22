# FUTURE.md: what is left to do

> Working roadmap. Last updated 2026-10-22, after PRs #5 through #8 landed.
> `CLAUDE.md` describes what exists. This file describes what does not yet.

## Where things stand

The thesis set on 2026-10-22: authenticating MCP servers is commoditized, so mcp-guard competes one level up, as the **policy and evidence layer between an agent and its tools**, starting on the developer's own machine. Four PRs delivered the local-mode core of that thesis:

| Shipped | What it gives a user |
|---|---|
| `doctor`, `wrap`, `unwrap` | One command puts the guard in front of every stdio server a client launches, and one command undoes it |
| `[policy]` globs + `[[policy.rules]]` | Allow and deny by tool name, and by argument (paths, methods, hosts) |
| Tool pinning + `pin` | A lockfile for tool definitions; changed or new tools are hidden until reviewed |
| `mode = "observe"` + `policy suggest` | Watch first, then generate a least-privilege policy from real use |
| Flight recorder + `replay` + `verify` | Hash-chained, redacted session recordings, timeline and HTML report, tamper check |
| Holds + `approve` / `deny` | A risky call waits for a person, with a desktop notification and a timeout |

Everything above is free, works offline, and needs no infrastructure. Gateway mode (`run`) is unchanged since v1.0.0.

## Priority 1: ship what exists

Nothing below matters until people can install it without a Rust toolchain. These are release tasks, not features.

- [ ] **Tag v1.1.0.** The workspace still says 1.0.0. Bump `Cargo.toml` and `crates/mcp-guard-cli/Cargo.toml`, move the CHANGELOG `[Unreleased]` block under `## [1.1.0]`, tag, push the tag. `release-tiers.yml` should build Linux, macOS and Windows binaries and attach them to the GitHub release. Check that the workflow actually succeeds; it has never run against a tag.
- [ ] **Publish to crates.io.** The README's `cargo install mcp-guard` only works if 1.1.0 is published. Confirm the crate name is still reserved under this account.
- [ ] **Homebrew tap.** `botzrDev/homebrew-mcp-guard` with a formula pointing at the release tarballs. The README already says there is no tap; remove that sentence when there is.
- [ ] **Install script.** `install.sh` exists in the repo root; make sure it downloads from the GitHub release and survives a fresh macOS and Ubuntu machine.
- [ ] **Smoke test the real clients.** Everything was tested against a mock server and a copied config. Run `wrap` against actual Claude Desktop, Claude Code, Cursor, Windsurf and VS Code installs on macOS and Windows. Expect Windows path quoting surprises in generated TOML and in `claude_desktop_config.json`.
- [ ] **Announce.** The local-mode story is complete enough to post: Hacker News, r/ClaudeAI, the Cursor and Claude Code communities, the MCP Discord. The riskiest assumption in the whole plan is that developers will route their local agents through a guard at all. Count wrapped installs and `doctor` runs in the two weeks after; that number decides how much more to invest in local mode versus gateway mode.

## Priority 2: close the gaps users will hit first

- [ ] **Server-initiated requests in serve mode.** Sampling, elicitation and roots requests from an upstream are currently answered with method-not-found. Servers that use them will misbehave behind the guard. Relay them to the client, correlate the client's response back, and record them. This is the most likely "the guard broke my server" report.
- [ ] **Concurrency in serve mode.** The proxy handles one client message at a time, so a held call or a slow upstream blocks everything else from that client. Move to per-request tasks with a shared upstream writer and an id-keyed response router. Holds then stop blocking unrelated calls.
- [ ] **Project-scoped Claude Code servers.** `doctor` and `wrap` read `~/.claude.json` top-level `mcpServers` and `.mcp.json`, not the per-project entries nested under `projects`. Add them.
- [ ] **Windows notification and paths.** The default hold notifier on Windows is a PowerShell message box; replace with a toast. Verify every generated path in `wrap` output round-trips on Windows.
- [ ] **`doctor --fix`.** Doctor already prints the exact `wrap` command for each finding; an interactive or `--yes` mode that applies them would shorten onboarding to one command.
- [ ] **Hold approvals from the client.** The 2026-07-28 spec's elicitation could let the client itself show the approval prompt instead of a desktop notification. Depends on the protocol shim below and on client support, so keep the file-based path as the fallback.

## Priority 3: the remaining roadmap items

In the order recommended earlier, with what each needs.

1. **Result inspection.** Today rules see arguments, not what comes back. Add `[[policy.results]]` (or a `on = "result"` flag on rules) that can redact secrets and PII in tool results before the model sees them, cap result size, and deny results matching patterns (a file read that returns a private key). The recorder already has the redaction engine; reuse it on the data plane. This is the feature Lasso markets; mcp-guard should have it with better policy.
2. **Recorder, pinning and rules in gateway mode.** `run` still uses only per-identity `allowed_tools`. Bring `[[policy.rules]]`, pinning per upstream, and the flight recorder (one session per identity per day, or per `Mcp-Session-Id`) to the HTTP path. Gateway users are the ones with compliance budgets.
3. **Protocol shim for 2026-07-28.** The stateless revision removed the `initialize` handshake and sessions, requires `Mcp-Method` and `Mcp-Name` headers on Streamable HTTP, moves tool schemas to JSON Schema 2020-12, and deprecates SSE. Work: (a) speak the new revision on both sides of `serve` and `run`; (b) expose an old stdio server as a compliant Streamable HTTP endpoint with OAuth, which is the "make my local server a proper remote server" feature people ask for; (c) keep old clients working against new servers. Verify details against the official changelog, not the blog posts; the sources used so far disagreed on deprecation windows.
4. **Re-tiering.** The current split paywalls HTTP transport, JWKS and SSE. Free competitors (agentgateway, Docker, IBM ContextForge) give those away, and SSE is deprecated by the spec. Proposed: make the open core complete (all auth, all transports, name-level authz, pinning, recorder, holds) and charge for policy packs, a fleet control plane, long retention and signed-recording custody, SSO and SCIM, and support. Code impact: remove the `tier.rs` gating of HTTP, SSE and JWKS, keep license verification for the paid features that remain. **This is a business decision, not an engineering one; decide before touching `tier.rs`.**
5. **Fleet control plane.** The half-built pieces (`src/db`, `server/dashboard.rs`, `server/billing.rs`, `landing/`, `workers/`) either become the paid product (push policy to many guards, collect their recordings, show a live view of what agents are doing) or get deleted to stop confusing contributors. Decide which. If kept, the first useful version is: guards upload recordings to a bucket the plane indexes, and the plane serves `replay` over the web.

## Priority 4: engineering debt worth paying

- [ ] `cargo clippy --workspace --all-targets -- -D warnings` fails on pre-existing lints in `tests/server_lifecycle_tests.rs`, `tier.rs` tests and `config/mod.rs` tests under some feature combinations. CI runs the narrower `--all-features` command and is green; make the strict one green too so contributors can run it.
- [ ] `src/db`, `server/billing.rs`, `server/dashboard.rs` pull in `sqlx` and `async-stripe`, which dominate compile time and binary size for a feature nobody uses yet. Put them behind a `control-plane` feature or remove them (see Priority 3, item 5).
- [ ] The performance targets in the README have never been measured on a release build with `wrap` in the path. Run `cargo bench`, measure serve-mode overhead per tool call, and publish the numbers.
- [ ] Rate limits do not apply in serve mode. Probably correct (nothing to key them on), but `[rate_limit] tool_limits` could meaningfully throttle `execute_*` locally. Decide and document.
- [ ] `policy suggest` proposes tool names only. It could propose argument prefixes too (the directories actually written to), which is where least privilege gets real.
- [ ] Redaction is pattern-based and the lists are hand-maintained. Consider entropy-based detection for high-entropy strings in known-secret argument names (`token`, `password`, `key`).
- [ ] The `landing/` Angular app and `workers/` have their own dependency trees and CI job. If the control plane is dropped, drop these with it.

## Decisions needed from the maintainer

These are not blocked on code.

1. **Release now?** Recommendation: yes, tag v1.1.0 from main as it stands and let the release workflow prove itself.
2. **Re-tier?** Recommendation: yes, before announcing widely. Announcing a free tier that is weaker than free competitors undercuts the launch.
3. **Control plane: build or delete?** Recommendation: delete from this repo now and start it fresh when there are paying users asking for fleet features. The half-built state costs compile time and credibility.
4. **Cedar or TOML rules long-term?** The TOML format has covered every case so far. Revisit only if users ask for conditions across multiple arguments or cross-call state.

## How to use this file

Move an item to `CLAUDE.md` when it ships, with a one-line note on how it works. Re-order priorities when the install numbers come in. Keep decisions and their dates in the section above so the next session does not re-argue them.
