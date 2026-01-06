# Authentication Setup Guide
**Project**: MCP Guard
**Last Updated**: 2025-01-06

---

## Quick Start

### 1. Fix GitHub OAuth (Local Development)

#### Step 1: Update GitHub OAuth App Settings
Go to https://github.com/settings/developers and select your OAuth App.

**Add both callback URLs:**
- `http://localhost:4200/auth/callback` (local dev)
- `https://mcpg.botzr.com/auth/callback` (production)

#### Step 2: Update Local Environment File
```bash
# Copy the template
cp .env .env.backup
cp .env.local .env

# Or manually update .env:
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=http://localhost:4200/auth/callback
```

#### Step 3: Restart Backend
```bash
cd /home/austingreen/Documents/botzr/projects/mcp-guard
cargo run --bin mcp-guard run
```

#### Step 4: Test the Flow
```bash
# Run automated tests
./tests/e2e/run_auth_tests.sh

# Or test manually:
# 1. Open http://localhost:4200/login
# 2. Click "Continue with GitHub"
# 3. Authorize on GitHub
# 4. Verify redirect to http://localhost:4200/auth/callback?token=<JWT>
# 5. Verify dashboard access
```

---

## Production Deployment

### 1. Configure Environment Variables

Set these in your deployment platform (Railway, Fly.io, etc.):

```bash
# GitHub OAuth
MCP_GUARD_AUTH_OAUTH_CLIENT_ID=your-production-github-client-id
MCP_GUARD_AUTH_OAUTH_CLIENT_SECRET=your-production-github-secret
MCP_GUARD_AUTH_OAUTH_REDIRECT_URI=https://mcpg.botzr.com/auth/callback

# Database
MCP_GUARD_DATABASE_URL=postgresql://user:password@host:5432/mcp_guard

# Stripe (if using billing)
STRIPE_SECRET_KEY=sk_live_xxxxxxxxxxxx
```

### 2. Update GitHub OAuth App (Production)

Create a **separate** GitHub OAuth App for production or add the production callback URL:

**Authorization callback URLs:**
- `https://mcpg.botzr.com/auth/callback`

### 3. Deploy Backend

```bash
# Build production binary
cargo build --release

# Or deploy via your platform's CLI
railway up  # or fly deploy, etc.
```

### 4. Test Production

```bash
# Run tests against production
./tests/e2e/run_auth_tests.sh --prod

# Or test manually at https://mcpg.botzr.com/login
```

---

## Adding Google OAuth

### Step 1: Create Google OAuth Credentials

1. Go to https://console.cloud.google.com/apis/credentials
2. Create OAuth 2.0 Client ID
3. Add authorized redirect URIs:
   - `http://localhost:4200/auth/callback` (local)
   - `https://mcpg.botzr.com/auth/callback` (production)
4. Copy Client ID and Client Secret

### Step 2: Update Configuration

#### Option A: Keep Single Provider (Current Architecture)
Edit `mcp-guard.toml` to switch from GitHub to Google:

```toml
[auth.oauth]
provider = "google"
client_id = "set_via_env_var"
client_secret = "set_via_env_var"
redirect_uri = "http://localhost:4200/auth/callback"
scopes = ["openid", "profile", "email"]
```

Add to `.env`:
```bash
MCP_GUARD_AUTH_OAUTH_CLIENT_ID=your-google-client-id
MCP_GUARD_AUTH_OAUTH_CLIENT_SECRET=your-google-secret
```

#### Option B: Support Multi-Provider (Requires Backend Changes)

This requires modifying the backend to support multiple OAuth providers simultaneously. See `docs/AUTH_AUDIT_REPORT.md` Issue #2 for implementation details.

---

## Implementing Magic Link Authentication

Magic link is currently **not implemented**. The frontend has UI for it, but the backend endpoint doesn't exist.

### Implementation Steps

#### 1. Add Email Service Dependency

```toml
# Cargo.toml
[dependencies]
resend = "0.8"  # Or use lettre, sendgrid, etc.
```

#### 2. Add Environment Variables

```bash
# .env
MAGIC_LINK_SECRET=your-secret-key-minimum-32-characters-long
MAGIC_LINK_TTL_SECONDS=900  # 15 minutes
RESEND_API_KEY=re_xxxxxxxxxxxx
```

#### 3. Create Magic Link Module

Create `crates/mcp-guard-core/src/auth/magic_link.rs` with:
- Token generation (HMAC signed, time-limited)
- Email sending logic
- Token verification
- JWT session minting

#### 4. Add Routes to Server

```rust
// In crates/mcp-guard-core/src/server/mod.rs

router = router
    .route("/auth/magic-link", post(send_magic_link))
    .route("/auth/magic-link/verify", get(verify_magic_link));
```

#### 5. Frontend Configuration

The frontend is already configured to use magic link. No changes needed once backend is implemented.

### Alternative: Disable Magic Link UI

If you don't want to implement magic link auth immediately:

1. Edit `landing/src/app/pages/auth/login/login.component.ts`
2. Remove or comment out the magic link form section (lines 84-116)
3. Remove `sendMagicLink()` and `resetMagicLink()` methods

---

## Testing

### Automated Tests

