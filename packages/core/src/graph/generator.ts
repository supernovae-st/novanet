// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v1.0.0

import { NODE_TYPES, NODE_SCOPES, NODE_BEHAVIORS, type NodeType, type Scope } from '../types/nodes.js';
import { RelationRegistry } from '../schemas/relations.schema.js';
import type { SchemaNode, SchemaEdge, SchemaGraphResult, HierarchicalSchemaData } from './types.js';
import { getSubcategory } from './subcategories.js';
import { SCOPE_HIERARCHY } from './hierarchy.js';

// =============================================================================
// NODE DISPLAY LABELS
// =============================================================================

/**
 * Human-readable labels for node types.
 * Can be extended with icons/colors in UI layer.
 */
const NODE_LABELS: Record<NodeType, string> = {
  // Project scope - foundation
  Project: 'Project',
  BrandIdentity: 'Brand Identity',
  ProjectL10n: 'Project L10n',
  // Project scope - structure
  Page: 'Page',
  Block: 'Block',
  BlockType: 'Block Type',
  PageType: 'Page Type',
  // Project scope - semantic
  Concept: 'Concept',
  ConceptL10n: 'Concept L10n',
  // Project scope - instruction
  PagePrompt: 'Page Prompt',
  BlockPrompt: 'Block Prompt',
  BlockRules: 'Block Rules',
  // Project scope - output
  PageL10n: 'Page L10n',
  BlockL10n: 'Block L10n',
  // Global scope - config
  Locale: 'Locale',
  // Global scope - knowledge
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
  // Shared scope - seo
  SEOKeywordL10n: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOMiningRun: 'SEO Mining',
  // Shared scope - geo
  GEOSeedL10n: 'GEO Seed',
  GEOSeedMetrics: 'GEO Metrics',
  GEOMiningRun: 'GEO Mining',
};

// =============================================================================
// SCOPE DESCRIPTIONS
// =============================================================================

/**
 * Scope descriptions for nodes
 */
const SCOPE_DESCRIPTIONS: Record<Scope, string> = {
  Global: 'Shared across all projects (Locale knowledge)',
  Shared: 'Shared across projects (SEO/GEO data)',
  Project: 'Project-specific content and structure',
};

// =============================================================================
// BEHAVIOR DESCRIPTIONS
// =============================================================================

/**
 * Behavior descriptions for nodes
 */
const BEHAVIOR_DESCRIPTIONS: Record<string, string> = {
  invariant: 'Language-independent, same across all locales',
  localized: 'Human-curated localized content',
  localeKnowledge: 'Locale-specific cultural/linguistic knowledge',
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
    const scope = NODE_SCOPES[nodeType];
    const behavior = NODE_BEHAVIORS[nodeType];
    const subcategory = getSubcategory(nodeType);

    nodes.push({
      id: `schema-${nodeType}`,
      nodeType,
      scope,
      subcategory,
      label: NODE_LABELS[nodeType],
      description: `${SCOPE_DESCRIPTIONS[scope]}. ${BEHAVIOR_DESCRIPTIONS[behavior]}.`,
      behavior,
    });
  }

  // ==========================================================================
  // GENERATE EDGES - From RelationRegistry (single source of truth)
  // ==========================================================================

  // P0 FIX: Validate node types exist before creating edges
  const validNodeTypes = new Set<string>(NODE_TYPES);
  let edgeId = 0;

  for (const relation of Object.values(RelationRegistry)) {
    const sourceTypes = Array.isArray(relation.from) ? relation.from : [relation.from];
    const targetTypes = Array.isArray(relation.to) ? relation.to : [relation.to];

    // Create Cartesian product of edges for multi-type relations
    for (const source of sourceTypes) {
      for (const target of targetTypes) {
        // P0 FIX: Skip edges with invalid node types
        if (!validNodeTypes.has(source) || !validNodeTypes.has(target)) {
          // Silently skip - these are likely old/deprecated relations
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
 * Generate hierarchical schema data grouped by scope and subcategory.
 * Used by visualizers that need grouped layout (like Studio).
 *
 * @returns HierarchicalSchemaData with scopes, nodes, edges, and stats
 *
 * @example
 * ```typescript
 * const hierarchy = getSchemaHierarchy();
 * console.log(hierarchy.stats);
 * // Output: { totalNodes: 35, totalEdges: ~89, nodesByScope: { Project: 14, Global: 15, Shared: 6 } }
 * ```
 */
export function getSchemaHierarchy(): HierarchicalSchemaData {
  const { nodes, edges } = generateSchemaGraph();

  // Count nodes by scope
  const nodesByScope: Record<Scope, number> = {
    Project: 0,
    Global: 0,
    Shared: 0,
  };

  for (const node of nodes) {
    nodesByScope[node.scope]++;
  }

  return {
    scopes: SCOPE_HIERARCHY,
    nodes,
    edges,
    stats: {
      totalNodes: nodes.length,
      totalEdges: edges.length,
      nodesByScope,
    },
  };
}
