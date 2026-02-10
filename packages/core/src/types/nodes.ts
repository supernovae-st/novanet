/**
 * @fileoverview NovaNet Node Type Taxonomy
 * @module @novanet/core/types/nodes
 * @version 11.6.0
 *
 * Defines the complete taxonomy for all 60 NovaNet node types across 2 realms and 10 layers.
 * This is the **single source of truth** for node classification in the knowledge graph.
 *
 * ## Architecture Overview
 *
 * NovaNet uses a **faceted classification system** with 3 axes:
 *
 * | Axis   | Question | Values |
 * |--------|----------|--------|
 * | Realm  | WHERE?   | `shared` (universal, READ-ONLY) or `org` (organization-specific) |
 * | Layer  | WHAT?    | 10 functional layers (4 shared + 6 org) |
 * | Trait  | HOW?     | Localization behavior (invariant, localized, knowledge, generated, aggregated) |
 *
 * ## Realm Distribution
 *
 * - **SHARED** (39 nodes): Universal locale knowledge, geography, SEO/GEO intelligence
 * - **ORG** (21 nodes): Organization-specific content, structure, generation pipeline
 *
 * @see {@link https://github.com/supernovae-st/novanet-hq/blob/main/.claude/rules/novanet-terminology.md | Terminology Reference}
 * @see {@link https://github.com/supernovae-st/novanet-hq/blob/main/.claude/rules/novanet-decisions.md | Architecture Decisions}
 */

// =============================================================================
// NODE TYPES (60 nodes across 2 realms, 10 layers)
// =============================================================================

/**
 * Complete list of all 60 NovaNet node types.
 *
 * Organized by realm and layer:
 * - **SHARED** (39 nodes): config (3) + locale (6) + geography (6) + knowledge (24)
 * - **ORG** (21 nodes): config (1) + foundation (3) + structure (3) + semantic (4) + instruction (7) + output (3)
 *
 * @example
 * ```typescript
 * import { NODE_TYPES, KIND_META } from '@novanet/core/types';
 *
 * // Iterate all node types
 * NODE_TYPES.forEach(type => {
 *   const { realm, layer, trait } = KIND_META[type];
 *   console.log(`${type}: ${realm}/${layer} (${trait})`);
 * });
 *
 * // Filter by realm
 * const sharedTypes = NODE_TYPES.filter(t => KIND_META[t].realm === 'shared');
 * // → 39 shared realm node types
 * ```
 *
 * @since 7.1.0
 * @version 11.6.0 - SEO/GEO moved to shared/knowledge
 */
export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (39 nodes) — 4 layers: config, locale, geography, knowledge
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — v11.5: classification nodes + Locale definition + SEO format
  'EntityCategory', 'Locale', 'SEOKeywordFormat',
  // locale (6) — Locale SETTINGS (not the Locale definition itself)
  'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  // geography (6) — Geographic classifications
  'Continent', 'GeoRegion', 'GeoSubRegion', 'IncomeGroup', 'LendingCategory', 'EconomicRegion',
  // knowledge (24) — Sets + Atoms + Linguistic/Cultural taxonomy + SEO/GEO
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  'LanguageFamily', 'LanguageBranch', 'CulturalRealm', 'CulturalSubRealm', 'PopulationCluster', 'PopulationSubCluster',
  // knowledge — SEO/GEO (6) — v11.5: moved from org to shared/knowledge
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOKeywordSet',
  'GEOQuery', 'GEOQuerySet', 'GEOAnswer',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — 6 layers: config, foundation, structure, semantic, instruction, output
  // v11.4: SEO/GEO nodes moved to shared/knowledge
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — v11.3: Organization + Tenant merged into OrgConfig
  'OrgConfig',
  // foundation (3)
  'Project', 'BrandIdentity', 'ProjectContent',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface',
  // instruction (7)
  'PageType', 'BlockType', 'PagePrompt', 'BlockPrompt', 'BlockRules', 'BlockInstruction', 'PromptArtifact',
  // output (3)
  'PageGenerated', 'BlockGenerated', 'OutputArtifact',
] as const;

