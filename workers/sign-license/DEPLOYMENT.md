# Quick Deployment Guide - License Signing Worker

## One-Time Setup

### 1. Set Secrets

```bash
cd workers/sign-license

# Ed25519 private key
cat ../../.keys/pro_license_private.pem | \
  wrangler secret put PRO_LICENSE_PRIVATE_KEY

# API secret (generate and save to password manager)
openssl rand -hex 32 | tee api-secret.txt | wrangler secret put API_SECRET_KEY
# IMPORTANT: Save api-secret.txt to password manager, then delete it
```

### 2. Install Dependencies

```bash
npm install
```

## Deploy

### Development

```bash
wrangler dev
# Test at http://localhost:8787
```

### Production

```bash
wrangler deploy --env production
# Deploys to https://sign.mcp-guard.io
```

## Test

```bash
# Health check
curl https://sign.mcp-guard.io/health

# Generate license (requires API secret from password manager)
export API_SECRET="<your-api-secret>"

curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{
    "licensee": "test@example.com",
    "expires_at": "2026-12-31T23:59:59Z"
  }' | jq .

# Extract and test license
LICENSE_KEY=$(curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{"licensee": "test@example.com"}' | jq -r .license_key)

export MCP_GUARD_LICENSE_KEY="$LICENSE_KEY"
mcp-guard version
# Should show: "Tier: Pro"
```

## Monitor

```bash
# Live logs
wrangler tail

# View metrics
# Go to: Cloudflare Dashboard → Workers → mcp-guard-license-signer → Metrics
```

## Troubleshooting

**"Invalid or missing API secret key"**
- Verify API secret in password manager
- Check `X-API-Secret` header spelling
- Verify secret: `wrangler secret list`

**"Failed to parse Ed25519 private key"**
- Re-set private key secret:
  ```bash
  cat ../../.keys/pro_license_private.pem | wrangler secret put PRO_LICENSE_PRIVATE_KEY
  ```

**Generated licenses fail validation**
- Verify public/private key pair match:
  ```bash
  openssl pkey -in .keys/pro_license_private.pem -pubout
  # Should match base64 in mcp-guard-pro/src/license.rs
  ```

## Security Notes

1. **API Secret**: Only share with:
   - Stripe webhook handler Worker
   - Admin portal (future)
   - Your secure password manager

2. **Private Key**: NEVER expose, log, or commit to git
   - Only stored in Cloudflare Secrets
   - Backup stored offline in secure location

3. **Access**: This Worker is NOT public
   - Not listed in customer documentation
   - Only callable by authorized services
   - Monitor unauthorized access attempts

## Custom Domain Setup

Edit `wrangler.toml`:
```toml
[env.production]
routes = [
  { pattern = "sign.mcp-guard.io/*", custom_domain = true }
]
```

Deploy:
```bash
wrangler deploy --env production
```

## Rollback

```bash
# Rollback to previous version
wrangler rollback
```
