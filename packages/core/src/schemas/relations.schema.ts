// NovaNet Core - Unified Relation Registry v8.0.0
// Single source of truth for all Neo4j relationships
//
// v7.12.1 CHANGES:
//   - ADDED: anchor_type property to LINKS_TO (exact_match | partial_match | branded | generic)
//   - ADDED: nofollow property to LINKS_TO (boolean, default false)
//
// v7.12.0 CHANGES:
//   - ADDED: LINKS_TO (Page → Page) for explicit internal linking with concept-based anchors
//   - ADDED: SUBTOPIC_OF (Page → Page) for pillar-cluster SEO hierarchy
//
// v7.11.0 CHANGES:
//   - ADDED: PREVIOUS_VERSION relation for L10n history chains
//   - REMOVED: PageGenerated → PageMetrics from HAS_METRICS (query GA/PostHog with date ranges)
//
// v7.10.0 CHANGES:
//   - UPDATED: OF_TYPE now supports Page → PageType (mirrors Block → BlockType)
//
// v7.8.5 CHANGES:
//   - UNIFIED: HAS_METRICS for all time-series observations
//     - PageGenerated → PageMetrics (existing)
//     - SEOKeyword → SEOKeywordMetrics (renamed from SEOSnapshot)
//     - GEOQuery → GEOMetrics (renamed from GEOCitation)
//   - REMOVED: HAS_SNAPSHOT (replaced by HAS_METRICS)
//   - REMOVED: HAS_CITATION (replaced by HAS_METRICS)
//
// v7.8.3 CHANGES:
//   - RENAMED: GEOSeed → GEOQuery (all LOCALIZED nodes use *L10n suffix)
//   - Updated all relations that reference GEOSeed
//
// v7.8.2 CHANGES:
//   - RENAMED: SEOKeyword → SEOKeyword (all LOCALIZED nodes use *L10n suffix)
//   - Updated all relations that reference SEOKeyword
//
// v7.2.0 CHANGES:
//   - ADDED: HAS_PROMPT, HAS_RULES, GENERATED relations for Prompt nodes
//   - ADDED: GeneratedPropsSchema for provenance tracking
//
// v7.1.0 CHANGES:
//   - ADDED: priority/freshness fields to all nodes for context budgeting
//   - ADDED: Standardized llm_context format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
//   - ADDED: UsedSEOKeywordPropsSchema, UsedGEOSeedPropsSchema for provenance
//
// v7.0.0 CHANGES:
//   - PAGE_USES_CONCEPT + BLOCK_USES_CONCEPT → USES_CONCEPT (v7.0) → USES_ENTITY (v10.3)
//   - HAS_PAGE_OUTPUT + HAS_BLOCK_OUTPUT → HAS_GENERATED (unified)
//   - Standard property: llm_hints → llm_context

import { z } from 'zod';

// =============================================================================
// RELATION TYPES
// =============================================================================

