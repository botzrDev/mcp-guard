# Stripe Free Trial Integration Audit

**Date:** 2026-01-06
**Status:** 🔴 BROKEN - Multiple Critical Issues Found

---

## Executive Summary

The Stripe free trial integration is currently **non-functional** due to multiple issues:

1. **Critical**: Frontend using deprecated Stripe API (`redirectToCheckout`)
2. **Critical**: Missing backend environment configuration
3. **Security**: Live Stripe keys hardcoded in source code
4. **Minor**: Incomplete error handling in frontend

---

## Issues Found

### 🔴 Issue 1: Deprecated Stripe API Usage

**File:** `landing/src/app/pages/signup/signup.component.ts:507-509`

**Current Code:**
```typescript
const { error } = await stripe.redirectToCheckout({
  sessionId: response.session_id
});
```

**Problem:**
- `stripe.redirectToCheckout()` is **deprecated** by Stripe
- Modern Stripe API requires using the `url` field directly
- This causes browser errors when users try to checkout

**Impact:** ⚠️ CRITICAL - Users cannot complete checkout

**Fix Required:**
```typescript
// Use the URL directly from the backend response
window.location.href = response.url;
```

---

### 🔴 Issue 2: Missing Backend Configuration

**File:** Backend environment (not found in repository)

**Problem:**
- Backend requires `STRIPE_SECRET_KEY` environment variable
- Config file `mcp-guard.toml` has no Stripe configuration
- Backend logs will show: `"Stripe secret key missing, billing route not registered"`
- The `/api/billing/checkout` endpoint will NOT be registered

**Evidence:**
```rust
// crates/mcp-guard-core/src/server/mod.rs:1244-1248
if state.config.stripe_secret_key.is_some() {
    tracing::info!("Registering Stripe billing route");
    router = router.route("/api/billing/checkout", post(billing::create_checkout_session));
} else {
    tracing::warn!("Stripe secret key missing, billing route not registered");
}
```

**Impact:** ⚠️ CRITICAL - Backend API endpoint doesn't exist

**Fix Required:**
1. Create `.env` file with:
   ```bash
   STRIPE_SECRET_KEY=sk_live_... # or sk_test_... for testing
   ```
2. Or add to `mcp-guard.toml`:
   ```toml
   stripe_secret_key = "set_via_env_var"  # Reference to env var
   ```

---

### 🟡 Issue 3: Live Keys in Source Code

**File:** `landing/src/environments/environment.ts:3-4`

**Current Code:**
```typescript
stripePublishableKey: 'pk_live_51Sc3idA6PFA1cvOSbizserkKTbflytKKDtySfj1bIGpytZMF78YJPlmgDKMUvfwPKzzc1DrP3xZlLC6wHTzLoMwM00Scg2SH3r',
stripePriceId: 'price_1SlwzhA6PFA1cvOS5YoY768A',
```

**Problem:**
- **LIVE** Stripe publishable key hardcoded in source
- Committed to git repository
- Should use environment variables or build-time injection

**Impact:** 🟡 SECURITY RISK - Keys should not be in source control

**Fix Required:**
1. Move to environment-specific files
2. Use Angular environment replacement
3. Add to `.gitignore`

---

### 🟡 Issue 4: Incomplete Error Handling

**File:** `landing/src/app/pages/signup/signup.component.ts:515-519`

**Current Code:**
```typescript
} catch (err) {
  console.error('Checkout error:', err);
  this.error.set('An unexpected error occurred. Please try again.');
  this.loading.set(false);
}
```

**Problem:**
- Generic error message doesn't help users
- Doesn't handle specific API errors (404, 500, network)
- Doesn't log enough context for debugging

**Impact:** 🟡 MINOR - Poor user experience on errors

**Fix Required:**
```typescript
} catch (err: any) {
  console.error('Checkout error:', err);

  // Provide specific error messages
  if (err.status === 404) {
    this.error.set('Checkout service is not available. Please contact support.');
  } else if (err.status === 500) {
    this.error.set('Server error. Please try again later.');
  } else if (!navigator.onLine) {
    this.error.set('No internet connection. Please check your network.');
  } else {
    this.error.set(err.error?.message || 'An unexpected error occurred. Please try again.');
  }

  this.loading.set(false);
}
```

---

## Backend API Analysis

### ✅ What's Working

The backend implementation is **correct**:

**File:** `crates/mcp-guard-core/src/server/billing.rs`

