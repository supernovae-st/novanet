/**
 * ELK Layout Engine for Schema Mode
 *
 * Applies hierarchical layout to schema visualization using ELK.js.
 * Creates React Flow nodes with proper parent/child relationships.
 *
 * CRITICAL FIXES (from plan):
 * - P0: Convert ELK absolute positions to React Flow RELATIVE positions for child nodes
 * - P1: Skip empty subcategories to prevent ELK failures
 * - Use 'elk.hierarchyHandling': 'INCLUDE_CHILDREN' for nested groups
 */

import ELK from 'elkjs/lib/elk.bundled.js';
import type { ElkNode, ElkExtendedEdge } from 'elkjs';
import type { Node, Edge } from '@xyflow/react';
import type {
  HierarchicalSchemaData,
  SchemaNode,
  Subcategory,
  SubcategoryMeta,
} from '@novanet/core/graph';
import type { Scope } from '@novanet/core/types';

// =============================================================================
// CONSTANTS
// =============================================================================

/** ELK layout options for hierarchical schema */
const ELK_OPTIONS: Record<string, string> = {
  'elk.algorithm': 'layered',
  'elk.direction': 'RIGHT',
  'elk.spacing.nodeNode': '20',
  'elk.layered.spacing.nodeNodeBetweenLayers': '50',
  'elk.hierarchyHandling': 'INCLUDE_CHILDREN', // Critical for nested groups
  'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
};

/** Padding for scope groups */
const SCOPE_GROUP_PADDING = 40;

/** Padding for subcategory groups */
const SUBCATEGORY_GROUP_PADDING = 20;

/** Width of schema node cards */
const SCHEMA_NODE_WIDTH = 140;

/** Height of schema node cards */
const SCHEMA_NODE_HEIGHT = 50;

// =============================================================================
// TYPES
// =============================================================================

/** Result of applying ELK layout */
export interface SchemaLayoutResult {
  nodes: Node[];
  edges: Edge[];
}

/** Data for scope group nodes */
interface ScopeGroupData extends Record<string, unknown> {
  scope: Scope;
  label: string;
  icon: string;
  nodeCount: number;
}

/** Data for subcategory group nodes */
interface SubcategoryGroupData extends Record<string, unknown> {
  scope: Scope;
  subcategory: string;
  label: string;
  icon: string;
  nodeCount: number;
}

/** Data for schema nodes */
interface SchemaNodeData extends Record<string, unknown> {
  nodeType: string;
  label: string;
  description: string;
  scope: Scope;
  subcategory: string;
}

// =============================================================================
// MAIN FUNCTION
// =============================================================================

/**
 * Apply ELK hierarchical layout to schema data.
 *
 * Creates a React Flow graph with:
 * - Scope group nodes (top-level containers)
 * - Subcategory group nodes (nested in scopes)
 * - Schema nodes (nested in subcategories)
 *
 * @param hierarchy - Hierarchical schema data from @novanet/core
 * @param direction - Layout direction ('RIGHT' or 'DOWN')
 * @returns Promise with React Flow nodes and edges
 */
export async function applySchemaLayout(
  hierarchy: HierarchicalSchemaData,
  direction: 'RIGHT' | 'DOWN' = 'RIGHT'
): Promise<SchemaLayoutResult> {
  const elk = new ELK();

  try {
    const elkGraph = buildElkGraph(hierarchy, direction);
    const layoutedGraph = await elk.layout(elkGraph);
    const result = convertElkToReactFlow(layoutedGraph, hierarchy);

    // If ELK produced no nodes but input has nodes, use fallback
    if (result.nodes.length === 0 && hierarchy.nodes.length > 0) {
      console.warn('ELK produced no nodes, using fallback grid layout');
      return fallbackGridLayout(hierarchy);
    }

    return result;
  } catch (error) {
    console.error('ELK layout failed:', error);
    // Fallback to simple grid layout
    return fallbackGridLayout(hierarchy);
  }
}

// =============================================================================
// ELK GRAPH BUILDING
// =============================================================================

/**
 * Build ELK graph structure from hierarchical schema data.
 *
 * P1 FIX: Skip empty subcategories to prevent ELK layout failures.
 *
 * @param hierarchy - Hierarchical schema data
 * @param direction - Layout direction
 * @returns ELK graph structure
 */
