// =============================================================================
// NOVANET STUDIO TYPES
// =============================================================================
// Re-export from @novanet/core (Single Source of Truth) + Studio-specific types

// -----------------------------------------------------------------------------
// RE-EXPORT FROM @NOVANET/CORE (v9.0.0)
// -----------------------------------------------------------------------------

// Node types - Single Source of Truth
export {
  NODE_TYPES,
  type NodeType,
  type Realm,
  NODE_REALMS,
  type Trait,
  NODE_TRAITS,
  type Layer,
  type KindMeta,
  KIND_META,
} from '@novanet/core/types';

// Standard properties and domain types (v10.4: Entity-Centric, no GEO layer)
export type {
  StandardNodeProperties,
  // Nodes
  Project,
  ProjectL10n,
  BrandIdentity,
  Entity,
  EntityContent,
  Page,
  PageType,
  PageGenerated,
  Block,
  BlockType,
  BlockGenerated,
  Locale,
  LocaleIdentity,
  LocaleVoice,
  LocaleCulture,
  LocaleMarket,
  LocaleLexicon,
  Expression,
  SEOKeyword,
  SEOKeywordMetrics,
  SEOMiningRun,
  // Prompts
  PagePrompt,
  BlockPrompt,
  BlockRules,
  // Relation props
  SemanticLinkProps,
  UsesEntityProps,
  HasBlockProps,
  LinksToProps,
} from '@novanet/core/types';

// Relation types from schema
export { RelationType } from '@novanet/core/schemas';

// -----------------------------------------------------------------------------
// STUDIO-SPECIFIC TYPES
// -----------------------------------------------------------------------------

import type { NodeType } from '@novanet/core/types';

/**
 * Normalized node for visualization
 * Aligned with neo4j.ts transformNode output
 * v9.0.0: Updated to use Realm, Layer, Trait terminology
 */
export interface GraphNode {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  description?: string;
  llmContext?: string;
  createdAt?: string;
  updatedAt?: string;
  /** Additional properties not in standard fields */
  data?: Record<string, unknown>;
}

/**
 * Schema mode synthetic group types (containers in schema visualization)
 */
export type SchemaGroupType = 'RealmGroup' | 'LayerGroup';

/**
 * Synthetic node info for schema mode containers (realm groups, layers)
 * Used for hover tooltips in schema visualization
 */
export interface SchemaGroupNode {
  id: string;
  type: SchemaGroupType;
  key: string;
  displayName: string;
}

/**
 * Union type for hover info - can be real GraphNode or synthetic schema group
 */
export type HoverNodeInfo = GraphNode | SchemaGroupNode;

/**
 * Normalized edge for visualization
 * Aligned with neo4j.ts transformRelationship output
 */
export interface GraphEdge {
  id: string;
  type: string;  // RelationType from core
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
    incoming: Array<{ type: string; node: GraphNode }>;
    outgoing: Array<{ type: string; node: GraphNode }>;
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
