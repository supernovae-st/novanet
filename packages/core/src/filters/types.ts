// src/filters/types.ts
// v8.1.0: NodeType and NodeCategory imported from single source of truth
import type { Priority, Freshness } from '../types/index.js';
import { NODE_CATEGORIES, type NodeType, type NodeCategory } from '../types/nodes.js';

// Re-export for backwards compatibility
export { NODE_CATEGORIES };
export type { NodeType, NodeCategory };

// =============================================================================
// FILTER CRITERIA
// =============================================================================

export interface FilterCriteria {
  // Node selection
  nodeTypes?: NodeType[];
  categories?: NodeCategory[];
  excludeTypes?: NodeType[];

  // Locale filtering
  locale?: string;
  localeFamily?: string;

  // Property filtering
  priority?: Priority[];
  freshness?: Freshness[];
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

export interface ViewRegistry {
  version: string;
  views: Array<{
    id: string;
    file: string;
    description: string;
  }>;
}
