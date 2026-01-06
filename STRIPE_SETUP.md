# Stripe Integration Setup Guide

This guide walks you through setting up the Stripe free trial integration for MCP-Guard.

## Overview

The Stripe integration enables:
- 7-day free trial for Pro tier
- Automatic subscription billing after trial
- Checkout session creation via backend API
- Webhook-based license delivery

---

## Prerequisites

1. **Stripe Account** - Sign up at https://stripe.com
2. **Stripe Product** - Create a product in Stripe Dashboard
3. **Test Keys** - Get test keys for development

---

## Step 1: Get Stripe Keys

### Development (Test Mode)

1. Go to https://dashboard.stripe.com/test/apikeys
2. Copy your **Publishable key** (starts with `pk_test_...`)
3. Copy your **Secret key** (starts with `sk_test_...`)

### Production (Live Mode)

1. Go to https://dashboard.stripe.com/apikeys
2. Copy your **Publishable key** (starts with `pk_live_...`)
3. Copy your **Secret key** (starts with `sk_live_...`)

---

## Step 2: Create Stripe Product

1. Go to https://dashboard.stripe.com/test/products (test mode)
2. Click **"Create product"**
3. Fill in details:
   - **Name:** MCP-Guard Pro
   - **Description:** Professional tier with OAuth 2.1, HTTP/SSE transports, and per-identity rate limiting
   - **Pricing:** $12/month (recurring)
   - **Trial period:** 7 days
4. Click **"Save product"**
5. Copy the **Price ID** (starts with `price_...`)

---

## Step 3: Configure Backend

### Option A: Environment Variable (Recommended)

Create a `.env` file in the project root:

```bash
# Copy the example file
cp .env.example .env

# Edit the file and add your Stripe secret key
STRIPE_SECRET_KEY=sk_test_YOUR_SECRET_KEY_HERE
```

### Option B: Config File

Add to `mcp-guard.toml`:

```toml
stripe_secret_key = "sk_test_YOUR_SECRET_KEY_HERE"
```

**Important:** Never commit real Stripe keys to git!

---

## Step 4: Configure Frontend

### Development Environment

Edit `landing/src/environments/environment.ts`:

```typescript
export const environment = {
    production: false,
    stripePublishableKey: 'pk_test_YOUR_PUBLISHABLE_KEY_HERE',
    stripePriceId: 'price_YOUR_TEST_PRICE_ID_HERE',
    apiUrl: 'http://localhost:3000'
};
```

### Production Environment

Edit `landing/src/environments/environment.prod.ts`:

```typescript
export const environment = {
    production: true,
    stripePublishableKey: 'pk_live_YOUR_LIVE_PUBLISHABLE_KEY_HERE',
    stripePriceId: 'price_YOUR_LIVE_PRICE_ID_HERE',
    apiUrl: 'https://api.mcpg.botzr.com'
};
```

---

## Step 5: Start the Backend

```bash
# From project root
cargo run

# You should see this log:
# ✅ "Registering Stripe billing route"

# If you see this, the secret key is missing:
# ❌ "Stripe secret key missing, billing route not registered"
```

---

## Step 6: Start the Frontend

```bash
# From landing directory
cd landing
npm install
npm start

# Frontend will be at: http://localhost:4200
```

---

## Step 7: Test the Integration

### Test Checkout Flow

1. Open http://localhost:4200/signup
2. Enter a test email (e.g., `test@example.com`)
3. Click **"Start Free Trial"**
4. You should be redirected to Stripe Checkout

### Use Test Card

On the Stripe Checkout page, use this test card:

- **Card number:** `4242 4242 4242 4242`
- **Expiration:** Any future date (e.g., `12/25`)
- **CVC:** Any 3 digits (e.g., `123`)
- **ZIP:** Any 5 digits (e.g., `12345`)

### Complete Checkout

1. Click **"Subscribe"** on Stripe Checkout
2. You should be redirected to `/success?session_id=cs_test_...`
3. Check Stripe Dashboard → Customers to see the new subscription

---

## Troubleshooting

### Issue: "Checkout service is not available"

**Cause:** Backend API endpoint not registered

**Solution:**
1. Check backend logs for: `"Registering Stripe billing route"`
2. If missing, verify `STRIPE_SECRET_KEY` is set
3. Restart backend: `cargo run`

