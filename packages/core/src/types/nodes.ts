/**
 * @fileoverview NovaNet Node Type Taxonomy
 * @module @novanet/core/types/nodes
 * @version 0.12.4
 *
 * Defines the complete taxonomy for all 61 NovaNet node types across 2 realms and 10 layers.
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
 * | Trait  | HOW?     | Data origin behavior (defined, authored, imported, generated, retrieved) |
 *
 * ## Realm Distribution
 *
 * - **SHARED** (40 nodes): Universal locale knowledge, geography, SEO/GEO intelligence
 * - **ORG** (21 nodes): Organization-specific content, structure, generation pipeline
 *
 * @see {@link https://github.com/supernovae-st/novanet-hq/blob/main/.claude/rules/novanet-terminology.md | Terminology Reference}
 * @see {@link https://github.com/supernovae-st/novanet-hq/blob/main/.claude/rules/novanet-decisions.md | Architecture Decisions}
 */

// =============================================================================
// NODE TYPES (61 nodes across 2 realms, 10 layers)
// =============================================================================

/**
 * Complete list of all 61 NovaNet node types.
 *
 * Organized by realm and layer:
 * - **SHARED** (40 nodes): config (3) + locale (6) + geography (7) + knowledge (24)
 * - **ORG** (21 nodes): config (1) + foundation (6) + structure (3) + semantic (4) + instruction (4) + output (3)
 *
 * @example
 * ```typescript
 * import { NODE_TYPES, CLASS_TAXONOMY } from '@novanet/core/types';
 *
 * // Iterate all node types
 * NODE_TYPES.forEach(type => {
 *   const { realm, layer, trait } = CLASS_TAXONOMY[type];
 *   console.log(`${type}: ${realm}/${layer} (${trait})`);
 * });
 *
 * // Filter by realm
 * const sharedTypes = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].realm === 'shared');
 * // → 40 shared realm node types
 * ```
 *
 * @since 7.1.0
 * @version 0.12.4 - Brand Architecture, Country added
 */
