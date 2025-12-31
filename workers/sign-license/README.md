# MCP-Guard License Signing Worker

Cloudflare Worker for generating and signing Pro license keys using Ed25519 cryptographic signatures.

## Overview

This Worker provides a secure API for generating Pro license keys after payment. It:

- **Signs licenses** with Ed25519 private key stored in Cloudflare Secrets
- **Authenticates requests** via API secret key (only authorized services can call it)
- **Generates payload** with tier, expiry, licensee, and features
- **Returns signed license** in format: `pro_<base64-payload>.<base64-signature>`

## Security Model

**Critical**: This Worker has access to the Ed25519 **private key**, which is used to sign all Pro licenses. It must be protected:

1. **API Secret Key**: All requests must include `X-API-Secret` header
2. **Private Secrets**: Private key stored in Cloudflare Secrets (never in code/config)
3. **Authorized Callers Only**: Only Stripe webhook handler and admin portal should call this
4. **HTTPS Only**: Worker is only accessible via HTTPS
5. **No Public Access**: Not listed in public documentation

## Prerequisites

1. **Ed25519 Keypair**: Generated via `scripts/generate_license_keypair.sh`
2. **Cloudflare Account**: With Workers enabled
3. **Wrangler CLI**: Installed (`npm install -g wrangler`)

## Installation

```bash
cd workers/sign-license
npm install
```

## Configuration

### 1. Set Secrets

```bash
# Ed25519 private key (from .keys/pro_license_private.pem)
cat ../../.keys/pro_license_private.pem | \
  wrangler secret put PRO_LICENSE_PRIVATE_KEY

# API secret key (generate a strong random key)
openssl rand -hex 32 | wrangler secret put API_SECRET_KEY
# Save this key securely - you'll need it for Stripe webhook and admin portal
```

**Security Note**:
- Never commit these secrets to git
- Store `API_SECRET_KEY` in password manager for use by Stripe webhook
- Rotate `API_SECRET_KEY` if compromised (requires updating Stripe webhook)

### 2. Environment Variables

Edit `wrangler.toml` if needed:
- `ENVIRONMENT`: Set to `development` or `production`

## Development

### Local Development

```bash
# Start local dev server
wrangler dev

# Test health endpoint
curl http://localhost:8787/health

# Test license signing (requires API secret)
curl -X POST http://localhost:8787/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: your-api-secret-key" \
  -d '{
    "licensee": "customer@example.com",
    "expires_at": "2026-12-31T23:59:59Z"
  }'
```

## Deployment

### Deploy to Production

```bash
wrangler deploy --env production
# Deploys to https://sign.mcp-guard.io
```

### Custom Domain

To use custom domain `sign.mcp-guard.io`:

1. Add route in `wrangler.toml`:
   ```toml
   [env.production]
   routes = [
     { pattern = "sign.mcp-guard.io/*", custom_domain = true }
   ]
   ```

2. Deploy:
   ```bash
   wrangler deploy --env production
   ```

3. Cloudflare automatically configures DNS and SSL.

## API Reference

### POST /sign

Generate and sign a Pro license.

**Authentication**: Required via `X-API-Secret` header

**Request Headers:**
```
Content-Type: application/json
X-API-Secret: <your-api-secret-key>
```

**Request Body:**
```json
{
  "licensee": "customer@example.com",
  "expires_at": "2026-12-31T23:59:59Z",  // Optional, defaults to 1 year
  "features": ["oauth", "http_transport"]  // Optional, defaults to all Pro features
}
```

**Fields:**
- `licensee` (required): Customer email or name
- `expires_at` (optional): ISO 8601 timestamp, defaults to 1 year from now
- `features` (optional): Array of feature names, defaults to all Pro features

**Default Pro Features:**
- `oauth` - OAuth 2.1 authentication
- `http_transport` - HTTP transport for MCP
- `sse_transport` - Server-Sent Events transport
- `per_identity_rate_limit` - Per-identity rate limiting
- `jwt_jwks` - JWT with JWKS support

**Success Response (200 OK):**
```json
{
  "license_key": "pro_eyJ0aWVyIjoicHJvIiwiaXNzdWVkX2F0IjoiMjAyNS0xMi0zMVQxMjowMDowMFoiLCJleHBpcmVzX2F0IjoiMjAyNi0xMi0zMVQyMzo1OTo1OVoiLCJsaWNlbnNlZSI6ImN1c3RvbWVyQGV4YW1wbGUuY29tIiwiZmVhdHVyZXMiOlsib2F1dGgiLCJodHRwX3RyYW5zcG9ydCIsInNzZV90cmFuc3BvcnQiLCJwZXJfaWRlbnRpdHlfcmF0ZV9saW1pdCIsImp3dF9qd2tzIl19.z9vN2h8pL6xQ4kR3mW5sT1uE7oY8aB6cD9fG2jK4lM3nO5pQ7rS9tU1vW3xY5zA",
  "payload": {
    "tier": "pro",
    "issued_at": "2025-12-31T12:00:00Z",
    "expires_at": "2026-12-31T23:59:59Z",
    "licensee": "customer@example.com",
    "features": ["oauth", "http_transport", "sse_transport", "per_identity_rate_limit", "jwt_jwks"]
  },
  "expires_at": "2026-12-31T23:59:59Z"
}
```

