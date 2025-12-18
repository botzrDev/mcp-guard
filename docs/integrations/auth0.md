# Auth0 Integration Guide

Integrate MCP Guard with Auth0 for JWT authentication using JWKS.

## Prerequisites

- Auth0 account ([sign up free](https://auth0.com))
- MCP Guard installed and running
- Basic familiarity with JWT authentication

---

## Step 1: Create an API in Auth0

1. Log in to your Auth0 Dashboard
2. Navigate to **Applications** → **APIs**
3. Click **Create API**
4. Configure the API:
   - **Name:** `MCP Guard`
   - **Identifier:** `mcp-guard` (this becomes your `audience`)
   - **Signing Algorithm:** RS256

5. Click **Create**

---

## Step 2: Configure Permissions (Scopes)

1. Go to your new API → **Permissions** tab
2. Add permissions for your MCP tools:

| Permission | Description |
|------------|-------------|
| `read:files` | Read files and list directories |
| `write:files` | Create and modify files |
| `admin` | Full administrative access |

3. Click **Add** for each permission

---

## Step 3: Get Your Auth0 Configuration Values

From your Auth0 dashboard, collect:

| Value | Where to Find | Example |
|-------|---------------|---------|
| Domain | **Settings** → **Domain** | `your-tenant.auth0.com` |
| JWKS URL | `https://{domain}/.well-known/jwks.json` | `https://your-tenant.auth0.com/.well-known/jwks.json` |
| Issuer | `https://{domain}/` | `https://your-tenant.auth0.com/` |
| Audience | The **Identifier** from Step 1 | `mcp-guard` |

---

## Step 4: Configure MCP Guard

Add the JWT configuration to your `mcp-guard.toml`:

```toml
[auth.jwt]
mode = "jwks"
jwks_url = "https://your-tenant.auth0.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://your-tenant.auth0.com/"
audience = "mcp-guard"
scopes_claim = "permissions"  # Auth0 RBAC uses 'permissions' claim

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory", "get_file_info"]
"write:files" = ["write_file", "create_directory", "delete_file"]
"admin" = ["*"]  # All tools
```

**Important:** Auth0 uses `permissions` (not `scope`) for RBAC-assigned permissions. Set `scopes_claim = "permissions"`.

---

## Step 5: Create a Test Application

To test the integration, create a Machine-to-Machine application:

1. Go to **Applications** → **Applications** → **Create Application**
2. Select **Machine to Machine Applications**
3. Name it `MCP Guard Test`
4. Select your `MCP Guard` API
5. Select the permissions to grant
6. Click **Create**

Note your **Client ID** and **Client Secret** from the application settings.

---

## Step 6: Get a Test Token

Request an access token using the Client Credentials flow:

```bash
curl -X POST "https://your-tenant.auth0.com/oauth/token" \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "YOUR_CLIENT_ID",
    "client_secret": "YOUR_CLIENT_SECRET",
    "audience": "mcp-guard",
    "grant_type": "client_credentials"
  }'
```

**Response:**

```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

---

## Step 7: Test Authentication

Call MCP Guard with the access token:

```bash
# List available tools
curl -X POST http://localhost:3000/mcp \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "tools/list", "id": 1}'
```

**Expected response:** List of tools the user is authorized to access.

---

## Step 8: Enable RBAC (Recommended)

For fine-grained authorization, enable Auth0's RBAC feature:

1. Go to your API → **Settings** tab
2. Enable **Enable RBAC**
3. Enable **Add Permissions in the Access Token**
4. Save changes

Now permissions assigned via roles appear in the `permissions` claim.

---

## Complete Configuration Example

```toml
[server]
host = "0.0.0.0"
port = 3000

[upstream]
transport = "stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/data"]

[auth.jwt]
mode = "jwks"
jwks_url = "https://your-tenant.auth0.com/.well-known/jwks.json"
algorithms = ["RS256"]
issuer = "https://your-tenant.auth0.com/"
audience = "mcp-guard"
scopes_claim = "permissions"
cache_duration_secs = 3600  # Cache JWKS for 1 hour

[auth.jwt.scope_tool_mapping]
"read:files" = ["read_file", "list_directory"]
"write:files" = ["write_file", "delete_file"]
"admin" = ["*"]

[rate_limit]
enabled = true
requests_per_second = 100
burst_size = 50

[audit]
enabled = true
file = "/var/log/mcp-guard/audit.log"
```

---

## Troubleshooting

### "Invalid JWT signature"

1. Verify your JWKS URL is accessible:
   ```bash
   curl https://your-tenant.auth0.com/.well-known/jwks.json
   ```
2. Ensure `algorithms = ["RS256"]` matches your Auth0 API signing algorithm
3. Check the token wasn't issued by a different Auth0 tenant

### "Invalid issuer"

1. Auth0 issuer requires a trailing slash: `https://your-tenant.auth0.com/`
2. Inspect the token at [jwt.io](https://jwt.io) to see the actual `iss` claim
3. Match the issuer exactly in your config

### "Invalid audience"

1. Verify the `aud` claim matches your API identifier (`mcp-guard`)
2. Check you're requesting a token for the correct audience

### "No tools authorized"

1. Ensure `scopes_claim = "permissions"` for Auth0 RBAC
2. Verify RBAC is enabled on your API
3. Check the user/application has permissions assigned
4. Inspect the token to see what permissions are included

### "JWKS fetch failed"

1. Verify network connectivity to Auth0
2. Check for firewall rules blocking outbound HTTPS
3. If behind a proxy, ensure it allows connections to `*.auth0.com`

---

## See Also

- [Authentication Guide](../authentication.md) - Complete auth provider documentation
- [Configuration Reference](../configuration.md) - All configuration options
- [Troubleshooting Guide](../troubleshooting.md) - Common issues and solutions
