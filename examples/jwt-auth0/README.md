# Auth0 JWT Authentication

This example shows how to integrate with Auth0 for JWT-based authentication using JWKS.

## Prerequisites

1. Create an Auth0 account at https://auth0.com
2. Create an API in Auth0:
   - Go to APIs > Create API
   - Set "Identifier" to `mcp-guard`
   - Select RS256 signing algorithm

3. Enable RBAC permissions:
   - In your API settings, enable "Enable RBAC" and "Add Permissions in the Access Token"
   - Add permissions: `admin`, `read:files`, `write:files`, `delete:files`

## Setup

1. Update `mcp-guard.toml`:
   - Replace `YOUR_AUTH0_DOMAIN` with your Auth0 domain (e.g., `myapp.us.auth0.com`)

2. Start the gateway:
   ```bash
   mcp-guard run --config mcp-guard.toml
   ```

3. Get a token from Auth0 (using client credentials):
   ```bash
   curl --request POST \
     --url https://YOUR_AUTH0_DOMAIN.auth0.com/oauth/token \
     --header 'content-type: application/json' \
     --data '{
       "client_id":"YOUR_CLIENT_ID",
       "client_secret":"YOUR_CLIENT_SECRET",
       "audience":"mcp-guard",
       "grant_type":"client_credentials"
     }'
   ```

4. Use the token:
   ```bash
   curl -X POST http://localhost:3000/mcp \
     -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
   ```

## Permission Mapping

Auth0 permissions map to MCP tools via `scope_tool_mapping`:

| Auth0 Permission | MCP Tools |
|-----------------|-----------|
| `admin` | All tools (`*`) |
| `read:files` | `read_file`, `list_directory`, `search_files` |
| `write:files` | `write_file`, `create_directory` |
| `delete:files` | `delete_file`, `delete_directory` |

## JWKS Cache

The JWKS is cached for 1 hour (`cache_duration_secs = 3600`) to reduce latency. The cache refreshes automatically in the background.

## Tracing

This example enables OpenTelemetry tracing. View traces in Jaeger:
```bash
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  jaegertracing/all-in-one:latest

# Open http://localhost:16686
```
