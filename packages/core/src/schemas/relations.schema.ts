/**
 * @fileoverview NovaNet Relation Registry
 * @module @novanet/core/schemas/relations
 * @version 0.12.4
 *
 * Unified registry for all Neo4j relationship types in the NovaNet knowledge graph.
 * This module defines relation types, their property schemas, and the complete registry.
 *
 * **Relation Categories:**
 * - Project Root: HAS_PAGE, HAS_BRAND_IDENTITY, SUPPORTS_LOCALE
 * - Locale: DEFAULT_LOCALE, FALLBACK_TO, FOR_LOCALE, VARIANT_OF
 * - Locale Knowledge: HAS_IDENTITY, HAS_VOICE, HAS_CULTURE, HAS_MARKET, HAS_LEXICON
 * - Localization: HAS_CONTENT, CONTENT_OF
 * - Page Structure: HAS_BLOCK, OF_TYPE, LINKS_TO, SUBTOPIC_OF
 * - Entity Usage: USES_ENTITY, REFERENCES, SEMANTIC_LINK, BELONGS_TO, HAS_KEYWORD
 * - Output: HAS_GENERATED, HAS_METRICS, ASSEMBLES
 * - SEO/GEO Targeting: HAS_SEO_TARGET, HAS_GEO_TARGET, TARGETS_SEO, TARGETS_GEO
 * - Provenance: INFLUENCED_BY, GENERATED_FROM, GENERATED
 * - Inverse: CONTENT_OF, GENERATED_FOR, BLOCK_OF, USED_BY
 *
 * @example
 * ```typescript
 * import { RelationType, RelationRegistry } from '@novanet/core/schemas/relations';
 *
 * // Get relation definition
 * const hasPage = RelationRegistry[RelationType.HAS_PAGE];
 * console.log(hasPage.cardinality); // '1:N'
 * ```
 *
 * @see ADR-015 — Unidirectional Ownership Arcs
 * @see packages/core/models/arc-kinds/ — Arc YAML definitions
 */

import { z } from 'zod';

// =============================================================================
// RELATION TYPES
// =============================================================================

