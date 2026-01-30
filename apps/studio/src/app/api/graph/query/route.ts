/**
 * Custom Query Execution API Route
 *
 * Executes custom Cypher queries (from AI chat) with safety checks.
 * Only read queries are allowed.
 *
 * @example POST /api/graph/query { cypher: "MATCH (n:Project) RETURN n LIMIT 10" }
 */

import { NextRequest, NextResponse } from 'next/server';
import { executeCustomQuery } from '@/lib/neo4j';
import { logger } from '@/lib/logger';

// =============================================================================
// POST /api/graph/query - Execute custom Cypher query
// =============================================================================

export async function POST(request: NextRequest) {
  const startTime = Date.now();

  try {
    const body = await request.json();

    // Validate request
    if (!body.cypher || typeof body.cypher !== 'string') {
      return NextResponse.json(
        {
          success: false,
          error: 'Missing or invalid "cypher" field in request body',
        },
        { status: 400 }
      );
    }

    const cypher = body.cypher.trim();
    const params = body.params || {};

    // Validate query length
    if (cypher.length > 5000) {
      return NextResponse.json(
        {
          success: false,
          error: 'Query too long (max 5000 characters)',
        },
        { status: 400 }
      );
    }

    // Execute query with safety checks
    const result = await executeCustomQuery(cypher, params);

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.totalNodes,
        totalEdges: result.totalEdges,
        queryDuration: result.duration,
        requestDuration: Date.now() - startTime,
        query: cypher,
      },
    });
  } catch (error) {
    logger.error('API', '/graph/query error', error);

    const errorMessage = error instanceof Error ? error.message : 'Unknown error';

    // Determine error type for appropriate status code
    let status = 500;
    if (errorMessage.includes('Write operations are not allowed')) {
      status = 403;
    } else if (errorMessage.includes('syntax') || errorMessage.includes('Invalid')) {
      status = 400;
    } else if (errorMessage.includes('connection') || errorMessage.includes('ECONNREFUSED')) {
      status = 503;
    }

    return NextResponse.json(
      {
        success: false,
        error: errorMessage,
        details: process.env.NODE_ENV === 'development' ? String(error) : undefined,
      },
      { status }
    );
  }
}
