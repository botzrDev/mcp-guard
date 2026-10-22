# Flight recorder: replay and verify

The flight recorder keeps a tamper-evident, redacted record of everything an agent did through `mcp-guard serve`: every request it sent, every response it got, and every decision the guard made in between. `mcp-guard replay` renders a session as a timeline. `mcp-guard verify` proves the record has not been edited.

Audit logs tell you *that* a tool was called. The recorder tells you *what was asked, what came back, and what was blocked*, in order, with a chain of hashes that an auditor or incident responder can check.

## Turning it on

`mcp-guard wrap` enables it for every server it wraps, with recordings under `~/.config/mcp-guard/recordings/<client>-<server>/`. For a hand-written config:

```toml
[recorder]
enabled = true
# dir = "recordings"            # default: <config stem>.recordings/ next to the config
# max_payload_bytes = 65536     # larger payloads are stored as a preview plus a hash
# include_results = true        # false records requests, decisions and errors only
# redact = true                 # built-in secret patterns plus [audit] redaction_rules
# signing_key_env = "MCP_GUARD_RECORDER_KEY"   # HMAC instead of plain hashes (see below)
```

`mcp-guard doctor` reports `RECORDER_DISABLED` for guarded servers that have it off.

## What a session looks like

```bash
mcp-guard replay --config ~/.config/mcp-guard/claude-desktop/filesystem.toml
```

```
Session 7f3a9c1d2e4b  /Users/me/.config/mcp-guard/recordings/claude-desktop-filesystem/20261022-181502-7f3a9c1d2e4b.jsonl
Upstream npx -y @modelcontextprotocol/server-filesystem /Users/me
Records  41   Policy mode: enforce

18:15:02.114  session start               npx -y @modelcontextprotocol/server-filesystem /Users/me  policy=enforce
18:15:02.301  initialize                  secure-filesystem-server 0.6.2 (2025-06-18)
18:15:02.420  tools/list                  11 tools
18:15:02.421  decision tool_hidden_policy delete_file  ✗ tool 'delete_file' is not allowed by policy
18:15:09.877  tools/call read_file        {"path":"/Users/me/project/README.md"}  → ok {"content":[{"type":"text","text":"# my project…
18:15:14.002  tools/call write_file       {"path":"/etc/hosts","content":"127.0.0.1 evil"}  → ✗ DENIED mcp-guard: call to 'write_file' denied by policy: writes outside the project are blocked
18:15:14.002  decision policy_deny write_file  ✗ writes outside the project are blocked
18:17:40.556  session end                 stdin closed

Chain OK
```

Useful views:

```bash
mcp-guard replay --config X --list              # every session, newest first
mcp-guard replay --config X --session 7f3a9c     # by id prefix, file name, or path
mcp-guard replay --config X --denied             # only denials and guard decisions
mcp-guard replay --config X --tool write_file    # one tool's calls
mcp-guard replay --config X --json               # raw records, one JSON object per line
mcp-guard replay --dir /path/to/recordings       # without a config
```

## Record format

One JSONL file per session, named `<UTC timestamp>-<session id>.jsonl`. Each line:

```json
{
  "seq": 5,
  "ts": "2026-10-22T18:15:14.002Z",
  "dir": "client_to_guard",
  "kind": "request",
  "id": 4,
  "method": "tools/call",
  "tool": "write_file",
  "payload": {"params": {"name": "write_file", "arguments": {"path": "/etc/hosts", "content": "127.0.0.1 evil"}}},
  "payload_sha256": "sha256:9c1e…",
  "prev": "sha256:4b8d…",
  "hash": "sha256:a07f…"
}
```

| Field | Meaning |
|---|---|
| `seq` | Position in the session, from 0 |
| `dir` | `client_to_guard`, `guard_to_client`, or `guard` for session boundaries and decisions |
| `kind` | `session_start`, `session_end`, `request`, `response`, `notification`, `decision` |
| `method` | JSON-RPC method, or the decision type for `decision` records |
| `tool` | Tool name for `tools/call` traffic and decisions |
| `payload` | Redacted, possibly truncated: `params`, `result`, `error`, or decision detail |
| `payload_sha256` | SHA-256 of the full original payload, before redaction and truncation |
| `truncated`, `redacted` | Present and `true` when either happened |
| `prev` | `hash` of the previous record; a fixed genesis value for `seq` 0 |
| `hash` | Hash of this record with `hash` removed, prefixed by `prev` |

Decision types: `policy_deny`, `pin_deny`, `policy_would_deny` (observe mode), `tool_hidden_policy`, `tool_hidden_pin`.

The `session_start` payload records the session id, start time, guard version, upstream command, config path, policy mode and whether the recording is signed. `session_end` records why the session ended.

## Redaction

Payloads are redacted before they are hashed and written, so a recording can be shared without leaking credentials. Built-in patterns cover private key blocks, AWS, GitHub, Slack, OpenAI, Anthropic, Google and Stripe keys, mcp-guard API keys, JWTs, bearer tokens and `user:password@` URLs. Replacements name the pattern, for example `[REDACTED:github_token]`. Your own `[audit] redaction_rules` are applied as well.

`payload_sha256` is computed over the original payload. If you still hold the original (a file the agent read, a request body you logged elsewhere), you can hash it and match it to the record without the recording itself containing the secret.

Redaction is pattern-based. It will not catch a secret that does not look like one. Review a recording before sending it to a third party.

## Tamper evidence

Every record's `hash` covers its own contents and the previous record's hash. Editing, deleting, inserting or reordering any record changes the hash chain from that point on, and `verify` reports the first record where the chain no longer holds:

```bash
$ mcp-guard verify --config X --session all
OK       .../20261022-181502-7f3a9c1d2e4b.jsonl  (41 records)
BROKEN   .../20261021-093311-c02d44e1aa90.jsonl  (12 records)
           record 7: hash does not match record contents
           record 8: prev does not match the previous record's hash
```

`verify` exits 1 when any session is broken, so it can run in CI or a cron job. `--json` gives the result per file.

### Plain hashes versus signed recordings

With plain SHA-256, anyone who can edit the file can also recompute the chain. That still catches accidental corruption, truncation and careless edits, and it makes a deliberate forgery a deliberate act rather than a slip. It does not stop someone with write access and intent.

Set `signing_key_env` to the name of an environment variable holding a secret, and hashes become HMAC-SHA256 under that key. The writing process needs the key; a forger needs it too. Keep the key somewhere the agent's machine cannot read it at rest (a secrets manager, a hardware token, a separate host that pulls the recordings), and `verify` on that other host becomes a real integrity check. Without the key, `verify` still checks continuity and says so (`OK*`).

Signed recordings mark `signed: true` in their `session_start` record.

## Operational notes

- Recording is append-only and flushed per record. A crash loses at most the record being written; the session simply has no `session_end`, which `verify` points out.
- Large tool results are truncated to `max_payload_bytes` with a preview and the hash of the full result. Set `include_results = false` to keep only requests, decisions and errors.
- Recordings are not rotated or deleted automatically. Prune the directory on your own schedule; each session is one file.
- The recorder never writes to stdout, which carries the protocol, and a recorder failure never blocks the agent. A failed write is logged to stderr and the session continues unrecorded from that point.
- Gateway mode (`mcp-guard run`) does not record yet. It writes the audit log.

## What is on the roadmap

Recording in gateway mode keyed by identity, retention policies, and exporting a session as a self-contained HTML report.
