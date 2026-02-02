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
 * View categories for UI grouping.
 * - overview: Layer views (complete, global, shared, project)
 * - generation: Orchestrator and sub-agent context views
 * - knowledge: Locale and concept views
 * - project: Project structure views
 * - mining: SEO and GEO pipeline views
 */
export type ViewCategory = 'overview' | 'generation' | 'knowledge' | 'project' | 'mining';

export interface ViewRegistryEntry {
  id: string;
  file: string;
  description: string;
  category: ViewCategory;
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
