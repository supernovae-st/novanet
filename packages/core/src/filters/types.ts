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
// VIEW REGISTRY (v0.12.5: views.yaml format)
// =============================================================================

/**
 * View categories for UI grouping (v0.12.5).
 * - schema: Schema exploration (Classes, ArcClasses)
 * - data: Instance exploration (Project, Locales, Geography)
 * - generation: AI agent context assembly
 * - contextual: Node-centered subgraphs
 */
export type ViewCategory = 'schema' | 'data' | 'generation' | 'contextual';

/**
 * Dual-format icon (web + terminal).
 */
export interface ViewIcon {
  web: string;
  terminal: string;
}

/**
 * Category definition from views.yaml.
 */
export interface ViewCategoryDef {
  label: string;
  icon?: ViewIcon;
  color?: string;
  description?: string;
}

/**
 * View entry from views.yaml (v0.12.5 format).
 */
export interface ViewRegistryEntry {
  id: string;
  name: string;
  description: string;
  category: ViewCategory;
  /** Dual-format icon (web + terminal) */
  icon?: ViewIcon;
  /** Hex color for visual identity */
  color?: string;
  /** Root node type (null for global views) */
  root_type?: string | null;
  /** Whether this is a contextual view (shown in node sidebar) */
  contextual?: boolean;
  /** Node types this view applies to (for contextual views) */
  applicable_types?: string[];
  /** Cypher query template */
  cypher?: string;
}

export interface ViewRegistry {
  version: string;
  description?: string;
  categories: Record<string, ViewCategoryDef>;
  views: ViewRegistryEntry[];
}

/**
 * Grouped view category for UI rendering.
 */
export interface ViewCategoryGroup {
  id: ViewCategory;
  name: string;
  icon?: ViewIcon;
  color?: string;
  views: ViewRegistryEntry[];
}