export const RelationType = {
  // ─────────────────────────────────────────────────────────────────────────────
  // PROJECT ROOT (v0.12.4: Brand Architecture — HAS_BRAND_IDENTITY → HAS_BRAND)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_PAGE: 'HAS_PAGE',                   // Project → Page
  HAS_BRAND: 'HAS_BRAND',                 // Project → Brand (v0.12.4: renamed from HAS_BRAND_IDENTITY)
  SUPPORTS_LOCALE: 'SUPPORTS_LOCALE',     // Project → Locale

  // ─────────────────────────────────────────────────────────────────────────────
  // BRAND ARCHITECTURE (v0.12.4: ADR-028)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_DESIGN: 'HAS_DESIGN',               // Brand → BrandDesign
  HAS_PRINCIPLES: 'HAS_PRINCIPLES',       // Brand → BrandPrinciples
  HAS_PROMPT_STYLE: 'HAS_PROMPT_STYLE',   // Brand → PromptStyle

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALE
  // ─────────────────────────────────────────────────────────────────────────────
  DEFAULT_LOCALE: 'DEFAULT_LOCALE', // Project → Locale (exactly one per project)
  FALLBACK_TO: 'FALLBACK_TO',       // Locale → Locale
  FOR_LOCALE: 'FOR_LOCALE',         // Content → Locale
  VARIANT_OF: 'VARIANT_OF',         // Locale → Locale (regional variant)

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALE KNOWLEDGE
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_IDENTITY: 'HAS_IDENTITY',     // Locale → LocaleIdentity
  HAS_VOICE: 'HAS_VOICE',           // Locale → LocaleVoice
  HAS_CULTURE: 'HAS_CULTURE',       // Locale → LocaleCulture
  HAS_MARKET: 'HAS_MARKET',         // Locale → LocaleMarket
  HAS_LEXICON: 'HAS_LEXICON',       // Locale → LocaleLexicon
  HAS_EXPRESSION: 'HAS_EXPRESSION', // LocaleLexicon → Expression
  // v7.2.0: Locale Rules
  HAS_RULES_ADAPTATION: 'HAS_RULES_ADAPTATION',   // Locale → LocaleRulesAdaptation
  HAS_RULES_FORMATTING: 'HAS_RULES_FORMATTING',   // Locale → LocaleRulesFormatting
  HAS_RULES_SLUG: 'HAS_RULES_SLUG',               // Locale → LocaleRulesSlug
  // v7.2.0: Culture References
  HAS_CULTURE_REFERENCES: 'HAS_CULTURE_REFERENCES', // LocaleCulture → LocaleCultureReferences
  HAS_REFERENCE: 'HAS_REFERENCE',   // LocaleCultureReferences → Reference
  HAS_METAPHOR: 'HAS_METAPHOR',     // LocaleCultureReferences → Metaphor
  HAS_PATTERN: 'HAS_PATTERN',       // LocaleRulesFormatting → Pattern
  HAS_CONSTRAINT: 'HAS_CONSTRAINT', // LocaleCulture → Constraint

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALIZATION (v7.0.0: unified HAS_CONTENT for all *Content nodes - v10.9: L10n suffix deprecated)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_CONTENT: 'HAS_CONTENT',             // Entity|Project → *Content (v10.9: L10n → Content)

  // ─────────────────────────────────────────────────────────────────────────────
  // PAGE STRUCTURE
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_BLOCK: 'HAS_BLOCK',           // Page → Block
  OF_TYPE: 'OF_TYPE',               // Block → BlockType

  // ─────────────────────────────────────────────────────────────────────────────
  // PAGE RELATIONSHIPS (v7.12.0)
  // ─────────────────────────────────────────────────────────────────────────────
  LINKS_TO: 'LINKS_TO',             // Page → Page (explicit internal link with concept anchor)
  SUBTOPIC_OF: 'SUBTOPIC_OF',       // Page → Page (cluster → pillar hierarchy)

  // ─────────────────────────────────────────────────────────────────────────────
  // ENTITY USAGE (v10.3: renamed from USES_CONCEPT)
  // ─────────────────────────────────────────────────────────────────────────────
  USES_ENTITY: 'USES_ENTITY',       // Page|Block → Entity (v10.3: renamed from USES_CONCEPT)
  REFERENCES: 'REFERENCES',         // Block → Entity (v0.12.4: ADR-028 Page-Entity Architecture)
  SEMANTIC_LINK: 'SEMANTIC_LINK',   // Entity → Entity (v10.3: was Concept)
  BELONGS_TO: 'BELONGS_TO',         // Entity → EntityCategory (v11.1: semantic classification)
  HAS_KEYWORD: 'HAS_KEYWORD',       // Entity → SEOKeyword (v0.12.4: ADR-028 Page-Entity Architecture)
  POPULAR_IN: 'POPULAR_IN',         // Entity → Country|GeoRegion (v0.12.4: geographic popularity)

  // ─────────────────────────────────────────────────────────────────────────────
  // OUTPUT (v7.0.0: unified HAS_GENERATED)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_GENERATED: 'HAS_GENERATED',         // Page|Block → PageGenerated|BlockGenerated (v7.0.0: unified)
  HAS_METRICS: 'HAS_METRICS',       // PageGenerated → PageMetrics
  ASSEMBLES: 'ASSEMBLES',           // PageGenerated → BlockGenerated

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO TARGETING (v7.7.0: locale-aligned + cross-locale shortcuts)
  // ─────────────────────────────────────────────────────────────────────────────
  // v7.7.0: Locale-aligned primary targeting
  HAS_SEO_TARGET: 'HAS_SEO_TARGET',     // EntityContent → SEOKeyword (locale-aligned)
  HAS_GEO_TARGET: 'HAS_GEO_TARGET',     // EntityContent → GEOQuery (locale-aligned)
  // Cross-locale shortcuts (kept for management/reporting)
  TARGETS_SEO: 'TARGETS_SEO',           // Entity → SEOKeyword (v10.3: was Concept)
  TARGETS_GEO: 'TARGETS_GEO',           // Entity → GEOQuery (v10.3: was Concept)
  // REMOVED v7.8.1: PAGE_TARGETS_SEO, PAGE_TARGETS_GEO
  // Reason: Direct Page → SEO/GEO bypasses semantic grouping
  // Correct flow: Page → Entity → EntityContent → SEOKeyword/GEOQuery

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO MINING (v11.2: Mining runs removed, deferred to v12+)
  // ─────────────────────────────────────────────────────────────────────────────
  // REMOVED v7.8.5: HAS_CITATION (replaced by HAS_METRICS)
  // REMOVED v7.8.4: GEO_DISCOVERED_BY, HAS_REFORMULATION, REFORMULATES (GEOReformulation deleted)

  // ─────────────────────────────────────────────────────────────────────────────
  // PROVENANCE (v7.9.0: REMOVED USED_SEO_KEYWORD, USED_GEO_SEED)
  // SEO/GEO provenance is implicit via: BlockGenerated → INFLUENCED_BY → EntityContent → HAS_*_TARGET → SEO/GEO
  // ─────────────────────────────────────────────────────────────────────────────
  INFLUENCED_BY: 'INFLUENCED_BY',       // BlockGenerated → EntityContent
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at EntityContent level)
  GENERATED_FROM: 'GENERATED_FROM',     // BlockGenerated → BlockType

  // ─────────────────────────────────────────────────────────────────────────────
  // OPTIMIZATION RELATIONS
  // ─────────────────────────────────────────────────────────────────────────────
  BELONGS_TO_PROJECT_CONTENT: 'BELONGS_TO_PROJECT_CONTENT', // PageGenerated → ProjectContent (locale-aligned)

  // ─────────────────────────────────────────────────────────────────────────────
  // INSTRUCTION RELATIONS (v0.12.4: PageInstruction deleted per ADR-028)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_INSTRUCTION: 'HAS_INSTRUCTION',   // Page|Block → BlockInstruction (v0.12.4: PageInstruction deleted)
  HAS_RULES: 'HAS_RULES',               // BlockType → BlockRules
  GENERATED: 'GENERATED',               // BlockInstruction → PageGenerated|BlockGenerated (provenance, v0.12.4)

  // ─────────────────────────────────────────────────────────────────────────────
  // VERSION HISTORY (v7.11.0)
  // ─────────────────────────────────────────────────────────────────────────────
  PREVIOUS_VERSION: 'PREVIOUS_VERSION', // BlockGenerated|PageGenerated → BlockGenerated|PageGenerated

  // ─────────────────────────────────────────────────────────────────────────────
  // INVERSE RELATIONSHIPS (v7.8.0 - bidirectional queries without full scans)
  // v10.9.0: L10N_OF → CONTENT_OF, HAS_LOCALIZED_CONTENT removed (ADR-014)
  // ─────────────────────────────────────────────────────────────────────────────
  CONTENT_OF: 'CONTENT_OF',             // EntityContent|ProjectContent → Entity|Project (inverse of HAS_CONTENT)
  GENERATED_FOR: 'GENERATED_FOR',       // PageGenerated|BlockGenerated → Page|Block (inverse of HAS_GENERATED)
  BLOCK_OF: 'BLOCK_OF',                 // Block → Page (inverse of HAS_BLOCK)
  USED_BY: 'USED_BY',                   // Entity → Page|Block (inverse of USES_ENTITY)
} as const;

