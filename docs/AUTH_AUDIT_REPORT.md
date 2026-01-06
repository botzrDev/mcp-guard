# Authentication Audit Report
**Date**: 2025-01-06
**Project**: MCP Guard
**Auditor**: Claude Code

---

## Executive Summary

Audited GitHub OAuth, Google OAuth, and Magic Link authentication methods. Found **3 critical issues** preventing authentication from working:

1. ❌ **GitHub OAuth**: Redirect URI misconfiguration
2. ❌ **Google OAuth**: Not implemented in backend
3. ❌ **Magic Link**: Endpoint does not exist

---

## Issue #1: GitHub OAuth - Redirect URI Mismatch

### Current State
```toml
# .env file
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=http://localhost:3000/oauth/callback
```

### Problem
- Frontend expects callback at `http://localhost:4200/auth/callback` (Angular SPA)
- Backend configured to receive callback at `http://localhost:3000/oauth/callback` (API server)
- GitHub OAuth App must be configured with the **frontend** callback URL

### Flow Analysis

**Current (Broken) Flow:**
```
User clicks "Login with GitHub"
  → Frontend redirects to backend /oauth/authorize
  → Backend redirects to GitHub
  → GitHub redirects to http://localhost:3000/oauth/callback
  → Backend receives code, exchanges for token
  → ❌ Frontend never receives the session token
```

**Expected (Working) Flow:**
```
User clicks "Login with GitHub"
  → Frontend redirects to backend /oauth/authorize?redirect_uri=http://localhost:4200/auth/callback
  → Backend generates PKCE challenge, redirects to GitHub with frontend callback
  → GitHub redirects to http://localhost:4200/auth/callback?code=xxx&state=yyy
  → Frontend sends code to backend
  → Backend exchanges code for token, mints JWT session
  → ✅ Frontend receives JWT and stores in localStorage
```

### Root Cause
The backend OAuth callback handler (server/mod.rs:576-679) expects to receive the OAuth callback directly, but the frontend is designed to handle the callback and then call the backend to exchange the code.

**However**, looking at line 671-678 in server/mod.rs:
```rust
if let Some(redirect_uri) = pkce_state.redirect_uri {
    let target_url = format!("{}?token={}", redirect_uri, session_token);
    tracing::info!(to = %target_url, "Redirecting to frontend with session token");
    return Ok((headers, Redirect::temporary(&target_url).into_response()).into_response());
}
```

The backend **already supports** redirecting back to the frontend with the token! The issue is that the `redirect_uri` parameter must be passed in the initial `/oauth/authorize` request.

### Solution

#### 1. Update .env for Local Development
```bash
# Local development - backend redirects to frontend after OAuth
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=http://localhost:4200/auth/callback
```

