/**
 * Graph Data API Route
 *
 * Fetches graph data from Neo4j with server-side filtering.
 * Supports node type filters, locale filter, search, and pagination.
 *
 * @example GET /api/graph?nodeTypes=Project,Page&locale=fr-FR&limit=500
 * @example POST /api/graph with body { nodeTypes: ['Project'], locale: 'fr-FR' }
 */

import { NextRequest, NextResponse } from 'next/server';
import {
  fetchGraphData,
  type QueryOptions,
} from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';
import type { NodeType } from '@/types';

// =============================================================================
// GET /api/graph - Fetch graph data with query params
// =============================================================================

export async function GET(request: NextRequest) {
  const startTime = Date.now();

  try {
    // Parse query parameters
    const searchParams = request.nextUrl.searchParams;

    const nodeTypesParam = searchParams.get('nodeTypes');
    const nodeTypes = nodeTypesParam
      ? (nodeTypesParam.split(',').filter(Boolean) as NodeType[])
      : [];

    // Parse and validate numeric params (clamp to safe ranges)
    const rawDepth = parseInt(searchParams.get('depth') || '2', 10);
    const rawLimit = parseInt(searchParams.get('limit') || '500', 10);
    const depth = Math.min(Math.max(1, isNaN(rawDepth) ? 2 : rawDepth), 5);
    const limit = Math.min(Math.max(1, isNaN(rawLimit) ? 500 : rawLimit), 5000);

    const options: QueryOptions = {
      nodeTypes,
      locale: searchParams.get('locale') || undefined,
      depth,
      limit,
      search: searchParams.get('search') || undefined,
    };

    // Fetch data from Neo4j
    const result = await fetchGraphData(options);

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.totalNodes,
        totalArcs: result.totalArcs,
        duration: result.duration,
        requestDuration: Date.now() - startTime,
        filters: options,
      },
    });
  } catch (error) {
    return handleApiError(error, '/graph GET');
  }
}

// =============================================================================
// POST /api/graph - Fetch graph data with body params (for complex filters)
// =============================================================================

export async function POST(request: NextRequest) {
  const startTime = Date.now();

  try {
    const body = await request.json();

    // Validate request body
    const options: QueryOptions = {
      nodeTypes: Array.isArray(body.nodeTypes) ? body.nodeTypes : [],
      locale: typeof body.locale === 'string' ? body.locale : undefined,
      depth: typeof body.depth === 'number' ? body.depth : 2,
      limit: typeof body.limit === 'number' ? Math.min(body.limit, 1000) : 500,
      search: typeof body.search === 'string' ? body.search : undefined,
    };

    // Fetch data from Neo4j
    const result = await fetchGraphData(options);

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.totalNodes,
        totalArcs: result.totalArcs,
        duration: result.duration,
        requestDuration: Date.now() - startTime,
        filters: options,
      },
    });
  } catch (error) {
    return handleApiError(error, '/graph POST');
  }
}