export type RelationType = typeof RelationType[keyof typeof RelationType];

// =============================================================================
// RELATION DEFINITIONS
// =============================================================================

export interface RelationDefinition {
  type: RelationType;
  from: string | string[];
  to: string | string[];
  cardinality: '1:1' | '1:N' | 'N:1' | 'N:M';
  props?: z.ZodTypeAny;
  description: string;
}

// =============================================================================
// RELATION PROPERTY SCHEMAS
// =============================================================================

/**
 * Properties for SUPPORTS_LOCALE relation (Project → Locale).
 */
export const SupportsLocalePropsSchema = z.object({
  default: z.boolean()
    .describe('Whether this is the default locale for the project'),
}).describe('SUPPORTS_LOCALE relation properties');

/**
 * Properties for HAS_BLOCK relation (Page → Block).
 */
export const HasBlockPropsSchema = z.object({
  position: z.number()
    .int()
    .min(0)
    .describe('Zero-indexed position of block within page'),
}).describe('HAS_BLOCK relation properties with ordering');

// ─────────────────────────────────────────────────────────────────────────────
// PAGE RELATIONSHIPS PROPS (v7.12.0, extended v7.12.1)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Properties for LINKS_TO relation (Page → Page).
 *
 * Defines internal link properties including anchor text derivation,
 * context, SEO weight, and anchor type optimization.
 *
 * @example
 * ```typescript
 * const linkProps = LinksToPropsSchema.parse({
 *   concept_key: 'qr-code-generator',
 *   context: 'body',
 *   seo_weight: 0.8,
 *   anchor_type: 'partial_match',
 *   nofollow: false,
 * });
 * ```
 */