```rust
pub async fn create_checkout_session(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCheckoutSessionRequest>,
) -> Result<impl IntoResponse, AppError> {
    // ... creates Stripe checkout session correctly ...

    Ok(Json(CreateCheckoutSessionResponse {
        session_id: checkout_session.id.as_str().to_string(),
        url,  // ✅ Returns URL for redirect
    }))
}
```

**Features:**
- ✅ Creates subscription with 7-day free trial
- ✅ Returns both `session_id` and `url`
- ✅ Proper error handling
- ✅ Security: validates secret key exists

**Problem:** The route is never registered due to missing `STRIPE_SECRET_KEY`

---

## Testing Flow

### Current Broken Flow

1. User enters email on `/signup` page
2. Frontend calls `POST /api/billing/checkout`
3. **❌ FAILS**: Backend returns 404 (route not registered)
4. **OR** if route exists: Frontend tries deprecated `redirectToCheckout()`
5. **❌ FAILS**: Browser error

### Expected Working Flow

1. User enters email on `/signup` page
2. Frontend calls `POST ${apiUrl}/api/billing/checkout` with:
   ```json
   {
     "email": "user@example.com",
     "price_id": "price_1SlwzhA6PFA1cvOS5YoY768A",
     "success_url": "http://localhost:4200/success?session_id={CHECKOUT_SESSION_ID}",
     "cancel_url": "http://localhost:4200/pricing"
   }
   ```
3. Backend creates Stripe Checkout Session:
   ```rust
   CheckoutSession::create(&client, CreateCheckoutSession {
       mode: CheckoutSessionMode::Subscription,
       customer_email: "user@example.com",
       subscription_data: Some(CreateCheckoutSessionSubscriptionData {
           trial_period_days: Some(7),  // 7-day free trial
           ..Default::default()
       }),
       // ...
   })
   ```
4. Backend returns:
   ```json
   {
     "session_id": "cs_test_...",
     "url": "https://checkout.stripe.com/c/pay/cs_test_..."
   }
   ```
5. Frontend redirects to Stripe: `window.location.href = response.url`
6. User completes checkout on Stripe (no credit card for trial)
7. Stripe redirects to success URL
8. Webhook delivers license

---

## Fix Priority

### P0 - Critical (Breaks functionality)

1. **Add STRIPE_SECRET_KEY to backend**
   - Create `.env` file or update config
   - Restart backend service

2. **Replace deprecated Stripe API in frontend**
   - Change `redirectToCheckout()` to direct URL redirect
   - Test with Stripe test mode

### P1 - High (Security/UX)

3. **Move Stripe keys to environment variables**
   - Create `environment.prod.ts` and `environment.dev.ts`
   - Use Angular environment replacement
   - Add `.env` files to `.gitignore`

4. **Improve error handling**
   - Add specific error messages
   - Log API errors for debugging

---

## How to Test

### Prerequisites

1. **Stripe Account**: Get test keys from https://dashboard.stripe.com/test/apikeys
2. **Product Setup**: Create test product with 7-day trial in Stripe Dashboard

### Test Steps

```bash
# 1. Set backend environment variable
export STRIPE_SECRET_KEY=sk_test_...  # Use test key

# 2. Start backend
cd /home/austingreen/Documents/botzr/projects/mcp-guard
cargo run

# Expected log:
# ✅ "Registering Stripe billing route"
# ❌ "Stripe secret key missing, billing route not registered" = BROKEN

# 3. Start frontend
cd landing
npm start

# 4. Test checkout flow
# - Open http://localhost:4200/signup
# - Enter test email
# - Click "Start Free Trial"
# - Should redirect to Stripe Checkout (not error)

# 5. Verify with Stripe test card
# Card: 4242 4242 4242 4242
# Exp: Any future date
# CVC: Any 3 digits
```

### Manual API Test

```bash
# Test backend endpoint directly
curl -X POST http://localhost:3000/api/billing/checkout \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "price_id": "price_1SlwzhA6PFA1cvOS5YoY768A",
    "success_url": "http://localhost:4200/success?session_id={CHECKOUT_SESSION_ID}",
    "cancel_url": "http://localhost:4200/pricing"
  }'

# Expected response:
# {
#   "session_id": "cs_test_...",
#   "url": "https://checkout.stripe.com/c/pay/cs_test_..."
# }

# If 404 = STRIPE_SECRET_KEY not set
# If 500 = Invalid Stripe credentials
```

---

## Recommended Fixes

### Fix 1: Backend Configuration

**Create:** `.env` (add to `.gitignore`)
```bash
# Stripe Configuration
STRIPE_SECRET_KEY=sk_test_51Sc3idA6PFA1cvOS... # Use test key for dev
```

