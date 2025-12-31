/**
 * TypeScript types for MCP-Guard Stripe Webhook Worker
 */

export interface Env {
  // KV namespace for storing license data
  LICENSES: KVNamespace;

  // Secrets (set with wrangler secret put)
  STRIPE_WEBHOOK_SECRET: string;        // Stripe webhook signing secret
  LICENSE_SIGNER_URL: string;           // URL of license signing Worker
  LICENSE_SIGNER_API_SECRET: string;    // API secret for license signing Worker
  RESEND_API_KEY: string;               // Resend.com API key for emails

  // Environment variables
  STRIPE_API_VERSION: string;           // Stripe API version
  FROM_EMAIL: string;                   // Sender email address
  ENVIRONMENT: 'development' | 'production';
}

export interface LicenseData {
  license_key: string;
  customer_id: string;       // Stripe customer ID
  customer_email: string;
  tier: 'pro' | 'enterprise';
  status: 'active' | 'cancelled' | 'expired';
  issued_at: string;         // ISO 8601 timestamp
  expires_at: string;        // ISO 8601 timestamp
  subscription_id?: string;  // Stripe subscription ID (if subscription)
}

export interface SignLicenseRequest {
  licensee: string;
  expires_at?: string;
  features?: string[];
}

export interface SignLicenseResponse {
  license_key: string;
  payload: {
    tier: string;
    issued_at: string;
    expires_at: string;
    licensee: string;
    features: string[];
  };
  expires_at: string;
}

export interface EmailRequest {
  from: string;
  to: string;
  subject: string;
  html: string;
}
