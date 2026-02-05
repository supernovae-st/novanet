// NovaNet Core Types v9.0.0 - Nodable Project Architecture
//
// v8.2.0 CHANGES (YAML v7.11.0 alignment):
//   - REMOVED: icon, priority, freshness from all interfaces
//   - REMOVED: Priority, Freshness type definitions
//   - UPDATED: StandardNodeProperties (6 props instead of 9)
//   - NOTE: Use NODE_ICONS config for icon display instead
//
// v8.0.0 CHANGES:
//   - ADDED: anchor_type property to LinksToProps (exact_match | partial_match | branded | generic)
//   - ADDED: nofollow property to LinksToProps (boolean, default false)
//   - ADDED: AnchorType type for anchor text optimization
//
// v7.12.0 CHANGES:
//   - ADDED: LINKS_TO (Page → Page) for explicit internal linking with concept-based anchors
//   - ADDED: SUBTOPIC_OF (Page → Page) for pillar-cluster SEO hierarchy
//   - UPDATED: PageTypeCategory to include 'pillar' | 'cluster'
//   - ADDED: LinksToProps, PageTypeSeoCategory
//
// v7.11.0 CHANGES:
//   - ADDED: version, published_at, replaced_at to BlockL10n, PageL10n (versioning)
//   - ADDED: PREVIOUS_VERSION relation for L10n history chains
//   - REMOVED: PageMetrics (query GA/PostHog with date ranges instead)
//
// v7.10.0 CHANGES:
//   - ADDED: PageType (mirrors BlockType pattern: Page -[:OF_TYPE]-> PageType)
//   - REMOVED: Page.page_type enum (replaced by PageType node)
//   - REMOVED: Page.generation_priority (redundant with priority)
//
// v7.9.0 CHANGES:
//   - FOLDER: Scope-based structure (global/shared/project)
//   - REMOVED: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at ConceptL10n level)
//   - ADDED: JSON Schema validation (models/schema/nodes.schema.json)
//
// v7.8.5 CHANGES:
//   - RENAMED: SEOSnapshot → SEOKeywordMetrics (unified metrics pattern)
//   - RENAMED: GEOCitation → GEOSeedMetrics (unified metrics pattern)
//   - UNIFIED: HAS_METRICS relation for all time-series observations
//   - ADDED: observed_at standardized timestamp for all metrics nodes
//
// v7.8.0 CHANGES:
//   - ADDED: EmbeddableNode interface for vector search support
//   - UPDATED: Concept, ConceptL10n, Page to include embedding properties
//
// v8.2.0 STANDARD PROPERTIES (all nodes):
//   key: string           - Unique identifier (with semantic prefix)
//   display_name: string  - Human-readable name
//   description: string   - Short description
//   llm_context: string   - "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
//   created_at: datetime
//   updated_at: datetime

// =============================================================================
// NODE TYPES + v9 TAXONOMY (Realm, Layer, Trait, KIND_META)
// =============================================================================

export {
  NODE_TYPES,
  type NodeType,
  type Realm,
  type Layer,
  type Trait,
  type KindMeta,
  KIND_META,
  NODE_REALMS,
  NODE_TRAITS,
} from './nodes.js';

// =============================================================================
// CONTEXT MANAGEMENT TYPES (v7.1.0)
// =============================================================================

// REMOVED v8.2.0: Priority and Freshness types (YAML v7.11.0 alignment)
// export type { Priority, Freshness } from './locale-knowledge.js';

// =============================================================================
// STANDARD PROPERTIES BASE
// =============================================================================

// REMOVED v8.2.0: Priority and Freshness types no longer needed
// import type { Priority, Freshness } from './locale-knowledge.js';

/**
 * Standard properties for all nodes (v8.2.0 - YAML v7.11.0 aligned)
 * REMOVED: icon, priority, freshness (presentation layer / YAGNI)
 */
export interface StandardNodeProperties {
  key: string;
  display_name: string;
  description: string;
  llm_context: string;
  created_at: Date;
  updated_at: Date;
}

// =============================================================================
// EMBEDDABLE NODE (v7.8.0 - Hybrid OntologyRAG)
// =============================================================================