**Or update:** `mcp-guard.toml`
```toml
# Add after [upstream] section
stripe_secret_key = "${STRIPE_SECRET_KEY}"  # Will be loaded from env
```

### Fix 2: Frontend - Replace Deprecated API

**File:** `landing/src/app/pages/signup/signup.component.ts`

**Replace lines 495-519 with:**
```typescript
async handleSubmit(event: Event) {
  event.preventDefault();

  if (!this.email || this.loading()) {
    return;
  }

  this.loading.set(true);
  this.error.set(null);

  try {
    // Create checkout session via backend API
    const response = await firstValueFrom(
      this.http.post<{ session_id: string; url: string }>(
        `${environment.apiUrl}/api/billing/checkout`,
        {
          email: this.email,
          price_id: environment.stripePriceId,
          success_url: `${window.location.origin}/success?session_id={CHECKOUT_SESSION_ID}`,
          cancel_url: `${window.location.origin}/pricing`
        }
      )
    );

    // Redirect to Stripe Checkout using the URL
    // Modern Stripe API - no need for stripe.js client-side redirect
    window.location.href = response.url;

  } catch (err: any) {
    console.error('Checkout error:', err);

    // Provide helpful error messages
    if (err.status === 404) {
      this.error.set('Checkout service unavailable. Please contact support.');
    } else if (err.status === 500) {
      this.error.set('Server error. Please try again in a few moments.');
    } else if (!navigator.onLine) {
      this.error.set('No internet connection. Please check your network.');
    } else {
      this.error.set(err.error?.error || 'Failed to start checkout. Please try again.');
    }

    this.loading.set(false);
  }
}
```

**Bonus:** Remove Stripe.js loading (no longer needed)
```typescript
// DELETE lines 470-483 (loadStripe method)
// DELETE line 471 (this.loadStripe() call)
// DELETE line 9 (declare const Stripe: any)
```

### Fix 3: Environment Variables for Frontend

**Create:** `landing/src/environments/environment.prod.ts`
```typescript
export const environment = {
  production: true,
  stripePublishableKey: 'pk_live_...',  // Injected at build time
  stripePriceId: 'price_1SlwzhA6PFA1cvOS5YoY768A',
  apiUrl: 'https://api.mcpg.botzr.com'  // Production API
};
```

**Update:** `landing/src/environments/environment.ts` (dev only)
```typescript
export const environment = {
  production: false,
  stripePublishableKey: 'pk_test_...',  // Use TEST key for dev
  stripePriceId: 'price_test_...',      // Use TEST price for dev
  apiUrl: 'http://localhost:3000'
};
```

**Update:** `landing/.gitignore`
```gitignore
# Environment secrets
src/environments/environment.prod.ts
.env
.env.local
```

---

## Verification Checklist

After applying fixes:

- [ ] Backend starts with log: `"Registering Stripe billing route"`
- [ ] `curl localhost:3000/api/billing/checkout` returns JSON (not 404)
- [ ] Frontend redirects to `checkout.stripe.com` (not browser error)
- [ ] Stripe test card completes checkout successfully
- [ ] Success page receives `session_id` query parameter
- [ ] Webhook delivers license email (separate system)

---

## Additional Notes

### Stripe Checkout Session Configuration

The backend correctly creates a subscription with:
- **Mode:** `Subscription` (recurring billing)
- **Trial:** 7 days free (`trial_period_days: 7`)
- **Promotion codes:** Enabled (`allow_promotion_codes: true`)
- **Customer email:** Pre-filled from form

### Success Flow After Checkout

After Stripe checkout completes:
1. Stripe redirects to `success_url` with `?session_id={CHECKOUT_SESSION_ID}`
2. Success page should display confirmation
3. Stripe webhook fires `checkout.session.completed` event
4. Cloudflare Worker processes webhook and sends license email
5. User receives email with license key and instructions

### Related Files

- Backend API: `crates/mcp-guard-core/src/server/billing.rs`
- Frontend form: `landing/src/app/pages/signup/signup.component.ts`
- Success page: `landing/src/app/pages/success/success.component.ts`
- Webhook handler: `workers/stripe-webhook/src/index.ts`

---

## Contact

If issues persist after applying fixes:
1. Check backend logs for Stripe API errors
2. Verify Stripe Dashboard shows checkout sessions being created
3. Test with Stripe CLI: `stripe listen --forward-to localhost:3000/webhook`
4. Review Stripe API version compatibility (backend uses `async-stripe 0.38`)

---

**End of Audit Report**