function buildElkGraph(
  hierarchy: HierarchicalSchemaData,
  direction: string
): ElkNode {
  const children: ElkNode[] = [];
  const edges: ElkExtendedEdge[] = [];

  // Iterate over scopes in consistent order
  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];

  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) {
      console.warn(`Skipping unknown scope: ${scope}`);
      continue;
    }

    const subcategoryChildren: ElkNode[] = [];

    // Create subcategory groups within scope
    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      const meta = subcatMeta as SubcategoryMeta;

      // P1 FIX: Skip empty subcategories (would cause ELK to fail)
      if (!meta.nodeTypes || meta.nodeTypes.length === 0) {
        console.warn(`Skipping empty subcategory: ${scope}/${subcatName}`);
        continue;
      }

      // Create schema nodes within subcategory
      const nodeChildren: ElkNode[] = meta.nodeTypes.map((nodeType) => ({
        id: `schema-${nodeType}`,
        width: SCHEMA_NODE_WIDTH,
        height: SCHEMA_NODE_HEIGHT,
        labels: [{ text: nodeType }], // ELK labels for debugging
      }));

      subcategoryChildren.push({
        id: `subcat-${scope}-${subcatName}`,
        layoutOptions: {
          'elk.padding': `[top=${SUBCATEGORY_GROUP_PADDING},left=${SUBCATEGORY_GROUP_PADDING},bottom=${SUBCATEGORY_GROUP_PADDING},right=${SUBCATEGORY_GROUP_PADDING}]`,
        },
        children: nodeChildren,
      });
    }

    // P1 FIX: Skip scope if all subcategories were empty
    if (subcategoryChildren.length === 0) {
      console.warn(`Skipping empty scope: ${scope}`);
      continue;
    }

    children.push({
      id: `scope-${scope}`,
      layoutOptions: {
        'elk.padding': `[top=${SCOPE_GROUP_PADDING},left=${SCOPE_GROUP_PADDING},bottom=${SCOPE_GROUP_PADDING},right=${SCOPE_GROUP_PADDING}]`,
      },
      children: subcategoryChildren,
    });
  }

  // Create edges (only between schema nodes, not groups)
  hierarchy.edges.forEach((edge, index) => {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    edges.push({
      id: `edge-${index}`,
      sources: [sourceId],
      targets: [targetId],
    });
  });

  return {
    id: 'root',
    layoutOptions: {
      ...ELK_OPTIONS,
      'elk.direction': direction,
    },
    children,
    edges,
  };
}

// =============================================================================
// ELK TO REACT FLOW CONVERSION
// =============================================================================

/**
 * Convert ELK layout result to React Flow nodes/edges.
 *
 * P0 CRITICAL: ELK returns ABSOLUTE positions for all nodes,
 * but React Flow child nodes require RELATIVE positions to their parent.
 *
 * When using parentId + extent: 'parent', React Flow positions children
 * relative to the parent's position. So we must:
 *   childPosition = elkChild.position (already relative in nested ELK structure)
 *
 * Note: When ELK uses INCLUDE_CHILDREN, nested children already have positions
 * relative to their direct parent, so we can use them directly.
 *
 * @param elkGraph - Layouted ELK graph
 * @param hierarchy - Original hierarchy data for metadata
 * @returns React Flow nodes and edges
 */
