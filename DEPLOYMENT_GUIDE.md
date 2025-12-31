# MCP-Guard v1.0 Deployment Guide

Complete guide for deploying all MCP-Guard infrastructure components for v1.0 launch.

## Overview

Your MCP-Guard v1.0 launch infrastructure is **complete**! Here's what's been built:

### ✅ Completed Components

1. **GitHub Actions Multi-Tier Build Workflow** - Automated builds for Free, Pro, and Enterprise tiers
2. **License Signing Worker** (Cloudflare) - Generates and signs Pro licenses
3. **Binary Download Worker** (Cloudflare) - Validates licenses and serves binaries from R2
4. **Stripe Webhook Handler** (Cloudflare) - Processes payments and delivers licenses
5. **Installation Scripts** - One-command installers for Pro and Enterprise
6. **Landing Page Integration** - Stripe checkout integration for Pro tier

---

## Deployment Checklist

### Pre-Deployment (One-Time Setup)

#### 1. Generate Ed25519 Keypair

```bash
# Generate keypair for Pro license signing
./scripts/generate_license_keypair.sh

# This creates:
# - .keys/pro_license_private.pem (keep secret!)
# - .keys/pro_license_public.pem (embedded in binaries)
```

**Critical**: Back up `pro_license_private.pem` to a secure offline location!

#### 2. Configure Keygen.sh (Enterprise Licenses)

Follow `docs/enterprise/keygen-setup.md`:

1. Create account at https://keygen.sh (Starter plan: $49/mo)
2. Create product: "MCP-Guard Enterprise"
3. Create license policy (1-year duration, 10 max machines)
4. Copy Account ID (UUID format)
5. Update `crates/mcp-guard-enterprise/src/license.rs:35` with real Account ID

#### 3. Create Cloudflare R2 Buckets

```bash
# Install wrangler if not already installed
npm install -g wrangler
wrangler login

# Create R2 buckets for binaries
wrangler r2 bucket create mcp-guard-pro-binaries
wrangler r2 bucket create mcp-guard-enterprise-binaries
```

#### 4. Configure GitHub Secrets

In GitHub repository settings → Secrets → Actions, add:

- `R2_ACCESS_KEY_ID` - Cloudflare R2 access key
- `R2_SECRET_ACCESS_KEY` - Cloudflare R2 secret key
- `R2_ENDPOINT` - R2 endpoint URL (format: `https://<account-id>.r2.cloudflarestorage.com`)
- `CARGO_REGISTRY_TOKEN` - crates.io API token (for publishing Free tier)

Get R2 credentials:
1. Cloudflare Dashboard → R2 → Manage R2 API Tokens
2. Create token with Read & Write permissions
3. Scope to `mcp-guard-*-binaries` buckets

#### 5. Configure Stripe

**Create Products:**

1. Go to Stripe Dashboard → Products
2. Create "MCP-Guard Pro":
   - Price: $12/month recurring
   - Copy Price ID (format: `price_xxx...`)
3. Create "MCP-Guard Enterprise" (optional):
   - Price: Custom or $29 base + $8/seat

**Get API Keys:**

1. Developers → API Keys
2. Copy Publishable Key (`pk_live_xxx` or `pk_test_xxx`)
3. Copy Secret Key (`sk_live_xxx` or `sk_test_xxx`)

**Update Landing Page:**

Edit `landing/src/app/pages/signup/signup.component.ts`:

```typescript
private stripePublishableKey = 'pk_live_YOUR_ACTUAL_KEY';
private stripePriceId = 'price_YOUR_ACTUAL_PRICE_ID';
```

#### 6. Configure Resend (Email Delivery)

1. Create account at https://resend.com (Free tier: 1k emails/mo)
2. Add and verify domain: `licenses@mcp-guard.io`
3. Create API key with "Sending access"
4. Copy API key (`re_xxx...`)

---

## Worker Deployment

