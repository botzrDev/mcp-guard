# Local mode: wrap, doctor, pin, serve

Local mode puts mcp-guard between an agent running on your machine (Claude Desktop, Claude Code, Cursor, Windsurf, VS Code) and the MCP servers that agent launches. There is no network and nothing to authenticate: you are the user. What local mode adds is control over which tools the agent may use, detection of tool definitions that change behind your back, and a record of what the agent did.

## How it works

An MCP client launches stdio servers as subprocesses, using the `command` and `args` in its JSON config. `mcp-guard wrap` rewrites those entries so the client launches `mcp-guard serve --config <generated config>` instead. mcp-guard then launches the original command itself and proxies the protocol in both directions.

```
Before:  Claude Desktop ──stdio──▶ npx @modelcontextprotocol/server-filesystem ~
After:   Claude Desktop ──stdio──▶ mcp-guard serve ──stdio──▶ npx @modelcontextprotocol/server-filesystem ~
                                       │
                                       ├─ [policy] allow / deny tools
                                       ├─ tool lockfile (pinning)
                                       └─ audit log
```

The proxy forwards the `initialize` handshake to the upstream, so servers that insist on being initialized first work. Resources, prompts, notifications and anything else the proxy does not understand pass through unchanged. Server-initiated requests (sampling, elicitation, roots) are not relayed yet; they are answered with a method-not-found error so the upstream does not hang.

## doctor

```bash
mcp-guard doctor                       # scan every known client
mcp-guard doctor --client cursor       # one client
mcp-guard doctor --client ./mcp.json   # any JSON file with mcpServers or servers
mcp-guard doctor --json                # machine-readable
mcp-guard doctor --strict              # exit 1 on WARN or HIGH (for CI)
```

Where it looks:

| Client | Files |
|---|---|
| Claude Desktop | `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS), `%APPDATA%\Claude\claude_desktop_config.json` (Windows), `~/.config/Claude/claude_desktop_config.json` (Linux) |
| Claude Code | `./.mcp.json`, `~/.claude.json` (top-level `mcpServers` only; project-scoped entries nested under `projects` are not scanned) |
| Cursor | `./.cursor/mcp.json`, `~/.cursor/mcp.json` |
| Windsurf | `~/.codeium/windsurf/mcp_config.json` |
| VS Code | `./.vscode/mcp.json` |

Finding codes:

| Code | Severity | Meaning |
|---|---|---|
| `UNGUARDED_STDIO` | WARN | A stdio server runs without mcp-guard |
| `REMOTE_PLAINTEXT` | HIGH | A remote server is reached over `http://` on a non-loopback host |
| `REMOTE_NO_STATIC_AUTH` | WARN | A remote server has no `Authorization` header in the client config. Fine if the server enforces OAuth; otherwise it is open |
| `SECRET_IN_CLIENT_CONFIG` | INFO | Credentials sit in plaintext in the client's `env` block |
| `GUARD_CONFIG_MISSING` | HIGH | A wrapped entry points at a guard config that cannot be loaded |
| `POLICY_ALLOW_ALL` | INFO | A guarded server allows every tool (audit and pinning still apply) |
| `PINNING_DISABLED` | WARN | A guarded server has `pin_tools = false` |
| `NOT_YET_PINNED` | INFO | Pinning is on but the lockfile has not been written yet |
| `AUDIT_DISABLED` | WARN | A guarded server has audit logging off |
| `CLIENT_CONFIG_UNREADABLE` | HIGH | The client file is not valid JSON |

## wrap

```bash
mcp-guard wrap --client claude-desktop
mcp-guard wrap --client cursor --server github --server filesystem
mcp-guard wrap --client claude-code --deny "delete_*,execute_*,run_*"
mcp-guard wrap --client ./some/mcp.json --allow "read_*,list_*" --dry-run
```

For every stdio server it:

1. Writes a guard config to `<config dir>/<client>/<server>.toml` with the original `command` and `args` as the upstream, an audit log path, and a `[policy]` section built from `--allow`, `--deny` and `--no-pin`.
2. Backs up the client file to `<file>.bak-<timestamp>`.
3. Replaces the entry's `command` with the path of the running `mcp-guard` binary (override with `--guard-bin`) and its `args` with `--config <guard config> serve`.

