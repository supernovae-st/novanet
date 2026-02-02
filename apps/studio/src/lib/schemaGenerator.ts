/**
 * Schema Graph Generator
 *
 * Generates the ontological schema graph (35 node types + relationships)
 * for "Schema Mode" visualization.
 *
 * v8.2.0: Uses RelationRegistry from @novanet/core as single source of truth
 * for relations. This ensures the schema visualization is always in sync
 * with the actual graph schema.
 */

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, type NodeType, type Realm } from '@novanet/core/types';
import { RelationRegistry } from '@novanet/core/schemas';
import { nodeTypeConfigs } from '@/config/nodeTypes';
import type { GraphNode, GraphEdge } from '@/types';

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  global: 'Shared across all projects (Locale knowledge)',
  shared: 'Shared across projects (SEO/GEO data)',
  project: 'Project-specific content and structure',
};

const BEHAVIOR_DESCRIPTIONS: Record<string, string> = {
  invariant: 'Language-independent, same across all locales',
  localized: 'Human-curated localized content',
  localeKnowledge: 'Locale-specific cultural/linguistic knowledge',
  derived: 'Computed from other data (metrics)',
  job: 'Background processing job',
};

// =============================================================================
// SCHEMA GENERATOR
// =============================================================================

export interface SchemaGraphResult {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

/**
 * Generate schema graph with all 35 node types and their relationships
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: GraphNode[] = [];
  const edges: GraphEdge[] = [];

  // Generate nodes for all 35 node types
  for (const nodeType of NODE_TYPES) {
    const config = nodeTypeConfigs[nodeType];
    const realm = NODE_REALMS[nodeType];
    const behavior = NODE_TRAITS[nodeType];

    nodes.push({
      id: `schema-${nodeType}`,
      type: nodeType,
      key: nodeType.toLowerCase(),
      displayName: config.label,
      description: `${REALM_DESCRIPTIONS[realm]}. ${BEHAVIOR_DESCRIPTIONS[behavior]}.`,
      llmContext: `Schema node representing the ${nodeType} type. Realm: ${realm}. Behavior: ${behavior}.`,
      data: {
        isSchema: true,
        scope: realm,
        behavior,
        category: config.category,
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

