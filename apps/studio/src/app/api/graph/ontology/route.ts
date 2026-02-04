/**
 * Schema Graph API Route
 *
 * Returns the ontological schema graph (46 node types + relationships)
 * instead of real data. Used for "Schema Mode" visualization.
 *
 * @example GET /api/graph/schema
 */

import { NextResponse } from 'next/server';
import { generateSchemaGraph } from '@/lib/schemaGenerator';

// =============================================================================
// GET /api/graph/schema - Fetch schema graph (ontology)
// =============================================================================

export async function GET() {
  const startTime = Date.now();

  try {
    const result = generateSchemaGraph();

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.nodes.length,
        totalEdges: result.edges.length,
        duration: Date.now() - startTime,
        mode: 'meta',
      },
    });
  } catch (error) {
    console.error('[/api/graph/schema] Error:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to generate schema graph',
      },
      { status: 500 }
    );
  }
}