### 1. License Signing Worker

```bash
cd workers/sign-license

# Install dependencies
npm install

# Set secrets
cat ../../.keys/pro_license_private.pem | \
  wrangler secret put PRO_LICENSE_PRIVATE_KEY

openssl rand -hex 32 | tee api-secret.txt | \
  wrangler secret put API_SECRET_KEY

# IMPORTANT: Save api-secret.txt to password manager, then delete it!

# Deploy
wrangler deploy --env production

# Test
curl https://sign.mcp-guard.io/health
```

### 2. Binary Download Worker

```bash
cd workers/download-binary

# Install dependencies
npm install

# Set secrets
cat ../../.keys/pro_license_public.pem | grep -v "BEGIN\|END" | tr -d '\n' | \
  wrangler secret put PRO_LICENSE_PUBLIC_KEY

wrangler secret put KEYGEN_ACCOUNT_ID
# Paste your Keygen Account ID

# Deploy
wrangler deploy --env production

# Test
curl https://download.mcp-guard.io/health
```

### 3. Stripe Webhook Handler

```bash
cd workers/stripe-webhook

# Install dependencies
npm install

# Create KV namespace
wrangler kv:namespace create LICENSES
# Copy namespace ID, update wrangler.toml

# Set secrets
wrangler secret put STRIPE_WEBHOOK_SECRET
# Get from Stripe Dashboard after creating webhook (step below)

wrangler secret put LICENSE_SIGNER_URL
# Enter: https://sign.mcp-guard.io

wrangler secret put LICENSE_SIGNER_API_SECRET
# Paste API secret from password manager (from step 1)

wrangler secret put RESEND_API_KEY
# Paste Resend API key

# Deploy
wrangler deploy --env production

# Test
curl https://webhook.mcp-guard.io/health
```

**Configure Stripe Webhook:**

1. Go to Stripe Dashboard → Developers → Webhooks
2. Click "Add endpoint"
3. URL: `https://webhook.mcp-guard.io/webhook`
4. Events:
   - `checkout.session.completed`
   - `customer.subscription.deleted`
   - `customer.subscription.updated`
   - `invoice.payment_failed`
5. Copy "Signing secret" (`whsec_xxx...`)
6. Set secret: `wrangler secret put STRIPE_WEBHOOK_SECRET`

---

## Landing Page Deployment

### Option 1: Cloudflare Pages (Recommended)

```bash
cd landing

# Build production
npm run build

# Deploy to Cloudflare Pages
wrangler pages deploy dist/landing/browser

# Configure custom domain
wrangler pages domain add mcp-guard.io
```

### Option 2: Manual Deployment

```bash
cd landing
npm run build

# Upload dist/landing/browser/ to your hosting provider
# (Netlify, Vercel, GitHub Pages, etc.)
```

**Configure Routes:**

Ensure your Angular router handles these routes:
- `/` - Home page
- `/pricing` - Pricing page (already exists)
- `/signup` - New Pro signup page
- `/success` - Post-checkout success page
- `/docs/*` - Documentation (if exists)

**Update app.routes.ts:**

```typescript
import { Routes } from '@angular/router';
import { SignupComponent } from './pages/signup/signup.component';
import { SuccessComponent } from './pages/success/success.component';

export const routes: Routes = [
  // ... existing routes
  { path: 'signup', component: SignupComponent },
  { path: 'success', component: SuccessComponent },
  // ...
];
```

---

## GitHub Actions Release

### Trigger First Release

```bash
# Commit all changes
git add .
git commit -m "chore: v1.0.0 release infrastructure"

# Create and push tag
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

This triggers `.github/workflows/release-tiers.yml` which:

1. **Free Tier**:
   - Builds for 5 platforms
   - Creates GitHub Release
   - Publishes to crates.io

2. **Pro Tier**:
   - Builds for 5 platforms
   - Uploads to R2: `s3://mcp-guard-pro-binaries/latest/`

