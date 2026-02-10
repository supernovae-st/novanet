// NovaNet Core Types v11.2.0 - 2-Realm Architecture
//
// v11.2.0: 2 realms (shared, org), derived → generated + aggregated, job nodes removed
//
// STANDARD PROPERTIES (all nodes):
//   key: string           - Unique identifier (with semantic prefix)
//   display_name: string  - Human-readable name
//   description: string   - Short description
//   llm_context: string   - "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
//   created_at: datetime
//   updated_at: datetime

// =============================================================================
// NODE TYPES + TAXONOMY (Realm, Layer, Trait, KIND_META)
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
 * Used by Entity, EntityContent, and Page for semantic search.
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
  type ProjectContent,
  type VoiceTone,
  // v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n removed
  type BrandIdentity,
  type ColorPaletteItem,
  type TypographyScaleItem,
  type ProjectNode,
} from './project.js';

// =============================================================================
// ENTITY (v11.2 - org realm, semantic layer, invariant trait)
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

export interface EntityContent extends EmbeddableNode {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // EntityContent-specific (same as former EntityContent)
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

export interface PageGenerated {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // PageGenerated-specific (v7.6.0: renamed from PageOutput)
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

export interface BlockGenerated {
  // Standard properties (v8.2.0 - L10n nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // BlockGenerated-specific (v7.6.0: renamed from BlockOutput)
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

// v11.2: SEOMiningRun removed (job concept deferred to v12+)

// =============================================================================
// GEO — REINTRODUCED in v10.7
// =============================================================================
// v10.3: GEOSeedL10n, GEOSeedMetrics, GEOMiningRun removed (old GEO layer deprecated)
// v10.7: New GEO schema: GEOQuery, GEOAnswer, GEOMetrics (AI visibility tracking)

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

// v10.3: UsesEntityProps (Page/Block -[:USES_ENTITY]-> Entity)
export interface UsesEntityProps {
  purpose: 'primary' | 'secondary' | 'contextual';
  temperature: number;
}

export interface InfluencedByProps {
  weight: number;
  entity_version: number;  // v10.3: renamed from concept_version
}

// ─────────────────────────────────────────────────────────────────────────────
// PAGE RELATIONSHIPS PROPS (v7.12.0, extended v8.0.0)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Anchor text optimization strategy (v8.0.0)
 * - exact_match: anchor = EntityContent.title exactly (5× traffic, use sparingly max 10%)
 * - partial_match: anchor includes concept keywords
 * - branded: anchor = brand name (QR Code AI)
 * - generic: anchor = "click here", "learn more" (low SEO value)
 */
export type AnchorType = 'exact_match' | 'partial_match' | 'branded' | 'generic';

/**
 * LinksToProps - Properties for Page-to-Page internal links (v7.12.0, extended v8.0.0)
 * Anchor text is derived from EntityContent.title at generation time.
 */
export interface LinksToProps {
  /** Concept key - anchor text derived from EntityContent.title for the target locale */
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
  TASK_TYPES,
  PRIORITIES,
  TASK_MODIFIERS,
} from './task-types.js';
