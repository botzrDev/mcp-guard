# Multi-Server Routing

This example shows how to route requests to different MCP servers based on path prefixes.

## Architecture

```
                    ┌─────────────────────┐
                    │    mcp-guard        │
                    │    (port 3000)      │
                    └──────────┬──────────┘
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
          ▼                    ▼                    ▼
    ┌───────────┐       ┌───────────┐       ┌───────────┐
    │ /fs       │       │ /github   │       │ /db       │
    │ Filesystem│       │ GitHub    │       │ Database  │
    │ (stdio)   │       │ (stdio)   │       │ (http)    │
    └───────────┘       └───────────┘       └───────────┘
```

## Setup

1. Generate an API key:
   ```bash
   mcp-guard keygen --user-id multi-user
   ```

2. Update `mcp-guard.toml` with the key hash

3. Start the gateway:
   ```bash
   mcp-guard run --config mcp-guard.toml
   ```

## Usage

### List available routes
```bash
curl http://localhost:3000/routes
# Response: {"routes":["filesystem","github","database","search","default"],"count":5}
```

### Access specific servers
```bash
# Filesystem server
curl -X POST http://localhost:3000/mcp/filesystem \
  -H "Authorization: Bearer YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'

# GitHub server
curl -X POST http://localhost:3000/mcp/github \
  -H "Authorization: Bearer YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

## Route Matching

Routes are matched by longest prefix:
- `/fs/subpath` -> `filesystem` server
- `/github/repos` -> `github` server
- `/db/query` -> `database` server
- `/anything/else` -> `default` server (matches `/`)

## Transport Types

This example demonstrates all three transport types:
- **stdio**: Spawns a local process (filesystem, github)
- **http**: HTTP POST to a remote server (database)
- **sse**: Server-Sent Events for streaming (search)

## Adding More Servers

Add a new `[[upstream.servers]]` block:
```toml
[[upstream.servers]]
name = "custom"
path_prefix = "/custom"
transport = "stdio"
command = "/path/to/custom-server"
args = ["--flag", "value"]
```
