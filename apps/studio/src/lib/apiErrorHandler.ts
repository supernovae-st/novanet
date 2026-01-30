/**
 * API Error Handler
 *
 * Standardized error handling for Next.js API routes.
 * Provides consistent error responses and logging.
 */

import { NextResponse } from 'next/server';
import { logger } from './logger';

/**
 * Create a standardized error response for API routes
 *
 * @param error - The caught error
 * @param context - Description of the API context (e.g., '/graph GET')
 * @returns NextResponse with error details
 *
 * @example
 * try {
 *   // ... API logic
 * } catch (error) {
 *   return handleApiError(error, '/graph/expand POST');
 * }
 */
export function handleApiError(error: unknown, context: string): NextResponse {
  const errorMessage = error instanceof Error ? error.message : 'Unknown error';
  const isConnectionError =
    errorMessage.includes('connection') ||
    errorMessage.includes('ECONNREFUSED') ||
    errorMessage.includes('ETIMEDOUT');

  logger.error('API', `${context} error`, {
    message: errorMessage,
    stack: error instanceof Error ? error.stack : undefined,
    isConnectionError,
  });

  return NextResponse.json(
    {
      success: false,
      error: isConnectionError
        ? 'Unable to connect to Neo4j database. Please check if the database is running.'
        : errorMessage,
      details: process.env.NODE_ENV === 'development' ? String(error) : undefined,
    },
    { status: isConnectionError ? 503 : 500 }
  );
}
