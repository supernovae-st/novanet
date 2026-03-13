/**
 * @fileoverview NovaNet Relation Registry
 * @module @novanet/core/schemas/relations
 * @version 0.13.0
 *
 * Unified registry for all Neo4j relationship types in the NovaNet knowledge graph.
 * This module defines relation types, their property schemas, and the complete registry.
 *
 * **v0.13.0 ADR-029 *Native Pattern:**
 * - HAS_NATIVE + HAS_NATIVE → HAS_NATIVE (unified ownership arc)
 * - NATIVE_OF + NATIVE_OF → NATIVE_OF (unified inverse arc)
 * - EntityNative/ProjectNative/PageNative/BlockNative → *Native nodes
 *
 * **Relation Categories:**
 * - Project Root: HAS_PAGE, HAS_BRAND, SUPPORTS_LOCALE
 * - Locale: DEFAULT_LOCALE, FALLBACK_TO, FOR_LOCALE, VARIANT_OF
 * - Locale Knowledge: HAS_IDENTITY, HAS_VOICE, HAS_CULTURE, HAS_LEXICON (v0.18.0: HAS_MARKET removed)
 * - Native Content: HAS_NATIVE, NATIVE_OF (v0.13.0 ADR-029)
 * - Page Structure: HAS_BLOCK, OF_TYPE, LINKS_TO, SUBTOPIC_OF
 * - Entity Usage: USES_ENTITY, REFERENCES, SEMANTIC_LINK, BELONGS_TO (v0.16: HAS_KEYWORD removed)
 * - Output: HAS_METRICS, ASSEMBLES
 * - SEO/GEO Targeting: HAS_SEO_TARGET, HAS_GEO_TARGET, TARGETS_SEO, TARGETS_GEO
 * - Provenance: INFLUENCED_BY, GENERATED_FROM, GENERATED
 * - Inverse: NATIVE_OF, BLOCK_OF, USED_BY
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
 * @see packages/core/models/arc-classes/ — Arc YAML definitions
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
  // v0.18.0: HAS_MARKET removed (market data from external APIs, not static graph)
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
  // NATIVE CONTENT (v0.13.0 ADR-029: unified HAS_NATIVE for all *Native nodes)
  // Merges: HAS_CONTENT + HAS_GENERATED → HAS_NATIVE
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_NATIVE: 'HAS_NATIVE',               // Entity|Project|Page|Block → *Native (v0.13.0 ADR-029)

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
  // v0.16: HAS_KEYWORD removed — use TARGETS on EntityNative instead (per ontology brainstorm decision)
  POPULAR_IN: 'POPULAR_IN',         // Entity → Country|GeoRegion (v0.12.4: geographic popularity)

  // ─────────────────────────────────────────────────────────────────────────────
  // OUTPUT (v0.13.0: HAS_GENERATED merged into HAS_NATIVE per ADR-029)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_METRICS: 'HAS_METRICS',       // PageNative → PageMetrics
  ASSEMBLES: 'ASSEMBLES',           // PageNative → BlockNative (v0.13.0: ADR-029 *Native)

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO TARGETING (v7.7.0: locale-aligned + cross-locale shortcuts)
  // ─────────────────────────────────────────────────────────────────────────────
  // v7.7.0: Locale-aligned primary targeting
  HAS_SEO_TARGET: 'HAS_SEO_TARGET',     // EntityNative → SEOKeyword (locale-aligned)
  HAS_GEO_TARGET: 'HAS_GEO_TARGET',     // EntityNative → GEOQuery (locale-aligned)
  // Cross-locale shortcuts (kept for management/reporting)
  TARGETS_SEO: 'TARGETS_SEO',           // Entity → SEOKeyword (v10.3: was Concept)
  TARGETS_GEO: 'TARGETS_GEO',           // Entity → GEOQuery (v10.3: was Concept)
  // REMOVED v7.8.1: PAGE_TARGETS_SEO, PAGE_TARGETS_GEO
  // Reason: Direct Page → SEO/GEO bypasses semantic grouping
  // Correct flow: Page → Entity → EntityNative → SEOKeyword/GEOQuery

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO MINING (v11.2: Mining runs removed, deferred to v12+)
  // ─────────────────────────────────────────────────────────────────────────────
  // REMOVED v7.8.5: HAS_CITATION (replaced by HAS_METRICS)
  // REMOVED v7.8.4: GEO_DISCOVERED_BY, HAS_REFORMULATION, REFORMULATES (GEOReformulation deleted)

  // ─────────────────────────────────────────────────────────────────────────────
  // PROVENANCE (v7.9.0: REMOVED USED_SEO_KEYWORD, USED_GEO_SEED)
  // SEO/GEO provenance is implicit via: BlockNative → INFLUENCED_BY → EntityNative → HAS_*_TARGET → SEO/GEO
  // ─────────────────────────────────────────────────────────────────────────────
  INFLUENCED_BY: 'INFLUENCED_BY',       // BlockNative → EntityNative
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at EntityNative level)
  GENERATED_FROM: 'GENERATED_FROM',     // BlockNative → BlockType

  // ─────────────────────────────────────────────────────────────────────────────
  // OPTIMIZATION RELATIONS
  // ─────────────────────────────────────────────────────────────────────────────
  BELONGS_TO_PROJECT_CONTENT: 'BELONGS_TO_PROJECT_CONTENT', // PageNative → ProjectNative (locale-aligned)

  // ─────────────────────────────────────────────────────────────────────────────
  // INSTRUCTION RELATIONS (v0.12.4: PageInstruction deleted per ADR-028)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_INSTRUCTION: 'HAS_INSTRUCTION',   // Page|Block → BlockInstruction (v0.12.4: PageInstruction deleted)
  // v0.19.1: HAS_RULES removed — rules merged into BlockType.rules property
  GENERATED: 'GENERATED',               // BlockInstruction → PageNative|BlockNative (provenance, v0.13.0 ADR-029)

  // ─────────────────────────────────────────────────────────────────────────────
  // VERSION HISTORY (v7.11.0, v0.13.0: ADR-029 *Native)
  // ─────────────────────────────────────────────────────────────────────────────
  PREVIOUS_VERSION: 'PREVIOUS_VERSION', // BlockNative|PageNative → BlockNative|PageNative (v0.13.0)

  // ─────────────────────────────────────────────────────────────────────────────
  // INVERSE RELATIONSHIPS (v0.13.0 ADR-029: CONTENT_OF + GENERATED_FOR → NATIVE_OF)
  // ─────────────────────────────────────────────────────────────────────────────
  NATIVE_OF: 'NATIVE_OF',               // *Native → Entity|Project|Page|Block (inverse of HAS_NATIVE, v0.13.0)
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
    .describe('Entity key for anchor text derivation from EntityNative.title'),
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

// v7.7.0: Locale-aligned targeting (v0.13.0: EntityNative → EntityNative)

/**
 * Properties for HAS_SEO_TARGET relation (EntityNative → SEOKeyword).
 * v0.13.0: ADR-029 *Native pattern.
 */
