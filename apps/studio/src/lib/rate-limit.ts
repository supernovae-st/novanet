/**
 * Rate Limiting Utility
 *
 * Simple in-memory rate limiting for API routes.
 * For production, consider Redis-based rate limiting.
 *
 * @example
 * import { rateLimit, RateLimitConfig } from '@/lib/rate-limit';
 *
 * const limiter = rateLimit({
 *   interval: 60 * 1000, // 1 minute
 *   maxRequests: 60,     // 60 requests per minute
 * });
 *
 * export async function POST(request: NextRequest) {
 *   const clientId = getClientId(request);
 *   const result = limiter.check(clientId);
 *
 *   if (!result.allowed) {
 *     return NextResponse.json(
 *       { error: 'Rate limit exceeded' },
 *       { status: 429, headers: result.headers }
 *     );
 *   }
 *
 *   // Continue with request
 * }
 */

import { NextResponse } from 'next/server';

// =============================================================================
// Types
// =============================================================================

export interface RateLimitConfig {
  /** Time window in milliseconds */
  interval: number;
  /** Maximum requests per interval */
  maxRequests: number;
  /** Unique identifier for this limiter (for headers) */
  name?: string;
}

export interface RateLimitResult {
  /** Whether the request is allowed */
  allowed: boolean;
  /** Remaining requests in current window */
  remaining: number;
  /** When the rate limit resets (epoch ms) */
  resetAt: number;
  /** Headers to include in response */
  headers: Record<string, string>;
}

interface RateLimitEntry {
  count: number;
  resetAt: number;
}

// =============================================================================
// Rate Limiter
// =============================================================================

/**
 * Create a rate limiter with the given configuration
 */
export function rateLimit(config: RateLimitConfig) {
  const { interval, maxRequests, name = 'api' } = config;
  const store = new Map<string, RateLimitEntry>();

  // Cleanup old entries periodically
  const cleanupInterval = setInterval(() => {
    const now = Date.now();
    for (const [key, entry] of store.entries()) {
      if (entry.resetAt <= now) {
        store.delete(key);
      }
    }
  }, interval);

  // Don't block process exit
  if (typeof cleanupInterval.unref === 'function') {
    cleanupInterval.unref();
  }

  return {
    /**
     * Check if request is allowed for the given client
     */
    check(clientId: string): RateLimitResult {
      const now = Date.now();
      let entry = store.get(clientId);

      // Reset if window expired
      if (!entry || entry.resetAt <= now) {
        entry = {
          count: 0,
          resetAt: now + interval,
        };
        store.set(clientId, entry);
      }

      entry.count++;

      const allowed = entry.count <= maxRequests;
      const remaining = Math.max(0, maxRequests - entry.count);

      const headers: Record<string, string> = {
        'X-RateLimit-Limit': String(maxRequests),
        'X-RateLimit-Remaining': String(remaining),
        'X-RateLimit-Reset': String(Math.ceil(entry.resetAt / 1000)),
        'X-RateLimit-Policy': `${maxRequests};w=${Math.ceil(interval / 1000)};name="${name}"`,
      };

      if (!allowed) {
        headers['Retry-After'] = String(Math.ceil((entry.resetAt - now) / 1000));
      }

      return { allowed, remaining, resetAt: entry.resetAt, headers };
    },

    /**
     * Reset rate limit for a client (e.g., after successful auth)
     */
    reset(clientId: string): void {
      store.delete(clientId);
    },

    /**
     * Get current stats for monitoring
     */
    stats(): { clients: number; config: RateLimitConfig } {
      return {
        clients: store.size,
        config: { interval, maxRequests, name },
      };
    },
  };
}

// =============================================================================
// Pre-configured Limiters
// =============================================================================

/**
 * Default rate limiter for graph queries
 * 100 requests per minute per client
 */
export const graphQueryLimiter = rateLimit({
  interval: 60 * 1000,
  maxRequests: 100,
  name: 'graph-query',
});

/**
 * Stricter rate limiter for AI chat
 * 20 requests per minute per client (more expensive)
 */
export const chatLimiter = rateLimit({
  interval: 60 * 1000,
  maxRequests: 20,
  name: 'ai-chat',
});

/**
 * Custom query rate limiter
 * 30 requests per minute per client (potentially expensive)
 */
export const customQueryLimiter = rateLimit({
  interval: 60 * 1000,
  maxRequests: 30,
  name: 'custom-query',
});

// =============================================================================
// Middleware Helper
// =============================================================================

/**
 * Create rate limit response when limit exceeded
 */
export function rateLimitExceeded(result: RateLimitResult): NextResponse {
  return NextResponse.json(
    {
      error: 'Rate limit exceeded',
      code: 'RATE_LIMIT_EXCEEDED',
      retryAfter: Math.ceil((result.resetAt - Date.now()) / 1000),
    },
    {
      status: 429,
      headers: result.headers,
    }
  );
}
