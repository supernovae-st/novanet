/**
 * Schema Graph API Route (Query-First Architecture)
 *
 * Returns the ontological schema graph (61 node classes + 170 arc classes)
 * directly from Neo4j schema-nodes. No TypeScript generation - pure Cypher.
 *
 * Schema-graph structure in Neo4j (v0.12.4 ADR-023/ADR-028):
 * - :Schema:Class (61) - node type definitions from YAML
 * - :Schema:ArcClass (128) - arc type definitions from YAML
 * - [:FROM_CLASS] ArcClass → Class (source)
 * - [:TO_CLASS] ArcClass → Class (target)
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
 * Query to fetch all Class nodes with their properties (v11.8 ADR-023).
 * Returns node properties directly to avoid label parsing issues.
 */
const CLASSES_QUERY = `
MATCH (c:Schema:Class)
RETURN
  c.label AS label,
  c.key AS key,
  c.realm AS realm,
  c.layer AS layer,
  c.trait AS trait,
  c.display_name AS displayName,
  c.llm_context AS llmContext,
  c.properties AS properties,
  c.required_properties AS requiredProperties,
  c.schema_hint AS schemaHint,
  c.context_budget AS contextBudget,
  c.visibility AS visibility
ORDER BY c.realm, c.layer, c.label
`;

/**
 * Query to fetch all schema edges (ArcClass → source/target Class) (v11.8 ADR-023).
 * Returns flattened data for edge construction.
 */
const ARCS_QUERY = `
MATCH (source:Schema:Class)<-[:FROM_CLASS]-(ac:Schema:ArcClass)-[:TO_CLASS]->(target:Schema:Class)
RETURN
  ac.key AS arcKey,
  ac.display_name AS arcDisplayName,
  ac.llm_context AS arcLlmContext,
  ac.family AS family,
  ac.scope AS scope,
  ac.cardinality AS cardinality,
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
    const [classesResult, arcsResult] = await Promise.all([
      session.run(CLASSES_QUERY, {}, { timeout: SCHEMA_QUERY_TIMEOUT }),
      session.run(ARCS_QUERY, {}, { timeout: SCHEMA_QUERY_TIMEOUT }),
    ]);

    // Transform Class records to GraphNodes
    const nodes: GraphNode[] = classesResult.records.map((record) => {
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
        error: 'Failed to fetch schema-graph from Neo4j',
        details: error instanceof Error ? error.message : String(error),
      },
      { status: 500 }
    );
  } finally {
    if (session) await session.close();
  }
}
