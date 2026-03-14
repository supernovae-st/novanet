// packages/core/src/graph/generator.ts
// Schema graph generator - Creates flat and hierarchical schema representations
// v0.20.0 — 59 nodes (36 shared + 23 org), traits deprecated

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
  // SHARED REALM (36 nodes) — 4 layers: config, locale, geography, knowledge
  // v0.20.0: 59 total nodes (36 shared + 23 org)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — v11.5: Locale + EntityCategory + SEOKeywordFormat
  EntityCategory: 'Entity Category',
  Locale: 'Locale',
  SEOKeywordFormat: 'SEO Keyword Format',

  // locale (5) — v0.17.3: Market removed (YAGNI)
  Formatting: 'Formatting',
  Slugification: 'Slugification',
  Adaptation: 'Adaptation',
  Style: 'Style',
  Culture: 'Culture',

  // geography (7) — v0.12.4: Country added
  Continent: 'Continent',
  Country: 'Country',
  GeoRegion: 'Geo Region',
  GeoSubRegion: 'Geo Sub-Region',
  IncomeGroup: 'Income Group',
  LendingCategory: 'Lending Category',
  EconomicRegion: 'Economic Region',

  // knowledge (21) — includes SEO/GEO (v0.17.3: Term, TermSet, SEOKeywordMetrics removed)
  ExpressionSet: 'Expression Set',
  PatternSet: 'Pattern Set',
  CultureSet: 'Culture Set',
  TabooSet: 'Taboo Set',
  AudienceSet: 'Audience Set',
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

  // knowledge — SEO/GEO (5) — v11.5: moved from org (v0.17.3: SEOKeywordMetrics removed)
  SEOKeyword: 'SEO Keyword',
  SEOKeywordSet: 'SEO Keyword Set',
  GEOQuery: 'GEO Query',
  GEOQuerySet: 'GEO Query Set',
  GEOAnswer: 'GEO Answer',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (23 nodes) — 6 layers — v0.12.4: Brand Architecture
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — v11.3: Organization + Tenant merged
  OrgConfig: 'Org Config',

  // foundation (8) — v0.17.3: +ProjectSEOScope, +ProjectGEOScope
  Project: 'Project',
  Brand: 'Brand',
  BrandDesign: 'Brand Design',
  BrandPrinciples: 'Brand Principles',
  PromptStyle: 'Prompt Style',
  ProjectNative: 'Project Native',
  ProjectSEOScope: 'Project SEO Scope',
  ProjectGEOScope: 'Project GEO Scope',

  // structure (3)
  Page: 'Page',
  Block: 'Block',
  ContentSlot: 'Content Slot',

  // semantic (2) — v0.17.3: AudiencePersona, ChannelSurface removed (YAGNI)
  Entity: 'Entity',
  EntityNative: 'Entity Native',

  // instruction (3) — v0.19.1: BlockRules removed (merged into BlockType.rules)
  BlockType: 'Block Type',
  BlockInstruction: 'Block Instruction',
  PromptArtifact: 'Prompt Artifact',

  // output (6) — v0.13.0: *Native Pattern (ADR-029), v0.19.0: +Enrichments
  PageNative: 'Page Native',
  BlockNative: 'Block Native',
  OutputArtifact: 'Output Artifact',
  CultureRefEnrichment: 'CultureRef Enrichment',
  ExpressionEnrichment: 'Expression Enrichment',
  TabooEnrichment: 'Taboo Enrichment',
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
 * Generate flat schema graph with all 59 node types and 159 arcs.
 * This is the canonical representation of the NovaNet ontology.
 *
 * @returns SchemaGraphResult with nodes and arcs
 *
 * @example
 * ```typescript
 * const { nodes, arcs } = generateSchemaGraph();
 * console.log(`${nodes.length} nodes, ${arcs.length} arcs`);
 * // Output: "59 nodes, 159 arcs"
 * ```
 */
export function generateSchemaGraph(): SchemaGraphResult {
  const nodes: SchemaNode[] = [];
  const arcs: SchemaArc[] = [];

  // ==========================================================================
  // GENERATE NODES - All 59 node types
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
 * // Output: { totalNodes: 59, totalArcs: 159, nodesByRealm: { shared: 36, org: 23 } }
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
