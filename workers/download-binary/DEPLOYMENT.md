# Quick Deployment Guide

## One-Time Setup

### 1. Create R2 Buckets

```bash
wrangler r2 bucket create mcp-guard-pro-binaries
wrangler r2 bucket create mcp-guard-enterprise-binaries
```

### 2. Set Secrets

```bash
# Pro license public key
cat ../../.keys/pro_license_public.pem | grep -v "BEGIN\|END" | tr -d '\n' | \
  wrangler secret put PRO_LICENSE_PUBLIC_KEY

# Keygen account ID (get from https://keygen.sh dashboard)
wrangler secret put KEYGEN_ACCOUNT_ID
# Paste your Keygen account UUID
```

### 3. Install Dependencies

```bash
cd workers/download-binary
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
# Deploys to https://download.mcp-guard.io (if custom domain configured)
```

## Upload Binaries

After GitHub Actions builds complete, binaries are automatically uploaded to R2.

**Manual upload** (for testing):

```bash
# Set AWS credentials for R2
export AWS_ACCESS_KEY_ID="your-r2-access-key"
export AWS_SECRET_ACCESS_KEY="your-r2-secret-key"
export AWS_ENDPOINT_URL="https://<account-id>.r2.cloudflarestorage.com"

# Upload Pro binary
aws s3 cp target/release/mcp-guard \
  s3://mcp-guard-pro-binaries/latest/mcp-guard-x86_64-linux

# Upload Enterprise binary
aws s3 cp target/release/mcp-guard \
  s3://mcp-guard-enterprise-binaries/latest/mcp-guard-x86_64-darwin
```

**Binary naming convention:**
- Linux (glibc): `latest/mcp-guard-x86_64-linux`
- Linux (musl): `latest/mcp-guard-x86_64-linux-musl`
- macOS (Intel): `latest/mcp-guard-x86_64-darwin`
- macOS (ARM): `latest/mcp-guard-aarch64-darwin`
- Windows: `latest/mcp-guard.exe-x86_64-windows`

## Test

```bash
# Health check
curl https://download.mcp-guard.io/health

# Download with license
curl "https://download.mcp-guard.io/download?tier=pro&platform=x86_64-linux&license=pro_xxx..." \
  -o mcp-guard

chmod +x mcp-guard
./mcp-guard version
```

## Monitor

```bash
# Live logs
wrangler tail

# View metrics
# Go to: Cloudflare Dashboard → Workers → mcp-guard-download → Metrics
```

## Troubleshooting

**"Binary not found"**
- Check R2 bucket contents: `wrangler r2 object list mcp-guard-pro-binaries`
- Verify naming convention matches expected format

**"Invalid license signature"**
- Verify public key secret matches signing key
- Regenerate license with correct keypair

**"Keygen validation failed"**
- Verify `KEYGEN_ACCOUNT_ID` secret is correct
- Test Keygen API: `curl https://api.keygen.sh/v1/accounts/<id>/licenses/<key>/actions/validate -X POST`

## Rollback

```bash
# Rollback to previous version
wrangler rollback
```

## Custom Domain Setup

Edit `wrangler.toml`:
```toml
[env.production]
routes = [
  { pattern = "download.mcp-guard.io/*", custom_domain = true }
]
```

Then deploy:
```bash
wrangler deploy --env production
```

Cloudflare automatically:
- Creates DNS record
- Provisions SSL certificate
- Routes traffic to Worker
