// NovaNet Core Types v0.18.0 - *Native Pattern
//
// v0.18.0: Added llm_context to all LocaleRules* and LocaleCultureReferences interfaces
// v0.13.0: ADR-029 *Native pattern (EntityNative→EntityNative, ProjectNative→ProjectNative, PageNative→PageNative, BlockNative→BlockNative)
// v11.8.0: ADR-023 terminology (Kind→Class, ArcKind→ArcClass), ADR-024 trait renames (defined/authored/imported/generated/retrieved)
// v11.7.0: Unified tree where Realm, Layer, Class, Instance, ArcFamily, ArcClass are all clickable nodes
// v11.2.0: 2 realms (shared, org), derived split into generated + retrieved
//
// STANDARD PROPERTIES (all nodes):
//   key: string           - Unique identifier (with semantic prefix)
//   display_name: string  - Human-readable name
//   description: string   - Short description
//   llm_context: string   - "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
//   created_at: datetime
//   updated_at: datetime

// =============================================================================
// NODE TYPES + TAXONOMY (Realm, Layer, Trait, CLASS_TAXONOMY)
// =============================================================================

export {
  NODE_TYPES,
  type NodeType,
  type Realm,
  type Layer,
  type Trait,
  type Classification,
  CLASS_TAXONOMY,
  NODE_REALMS,
  NODE_TRAITS,
} from './nodes.js';

// =============================================================================
// STANDARD PROPERTIES BASE
// =============================================================================

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
 * Used by Entity, EntityNative, and Page for semantic search.
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
// LOCALE (v7.1.0) - Re-exported from shared.ts
// =============================================================================

// Locale is the primary definition in shared.ts (source of truth)
// Re-exported below in SHARED section

// =============================================================================
// PROJECT + NODABLE ARCHITECTURE (v7.1.0)
// =============================================================================

export {
  type Project,
  type ProjectNative,
  type VoiceTone,
  // v7.2.5: Audience, AudienceL10n, ValuePropL10n, SocialProofL10n removed
  // v0.12.4: BrandIdentity removed (use Brand) ADR-028
  type ColorPaletteItem,
  type TypographyScaleItem,
  type ProjectNode,
} from './project.js';

// =============================================================================
// ENTITY (v0.13.0 - org realm, semantic layer, defined trait)
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

/**
 * EntityNative - Entity content per locale (v0.13.0 ADR-029).
 * Locale-specific content for an Entity node.
 */
export interface EntityNative extends EmbeddableNode {
  // Standard properties (v8.2.0 - Content nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // EntityNative-specific properties
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
// PAGE (v0.13.0 - *Native pattern)
// =============================================================================

export interface Page extends StandardNodeProperties, EmbeddableNode {
  // key: "page-pricing" (v7.1.0 prefix convention)
  // display_name: "Pricing Page"
  // icon: "📄"
  // description: "Main pricing page"
  // llm_context: "USE: orchestrate pricing page. TRIGGERS: pricing, tarifs. NOT: individual blocks."
}

// v0.12.4: PageStructureCategory and PageStructure removed (ADR-028)
// Page structure is now computed from HAS_BLOCK.order at runtime

/**
 * PageNative - Page output per locale (v0.13.0 ADR-029).
 * LLM-generated page content for a specific locale.
 */
export interface PageNative {
  // Standard properties (v8.2.0 - Content nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // PageNative-specific properties (v0.13.0: renamed from PageNative)
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
// BLOCK (v0.13.0 - *Native pattern)
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
 * Note: Block-specific instructions are in BlockInstruction nodes
 */
export type Block = StandardNodeProperties;

/**
 * BlockNative - Block output per locale (v0.13.0 ADR-029).
 * LLM-generated block content for a specific locale.
 */
export interface BlockNative {
  // Standard properties (v8.2.0 - Content nodes don't have key, no icon/priority/freshness)
  display_name: string;
  description: string;
  llm_context: string;

  // BlockNative-specific properties (v0.13.0: renamed from BlockNative)
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
  // v0.18.0: LocaleMarket removed (market data from external APIs, not static graph)
  type LocaleLexicon,
  type Expression,
  type LocaleRulesAdaptation,
  type LocaleRulesFormatting,
  type LocaleRulesSlug,
  type LocaleCultureReferences,
  type LocaleKnowledgeNode,
} from './shared.js';

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

// v0.17.3: SEOKeywordMetrics removed (YAGNI)
// Metrics (volume, difficulty, cpc, position) are now stored directly on SEOKeyword node.
// See ADR-024 for trait semantics: SEOKeyword.trait = 'imported'

// v11.2: SEOMiningRun removed (job concept deferred to v12+)

// =============================================================================
// GEO — REINTRODUCED in v10.7
// =============================================================================
// v10.3: GEOSeedL10n, GEOSeedMetrics, GEOMiningRun removed (old GEO layer deprecated)
// v10.7: New GEO schema: GEOQuery, GEOAnswer (AI visibility tracking)
// Note: GEOAnswer serves as time-series storage; metrics calculated on-demand

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
 * - exact_match: anchor = EntityNative.title exactly (5× traffic, use sparingly max 10%)
 * - partial_match: anchor includes concept keywords
 * - branded: anchor = brand name (QR Code AI)
 * - generic: anchor = "click here", "learn more" (low SEO value)
 */
export type AnchorType = 'exact_match' | 'partial_match' | 'branded' | 'generic';

/**
 * LinksToProps - Properties for Page-to-Page internal links (v7.12.0, extended v8.0.0)
 * Anchor text is derived from EntityNative.title at generation time.
 */
export interface LinksToProps {
  /** Concept key - anchor text derived from EntityNative.title for the target locale */
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
// INSTRUCTIONS (v0.12.4: PageInstruction removed per ADR-028)
// =============================================================================

export {
  // v0.12.4: PageInstruction removed - instructions composed from BlockInstructions
  type BlockInstruction,
  type BlockRules,
  type InstructionNode,
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

// =============================================================================
// UNIFIED TREE (v11.7.0 - Unified Tree Architecture)
// =============================================================================

export {
  // Dual icon format
  type DualIcon,

  // Node types
  type UnifiedNodeType,
  type SectionType,
  type UnifiedNodeBase,
  type SectionNode,
  type RealmNode,
  type LayerNode,
  type ClassNode,
  type InstanceNode,
  type ArcFamilyNode,
  type ArcClassNode,
  type UnifiedNode,

  // Classification types
  type NodeTrait,
  type ArcCardinality,

  // Badge types
  type NodeBadge,
  BADGES,

  // Lazy loading types
  type LazyChildrenState,
  PAGINATION,

  // Tree store types
  type UnifiedTreeState,
  type UnifiedTreeActions,

  // API types
  type LoadInstancesRequest,
  type LoadInstancesResponse,
  type NodeDetailsResponse,

  // View types (unified tree specific)
  type UnifiedViewDefinition,
  type UnifiedViewRegistry,

  // Type guards
  isSectionNode,
  isRealmNode,
  isLayerNode,
  isClassNode,
  isInstanceNode,
  isArcFamilyNode,
  isArcClassNode,

  // Utility functions
  makeNodeId,
  parseNodeId,
  getLayerBadge,
  getRealmBadge,
} from './unified-tree.js';
