# MCP-Guard Binary Download Worker

Cloudflare Worker for validating licenses and serving Pro/Enterprise binaries from R2 storage.

## Features

- **License Validation**: Validates Pro licenses (Ed25519 signatures) and Enterprise licenses (Keygen.sh)
- **Binary Delivery**: Streams binaries from Cloudflare R2 to authenticated users
- **Platform Detection**: Auto-detects platform from User-Agent or accepts explicit platform parameter
- **Secure**: No license bypass - all downloads require valid license validation

## Architecture

```
User Request → Worker validates license → Fetch from R2 → Stream binary
```

- **Pro licenses**: Validated offline using Ed25519 signature verification
- **Enterprise licenses**: Validated online via Keygen.sh API
- **Binaries**: Stored in separate R2 buckets (mcp-guard-pro-binaries, mcp-guard-enterprise-binaries)

## Prerequisites

1. **Cloudflare Account** with Workers and R2 enabled
2. **R2 Buckets** created:
   - `mcp-guard-pro-binaries`
   - `mcp-guard-enterprise-binaries`
3. **Secrets** configured (see Configuration section)
4. **Wrangler CLI** installed: `npm install -g wrangler`

## Installation

```bash
cd workers/download-binary
npm install
```

## Configuration

### 1. R2 Buckets

Create two R2 buckets in Cloudflare dashboard:

```bash
# Via wrangler CLI
wrangler r2 bucket create mcp-guard-pro-binaries
wrangler r2 bucket create mcp-guard-enterprise-binaries
```

Or create via Cloudflare Dashboard:
1. Go to R2 → Create Bucket
2. Create `mcp-guard-pro-binaries` (private)
3. Create `mcp-guard-enterprise-binaries` (private)

### 2. Secrets

Set the following secrets using Wrangler:

```bash
# Pro license public key (Ed25519)
# Get this from .keys/pro_license_public.pem (base64 encoded)
cat ../../.keys/pro_license_public.pem | grep -v "BEGIN\|END" | tr -d '\n' | \
  wrangler secret put PRO_LICENSE_PUBLIC_KEY

# Keygen.sh account ID (for Enterprise licenses)
# Get this from your Keygen.sh dashboard
wrangler secret put KEYGEN_ACCOUNT_ID
# Enter your Keygen account ID (UUID format)
```

**Security Note**: Never commit these secrets to git. They are stored securely in Cloudflare's secret storage.

### 3. Environment Variables

Edit `wrangler.toml` if you need to change:
- `KEYGEN_API_URL`: Default is `https://api.keygen.sh/v1/accounts`
- `ENVIRONMENT`: Set to `development` or `production`

## Development

### Local Development

```bash
# Start local dev server with R2 bindings
wrangler dev

# Test health endpoint
curl http://localhost:8787/health

# Test download (requires valid license)
curl "http://localhost:8787/download?tier=pro&platform=x86_64-linux&license=pro_xxx..." \
  -o mcp-guard
```

**Note**: Local development requires binaries to be uploaded to your R2 buckets first.

### Upload Test Binaries

```bash
# Build a test binary
cd ../..
cargo build --release --features pro

# Upload to R2 (via AWS CLI with R2 credentials)
aws s3 cp target/release/mcp-guard \
  s3://mcp-guard-pro-binaries/latest/mcp-guard-x86_64-linux \
  --endpoint-url https://<account-id>.r2.cloudflarestorage.com
```

## Deployment

### Deploy to Production

```bash
# Deploy to production environment
wrangler deploy --env production

# Output will show your Worker URL:
# https://download.mcp-guard.io
```

### Custom Domain

To use a custom domain like `download.mcp-guard.io`:

1. Add route in `wrangler.toml`:
   ```toml
   [env.production]
   routes = [
     { pattern = "download.mcp-guard.io/*", custom_domain = true }
   ]
   ```

2. Deploy:
   ```bash
   wrangler deploy --env production
   ```

3. Cloudflare will automatically configure DNS and SSL.

