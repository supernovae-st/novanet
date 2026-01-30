/**
 * Graph Statistics API Route
 *
 * Returns node counts by type for the graph.
 * Useful for displaying statistics and building filter presets.
 *
 * @example GET /api/graph/stats
 */

import { NextResponse } from 'next/server';
import { fetchGraphStats, verifyConnection } from '@/lib/neo4j';
import { logger } from '@/lib/logger';

// =============================================================================
// GET /api/graph/stats - Get node counts by type
// =============================================================================

export async function GET() {
  const startTime = Date.now();

  try {
    // Verify connection first
    const isConnected = await verifyConnection();
    if (!isConnected) {
      return NextResponse.json(
        {
          success: false,
          error: 'Unable to connect to Neo4j database',
        },
        { status: 503 }
      );
    }

    // Fetch statistics
    const stats = await fetchGraphStats();

    // Calculate total
    const total = Object.values(stats).reduce((sum, count) => sum + count, 0);

    return NextResponse.json({
      success: true,
      data: {
        byType: stats,
        total,
      },
      meta: {
        requestDuration: Date.now() - startTime,
      },
    });
  } catch (error) {
    logger.error('API', '/graph/stats error', error);

    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      },
      { status: 500 }
    );
  }
}