export const RelationType = {
  // ─────────────────────────────────────────────────────────────────────────────
  // PROJECT ROOT (v10.3: HAS_CONCEPT removed — Entity in global realm, use USES_ENTITY)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_PAGE: 'HAS_PAGE',                   // Project → Page
  HAS_BRAND_IDENTITY: 'HAS_BRAND_IDENTITY', // Project → BrandIdentity
  SUPPORTS_LOCALE: 'SUPPORTS_LOCALE',     // Project → Locale

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
  // LOCALIZATION (v7.0.0: unified HAS_CONTENT for all *L10n nodes)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_CONTENT: 'HAS_CONTENT',             // Entity|Project → *L10n (v10.3: was Concept)

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
  SEMANTIC_LINK: 'SEMANTIC_LINK',   // Entity → Entity (v10.3: was Concept)
  BELONGS_TO: 'BELONGS_TO',         // Entity → EntityCategory (v11.1: semantic classification)

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
  // SEO MINING (v7.8.5: HAS_SNAPSHOT → HAS_METRICS)
  // ─────────────────────────────────────────────────────────────────────────────
  SEO_MINES: 'SEO_MINES',               // SEOMiningRun → SEOKeyword
  // REMOVED v7.8.5: HAS_SNAPSHOT (replaced by HAS_METRICS)
  // REMOVED v7.8.4: SEO_DISCOVERED_BY, HAS_VARIATION, VARIATES (SEOVariation deleted)

  // ─────────────────────────────────────────────────────────────────────────────
  // GEO MINING (v7.8.5: HAS_CITATION → HAS_METRICS)
  // ─────────────────────────────────────────────────────────────────────────────
  GEO_MINES: 'GEO_MINES',               // GEOMiningRun → GEOQuery
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
  // PROMPT RELATIONS (v7.2.0 - AI instructions with versioning)
  // ─────────────────────────────────────────────────────────────────────────────
  HAS_PROMPT: 'HAS_PROMPT',   // Page|Block → PagePrompt|BlockPrompt
  HAS_RULES: 'HAS_RULES',     // BlockType → BlockRules
  GENERATED: 'GENERATED',     // PagePrompt|BlockPrompt → PageGenerated|BlockGenerated (provenance)

  // ─────────────────────────────────────────────────────────────────────────────
  // VERSION HISTORY (v7.11.0)
  // ─────────────────────────────────────────────────────────────────────────────
  PREVIOUS_VERSION: 'PREVIOUS_VERSION', // BlockGenerated|PageGenerated → BlockGenerated|PageGenerated

  // ─────────────────────────────────────────────────────────────────────────────
  // INVERSE RELATIONSHIPS (v7.8.0 - bidirectional queries without full scans)
  // ─────────────────────────────────────────────────────────────────────────────
  L10N_OF: 'L10N_OF',                   // EntityContent|ProjectContent → Entity|Project (inverse of HAS_CONTENT)
  OUTPUT_OF: 'OUTPUT_OF',               // PageGenerated|BlockGenerated → Page|Block (inverse of HAS_GENERATED)
  BLOCK_OF: 'BLOCK_OF',                 // Block → Page (inverse of HAS_BLOCK)
  USED_BY: 'USED_BY',                   // Entity → Page|Block (inverse of USES_ENTITY)
  HAS_LOCALIZED_CONTENT: 'HAS_LOCALIZED_CONTENT', // Locale → *L10n nodes (inverse of FOR_LOCALE)
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

// Relation property schemas
export const SupportsLocalePropsSchema = z.object({
  default: z.boolean(),
});

export const HasBlockPropsSchema = z.object({
  position: z.number().int().min(0),
});

// ─────────────────────────────────────────────────────────────────────────────
// PAGE RELATIONSHIPS PROPS (v7.12.0, extended v7.12.1)
// ─────────────────────────────────────────────────────────────────────────────

export const LinksToPropsSchema = z.object({
  concept_key: z.string().regex(/^[a-z0-9-]+$/),  // Anchor text derived from EntityContent.title
  context: z.enum(['cta', 'body', 'related', 'nav']),  // Where link appears
  seo_weight: z.number().min(0).max(1),  // Link importance for SEO
  // v7.12.1: SEO anchor optimization
  anchor_type: z.enum(['exact_match', 'partial_match', 'branded', 'generic']).default('partial_match'),
  // exact_match: anchor = EntityContent.title exactly (5× traffic, use sparingly max 10%)
  // partial_match: anchor includes concept keywords
  // branded: anchor = brand name (QR Code AI)
  // generic: anchor = "click here", "learn more" (low SEO value)
  nofollow: z.boolean().default(false),  // Set true for login/legal pages to prevent equity flow
});

export const SemanticLinkPropsSchema = z.object({
  type: z.enum([
    'is_action_on', 'has_action',
    'includes', 'included_in',
    'type_of', 'has_type',
    'requires', 'required_by',
    'related', 'opposite',
  ]),
  temperature: z.number().min(0).max(1),
});

export const UsesEntityPropsSchema = z.object({
  purpose: z.enum(['primary', 'secondary', 'contextual']),
  temperature: z.number().min(0).max(1),
});

export const TargetsSEOPropsSchema = z.object({
  status: z.enum(['active', 'paused', 'archived']),
  priority: z.number().int().min(1).max(10),
});

export const TargetsGEOPropsSchema = z.object({
  status: z.enum(['active', 'monitoring', 'archived']),
  priority: z.number().int().min(1).max(10),
});

// v7.7.0: Locale-aligned targeting (EntityContent → SEO/GEO)
export const HasSEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'secondary', 'long-tail']),
  priority: z.number().int().min(1).max(10),
});

export const HasGEOTargetPropsSchema = z.object({
  role: z.enum(['primary', 'contextual']),
  priority: z.number().int().min(1).max(10),
});

export const AssemblesPropsSchema = z.object({
  position: z.number().int().min(0),
});

export const InfluencedByPropsSchema = z.object({
  weight: z.number().min(0).max(1),
  concept_version: z.number().int().positive(),
});

export const UsedSEOKeywordPropsSchema = z.object({
  weight: z.number().min(0).max(1),
});

export const UsedGEOQueryPropsSchema = z.object({
  weight: z.number().min(0).max(1),
});

// ─────────────────────────────────────────────────────────────────────────────
// PROMPT RELATION PROPS (v7.2.0)
// ─────────────────────────────────────────────────────────────────────────────

export const GeneratedPropsSchema = z.object({
  generated_at: z.date(),
});

// =============================================================================
// RELATION REGISTRY
// =============================================================================

