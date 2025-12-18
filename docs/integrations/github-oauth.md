# GitHub OAuth Integration Guide

Integrate MCP Guard with GitHub OAuth for user authentication.

## Prerequisites

- GitHub account
- MCP Guard installed and running
- Public URL for OAuth callback (or `localhost` for development)

---

## Step 1: Create a GitHub OAuth App

1. Go to GitHub **Settings** → **Developer settings** → **OAuth Apps**
2. Click **New OAuth App**
3. Fill in the application details:

| Field | Value |
|-------|-------|
| **Application name** | `MCP Guard` |
| **Homepage URL** | `https://your-domain.com` (or `http://localhost:3000`) |
| **Authorization callback URL** | `https://your-domain.com/oauth/callback` |

4. Click **Register application**

---

## Step 2: Get Your Credentials

After creating the app, you'll see:

- **Client ID:** Displayed on the app page
- **Client Secret:** Click **Generate a new client secret**

**Important:** Save the client secret immediately. It's only shown once.

---

## Step 3: Configure MCP Guard

Add the OAuth configuration to your `mcp-guard.toml`:

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "https://your-domain.com/oauth/callback"
scopes = ["read:user"]
user_id_claim = "id"  # GitHub uses numeric user ID
```

For development with localhost:

```toml
[auth.oauth]
provider = "github"
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "http://localhost:3000/oauth/callback"
scopes = ["read:user"]
user_id_claim = "id"
```

---

## Step 4: Configure Scope-to-Tool Mapping

Map GitHub OAuth scopes to MCP tools:

```toml
[auth.oauth.scope_tool_mapping]
"read:user" = ["read_file", "list_directory"]
"repo" = ["read_file", "write_file", "list_directory"]
"admin:org" = ["*"]  # All tools for org admins
```

---

## Step 5: Test the OAuth Flow

### Start the Authorization Flow

1. Start MCP Guard:
   ```bash
   mcp-guard run
   ```

2. Open the authorization URL in a browser:
   ```
   http://localhost:3000/oauth/authorize
   ```

3. GitHub will prompt you to authorize the application

4. After authorization, you're redirected to the callback URL with an access token

### Use the Access Token

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_GITHUB_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

---

## GitHub Scopes Reference

| Scope | Description | Recommended Use |
|-------|-------------|-----------------|
| `read:user` | Read user profile | Basic authentication |
| `user:email` | Access email addresses | Email-based identity |
| `repo` | Full repository access | Repository tools |
| `public_repo` | Public repositories only | Limited repo access |
| `read:org` | Read organization membership | Org-based authorization |
| `admin:org` | Full org admin access | Admin tools |

**Recommendation:** Request the minimum scopes needed. Start with `read:user` and add more as needed.

---

## Complete Configuration Example

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-github"]

[auth.oauth]
provider = "github"
client_id = "Iv1.abc123def456"
client_secret = "your-secret-here"
redirect_uri = "https://mcp.example.com/oauth/callback"
scopes = ["read:user", "repo"]
user_id_claim = "id"

[auth.oauth.scope_tool_mapping]
"read:user" = ["get_user", "list_repos"]
"repo" = ["read_file", "write_file", "create_pr"]

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
stdout = true
```

---

## Production Considerations

### Use HTTPS

GitHub OAuth requires HTTPS for production callback URLs:

```toml
redirect_uri = "https://mcp.example.com/oauth/callback"
```

### Secure Your Client Secret

Store the client secret securely:

```toml
# Option 1: Direct in config (less secure)
client_secret = "your-secret"

# Option 2: Environment variable (recommended)
# Set GITHUB_CLIENT_SECRET environment variable
# Then reference in config (if supported by your deployment)
```

### Token Validation

GitHub OAuth tokens are validated using the GitHub UserInfo endpoint. MCP Guard caches valid tokens for 5 minutes to reduce API calls.

---

## Troubleshooting

### "Redirect URI mismatch"

1. The callback URL must match **exactly** what's registered in GitHub
2. Check for:
   - Protocol mismatch (`http` vs `https`)
   - Trailing slash differences
   - Port number differences
3. Update either GitHub or your config to match

### "Invalid client credentials"

1. Verify your `client_id` matches GitHub
2. Regenerate the client secret if needed
3. Check for whitespace or encoding issues

### "Bad verification code"

1. Authorization codes are single-use and expire quickly
2. Don't reuse codes from previous attempts
3. Complete the OAuth flow within a few minutes

### "Scope not granted"

1. Users can decline certain scopes during authorization
2. Check which scopes the user actually granted
3. Request only scopes you need

### "Token validation failed"

1. Tokens may be revoked by the user
2. Token may have expired
3. Network issues reaching GitHub's API

---

## Development Tips

### Testing Locally

For local development, you can use `localhost`:

1. Register `http://localhost:3000/oauth/callback` in GitHub
2. Use the same URL in your config
3. GitHub allows `localhost` without HTTPS

### Multiple Environments

Create separate OAuth apps for development and production:

- `MCP Guard (Dev)` → `http://localhost:3000/oauth/callback`
- `MCP Guard (Prod)` → `https://mcp.example.com/oauth/callback`

### Debugging the Flow

Enable verbose logging to see OAuth flow details:

```bash
mcp-guard -v run
```

---

## See Also

- [Authentication Guide](../authentication.md) - Complete auth provider documentation
- [Configuration Reference](../configuration.md) - All configuration options
- [Troubleshooting Guide](../troubleshooting.md) - Common issues and solutions
