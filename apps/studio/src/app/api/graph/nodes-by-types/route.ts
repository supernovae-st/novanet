/**
 * Nodes by Types API Route
 *
 * Fetches nodes of specific types for contextual view selection.
 * Used when user clicks a contextual view without a node selected.
 *
 * @example GET /api/graph/nodes-by-types?types=Page,Block&limit=50
 */

import { NextRequest, NextResponse } from 'next/server';
import neo4j from 'neo4j-driver';
import { getDriver } from '@/lib/neo4j';
import { logger } from '@/lib/logger';

// =============================================================================
// GET /api/graph/nodes-by-types - Fetch nodes by type(s)
// =============================================================================

export async function GET(request: NextRequest) {
  const startTime = Date.now();

  try {
    const searchParams = request.nextUrl.searchParams;
    const typesParam = searchParams.get('types');
    const limitParam = searchParams.get('limit');
    const searchParam = searchParams.get('search');

    // Validate types parameter
    if (!typesParam) {
      return NextResponse.json(
        {
          success: false,
          error: 'Missing "types" query parameter',
        },
        { status: 400 }
      );
    }

    // Parse types (comma-separated)
    const types = typesParam.split(',').map((t) => t.trim()).filter(Boolean);
    if (types.length === 0) {
      return NextResponse.json(
        {
          success: false,
          error: 'No valid types provided',
        },
        { status: 400 }
      );
    }

    // Parse limit (default 50, max 100)
    const limit = Math.min(Math.max(parseInt(limitParam || '50', 10), 1), 100);

    // Build Cypher query
    // Using UNION for multiple types to ensure we get nodes of each type
    const typeQueries = types.map((type, i) => {
      const searchClause = searchParam
        ? `WHERE n.key CONTAINS $search OR n.display_name CONTAINS $search`
        : '';
      return `
        MATCH (n:${type})
        ${searchClause}
        RETURN n.key AS key, n.display_name AS displayName, '${type}' AS type
        ORDER BY n.display_name, n.key
        LIMIT $limit
      `;
    });

    const cypher = typeQueries.join(' UNION ');

    // Execute query
    const driver = getDriver();
    const session = driver.session({ database: 'neo4j' });

    try {
      // Neo4j requires integer for LIMIT - use neo4j.int()
      const limitPerType = Math.ceil(limit / types.length);
      const result = await session.run(cypher, {
        limit: neo4j.int(limitPerType),
        ...(searchParam ? { search: searchParam } : {}),
      });

      const nodes = result.records.map((record) => ({
        key: record.get('key'),
        displayName: record.get('displayName') || record.get('key'),
        type: record.get('type'),
      }));

      // Sort by type, then by displayName
      nodes.sort((a, b) => {
        if (a.type !== b.type) return a.type.localeCompare(b.type);
        return (a.displayName || a.key).localeCompare(b.displayName || b.key);
      });

      logger.debug('API', '/graph/nodes-by-types', {
        types,
        nodeCount: nodes.length,
        duration: Date.now() - startTime,
      });

      return NextResponse.json({
        success: true,
        data: {
          nodes,
          types,
        },
        meta: {
          totalNodes: nodes.length,
          requestDuration: Date.now() - startTime,
        },
      });
    } finally {
      await session.close();
    }
  } catch (error) {
    logger.error('API', '/graph/nodes-by-types error', error);

    const errorMessage = error instanceof Error ? error.message : 'Unknown error';

    return NextResponse.json(
      {
        success: false,
        error: errorMessage,
        details: process.env.NODE_ENV === 'development' ? String(error) : undefined,
      },
      { status: 500 }
    );
  }
}