### Issue: Browser console error on checkout

**Cause:** Old Stripe API or missing backend

**Solution:**
1. Verify you've updated `signup.component.ts` with the fix
2. Test backend endpoint:
   ```bash
   curl -X POST http://localhost:3000/api/billing/checkout \
     -H "Content-Type: application/json" \
     -d '{"email":"test@example.com","price_id":"price_...","success_url":"http://localhost:4200/success","cancel_url":"http://localhost:4200/pricing"}'
   ```

### Issue: 404 error when calling `/api/billing/checkout`

**Cause:** Backend not running or secret key not set

**Solution:**
1. Start backend: `cargo run`
2. Check logs for "Registering Stripe billing route"
3. Verify `.env` file exists and has correct key

### Issue: Stripe checkout shows error

**Cause:** Invalid price ID or secret key

**Solution:**
1. Verify `stripePriceId` matches your Stripe Dashboard
2. Verify `STRIPE_SECRET_KEY` is correct
3. Check Stripe Dashboard → Logs for API errors

---

## API Endpoint Details

### POST /api/billing/checkout

**Request:**
```json
{
  "email": "user@example.com",
  "price_id": "price_1SlwzhA6PFA1cvOS5YoY768A",
  "success_url": "http://localhost:4200/success?session_id={CHECKOUT_SESSION_ID}",
  "cancel_url": "http://localhost:4200/pricing"
}
```

**Response (Success):**
```json
{
  "session_id": "cs_test_...",
  "url": "https://checkout.stripe.com/c/pay/cs_test_..."
}
```

**Response (Error - No Stripe Key):**
```json
{
  "error": "Server configuration error",
  "error_id": "uuid-here"
}
```

---

## Frontend Changes Summary

### What Was Fixed

1. **Removed deprecated Stripe.js API**
   - Old: `stripe.redirectToCheckout({ sessionId })`
   - New: `window.location.href = response.url`

2. **Removed Stripe.js script loading**
   - No longer need to load Stripe.js client-side
   - Checkout URL comes directly from backend

3. **Improved error handling**
   - Specific messages for 404, 500, network errors
   - Better user experience on failures

4. **Environment configuration**
   - Separated dev/prod environment files
   - Clear documentation for which keys to use

---

## Backend Configuration

The backend uses the `async-stripe` crate to create checkout sessions:

```rust
CheckoutSession::create(&client, CreateCheckoutSession {
    mode: Some(CheckoutSessionMode::Subscription),
    customer_email: Some(&payload.email),
    line_items: Some(vec![CreateCheckoutSessionLineItems {
        price: Some(payload.price_id),
        quantity: Some(1),
        ..Default::default()
    }]),
    subscription_data: Some(CreateCheckoutSessionSubscriptionData {
        trial_period_days: Some(7),  // 7-day free trial
        ..Default::default()
    }),
    allow_promotion_codes: Some(true),
    success_url: Some(&payload.success_url),
    cancel_url: Some(&payload.cancel_url),
    ..Default::default()
})
```

---

## Security Notes

1. **Never commit Stripe secret keys** - Use environment variables
2. **Use test keys in development** - Only use live keys in production
3. **Publishable keys are safe to commit** - They're meant to be public
4. **Validate webhook signatures** - Always verify Stripe webhook events

---

## Next Steps

After checkout works:

1. **Set up Stripe Webhook** - For automatic license delivery
   - See `workers/stripe-webhook/README.md`
   - Configure webhook endpoint in Stripe Dashboard

2. **Test subscription lifecycle**
   - Trial period (7 days)
   - First payment after trial
   - Subscription cancellation

3. **Production deployment**
   - Switch to live Stripe keys
   - Update `environment.prod.ts`
   - Deploy backend with `STRIPE_SECRET_KEY` env var

---

## Resources

- **Stripe Checkout Docs:** https://stripe.com/docs/payments/checkout
- **Stripe Test Cards:** https://stripe.com/docs/testing
- **Stripe Dashboard:** https://dashboard.stripe.com
- **MCP-Guard Webhook Setup:** `workers/stripe-webhook/README.md`

---

**Questions?** Check the full audit report in `STRIPE_INTEGRATION_AUDIT.md`
