/**
 * Unified Fetch Client
 *
 * Centralized fetch wrapper with consistent error handling,
 * timeout support, and response parsing.
 *
 * Replaces inconsistent fetch patterns across:
 * - useGraphData
 * - useNodeExpansion
 * - queryStore
 * - useDatabaseSchema
 */

// =============================================================================
// TYPES
// =============================================================================

export interface FetchOptions extends RequestInit {
  /** Request timeout in milliseconds (default: 30000) */
  timeout?: number;
  /** Whether to parse response as JSON (default: true) */
  parseJson?: boolean;
}

export interface FetchResult<T> {
  data: T;
  status: number;
  ok: boolean;
}

export class FetchError extends Error {
  constructor(
    message: string,
    public status?: number,
    public statusText?: string,
    public data?: unknown
  ) {
    super(message);
    this.name = 'FetchError';
  }
}

// =============================================================================
// CONSTANTS
// =============================================================================

const DEFAULT_TIMEOUT = 30000; // 30 seconds
const DEFAULT_HEADERS = {
  'Content-Type': 'application/json',
};

// =============================================================================
// FETCH CLIENT
// =============================================================================

/**
 * Fetch with timeout support
 * Respects caller's abort signal while adding timeout protection
 */
async function fetchWithTimeout(
  url: string,
  options: FetchOptions
): Promise<Response> {
  const { timeout = DEFAULT_TIMEOUT, signal: callerSignal, ...fetchOptions } = options;

  // If caller provided a signal, use it directly (they handle their own abort)
  // This allows AbortController from the caller to work properly
  if (callerSignal) {
    return fetch(url, { ...fetchOptions, signal: callerSignal });
  }

  // No caller signal - add our own timeout protection
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, {
      ...fetchOptions,
      signal: controller.signal,
    });
    return response;
  } finally {
    clearTimeout(timeoutId);
  }
}

/**
 * Unified fetch function with consistent error handling
 *
 * @example
 * // GET request
 * const data = await fetchJSON<User[]>('/api/users');
 *
 * @example
 * // POST request
 * const result = await fetchJSON<CreateResponse>('/api/users', {
 *   method: 'POST',
 *   body: JSON.stringify({ name: 'John' }),
 * });
 *
 * @example
 * // With custom timeout
 * const data = await fetchJSON<Data>('/api/slow', { timeout: 60000 });
 */
export async function fetchJSON<T>(
  url: string,
  options: FetchOptions = {}
): Promise<T> {
  const { parseJson = true, ...fetchOptions } = options;

  // Merge default headers
  const headers = {
    ...DEFAULT_HEADERS,
    ...fetchOptions.headers,
  };

  try {
    const response = await fetchWithTimeout(url, { ...fetchOptions, headers });

    // Parse response
    let data: unknown;
    if (parseJson) {
      try {
        data = await response.json();
      } catch {
        // JSON parse failed, might be empty response
        data = null;
      }
    } else {
      data = await response.text();
    }

    // Check for API-level errors (success: false in body)
    if (parseJson && data && typeof data === 'object' && 'success' in data) {
      const apiResponse = data as { success: boolean; error?: string };
      if (!apiResponse.success) {
        throw new FetchError(
          apiResponse.error || 'API request failed',
          response.status,
          response.statusText,
          data
        );
      }
    }

    // Check HTTP status
    if (!response.ok) {
      const errorMessage =
        (data && typeof data === 'object' && 'error' in data
          ? (data as { error: string }).error
          : null) ||
        `HTTP ${response.status}: ${response.statusText}`;

      throw new FetchError(
        errorMessage,
        response.status,
        response.statusText,
        data
      );
    }

    return data as T;
  } catch (error) {
    // Let AbortError bubble up unchanged so callers can detect manual abort
    if (error instanceof Error && error.name === 'AbortError') {
      throw error;
    }

    // Re-throw FetchError
    if (error instanceof FetchError) {
      throw error;
    }

    // Wrap other errors
    throw new FetchError(
      error instanceof Error ? error.message : 'Unknown fetch error'
    );
  }
}

/**
 * POST convenience wrapper
 */
export async function postJSON<T, B = unknown>(
  url: string,
  body: B,
  options: Omit<FetchOptions, 'method' | 'body'> = {}
): Promise<T> {
  return fetchJSON<T>(url, {
    ...options,
    method: 'POST',
    body: JSON.stringify(body),
  });
}

/**
 * Extract error message from any error type
 */
export function getErrorMessage(error: unknown): string {
  if (error instanceof FetchError) {
    return error.message;
  }
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === 'string') {
    return error;
  }
  return 'An unknown error occurred';
}