export const NODE_TYPES = [
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM (40 nodes) — 4 layers: config, locale, geography, knowledge
  // ═══════════════════════════════════════════════════════════════════════════
  // config (3) — v11.5: classification nodes + Locale definition + SEO format
  'EntityCategory', 'Locale', 'SEOKeywordFormat',
  // locale (6) — Locale SETTINGS (not the Locale definition itself)
  'Formatting', 'Slugification', 'Adaptation', 'Style', 'Culture', 'Market',
  // geography (7) — Geographic classifications (v0.12.4: Country added)
  'Continent', 'Country', 'GeoRegion', 'GeoSubRegion', 'IncomeGroup', 'LendingCategory', 'EconomicRegion',
  // knowledge (24) — Sets + Atoms + Linguistic/Cultural taxonomy + SEO/GEO
  'TermSet', 'ExpressionSet', 'PatternSet', 'CultureSet', 'TabooSet', 'AudienceSet',
  'Term', 'Expression', 'Pattern', 'CultureRef', 'Taboo', 'AudienceTrait',
  'LanguageFamily', 'LanguageBranch', 'CulturalRealm', 'CulturalSubRealm', 'PopulationCluster', 'PopulationSubCluster',
  // knowledge — SEO/GEO (6) — v11.5: moved from org to shared/knowledge
  'SEOKeyword', 'SEOKeywordMetrics', 'SEOKeywordSet',
  'GEOQuery', 'GEOQuerySet', 'GEOAnswer',

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM (21 nodes) — 6 layers: config, foundation, structure, semantic, instruction, output
  // v0.12.4: Brand Architecture (+4), PageStructure/PageInstruction deleted (-2)
  // ═══════════════════════════════════════════════════════════════════════════
  // config (1) — v11.3: Organization + Tenant merged into OrgConfig
  'OrgConfig',
  // foundation (6) — v0.12.4: Brand Architecture (Brand, BrandDesign, BrandPrinciples, PromptStyle)
  'Project', 'Brand', 'BrandDesign', 'BrandPrinciples', 'PromptStyle', 'ProjectContent',
  // structure (3)
  'Page', 'Block', 'ContentSlot',
  // semantic (4)
  'Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface',
  // instruction (4) — v0.12.4: PageStructure, PageInstruction deleted
  'BlockType', 'BlockInstruction', 'BlockRules', 'PromptArtifact',
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
 *   const { layer } = CLASS_TAXONOMY[type];
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
 * const isShared = CLASS_TAXONOMY[nodeType].realm === 'shared';
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
 * - **`foundation`**: Project identity (Project, Brand, BrandDesign, BrandPrinciples, PromptStyle, ProjectContent)
 * - **`structure`**: Page/Block hierarchy (Page, Block, ContentSlot)
 * - **`semantic`**: Business entities (Entity, EntityContent, AudiencePersona)
 * - **`instruction`**: Generation prompts (BlockType, BlockInstruction, BlockRules, PromptArtifact)
 * - **`output`**: Generated artifacts (PageGenerated, BlockGenerated)
 *
 * @example
 * ```typescript
 * // Get all semantic layer nodes
 * const semanticNodes = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].layer === 'semantic');
 * // → ['Entity', 'EntityContent', 'AudiencePersona', 'ChannelSurface']
 * ```
 *
 * @see ADR-020 — Layer reorganization
 */
export type Layer =
  | 'config' | 'locale' | 'geography' | 'knowledge'  // shared realm layers (4)
  | 'foundation' | 'structure' | 'semantic' | 'instruction' | 'output';  // org realm layers (6) — note: 'config' shared with shared realm

/**
 * Node trait classification: HOW the data originates (Data Origin axis).
 *
 * | Trait | Description | Visual Encoding | Example |
 * |-------|-------------|-----------------|---------|
 * | `defined` | Schema-defined, universal | Solid border | Entity, Page, Block |
 * | `authored` | Locale-specific authored content | Dashed border | EntityContent, ProjectContent |
 * | `imported` | Imported from external sources | Dotted border | Term, Expression, Pattern |
 * | `generated` | LLM-generated output | Double border | PageGenerated, BlockGenerated |
 * | `retrieved` | Retrieved/computed metrics | Dotted thin | SEOKeywordMetrics, GEOAnswer |
 *
 * @example
 * ```typescript
 * // Check if node needs locale-specific content
 * const needsAuthoring = CLASS_TAXONOMY[nodeType].trait === 'authored';
 *
 * // Get all generated output nodes
 * const outputs = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].trait === 'generated');
 * // → ['PageGenerated', 'BlockGenerated', 'OutputArtifact', 'PromptArtifact']
 * ```
 *
 * @see ADR-005 — Trait-based visual encoding
 * @see ADR-024 — Trait renames (Data Origin)
 */
export type Trait = 'defined' | 'authored' | 'imported' | 'generated' | 'retrieved';

// =============================================================================
// CLASS_TAXONOMY — unified classification for all 61 node types
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
 * const meta: Classification = CLASS_TAXONOMY['Entity'];
 * // → { realm: 'org', layer: 'semantic', trait: 'defined' }
 * ```
 */
export interface Classification {
  /** Where the node lives: 'shared' (universal) or 'org' (organization-specific) */
  realm: Realm;
  /** Functional category: one of 10 layers (4 shared + 6 org) */
  layer: Layer;
  /** Data origin: defined, authored, imported, generated, or retrieved */
  trait: Trait;
}

/**
 * Complete classification registry for all 61 NovaNet node types.
 *
 * This is the **single source of truth** for node classification. Use this to:
 * - Determine which realm/layer a node belongs to
 * - Filter nodes by classification
 * - Apply visual encoding based on classification
 *
 * @example
 * ```typescript
 * import { NODE_TYPES, CLASS_TAXONOMY } from '@novanet/core/types';
 *
 * // Get classification for a specific type
 * const { realm, layer, trait } = CLASS_TAXONOMY['Entity'];
 * // → { realm: 'org', layer: 'semantic', trait: 'defined' }
 *
 * // Count nodes by realm
 * const sharedCount = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].realm === 'shared').length;
 * // → 39
 *
 * // Get all output layer nodes
 * const outputs = NODE_TYPES.filter(t => CLASS_TAXONOMY[t].layer === 'output');
 * // → ['PageGenerated', 'BlockGenerated', 'OutputArtifact']
 * ```
 *
 * @version 0.12.0
 */
export const CLASS_TAXONOMY: Record<NodeType, Classification> = {
  // ═══════════════════════════════════════════════════════════════════════════
  // SHARED REALM — config (3) — v11.5: classification nodes + Locale definition
  // ═══════════════════════════════════════════════════════════════════════════
  EntityCategory:   { realm: 'shared', layer: 'config', trait: 'defined' },
  Locale:           { realm: 'shared', layer: 'config', trait: 'defined' },
  SEOKeywordFormat: { realm: 'shared', layer: 'config', trait: 'defined' },

  // SHARED REALM — locale (6) — locale SETTINGS
  Formatting:     { realm: 'shared', layer: 'locale', trait: 'imported' },
  Slugification:  { realm: 'shared', layer: 'locale', trait: 'imported' },
  Adaptation:     { realm: 'shared', layer: 'locale', trait: 'imported' },
  Style:          { realm: 'shared', layer: 'locale', trait: 'imported' },
  Culture:        { realm: 'shared', layer: 'locale', trait: 'imported' },
  Market:         { realm: 'shared', layer: 'locale', trait: 'imported' },

  // SHARED REALM — geography (7) — v0.12.4: Country added
  Continent:      { realm: 'shared', layer: 'geography', trait: 'defined' },
  Country:        { realm: 'shared', layer: 'geography', trait: 'defined' },
  GeoRegion:      { realm: 'shared', layer: 'geography', trait: 'defined' },
  GeoSubRegion:   { realm: 'shared', layer: 'geography', trait: 'defined' },
  IncomeGroup:    { realm: 'shared', layer: 'geography', trait: 'defined' },
  LendingCategory:{ realm: 'shared', layer: 'geography', trait: 'defined' },
  EconomicRegion: { realm: 'shared', layer: 'geography', trait: 'defined' },

  // SHARED REALM — knowledge (24) — containers, atoms, SEO/GEO
  TermSet:             { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  ExpressionSet:       { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  PatternSet:          { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  CultureSet:          { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  TabooSet:            { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  AudienceSet:         { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  Term:                { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  Expression:          { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  Pattern:             { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  CultureRef:          { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  Taboo:               { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  AudienceTrait:       { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  LanguageFamily:      { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  LanguageBranch:      { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  CulturalRealm:       { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  CulturalSubRealm:    { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  PopulationCluster:   { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  PopulationSubCluster:{ realm: 'shared', layer: 'knowledge', trait: 'imported' },

  // ═══════════════════════════════════════════════════════════════════════════
  // ORG REALM — config (1) — v11.3: Organization + Tenant merged
  // ═══════════════════════════════════════════════════════════════════════════
  OrgConfig: { realm: 'org', layer: 'config', trait: 'defined' },

  // ORG REALM — foundation (6) — v0.12.4: Brand Architecture
  Project:         { realm: 'org', layer: 'foundation',  trait: 'defined' },
  Brand:           { realm: 'org', layer: 'foundation',  trait: 'defined' },
  BrandDesign:     { realm: 'org', layer: 'foundation',  trait: 'defined' },
  BrandPrinciples: { realm: 'org', layer: 'foundation',  trait: 'defined' },
  PromptStyle:     { realm: 'org', layer: 'foundation',  trait: 'defined' },
  ProjectContent:  { realm: 'org', layer: 'foundation',  trait: 'authored' },

  // ORG REALM — structure (3)
  Page:         { realm: 'org', layer: 'structure',   trait: 'defined' },
  Block:        { realm: 'org', layer: 'structure',   trait: 'defined' },
  ContentSlot:  { realm: 'org', layer: 'structure',   trait: 'defined' },

  // ORG REALM — semantic (4)
  Entity:          { realm: 'org', layer: 'semantic', trait: 'defined' },
  EntityContent:   { realm: 'org', layer: 'semantic', trait: 'authored' },
  AudiencePersona: { realm: 'org', layer: 'semantic', trait: 'defined' },
  ChannelSurface:  { realm: 'org', layer: 'semantic', trait: 'defined' },

  // ORG REALM — instruction (4) — v0.12.4: PageStructure, PageInstruction deleted
  BlockType:         { realm: 'org', layer: 'instruction', trait: 'defined' },
  BlockInstruction:  { realm: 'org', layer: 'instruction', trait: 'defined' },
  BlockRules:        { realm: 'org', layer: 'instruction', trait: 'defined' },
  PromptArtifact:    { realm: 'org', layer: 'instruction', trait: 'generated' },

  // SHARED REALM — knowledge (SEO/GEO) — v11.5: moved from org to shared
  SEOKeyword:       { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  SEOKeywordMetrics:{ realm: 'shared', layer: 'knowledge', trait: 'retrieved' },
  SEOKeywordSet:    { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  GEOQuery:         { realm: 'shared', layer: 'knowledge', trait: 'imported' },
  GEOQuerySet:      { realm: 'shared', layer: 'knowledge', trait: 'defined' },
  GEOAnswer:        { realm: 'shared', layer: 'knowledge', trait: 'retrieved' },

  // ORG REALM — output (3)
  PageGenerated:  { realm: 'org', layer: 'output', trait: 'generated' },
  BlockGenerated: { realm: 'org', layer: 'output', trait: 'generated' },
  OutputArtifact: { realm: 'org', layer: 'output', trait: 'generated' },
};

// =============================================================================
// DERIVED MAPS — computed from CLASS_TAXONOMY
// =============================================================================

/**
 * Creates a derived lookup map from CLASS_TAXONOMY for a specific field.
 *
 * @internal
 * @param field - The Classification field to extract ('realm', 'layer', or 'trait')
 * @returns Record mapping NodeType to the extracted field value
 */
function deriveMap<K extends keyof Classification>(field: K): Record<NodeType, Classification[K]> {
  return Object.fromEntries(
    Object.entries(CLASS_TAXONOMY).map(([k, v]) => [k, v[field]])
  ) as Record<NodeType, Classification[K]>;
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
 * const trait = NODE_TRAITS['Entity'];        // → 'defined'
 * const trait = NODE_TRAITS['EntityContent']; // → 'authored'
 * const trait = NODE_TRAITS['PageGenerated']; // → 'generated'
 * ```
 */
export const NODE_TRAITS: Record<NodeType, Trait> = deriveMap('trait');
