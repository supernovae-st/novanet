// =============================================================================
// NOVANET VISUALIZER TYPES
// =============================================================================
// Re-export from novanet-core + visualization-specific types

// -----------------------------------------------------------------------------
// RE-EXPORT FROM NOVANET-CORE (Single Source of Truth)
// -----------------------------------------------------------------------------

// To import from novanet-core (requires build):
// export {
//   StandardNodeProperties,
//   Concept,
//   ConceptL10n,
//   Page,
//   PageOutput,
//   Block,
//   BlockType,
//   BlockOutput,
//   Locale,
//   LocaleIdentity,
//   LocaleVoice,
//   LocaleCulture,
//   LocaleMarket,
//   LocaleLexicon,
//   Expression,
//   Project,
// } from '@novanet-core/types';

// Local definitions (aligned with novanet-core v7.2.4)

/**
 * Standard properties for all v7.2.4 nodes
 */
export interface StandardNodeProperties {
  key: string;
  display_name: string;
  icon: string;
  description: string;
  llm_context: string;
  // Context management (v7.1.0)
  priority: 'critical' | 'high' | 'medium' | 'low';
  freshness: 'realtime' | 'hourly' | 'daily' | 'static';
  created_at: Date;
  updated_at: Date;
}

/**
 * All node types in the NovaNet knowledge graph (v7.2.5)
 * Aligned with novanet-core/models/_index.yaml
 *
 * Categories (7):
 * - project (📦): Business definition + brand L10n
 * - content (💡): Semantic content structure
 * - locale (🌍): Locale + knowledge nodes
 * - generation (🤖): AI prompts + output
 * - seo (🔍): SEO keywords + mining
 * - geo (🎯): GEO seeds + mining
 * - analytics (📊): External metrics
 */
export type NodeType =
  // ==========================================================================
  // CATEGORY: PROJECT (📦) - Business definition (3 nodes)
  // Note: Audience merged into ProjectL10n.target_audience (v7.2.5)
  // Note: ValuePropL10n + SocialProofL10n removed (v7.2.5)
  // ==========================================================================
  | 'Project'
  | 'BrandIdentity'
  | 'ProjectL10n'
  // ==========================================================================
  // CATEGORY: CONTENT (💡) - Semantic structure (5 nodes)
  // ==========================================================================
  | 'Concept'
  | 'ConceptL10n'
  | 'Page'
  | 'Block'
  | 'BlockType'
  // ==========================================================================
  // CATEGORY: LOCALE (🌍) - Locale + knowledge (7 nodes)
  // ==========================================================================
  | 'Locale'
  | 'LocaleIdentity'
  | 'LocaleVoice'
  | 'LocaleCulture'
  | 'LocaleMarket'
  | 'LocaleLexicon'
  | 'Expression'
  // ==========================================================================
  // CATEGORY: GENERATION (🤖) - Prompts + output (5 nodes)
  // ==========================================================================
  | 'PagePrompt'
  | 'BlockPrompt'
  | 'BlockRules'
  | 'PageOutput'
  | 'BlockOutput'
  // ==========================================================================
  // CATEGORY: SEO (🔍) - Keywords + mining (4 nodes)
  // ==========================================================================
  | 'SEOKeyword'
  | 'SEOVariation'
  | 'SEOSnapshot'
  | 'SEOMiningRun'
  // ==========================================================================
  // CATEGORY: GEO (🎯) - Seeds + mining (4 nodes)
  // ==========================================================================
  | 'GEOSeed'
  | 'GEOReformulation'
  | 'GEOCitation'
  | 'GEOMiningRun'
  // ==========================================================================
  // CATEGORY: ANALYTICS (📊) - External metrics (1 node)
  // ==========================================================================
  | 'PageMetrics';

/**
 * Active relationship types (v7.1.0)
 * Aligned with novanet-core/models/relations.yaml + Neo4j db.relationshipTypes()
 */
export type RelationType =
  // Project relations
  | 'HAS_CONCEPT'
  | 'HAS_PAGE'
  | 'SUPPORTS_LOCALE'
  | 'HAS_AUDIENCE'
  | 'HAS_BRAND_IDENTITY'
  | 'HAS_L10N'
  // Page/Block relations
  | 'HAS_BLOCK'
  | 'OF_TYPE'
  | 'USES_CONCEPT'
  | 'HAS_OUTPUT'
  | 'HAS_METRICS'
  | 'ASSEMBLES'
  // ProjectL10n sub-relations
  | 'HAS_VALUE_PROP'
  | 'HAS_SOCIAL_PROOF'
  | 'HAS_VOICE_EXAMPLE'
  // Concept relations
  | 'SEMANTIC_LINK'
  | 'FOR_LOCALE'
  | 'INFLUENCED_BY'
  | 'TARGETS_SEO'
  | 'TARGETS_GEO'
  // Page-level SEO/GEO targeting (in Neo4j)
  | 'PAGE_TARGETS_SEO'
  | 'PAGE_TARGETS_GEO'
  // Locale Knowledge relations
  | 'HAS_IDENTITY'
  | 'HAS_VOICE'
  | 'HAS_CULTURE'
  | 'HAS_MARKET'
  | 'HAS_LEXICON'
  | 'FALLBACK_TO'
  // Lexicon relations
  | 'HAS_EXPRESSION'
  // Output provenance (in Neo4j)
  | 'USED_SEO_KEYWORD'
  | 'USED_GEO_SEED'
  // SEO relations
  | 'HAS_VARIATION'
  | 'HAS_SNAPSHOT'
  | 'MINED_BY'
  // GEO relations
  | 'HAS_REFORMULATION'
  | 'HAS_CITATION';

