// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v10.7.0 — 7-node locale knowledge architecture (GLOBAL / TENANT)

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, type NodeType, type Realm } from '../types/nodes.js';
import { RelationRegistry } from '../schemas/relations.schema.js';
import type { SchemaNode, SchemaArc, SchemaGraphResult, HierarchicalSchemaData } from './types.js';
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
  // ═══════════════════════════════════════════════════════════════════════════
  // GLOBAL REALM (25 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (7) - v10.7: added Culture, Market
  Locale: 'Locale',
  Formatting: 'Formatting',
  Slugification: 'Slugification',
  Adaptation: 'Adaptation',
  Style: 'Style',
  Culture: 'Culture',
  Market: 'Market',

  // locale-knowledge (12) — Sets + Atoms
  TermSet: 'Term Set',
  ExpressionSet: 'Expression Set',
  PatternSet: 'Pattern Set',
  CultureSet: 'Culture Set',
  TabooSet: 'Taboo Set',
  AudienceSet: 'Audience Set',
  Term: 'Term',
  Expression: 'Expression',
  Pattern: 'Pattern',
  CultureRef: 'Culture Ref',
  Taboo: 'Taboo',
  AudienceTrait: 'Audience Trait',

  // seo (6)
  SEOKeyword: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOMiningRun: 'SEO Mining',
  SEOComparison: 'SEO Comparison',
  SEOPreposition: 'SEO Preposition',
  SEOQuestion: 'SEO Question',

  // ═══════════════════════════════════════════════════════════════════════════
  // TENANT REALM (23 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1)
  Organization: 'Organization',

  // foundation (3)
  Project: 'Project',
  BrandIdentity: 'Brand Identity',
  ProjectL10n: 'Project L10n',

  // structure (3)
  Page: 'Page',
  Block: 'Block',
  ContentSlot: 'Content Slot',

  // semantic (4)
  Entity: 'Entity',
  EntityL10n: 'Entity L10n',
  AudiencePersona: 'Audience Persona',
  ChannelSurface: 'Channel Surface',

  // instruction (7)
  PageType: 'Page Type',
  BlockType: 'Block Type',
  PagePrompt: 'Page Prompt',
  BlockPrompt: 'Block Prompt',
  BlockRules: 'Block Rules',
  BlockInstruction: 'Block Instruction',
  PromptArtifact: 'Prompt Artifact',

  // output (5)
  PageL10n: 'Page L10n',
  BlockL10n: 'Block L10n',
  GenerationJob: 'Generation Job',
  OutputArtifact: 'Output Artifact',
  EvaluationSignal: 'Evaluation Signal',
};

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  global: 'Shared across all tenants (Locale knowledge, SEO)',
  tenant: 'Tenant-specific content, structure, Entity nodes',
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
 * Generate flat schema graph with all 43 node types and arcs.
 * This is the canonical representation of the NovaNet ontology.
 *
 * @returns SchemaGraphResult with nodes and arcs
 *
 * @example
 * ```typescript
 * const { nodes, arcs } = generateSchemaGraph();
 * console.log(`${nodes.length} nodes, ${arcs.length} arcs`);
 * // Output: "43 nodes, ~XX arcs"
 * ```
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const arcs: SchemaArc[] = [];

  // ==========================================================================
  // GENERATE NODES - All 43 node types
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
  // GENERATE ARCS - From RelationRegistry (single source of truth)
  // ==========================================================================

  const validNodeTypes = new Set<string>(NODE_TYPES);
  let arcId = 0;

  for (const relation of Object.values(RelationRegistry)) {
    const sourceTypes = Array.isArray(relation.from) ? relation.from : [relation.from];
    const targetTypes = Array.isArray(relation.to) ? relation.to : [relation.to];

    // Create Cartesian product of arcs for multi-type relations
    for (const source of sourceTypes) {
      for (const target of targetTypes) {
        // Skip arcs with invalid node types
        if (!validNodeTypes.has(source) || !validNodeTypes.has(target)) {
          continue;
        }

        arcs.push({
          id: `schema-arc-${arcId++}`,
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

  return { nodes, arcs };
}

/**
 * Generate hierarchical schema data grouped by realm and layer.
 * Used by visualizers that need grouped layout (like Studio).
 *
 * @returns HierarchicalSchemaData with realms, nodes, arcs, and stats
 *
 * @example
 * ```typescript
 * const hierarchy = getSchemaHierarchy();
 * console.log(hierarchy.stats);
 * // Output: { totalNodes: 42, totalArcs: ~XX, nodesByRealm: { global: 20, tenant: 22 } }
 * ```
 */
export function getSchemaHierarchy(): HierarchicalSchemaData {
  const { nodes, arcs } = generateSchemaGraph();

  // Count nodes by realm
  const nodesByRealm: Record<Realm, number> = {
    global: 0,
    tenant: 0,
  };

  for (const node of nodes) {
    nodesByRealm[node.realm]++;
  }

  return {
    realms: REALM_HIERARCHY,
    nodes,
    arcs,
    stats: {
      totalNodes: nodes.length,
      totalArcs: arcs.length,
      nodesByRealm,
    },
  };
}