export const LinksToPropsSchema = z.object({
  concept_key: z.string()
    .regex(/^[a-z0-9-]+$/)
    .describe('Entity key for anchor text derivation from EntityContent.title'),
  context: z.enum(['cta', 'body', 'related', 'nav'])
    .describe('Where the link appears: CTA button, body text, related section, or navigation'),
  seo_weight: z.number()
    .min(0).max(1)
    .describe('Link importance for SEO (0-1 scale)'),
  anchor_type: z.enum(['exact_match', 'partial_match', 'branded', 'generic'])
    .default('partial_match')
    .describe('Anchor text optimization: exact_match (5× traffic, max 10%), partial_match (keywords), branded (brand name), generic (low SEO)'),
  nofollow: z.boolean()
    .default(false)
    .describe('Set true for login/legal pages to prevent PageRank flow'),
}).describe('LINKS_TO relation properties for internal linking');

/**
 * Properties for SEMANTIC_LINK relation (Entity → Entity).
 */
export const SemanticLinkPropsSchema = z.object({
  type: z.enum([
    'is_action_on', 'has_action',
    'includes', 'included_in',
    'type_of', 'has_type',
    'requires', 'required_by',
    'related', 'opposite',
  ]).describe('Semantic relationship type between entities'),
  temperature: z.number()
    .min(0).max(1)
    .describe('Activation spreading weight (0-1 scale)'),
}).describe('SEMANTIC_LINK relation properties for entity connections');

/**
 * Properties for USES_ENTITY relation (Page|Block → Entity).
 */
export const UsesEntityPropsSchema = z.object({
  purpose: z.enum(['primary', 'secondary', 'contextual'])
    .describe('Entity role: primary (main topic), secondary (supporting), contextual (mentioned)'),
  temperature: z.number()
    .min(0).max(1)
    .describe('Relevance weight for LLM context loading (0-1 scale)'),
}).describe('USES_ENTITY relation properties for content-entity connections');

/**
 * Properties for TARGETS_SEO relation (Entity → SEOKeyword).
 */
export const TargetsSEOPropsSchema = z.object({
  status: z.enum(['active', 'paused', 'archived'])
    .describe('Targeting status: active (being optimized), paused (temporary hold), archived (historical)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking for SEO efforts (1-10 scale)'),
}).describe('TARGETS_SEO relation properties for cross-locale SEO shortcuts');

/**
 * Properties for TARGETS_GEO relation (Entity → GEOQuery).
 */
export const TargetsGEOPropsSchema = z.object({
  status: z.enum(['active', 'monitoring', 'archived'])
    .describe('Targeting status: active (optimizing), monitoring (tracking), archived (historical)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking for GEO efforts (1-10 scale)'),
}).describe('TARGETS_GEO relation properties for cross-locale GEO shortcuts');

// v7.7.0: Locale-aligned targeting (EntityContent → SEO/GEO)

/**
 * Properties for HAS_SEO_TARGET relation (EntityContent → SEOKeyword).
 */
export const HasSEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'secondary', 'long-tail'])
    .describe('Keyword role: primary (main target), secondary (supporting), long-tail (niche)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking within role (1-10 scale)'),
}).describe('HAS_SEO_TARGET relation properties for locale-aligned SEO');

/**
 * Properties for HAS_GEO_TARGET relation (EntityContent → GEOQuery).
 */
export const HasGEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'contextual'])
    .describe('Query role: primary (main intent), contextual (related)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking within role (1-10 scale)'),
}).describe('HAS_GEO_TARGET relation properties for locale-aligned GEO');

/**
 * Properties for ASSEMBLES relation (PageGenerated → BlockGenerated).
 */
export const AssemblesPropsSchema = z.object({
  position: z.number()
    .int()
    .min(0)
    .describe('Zero-indexed position of block within generated page'),
}).describe('ASSEMBLES relation properties for page assembly');

/**
 * Properties for INFLUENCED_BY relation (BlockGenerated → EntityContent).
 */
export const InfluencedByPropsSchema = z.object({
  weight: z.number()
    .min(0).max(1)
    .describe('Influence weight in generation (0-1 scale)'),
  concept_version: z.number()
    .int()
    .positive()
    .describe('Version of EntityContent used during generation'),
}).describe('INFLUENCED_BY relation properties for provenance tracking');

// ─────────────────────────────────────────────────────────────────────────────
// INSTRUCTION RELATION PROPS
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Properties for GENERATED relation (BlockInstruction → PageGenerated|BlockGenerated).
 * v0.12.4: PageInstruction deleted per ADR-028.
 */
export const GeneratedPropsSchema = z.object({
  generated_at: z.date()
    .describe('Timestamp when the generation occurred'),
}).describe('GENERATED relation properties for instruction-to-output provenance');

