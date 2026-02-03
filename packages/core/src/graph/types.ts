// packages/core/src/graph/types.ts
// NovaNet Graph Module Types - Schema visualization structures
// v9.0.0

import type { NodeType, Realm, Layer } from '../types/nodes.js';
import type { RelationType } from '../schemas/relations.schema.js';

// Re-export Layer for auto-generated files that import from here
export type { Layer } from '../types/nodes.js';

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
  /** Realm (global, shared, project) */
  realm: Realm;
  /** Layer within the realm */
  layer: Layer;
  /** Human-readable label */
  label: string;
  /** Description of this node type */
  description: string;
  /** Trait (invariant, localized, knowledge, derived, job) */
  trait: string;
  /** Optional icon for display */
  icon?: string;
  /** Optional color for display */
  color?: string;
}

// =============================================================================
// SCHEMA ARC - Represents an arc (relationship) type
// =============================================================================

/**
 * Schema arc representing an arc type in the ontology.
 * Used for schema visualization (arcs between node types).
 */
export interface SchemaArc {
  /** Unique identifier (format: "schema-arc-{index}") */
  id: string;
  /** The RelationType this represents (Neo4j relationship type) */
  relationType: RelationType;
  /** Source NodeType(s) */
  sourceType: NodeType | NodeType[];
  /** Target NodeType(s) */
  targetType: NodeType | NodeType[];
  /** Human-readable label */
  label: string;
  /** Description of this arc type */
  description: string;
  /** Cardinality (1:1, 1:N, N:1, N:M) */
  cardinality: string;
}

/** @deprecated Use SchemaArc instead */
export type SchemaEdge = SchemaArc;

// =============================================================================
// LAYER METADATA
// =============================================================================

/**
 * Metadata for a layer
 */
export interface LayerMeta {
  /** Display label */
  label: string;
  /** Description of this layer */
  description: string;
  /** Icon for display */
  icon: string;
  /** NodeTypes belonging to this layer */
  nodeTypes: NodeType[];
}

// =============================================================================
// REALM DEFINITION
// =============================================================================

/**
 * Realm hierarchy definition.
 * Contains all layers and their metadata for a realm.
 */
export interface RealmDefinition {
  /** The realm name */
  realm: Realm;
  /** Display label (uppercase) */
  label: string;
  /** Icon for display */
  icon: string;
  /** Description of this realm */
  description: string;
  /** Layers within this realm */
  layers: Record<Layer, LayerMeta>;
}

// =============================================================================
// HIERARCHICAL SCHEMA DATA
// =============================================================================

/**
 * Complete hierarchical schema data.
 * Used by visualizers that need grouped layout (like Studio).
 */
export interface HierarchicalSchemaData {
  /** All 3 realm definitions */
  realms: Record<Realm, RealmDefinition>;
  /** All schema nodes (44 nodes) */
  nodes: SchemaNode[];
  /** All schema arcs (~89 arcs expanded from 83 arc types) */
  arcs: SchemaArc[];
  /** @deprecated Use arcs instead */
  edges?: SchemaArc[];
  /** Statistics */
  stats: {
    /** Total number of node types (44) */
    totalNodes: number;
    /** Total number of arcs */
    totalArcs: number;
    /** @deprecated Use totalArcs instead */
    totalEdges?: number;
    /** Node counts per realm */
    nodesByRealm: Record<Realm, number>;
  };
}

// =============================================================================
// FLAT SCHEMA GRAPH RESULT
// =============================================================================

/**
 * Flat schema graph result (for simple consumers).
 * Just nodes and arcs without hierarchy metadata.
 */
export interface SchemaGraphResult {
  /** All schema nodes */
  nodes: SchemaNode[];
  /** All schema arcs */
  arcs: SchemaArc[];
  /** @deprecated Use arcs instead */
  edges?: SchemaArc[];
}
