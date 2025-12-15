# GitHub OAuth 2.1 Authentication

This example shows how to use GitHub OAuth with PKCE for user authentication.

## Prerequisites

1. Create a GitHub OAuth App:
   - Go to https://github.com/settings/developers
   - Click "New OAuth App"
   - Set "Authorization callback URL" to `http://localhost:3000/oauth/callback`
   - Note your Client ID and Client Secret

## Setup

1. Update `mcp-guard.toml` with your GitHub OAuth credentials:
   - `client_id`: Your GitHub OAuth App Client ID
   - `client_secret`: Your GitHub OAuth App Client Secret

2. Start the gateway:
   ```bash
   mcp-guard run --config mcp-guard.toml
   ```

3. Initiate OAuth flow (opens browser):
   ```bash
   # Start authorization
   open http://localhost:3000/oauth/authorize
   ```

4. After authorization, you'll receive an access token. Use it:
   ```bash
   curl -X POST http://localhost:3000/mcp \
     -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
   ```

## Scope-to-Tool Mapping

The `scope_tool_mapping` controls which tools users can access based on their OAuth scopes:

- Users with the `repo` scope can access repository-modifying tools
- Users with only `read:user` scope have read-only access

## Security Notes

- Store `client_secret` in environment variables for production
- Use HTTPS in production (set `redirect_uri` to HTTPS URL)
- The OAuth flow uses PKCE (Proof Key for Code Exchange) for security