**Error Responses:**

```json
// 401 Unauthorized - Missing or invalid API secret
{
  "error": "Unauthorized",
  "message": "Invalid or missing API secret key"
}

// 400 Bad Request - Missing required field
{
  "error": "Invalid request",
  "message": "Missing or invalid \"licensee\" field (must be a string)"
}

// 400 Bad Request - Invalid expiry
{
  "error": "Invalid request",
  "message": "expires_at must be in the future"
}

// 500 Internal Server Error
{
  "error": "Internal server error",
  "message": "Failed to sign license",
  "details": "Error details..."
}
```

### GET /health

Health check endpoint (no authentication required).

**Response:**
```json
{
  "status": "ok",
  "service": "mcp-guard-license-signer"
}
```

## Usage Examples

### cURL

```bash
# Set API secret
export API_SECRET="your-api-secret-key"

# Generate license with default expiry (1 year)
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{
    "licensee": "customer@example.com"
  }'

# Generate license with custom expiry
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{
    "licensee": "enterprise-customer@example.com",
    "expires_at": "2027-12-31T23:59:59Z"
  }'

# Generate license with specific features
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{
    "licensee": "trial@example.com",
    "expires_at": "2025-02-01T00:00:00Z",
    "features": ["oauth", "http_transport"]
  }'
```

### JavaScript (Stripe Webhook)

```typescript
// In Stripe webhook handler
async function generateProLicense(customerEmail: string): Promise<string> {
  const response = await fetch('https://sign.mcp-guard.io/sign', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-Secret': env.LICENSE_SIGNER_API_SECRET
    },
    body: JSON.stringify({
      licensee: customerEmail,
      expires_at: new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString()
    })
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(`License signing failed: ${error.message}`);
  }

  const data = await response.json();
  return data.license_key;
}
```

### Python (Admin Script)

```python
import requests
from datetime import datetime, timedelta

def generate_license(email: str, api_secret: str) -> dict:
    expires_at = (datetime.now() + timedelta(days=365)).isoformat()

    response = requests.post(
        'https://sign.mcp-guard.io/sign',
        headers={
            'Content-Type': 'application/json',
            'X-API-Secret': api_secret
        },
        json={
            'licensee': email,
            'expires_at': expires_at
        }
    )

    response.raise_for_status()
    return response.json()

# Usage
api_secret = os.environ['LICENSE_SIGNER_API_SECRET']
license_data = generate_license('customer@example.com', api_secret)
print(f"License key: {license_data['license_key']}")
```

## Testing

### Test License Generation

```bash
# Generate test license
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{
    "licensee": "test@example.com",
    "expires_at": "2026-12-31T23:59:59Z"
  }' | jq .

# Extract license key
LICENSE_KEY=$(curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: $API_SECRET" \
  -d '{"licensee": "test@example.com"}' | jq -r .license_key)

echo "License: $LICENSE_KEY"

# Test license with MCP-Guard binary
export MCP_GUARD_LICENSE_KEY="$LICENSE_KEY"
mcp-guard version
# Should show: "Tier: Pro"
```

### Test Authentication

```bash
# Should fail without API secret
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -d '{"licensee": "test@example.com"}'
# Expected: 401 Unauthorized

# Should fail with invalid API secret
curl -X POST https://sign.mcp-guard.io/sign \
  -H "Content-Type: application/json" \
  -H "X-API-Secret: wrong-secret" \
  -d '{"licensee": "test@example.com"}'
# Expected: 401 Unauthorized
```

## Monitoring

### View Logs

```bash
# Tail live logs
wrangler tail

# Filter errors
wrangler tail --status error
```

### Metrics

View metrics in Cloudflare Dashboard:
1. Workers & Pages → mcp-guard-license-signer
2. Metrics tab

Key metrics:
- **Requests**: Total license generation requests
- **Errors**: Failed signings (should be near zero)
- **Unauthorized**: 401 responses (monitor for abuse)
- **Duration**: P95 should be <100ms

### Alerts

Set up alerts for:
- Error rate >1%
- Unauthorized requests >10/hour (potential abuse)
- Worker failures

## Security Considerations

### Private Key Protection

