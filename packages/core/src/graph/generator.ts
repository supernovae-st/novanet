// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v11.5.0 — SEO/GEO moved to shared/knowledge, Locale to shared/config

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
  // SHARED REALM (40 nodes) — 4 layers: config, locale, geography, knowledge
  // v0.12.4: 61 total nodes (40 shared + 21 org)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — v11.5: Locale + EntityCategory + SEOKeywordFormat
  EntityCategory: 'Entity Category',
  Locale: 'Locale',
  SEOKeywordFormat: 'SEO Keyword Format',

  // locale (6)
  Formatting: 'Formatting',
  Slugification: 'Slugification',
  Adaptation: 'Adaptation',
  Style: 'Style',
  Culture: 'Culture',
  Market: 'Market',

  // geography (7) — v0.12.4: Country added
  Continent: 'Continent',
  Country: 'Country',
  GeoRegion: 'Geo Region',
  GeoSubRegion: 'Geo Sub-Region',
  IncomeGroup: 'Income Group',
  LendingCategory: 'Lending Category',
  EconomicRegion: 'Economic Region',

  // knowledge (24) — includes SEO/GEO
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

  // knowledge — SEO/GEO (6) — v11.5: moved from org
  SEOKeyword: 'SEO Keyword',
  SEOKeywordMetrics: 'SEO Metrics',
  SEOKeywordSet: 'SEO Keyword Set',
  GEOQuery: 'GEO Query',
  GEOQuerySet: 'GEO Query Set',
  GEOAnswer: 'GEO Answer',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — 6 layers — v0.12.4: Brand Architecture
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — v11.3: Organization + Tenant merged
  OrgConfig: 'Org Config',

  // foundation (6) — v0.12.4: Brand Architecture
  Project: 'Project',
  Brand: 'Brand',
  BrandDesign: 'Brand Design',
  BrandPrinciples: 'Brand Principles',
  PromptStyle: 'Prompt Style',
  ProjectNative: 'Project Content',

  // structure (3)
  Page: 'Page',
  Block: 'Block',
  ContentSlot: 'Content Slot',

  // semantic (4)
  Entity: 'Entity',
  EntityNative: 'Entity Content',
  AudiencePersona: 'Audience Persona',
  ChannelSurface: 'Channel Surface',

  // instruction (4) — v0.12.4: PageStructure, PageInstruction deleted
  BlockType: 'Block Type',
  BlockInstruction: 'Block Instruction',
  BlockRules: 'Block Rules',
  PromptArtifact: 'Prompt Artifact',

  // output (3)
  PageNative: 'Page Generated',
  BlockNative: 'Block Generated',
  OutputArtifact: 'Output Artifact',
};

// =============================================================================
// REALM DESCRIPTIONS
// =============================================================================

const REALM_DESCRIPTIONS: Record<Realm, string> = {
  shared: 'Shared across all organizations (Locale, Geography, Knowledge)',
  org: 'Organization-specific content, structure, Entity nodes',
};

// =============================================================================
// TRAIT DESCRIPTIONS
// =============================================================================

const TRAIT_DESCRIPTIONS: Record<string, string> = {
  defined: 'Human-created ONCE, structure/template',
  authored: 'Human-written PER locale, editorial content',
  imported: 'External data brought in, corpora/APIs',
  generated: 'Produced by NovaNet LLM',
  retrieved: 'Fetched from EXTERNAL APIs, snapshots',
};

// =============================================================================
// GENERATORS
// =============================================================================

/**
 * Generate flat schema graph with all 61 node types and 146 arcs.
 * This is the canonical representation of the NovaNet ontology.
 *
 * @returns SchemaGraphResult with nodes and arcs
 *
 * @example
 * ```typescript
 * const { nodes, arcs } = generateSchemaGraph();
 * console.log(`${nodes.length} nodes, ${arcs.length} arcs`);
 * // Output: "61 nodes, 146 arcs"
 * ```
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const arcs: SchemaArc[] = [];

  // ==========================================================================
  // GENERATE NODES - All 61 node types
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
 * // Output: { totalNodes: 61, totalArcs: 146, nodesByRealm: { shared: 40, org: 21 } }
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