/**
 * Union type of all valid NovaNet node type names.
 *
 * Use this for type-safe node type references in your code.
 *
 * @example
 * ```typescript
 * function getNodeColor(type: NodeType): string {
 *   const { layer } = KIND_META[type];
 *   return LAYER_COLORS[layer];
 * }
 * ```
 */
export type NodeType = typeof NODE_TYPES[number];

// =============================================================================
// v11.5 TAXONOMY TYPES (2 realms, 10 layers: 4 shared + 6 org)
// =============================================================================

/**
 * Node realm classification: WHERE the node lives in the graph.
 *
 * - **`shared`**: Universal locale knowledge (READ-ONLY). Examples: Locale, Term, SEOKeyword
 * - **`org`**: Organization-specific content. Examples: Project, Entity, Page
 *
 * @example
 * ```typescript
 * // Check if a node is in the shared realm
 * const isShared = KIND_META[nodeType].realm === 'shared';
 *
 * // Filter Cypher query by realm
 * const query = `MATCH (n:${nodeType}) WHERE n.realm = 'shared' RETURN n`;
 * ```
 *
 * @see ADR-012 — 2-Realm Architecture
 * @see ADR-018 — Realm renames (global → shared, tenant → org)
 */
export type Realm = 'shared' | 'org';

/**
 * Node layer classification: WHAT functional category the node belongs to.
 *
 * ## Shared Realm Layers (4)
 * - **`config`**: Classification definitions (EntityCategory, Locale, SEOKeywordFormat)
 * - **`locale`**: Locale settings (Formatting, Style, Culture, Market, etc.)
 * - **`geography`**: Geographic classifications (Continent, GeoRegion, IncomeGroup, etc.)
 * - **`knowledge`**: Knowledge atoms (Term, Expression, SEOKeyword, GEOQuery, etc.)
 *
 * ## Org Realm Layers (6)
 * - **`foundation`**: Project identity (Project, BrandIdentity, ProjectContent)
 * - **`structure`**: Page/Block hierarchy (Page, Block, ContentSlot)
 * - **`semantic`**: Business entities (Entity, EntityContent, AudiencePersona)
 * - **`instruction`**: Generation prompts (PageType, BlockType, prompts, rules)
 * - **`output`**: Generated artifacts (PageGenerated, BlockGenerated)
 *
 * @example
 * ```typescript
 * // Get all semantic layer nodes
 * const semanticNodes = NODE_TYPES.filter(t => KIND_META[t].layer === 'semantic');
 * // → ['Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface']
 * ```
 *
 * @see ADR-020 — Layer reorganization
 */
export type Layer =
  | 'config' | 'locale' | 'geography' | 'knowledge'  // shared realm layers (4)
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';  // org realm layers (6) — note: 'config' shared with shared realm

/**
 * Node trait classification: HOW the node behaves with respect to localization.
 *
 * | Trait | Description | Visual Encoding | Example |
 * |-------|-------------|-----------------|---------|
 * | `invariant` | Same across all locales | Solid border | Entity, Page, Block |
 * | `localized` | Locale-specific content | Dashed border | EntityContent, ProjectContent |
 * | `knowledge` | Locale-native atoms | Dotted border | Term, Expression, Pattern |
 * | `generated` | LLM-generated output | Double border | PageGenerated, BlockGenerated |
 * | `aggregated` | Computed metrics | Dotted thin | SEOKeywordMetrics, GEOAnswer |
 *
 * @example
 * ```typescript
 * // Check if node needs locale-specific content
 * const needsLocalization = KIND_META[nodeType].trait === 'localized';
 *
 * // Get all generated output nodes
 * const outputs = NODE_TYPES.filter(t => KIND_META[t].trait === 'generated');
 * // → ['PageGenerated', 'BlockGenerated', 'OutputArtifact', 'PromptArtifact']
 * ```
 *
 * @see ADR-005 — Trait-based visual encoding
 * @see ADR-018 — Trait split (derived → generated + aggregated)
 */
