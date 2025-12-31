# Keygen.sh Setup for Enterprise Licenses

This guide explains how to configure Keygen.sh for Enterprise license validation.

## Overview

Enterprise licenses use online validation through [Keygen.sh](https://keygen.sh), a commercial license key API service. This provides:

- **Online validation** with offline grace period (30 days)
- **Seat management** for team/user limits
- **Auto-expiry** and renewal tracking
- **Analytics** on license usage
- **Revocation** capability for security

## Prerequisites

1. Keygen.sh account (free trial available)
2. Product created in Keygen dashboard
3. Keygen API account ID

## Setup Steps

### 1. Create Keygen.sh Account

1. Visit https://keygen.sh and sign up
2. Create a new **Account**
3. Note your **Account ID** (format: `UUID`)

### 2. Create Product

1. In Keygen dashboard, go to **Products**
2. Click **Create Product**
3. Configure:
   - **Name:** MCP-Guard Enterprise
   - **Distribution strategy:** Licensed
   - **Platform:** any

### 3. Create License Policy

1. Go to **Policies** → **Create Policy**
2. Configure:
   - **Name:** Enterprise Tier
   - **Duration:** 1 year (or custom)
   - **Max machines:** Configure per customer
   - **Max users:** Based on seats purchased
   - **Concurrent processes:** 1 (or as needed)

### 4. Update MCP-Guard Configuration

Edit `crates/mcp-guard-enterprise/src/license.rs`:

```rust
/// Account ID for mcp-guard on keygen.sh
const KEYGEN_ACCOUNT_ID: &str = "YOUR_KEYGEN_ACCOUNT_ID_HERE";
```

Replace `YOUR_KEYGEN_ACCOUNT_ID_HERE` with your actual Account ID from step 1.

### 5. Generate License Keys

When a customer purchases Enterprise, generate a license in Keygen:

```bash
# Using Keygen CLI
keygen licenses create \
  --policy-id <policy-id> \
  --name "Customer Name" \
  --metadata '{"email":"customer@example.com","seats":10}'

# Or via API
curl https://api.keygen.sh/v1/accounts/<account-id>/licenses \
  -H "Authorization: Bearer <admin-api-key>" \
  -H "Content-Type: application/json" \
  -d '{
    "data": {
      "type": "licenses",
      "attributes": {
        "name": "Customer Name",
        "metadata": {"email":"customer@example.com"}
      },
      "relationships": {
        "policy": {"data":{"type":"policies","id":"<policy-id>"}}
      }
    }
  }'
```

The license key format will be: `ent_<random-key>`

### 6. Deliver License to Customer

Send the customer their license key with activation instructions:

```
Your MCP-Guard Enterprise License
==================================

License Key: ent_ABC123XYZ...
Licensed to: customer@example.com
Seats: 10 users
Expires: 2026-12-31

Activation:
1. Set environment variable:
   export MCP_GUARD_LICENSE_KEY="ent_ABC123XYZ..."

2. Start MCP-Guard:
   mcp-guard run

3. License will be validated online once, then cached for 30 days

Support: austin@botzr.dev
```

## Offline Grace Period

Enterprise deployments can run offline for up to 30 days after successful online validation. This allows:

- Air-gapped environments (with initial online activation)
- Temporary network outages
- Travel/remote work scenarios

After 30 days offline, the license must revalidate online.

## Testing License Validation

Create a test license in Keygen:

```bash
# In your local environment
export MCP_GUARD_LICENSE_KEY="ent_test_key_from_keygen"

# Run with enterprise features
cargo run --features enterprise -- run

# Should see:
# INFO Enterprise license validated: Test Customer
# INFO Using cached license validation (age: 0 days)
```

## Troubleshooting

### Error: "Invalid license key format"
- License must start with `ent_`
- Check for extra spaces/newlines in environment variable

### Error: "License validation failed: Network error"
- First-time activation requires internet
- Check firewall allows HTTPS to api.keygen.sh

### Error: "Offline grace period expired"
- License cache is > 30 days old
- Connect to internet to revalidate

### Error: "License has expired"
- License expiry date has passed
- Contact customer to renew license

## Security Best Practices

1. **Protect Admin Keys:** Never commit Keygen admin API keys to git
2. **Rate Limiting:** Keygen has API rate limits, cache aggressively
3. **Webhook Validation:** Set up webhooks for license changes/revocations
4. **Audit Logging:** Track license validation events

## Production Checklist

- [ ] Keygen account created and verified
- [ ] Product and policy configured
- [ ] Account ID updated in source code
- [ ] Test license generated and validated
- [ ] License delivery process documented
- [ ] Customer support trained on license issues
- [ ] Webhook endpoints configured (optional)
- [ ] Monitoring for validation failures

## Cost Estimate

Keygen.sh pricing (as of 2025):

- **Starter:** $49/month - 500 licenses
- **Growth:** $199/month - 2,000 licenses
- **Business:** $499/month - 10,000 licenses

Choose based on expected customer volume.

## References

- [Keygen API Docs](https://keygen.sh/docs/api/)
- [License Policies](https://keygen.sh/docs/choosing-a-licensing-model/)
- [Machine Activation](https://keygen.sh/docs/api/machines/)
