/**
 * MCP-Guard Binary Download Worker
 *
 * Validates licenses and serves Pro/Enterprise binaries from R2 storage.
 */

import { validateProLicense, validateEnterpriseLicense } from './license-validator';
import { detectPlatform } from './platform-detector';
import type { Env, Tier, Platform } from './types';

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    // Health check endpoint
    if (url.pathname === '/health') {
      return new Response(JSON.stringify({ status: 'ok', service: 'mcp-guard-download' }), {
        headers: { 'Content-Type': 'application/json' }
      });
    }

    // Download endpoint
    if (url.pathname === '/download' || url.pathname === '/') {
      return handleDownload(request, env, url);
    }

    return new Response('Not Found', { status: 404 });
  }
};

async function handleDownload(request: Request, env: Env, url: URL): Promise<Response> {
  try {
    // Parse query parameters
    const license = url.searchParams.get('license');
    const tierParam = url.searchParams.get('tier');
    const platformParam = url.searchParams.get('platform');

    // Validate required parameters
    if (!license) {
      return new Response(
        JSON.stringify({
          error: 'Missing license parameter',
          message: 'Please provide your license key using ?license=YOUR_KEY',
          documentation: 'https://mcp-guard.io/docs/installation'
        }),
        {
          status: 400,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    if (!tierParam) {
      return new Response(
        JSON.stringify({
          error: 'Missing tier parameter',
          message: 'Please specify tier using ?tier=pro or ?tier=enterprise'
        }),
        {
          status: 400,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Validate tier
    const tier = tierParam.toLowerCase() as Tier;
    if (tier !== 'pro' && tier !== 'enterprise') {
      return new Response(
        JSON.stringify({
          error: 'Invalid tier',
          message: `Tier must be 'pro' or 'enterprise', got '${tierParam}'`
        }),
        {
          status: 400,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Detect or validate platform
    let platform: Platform;
    if (platformParam) {
      platform = platformParam as Platform;
      // Validate platform
      const validPlatforms: Platform[] = [
        'x86_64-linux',
        'x86_64-linux-musl',
        'x86_64-darwin',
        'aarch64-darwin',
        'x86_64-windows'
      ];
      if (!validPlatforms.includes(platform)) {
        return new Response(
          JSON.stringify({
            error: 'Invalid platform',
            message: `Platform must be one of: ${validPlatforms.join(', ')}`,
            received: platformParam
          }),
          {
            status: 400,
            headers: { 'Content-Type': 'application/json' }
          }
        );
      }
    } else {
      // Auto-detect from User-Agent
      const userAgent = request.headers.get('User-Agent') || '';
      const detectedPlatform = detectPlatform(userAgent);
      if (!detectedPlatform) {
        return new Response(
          JSON.stringify({
            error: 'Cannot detect platform',
            message: 'Please specify platform using ?platform=PLATFORM',
            hint: 'Valid platforms: x86_64-linux, x86_64-linux-musl, x86_64-darwin, aarch64-darwin, x86_64-windows'
          }),
          {
            status: 400,
            headers: { 'Content-Type': 'application/json' }
          }
        );
      }
      platform = detectedPlatform;
    }

    // Validate license
    const validation = tier === 'pro'
      ? await validateProLicense(license, env)
      : await validateEnterpriseLicense(license, env);

    if (!validation.valid) {
      return new Response(
        JSON.stringify({
          error: 'License validation failed',
          message: validation.error,
          tier,
          action: tier === 'pro'
            ? 'Purchase a Pro license at https://mcp-guard.io/pricing'
            : 'Contact sales at sales@mcp-guard.io for Enterprise licenses'
        }),
        {
          status: 403,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Determine binary path in R2
    // Format: latest/mcp-guard-{platform}{.exe for Windows}
    const isWindows = platform === 'x86_64-windows';
    const binaryName = isWindows ? 'mcp-guard.exe' : 'mcp-guard';
    const objectKey = `latest/${binaryName}-${platform}`;

    // Fetch from appropriate R2 bucket
    const bucket = tier === 'pro' ? env.PRO_BINARIES : env.ENTERPRISE_BINARIES;
    const object = await bucket.get(objectKey);

    if (!object) {
      console.error(`Binary not found: ${objectKey} in ${tier} bucket`);
      return new Response(
        JSON.stringify({
          error: 'Binary not found',
          message: `Binary for platform '${platform}' is not available yet`,
          tier,
          platform,
          contact: 'Please contact support@mcp-guard.io if this persists'
        }),
        {
          status: 404,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Stream binary to client
    const headers = new Headers({
      'Content-Type': 'application/octet-stream',
      'Content-Disposition': `attachment; filename="${binaryName}"`,
      'X-License-Tier': tier,
      'X-License-Licensee': validation.licensee || 'Unknown',
      'Cache-Control': 'no-cache, no-store, must-revalidate'
    });

    // Add Content-Length if available
    if (object.size) {
      headers.set('Content-Length', object.size.toString());
    }

    console.log(`Serving ${tier} binary for ${platform} to ${validation.licensee}`);

    return new Response(object.body, {
      status: 200,
      headers
    });

  } catch (error) {
    console.error('Download handler error:', error);
    return new Response(
      JSON.stringify({
        error: 'Internal server error',
        message: error instanceof Error ? error.message : 'Unknown error occurred',
        contact: 'Please contact support@mcp-guard.io'
      }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }
}
