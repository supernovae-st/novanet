/**
 * Unified Tree Types for v11.7
 *
 * This module defines the core data structures for the unified tree architecture
 * where Realm, Layer, Class, Instance, ArcFamily, and ArcClass are all represented
 * as clickable nodes with detail panels.
 *
 * v11.7 Principle: "If it's a node in Neo4j, it's a node everywhere"
 */

// ============================================================================
// Dual Icon Format
// ============================================================================

/**
 * Dual icon format for different rendering contexts.
 * - web: Lucide icon name (for Studio/React)
 * - terminal: Unicode symbol (for TUI/Rust)
 */
export interface DualIcon {
  /** Lucide icon name (e.g., "globe", "settings", "box") */
  web: string;
  /** Unicode symbol (e.g., "◉", "⚙", "◆") */
  terminal: string;
}

// ============================================================================
// Node Types
// ============================================================================

/**
 * All possible node types in the unified tree.
 */
export type UnifiedNodeType =
  | 'section'
  | 'realm'
  | 'layer'
  | 'class'
  | 'instance'
  | 'arcFamily'
  | 'arcClass';

/**
 * Section types at the top level of the tree.
 */
export type SectionType = 'nodes' | 'arcs';

/**
 * Base interface for all unified tree nodes.
 */
export interface UnifiedNodeBase {
  /** Unique identifier (e.g., "Realm:shared", "Class:Locale") */
  id: string;
  /** Node type for discriminated union */
  type: UnifiedNodeType;
  /** Depth in tree hierarchy */
  depth: number;
  /** Display label */
  label: string;
  /** Icon for this node */
  icon: DualIcon;
  /** Parent node ID (null for root sections) */
  parentId: string | null;
  /** Whether this node can be expanded */
  expandable: boolean;
}

/**
 * Section node (top-level: "Nodes" or "Arcs")
 */
export interface SectionNode extends UnifiedNodeBase {
  type: 'section';
  sectionType: SectionType;
  childCount: number;
}

/**
 * Realm node (e.g., "shared", "org")
 */
export interface RealmNode extends UnifiedNodeBase {
  type: 'realm';
  key: string;
  color: string;
  layerCount: number;
  classCount: number;
  instanceCount: number;
}

/**
 * Layer node (e.g., "config", "semantic")
 */
export interface LayerNode extends UnifiedNodeBase {
  type: 'layer';
  key: string;
  realm: string;
  color: string;
  classCount: number;
  instanceCount: number;
}

/**
 * Class node (e.g., "Locale", "Entity")
 */
export interface ClassNode extends UnifiedNodeBase {
  type: 'class';
  name: string;
  realm: string;
  layer: string;
  trait: NodeTrait;
  instanceCount: number;
  outgoingArcs: number;
  incomingArcs: number;
  requiredProps: number;
  totalProps: number;
}

/**
 * Instance node (actual data node in Neo4j)
 */
export interface InstanceNode extends UnifiedNodeBase {
  type: 'instance';
  key: string;
  className: string;
  displayName: string;
  labels: string[];
}

/**
 * ArcFamily node (e.g., "ownership", "semantic")
 */
export interface ArcFamilyNode extends UnifiedNodeBase {
  type: 'arcFamily';
  key: string;
  color: string;
  arcClassCount: number;
  instanceCount: number;
}

/**
 * ArcClass node (e.g., "HAS_PAGE", "USES_ENTITY")
 */
export interface ArcClassNode extends UnifiedNodeBase {
  type: 'arcClass';
  name: string;
  family: string;
  source: string;
  target: string;
  cardinality: ArcCardinality;
  instanceCount: number;
}

/**
 * Discriminated union of all node types.
 */
export type UnifiedNode =
  | SectionNode
  | RealmNode
  | LayerNode
  | ClassNode
  | InstanceNode
  | ArcFamilyNode
  | ArcClassNode;

// ============================================================================
// Classification Types (from taxonomy)
// ============================================================================

/**
 * Node trait (locale behavior).
 */
export type NodeTrait =
  | 'defined'
  | 'authored'
  | 'imported'
  | 'generated'
  | 'retrieved';

/**
 * Arc cardinality.
 */
export type ArcCardinality =
  | 'zero_to_one'
  | 'one_to_one'
  | 'one_to_many'
  | 'many_to_many';

// ============================================================================
// Badge Types
// ============================================================================

/**
 * Badge displayed on the right side of tree nodes.
 */
export interface NodeBadge {
  /** Display icon */
  icon: string;
  /** Short abbreviation (e.g., "fam", "shd", "cfg") */
  abbrev: string;
  /** Color (hex or CSS variable) */
  color: string;
}

/**
 * Pre-defined badge configurations.
 */
export const BADGES = {
  // Realm badges
  REALM_SHARED: { icon: '◎', abbrev: 'shd', color: '#2aa198' },
  REALM_ORG: { icon: '●', abbrev: 'org', color: '#6c71c4' },

  // Meta-type badges
  ARC_FAMILY: { icon: '●', abbrev: 'fam', color: '#f59e0b' },
  ARC_CLASS: { icon: '●', abbrev: 'arc', color: '#f59e0b' },

  // Layer badges
  LAYER_CONFIG: { icon: '◎', abbrev: 'cfg', color: '#64748b' },
  LAYER_LOCALE: { icon: '◎', abbrev: 'loc', color: '#64748b' },
  LAYER_GEOGRAPHY: { icon: '▧', abbrev: 'geo', color: '#10b981' },
  LAYER_KNOWLEDGE: { icon: '◇', abbrev: 'kno', color: '#8b5cf6' },
  LAYER_FOUNDATION: { icon: '▤', abbrev: 'fnd', color: '#3b82f6' },
  LAYER_STRUCTURE: { icon: '▣', abbrev: 'str', color: '#06b6d4' },
  LAYER_SEMANTIC: { icon: '◆', abbrev: 'sem', color: '#f97316' },
  LAYER_INSTRUCTION: { icon: '▥', abbrev: 'ins', color: '#eab308' },
  LAYER_OUTPUT: { icon: '●', abbrev: 'out', color: '#22c55e' },
} as const satisfies Record<string, NodeBadge>;