## API Reference

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "service": "mcp-guard-download"
}
```

### GET /download

Download binary with license validation.

**Query Parameters:**
- `license` (required): License key (`pro_xxx...` or `ent_xxx...`)
- `tier` (required): `pro` or `enterprise`
- `platform` (optional): Platform identifier (auto-detected if not provided)
  - `x86_64-linux` - Linux (glibc)
  - `x86_64-linux-musl` - Linux (musl)
  - `x86_64-darwin` - macOS (Intel)
  - `aarch64-darwin` - macOS (Apple Silicon)
  - `x86_64-windows` - Windows

**Example:**
```bash
# Explicit platform
curl "https://download.mcp-guard.io/download?tier=pro&platform=x86_64-linux&license=pro_xxx..." \
  -o mcp-guard

# Auto-detect platform
curl "https://download.mcp-guard.io/download?tier=pro&license=pro_xxx..." \
  -o mcp-guard
```

**Success Response (200 OK):**
- Binary file streamed as `application/octet-stream`
- Headers:
  - `Content-Disposition: attachment; filename="mcp-guard"` (or `mcp-guard.exe` on Windows)
  - `X-License-Tier: pro` or `enterprise`
  - `X-License-Licensee: user@example.com`

**Error Responses:**

```json
// 400 Bad Request - Missing parameters
{
  "error": "Missing license parameter",
  "message": "Please provide your license key using ?license=YOUR_KEY",
  "documentation": "https://mcp-guard.io/docs/installation"
}

// 403 Forbidden - Invalid license
{
  "error": "License validation failed",
  "message": "License expired on 2024-12-01",
  "tier": "pro",
  "action": "Purchase a Pro license at https://mcp-guard.io/pricing"
}

// 404 Not Found - Binary not available
{
  "error": "Binary not found",
  "message": "Binary for platform 'x86_64-linux' is not available yet",
  "tier": "pro",
  "platform": "x86_64-linux",
  "contact": "Please contact support@mcp-guard.io if this persists"
}

// 500 Internal Server Error
{
  "error": "Internal server error",
  "message": "Error details...",
  "contact": "Please contact support@mcp-guard.io"
}
```

## Testing

### Test Pro License Validation

```bash
# Generate a test Pro license using the signing script
cd ../..
cargo run --bin sign-license -- \
  --tier pro \
  --licensee "test@example.com" \
  --expires-at "2026-12-31T23:59:59Z"

# Copy the generated license key: pro_xxx...

# Test download
curl "https://download.mcp-guard.io/download?tier=pro&platform=x86_64-linux&license=pro_xxx..." \
  -o mcp-guard-test

# Verify binary
chmod +x mcp-guard-test
./mcp-guard-test version
```

### Test Enterprise License Validation

```bash
# Create a test Enterprise license in Keygen.sh dashboard
# Or via Keygen CLI:
keygen licenses create \
  --policy-id <your-policy-id> \
  --name "Test Customer" \
  --metadata '{"email":"test@example.com"}'

# Test download
curl "https://download.mcp-guard.io/download?tier=enterprise&platform=x86_64-darwin&license=ent_xxx..." \
  -o mcp-guard-test
```

### Test Platform Detection

```bash
# macOS (Intel)
curl -A "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)" \
  "https://download.mcp-guard.io/download?tier=pro&license=pro_xxx..." \
  -o mcp-guard

# macOS (Apple Silicon)
curl -A "Mozilla/5.0 (Macintosh; ARM64 Mac OS X 14_0)" \
  "https://download.mcp-guard.io/download?tier=pro&license=pro_xxx..." \
  -o mcp-guard

# Linux
curl -A "Mozilla/5.0 (X11; Linux x86_64)" \
  "https://download.mcp-guard.io/download?tier=pro&license=pro_xxx..." \
  -o mcp-guard

# Windows
curl -A "Mozilla/5.0 (Windows NT 10.0; Win64; x64)" \
  "https://download.mcp-guard.io/download?tier=pro&license=pro_xxx..." \
  -o mcp-guard.exe
```

## Monitoring

### View Logs

```bash
# Tail live logs
wrangler tail

# Filter by status
wrangler tail --status error