/**
 * Base interface for nodes that support vector embeddings.
 * Used by Concept, ConceptL10n, and Page for semantic search.
 */
export interface EmbeddableNode {
  /** OpenAI text-embedding-3-small vector (1536 dimensions) */
  embedding?: number[];
  /** Text used to generate the embedding (for debugging) */
  embedding_source?: string;
  /** Timestamp of last embedding update */
  embedding_updated_at?: Date;
}

// =============================================================================
// LOCALE (v7.1.0) - Re-exported from locale-knowledge.ts
// =============================================================================

// Locale is the primary definition in locale-knowledge.ts (source of truth)
// Re-exported below in LOCALE KNOWLEDGE section

// =============================================================================
// PROJECT + NODABLE ARCHITECTURE (v7.1.0)
// =============================================================================

export {
  type Project,
  type ProjectL10n,
  type VoiceTone,
  // v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n removed
  type BrandIdentity,
  type ColorPaletteItem,
  type TypographyScaleItem,
  type ProjectNode,
} from './project.js';

// =============================================================================
// ENTITY (v10.3 - replaces Concept, global realm, knowledge layer)
// =============================================================================

export interface Entity extends StandardNodeProperties, EmbeddableNode {
  // key: "action-create-qr" (semantic prefix + identifier)
  // display_name: "Create QR Code"
  // description: "Core knowledge entity"
  // llm_context: "USE: when creating QR codes. TRIGGERS: create, generate. NOT: editing."

  // Entity-specific properties (same as former Concept)
  feature_category?: 'core' | 'analytics' | 'design' | 'integration' | 'api';
  feature_priority?: number;
  is_core?: boolean;
  is_premium?: boolean;
  search_intent?: 'transactional' | 'informational' | 'navigational';
}

export interface EntityL10n extends EmbeddableNode {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // EntityL10n-specific (same as former ConceptL10n)
  title: string;
  definition: string;
  purpose?: string;
  benefits?: string[];
  use_cases?: string[];
  version: number;
  influence_count: number;
  created_at: Date;
  updated_at: Date;
}

// v10.3 backwards compatibility aliases (deprecated, will be removed)
/** @deprecated Use Entity instead */
export type Concept = Entity;
/** @deprecated Use EntityL10n instead */
export type ConceptL10n = EntityL10n;

// =============================================================================
// PAGE (v7.1.0)
// =============================================================================

export interface Page extends StandardNodeProperties, EmbeddableNode {
  // key: "page-pricing" (v7.1.0 prefix convention)
  // display_name: "Pricing Page"
  // icon: "📄"
  // description: "Main pricing page"
  // llm_context: "USE: orchestrate pricing page. TRIGGERS: pricing, tarifs. NOT: individual blocks."
}

/**
 * PageType categories for organization and SEO hierarchy (v7.12.0)
 * - marketing, product, content, legal, support: traditional categories
 * - pillar: comprehensive guide page that links to cluster pages
 * - cluster: focused subtopic page that links to its pillar via :SUBTOPIC_OF
 */
export type PageTypeCategory = 'marketing' | 'product' | 'content' | 'legal' | 'support' | 'pillar' | 'cluster';

/**
 * PageType - Template defining meta requirements, layout rules, and block composition (v7.10.0)
 * Mirrors BlockType pattern: Page -[:OF_TYPE]-> PageType (like Block -[:OF_TYPE]-> BlockType)
 */
export interface PageType extends StandardNodeProperties {
  // key: "pagetype-landing" (v7.10.0 prefix convention)
  // display_name: "Landing Page"
  // icon: "📐"
  // description: "High-conversion landing page template"
  // llm_context: "USE: orchestrate landing page. TRIGGERS: landing, conversion. NOT: blog posts."

  /** Page category for organization and SEO hierarchy (v7.12.0: added pillar/cluster) */
  category: PageTypeCategory;

  /** JSON schema defining required meta/SEO fields */
  meta_schema?: Record<string, unknown>;

  /** Constraints on block organization and positioning */
  layout_rules?: Record<string, unknown>;

  /** BlockType keys that MUST appear on pages of this type */
  required_block_types?: string[];

  /** BlockType keys recommended but not required */
  optional_block_types?: string[];

