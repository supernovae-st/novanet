// src/filters/types.ts
// v9.0.0: NodeCategory removed — use Layer directly
import type { NodeType } from '../types/nodes.js';

export type { NodeType };

// =============================================================================
// FILTER CRITERIA
// =============================================================================

export interface FilterCriteria {
  // Node selection
  nodeTypes?: NodeType[];
  excludeTypes?: NodeType[];

  // Locale filtering
  locale?: string;
  localeFamily?: string;

  // Property filtering
  active?: boolean;

  // Search
  searchQuery?: string;
  searchFields?: string[];

  // Depth
  maxDepth?: number;
}

// =============================================================================
// INCLUDE RULES (for relationship traversal)
// =============================================================================

export type RelationDirection = 'outgoing' | 'incoming' | 'both';

export interface IncludeRule {
  relation: string;
  direction: RelationDirection;
  depth?: number;
  targetTypes?: NodeType[];
  filters?: FilterCriteria;
  include?: IncludeRule[];  // Nested includes
}

// =============================================================================
// VIEW DEFINITION (YAML structure)
// =============================================================================

export interface ViewDefinition {
  id: string;
  name: string;
  description: string;
  version: string;

  root: {
    type: NodeType;
    key?: string;
  };

  include: IncludeRule[];
  filters?: FilterCriteria;
}

// =============================================================================
// CYPHER OUTPUT
// =============================================================================

export interface CypherQuery {
  query: string;
  params: Record<string, unknown>;
}

// =============================================================================
// VIEW REGISTRY
// =============================================================================

/**
 * View categories for UI grouping (v11.6.1).
 * - meta: Schema exploration (Realm, Layer, Kind, ArcKind)
 * - data: Instance exploration by realm/layer/purpose
 * - overlay: Meta + Data combined for debugging
 * - contextual: Node-centered subgraphs
 */
export type ViewCategory = 'meta' | 'data' | 'overlay' | 'contextual';

/**
 * Navigation modes that a view supports.
 * Defined here (not imported from Studio) to avoid circular dependencies.
 */
export type ViewNavigationMode = 'data' | 'meta' | 'overlay';

/**
 * View entry from _registry.yaml
 */
export interface ViewRegistryEntry {
  id: string;
  description: string;
  category: ViewCategory;
  /** Emoji icon for display */
  icon?: string;
  /** Hex color for visual identity */
  color?: string;
  /** Which navigation modes show this view */
  modes?: ViewNavigationMode[];
  /** Whether this is a contextual view (shown in node sidebar) */
  contextual?: boolean;
  /** Node types this view applies to (for contextual views) */
  applicable_types?: string[];
  /** Required params (e.g., ['realm', 'kind']) */
  params?: string[];
  /** Cypher query template (embedded in _registry.yaml) */
  cypher?: string;
}

export interface ViewRegistry {
  version: string;
  description?: string;
  views: ViewRegistryEntry[];
}

/**
 * Grouped view category for UI rendering.
 */
export interface ViewCategoryGroup {
  id: ViewCategory;
  name: string;
  views: ViewRegistryEntry[];
}
