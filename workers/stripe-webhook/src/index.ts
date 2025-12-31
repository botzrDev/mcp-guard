/**
 * MCP-Guard Stripe Webhook Handler
 *
 * Processes Stripe webhook events for Pro tier purchases:
 * - checkout.session.completed: Generate and email license
 * - customer.subscription.deleted: Revoke license
 * - invoice.payment_failed: Notify customer
 */

import Stripe from 'stripe';
import type {
  Env,
  LicenseData,
  SignLicenseRequest,
  SignLicenseResponse,
  EmailRequest
} from './types';

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    // CORS headers (for webhook testing tools)
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Stripe-Signature'
    };

    // Handle OPTIONS request (CORS preflight)
    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    // Health check endpoint
    if (request.url.endsWith('/health')) {
      return new Response(
        JSON.stringify({ status: 'ok', service: 'mcp-guard-stripe-webhook' }),
        { headers: { 'Content-Type': 'application/json', ...corsHeaders } }
      );
    }

    // Webhook endpoint
    if (request.method === 'POST' && (request.url.endsWith('/webhook') || request.url.endsWith('/'))) {
      return handleWebhook(request, env, corsHeaders);
    }

    return new Response('Not Found', { status: 404, headers: corsHeaders });
  }
};

/**
 * Handle Stripe webhook event
 */
async function handleWebhook(
  request: Request,
  env: Env,
  corsHeaders: Record<string, string>
): Promise<Response> {
  try {
    // Get raw body for signature verification
    const payload = await request.text();
    const signature = request.headers.get('stripe-signature');

    if (!signature) {
      console.error('Missing Stripe signature header');
      return new Response('Missing signature', {
        status: 400,
        headers: corsHeaders
      });
    }

    // Verify webhook signature
    const stripe = new Stripe(env.STRIPE_WEBHOOK_SECRET, {
      apiVersion: env.STRIPE_API_VERSION as Stripe.LatestApiVersion,
      httpClient: Stripe.createFetchHttpClient()
    });

    let event: Stripe.Event;
    try {
      event = stripe.webhooks.constructEvent(
        payload,
        signature,
        env.STRIPE_WEBHOOK_SECRET
      );
    } catch (error) {
      console.error('Webhook signature verification failed:', error);
      return new Response('Invalid signature', {
        status: 400,
        headers: corsHeaders
      });
    }

    console.log(`Received Stripe event: ${event.type}`);

    // Handle event by type
    switch (event.type) {
      case 'checkout.session.completed':
        await handleCheckoutCompleted(event.data.object as Stripe.Checkout.Session, env);
        break;

      case 'customer.subscription.deleted':
        await handleSubscriptionDeleted(event.data.object as Stripe.Subscription, env);
        break;

      case 'customer.subscription.updated':
        await handleSubscriptionUpdated(event.data.object as Stripe.Subscription, env);
        break;

      case 'invoice.payment_failed':
        await handlePaymentFailed(event.data.object as Stripe.Invoice, env);
        break;

      default:
        console.log(`Unhandled event type: ${event.type}`);
    }

    return new Response(JSON.stringify({ received: true }), {
      status: 200,
      headers: { 'Content-Type': 'application/json', ...corsHeaders }
    });

  } catch (error) {
    console.error('Webhook handler error:', error);
    return new Response(
      JSON.stringify({
        error: 'Webhook processing failed',
        message: error instanceof Error ? error.message : 'Unknown error'
      }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json', ...corsHeaders }
      }
    );
  }
}

/**
 * Handle checkout.session.completed
 * Generate Pro license and email to customer
 */
async function handleCheckoutCompleted(
  session: Stripe.Checkout.Session,
  env: Env
): Promise<void> {
  console.log(`Processing checkout for customer: ${session.customer_email}`);

  // Extract metadata
  const tier = session.metadata?.tier || 'pro';
  const email = session.customer_email;

  if (!email) {
    console.error('No customer email in checkout session');
    return;
  }

  // Generate license
  const expiresAt = session.mode === 'subscription'
    ? new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toISOString()  // 1 year
    : new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();  // 30 days for one-time

  const license = await generateLicense(email, expiresAt, env);

  // Store license data
  const licenseData: LicenseData = {
    license_key: license.license_key,
    customer_id: session.customer as string,
    customer_email: email,
    tier: tier as 'pro' | 'enterprise',
    status: 'active',
    issued_at: license.payload.issued_at,
    expires_at: license.payload.expires_at,
    subscription_id: session.subscription as string | undefined
  };

  await env.LICENSES.put(
    `customer:${session.customer}`,
    JSON.stringify(licenseData)
  );

  await env.LICENSES.put(
    `license:${license.license_key}`,
    JSON.stringify(licenseData)
  );

  console.log(`License generated and stored for ${email}`);

  // Send license email
  await sendLicenseEmail(email, license.license_key, env);

  console.log(`License email sent to ${email}`);
}