# Filter by method
wrangler tail --method GET
```

### Metrics

View metrics in Cloudflare Dashboard:
1. Workers & Pages → mcp-guard-download
2. Metrics tab

Key metrics to monitor:
- **Requests**: Total download requests
- **Errors**: Failed license validations or missing binaries
- **Success Rate**: Should be >95% for valid licenses
- **Duration**: P50/P95/P99 latency (should be <500ms)

## Troubleshooting

### "Binary not found" errors

**Cause**: Binary not uploaded to R2 or wrong object key format.

**Solution**:
1. Check R2 bucket contents:
   ```bash
   wrangler r2 object list mcp-guard-pro-binaries
   ```

2. Verify object key format: `latest/mcp-guard-{platform}`
   - `latest/mcp-guard-x86_64-linux`
   - `latest/mcp-guard.exe-x86_64-windows`

3. Re-upload binary:
   ```bash
   aws s3 cp target/release/mcp-guard \
     s3://mcp-guard-pro-binaries/latest/mcp-guard-x86_64-linux \
     --endpoint-url https://<account-id>.r2.cloudflarestorage.com
   ```

### "Invalid license signature" errors

**Cause**: Public key mismatch or corrupted license.

**Solution**:
1. Verify public key in Cloudflare Secrets matches the one used for signing:
   ```bash
   # Check current secret (won't show value)
   wrangler secret list

   # Re-set if needed
   cat ../../.keys/pro_license_public.pem | grep -v "BEGIN\|END" | tr -d '\n' | \
     wrangler secret put PRO_LICENSE_PUBLIC_KEY
   ```

2. Regenerate test license with matching keypair

### "Keygen validation failed" errors

**Cause**: Incorrect `KEYGEN_ACCOUNT_ID` or network issues.

**Solution**:
1. Verify Keygen account ID in dashboard
2. Update secret:
   ```bash
   wrangler secret put KEYGEN_ACCOUNT_ID
   # Enter your actual UUID
   ```

3. Test Keygen API directly:
   ```bash
   curl "https://api.keygen.sh/v1/accounts/<account-id>/licenses/<license-key>/actions/validate" \
     -X POST \
     -H "Accept: application/json"
   ```

### Platform detection not working

**Cause**: User-Agent header doesn't contain OS information.

**Solution**: Always specify platform explicitly in installation scripts:
```bash
# Detect platform in shell script
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
  Linux-x86_64) PLATFORM="x86_64-linux" ;;
  Darwin-x86_64) PLATFORM="x86_64-darwin" ;;
  Darwin-arm64) PLATFORM="aarch64-darwin" ;;
  *) echo "Unsupported platform"; exit 1 ;;
esac

curl "https://download.mcp-guard.io/download?tier=pro&platform=$PLATFORM&license=$LICENSE" \
  -o mcp-guard
```

## Security Considerations

1. **License Key Protection**: License keys should be treated as secrets by end users
   - Never log license keys in plaintext
   - Recommend users set `MCP_GUARD_LICENSE_KEY` environment variable

2. **R2 Bucket Access**: Buckets must remain private
   - Only accessible via Worker with license validation
   - No public URLs

3. **Secrets Rotation**: Rotate Keygen credentials if compromised
   ```bash
   wrangler secret put KEYGEN_ACCOUNT_ID
   wrangler deploy
   ```

4. **Rate Limiting**: Consider adding rate limiting to prevent abuse
   - Cloudflare Workers KV for tracking download counts per license
   - Return 429 Too Many Requests if exceeded

## Production Checklist

Before deploying to production:

- [ ] R2 buckets created and configured in `wrangler.toml`
- [ ] Secrets set via `wrangler secret put`:
  - [ ] `PRO_LICENSE_PUBLIC_KEY`
  - [ ] `KEYGEN_ACCOUNT_ID`
- [ ] Test binaries uploaded to R2 buckets
- [ ] Custom domain configured (optional)
- [ ] Test Pro license validation works
- [ ] Test Enterprise license validation works
- [ ] Platform detection tested for all 5 platforms
- [ ] Monitoring configured (logs, metrics)
- [ ] Error responses reviewed for security (no sensitive data leaked)
- [ ] Installation scripts updated with Worker URL

## Support

For issues or questions:
- **GitHub**: https://github.com/yourusername/mcp-guard/issues
- **Email**: support@mcp-guard.io
- **Documentation**: https://mcp-guard.io/docs

## License

This Worker is part of the MCP-Guard commercial infrastructure.
Copyright (c) 2025 Austin Green.
