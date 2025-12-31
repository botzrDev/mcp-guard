/**
 * TypeScript types for MCP-Guard download worker
 */

export interface Env {
  // R2 bucket bindings
  PRO_BINARIES: R2Bucket;
  ENTERPRISE_BINARIES: R2Bucket;

  // Secrets (set with wrangler secret put)
  PRO_LICENSE_PUBLIC_KEY: string;
  KEYGEN_ACCOUNT_ID: string;

  // Environment variables
  KEYGEN_API_URL: string;
  ENVIRONMENT: 'development' | 'production';
}

export type Tier = 'pro' | 'enterprise';

export type Platform =
  | 'x86_64-linux'
  | 'x86_64-linux-musl'
  | 'x86_64-darwin'
  | 'aarch64-darwin'
  | 'x86_64-windows';

export interface ProLicensePayload {
  tier: string;
  issued_at: string;
  expires_at: string;
  licensee: string;
  features: string[];
}

export interface ValidationResult {
  valid: boolean;
  error?: string;
  licensee?: string;
  tier?: Tier;
}

export interface DownloadRequest {
  license: string;
  tier: Tier;
  platform: Platform;
}
