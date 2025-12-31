/**
 * MCP-Guard License Signing Worker
 *
 * Generates and signs Pro license keys using Ed25519 signatures.
 * Protected by API secret key - only callable by authorized services (Stripe webhook, admin portal).
 */

import { ed25519 } from '@noble/ed25519';
import type {
  Env,
  SignLicenseRequest,
  SignLicenseResponse,
  ProLicensePayload,
  ErrorResponse
} from './types';

// Default Pro features
const DEFAULT_PRO_FEATURES = [
  'oauth',
  'http_transport',
  'sse_transport',
  'per_identity_rate_limit',
  'jwt_jwks'
];

// Default license duration: 1 year
const DEFAULT_LICENSE_DURATION_MS = 365 * 24 * 60 * 60 * 1000;

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    // CORS headers for development
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, X-API-Secret'
    };

    // Handle OPTIONS request (CORS preflight)
    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    // Health check endpoint
    if (request.url.endsWith('/health')) {
      return new Response(
        JSON.stringify({ status: 'ok', service: 'mcp-guard-license-signer' }),
        { headers: { 'Content-Type': 'application/json', ...corsHeaders } }
      );
    }

    // Sign license endpoint
    if (request.method === 'POST' && (request.url.endsWith('/sign') || request.url.endsWith('/'))) {
      return handleSignLicense(request, env, corsHeaders);
    }

    return new Response('Not Found', { status: 404, headers: corsHeaders });
  }
};

/**
 * Handle license signing request
 */
async function handleSignLicense(
  request: Request,
  env: Env,
  corsHeaders: Record<string, string>
): Promise<Response> {
  try {
    // Authenticate request
    const apiSecret = request.headers.get('X-API-Secret');
    if (!apiSecret || apiSecret !== env.API_SECRET_KEY) {
      return errorResponse(
        'Unauthorized',
        'Invalid or missing API secret key',
        401,
        corsHeaders
      );
    }

    // Parse request body
    let body: SignLicenseRequest;
    try {
      body = await request.json();
    } catch (error) {
      return errorResponse(
        'Invalid request',
        'Request body must be valid JSON',
        400,
        corsHeaders,
        error
      );
    }

    // Validate required fields
    if (!body.licensee || typeof body.licensee !== 'string') {
      return errorResponse(
        'Invalid request',
        'Missing or invalid "licensee" field (must be a string)',
        400,
        corsHeaders
      );
    }

    // Create license payload
    const now = new Date();
    const expiresAt = body.expires_at
      ? new Date(body.expires_at)
      : new Date(now.getTime() + DEFAULT_LICENSE_DURATION_MS);

    // Validate expiry is in the future
    if (expiresAt <= now) {
      return errorResponse(
        'Invalid request',
        'expires_at must be in the future',
        400,
        corsHeaders
      );
    }

    const payload: ProLicensePayload = {
      tier: 'pro',
      issued_at: now.toISOString(),
      expires_at: expiresAt.toISOString(),
      licensee: body.licensee,
      features: body.features || DEFAULT_PRO_FEATURES
    };

    // Sign license
    const licenseKey = await signLicense(payload, env);

    // Return response
    const response: SignLicenseResponse = {
      license_key: licenseKey,
      payload: payload,
      expires_at: expiresAt.toISOString()
    };

    console.log(`Signed Pro license for ${payload.licensee}, expires ${expiresAt.toISOString()}`);

    return new Response(JSON.stringify(response), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
        ...corsHeaders
      }
    });

  } catch (error) {
    console.error('License signing error:', error);
    return errorResponse(
      'Internal server error',
      'Failed to sign license',
      500,
      corsHeaders,
      error
    );
  }
}

/**
 * Sign a Pro license using Ed25519
 */
async function signLicense(payload: ProLicensePayload, env: Env): Promise<string> {
  // Serialize payload to JSON and encode to base64
  const payloadJson = JSON.stringify(payload);
  const payloadB64 = btoa(payloadJson)
    .replace(/\+/g, '-')  // URL-safe: + -> -
    .replace(/\//g, '_')  // URL-safe: / -> _
    .replace(/=/g, '');   // Remove padding

  // Parse private key from PEM format
  const privateKey = parsePrivateKey(env.PRO_LICENSE_PRIVATE_KEY);

  // Sign the base64-encoded payload
  const messageBytes = new TextEncoder().encode(payloadB64);
  const signature = await ed25519.sign(messageBytes, privateKey);

  // Encode signature to URL-safe base64
  const signatureB64 = btoa(String.fromCharCode(...signature))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');

  // Construct license key: pro_<payload>.<signature>
  return `pro_${payloadB64}.${signatureB64}`;
}

/**
 * Parse Ed25519 private key from PEM format
 */
function parsePrivateKey(pem: string): Uint8Array {
  try {
    // Remove PEM headers and whitespace
    const b64 = pem
      .replace(/-----BEGIN PRIVATE KEY-----/g, '')
      .replace(/-----END PRIVATE KEY-----/g, '')
      .replace(/\s/g, '');

    // Decode base64
    const der = Uint8Array.from(atob(b64), c => c.charCodeAt(0));

    // Parse DER format to extract raw Ed25519 key (32 bytes)
    // PKCS#8 format for Ed25519:
    // - Header: 16 bytes
    // - Key data: 34 bytes (2 bytes length prefix + 32 bytes key)
    // Total: 48 bytes minimum
    if (der.length < 48) {
      throw new Error('Invalid private key length');
    }

    // Extract raw 32-byte key from PKCS#8 format
    // Skip PKCS#8 header (16 bytes) and length prefix (2 bytes)
    const rawKey = der.slice(16 + 2, 16 + 2 + 32);

    if (rawKey.length !== 32) {
      throw new Error(`Expected 32-byte Ed25519 key, got ${rawKey.length} bytes`);
    }

    return rawKey;

  } catch (error) {
    console.error('Private key parsing error:', error);
    throw new Error('Failed to parse Ed25519 private key from PEM format');
  }
}

/**
 * Helper: Create error response
 */
function errorResponse(
  error: string,
  message: string,
  status: number,
  corsHeaders: Record<string, string>,
  details?: unknown
): Response {
  const body: ErrorResponse = { error, message };
  if (details) {
    body.details = details instanceof Error ? details.message : details;
  }

  return new Response(JSON.stringify(body), {
    status,
    headers: {
      'Content-Type': 'application/json',
      ...corsHeaders
    }
  });
}