export type Trait = 'invariant' | 'localized' | 'knowledge' | 'generated' | 'aggregated';

// =============================================================================
// KIND_META — unified classification for all 60 node types
// =============================================================================

/**
 * Classification metadata for a node type.
 *
 * Every node type in NovaNet is classified along 3 axes:
 * - `realm`: WHERE — shared (universal) or org (organization-specific)
 * - `layer`: WHAT — functional category (10 layers total)
 * - `trait`: HOW — localization behavior (5 traits)
 *
 * @example
 * ```typescript
 * const meta: KindMeta = KIND_META['Entity'];
 * // → { realm: 'org', layer: 'semantic', trait: 'invariant' }
 * ```
 */
export interface KindMeta {
  /** Where the node lives: 'shared' (universal) or 'org' (organization-specific) */
  realm: Realm;
  /** Functional category: one of 10 layers (4 shared + 6 org) */
  layer: Layer;
  /** Localization behavior: invariant, localized, knowledge, generated, or aggregated */
  trait: Trait;
}

/**
 * Complete classification registry for all 60 NovaNet node types.
 *
 * This is the **single source of truth** for node classification. Use this to:
 * - Determine which realm/layer a node belongs to
 * - Filter nodes by classification
 * - Apply visual encoding based on classification
 *
 * @example
 * ```typescript
 * import { NODE_TYPES, KIND_META } from '@novanet/core/types';
 *
 * // Get classification for a specific type
 * const { realm, layer, trait } = KIND_META['Entity'];
 * // → { realm: 'org', layer: 'semantic', trait: 'invariant' }
 *
 * // Count nodes by realm
 * const sharedCount = NODE_TYPES.filter(t => KIND_META[t].realm === 'shared').length;
 * // → 39
 *
 * // Get all output layer nodes
 * const outputs = NODE_TYPES.filter(t => KIND_META[t].layer === 'output');
 * // → ['PageGenerated', 'BlockGenerated', 'OutputArtifact']
 * ```
 *
 * @version 11.6.0
 */