  /** Custom validation rules for page generation */
  validation_rules?: Record<string, unknown>;
}

export interface PageL10n {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // PageL10n-specific (v7.6.0: renamed from PageOutput)
  assembled: Record<string, unknown>;
  assembled_at: Date;
  assembler_version: string;
  created_at: Date;
  updated_at: Date;

  // Versioning (v7.11.0)
  /** Version number (1, 2, 3...) */
  version: number;
  /** When this version was put live */
  published_at?: Date;
  /** When this version was replaced (null = current) */
  replaced_at?: Date;
}

// =============================================================================
// BLOCK (v7.1.0)
// =============================================================================

export interface BlockType extends StandardNodeProperties {
  // key: "blocktype-hero" (v7.1.0 prefix convention)
  // display_name: "Hero Block"
  // icon: "🎯"
  // description: "Hero section template"
  // llm_context: "USE: generate hero sections. TRIGGERS: hero, header. NOT: body content."
  category: string;
  structure?: string;
}

/**
 * Block node - a content unit within a Page.
 * key: "block-pricing-hero" (v7.1.0 prefix convention)
 * Note: Block-specific instructions are in BlockPrompt nodes (v8.0.0)
 */
export type Block = StandardNodeProperties;

export interface BlockL10n {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // BlockL10n-specific (v7.6.0: renamed from BlockOutput)
  generated: Record<string, unknown>;
  generated_at: Date;
  generator_version: string;
  created_at: Date;
  updated_at: Date;

  // Versioning (v7.11.0)
  /** Version number (1, 2, 3...) */
  version: number;
  /** When this version was put live */
  published_at?: Date;
  /** When this version was replaced (null = current) */
  replaced_at?: Date;
}

// =============================================================================
// LOCALE KNOWLEDGE (v7.1.0 - replaces deprecated L10NCategory/L10NContent)
// =============================================================================

export {
  type Locale,               // Primary Locale definition (v7.1.0)
  type LocaleIdentity,
  type LocaleVoice,
  type LocaleCulture,
  type LocaleMarket,
  type LocaleLexicon,
  type Expression,
  type SemanticField,
  type LocaleRulesAdaptation,
  type LocaleRulesFormatting,
  type LocaleRulesSlug,
  type LocaleCultureReferences,
  type LocaleKnowledgeNode,
} from './locale-knowledge.js';

// =============================================================================
// SEO (v7.1.0, v7.8.2: SEOKeyword → SEOKeyword)
// =============================================================================

export interface SEOKeyword extends StandardNodeProperties {
  // key: "creer-qr-code-gratuit-fr"
  // display_name: "créer qr code gratuit"
  // icon: "🔍"
  // description: "Primary SEO keyword for France"
  // llm_context: "High-volume transactional keyword..."
  value: string;
  volume: number;
  difficulty: number;
  cpc: number;
  intent: string;
  platform: string;
  source: string;
}

/**
 * SEOKeywordMetrics - Historical metrics snapshot for an SEO keyword (v7.8.5)
 * Unified metrics pattern: SEOKeyword -[:HAS_METRICS]-> SEOKeywordMetrics
 */
export interface SEOKeywordMetrics {
  // Standard properties (v8.2.0 - no icon/priority/freshness)
  key: string;             // "seometrics-creer-qr-code-fr-2024-01-15"
  display_name: string;    // "créer qr code - 2024-01-15"
  description: string;     // "Metrics snapshot for tracking"
  llm_context: string;     // Not used in spreading activation

  // SEOKeywordMetrics-specific (v7.8.5)
  observed_at: Date;       // When these metrics were observed
  volume?: number;         // Monthly search volume
  difficulty?: number;     // Keyword difficulty (0-100)
  cpc?: number;            // Cost per click
  position?: number;       // Our ranking position
  url?: string;            // Our ranking URL
  source: string;          // Data source: semrush | ahrefs | google_search_console
  created_at: Date;
  updated_at: Date;
}

export interface SEOMiningRun {
  // Standard properties (v8.2.0 - no key, UUID-identified, no icon/priority/freshness)
  display_name: string;    // "SEO Mining 2024-01-15 10:00"
  description: string;     // "Mining run for keyword variations"
  llm_context: string;     // "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."