function convertElkToReactFlow(
  elkGraph: ElkNode,
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Process scope groups (top-level - use absolute positions)
  elkGraph.children?.forEach((scopeGroup) => {
    const scopeId = scopeGroup.id;
    const scope = scopeId.replace('scope-', '') as Scope;
    const scopeDef = hierarchy.scopes[scope];

    if (!scopeDef) {
      console.warn(`Cannot find scope definition for: ${scope}`);
      return;
    }

    // Scope group position (top-level, so this is absolute from root)
    const scopeX = scopeGroup.x || 0;
    const scopeY = scopeGroup.y || 0;

    // Add scope group node (absolute position, it's top-level)
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: scopeX, y: scopeY },
      style: { width: scopeGroup.width, height: scopeGroup.height },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: hierarchy.stats.nodesByScope[scope],
      } as ScopeGroupData,
    });

    // Process subcategory groups
    scopeGroup.children?.forEach((subcatGroup) => {
      const subcatId = subcatGroup.id;
      // Extract subcategory name (format: "subcat-Scope-subcatName")
      const subcatName = subcatId.replace(`subcat-${scope}-`, '');
      const subcatMeta = scopeDef.subcategories[subcatName as Subcategory];

      // Subcategory position - ELK nested children have relative positions
      // P0 FIX: Use directly since ELK INCLUDE_CHILDREN gives relative positions
      const subcatRelX = subcatGroup.x || 0;
      const subcatRelY = subcatGroup.y || 0;

      // Add subcategory group node (relative to scope parent)
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: subcatRelX, y: subcatRelY },
        style: { width: subcatGroup.width, height: subcatGroup.height },
        data: {
          scope,
          subcategory: subcatName,
          label: subcatMeta?.label || subcatName,
          icon: subcatMeta?.icon || '',
          nodeCount: subcatMeta?.nodeTypes.length || 0,
        } as SubcategoryGroupData,
      });

      // Process schema nodes
      subcatGroup.children?.forEach((schemaNode) => {
        const nodeType = schemaNode.id.replace('schema-', '');
        const schemaData = hierarchy.nodes.find((n) => n.nodeType === nodeType);

        // Schema node position - ELK nested children have relative positions
        // P0 FIX: Use directly since ELK INCLUDE_CHILDREN gives relative positions
        const nodeRelX = schemaNode.x || 0;
        const nodeRelY = schemaNode.y || 0;

        nodes.push({
          id: schemaNode.id,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: false, // Schema nodes are not user-draggable
          position: { x: nodeRelX, y: nodeRelY },
          data: {
            nodeType,
            label: schemaData?.label || nodeType,
            description: schemaData?.description || '',
            scope,
            subcategory: subcatName,
          } as SchemaNodeData,
        });
      });
    });
  });

  // Build set of valid node IDs for edge validation
  const validNodeIds = new Set(nodes.map((n) => n.id));

  // Convert edges - only include edges where both source and target exist
  hierarchy.edges.forEach((edge, index) => {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    // Skip edges where source or target node doesn't exist (filtered/hidden)
    if (!validNodeIds.has(sourceId) || !validNodeIds.has(targetId)) {
      return;
    }

    edges.push({
      id: `edge-${index}`,
      source: sourceId,
      target: targetId,
      type: 'floating', // Use floating edge type which is properly registered
      data: {
        relationType: edge.relationType,
        label: edge.label,
        description: edge.description,
      },
    });
  });

  return { nodes, edges };
}

// =============================================================================
// FALLBACK LAYOUT
// =============================================================================

/**
 * Fallback grid layout if ELK fails.
 * Creates a simple grid of schema nodes without groups.
 *
 * @param hierarchy - Hierarchical schema data
 * @returns React Flow nodes and edges in grid layout
 */
function fallbackGridLayout(hierarchy: HierarchicalSchemaData): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  const spacing = 200;
  const columns = 7;

  hierarchy.nodes.forEach((node: SchemaNode, index: number) => {
    nodes.push({
      id: `schema-${node.nodeType}`,
      type: 'schemaNode',
      position: {
        x: (index % columns) * spacing,
        y: Math.floor(index / columns) * spacing,
      },
      data: {
        nodeType: node.nodeType,
        label: node.label,
        description: node.description,
        scope: node.scope,
        subcategory: node.subcategory,
      } as SchemaNodeData,
    });
  });

  // Build set of valid node IDs for edge validation
  const validNodeIds = new Set(nodes.map((n) => n.id));

  // Convert edges - only include edges where both source and target exist
  hierarchy.edges.forEach((edge, index) => {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    // Skip edges where source or target node doesn't exist
    if (!validNodeIds.has(sourceId) || !validNodeIds.has(targetId)) {
      return;
    }

    edges.push({
      id: `edge-${index}`,
      source: sourceId,
      target: targetId,
      type: 'floating', // Use floating edge type which is properly registered
      data: {
        relationType: edge.relationType,
        label: edge.label,
      },
    });
  });

  return { nodes, edges };
}