3. **Enterprise Tier**:
   - Builds for 5 platforms
   - Uploads to R2: `s3://mcp-guard-enterprise-binaries/latest/`

**Monitor Progress:**

1. GitHub → Actions tab
2. Watch "Release All Tiers" workflow
3. Verify R2 uploads: `wrangler r2 object list mcp-guard-pro-binaries`

---

## Testing End-to-End Flow

### Test Pro Purchase Flow

1. **Start Checkout**:
   - Go to `https://mcp-guard.io/signup?plan=pro`
   - Enter email
   - Complete Stripe checkout (use test card: `4242 4242 4242 4242`)

2. **Verify License Generation**:
   ```bash
   # Check Worker logs
   wrangler tail mcp-guard-stripe-webhook

   # Check KV storage
   wrangler kv:key list --namespace-id=<your-namespace-id>
   ```

3. **Check Email**:
   - Verify license email received
   - Copy license key from email

4. **Test Installation**:
   ```bash
   # Use license from email
   export MCP_GUARD_LICENSE_KEY="pro_xxx..."

   # Test download
   curl "https://download.mcp-guard.io/download?tier=pro&platform=x86_64-linux&license=$MCP_GUARD_LICENSE_KEY" \
     -o mcp-guard-test

   # Verify binary
   chmod +x mcp-guard-test
   ./mcp-guard-test version
   # Should show: "Tier: Pro"
   ```

5. **Test Installation Script**:
   ```bash
   curl -fsSL https://mcp-guard.io/install-pro.sh | \
     MCP_GUARD_LICENSE_KEY="$MCP_GUARD_LICENSE_KEY" bash

   mcp-guard version
   # Should show Pro features
   ```

### Test Enterprise License

```bash
# Generate test Enterprise license via Keygen Dashboard
# Or via CLI:
keygen licenses create \
  --policy-id <your-policy-id> \
  --name "Test Customer" \
  --metadata '{"email":"test@example.com"}'

# Test download
export MCP_GUARD_LICENSE_KEY="ent_xxx..."
curl "https://download.mcp-guard.io/download?tier=enterprise&platform=x86_64-linux&license=$MCP_GUARD_LICENSE_KEY" \
  -o mcp-guard-ent

chmod +x mcp-guard-ent
./mcp-guard-ent version
# Should show: "Tier: Enterprise"
```

---

## Post-Launch Monitoring

### Cloudflare Workers

```bash
# View real-time logs
wrangler tail mcp-guard-stripe-webhook
wrangler tail mcp-guard-license-signer
wrangler tail mcp-guard-download

# View metrics
# Go to: Cloudflare Dashboard → Workers → [worker-name] → Metrics
```

### Stripe

Monitor in Stripe Dashboard:
- Payments → Overview: Track MRR
- Webhooks → [your-webhook]: Check delivery success rate
- Customers: View customer list and subscriptions

### GitHub Actions

Watch for build failures:
- GitHub → Actions → Release All Tiers
- Set up notifications for workflow failures

### R2 Storage

```bash
# Check binary uploads
wrangler r2 object list mcp-guard-pro-binaries
wrangler r2 object list mcp-guard-enterprise-binaries

# Check storage usage
# Cloudflare Dashboard → R2 → Buckets
```

---

## Break-Even Calculation

**Monthly Costs:**

| Service | Cost |
|---------|------|
| Cloudflare Workers (3) | $0-5 |
| Cloudflare R2 | ~$1 |
| Cloudflare KV | $0 |
| Keygen.sh | $49 |
| Resend | $0 (free tier) |
| Stripe fees | 2.9% + $0.30 per transaction |
| **Total Fixed** | **~$50-55/mo** |

**Break-Even:**
- Pro pricing: $12/month
- Break-even: 5 customers ($60 MRR)

**Revenue Milestones:**