/**
 * Handle customer.subscription.deleted
 * Mark license as cancelled
 */
async function handleSubscriptionDeleted(
  subscription: Stripe.Subscription,
  env: Env
): Promise<void> {
  console.log(`Processing subscription deletion for customer: ${subscription.customer}`);

  // Retrieve license data
  const licenseDataJson = await env.LICENSES.get(`customer:${subscription.customer}`);
  if (!licenseDataJson) {
    console.warn(`No license found for customer: ${subscription.customer}`);
    return;
  }

  const licenseData: LicenseData = JSON.parse(licenseDataJson);

  // Update status
  licenseData.status = 'cancelled';

  // Store updated data
  await env.LICENSES.put(
    `customer:${subscription.customer}`,
    JSON.stringify(licenseData)
  );

  await env.LICENSES.put(
    `license:${licenseData.license_key}`,
    JSON.stringify(licenseData)
  );

  console.log(`License marked as cancelled for ${licenseData.customer_email}`);

  // Send cancellation email
  await sendCancellationEmail(licenseData.customer_email, licenseData.expires_at, env);
}

/**
 * Handle customer.subscription.updated
 * Renew license if subscription renews
 */
async function handleSubscriptionUpdated(
  subscription: Stripe.Subscription,
  env: Env
): Promise<void> {
  console.log(`Processing subscription update for customer: ${subscription.customer}`);

  // Check if subscription status changed to active (renewal)
  if (subscription.status === 'active') {
    const licenseDataJson = await env.LICENSES.get(`customer:${subscription.customer}`);
    if (!licenseDataJson) {
      console.warn(`No license found for customer: ${subscription.customer}`);
      return;
    }

    const licenseData: LicenseData = JSON.parse(licenseDataJson);

    // Update expiry date based on subscription period
    const periodEnd = new Date(subscription.current_period_end * 1000);
    licenseData.expires_at = periodEnd.toISOString();
    licenseData.status = 'active';

    // Store updated data
    await env.LICENSES.put(
      `customer:${subscription.customer}`,
      JSON.stringify(licenseData)
    );

    await env.LICENSES.put(
      `license:${licenseData.license_key}`,
      JSON.stringify(licenseData)
    );

    console.log(`License renewed until ${periodEnd.toISOString()} for ${licenseData.customer_email}`);
  }
}

/**
 * Handle invoice.payment_failed
 * Notify customer of payment failure
 */
async function handlePaymentFailed(
  invoice: Stripe.Invoice,
  env: Env
): Promise<void> {
  console.log(`Processing payment failure for customer: ${invoice.customer}`);

  const licenseDataJson = await env.LICENSES.get(`customer:${invoice.customer}`);
  if (!licenseDataJson) {
    return;
  }

  const licenseData: LicenseData = JSON.parse(licenseDataJson);

  // Send payment failure email
  await sendPaymentFailedEmail(
    licenseData.customer_email,
    invoice.hosted_invoice_url || '',
    env
  );
}

/**
 * Generate Pro license by calling license signing Worker
 */
async function generateLicense(
  email: string,
  expiresAt: string,
  env: Env
): Promise<SignLicenseResponse> {
  const request: SignLicenseRequest = {
    licensee: email,
    expires_at: expiresAt
  };

  const response = await fetch(`${env.LICENSE_SIGNER_URL}/sign`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-API-Secret': env.LICENSE_SIGNER_API_SECRET
    },
    body: JSON.stringify(request)
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`License generation failed: ${error}`);
  }

  return await response.json();
}

/**
 * Send license email via Resend.com
 */