// ============================================================================
// Lazy Loading Types
// ============================================================================

/**
 * State of child nodes for lazy loading.
 */
export type LazyChildrenState =
  | { status: 'not_loaded' }
  | { status: 'loading' }
  | { status: 'loaded'; items: string[]; total: number; hasMore: boolean }
  | { status: 'leaf' };

/**
 * Pagination constants.
 */
export const PAGINATION = {
  /** Initial batch size when expanding a Class */
  INITIAL_BATCH: 10,
  /** Page size for "Load more" */
  PAGE_SIZE: 50,
  /** Maximum before showing warning */
  MAX_DISPLAY: 1000,
} as const;

// ============================================================================
// Tree Store Types
// ============================================================================

/**
 * State for the unified tree store.
 */
export interface UnifiedTreeState {
  /** All nodes by ID */
  nodes: Map<string, UnifiedNode>;
  /** Root node IDs in display order */
  rootOrder: string[];
  /** Set of expanded node IDs */
  expanded: Set<string>;
  /** Set of currently loading node IDs */
  loading: Set<string>;
  /** Children state per node */
  children: Map<string, LazyChildrenState>;
  /** Currently selected node ID */
  selectedId: string | null;
  /** Currently focused node ID (keyboard navigation) */
  focusedId: string | null;
}

/**
 * Actions for the unified tree store.
 */
export interface UnifiedTreeActions {
  /** Toggle expand/collapse for a node */
  toggleExpand: (nodeId: string) => void;
  /** Select a node (show in detail panel) */
  selectNode: (nodeId: string) => void;
  /** Load children for a node */
  loadChildren: (nodeId: string) => Promise<void>;
  /** Load more children (pagination) */
  loadMoreChildren: (nodeId: string) => Promise<void>;
  /** Refresh the entire tree */
  refreshTree: () => Promise<void>;
  /** Reset to initial state */
  reset: () => void;
}

// ============================================================================
// API Types
// ============================================================================

/**
 * Request to load instances for a Class.
 */
export interface LoadInstancesRequest {
  class: string;
  offset: number;
  limit: number;
}

/**
 * Response from loading instances.
 */
export interface LoadInstancesResponse {
  class: string;
  instances: Array<{
    key: string;
    displayName: string;
    labels: string[];
  }>;
  total: number;
  offset: number;
}

/**
 * Response for node details (panel).
 */
export interface NodeDetailsResponse {
  nodeId: string;
  type: UnifiedNodeType;
  properties: Record<string, unknown>;
  relationships: Array<{
    type: string;
    direction: 'outgoing' | 'incoming';
    targetLabel: string;
    count: number;
  }>;
}

// ============================================================================
// View Types (updated for v11.7)
// ============================================================================

/**
 * Unified view definition with dual icons (v11.7).
 * This extends the base ViewDefinition with unified tree features.
 */
export interface UnifiedViewDefinition {
  id: string;
  description: string;
  icon: DualIcon;
  color: string;
  category: 'meta' | 'data' | 'overlay' | 'contextual';
  contextual?: boolean;
  applicableTypes?: string[];
  modes: Array<'graph' | 'nexus'>;
  params?: string[];
  cypher: string;
}

/**
 * Unified view registry structure (v11.7).
 */
export interface UnifiedViewRegistry {
  version: string;
  description: string;
  views: UnifiedViewDefinition[];
}

// ============================================================================
// Type Guards
// ============================================================================

export function isSectionNode(node: UnifiedNode): node is SectionNode {
  return node.type === 'section';
}

export function isRealmNode(node: UnifiedNode): node is RealmNode {
  return node.type === 'realm';
}

export function isLayerNode(node: UnifiedNode): node is LayerNode {
  return node.type === 'layer';
}

export function isClassNode(node: UnifiedNode): node is ClassNode {
  return node.type === 'class';
}

export function isInstanceNode(node: UnifiedNode): node is InstanceNode {
  return node.type === 'instance';
}

export function isArcFamilyNode(node: UnifiedNode): node is ArcFamilyNode {
  return node.type === 'arcFamily';
}

export function isArcClassNode(node: UnifiedNode): node is ArcClassNode {
  return node.type === 'arcClass';
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Generate a node ID from components.
 */
export function makeNodeId(type: UnifiedNodeType, ...parts: string[]): string {
  const prefix = type.charAt(0).toUpperCase() + type.slice(1);
  return `${prefix}:${parts.join(':')}`;
}

/**
 * Parse a node ID into components.
 */
export function parseNodeId(id: string): { type: string; parts: string[] } | null {
  const colonIndex = id.indexOf(':');
  if (colonIndex === -1) return null;

  const type = id.slice(0, colonIndex).toLowerCase();
  const parts = id.slice(colonIndex + 1).split(':');

  return { type, parts };
}

/**
 * Get badge for a layer key.
 */
export function getLayerBadge(layerKey: string): NodeBadge | undefined {
  const badgeKey = `LAYER_${layerKey.toUpperCase()}` as keyof typeof BADGES;
  return BADGES[badgeKey];
}

/**
 * Get badge for a realm key.
 */
export function getRealmBadge(realmKey: string): NodeBadge | undefined {
  const badgeKey = `REALM_${realmKey.toUpperCase()}` as keyof typeof BADGES;
  return BADGES[badgeKey];
}