| Customers | MRR | Annual |
|-----------|-----|--------|
| 5 | $60 | $720 |
| 10 | $120 | $1,440 |
| 25 | $300 | $3,600 |
| 50 | $600 | $7,200 |
| 100 | $1,200 | $14,400 |

---

## Rollback Procedures

### Worker Rollback

```bash
# Rollback specific worker
cd workers/stripe-webhook
wrangler rollback

# Or rollback to specific deployment
wrangler deployments list
wrangler rollback --deployment-id <id>
```

### Landing Page Rollback

```bash
cd landing
wrangler pages deployment list
wrangler pages rollback <deployment-id>
```

### Binary Rollback

```bash
# GitHub Actions builds are tagged by version
# To rollback, re-upload old binaries to R2:
aws s3 cp old-binary.tar.gz \
  s3://mcp-guard-pro-binaries/latest/mcp-guard-x86_64-linux \
  --endpoint-url https://<account-id>.r2.cloudflarestorage.com
```

---

## Troubleshooting

### "License generation failed"

1. Check license signing Worker: `curl https://sign.mcp-guard.io/health`
2. Verify API secret in Stripe webhook: `wrangler secret list`
3. Check Worker logs: `wrangler tail mcp-guard-stripe-webhook`

### "Binary not found"

1. Verify R2 upload: `wrangler r2 object list mcp-guard-pro-binaries`
2. Check object naming: `latest/mcp-guard-x86_64-linux`
3. Re-trigger GitHub Actions if needed

### "Email not received"

1. Check Resend dashboard for delivery status
2. Verify domain verification: DNS records correct
3. Check spam folder
4. Test Resend API manually:
   ```bash
   curl https://api.resend.com/emails \
     -H "Authorization: Bearer $RESEND_API_KEY" \
     -H "Content-Type: application/json" \
     -d '{"from":"licenses@mcp-guard.io","to":"test@example.com","subject":"Test","html":"<p>Test</p>"}'
   ```

### "Webhook signature verification failed"

1. Verify webhook secret in Stripe Dashboard matches Worker
2. Re-set secret: `wrangler secret put STRIPE_WEBHOOK_SECRET`
3. Check webhook URL is correct: `https://webhook.mcp-guard.io/webhook`

---

## Security Checklist

- [ ] Ed25519 private key backed up offline (encrypted)
- [ ] All Cloudflare secrets set (no secrets in code)
- [ ] GitHub secrets configured for R2 upload
- [ ] Stripe webhook secret verified
- [ ] API secret for license signer stored in password manager
- [ ] R2 buckets are private (no public access)
- [ ] Resend domain verified (SPF/DKIM configured)
- [ ] No license keys logged in Worker logs
- [ ] Test mode disabled in production (Stripe live keys)

---

## Support

### Documentation

- **Pro tier**: https://mcp-guard.io/docs/pro
- **Enterprise**: https://mcp-guard.io/docs/enterprise
- **Troubleshooting**: https://mcp-guard.io/docs/troubleshooting

### Contact

- **General**: support@mcp-guard.io
- **Sales**: sales@mcp-guard.io
- **Emergencies**: (configure on-call rotation)

---

## Next Steps After Launch

1. **Week 1**: Monitor all systems, fix any issues
2. **Week 2**: Collect customer feedback, iterate on docs
3. **Month 1**: Analyze metrics, optimize conversion funnel
4. **Month 2**: Plan Enterprise sales strategy
5. **Month 3**: Build customer portal for license management

---

## Success! 🚀

Your MCP-Guard v1.0 infrastructure is ready to launch. Everything from payment processing to binary delivery is automated.

**Final Steps:**

1. Review this checklist
2. Deploy all Workers
3. Configure Stripe and Resend
4. Push v1.0.0 tag
5. Test end-to-end flow
6. Go live!

**Questions?** Review the component-specific READMEs in each `workers/*/` directory.