async function sendLicenseEmail(
  to: string,
  licenseKey: string,
  env: Env
): Promise<void> {
  const emailRequest: EmailRequest = {
    from: env.FROM_EMAIL,
    to,
    subject: 'Your MCP-Guard Pro License Key',
    html: `
      <!DOCTYPE html>
      <html>
      <head>
        <style>
          body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
          .container { max-width: 600px; margin: 0 auto; padding: 20px; }
          .header { background: #4F46E5; color: white; padding: 20px; text-align: center; }
          .content { padding: 20px; background: #f9fafb; }
          .license-box { background: #fff; border: 2px solid #4F46E5; padding: 15px; margin: 20px 0; font-family: monospace; word-break: break-all; }
          .button { display: inline-block; background: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; margin: 10px 0; }
          .footer { text-align: center; padding: 20px; color: #666; font-size: 12px; }
        </style>
      </head>
      <body>
        <div class="container">
          <div class="header">
            <h1>🚀 Welcome to MCP-Guard Pro!</h1>
          </div>
          <div class="content">
            <p>Hi there,</p>
            <p>Thank you for purchasing MCP-Guard Pro! Your license key is ready.</p>

            <h3>Your License Key:</h3>
            <div class="license-box">${licenseKey}</div>

            <h3>Installation:</h3>
            <p>Install MCP-Guard Pro with one command:</p>
            <pre style="background: #1f2937; color: #f3f4f6; padding: 15px; border-radius: 5px; overflow-x: auto;">curl -fsSL https://mcp-guard.io/install-pro.sh | \
  MCP_GUARD_LICENSE_KEY="${licenseKey}" bash</pre>

            <h3>What's Included:</h3>
            <ul>
              <li>✓ OAuth 2.1 authentication (GitHub, Google, etc.)</li>
              <li>✓ JWT with JWKS support</li>
              <li>✓ HTTP and SSE transports</li>
              <li>✓ Per-identity rate limiting</li>
              <li>✓ Priority email support</li>
            </ul>

            <a href="https://mcp-guard.io/docs/pro" class="button">View Documentation</a>

            <p>Need help? Reply to this email or visit <a href="https://mcp-guard.io/support">our support page</a>.</p>

            <p>Best regards,<br>The MCP-Guard Team</p>
          </div>
          <div class="footer">
            <p>MCP-Guard Pro | <a href="https://mcp-guard.io">mcp-guard.io</a></p>
            <p>© 2025 Botzr. All rights reserved.</p>
          </div>
        </div>
      </body>
      </html>
    `
  };

  await sendEmail(emailRequest, env);
}

/**
 * Send cancellation email
 */
async function sendCancellationEmail(
  to: string,
  expiresAt: string,
  env: Env
): Promise<void> {
  const emailRequest: EmailRequest = {
    from: env.FROM_EMAIL,
    to,
    subject: 'MCP-Guard Pro Subscription Cancelled',
    html: `
      <!DOCTYPE html>
      <html>
      <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
          <h2>Subscription Cancelled</h2>
          <p>Your MCP-Guard Pro subscription has been cancelled.</p>
          <p>Your license will remain active until: <strong>${new Date(expiresAt).toLocaleDateString()}</strong></p>
          <p>After this date, Pro features will no longer be available.</p>
          <p>Want to continue using Pro features? <a href="https://mcp-guard.io/pricing">Resubscribe here</a>.</p>
          <p>Questions? Reply to this email.</p>
          <p>Best regards,<br>The MCP-Guard Team</p>
        </div>
      </body>
      </html>
    `
  };

  await sendEmail(emailRequest, env);
}

/**
 * Send payment failed email
 */
async function sendPaymentFailedEmail(
  to: string,
  invoiceUrl: string,
  env: Env
): Promise<void> {
  const emailRequest: EmailRequest = {
    from: env.FROM_EMAIL,
    to,
    subject: 'MCP-Guard Pro - Payment Failed',
    html: `
      <!DOCTYPE html>
      <html>
      <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
        <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
          <h2 style="color: #dc2626;">Payment Failed</h2>
          <p>We couldn't process your payment for MCP-Guard Pro.</p>
          <p>To continue using Pro features, please update your payment method:</p>
          <p><a href="${invoiceUrl}" style="display: inline-block; background: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px;">Update Payment Method</a></p>
          <p>If you have questions, please reply to this email.</p>
          <p>Best regards,<br>The MCP-Guard Team</p>
        </div>
      </body>
      </html>
    `
  };

  await sendEmail(emailRequest, env);
}

/**
 * Send email via Resend.com API
 */
async function sendEmail(emailRequest: EmailRequest, env: Env): Promise<void> {
  const response = await fetch('https://api.resend.com/emails', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${env.RESEND_API_KEY}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(emailRequest)
  });

  if (!response.ok) {
    const error = await response.text();
    console.error('Email sending failed:', error);
    throw new Error(`Failed to send email: ${error}`);
  }

  console.log('Email sent successfully');
}
