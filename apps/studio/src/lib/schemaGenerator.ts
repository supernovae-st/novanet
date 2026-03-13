/**
 * Schema Graph Generator
 *
 * NOTE: Use Query-First Architecture instead via /api/graph/ontology.
 * See: apps/studio/src/app/api/graph/ontology/route.ts
 *
 * Query-First Architecture Flow:
 * YAML → Rust Generator → Cypher Seed → Neo4j ← Cypher Query ← Studio
 *
 * ---
 *
 * (Legacy) Generates the ontological schema graph (60 node types + relationships)
 * for "Schema Mode" visualization.
 *
 * v0.19.0: 60 nodes, 10 layers, 2 realms (*Native Pattern architecture)
 * Uses RelationRegistry from @novanet/core as single source of truth
 * for relations. This ensures the schema visualization is always in sync
 * with the actual graph schema.
 */

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, type Realm } from '@novanet/core/types';
import { RelationRegistry } from '@novanet/core/schemas';
import { nodeTypeConfigs } from '@/config/nodeTypes';
import type { GraphNode, GraphEdge } from '@/types';

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  shared: 'Universal knowledge (READ-ONLY)',
  org: 'Organization-specific content and structure',
};

// v11.8: Renamed per ADR-024 Data Origin semantics
const BEHAVIOR_DESCRIPTIONS: Record<string, string> = {
  defined: 'Structurally fixed, version-controlled definitions',  // was: invariant
  authored: 'Human-authored locale-specific content',             // was: localized
  imported: 'External data from authoritative sources',           // was: knowledge
  generated: 'LLM-generated output',
  retrieved: 'Computed/aggregated from external APIs',            // was: aggregated (derived)
};

// =============================================================================
// SCHEMA GENERATOR
// =============================================================================

export interface SchemaGraphResult {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

/**
 * Generate schema graph with all 60 node types and their relationships
 * v0.19.0: 60 nodes, 10 layers, 2 realms
 *
 * NOTE: Use /api/graph/ontology instead (Query-First Architecture)
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: GraphNode[] = [];
  const edges: GraphEdge[] = [];

  // Generate nodes for all 60 node types
  for (const nodeType of NODE_TYPES) {
    const config = nodeTypeConfigs[nodeType];
    const realm = NODE_REALMS[nodeType];
    const behavior = NODE_TRAITS[nodeType];

    nodes.push({
      id: `schema-${nodeType}`,
      type: nodeType,
      key: nodeType.toLowerCase(),
      displayName: config.label,
      nodeClass: nodeType,
      content: `${REALM_DESCRIPTIONS[realm]}. ${BEHAVIOR_DESCRIPTIONS[behavior]}.`,
      triggers: [nodeType.toLowerCase(), realm, behavior, config.layer],
      provenance: 'seed',
      data: {
        isSchema: true,
        scope: realm,
        behavior,
        category: config.layer,
        icon: config.icon,
        color: config.color,
      },
    });
  }

  // Generate edges from RelationRegistry (single source of truth)
  // This ensures schema visualization matches relations.yaml exactly
  let edgeId = 0;
  for (const relation of Object.values(RelationRegistry)) {
    const fromTypes = Array.isArray(relation.from) ? relation.from : [relation.from];
    const toTypes = Array.isArray(relation.to) ? relation.to : [relation.to];

    // Create Cartesian product of edges for multi-type relations
    for (const from of fromTypes) {
      for (const to of toTypes) {
        edges.push({
          id: `schema-edge-${edgeId++}`,
          type: relation.type as string,
          source: `schema-${from}`,
          target: `schema-${to}`,
          data: {
            isSchema: true,
            description: relation.description,
            cardinality: relation.cardinality,
          },
        });
      }
    }
  }

  return { nodes, edges };
}

