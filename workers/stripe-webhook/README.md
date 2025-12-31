# MCP-Guard Stripe Webhook Handler

Cloudflare Worker for processing Stripe webhook events and automating Pro license delivery.

## Overview

This Worker handles the complete Pro tier payment-to-license flow:

1. **Payment** → Stripe checkout completed
2. **License Generation** → Calls license signing Worker
3. **Storage** → Saves license data to Workers KV
4. **Email** → Sends license key via Resend.com
5. **Lifecycle** → Handles subscriptions, cancellations, renewals

## Architecture

```
Stripe → Webhook → [Verify Signature] → [Generate License] → [Send Email] → [Store in KV]
                                              ↓
                                    License Signing Worker
```

## Features

- **Webhook Verification**: Validates Stripe signatures for security
- **Automatic License Generation**: No manual intervention required
- **Email Delivery**: Professional HTML emails with license keys
- **Subscription Management**: Handles renewals and cancellations
- **Payment Failure Handling**: Notifies customers of failed payments
- **License Storage**: Persistent storage in Workers KV

## Prerequisites

1. **Stripe Account**: https://stripe.com
2. **Resend Account**: https://resend.com (for sending emails)
3. **License Signing Worker**: Deployed and accessible
4. **Workers KV Namespace**: For storing license data
5. **Wrangler CLI**: `npm install -g wrangler`

## Installation

```bash
cd workers/stripe-webhook
npm install
```

## Configuration

### 1. Create KV Namespace

```bash
# Create KV namespace
wrangler kv:namespace create LICENSES

# Output will show namespace ID, copy it to wrangler.toml
# Example: id = "abc123..."
```

Edit `wrangler.toml` and update the KV namespace ID:
```toml
[[kv_namespaces]]
binding = "LICENSES"
id = "your-namespace-id-here"  # Replace with actual ID from above
```

### 2. Set Secrets

```bash
# Stripe webhook secret (from Stripe Dashboard → Webhooks)
wrangler secret put STRIPE_WEBHOOK_SECRET
# Paste your webhook signing secret (whsec_...)

# License signing Worker URL
wrangler secret put LICENSE_SIGNER_URL
# Enter: https://sign.mcp-guard.io

# License signing Worker API secret (from password manager)
wrangler secret put LICENSE_SIGNER_API_SECRET
# Paste the API secret from license signing Worker setup

# Resend API key (from resend.com dashboard)
wrangler secret put RESEND_API_KEY
# Paste your Resend API key (re_...)
```

### 3. Configure Stripe Webhook

1. Go to Stripe Dashboard → Developers → Webhooks
2. Click "Add endpoint"
3. URL: `https://webhook.mcp-guard.io/webhook`
4. Events to send:
   - `checkout.session.completed`
   - `customer.subscription.deleted`
   - `customer.subscription.updated`
   - `invoice.payment_failed`
5. Click "Add endpoint"
6. Copy the "Signing secret" (whsec_...) and set it via `wrangler secret put STRIPE_WEBHOOK_SECRET`

### 4. Configure Resend

1. Go to https://resend.com → API Keys
2. Create new API key with "Sending access"
3. Copy API key (re_...) and set via `wrangler secret put RESEND_API_KEY`
4. Add and verify your sending domain (licenses@mcp-guard.io)

## Development

### Local Development

```bash
# Start local dev server
wrangler dev

# Test health endpoint
curl http://localhost:8787/health
```

### Test with Stripe CLI

```bash
# Install Stripe CLI
brew install stripe/stripe-cli/stripe

# Login to Stripe
stripe login

# Forward webhooks to local dev server
stripe listen --forward-to localhost:8787/webhook

# Trigger test event
stripe trigger checkout.session.completed
```

## Deployment

### Deploy to Production

```bash
wrangler deploy --env production
# Deploys to https://webhook.mcp-guard.io
```

### Custom Domain

Edit `wrangler.toml`:
```toml
[env.production]
routes = [
  { pattern = "webhook.mcp-guard.io/*", custom_domain = true }
]
```

Deploy:
```bash
wrangler deploy --env production
```

## Event Handling

### checkout.session.completed

**Triggered**: Customer completes Pro purchase

**Actions**:
1. Extract customer email from session
2. Generate Pro license (1-year expiry for subscription, 30-day for one-time)
3. Store license in KV:
   - Key: `customer:{stripe_customer_id}`
   - Key: `license:{license_key}`
