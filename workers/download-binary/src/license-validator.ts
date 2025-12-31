/**
 * License validation for Pro and Enterprise tiers
 */

import { ed25519 } from '@noble/ed25519';
import type { Env, ProLicensePayload, ValidationResult, Tier } from './types';

/**
 * Validate a Pro license key using Ed25519 signature verification
 *
 * License format: pro_<base64-payload>.<base64-signature>
 */
export async function validateProLicense(
  licenseKey: string,
  env: Env
): Promise<ValidationResult> {
  try {
    // Check prefix
    if (!licenseKey.startsWith('pro_')) {
      return { valid: false, error: 'Invalid license key format (must start with pro_)' };
    }

    const keyBody = licenseKey.substring(4); // Remove 'pro_' prefix
    const parts = keyBody.split('.');

    if (parts.length !== 2) {
      return { valid: false, error: 'Invalid license key format (missing signature)' };
    }

    const [payloadB64, signatureB64] = parts;

    // Decode payload
    const payloadJson = atob(payloadB64);
    const payload: ProLicensePayload = JSON.parse(payloadJson);

    // Verify signature
    const isValidSignature = await verifyEd25519Signature(
      payloadB64,
      signatureB64,
      env.PRO_LICENSE_PUBLIC_KEY
    );

    if (!isValidSignature) {
      return { valid: false, error: 'Invalid license signature' };
    }

    // Check tier
    if (payload.tier !== 'pro') {
      return { valid: false, error: `License tier mismatch: expected 'pro', got '${payload.tier}'` };
    }

    // Check expiry
    const expiresAt = new Date(payload.expires_at);
    const now = new Date();

    if (expiresAt < now) {
      return {
        valid: false,
        error: `License expired on ${expiresAt.toISOString().split('T')[0]}`
      };
    }

    // All checks passed
    return {
      valid: true,
      licensee: payload.licensee,
      tier: 'pro'
    };

  } catch (error) {
    console.error('Pro license validation error:', error);
    return {
      valid: false,
      error: `License validation failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    };
  }
}

/**
 * Verify Ed25519 signature
 */
async function verifyEd25519Signature(
  message: string,
  signatureB64: string,
  publicKeyPem: string
): Promise<boolean> {
  try {
    // Parse public key from PEM format (SPKI)
    // Public key format: "MCowBQYDK2VwAyEA<32-bytes-base64>"
    const publicKeyDer = base64ToUint8Array(publicKeyPem);

    // Skip SPKI header (12 bytes) to get raw Ed25519 public key (32 bytes)
    if (publicKeyDer.length < 44) {
      throw new Error('Invalid public key length');
    }
    const rawPublicKey = publicKeyDer.slice(12, 44);

    // Decode signature (URL-safe base64)
    const signature = base64ToUint8Array(signatureB64, true);

    // Message to verify (UTF-8 bytes of base64 payload)
    const messageBytes = new TextEncoder().encode(message);

    // Verify signature
    return await ed25519.verify(signature, messageBytes, rawPublicKey);

  } catch (error) {
    console.error('Signature verification error:', error);
    return false;
  }
}

/**
 * Validate an Enterprise license key using Keygen.sh API
 *
 * License format: ent_<keygen-license-key>
 */
export async function validateEnterpriseLicense(
  licenseKey: string,
  env: Env
): Promise<ValidationResult> {
  try {
    // Check prefix
    if (!licenseKey.startsWith('ent_')) {
      return { valid: false, error: 'Invalid license key format (must start with ent_)' };
    }

    // Call Keygen.sh validation API
    const response = await fetch(
      `${env.KEYGEN_API_URL}/${env.KEYGEN_ACCOUNT_ID}/licenses/${licenseKey}/actions/validate`,
      {
        method: 'POST',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          meta: {
            scope: { product: 'mcp-guard-enterprise' }
          }
        })
      }
    );

    if (!response.ok) {
      const errorText = await response.text();
      return {
        valid: false,
        error: `Keygen validation failed: ${response.status} ${errorText}`
      };
    }

    const data = await response.json();

    // Check validation result
    if (!data.meta || data.meta.valid !== true) {
      const code = data.meta?.code || 'UNKNOWN';
      const detail = data.meta?.detail || 'License validation failed';
      return { valid: false, error: `${code}: ${detail}` };
    }

    // Extract license data
    const license = data.data;
    const licensee = license.attributes?.metadata?.email || license.attributes?.name || 'Unknown';

    return {
      valid: true,
      licensee,
      tier: 'enterprise'
    };

  } catch (error) {
    console.error('Enterprise license validation error:', error);
    return {
      valid: false,
      error: `License validation failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    };
  }
}

/**
 * Helper: Convert base64 string to Uint8Array
 */
function base64ToUint8Array(base64: string, urlSafe: boolean = false): Uint8Array {
  // Handle URL-safe base64
  if (urlSafe) {
    base64 = base64.replace(/-/g, '+').replace(/_/g, '/');
    // Add padding if needed
    while (base64.length % 4 !== 0) {
      base64 += '=';
    }
  }

  const binaryString = atob(base64);
  const bytes = new Uint8Array(binaryString.length);
  for (let i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes;
}
