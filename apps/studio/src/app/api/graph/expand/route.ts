import { NextResponse } from 'next/server';
import { fetchNodeNeighbors } from '@/lib/neo4j';
import { EXPAND_QUERY_LIMIT } from '@/config/constants';
import { handleApiError } from '@/lib/apiErrorHandler';

/**
 * POST /api/graph/expand
 * Expand a node by fetching its neighbors from Neo4j
 * Used by double-click expand feature (Neo4j Browser style)
 */
export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { nodeId } = body;
    // Cap limit to prevent DoS via large requests
    const limit = typeof body.limit === 'number'
      ? Math.min(Math.max(1, body.limit), EXPAND_QUERY_LIMIT)
      : EXPAND_QUERY_LIMIT;

    if (!nodeId) {
      return NextResponse.json(
        { success: false, error: 'nodeId is required' },
        { status: 400 }
      );
    }

    const result = await fetchNodeNeighbors(nodeId, limit);

    return NextResponse.json({
      success: true,
      nodes: result.nodes,
      edges: result.edges,
      totalNodes: result.totalNodes,
      totalArcs: result.totalArcs,
      duration: result.duration,
    });
  } catch (error) {
    return handleApiError(error, '/graph/expand POST');
  }
}