4. Send welcome email with license key and installation instructions

**Metadata**:
- `tier`: "pro" or "enterprise" (optional, defaults to "pro")

### customer.subscription.deleted

**Triggered**: Customer cancels subscription

**Actions**:
1. Retrieve license from KV
2. Mark status as "cancelled"
3. Update KV records
4. Send cancellation email with expiry date

**Note**: License remains valid until expiry date, not immediately revoked.

### customer.subscription.updated

**Triggered**: Subscription renews or status changes

**Actions**:
1. If status is "active":
   - Update license expiry to match subscription period end
   - Mark status as "active"
   - Update KV records

### invoice.payment_failed

**Triggered**: Subscription renewal payment fails

**Actions**:
1. Retrieve license from KV
2. Send payment failure email with invoice link
3. Customer has grace period to update payment method

## API Reference

### POST /webhook

Stripe webhook endpoint.

**Headers**:
- `Stripe-Signature`: Stripe webhook signature (added by Stripe)

**Request Body**: Stripe event JSON

**Response**:
```json
{
  "received": true
}
```

### GET /health

Health check endpoint.

**Response**:
```json
{
  "status": "ok",
  "service": "mcp-guard-stripe-webhook"
}
```

## Data Storage

### KV Schema

**Key**: `customer:{stripe_customer_id}`
**Value**:
```json
{
  "license_key": "pro_xxx...",
  "customer_id": "cus_xxx",
  "customer_email": "customer@example.com",
  "tier": "pro",
  "status": "active",
  "issued_at": "2025-12-31T12:00:00Z",
  "expires_at": "2026-12-31T12:00:00Z",
  "subscription_id": "sub_xxx"
}
```

**Key**: `license:{license_key}`
**Value**: Same as above (allows lookup by either customer ID or license key)

## Testing

### Test Webhook with Stripe CLI

```bash
# Test checkout completed
stripe trigger checkout.session.completed

# Test subscription cancelled
stripe trigger customer.subscription.deleted

# Test payment failed
stripe trigger invoice.payment_failed

# View logs
wrangler tail
```

### Manual Test

```bash
# Create test checkout session in Stripe Dashboard
# Complete checkout with test card: 4242 4242 4242 4242

# Check KV for license
wrangler kv:key list --namespace-id=<your-namespace-id>

# Get license data
wrangler kv:key get "customer:cus_xxx" --namespace-id=<your-namespace-id>

# Check email was sent (check inbox for licenses@mcp-guard.io)
```

### Test License Validation

```bash
# After receiving license email, test license works
export MCP_GUARD_LICENSE_KEY="pro_xxx..."
mcp-guard version
# Should show: "Tier: Pro"
```

## Email Templates

### Welcome Email (License Delivery)

- **Subject**: "Your MCP-Guard Pro License Key"
- **Content**:
  - Welcome message
  - License key in monospace box
  - One-line installation command
  - List of Pro features
  - Links to documentation and support

### Cancellation Email

- **Subject**: "MCP-Guard Pro Subscription Cancelled"
- **Content**:
  - Confirmation of cancellation
  - License expiry date
  - Link to resubscribe
  - Support contact

### Payment Failed Email

- **Subject**: "MCP-Guard Pro - Payment Failed"
- **Content**:
  - Payment failure notice
  - Link to update payment method (Stripe invoice URL)
  - Support contact

## Monitoring

### View Logs

```bash
# Tail live logs
wrangler tail

# Filter by event type
wrangler tail | grep "checkout.session.completed"

# Filter errors
wrangler tail --status error
```

### Metrics

Track in Cloudflare Dashboard:
- **Total webhooks received**: Count of POST /webhook requests
- **License generations**: Count of checkout.session.completed events
- **Email send failures**: Count of Resend API errors
- **Signature verification failures**: Count of 400 responses

### KV Usage

```bash
# List all licenses
wrangler kv:key list --namespace-id=<your-namespace-id>

# Count active licenses
wrangler kv:key list --namespace-id=<your-namespace-id> | grep customer | wc -l

# Get specific license
wrangler kv:key get "customer:cus_xxx" --namespace-id=<your-namespace-id>
```

## Troubleshooting

### "Invalid signature" errors

**Cause**: Stripe signature verification failed