// -----------------------------------------------------------------------------
// VISUALIZATION TYPES
// -----------------------------------------------------------------------------

/**
 * Normalized node for visualization (v7.1.0)
 * Aligned with neo4j.ts transformNode output
 */
export interface GraphNode {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  icon?: string;
  description?: string;
  llmContext?: string;
  // Context management (v7.1.0)
  priority?: 'critical' | 'high' | 'medium' | 'low';
  freshness?: 'realtime' | 'hourly' | 'daily' | 'static';
  createdAt?: string;
  updatedAt?: string;
  /** Additional properties not in standard fields */
  data?: Record<string, unknown>;
}

/**
 * Normalized edge for visualization
 * Aligned with neo4j.ts transformRelationship output
 */
export interface GraphEdge {
  id: string;
  type: RelationType;
  source: string;
  target: string;
  /** Additional relationship properties */
  data?: Record<string, unknown>;
}

/**
 * Graph data for rendering
 */
export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

/**
 * Node detail with relations
 */
export interface NodeDetail {
  node: GraphNode;
  relations: {
    incoming: Array<{ type: RelationType; node: GraphNode }>;
    outgoing: Array<{ type: RelationType; node: GraphNode }>;
  };
}

// -----------------------------------------------------------------------------
// NEO4J RAW TYPES
// -----------------------------------------------------------------------------

export interface Neo4jNode {
  identity: number;
  labels: string[];
  properties: Record<string, unknown>;
  elementId: string;
}

export interface Neo4jRelationship {
  identity: number;
  type: string;
  start: number;
  end: number;
  startNodeElementId: string;
  endNodeElementId: string;
  properties: Record<string, unknown>;
  elementId: string;
}

// -----------------------------------------------------------------------------
// FILTER & PRESET TYPES
// -----------------------------------------------------------------------------

export interface FilterPreset {
  id: string;
  name: string;
  description: string;
  icon: string;
  shortcut: string;
  nodeTypes: NodeType[];
  locale: string | null;
  isCustom?: boolean;
}

// -----------------------------------------------------------------------------
// UI TYPES
// -----------------------------------------------------------------------------

export type ViewMode = '2d' | '3d';

export interface UIState {
  viewMode: ViewMode;
  sidebarOpen: boolean;
  panelOpen: boolean;
  searchOpen: boolean;
  focusMode: boolean;
}

export interface SelectionState {
  selectedNodeId: string | null;
  hoveredNodeId: string | null;
  highlightedNodeIds: Set<string>;
}

// -----------------------------------------------------------------------------
// AI CHAT TYPES
// -----------------------------------------------------------------------------

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
  metadata?: {
    cypherQuery?: string;
    nodeCount?: number;
    duration?: number;
  };
}

export interface ChatState {
  messages: ChatMessage[];
  isLoading: boolean;
  error: string | null;
}

// -----------------------------------------------------------------------------
// AI QUERY TYPES
// -----------------------------------------------------------------------------

/**
 * A query executed via AI (stored in history)
 * Note: Uses ISO string for createdAt (localStorage serialization)
 */
export interface AiQuery {
  id: string;
  /** Original natural language question */
  question: string;
  /** Generated Cypher query */
  cypher: string;
  /** Execution result */
  result?: {
    nodeCount: number;
    edgeCount: number;
    duration: number;
  };
  /** Execution status */
  status: 'pending' | 'generated' | 'executing' | 'success' | 'error';
  /** Error message if failed */
  error?: string;
  /** Timestamp (ISO string for localStorage) */
  createdAt: string;
}

/**
 * A user-saved Cypher query (persisted to localStorage)
 * Note: Uses ISO strings for dates (localStorage serialization)
 */
export interface SavedQuery {
  id: string;
  /** Display name */
  name: string;
  /** Optional description */
  description?: string;
  /** Emoji icon */
  icon: string;
  /** Cypher query */
  cypher: string;
  /** Creation timestamp (ISO string for localStorage) */
  createdAt: string;
  /** Last modified timestamp (ISO string for localStorage) */
  updatedAt: string;
}