export const KIND_META: Record<NodeType, KindMeta> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM — config (3) — v11.5: classification nodes + Locale definition
  // ═══════════════════════════════════════════════════════════════════════════
  EntityCategory:   { realm: 'shared', layer: 'config', trait: 'invariant' },
  Locale:           { realm: 'shared', layer: 'config', trait: 'invariant' },
  SEOKeywordFormat: { realm: 'shared', layer: 'config', trait: 'invariant' },

  // SHARED REALM — locale (6) — locale SETTINGS
  Formatting:     { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Slugification:  { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Adaptation:     { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Style:          { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Culture:        { realm: 'shared', layer: 'locale', trait: 'knowledge' },
  Market:         { realm: 'shared', layer: 'locale', trait: 'knowledge' },

  // SHARED REALM — geography (6)
  Continent:      { realm: 'shared', layer: 'geography', trait: 'invariant' },
  GeoRegion:      { realm: 'shared', layer: 'geography', trait: 'invariant' },
  GeoSubRegion:   { realm: 'shared', layer: 'geography', trait: 'invariant' },
  IncomeGroup:    { realm: 'shared', layer: 'geography', trait: 'invariant' },
  LendingCategory:{ realm: 'shared', layer: 'geography', trait: 'invariant' },
  EconomicRegion: { realm: 'shared', layer: 'geography', trait: 'invariant' },

  // SHARED REALM — knowledge (24) — containers, atoms, SEO/GEO
  TermSet:             { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  ExpressionSet:       { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  PatternSet:          { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  CultureSet:          { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  TabooSet:            { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  AudienceSet:         { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  Term:                { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Expression:          { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Pattern:             { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CultureRef:          { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  Taboo:               { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  AudienceTrait:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  LanguageFamily:      { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  LanguageBranch:      { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CulturalRealm:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  CulturalSubRealm:    { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  PopulationCluster:   { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  PopulationSubCluster:{ realm: 'shared', layer: 'knowledge', trait: 'knowledge' },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM — config (1) — v11.3: Organization + Tenant merged
  // ═══════════════════════════════════════════════════════════════════════════
  OrgConfig: { realm: 'org', layer: 'config', trait: 'invariant' },

  // ORG REALM — foundation (3)
  Project:        { realm: 'org', layer: 'foundation',  trait: 'invariant' },
  BrandIdentity:  { realm: 'org', layer: 'foundation',  trait: 'invariant' },
  ProjectContent: { realm: 'org', layer: 'foundation',  trait: 'localized' },

  // ORG REALM — structure (3)
  Page:         { realm: 'org', layer: 'structure',   trait: 'invariant' },
  Block:        { realm: 'org', layer: 'structure',   trait: 'invariant' },
  ContentSlot:  { realm: 'org', layer: 'structure',   trait: 'invariant' },

  // ORG REALM — semantic (4)
  Entity:          { realm: 'org', layer: 'semantic', trait: 'invariant' },
  EntityContent:   { realm: 'org', layer: 'semantic', trait: 'localized' },
  AudiencePersona: { realm: 'org', layer: 'semantic', trait: 'invariant' },
  ChannelSurface:  { realm: 'org', layer: 'semantic', trait: 'invariant' },

  // ORG REALM — instruction (7)
  PageType:        { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockType:       { realm: 'org', layer: 'instruction', trait: 'invariant' },
  PagePrompt:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockPrompt:     { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockRules:      { realm: 'org', layer: 'instruction', trait: 'invariant' },
  BlockInstruction:{ realm: 'org', layer: 'instruction', trait: 'invariant' },
  PromptArtifact:  { realm: 'org', layer: 'instruction', trait: 'generated' },

  // SHARED REALM — knowledge (SEO/GEO) — v11.5: moved from org to shared
  SEOKeyword:       { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  SEOKeywordMetrics:{ realm: 'shared', layer: 'knowledge', trait: 'aggregated' },
  SEOKeywordSet:    { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  GEOQuery:         { realm: 'shared', layer: 'knowledge', trait: 'knowledge' },
  GEOQuerySet:      { realm: 'shared', layer: 'knowledge', trait: 'invariant' },
  GEOAnswer:        { realm: 'shared', layer: 'knowledge', trait: 'aggregated' },

  // ORG REALM — output (3)
  PageGenerated:  { realm: 'org', layer: 'output', trait: 'generated' },
  BlockGenerated: { realm: 'org', layer: 'output', trait: 'generated' },
  OutputArtifact: { realm: 'org', layer: 'output', trait: 'generated' },
};

// =============================================================================
// DERIVED MAPS — computed from KIND_META
// =============================================================================

/**
 * Creates a derived lookup map from KIND_META for a specific field.
 *
 * @internal
 * @param field - The KindMeta field to extract ('realm', 'layer', or 'trait')
 * @returns Record mapping NodeType to the extracted field value
 */
function deriveMap<K extends keyof KindMeta>(field: K): Record<NodeType, KindMeta[K]> {
  return Object.fromEntries(
    Object.entries(KIND_META).map(([k, v]) => [k, v[field]])
  ) as Record<NodeType, KindMeta[K]>;
}

/**
 * Lookup map from NodeType to its realm classification.
 *
 * @example
 * ```typescript
 * const realm = NODE_REALMS['Entity'];  // → 'org'
 * const realm = NODE_REALMS['Locale'];  // → 'shared'
 * ```
 */
export const NODE_REALMS: Record<NodeType, Realm> = deriveMap('realm');

/**
 * Lookup map from NodeType to its trait classification.
 *
 * @example
 * ```typescript
 * const trait = NODE_TRAITS['Entity'];        // → 'invariant'
 * const trait = NODE_TRAITS['EntityContent']; // → 'localized'
 * const trait = NODE_TRAITS['PageGenerated']; // → 'generated'
 * ```
 */
export const NODE_TRAITS: Record<NodeType, Trait> = deriveMap('trait');
