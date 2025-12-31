/**
 * TypeScript types for MCP-Guard License Signing Worker
 */

export interface Env {
  // Secrets (set with wrangler secret put)
  PRO_LICENSE_PRIVATE_KEY: string;  // Ed25519 private key (PEM format)
  API_SECRET_KEY: string;            // Secret for authenticating requests

  // Environment variables
  ENVIRONMENT: 'development' | 'production';
}

export interface SignLicenseRequest {
  licensee: string;          // Customer email or name
  expires_at?: string;       // ISO 8601 timestamp (defaults to 1 year)
  features?: string[];       // Optional feature list (defaults to all Pro features)
}

export interface SignLicenseResponse {
  license_key: string;       // Full license key: pro_<payload>.<signature>
  payload: ProLicensePayload;
  expires_at: string;
}

export interface ProLicensePayload {
  tier: string;              // Always "pro"
  issued_at: string;         // ISO 8601 timestamp
  expires_at: string;        // ISO 8601 timestamp
  licensee: string;          // Customer email or name
  features: string[];        // List of enabled features
}

export interface ErrorResponse {
  error: string;
  message: string;
  details?: unknown;
}
