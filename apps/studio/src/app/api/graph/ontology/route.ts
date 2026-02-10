/**
 * Schema Graph API Route (Query-First Architecture)
 *
 * Returns the ontological schema graph (60 node types + 114 arc kinds)
 * directly from Neo4j meta-nodes. No TypeScript generation - pure Cypher.
 *
 * Meta-graph structure in Neo4j:
 * - :Meta:Kind (60) - node type definitions from YAML
 * - :Meta:ArcKind (114) - arc type definitions from YAML
 * - [:FROM_KIND] ArcKind → Kind (source)
 * - [:TO_KIND] ArcKind → Kind (target)
 *
 * Query-First Architecture Flow:
 * YAML → Rust Generator → Cypher Seed → Neo4j ← Cypher Query ← Studio
 *
 * @example GET /api/graph/ontology
 */

import { NextResponse } from 'next/server';
import neo4j from 'neo4j-driver';
import { getDriver } from '@/lib/neo4j';
import type { GraphNode, GraphEdge, NodeType, RelationType } from '@/types';

// =============================================================================
// Meta-Graph Cypher Queries
// =============================================================================

/**
 * Query to fetch all Kind nodes with their properties.
 * Returns node properties directly to avoid label parsing issues.
 */
const KINDS_QUERY = `
MATCH (k:Meta:Kind)
RETURN
  k.label AS label,
  k.key AS key,
  k.realm AS realm,
  k.layer AS layer,
  k.trait AS trait,
  k.display_name AS displayName,
  k.llm_context AS llmContext,
  k.properties AS properties,
  k.required_properties AS requiredProperties,
  k.schema_hint AS schemaHint,
  k.context_budget AS contextBudget,
  k.visibility AS visibility
ORDER BY k.realm, k.layer, k.label
`;

/**
 * Query to fetch all schema edges (ArcKind → source/target Kind).
 * Returns flattened data for edge construction.
 */
const ARCS_QUERY = `
MATCH (source:Meta:Kind)<-[:FROM_KIND]-(ak:Meta:ArcKind)-[:TO_KIND]->(target:Meta:Kind)
RETURN
  ak.key AS arcKey,
  ak.display_name AS arcDisplayName,
  ak.llm_context AS arcLlmContext,
  ak.family AS family,
  ak.scope AS scope,
  ak.cardinality AS cardinality,
  source.label AS sourceLabel,
  target.label AS targetLabel
`;

// =============================================================================
// Configuration
// =============================================================================

/** Schema queries should complete quickly (5 seconds max) */
const SCHEMA_QUERY_TIMEOUT = 5000;

// =============================================================================
// GET /api/graph/ontology - Fetch schema graph from Neo4j
// =============================================================================

export async function GET() {
  const startTime = Date.now();
  const driver = getDriver();

  // Session created inside try for proper error handling
  let session: ReturnType<typeof driver.session> | null = null;

  try {
    session = driver.session({ defaultAccessMode: neo4j.session.READ });

    // Execute both queries in parallel with timeout
    const [kindsResult, arcsResult] = await Promise.all([
      session.run(KINDS_QUERY, {}, { timeout: SCHEMA_QUERY_TIMEOUT }),
      session.run(ARCS_QUERY, {}, { timeout: SCHEMA_QUERY_TIMEOUT }),
    ]);

    // Transform Kind records to GraphNodes
    const nodes: GraphNode[] = kindsResult.records.map((record) => {
      const label = record.get('label') as string;
      return {
        id: `schema-${label}`,
        type: label as NodeType,
        key: record.get('key') as string || label.toLowerCase(),
        displayName: record.get('displayName') as string || label,
        description: record.get('llmContext') as string | undefined,
        llmContext: record.get('llmContext') as string | undefined,
        data: {
          isSchema: true,
          scope: record.get('realm') as string | undefined,
          behavior: record.get('trait') as string | undefined,
          category: record.get('layer') as string | undefined,
          properties: record.get('properties') as string[] | undefined,
          requiredProperties: record.get('requiredProperties') as string[] | undefined,
          schemaHint: record.get('schemaHint') as string | undefined,
          contextBudget: record.get('contextBudget') as string | undefined,
          visibility: record.get('visibility') as string | undefined,
        },
      };
    });

    // Transform Arc records to GraphEdges (with deduplication)
    const edges: GraphEdge[] = [];
    const seenEdges = new Set<string>();
    let edgeCounter = 0;

    for (const record of arcsResult.records) {
      const arcKey = record.get('arcKey') as string;
      const sourceLabel = record.get('sourceLabel') as string;
      const targetLabel = record.get('targetLabel') as string;
      const edgeKey = `${sourceLabel}-${arcKey}-${targetLabel}`;

      if (!seenEdges.has(edgeKey)) {
        seenEdges.add(edgeKey);
        edges.push({
          id: `schema-edge-${edgeCounter++}`,
          type: arcKey as RelationType,
          source: `schema-${sourceLabel}`,
          target: `schema-${targetLabel}`,
          data: {
            isSchema: true,
            description: record.get('arcLlmContext') as string | undefined,
            cardinality: record.get('cardinality') as string | undefined,
            family: record.get('family') as string | undefined,
            scope: record.get('scope') as string | undefined,
          },
        });
      }
    }

    return NextResponse.json({
      success: true,
      data: {
        nodes,
        edges,
      },
      meta: {
        totalNodes: nodes.length,
        totalArcs: edges.length,
        duration: Date.now() - startTime,
        mode: 'meta',
        source: 'neo4j', // Indicates Query-First architecture
      },
    });
  } catch (error) {
    console.error('[/api/graph/ontology] Error:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch meta-graph from Neo4j',
        details: error instanceof Error ? error.message : String(error),
      },
      { status: 500 }
    );
  } finally {
    if (session) await session.close();
  }
}
