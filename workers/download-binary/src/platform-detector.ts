/**
 * Platform detection from User-Agent headers
 */

import type { Platform } from './types';

/**
 * Detect platform from User-Agent string
 *
 * @param userAgent - The User-Agent header value
 * @returns Platform identifier or null if cannot detect
 */
export function detectPlatform(userAgent: string): Platform | null {
  const ua = userAgent.toLowerCase();

  // Windows detection
  if (ua.includes('windows') || ua.includes('win64') || ua.includes('win32')) {
    return 'x86_64-windows';
  }

  // macOS detection
  if (ua.includes('macintosh') || ua.includes('mac os x') || ua.includes('darwin')) {
    // Check for ARM (M1/M2/M3) vs Intel
    if (ua.includes('arm64') || ua.includes('aarch64')) {
      return 'aarch64-darwin';
    }
    return 'x86_64-darwin';
  }

  // Linux detection
  if (ua.includes('linux')) {
    // Check for musl vs glibc
    // Note: Most User-Agents won't specify this, default to gnu (glibc)
    if (ua.includes('musl') || ua.includes('alpine')) {
      return 'x86_64-linux-musl';
    }
    return 'x86_64-linux';
  }

  // curl detection (common for installation scripts)
  if (ua.startsWith('curl/')) {
    // Try to detect OS from environment
    // This is a best-effort fallback
    return null; // Caller should request platform explicitly
  }

  // wget detection
  if (ua.startsWith('wget/')) {
    return null; // Caller should request platform explicitly
  }

  return null;
}

/**
 * Get human-readable platform name
 *
 * @param platform - Platform identifier
 * @returns Human-readable name
 */
export function getPlatformName(platform: Platform): string {
  switch (platform) {
    case 'x86_64-linux':
      return 'Linux (x86_64, glibc)';
    case 'x86_64-linux-musl':
      return 'Linux (x86_64, musl)';
    case 'x86_64-darwin':
      return 'macOS (Intel)';
    case 'aarch64-darwin':
      return 'macOS (Apple Silicon)';
    case 'x86_64-windows':
      return 'Windows (x86_64)';
    default:
      return platform;
  }
}

/**
 * Validate platform identifier
 *
 * @param platform - Platform string to validate
 * @returns True if valid platform
 */
export function isValidPlatform(platform: string): platform is Platform {
  const validPlatforms: Platform[] = [
    'x86_64-linux',
    'x86_64-linux-musl',
    'x86_64-darwin',
    'aarch64-darwin',
    'x86_64-windows'
  ];
  return validPlatforms.includes(platform as Platform);
}