The entry's `env` block is left on the client entry. mcp-guard inherits it and the upstream inherits it from mcp-guard, so API keys never land in the generated config. `type`, `cwd`, `disabled` and similar client-specific keys are preserved.

Skipped entries are reported with a reason: already wrapped, remote (use gateway mode for those), or a command that fails mcp-guard's shell-injection check.

The config directory is `$MCP_GUARD_HOME` if set, otherwise `~/.config/mcp-guard` (`%APPDATA%\mcp-guard` on Windows). Override per call with `--config-dir`.

Restart the client afterwards. Most clients only read their config at startup.

## The `[policy]` section

```toml
[policy]
# Globs. "*" allows everything. Checked after denied_tools.
allowed_tools = ["*"]

# Globs. Checked first; a match here always wins.
denied_tools = ["delete_*", "execute_*"]

# Hash tool definitions into a lockfile and compare on every tools/list.
pin_tools = true

# Lockfile path. Default: <config>.tools.lock.json next to the config file.
# tools_lock = "/path/to/tools.lock.json"

# "deny": hide and reject new or changed tools until re-pinned.
# "warn": allow them, but log a warning and an audit event.
on_pin_mismatch = "deny"
```

Denied tools are removed from `tools/list` so the model never sees them, and calls to them are rejected with JSON-RPC error `-32010`. The built-in `guard/*` tools obey the same policy.

## pin

A tool definition is its name, description, input schema, and output schema or annotations when present. The model reads the description as instructions, so a server that rewrites a description after you approved it can redirect the agent. Pinning records a `sha256` of each definition and compares every later `tools/list` against the record.

```bash
mcp-guard pin --config ~/.config/mcp-guard/claude-desktop/filesystem.toml          # report, exit 1 if changed
mcp-guard pin --config ... --show                                                   # list every tool and hash
mcp-guard pin --config ... --accept                                                 # write the lockfile
```

On the first `tools/list` with no lockfile present, `serve` writes the lockfile itself (trust on first use) and logs how many tools it pinned. After that:

- a **changed** tool is hidden from `tools/list` and calls to it fail with error `-32011` and a message telling the agent to ask you to run `mcp-guard pin`
- a **new** tool is treated the same way
- a **removed** tool is reported by `pin` but does not block anything

`pin` spawns the upstream from the config, performs the handshake, fetches `tools/list`, prints a readable diff (old and new descriptions), and exits 1 when the lockfile is out of date. Commit the lockfile if you want tool changes to show up in code review.

Lockfile format:

```json
{
  "version": 1,
  "upstream": "npx -y @modelcontextprotocol/server-filesystem /Users/you",
  "pinned_at": "2026-10-22T17:00:00Z",
  "tools": {
    "read_file": {
      "hash": "sha256:3f1c...",
      "description": "Read the complete contents of a file"
    }
  }
}
```

## Audit log

Each generated config logs to `<config dir>/logs/<client>-<server>.audit.jsonl`. One JSON object per line, with the event type (`ToolCall`, `AuthzDenied`), the tool name, the request id and a timestamp. Redaction rules from `[audit] redaction_rules` apply. `audit.stdout` is ignored in serve mode because stdout carries the protocol.

## unwrap

```bash
mcp-guard unwrap --client claude-desktop
mcp-guard unwrap --client cursor --server github --dry-run
```

Reads each wrapped entry's guard config, restores the original `command` and `args`, keeps `env` and other client keys, backs up the client file, and writes it. Guard configs and lockfiles are left in place.

## Limits of local mode today

- Server-initiated requests are not relayed. Servers that use sampling or elicitation will see a method-not-found error for those calls.
- Policy is by tool name. Argument-level conditions are on the roadmap.
- Rate limits are not applied in serve mode.
- Paginated `tools/list` responses are supported: all pages are read for pinning, `nextCursor` is preserved, and `guard/*` tools are added to the first page only.
- Remote servers in a client config are reported by `doctor` but not wrapped. Put `mcp-guard run` in front of them instead.