export const HasSEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'secondary', 'long-tail'])
    .describe('Keyword role: primary (main target), secondary (supporting), long-tail (niche)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking within role (1-10 scale)'),
}).describe('HAS_SEO_TARGET relation properties for locale-aligned SEO');

/**
 * Properties for HAS_GEO_TARGET relation (EntityNative → GEOQuery).
 * v0.13.0: ADR-029 *Native pattern.
 */
export const HasGEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'contextual'])
    .describe('Query role: primary (main intent), contextual (related)'),
  priority: z.number()
    .int().min(1).max(10)
    .describe('Priority ranking within role (1-10 scale)'),
}).describe('HAS_GEO_TARGET relation properties for locale-aligned GEO');

/**
 * Properties for ASSEMBLES relation (PageNative → BlockNative).
 * v0.13.0: ADR-029 *Native pattern.
 */
export const AssemblesPropsSchema = z.object({
  position: z.number()
    .int()
    .min(0)
    .describe('Zero-indexed position of block within generated page'),
}).describe('ASSEMBLES relation properties for page assembly');

/**
 * Properties for INFLUENCED_BY relation (BlockNative → EntityNative).
 * v0.13.0: ADR-029 *Native pattern.
 */
export const InfluencedByPropsSchema = z.object({
  weight: z.number()
    .min(0).max(1)
    .describe('Influence weight in generation (0-1 scale)'),
  concept_version: z.number()
    .int()
    .positive()
    .describe('Version of EntityNative used during generation'),
}).describe('INFLUENCED_BY relation properties for provenance tracking');