```bash
# Test all auth methods
./tests/e2e/run_auth_tests.sh

# Test specific method
./tests/e2e/github_oauth_test.sh

# Test against production
./tests/e2e/run_auth_tests.sh --prod
```

### Manual Testing Checklist

#### GitHub OAuth
- [ ] Local: Click "Continue with GitHub" at http://localhost:4200/login
- [ ] Authorize on GitHub
- [ ] Verify redirect to `http://localhost:4200/auth/callback?token=<JWT>`
- [ ] Verify JWT stored in localStorage
- [ ] Verify dashboard access works
- [ ] Logout and re-login
- [ ] Production: Repeat above at https://mcpg.botzr.com

#### Google OAuth (when implemented)
- [ ] Click "Continue with Google"
- [ ] Authorize on Google
- [ ] Verify redirect with token
- [ ] Verify dashboard access

#### Magic Link (when implemented)
- [ ] Enter email address
- [ ] Check email for magic link
- [ ] Click link in email
- [ ] Verify redirect with token
- [ ] Verify dashboard access

---

## Security Checklist

- [x] PKCE implemented for OAuth (protects against code interception)
- [x] State parameter validated (prevents CSRF)
- [x] Client IP binding for OAuth state (prevents state fixation)
- [x] OAuth token caching with TTL
- [x] OAuth state expiry (10 minutes)
- [x] DoS protection (MAX_PENDING_OAUTH_STATES = 10,000)
- [ ] Rate limiting on `/oauth/authorize` (recommended)
- [ ] Rate limiting on `/auth/magic-link` (when implemented)
- [ ] Email validation for magic link (when implemented)

---

## Troubleshooting

### GitHub OAuth "Redirect URI Mismatch" Error

**Symptom:** GitHub shows "The redirect_uri MUST match the registered callback URL"

**Solution:**
1. Check your GitHub OAuth App settings
2. Ensure callback URL is exactly: `http://localhost:4200/auth/callback` (local) or `https://mcpg.botzr.com/auth/callback` (prod)
3. Verify `.env` has matching `MCP_GUARD_AUTH_OAUTH_REDIRECT_URI`
4. Restart backend after changing `.env`

### Token Not Stored in Frontend

**Symptom:** Redirected to callback but not logged in

**Check:**
1. Open browser dev tools → Network tab
2. Look for callback request with `?token=` parameter
3. Verify token is present in URL
4. Check Console for JavaScript errors
5. Verify `auth.service.ts` is extracting and storing token

### "OAuth not configured" Error

**Symptom:** Backend returns 500 error on `/oauth/authorize`

**Solution:**
1. Verify `[auth.oauth]` section exists in `mcp-guard.toml`
2. Verify environment variables are set: `MCP_GUARD_AUTH_OAUTH_CLIENT_ID` and `MCP_GUARD_AUTH_OAUTH_CLIENT_SECRET`
3. Check backend logs for OAuth provider initialization
4. Restart backend after config changes

### Magic Link Button Shows But Doesn't Work

**Symptom:** "Failed to send magic link" or network error

**Solution:**
This is expected - magic link is not implemented yet. Either:
- Implement magic link (see "Implementing Magic Link Authentication" above)
- Or remove the magic link UI from frontend

---

## File Reference

### Configuration Files
- `.env` - Environment variables (local dev)
- `.env.local` - Template for local development
- `.env.production` - Template for production
- `mcp-guard.toml` - Main configuration file

### Backend Files
- `crates/mcp-guard-core/src/auth/oauth.rs` - OAuth implementation (GitHub, Google)
- `crates/mcp-guard-core/src/server/mod.rs:491-771` - OAuth endpoints (`/oauth/authorize`, `/oauth/callback`)
- `crates/mcp-guard-core/src/config/mod.rs:772-797` - Environment variable overrides

### Frontend Files
- `landing/src/app/core/auth/auth.service.ts` - Auth service (login methods)
- `landing/src/app/pages/auth/login/login.component.ts` - Login UI
- `landing/src/app/core/auth/auth.models.ts` - Auth TypeScript types

### Test Files
- `tests/e2e/github_oauth_test.sh` - GitHub OAuth automated test
- `tests/e2e/run_auth_tests.sh` - Test runner for all auth methods

### Documentation
- `docs/AUTH_AUDIT_REPORT.md` - Detailed audit findings
- `docs/AUTH_SETUP_GUIDE.md` - This file

---

## Quick Reference

### Start Backend (Local)
```bash
cd /home/austingreen/Documents/botzr/projects/mcp-guard
cargo run --bin mcp-guard run
```

### Start Frontend (Local)
```bash
cd /home/austingreen/Documents/botzr/projects/mcp-guard/landing
npm install
npm start
```

### Run All Tests
```bash
./tests/e2e/run_auth_tests.sh
```

### Check Server Health
```bash
curl http://localhost:3000/health
```

### Test OAuth Authorize Endpoint
```bash
curl -v "http://localhost:3000/oauth/authorize?provider=github&redirect_uri=http://localhost:4200/auth/callback"
```

---

## Support

For issues or questions:
1. Check `docs/AUTH_AUDIT_REPORT.md` for detailed technical analysis
2. Review backend logs for error messages
3. Run automated tests to identify specific failures
4. Check GitHub OAuth App configuration matches redirect URIs