  // SEOMiningRun-specific
  status: 'running' | 'completed' | 'failed';
  sources: string[];
  started_at: Date;
  completed_at?: Date;
  total_variations: number;
  unique_variations: number;
  created_at: Date;
  updated_at: Date;
}

// =============================================================================
// GEO — REMOVED in v10.3
// =============================================================================
// GEOSeedL10n, GEOSeedMetrics, GEOMiningRun removed (GEO layer deprecated)
// Will be reintroduced in future version with different architecture

// =============================================================================
// RELATION PROPS
// =============================================================================

export interface SupportsLocaleProps {
  default: boolean;
}

export interface HasBlockProps {
  position: number;
}

export interface SemanticLinkProps {
  // 10 semantic link types (v7.1.0)
  // Verb pairs: is_action_on/has_action
  // Container pairs: includes/included_in
  // Taxonomy pairs: type_of/has_type
  // Dependency pairs: requires/required_by
  // Generic/contrast: related, opposite
  type:
    | 'is_action_on' | 'has_action'
    | 'includes' | 'included_in'
    | 'type_of' | 'has_type'
    | 'requires' | 'required_by'
    | 'related' | 'opposite';
  temperature: number;
}

// v10.3: UsesEntityProps replaces UsesConceptProps
export interface UsesEntityProps {
  purpose: 'primary' | 'secondary' | 'contextual';
  temperature: number;
}

/** @deprecated Use UsesEntityProps instead */
export type UsesConceptProps = UsesEntityProps;

// v10.3: ExpressesProps replaces TargetsSEOProps (EntityL10n -[:EXPRESSES]-> SEOKeyword)
export interface ExpressesProps {
  status: 'active' | 'paused' | 'archived';
  priority: number;
}

/** @deprecated Use ExpressesProps instead */
export type TargetsSEOProps = ExpressesProps;

// REMOVED v10.3: TargetsGEOProps (GEO layer removed)

export interface InfluencedByProps {
  weight: number;
  entity_version: number;  // v10.3: renamed from concept_version
}

// ─────────────────────────────────────────────────────────────────────────────
// PAGE RELATIONSHIPS PROPS (v7.12.0, extended v8.0.0)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Anchor text optimization strategy (v8.0.0)
 * - exact_match: anchor = ConceptL10n.title exactly (5× traffic, use sparingly max 10%)
 * - partial_match: anchor includes concept keywords
 * - branded: anchor = brand name (QR Code AI)
 * - generic: anchor = "click here", "learn more" (low SEO value)
 */
export type AnchorType = 'exact_match' | 'partial_match' | 'branded' | 'generic';

/**
 * LinksToProps - Properties for Page-to-Page internal links (v7.12.0, extended v8.0.0)
 * Anchor text is derived from ConceptL10n.title at generation time.
 */
export interface LinksToProps {
  /** Concept key - anchor text derived from ConceptL10n.title for the target locale */
  concept_key: string;
  /** Where the link appears in the content */
  context: 'cta' | 'body' | 'related' | 'nav';
  /** Link importance for SEO (0.0-1.0) */
  seo_weight: number;
  /** Anchor text strategy (v8.0.0). exact_match = 5× traffic but use sparingly (max 10%) */
  anchor_type: AnchorType;
  /** Set true for login/legal pages to prevent link equity flow (v8.0.0) */
  nofollow: boolean;
}

// =============================================================================
// PROMPTS (v7.2.0)
// =============================================================================

export {
  type PagePrompt,
  type BlockPrompt,
  type BlockRules,
  type PromptNode,
} from './prompts.js';

// =============================================================================
// TASK TYPES (v7.8.0 - Hybrid OntologyRAG)
// =============================================================================

export {
  type TaskType,
  type SemanticType,
  type SemanticBoosts,
  type Priority as TaskPriority,
  type TaskModifier,
  type GenerationContext,
  type ActivatedConcept,
  type LocaleContext,
  type RetrievalRequest,
  type RetrievalResult,
  TASK_TYPES,
  PRIORITIES,
  TASK_MODIFIERS,
} from './task-types.js';