// ─────────────────────────────────────────────────────────────────────────────
// INSTRUCTION RELATION PROPS
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Properties for GENERATED relation (BlockInstruction → PageNative|BlockNative).
 * v0.12.4: PageInstruction deleted per ADR-028.
 * v0.13.0: ADR-029 *Native pattern.
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
    from: ['EntityNative', 'ProjectNative', 'PageNative', 'BlockNative', 'SEOKeyword', 'GEOQuery'],
    to: 'Locale',
    cardinality: 'N:1',
    description: 'Native content node targets a specific locale (v0.13.0 ADR-029)',
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
  // v0.18.0: HAS_MARKET removed (market data from external APIs, not static graph)
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
  // NATIVE CONTENT (v0.13.0 ADR-029: unified HAS_NATIVE = HAS_CONTENT + HAS_GENERATED)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_NATIVE]: {
    type: RelationType.HAS_NATIVE,
    from: ['Entity', 'Project', 'Page', 'Block'],
    to: ['EntityNative', 'ProjectNative', 'PageNative', 'BlockNative'],
    cardinality: '1:N',
    description: 'Structure node has native content per locale (v0.13.0 ADR-029)',
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
    description: 'Explicit internal link for SEO. Anchor text derived from EntityNative.title (v7.12.0)',
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
  // v0.16: HAS_KEYWORD removed — use TARGETS on EntityNative instead
  [RelationType.POPULAR_IN]: {
    type: RelationType.POPULAR_IN,
    from: 'Entity',
    to: ['Country', 'GeoRegion', 'Continent'],
    cardinality: 'N:M',
    description: 'Entity is popular in geographic regions (v0.12.4: cross-realm semantic)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // OUTPUT RELATIONS (v0.13.0: HAS_GENERATED merged into HAS_NATIVE per ADR-029)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_METRICS]: {
    type: RelationType.HAS_METRICS,
    from: 'GEOQuery',
    to: 'GEOAnswer',
    cardinality: '1:N',
    description: 'GEO answer observations (v0.17.3: SEOKeywordMetrics removed, metrics on SEOKeyword node)',
    // v0.17.3: SEOKeyword → SEOKeywordMetrics removed (metrics stored on SEOKeyword node itself)
    // GEOQuery → GEOAnswer (AI citation observations) - kept
  },
  [RelationType.ASSEMBLES]: {
    type: RelationType.ASSEMBLES,
    from: 'PageNative',
    to: 'BlockNative',
    cardinality: '1:N',
    props: AssemblesPropsSchema,
    description: 'PageNative assembles BlockNatives with position (v0.13.0 ADR-029)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO TARGETING (v7.7.0: locale-aligned + cross-locale shortcuts)
  // ─────────────────────────────────────────────────────────────────────────────
  // v7.7.0: Locale-aligned primary targeting (v0.13.0: EntityNative → EntityNative)
  [RelationType.HAS_SEO_TARGET]: {
    type: RelationType.HAS_SEO_TARGET,
    from: 'EntityNative',
    to: 'SEOKeyword',
    cardinality: '1:N',
    props: HasSEOTargetPropsSchema,
    description: 'Primary SEO targeting - locale-aligned (EntityNative and SEOKeyword share same locale)',
  },
  [RelationType.HAS_GEO_TARGET]: {
    type: RelationType.HAS_GEO_TARGET,
    from: 'EntityNative',
    to: 'GEOQuery',
    cardinality: '1:N',
    props: HasGEOTargetPropsSchema,
    description: 'Primary GEO targeting - locale-aligned (EntityNative and GEOQuery share same locale)',
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
  // Correct flow: Page → Entity → EntityNative → SEOKeyword/GEOQuery

  // ─────────────────────────────────────────────────────────────────────────────
  // SEO/GEO MINING (v11.2: Mining runs removed, deferred to v12+)
  // ─────────────────────────────────────────────────────────────────────────────

  // ─────────────────────────────────────────────────────────────────────────────
  // PROVENANCE
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.INFLUENCED_BY]: {
    type: RelationType.INFLUENCED_BY,
    from: 'BlockNative',
    to: 'EntityNative',
    cardinality: 'N:M',
    props: InfluencedByPropsSchema,
    description: 'BlockNative was influenced by EntityNative (provenance, v0.13.0 ADR-029)',
  },
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at EntityNative level)
  // Provenance is implicit via: BlockNative → INFLUENCED_BY → EntityNative → HAS_*_TARGET → SEO/GEO (v0.13.0)

  [RelationType.GENERATED_FROM]: {
    type: RelationType.GENERATED_FROM,
    from: 'BlockNative',
    to: 'BlockType',
    cardinality: 'N:1',
    description: 'BlockNative was generated from a BlockType template (v0.13.0 ADR-029)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // OPTIMIZATION RELATIONS (v0.13.0: *Content/*Generated → *Native per ADR-029)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.BELONGS_TO_PROJECT_CONTENT]: {
    type: RelationType.BELONGS_TO_PROJECT_CONTENT,
    from: 'PageNative',
    to: 'ProjectNative',
    cardinality: 'N:1',
    description: 'PageNative belongs to ProjectNative for locale-aligned generation context (voice, tagline, CTAs). v0.13.0 ADR-029.',
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
  // v0.19.1: HAS_RULES removed — rules merged into BlockType.rules property
  [RelationType.GENERATED]: {
    type: RelationType.GENERATED,
    from: 'BlockInstruction',
    to: ['PageNative', 'BlockNative'],
    cardinality: 'N:M',
    props: GeneratedPropsSchema,
    description: 'Provenance: which instruction generated which output (v0.13.0 ADR-029)',
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // VERSION HISTORY (v7.11.0)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.PREVIOUS_VERSION]: {
    type: RelationType.PREVIOUS_VERSION,
    from: ['BlockNative', 'PageNative'],
    to: ['BlockNative', 'PageNative'],
    cardinality: '1:1',
    description: 'Links to previous version in history chain (v0.13.0 ADR-029)',
    // Current version has replaced_at IS NULL
    // HAS_NATIVE always points to current version
  },

  // ─────────────────────────────────────────────────────────────────────────────
  // INVERSE RELATIONSHIPS (v0.13.0: CONTENT_OF + GENERATED_FOR → NATIVE_OF per ADR-029)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.NATIVE_OF]: {
    type: RelationType.NATIVE_OF,
    from: ['EntityNative', 'ProjectNative', 'PageNative', 'BlockNative'],
    to: ['Entity', 'Project', 'Page', 'Block'],
    cardinality: 'N:1',
    description: 'Inverse of HAS_NATIVE - native content points to parent structure (v0.13.0 ADR-029)',
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