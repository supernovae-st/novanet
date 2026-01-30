/**
 * API Authentication Middleware
 *
 * Provides authentication and authorization for API routes.
 * Currently implements API key authentication with optional rate limiting.
 *
 * @example
 * // In an API route:
 * import { withAuth } from '@/lib/auth';
 *
 * export const POST = withAuth(async (request) => {
 *   // Authenticated request handling
 *   return NextResponse.json({ success: true });
 * });
 */

import { NextRequest, NextResponse } from 'next/server';
import { logger } from './logger';

// =============================================================================
// Configuration
// =============================================================================

/**
 * Authentication mode:
 * - 'disabled': No authentication (development only)
 * - 'api-key': Require API key in Authorization header
 * - 'jwt': Require JWT token (future)
 */
const AUTH_MODE = process.env.AUTH_MODE || 'disabled';

/**
 * Valid API keys (comma-separated in env)
 * In production, use a proper secrets manager
 */
const API_KEYS = new Set(
  (process.env.API_KEYS || '')
    .split(',')
    .map((key) => key.trim())
    .filter(Boolean)
);

/**
 * Paths that skip authentication
 */
const PUBLIC_PATHS = new Set([
  '/api/health',
  '/api/graph/schema', // Schema is public for introspection
]);

// =============================================================================
// Types
// =============================================================================

export interface AuthContext {
  /** The authenticated API key (if any) */
  apiKey?: string;
  /** Whether the request is authenticated */
  authenticated: boolean;
  /** Client identifier for rate limiting */
  clientId: string;
}

export type AuthenticatedHandler = (
  request: NextRequest,
  context: AuthContext
) => Promise<NextResponse>;

// =============================================================================
// Middleware
// =============================================================================

/**
 * Extract API key from Authorization header
 * Supports: "Bearer <key>" or "ApiKey <key>"
 */
function extractApiKey(request: NextRequest): string | null {
  const authHeader = request.headers.get('authorization');
  if (!authHeader) return null;

  const [scheme, key] = authHeader.split(' ');
  if (!key) return null;

  if (scheme.toLowerCase() === 'bearer' || scheme.toLowerCase() === 'apikey') {
    return key;
  }

  return null;
}

/**
 * Get client identifier for rate limiting
 * Uses X-Forwarded-For, X-Real-IP, or falls back to 'anonymous'
 */
function getClientId(request: NextRequest): string {
  const forwarded = request.headers.get('x-forwarded-for');
  if (forwarded) {
    return forwarded.split(',')[0].trim();
  }

  const realIp = request.headers.get('x-real-ip');
  if (realIp) {
    return realIp;
  }

  return 'anonymous';
}

/**
 * Check if path is public (skips auth)
 */
function isPublicPath(pathname: string): boolean {
  return PUBLIC_PATHS.has(pathname);
}

/**
 * Validate API key
 */
function validateApiKey(key: string): boolean {
  return API_KEYS.has(key);
}

/**
 * Authentication middleware wrapper
 *
 * @example
 * export const POST = withAuth(async (request, context) => {
 *   console.log('Client:', context.clientId);
 *   return NextResponse.json({ success: true });
 * });
 */
export function withAuth(handler: AuthenticatedHandler) {
  return async (request: NextRequest): Promise<NextResponse> => {
    const pathname = new URL(request.url).pathname;
    const clientId = getClientId(request);

    // Skip auth for public paths
    if (isPublicPath(pathname)) {
      return handler(request, {
        authenticated: false,
        clientId,
      });
    }

    // Auth disabled (development mode)
    if (AUTH_MODE === 'disabled') {
      if (process.env.NODE_ENV === 'production') {
        logger.warn('Auth', 'Authentication disabled in production - this is insecure!');
      }
      return handler(request, {
        authenticated: false,
        clientId,
      });
    }

    // API key authentication
    if (AUTH_MODE === 'api-key') {
      const apiKey = extractApiKey(request);

      if (!apiKey) {
        return NextResponse.json(
          { error: 'Missing API key', code: 'UNAUTHORIZED' },
          { status: 401 }
        );
      }

      if (!validateApiKey(apiKey)) {
        logger.warn('Auth', `Invalid API key attempt from ${clientId}`);
        return NextResponse.json(
          { error: 'Invalid API key', code: 'FORBIDDEN' },
          { status: 403 }
        );
      }

      return handler(request, {
        apiKey,
        authenticated: true,
        clientId,
      });
    }

    // Unknown auth mode
    logger.error('Auth', `Unknown AUTH_MODE: ${AUTH_MODE}`);
    return NextResponse.json(
      { error: 'Server configuration error', code: 'INTERNAL_ERROR' },
      { status: 500 }
    );
  };
}

/**
 * Simple auth check without wrapping (for use in existing routes)
 *
 * @example
 * export async function POST(request: NextRequest) {
 *   const auth = checkAuth(request);
 *   if (!auth.ok) {
 *     return auth.response;
 *   }
 *   // Continue with authenticated request
 * }
 */
export function checkAuth(request: NextRequest): {
  ok: boolean;
  response?: NextResponse;
  context: AuthContext;
} {
  const pathname = new URL(request.url).pathname;
  const clientId = getClientId(request);

  // Skip auth for public paths or disabled mode
  if (isPublicPath(pathname) || AUTH_MODE === 'disabled') {
    return {
      ok: true,
      context: { authenticated: false, clientId },
    };
  }

  // API key authentication
  if (AUTH_MODE === 'api-key') {
    const apiKey = extractApiKey(request);

    if (!apiKey) {
      return {
        ok: false,
        response: NextResponse.json(
          { error: 'Missing API key', code: 'UNAUTHORIZED' },
          { status: 401 }
        ),
        context: { authenticated: false, clientId },
      };
    }

    if (!validateApiKey(apiKey)) {
      return {
        ok: false,
        response: NextResponse.json(
          { error: 'Invalid API key', code: 'FORBIDDEN' },
          { status: 403 }
        ),
        context: { authenticated: false, clientId },
      };
    }

    return {
      ok: true,
      context: { apiKey, authenticated: true, clientId },
    };
  }

  return {
    ok: true,
    context: { authenticated: false, clientId },
  };
}
