// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v11.2.0 — Realm renames (shared/org), job nodes removed, trait split

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
  // SHARED REALM (32 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (14)
  Locale: 'Locale',
  EntityCategory: 'Entity Category',
  Formatting: 'Formatting',
  Slugification: 'Slugification',
  Adaptation: 'Adaptation',
  Style: 'Style',
  Culture: 'Culture',
  Market: 'Market',
  Continent: 'Continent',
  GeoRegion: 'Geo Region',
  GeoSubRegion: 'Geo Sub-Region',
  IncomeGroup: 'Income Group',
  LendingCategory: 'Lending Category',
  EconomicRegion: 'Economic Region',

  // locale-knowledge (18) — Sets + Atoms + Linguistic/Cultural taxonomy
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
  LanguageFamily: 'Language Family',
  LanguageBranch: 'Language Branch',
  CulturalRealm: 'Cultural Realm',
  CulturalSubRealm: 'Cultural Sub-Realm',
  PopulationCluster: 'Population Cluster',
  PopulationSubCluster: 'Population Sub-Cluster',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (30 nodes)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (2)
  Organization: 'Organization',
  Tenant: 'Tenant',

  // foundation (3)
  Project: 'Project',
  BrandIdentity: 'Brand Identity',
  ProjectContent: 'Project Content',

  // structure (3)
  Page: 'Page',
  Block: 'Block',
  ContentSlot: 'Content Slot',

  // semantic (4)
  Entity: 'Entity',
  EntityContent: 'Entity Content',
  AudiencePersona: 'Audience Persona',
  ChannelSurface: 'Channel Surface',

  // seo (8) — SEO + GEO (Generative Engine Optimization)
  SEOKeyword: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOComparison: 'SEO Comparison',
  SEOPreposition: 'SEO Preposition',
  SEOQuestion: 'SEO Question',
  GEOQuery: 'GEO Query',
  GEOAnswer: 'GEO Answer',
  GEOMetrics: 'GEO Metrics',

  // instruction (7)
  PageType: 'Page Type',
  BlockType: 'Block Type',
  PagePrompt: 'Page Prompt',
  BlockPrompt: 'Block Prompt',
  BlockRules: 'Block Rules',
  BlockInstruction: 'Block Instruction',
  PromptArtifact: 'Prompt Artifact',

  // output (3)
  PageGenerated: 'Page Generated',
  BlockGenerated: 'Block Generated',
  OutputArtifact: 'Output Artifact',
};

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  shared: 'Shared across all tenants (Locale knowledge)',
  org: 'Organization-specific content, structure, Entity nodes',
};

// =============================================================================
// TRAIT DESCRIPTIONS
// =============================================================================

const TRAIT_DESCRIPTIONS: Record<string, string> = {
  invariant: 'Language-independent, same across all locales',
  localized: 'Human-curated localized content',
  knowledge: 'Locale-specific cultural/linguistic knowledge',
  generated: 'LLM-generated content output',
  aggregated: 'Computed metrics and aggregated data',
};

// =============================================================================
// GENERATORS
// =============================================================================

/**
 * Generate flat schema graph with all 62 node types and arcs.
 * This is the canonical representation of the NovaNet ontology.
 *
 * @returns SchemaGraphResult with nodes and arcs
 *
 * @example
 * ```typescript
 * const { nodes, arcs } = generateSchemaGraph();
 * console.log(`${nodes.length} nodes, ${arcs.length} arcs`);
 * // Output: "62 nodes, ~XX arcs"
 * ```
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const arcs: SchemaArc[] = [];

  // ==========================================================================
  // GENERATE NODES - All 62 node types
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
 * // Output: { totalNodes: 62, totalArcs: ~XX, nodesByRealm: { shared: 32, org: 30 } }
 * ```
 */
export function getSchemaHierarchy(): HierarchicalSchemaData {
  const { nodes, arcs } = generateSchemaGraph();

  // Count nodes by realm
  const nodesByRealm: Record<Realm, number> = {
    shared: 0,
    org: 0,
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