1. **Never Expose**: Private key must NEVER be logged, returned in responses, or committed to git
2. **Cloudflare Secrets**: Only storage location for private key
3. **Rotation**: If private key is compromised:
   - Generate new keypair
   - Update public key in mcp-guard binaries
   - Update private key secret in Worker
   - Invalidate all existing Pro licenses (require renewal)

### API Secret Key

1. **Strong Random**: Use `openssl rand -hex 32` or similar
2. **Secure Storage**: Store in password manager
3. **Limited Distribution**: Only give to:
   - Stripe webhook handler
   - Admin portal
   - Trusted automation scripts
4. **Rotation**: If compromised:
   - Generate new secret
   - Update Worker secret
   - Update all callers (Stripe webhook, admin portal)

### Request Validation

Worker validates:
- API secret header presence and correctness
- Request body is valid JSON
- `licensee` field is present and non-empty
- `expires_at` is in the future (if provided)
- All inputs before signing

### Rate Limiting

Consider adding rate limiting:
```typescript
// Example: Max 100 license generations per hour per API secret
// Implementation left as exercise - use Workers KV or Durable Objects
```

## Troubleshooting

### "Invalid or missing API secret key"

**Cause**: `X-API-Secret` header missing or incorrect.

**Solution**:
1. Verify API secret in password manager
2. Check header name is exactly `X-API-Secret`
3. Verify secret matches what was set via `wrangler secret put`

### "Failed to parse Ed25519 private key"

**Cause**: Private key secret is corrupted or wrong format.

**Solution**:
1. Verify private key file exists: `ls -la .keys/pro_license_private.pem`
2. Re-set secret:
   ```bash
   cat .keys/pro_license_private.pem | wrangler secret put PRO_LICENSE_PRIVATE_KEY
   ```
3. Verify key format:
   ```bash
   openssl pkey -in .keys/pro_license_private.pem -text
   # Should show: ED25519 Private-Key
   ```

### "expires_at must be in the future"

**Cause**: Provided expiry date is in the past.

**Solution**: Use future ISO 8601 timestamp:
```bash
# Generate timestamp for 1 year from now
date -u -d "+1 year" +"%Y-%m-%dT%H:%M:%SZ"
```

### Generated licenses fail validation

**Cause**: Public key mismatch or signature format issue.

**Solution**:
1. Verify public key in binaries matches private key:
   ```bash
   # Extract public key from private key
   openssl pkey -in .keys/pro_license_private.pem -pubout

   # Should match the base64 in mcp-guard-pro/src/license.rs
   ```

2. Test end-to-end:
   ```bash
   # Generate license
   LICENSE=$(curl -X POST https://sign.mcp-guard.io/sign \
     -H "Content-Type: application/json" \
     -H "X-API-Secret: $API_SECRET" \
     -d '{"licensee": "test@example.com"}' | jq -r .license_key)

   # Test with binary
   export MCP_GUARD_LICENSE_KEY="$LICENSE"
   mcp-guard version
   ```

## Production Checklist

Before deploying to production:

- [ ] Ed25519 keypair generated and secured
- [ ] Private key set via `wrangler secret put PRO_LICENSE_PRIVATE_KEY`
- [ ] Strong API secret generated and set via `wrangler secret put API_SECRET_KEY`
- [ ] API secret stored in password manager
- [ ] API secret provided to Stripe webhook handler
- [ ] Custom domain configured (optional)
- [ ] Test license generation works
- [ ] Test license validation works in binary
- [ ] Monitoring configured (logs, metrics, alerts)
- [ ] Private key backup stored securely offline
- [ ] Emergency key rotation procedure documented

## Emergency Procedures

### Key Rotation

If private key is compromised:

1. **Immediate**:
   ```bash
   # Generate new keypair
   ./scripts/generate_license_keypair.sh

   # Update Worker secret
   cat .keys/pro_license_private.pem | wrangler secret put PRO_LICENSE_PRIVATE_KEY
   wrangler deploy --env production
   ```

2. **Within 24 hours**:
   - Update public key in `mcp-guard-pro/src/license.rs`
   - Release new binaries with new public key
   - Notify customers to update binaries

3. **Follow-up**:
   - Invalidate all old licenses in customer database
   - Contact all Pro customers for license renewal
   - Review access logs to identify breach source

### Worker Compromise

If Worker is compromised:

1. **Immediate**:
   ```bash
   # Rotate API secret
   openssl rand -hex 32 | wrangler secret put API_SECRET_KEY

   # Redeploy
   wrangler deploy --env production
   ```

2. **Update all callers**:
   - Stripe webhook handler
   - Admin portal
   - Any automation scripts

## Support

For issues or questions:
- **GitHub**: https://github.com/yourusername/mcp-guard/issues
- **Email**: support@mcp-guard.io
- **Internal**: This Worker is internal infrastructure - not customer-facing

## License

This Worker is part of the MCP-Guard commercial infrastructure.
Copyright (c) 2025 Austin Green.
