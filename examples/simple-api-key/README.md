# Simple API Key Authentication

This example shows the simplest way to secure an MCP server with API key authentication.

## Setup

1. Generate an API key:
   ```bash
   mcp-guard keygen --user-id my-agent
   ```

2. Copy the hash from the output and replace `REPLACE_WITH_YOUR_KEY_HASH` in `mcp-guard.toml`

3. Start the gateway:
   ```bash
   mcp-guard run --config mcp-guard.toml
   ```

4. Make authenticated requests:
   ```bash
   curl -X POST http://localhost:3000/mcp \
     -H "Authorization: Bearer YOUR_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
   ```

## Configuration Notes

- `allowed_tools`: Restricts which tools the API key can access. Use an empty array `[]` for unrestricted access.
- `rate_limit`: Per-identity request limit (requests per second). Overrides the global default.

## Security

- Never commit your actual API keys to version control
- Store the key securely (password manager, environment variable)
- Only the hash is stored in the config file