**Solution**:
1. Verify webhook secret in Stripe Dashboard matches secret in Worker
2. Re-set secret:
   ```bash
   wrangler secret put STRIPE_WEBHOOK_SECRET
   ```
3. Ensure Stripe webhook URL is correct: `https://webhook.mcp-guard.io/webhook`

### License generation fails

**Cause**: License signing Worker is unreachable or returns error

**Solution**:
1. Verify `LICENSE_SIGNER_URL` is correct:
   ```bash
   wrangler secret list
   ```
2. Test license signing Worker directly:
   ```bash
   curl https://sign.mcp-guard.io/health
   ```
3. Verify `LICENSE_SIGNER_API_SECRET` matches

### Emails not sending

**Cause**: Resend API key invalid or domain not verified

**Solution**:
1. Verify Resend API key:
   ```bash
   curl https://api.resend.com/emails \
     -H "Authorization: Bearer $RESEND_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{"from":"licenses@mcp-guard.io","to":"test@example.com","subject":"Test","html":"<p>Test</p>"}'
   ```
2. Check domain verification in Resend dashboard
3. Verify `FROM_EMAIL` in `wrangler.toml` matches verified domain

### Webhooks not received

**Cause**: Stripe webhook not configured or Worker not deployed

**Solution**:
1. Verify webhook endpoint in Stripe Dashboard
2. Check Worker is deployed:
   ```bash
   curl https://webhook.mcp-guard.io/health
   ```
3. Test with Stripe CLI:
   ```bash
   stripe trigger checkout.session.completed
   ```

### KV data not found

**Cause**: KV namespace ID mismatch

**Solution**:
1. Verify namespace ID in `wrangler.toml`:
   ```bash
   wrangler kv:namespace list
   ```
2. Update `id` in `wrangler.toml` if needed
3. Redeploy:
   ```bash
   wrangler deploy --env production
   ```

## Security Considerations

### Webhook Signature Verification

**Critical**: Always verify Stripe signatures before processing events.

- Worker MUST verify signature using `stripe.webhooks.constructEvent()`
- Invalid signatures return 400 error
- Protects against replay attacks and unauthorized events

### Secrets Management

1. **Never log secrets**: License keys, API keys, webhook secrets
2. **Rotate regularly**: If secrets are compromised
3. **Limit access**: Only authorized team members

### Email Security

1. **SPF/DKIM**: Configure for licenses@mcp-guard.io
2. **Rate limiting**: Monitor for email abuse
3. **Unsubscribe**: Include in transactional emails (optional)

### KV Data Protection

1. **No PII in keys**: Use Stripe customer IDs, not emails
2. **Encrypt sensitive data**: Consider encrypting license data (future enhancement)
3. **Access control**: Limit KV namespace access to this Worker only

## Production Checklist

Before deploying to production:

- [ ] KV namespace created and ID set in `wrangler.toml`
- [ ] All secrets set via `wrangler secret put`:
  - [ ] `STRIPE_WEBHOOK_SECRET`
  - [ ] `LICENSE_SIGNER_URL`
  - [ ] `LICENSE_SIGNER_API_SECRET`
  - [ ] `RESEND_API_KEY`
- [ ] Stripe webhook endpoint configured with correct URL
- [ ] Stripe webhook events selected (checkout, subscription, invoice)
- [ ] Resend domain verified (licenses@mcp-guard.io)
- [ ] License signing Worker deployed and accessible
- [ ] Custom domain configured (optional)
- [ ] Test end-to-end flow with Stripe test mode
- [ ] Monitor logs during first real purchase
- [ ] Email templates reviewed for branding
- [ ] Support email address monitored (replies to license emails)

## Stripe Test Cards

For testing in Stripe test mode:

| Card | Use Case |
|------|----------|
| 4242 4242 4242 4242 | Successful payment |
| 4000 0000 0000 0002 | Declined payment |
| 4000 0000 0000 3220 | 3D Secure required |

**Note**: Use any future expiry date and any 3-digit CVC.

## Support

For issues or questions:
- **GitHub**: https://github.com/yourusername/mcp-guard/issues
- **Email**: support@mcp-guard.io
- **Stripe**: https://stripe.com/docs/webhooks

## License

This Worker is part of the MCP-Guard commercial infrastructure.
Copyright (c) 2025 Austin Green.