// =============================================================================
// RELATION REGISTRY
// =============================================================================

export const RelationRegistry: Record<RelationType, RelationDefinition> = {
  // ─────────────────────────────────────────────────────────────────────────────
  // PROJECT ROOT
  // ─────────────────────────────────────────────────────────────────────────────
  // v10.3: HAS_CONCEPT removed — Entity is in shared realm, use USES_ENTITY from Page/Block
  [RelationType.HAS_PAGE]: {
    type: RelationType.HAS_PAGE,
    from: 'Project',
    to: 'Page',
    cardinality: '1:N',
    description: 'Project contains pages',
  },
  // v0.12.4: HAS_BRAND_IDENTITY → HAS_BRAND + Brand Architecture arcs
  [RelationType.HAS_BRAND]: {
    type: RelationType.HAS_BRAND,
    from: 'Project',
    to: 'Brand',
    cardinality: '1:1',
    description: 'Project has one brand (visual/voice/style foundation)',
  },
  [RelationType.HAS_DESIGN]: {
    type: RelationType.HAS_DESIGN,
    from: 'Brand',
    to: 'BrandDesign',
    cardinality: '1:N',
    description: 'Brand has design tokens (colors, typography, spacing)',
  },
  [RelationType.HAS_PRINCIPLES]: {
    type: RelationType.HAS_PRINCIPLES,
    from: 'Brand',
    to: 'BrandPrinciples',
    cardinality: '1:N',
    description: 'Brand has guiding principles (voice, tone, writing rules)',
  },
  [RelationType.HAS_PROMPT_STYLE]: {
    type: RelationType.HAS_PROMPT_STYLE,
    from: 'Brand',
    to: 'PromptStyle',
    cardinality: '1:N',
    description: 'Brand has prompt styles for LLM generation',
  },
  [RelationType.SUPPORTS_LOCALE]: {
    type: RelationType.SUPPORTS_LOCALE,
    from: 'Project',
    to: 'Locale',
    cardinality: '1:N',
    props: SupportsLocalePropsSchema,
    description: 'Project declares which locales it supports',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALE
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.DEFAULT_LOCALE]: {
    type: RelationType.DEFAULT_LOCALE,
    from: 'Project',
    to: 'Locale',
    cardinality: '1:1',
    description: 'Project has exactly one default/fallback locale',
  },
  [RelationType.FALLBACK_TO]: {
    type: RelationType.FALLBACK_TO,
    from: 'Locale',
    to: 'Locale',
    cardinality: 'N:1',
    description: 'Locale falls back to another locale for missing content',
  },
  [RelationType.FOR_LOCALE]: {
    type: RelationType.FOR_LOCALE,
    from: ['EntityContent', 'ProjectContent', 'PageGenerated', 'BlockGenerated', 'SEOKeyword', 'GEOQuery'],
    to: 'Locale',
    cardinality: 'N:1',
    description: 'Content node targets a specific locale',
  },
  [RelationType.VARIANT_OF]: {
    type: RelationType.VARIANT_OF,
    from: 'Locale',
    to: 'Locale',
    cardinality: 'N:1',
    description: 'Regional variant relationship (en-AU → en-GB for inheritance)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALE KNOWLEDGE
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_IDENTITY]: {
    type: RelationType.HAS_IDENTITY,
    from: 'Locale',
    to: 'LocaleIdentity',
    cardinality: '1:1',
    description: 'Locale has identity (script, timezone, formats)',
  },
  [RelationType.HAS_VOICE]: {
    type: RelationType.HAS_VOICE,
    from: 'Locale',
    to: 'LocaleVoice',
    cardinality: '1:1',
    description: 'Locale has voice (formality, tone, pronouns)',
  },
  [RelationType.HAS_CULTURE]: {
    type: RelationType.HAS_CULTURE,
    from: 'Locale',
    to: 'LocaleCulture',
    cardinality: '1:1',
    description: 'Locale has culture (norms, taboos, references)',
  },
  [RelationType.HAS_MARKET]: {
    type: RelationType.HAS_MARKET,
    from: 'Locale',
    to: 'LocaleMarket',
    cardinality: '1:1',
    description: 'Locale has market data (demographics, platforms)',
  },
  [RelationType.HAS_LEXICON]: {
    type: RelationType.HAS_LEXICON,
    from: 'Locale',
    to: 'LocaleLexicon',
    cardinality: '1:1',
    description: 'Locale has lexicon (vocabulary preferences)',
  },
  [RelationType.HAS_EXPRESSION]: {
    type: RelationType.HAS_EXPRESSION,
    from: 'LocaleLexicon',
    to: 'Expression',
    cardinality: '1:N',
    description: 'Lexicon contains expressions per semantic field',
  },
  // v7.2.0: Locale Rules
  [RelationType.HAS_RULES_ADAPTATION]: {
    type: RelationType.HAS_RULES_ADAPTATION,
    from: 'Locale',
    to: 'LocaleRulesAdaptation',
    cardinality: '1:1',
    description: 'Content adaptation rules (measurement units, date formats, honorifics)',
  },
  [RelationType.HAS_RULES_FORMATTING]: {
    type: RelationType.HAS_RULES_FORMATTING,
    from: 'Locale',
    to: 'LocaleRulesFormatting',
    cardinality: '1:1',
    description: 'Text formatting rules (number formats, currency placement)',
  },
  [RelationType.HAS_RULES_SLUG]: {
    type: RelationType.HAS_RULES_SLUG,
    from: 'Locale',
    to: 'LocaleRulesSlug',
    cardinality: '1:1',
    description: 'URL slug rules (character transliteration, word separators)',
  },
  // v7.2.0: Culture References
  [RelationType.HAS_CULTURE_REFERENCES]: {
    type: RelationType.HAS_CULTURE_REFERENCES,
    from: 'LocaleCulture',
    to: 'LocaleCultureReferences',
    cardinality: '1:1',
    description: 'Cultural touchpoints (celebrities, events, idioms)',
  },
  [RelationType.HAS_REFERENCE]: {
    type: RelationType.HAS_REFERENCE,
    from: 'LocaleCultureReferences',
    to: 'Reference',
    cardinality: '1:N',
    description: 'Cultural reference entity (person, event, brand)',
  },
  [RelationType.HAS_METAPHOR]: {
    type: RelationType.HAS_METAPHOR,
    from: 'LocaleCultureReferences',
    to: 'Metaphor',
    cardinality: '1:N',
    description: 'Cultural metaphor (locale-specific imagery)',
  },
  [RelationType.HAS_PATTERN]: {
    type: RelationType.HAS_PATTERN,
    from: 'LocaleRulesFormatting',
    to: 'Pattern',
    cardinality: '1:N',
    description: 'Formatting pattern (regex/template for dates, numbers, addresses)',
  },
  [RelationType.HAS_CONSTRAINT]: {
    type: RelationType.HAS_CONSTRAINT,
    from: 'LocaleCulture',
    to: 'Constraint',
    cardinality: '1:N',
    description: 'Cultural constraint (topics to avoid, sensitivities)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // LOCALIZATION (v7.0.0: unified HAS_CONTENT for all *L10n nodes)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_CONTENT]: {
    type: RelationType.HAS_CONTENT,
    from: ['Entity', 'Project'],
    to: ['EntityContent', 'ProjectContent'],
    cardinality: '1:N',
    description: 'Defined node has authored content (v11.8: ADR-024 renamed)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // PAGE STRUCTURE
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_BLOCK]: {
    type: RelationType.HAS_BLOCK,
    from: 'Page',
    to: 'Block',
    cardinality: '1:N',
    props: HasBlockPropsSchema,
    description: 'Page contains blocks with position',
  },
  // v0.12.4: HAS_STRUCTURE removed (PageStructure node deleted per ADR-028)
  // Page structure is now computed from HAS_BLOCK.order at runtime
  [RelationType.OF_TYPE]: {
    type: RelationType.OF_TYPE,
    from: 'Block',
    to: 'BlockType',
    cardinality: 'N:1',
    description: 'Block is of a specific type (Block → BlockType)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // PAGE RELATIONSHIPS (v7.12.0)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.LINKS_TO]: {
    type: RelationType.LINKS_TO,
    from: 'Page',
    to: 'Page',
    cardinality: 'N:M',
    props: LinksToPropsSchema,
    description: 'Explicit internal link for SEO. Anchor text derived from EntityContent.title (v7.12.0)',
  },
  [RelationType.SUBTOPIC_OF]: {
    type: RelationType.SUBTOPIC_OF,
    from: 'Page',
    to: 'Page',
    cardinality: 'N:1',
    description: 'Cluster page is subtopic of pillar page (pillar-cluster SEO hierarchy, v7.12.0)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // ENTITY USAGE (v10.3: renamed from USES_CONCEPT)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.USES_ENTITY]: {
    type: RelationType.USES_ENTITY,
    from: ['Page', 'Block'],
    to: 'Entity',
    cardinality: 'N:M',
    props: UsesEntityPropsSchema,
    description: 'Page or Block references entities via @key (v10.3: renamed from USES_CONCEPT)',
  },
  [RelationType.REFERENCES]: {
    type: RelationType.REFERENCES,
    from: 'Block',
    to: 'Entity',
    cardinality: 'N:M',
    description: 'Block references Entity for content generation context (v0.12.4: ADR-028)',
  },
  [RelationType.SEMANTIC_LINK]: {
    type: RelationType.SEMANTIC_LINK,
    from: 'Entity',
    to: 'Entity',
    cardinality: 'N:M',
    props: SemanticLinkPropsSchema,
    description: 'Entities are semantically linked for spreading activation (v10.3: was Concept)',
  },
  [RelationType.BELONGS_TO]: {
    type: RelationType.BELONGS_TO,
    from: 'Entity',
    to: 'EntityCategory',
    cardinality: 'N:1',
    description: 'Entity belongs to a semantic category (v11.1: cross-realm classification)',
  },
  [RelationType.HAS_KEYWORD]: {
    type: RelationType.HAS_KEYWORD,
    from: 'Entity',
    to: 'SEOKeyword',
    cardinality: '1:N',
    description: 'Entity has associated SEO keywords for targeting (v0.12.4: ADR-028)',
  },
  [RelationType.POPULAR_IN]: {
    type: RelationType.POPULAR_IN,
    from: 'Entity',
    to: ['Country', 'GeoRegion', 'Continent'],
    cardinality: 'N:M',
    description: 'Entity is popular in geographic regions (v0.12.4: cross-realm semantic)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // OUTPUT (v7.0.0: unified HAS_GENERATED)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_GENERATED]: {
    type: RelationType.HAS_GENERATED,
    from: ['Page', 'Block'],
    to: ['PageGenerated', 'BlockGenerated'],
    cardinality: '1:N',
    description: 'Page or Block has generated output per locale (v7.0.0: unified)',
  },
  [RelationType.HAS_METRICS]: {
    type: RelationType.HAS_METRICS,
    from: ['SEOKeyword', 'GEOQuery'],
    to: ['SEOKeywordMetrics', 'GEOAnswer'],
    cardinality: '1:N',
    description: 'Time-series observations (v7.11.0: PageMetrics removed, query GA/PostHog)',
    // REMOVED v7.11.0: PageGenerated → PageMetrics (query GA/PostHog with published_at/replaced_at date ranges)
    // SEOKeyword → SEOKeywordMetrics (keyword ranking/volume history)
    // GEOQuery → GEOAnswer (AI citation observations)
  },
  [RelationType.ASSEMBLES]: {
    type: RelationType.ASSEMBLES,
    from: 'PageGenerated',
    to: 'BlockGenerated',
    cardinality: '1:N',
    props: AssemblesPropsSchema,
    description: 'PageGenerated assembles BlockGenerateds with position',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO TARGETING (v7.7.0: locale-aligned + cross-locale shortcuts)
  // ─────────────────────────────────────────────────────────────────────────────
  // v7.7.0: Locale-aligned primary targeting
  [RelationType.HAS_SEO_TARGET]: {
    type: RelationType.HAS_SEO_TARGET,
    from: 'EntityContent',
    to: 'SEOKeyword',
    cardinality: '1:N',
    props: HasSEOTargetPropsSchema,
    description: 'Primary SEO targeting - locale-aligned (EntityContent and SEOKeyword share same locale)',
  },
  [RelationType.HAS_GEO_TARGET]: {
    type: RelationType.HAS_GEO_TARGET,
    from: 'EntityContent',
    to: 'GEOQuery',
    cardinality: '1:N',
    props: HasGEOTargetPropsSchema,
    description: 'Primary GEO targeting - locale-aligned (EntityContent and GEOQuery share same locale)',
  },
  // Cross-locale shortcuts (kept for management/reporting)
  [RelationType.TARGETS_SEO]: {
    type: RelationType.TARGETS_SEO,
    from: 'Entity',
    to: 'SEOKeyword',
    cardinality: '1:N',
    props: TargetsSEOPropsSchema,
    description: 'Cross-locale SEO shortcut for management/reporting',
  },
  [RelationType.TARGETS_GEO]: {
    type: RelationType.TARGETS_GEO,
    from: 'Entity',
    to: 'GEOQuery',
    cardinality: '1:N',
    props: TargetsGEOPropsSchema,
    description: 'Cross-locale GEO shortcut for management/reporting',
  },
  // REMOVED v7.8.1: PAGE_TARGETS_SEO and PAGE_TARGETS_GEO definitions
  // Reason: Direct Page → SEO/GEO bypasses semantic grouping
  // Correct flow: Page → Entity → EntityContent → SEOKeyword/GEOQuery

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO MINING (v11.2: Mining runs removed, deferred to v12+)
  // ─────────────────────────────────────────────────────────────────────────────

  // ─────────────────────────────────────────────────────────────────────────────
  // PROVENANCE
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.INFLUENCED_BY]: {
    type: RelationType.INFLUENCED_BY,
    from: 'BlockGenerated',
    to: 'EntityContent',
    cardinality: 'N:M',
    props: InfluencedByPropsSchema,
    description: 'BlockGenerated was influenced by EntityContent (provenance)',
  },
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at EntityContent level)
  // Provenance is implicit via: BlockGenerated → INFLUENCED_BY → EntityContent → HAS_*_TARGET → SEO/GEO

  [RelationType.GENERATED_FROM]: {
    type: RelationType.GENERATED_FROM,
    from: 'BlockGenerated',
    to: 'BlockType',
    cardinality: 'N:1',
    description: 'BlockGenerated was generated from a BlockType template',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // OPTIMIZATION RELATIONS
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.BELONGS_TO_PROJECT_CONTENT]: {
    type: RelationType.BELONGS_TO_PROJECT_CONTENT,
    from: 'PageGenerated',
    to: 'ProjectContent',
    cardinality: 'N:1',
    description: 'PageGenerated belongs to ProjectContent for locale-aligned generation context (voice, tagline, CTAs)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // INSTRUCTION RELATIONS (v0.12.4: PageInstruction deleted per ADR-028)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_INSTRUCTION]: {
    type: RelationType.HAS_INSTRUCTION,
    from: ['Page', 'Block'],
    to: 'BlockInstruction',
    cardinality: '1:N',
    description: 'Links structure nodes to their AI instructions (v0.12.4: PageInstruction deleted)',
  },
  [RelationType.HAS_RULES]: {
    type: RelationType.HAS_RULES,
    from: 'BlockType',
    to: 'BlockRules',
    cardinality: '1:N',
    description: 'Links BlockType to generation rules',
  },
  [RelationType.GENERATED]: {
    type: RelationType.GENERATED,
    from: 'BlockInstruction',
    to: ['PageGenerated', 'BlockGenerated'],
    cardinality: 'N:M',
    props: GeneratedPropsSchema,
    description: 'Provenance: which instruction generated which output (v0.12.4)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // VERSION HISTORY (v7.11.0)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.PREVIOUS_VERSION]: {
    type: RelationType.PREVIOUS_VERSION,
    from: ['BlockGenerated', 'PageGenerated'],
    to: ['BlockGenerated', 'PageGenerated'],
    cardinality: '1:1',
    description: 'Links to previous version in history chain (v7.11.0)',
    // Current version has replaced_at IS NULL
    // HAS_GENERATED always points to current version
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // INVERSE RELATIONSHIPS (v7.8.0 - bidirectional queries without full scans)
  // v10.9.0: L10N_OF → CONTENT_OF, OUTPUT_OF → GENERATED_FOR, HAS_LOCALIZED_CONTENT removed
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.CONTENT_OF]: {
    type: RelationType.CONTENT_OF,
    from: ['EntityContent', 'ProjectContent'],
    to: ['Entity', 'Project'],
    cardinality: 'N:1',
    description: 'Inverse of HAS_CONTENT - authored content points to parent',
  },
  [RelationType.GENERATED_FOR]: {
    type: RelationType.GENERATED_FOR,
    from: ['PageGenerated', 'BlockGenerated'],
    to: ['Page', 'Block'],
    cardinality: 'N:1',
    description: 'Inverse of HAS_GENERATED - generated content points to structure',
  },
  [RelationType.BLOCK_OF]: {
    type: RelationType.BLOCK_OF,
    from: 'Block',
    to: 'Page',
    cardinality: 'N:1',
    props: HasBlockPropsSchema,
    description: 'Inverse of HAS_BLOCK - block points to its page',
  },
  [RelationType.USED_BY]: {
    type: RelationType.USED_BY,
    from: 'Entity',
    to: ['Page', 'Block'],
    cardinality: '1:N',
    description: 'Inverse of USES_ENTITY - entity knows who uses it',
  },
};