export const RelationRegistry: Record<RelationType, RelationDefinition> = {
  // ─────────────────────────────────────────────────────────────────────────────
  // PROJECT ROOT
  // ─────────────────────────────────────────────────────────────────────────────
  // v10.3: HAS_CONCEPT removed — Entity is in global realm, use USES_ENTITY from Page/Block
  [RelationType.HAS_PAGE]: {
    type: RelationType.HAS_PAGE,
    from: 'Project',
    to: 'Page',
    cardinality: '1:N',
    description: 'Project contains pages',
  },
  [RelationType.HAS_BRAND_IDENTITY]: {
    type: RelationType.HAS_BRAND_IDENTITY,
    from: 'Project',
    to: 'BrandIdentity',
    cardinality: '1:1',
    description: 'Project has one brand identity (visual/voice/style)',
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
    description: 'Invariant node has localized content (v7.0.0: unified)',
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
  [RelationType.OF_TYPE]: {
    type: RelationType.OF_TYPE,
    from: ['Page', 'Block'],
    to: ['PageType', 'BlockType'],
    cardinality: 'N:1',
    description: 'Page or Block is of a specific type (v7.10.0: Page → PageType, Block → BlockType)',
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
    to: ['SEOKeywordMetrics', 'GEOMetrics'],
    cardinality: '1:N',
    description: 'Time-series observations (v7.11.0: PageMetrics removed, query GA/PostHog)',
    // REMOVED v7.11.0: PageGenerated → PageMetrics (query GA/PostHog with published_at/replaced_at date ranges)
    // SEOKeyword → SEOKeywordMetrics (keyword ranking/volume history)
    // GEOQuery → GEOMetrics (AI citation observations)
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
  // SEO MINING (v7.8.5: HAS_SNAPSHOT → HAS_METRICS)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.SEO_MINES]: {
    type: RelationType.SEO_MINES,
    from: 'SEOMiningRun',
    to: 'SEOKeyword',
    cardinality: 'N:M',
    description: 'Mining run targets SEO keywords (v7.8.2: SEOKeyword → SEOKeyword)',
  },
  // REMOVED v7.8.5: HAS_SNAPSHOT (use HAS_METRICS: SEOKeyword → SEOKeywordMetrics)
  // REMOVED v7.8.4: SEO_DISCOVERED_BY, HAS_VARIATION, VARIATES (SEOVariation deleted)

  // ─────────────────────────────────────────────────────────────────────────────
  // GEO MINING (v7.8.5: HAS_CITATION → HAS_METRICS)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.GEO_MINES]: {
    type: RelationType.GEO_MINES,
    from: 'GEOMiningRun',
    to: 'GEOQuery',
    cardinality: 'N:M',
    description: 'Mining run targets GEO seeds (v7.8.3: GEOSeed → GEOQuery)',
  },
  // REMOVED v7.8.5: HAS_CITATION (use HAS_METRICS: GEOQuery → GEOMetrics)
  // REMOVED v7.8.4: GEO_DISCOVERED_BY, HAS_REFORMULATION, REFORMULATES (GEOReformulation deleted)

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
  // PROMPT RELATIONS (v7.2.0 - AI instructions with versioning)
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.HAS_PROMPT]: {
    type: RelationType.HAS_PROMPT,
    from: ['Page', 'Block'],
    to: ['PagePrompt', 'BlockPrompt'],
    cardinality: '1:N',
    description: 'Links structure nodes to their AI prompts (v7.2.0)',
  },
  [RelationType.HAS_RULES]: {
    type: RelationType.HAS_RULES,
    from: 'BlockType',
    to: 'BlockRules',
    cardinality: '1:N',
    description: 'Links BlockType to generation rules (v7.2.0)',
  },
  [RelationType.GENERATED]: {
    type: RelationType.GENERATED,
    from: ['PagePrompt', 'BlockPrompt'],
    to: ['PageGenerated', 'BlockGenerated'],
    cardinality: 'N:M',
    props: GeneratedPropsSchema,
    description: 'Provenance: which prompt generated which output (v7.2.0)',
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
  // ─────────────────────────────────────────────────────────────────────────────
  [RelationType.L10N_OF]: {
    type: RelationType.L10N_OF,
    from: ['EntityContent', 'ProjectContent'],
    to: ['Entity', 'Project'],
    cardinality: 'N:1',
    description: 'Inverse of HAS_CONTENT - localized content points to parent',
  },
  [RelationType.OUTPUT_OF]: {
    type: RelationType.OUTPUT_OF,
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
  [RelationType.HAS_LOCALIZED_CONTENT]: {
    type: RelationType.HAS_LOCALIZED_CONTENT,
    from: 'Locale',
    to: ['ProjectContent', 'EntityContent', 'PageGenerated', 'BlockGenerated', 'SEOKeyword', 'GEOQuery'],
    cardinality: '1:N',
    description: 'Inverse of FOR_LOCALE - locale knows all its content',
  },
};