#### 2. Update GitHub OAuth App Settings
In GitHub OAuth App configuration (https://github.com/settings/developers):

**Authorization callback URLs:**
- `http://localhost:4200/auth/callback` (local dev)
- `https://mcpg.botzr.com/auth/callback` (production)

#### 3. Frontend Auth Service (Already Correct!)
The frontend auth service (landing/src/app/core/auth/auth.service.ts:72-77) already passes the redirect_uri:
```typescript
loginWithGitHub(): void {
    this.loading.set(true);
    this.error.set(null);
    const returnUrl = encodeURIComponent(window.location.origin + '/auth/callback');
    window.location.href = `${API_BASE}/oauth/authorize?provider=github&redirect_uri=${returnUrl}`;
}
```

This is **correct** ✅

### Testing Steps
1. Update `.env` redirect URI to `http://localhost:4200/auth/callback`
2. Restart backend: `cargo run --bin mcp-guard run`
3. Click "Continue with GitHub" in frontend
4. Verify redirect flow:
   - Browser redirects to GitHub
   - GitHub asks for permission
   - GitHub redirects back to `http://localhost:4200/auth/callback?token=<JWT>`
   - Frontend extracts token and stores in localStorage
5. Verify dashboard access works

---

## Issue #2: Google OAuth - Not Configured

### Current State
- Frontend has "Continue with Google" button (login.component.ts:66-78)
- Backend OAuth config only supports **single provider** (mcp-guard.toml:38-44 shows only GitHub)
- No Google credentials in `.env`

### Problem
The backend `OAuthConfig` struct supports provider selection via the `provider` enum (GitHub, Google, Okta, Custom), but the config file only configures **one** OAuth provider at a time.

The frontend tries to select a provider via query param:
```typescript
window.location.href = `${API_BASE}/oauth/authorize?provider=google&redirect_uri=${returnUrl}`;
```

But the backend doesn't support this pattern - it uses whatever provider is configured in `auth.oauth`.

### Solution

#### Option A: Support Multi-Provider (Recommended)
Modify the backend to support multiple OAuth providers simultaneously:

1. Change `auth.oauth` from single config to array: `auth.oauth_providers = [...]`
2. Update `/oauth/authorize` to accept `provider` query param and select the correct config
3. Add Google credentials to `.env`:
```bash
MCP_GUARD_AUTH_OAUTH_GOOGLE_CLIENT_ID=your-google-client-id
MCP_GUARD_AUTH_OAUTH_GOOGLE_CLIENT_SECRET=your-google-secret
```

4. Update `mcp-guard.toml`:
```toml
[[auth.oauth_providers]]
provider = "github"
client_id = "set_via_env_var"
client_secret = "set_via_env_var"
redirect_uri = "http://localhost:4200/auth/callback"
scopes = ["read:user", "user:email"]

[[auth.oauth_providers]]
provider = "google"
client_id = "set_via_env_var"
client_secret = "set_via_env_var"
redirect_uri = "http://localhost:4200/auth/callback"
scopes = ["openid", "profile", "email"]
```

#### Option B: Disable Google Login (Quick Fix)
Remove the Google button from the frontend until multi-provider support is added.

---

## Issue #3: Magic Link - Not Implemented

### Current State
- Frontend calls `POST /auth/magic-link` (auth.service.ts:91-122)
- **This endpoint does not exist in the backend**

### Missing Components
1. `POST /auth/magic-link` endpoint
2. Magic link token generation (signed, time-limited)
3. Email sending service integration (e.g., SendGrid, Resend, AWS SES)
4. `GET /auth/magic-link/verify?token=xxx` endpoint
5. Token validation and JWT session minting

### Solution

Implement magic link authentication:

#### 1. Add Dependencies
```toml
# Cargo.toml
lettre = "0.11"  # Email sending
resend = "0.8"   # Or use Resend API
```

#### 2. Create Magic Link Module
```rust
// src/auth/magic_link.rs
pub struct MagicLinkProvider {
    secret: String,
    email_sender: Arc<dyn EmailSender>,
    ttl_seconds: u64,
}

impl MagicLinkProvider {
    pub fn generate_token(&self, email: &str) -> String {
        // Generate signed, time-limited token
    }

    pub fn verify_token(&self, token: &str) -> Result<String, AuthError> {
        // Verify signature and expiration, return email
    }

    pub async fn send_magic_link(&self, email: &str, redirect_uri: &str) -> Result<(), AuthError> {
        // Generate token, send email with link
    }
}
```

#### 3. Add Routes
```rust
// src/server/mod.rs
router = router
    .route("/auth/magic-link", post(send_magic_link))
    .route("/auth/magic-link/verify", get(verify_magic_link));
```

#### 4. Environment Config
```bash
# .env
MAGIC_LINK_SECRET=your-secret-key-min-32-chars
MAGIC_LINK_TTL_SECONDS=900  # 15 minutes
RESEND_API_KEY=re_xxxxxxxxxxxx
```

### Alternative: Disable Magic Link
If email infrastructure is not ready, remove the magic link form from the frontend login component.

---

## Automated Testing Plan

### Test 1: GitHub OAuth End-to-End
```bash
#!/bin/bash
# tests/e2e/github_oauth_test.sh

# 1. Start backend
# 2. Simulate OAuth authorize request
# 3. Verify redirect to GitHub with correct params
# 4. Simulate GitHub callback with authorization code
# 5. Verify JWT token is minted
# 6. Verify token can authenticate to /mcp endpoint
```

### Test 2: Google OAuth End-to-End
Same as GitHub test, but with Google provider.

### Test 3: Magic Link Flow
```bash
#!/bin/bash
# tests/e2e/magic_link_test.sh

# 1. POST /auth/magic-link with email
# 2. Verify 200 response
# 3. Extract magic link from email (test email service)
# 4. GET magic link URL
# 5. Verify JWT token is returned
# 6. Verify token can authenticate
```

### Test 4: Redirect URI Validation
```rust
#[tokio::test]
async fn test_oauth_callback_ip_binding() {
    // Verify PKCE state is bound to client IP (security test)
}

#[tokio::test]
async fn test_oauth_state_expiry() {
    // Verify OAuth states expire after 10 minutes
}
```

---

## Recommendations

### Immediate Actions (P0)
1. ✅ Fix GitHub OAuth redirect URI in `.env`
2. ✅ Update GitHub OAuth App callback URLs
3. ✅ Test GitHub OAuth flow end-to-end

### Short Term (P1)
4. Implement multi-provider OAuth support
5. Add Google OAuth credentials
6. Implement magic link authentication OR remove from UI

### Long Term (P2)
7. Add automated E2E tests for all auth methods
8. Add monitoring/alerting for OAuth failures
9. Implement OAuth token refresh flow
10. Add rate limiting on `/auth/*` endpoints

---

## Environment Variables Summary

### Required for GitHub OAuth (Local Dev)
```bash
MCP_GUARD_AUTH_OAUTH_CLIENT_ID=Ov23liPmETYO7XQqmW9g
MCP_GUARD_AUTH_OAUTH_CLIENT_SECRET=8078b85f829e4c9c4b8396a78119331de3455234
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=http://localhost:4200/auth/callback
```

### Required for Production
```bash
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=https://mcpg.botzr.com/auth/callback
```

### Required for Google OAuth (when implemented)
```bash
MCP_GUARD_AUTH_OAUTH_GOOGLE_CLIENT_ID=your-google-client-id
MCP_GUARD_AUTH_OAUTH_GOOGLE_CLIENT_SECRET=your-google-secret
```

### Required for Magic Link (when implemented)
```bash
MAGIC_LINK_SECRET=your-secret-key-min-32-chars
RESEND_API_KEY=re_xxxxxxxxxxxx
```

---

## Security Notes

1. ✅ **PKCE is implemented** - Protects against authorization code interception
2. ✅ **State parameter is validated** - Prevents CSRF attacks
3. ✅ **Client IP binding** - Prevents state fixation attacks (line 527-536 in server/mod.rs)
4. ✅ **Token caching with TTL** - Reduces OAuth provider load
5. ⚠️ **No rate limiting on `/oauth/authorize`** - Add rate limiting to prevent DoS
6. ⚠️ **OAuth state store has memory limit** - Good (MAX_PENDING_OAUTH_STATES = 10,000)

---

## Files Modified
- `.env` - Update redirect URI
- `docs/AUTH_AUDIT_REPORT.md` - This document

## Files to Create
- `tests/e2e/github_oauth_test.sh` - GitHub OAuth E2E test
- `tests/e2e/google_oauth_test.sh` - Google OAuth E2E test
- `tests/e2e/magic_link_test.sh` - Magic link E2E test
- `crates/mcp-guard-core/src/auth/magic_link.rs` - Magic link implementation
