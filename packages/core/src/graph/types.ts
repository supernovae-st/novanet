// packages/core/src/graph/types.ts
// NovaNet Graph Module Types - Schema visualization structures
// v1.0.0

import type { NodeType, Scope } from '../types/nodes.js';
import type { RelationType } from '../schemas/relations.schema.js';

// =============================================================================
// SUBCATEGORY (9 subcategories across 3 scopes)
// TODO(v9): Rename Subcategory -> Layer, SubcategoryMeta -> LayerMeta
// TODO(v9): Rename ScopeDefinition -> RealmDefinition
// =============================================================================

/**
 * Subcategory within a scope (from _index.yaml hierarchy)
 * Each NodeType belongs to exactly one subcategory
 */
export type Subcategory =
  // Project scope (5 subcategories) - source: models/nodes/project/
  | 'foundation'   // Project, BrandIdentity, ProjectL10n
  | 'structure'    // Page, Block
  | 'semantic'     // Concept, ConceptL10n
  | 'instruction'  // PageType, PagePrompt, BlockType, BlockPrompt, BlockRules
  | 'output'       // PageL10n, BlockL10n
  // Global scope (2 subcategories)
  | 'config'       // Locale
  | 'knowledge'    // 14 LocaleKnowledge nodes
  // Shared scope (2 subcategories)
  | 'seo'          // SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun
  | 'geo';         // GEOSeedL10n, GEOSeedMetrics, GEOMiningRun

// =============================================================================
// SCHEMA NODE - Represents a NodeType in the ontology
// =============================================================================

/**
 * Schema node representing a NodeType in the ontology.
 * Used for schema visualization (grouped layout with ELK).
 */
export interface SchemaNode {
  /** Unique identifier (format: "schema-{NodeType}") */
  id: string;
  /** The NodeType this represents */
  nodeType: NodeType;
  /** Scope (Global, Shared, Project) */
  scope: Scope;
  /** Subcategory within the scope */
  subcategory: Subcategory;
  /** Human-readable label */
  label: string;
  /** Description of this node type */
  description: string;
  /** Locale behavior (invariant, localized, localeKnowledge, derived, job) */
  behavior: string;
  /** Optional icon for display */
  icon?: string;
  /** Optional color for display */
  color?: string;
}

// =============================================================================
// SCHEMA EDGE - Represents a relation type
// =============================================================================

/**
 * Schema edge representing a relation type.
 * Used for schema visualization (edges between node types).
 */
export interface SchemaEdge {
  /** Unique identifier (format: "schema-edge-{index}") */
  id: string;
  /** The RelationType this represents */
  relationType: RelationType;
  /** Source NodeType(s) */
  sourceType: NodeType | NodeType[];
  /** Target NodeType(s) */
  targetType: NodeType | NodeType[];
  /** Human-readable label */
  label: string;
  /** Description of this relation type */
  description: string;
  /** Cardinality (1:1, 1:N, N:1, N:M) */
  cardinality: string;
}

// =============================================================================
// SUBCATEGORY METADATA
// =============================================================================

/**
 * Metadata for a subcategory
 */
export interface SubcategoryMeta {
  /** Display label */
  label: string;
  /** Description of this subcategory */
  description: string;
  /** Icon for display */
  icon: string;
  /** NodeTypes belonging to this subcategory */
  nodeTypes: NodeType[];
}

// =============================================================================
// SCOPE DEFINITION
// =============================================================================

/**
 * Scope hierarchy definition.
 * Contains all subcategories and their metadata for a scope.
 */
export interface ScopeDefinition {
  /** The scope name */
  scope: Scope;
  /** Display label (uppercase) */
  label: string;
  /** Icon for display */
  icon: string;
  /** Description of this scope */
  description: string;
  /** Subcategories within this scope */
  subcategories: Record<Subcategory, SubcategoryMeta>;
}

// =============================================================================
// HIERARCHICAL SCHEMA DATA
// =============================================================================

/**
 * Complete hierarchical schema data.
 * Used by visualizers that need grouped layout (like Studio).
 */
export interface HierarchicalSchemaData {
  /** All 3 scope definitions */
  scopes: Record<Scope, ScopeDefinition>;
  /** All schema nodes (35 nodes) */
  nodes: SchemaNode[];
  /** All schema edges (~89 edges expanded from 50 relation types) */
  edges: SchemaEdge[];
  /** Statistics */
  stats: {
    /** Total number of node types (35) */
    totalNodes: number;
    /** Total number of edges */
    totalEdges: number;
    /** Node counts per scope */
    nodesByScope: Record<Scope, number>;
  };
}

// =============================================================================
// FLAT SCHEMA GRAPH RESULT
// =============================================================================

/**
 * Flat schema graph result (for simple consumers).
 * Just nodes and edges without hierarchy metadata.
 */
export interface SchemaGraphResult {
  /** All schema nodes */
  nodes: SchemaNode[];
  /** All schema edges */
  edges: SchemaEdge[];
}
