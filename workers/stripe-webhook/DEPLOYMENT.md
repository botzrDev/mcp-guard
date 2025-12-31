# Quick Deployment Guide - Stripe Webhook Handler

## One-Time Setup

### 1. Create KV Namespace

```bash
cd workers/stripe-webhook

# Create KV namespace
wrangler kv:namespace create LICENSES

# Copy the namespace ID from output, update wrangler.toml
# [[kv_namespaces]]
# binding = "LICENSES"
# id = "your-namespace-id-here"
```

### 2. Set Secrets

```bash
# Stripe webhook secret (get from Stripe Dashboard after creating webhook)
wrangler secret put STRIPE_WEBHOOK_SECRET

# License signing Worker URL
wrangler secret put LICENSE_SIGNER_URL
# Enter: https://sign.mcp-guard.io

# License signing Worker API secret (from password manager)
wrangler secret put LICENSE_SIGNER_API_SECRET

# Resend API key (from resend.com dashboard)
wrangler secret put RESEND_API_KEY
```

### 3. Install Dependencies

```bash
npm install
```

### 4. Configure Stripe Webhook

1. Go to Stripe Dashboard → Developers → Webhooks
2. Click "Add endpoint"
3. URL: `https://webhook.mcp-guard.io/webhook` (or your custom domain)
4. Events to send:
   - `checkout.session.completed`
   - `customer.subscription.deleted`
   - `customer.subscription.updated`
   - `invoice.payment_failed`
5. Click "Add endpoint"
6. Copy "Signing secret" (whsec_...)
7. Set secret: `wrangler secret put STRIPE_WEBHOOK_SECRET`

### 5. Configure Resend

1. Go to https://resend.com → API Keys
2. Create API key with "Sending access"
3. Copy API key (re_...)
4. Set secret: `wrangler secret put RESEND_API_KEY`
5. Add domain: licenses@mcp-guard.io
6. Verify domain (add DNS records)

## Deploy

### Development

```bash
# Local development with Stripe CLI
stripe listen --forward-to localhost:8787/webhook
wrangler dev
```

### Production

```bash
wrangler deploy --env production
# Deploys to https://webhook.mcp-guard.io
```

## Test

### Test with Stripe CLI

```bash
# Install Stripe CLI
brew install stripe/stripe-cli/stripe
stripe login

# Trigger test event
stripe trigger checkout.session.completed

# View logs
wrangler tail
```

### Test End-to-End

1. Create test product in Stripe Dashboard (test mode)
2. Create checkout session
3. Complete checkout with test card: `4242 4242 4242 4242`
4. Verify:
   - License generated (check Worker logs)
   - Email sent (check inbox)
   - License stored in KV: `wrangler kv:key list --namespace-id=<id>`
   - License validates: `export MCP_GUARD_LICENSE_KEY="pro_xxx..." && mcp-guard version`

## Monitor

```bash
# Live logs
wrangler tail

# List licenses
wrangler kv:key list --namespace-id=<your-namespace-id>

# Get specific license
wrangler kv:key get "customer:cus_xxx" --namespace-id=<your-namespace-id>
```

## Troubleshooting

**"Invalid signature"**
- Verify webhook secret matches Stripe Dashboard
- Re-set: `wrangler secret put STRIPE_WEBHOOK_SECRET`

**"License generation failed"**
- Verify license signing Worker is deployed: `curl https://sign.mcp-guard.io/health`
- Verify API secret matches: `wrangler secret list`

**"Email sending failed"**
- Verify Resend domain is verified
- Test Resend API key manually:
  ```bash
  curl https://api.resend.com/emails \
    -H "Authorization: Bearer $RESEND_API_KEY" \
    -H "Content-Type: application/json" \
    -d '{"from":"licenses@mcp-guard.io","to":"test@example.com","subject":"Test","html":"<p>Test</p>"}'
  ```

**Webhooks not received**
- Verify Stripe webhook URL is correct
- Test Worker health: `curl https://webhook.mcp-guard.io/health`
- Check Stripe Dashboard → Webhooks for delivery errors

## Custom Domain Setup

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

Update Stripe webhook URL to use custom domain.

## Rollback

```bash
# Rollback to previous version
wrangler rollback
```

## Production Checklist

- [ ] KV namespace created
- [ ] All secrets set
- [ ] Stripe webhook configured
- [ ] Resend domain verified
- [ ] License signing Worker deployed
- [ ] Test checkout completed successfully
- [ ] Email received
- [ ] License validates
