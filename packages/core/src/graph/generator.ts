// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v9.0.0

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, type NodeType, type Realm } from '../types/nodes.js';
import { RelationRegistry } from '../schemas/relations.schema.js';
import type { SchemaNode, SchemaEdge, SchemaGraphResult, HierarchicalSchemaData } from './types.js';
import { getLayer } from './layers.js';
import { REALM_HIERARCHY } from './hierarchy.js';

// =============================================================================
// NODE DISPLAY LABELS
// =============================================================================

/**
 * Human-readable labels for node types.
 * Can be extended with icons/colors in UI layer.
 */
const NODE_LABELS: Record<NodeType, string> = {
  // Project realm - foundation
  Project: 'Project',
  BrandIdentity: 'Brand Identity',
  ProjectL10n: 'Project L10n',
  // Project realm - structure
  Page: 'Page',
  Block: 'Block',
  BlockType: 'Block Type',
  PageType: 'Page Type',
  // Project realm - semantic
  Concept: 'Concept',
  ConceptL10n: 'Concept L10n',
  // Project realm - instruction
  PagePrompt: 'Page Prompt',
  BlockPrompt: 'Block Prompt',
  BlockRules: 'Block Rules',
  // Project realm - output
  PageL10n: 'Page L10n',
  BlockL10n: 'Block L10n',
  // Global realm - config
  Locale: 'Locale',
  // Global realm - knowledge
  LocaleIdentity: 'Locale Identity',
  LocaleVoice: 'Locale Voice',
  LocaleCulture: 'Locale Culture',
  LocaleCultureReferences: 'Culture References',
  LocaleMarket: 'Locale Market',
  LocaleLexicon: 'Locale Lexicon',
  LocaleRulesAdaptation: 'Rules Adaptation',
  LocaleRulesFormatting: 'Rules Formatting',
  LocaleRulesSlug: 'Rules Slug',
  Expression: 'Expression',
  Reference: 'Reference',
  Metaphor: 'Metaphor',
  Pattern: 'Pattern',
  Constraint: 'Constraint',
  // Shared realm - seo
  SEOKeywordL10n: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOMiningRun: 'SEO Mining',
  // Shared realm - geo
  GEOSeedL10n: 'GEO Seed',
  GEOSeedMetrics: 'GEO Metrics',
  GEOMiningRun: 'GEO Mining',
};

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  global: 'Shared across all projects (Locale knowledge)',
  shared: 'Shared across projects (SEO/GEO data)',
  project: 'Project-specific content and structure',
};

// =============================================================================
// TRAIT DESCRIPTIONS
// =============================================================================

const TRAIT_DESCRIPTIONS: Record<string, string> = {
  invariant: 'Language-independent, same across all locales',
  localized: 'Human-curated localized content',
  knowledge: 'Locale-specific cultural/linguistic knowledge',
  derived: 'Computed from other data (metrics)',
  job: 'Background processing job',
};

// =============================================================================
// GENERATORS
// =============================================================================

/**
 * Generate flat schema graph with all 35 node types and relationships.
 * This is the canonical representation of the NovaNet ontology.
 *
 * @returns SchemaGraphResult with nodes and edges
 *
 * @example
 * ```typescript
 * const { nodes, edges } = generateSchemaGraph();
 * console.log(`${nodes.length} nodes, ${edges.length} edges`);
 * // Output: "35 nodes, ~89 edges"
 * ```
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const edges: SchemaEdge[] = [];

  // ==========================================================================
  // GENERATE NODES - All 35 node types
  // ==========================================================================

  for (const nodeType of NODE_TYPES) {
    const realm = NODE_REALMS[nodeType];
    const trait = NODE_TRAITS[nodeType];
    const layer = getLayer(nodeType);

    nodes.push({
      id: `schema-${nodeType}`,
      nodeType,
      realm,
      layer,
      label: NODE_LABELS[nodeType],
      description: `${REALM_DESCRIPTIONS[realm]}. ${TRAIT_DESCRIPTIONS[trait]}.`,
      trait,
    });
  }

  // ==========================================================================
  // GENERATE EDGES - From RelationRegistry (single source of truth)
  // ==========================================================================

  const validNodeTypes = new Set<string>(NODE_TYPES);
  let edgeId = 0;

  for (const relation of Object.values(RelationRegistry)) {
    const sourceTypes = Array.isArray(relation.from) ? relation.from : [relation.from];
    const targetTypes = Array.isArray(relation.to) ? relation.to : [relation.to];

    // Create Cartesian product of edges for multi-type relations
    for (const source of sourceTypes) {
      for (const target of targetTypes) {
        // Skip edges with invalid node types
        if (!validNodeTypes.has(source) || !validNodeTypes.has(target)) {
          continue;
        }

        edges.push({
          id: `schema-edge-${edgeId++}`,
          relationType: relation.type,
          sourceType: source as NodeType,
          targetType: target as NodeType,
          label: relation.type,
          description: relation.description,
          cardinality: relation.cardinality,
        });
      }
    }
  }

  return { nodes, edges };
}

/**
 * Generate hierarchical schema data grouped by realm and layer.
 * Used by visualizers that need grouped layout (like Studio).
 *
 * @returns HierarchicalSchemaData with realms, nodes, edges, and stats
 *
 * @example
 * ```typescript
 * const hierarchy = getSchemaHierarchy();
 * console.log(hierarchy.stats);
 * // Output: { totalNodes: 35, totalEdges: ~89, nodesByRealm: { project: 14, global: 15, shared: 6 } }
 * ```
 */
export function getSchemaHierarchy(): HierarchicalSchemaData {
  const { nodes, edges } = generateSchemaGraph();

  // Count nodes by realm
  const nodesByRealm: Record<Realm, number> = {
    project: 0,
    global: 0,
    shared: 0,
  };

  for (const node of nodes) {
    nodesByRealm[node.realm]++;
  }

  return {
    realms: REALM_HIERARCHY,
    nodes,
    edges,
    stats: {
      totalNodes: nodes.length,
      totalEdges: edges.length,
      nodesByRealm,
    },
  };
}
