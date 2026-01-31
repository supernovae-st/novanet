// src/lib/schemaLayouts/elkLayered.ts
/**
 * ELK Layered Layout - Sugiyama-based with edge crossing minimization
 *
 * Uses Eclipse Layout Kernel's layered algorithm which implements
 * a modified Sugiyama algorithm with 5 phases:
 * 1. Cycle Breaking - Remove back edges
 * 2. Layer Assignment - Assign nodes to horizontal/vertical layers
 * 3. Crossing Minimization - Minimize edge crossings (LAYER_SWEEP)
 * 4. Node Placement - Position nodes within layers (NETWORK_SIMPLEX)
 * 5. Edge Routing - Route edges (ORTHOGONAL for clarity)
 *
 * This produces much cleaner layouts than treemap for graphs with
 * many relationships, as it specifically optimizes for edge visibility.
 */

import ELK, { type ElkNode, type ElkExtendedEdge, type LayoutOptions } from 'elkjs/lib/elk.bundled.js';
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import {
  SCOPE_CONFIGS,
  NODE_WIDTH,
  NODE_HEIGHT,
} from './types';

// =============================================================================
// LOCAL SPACING CONSTANTS - Clean semantics for bottom-up grid layout
// =============================================================================

// Internal spacing (tight - inside containers)
const SUBCAT_PAD = 16;            // Padding inside subcategory container
const SUBCAT_HEADER = 28;         // Height for subcategory label
const SCOPE_PAD = 32;             // Padding inside scope container
const SCOPE_HEADER = 44;          // Height for scope label
const NODE_CELL_GAP = 12;         // Gap between nodes in grid

// External spacing (wide - between containers for edge visibility)
const SUBCAT_SPACING = 60;        // Gap between subcategories
const SCOPE_SPACING = 120;        // Gap between scopes

// Canvas
const LAYOUT_MARGIN = 40;         // Margin around entire layout
import type { Scope } from '@novanet/core/types';

// Initialize ELK
const elk = new ELK();

/**
 * ELK Layout Options for optimal edge crossing minimization
 */
const ELK_OPTIONS: LayoutOptions = {
  // Core algorithm
  'elk.algorithm': 'layered',
  'elk.direction': 'DOWN',

  // Crossing minimization - the key optimization
  'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
  'elk.layered.crossingMinimization.greedySwitch.type': 'TWO_SIDED',

  // Node placement - Network Simplex for balanced positioning
  'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
  'elk.layered.nodePlacement.bk.fixedAlignment': 'BALANCED',

  // Edge routing - Orthogonal for clarity
  'elk.edgeRouting': 'ORTHOGONAL',
  'elk.layered.unnecessaryBendpoints': 'true',

  // Spacing - tight within, wide between
  'elk.spacing.nodeNode': String(SUBCAT_SPACING),
  'elk.layered.spacing.nodeNodeBetweenLayers': String(SCOPE_SPACING),
  'elk.spacing.edgeNode': String(30),
  'elk.spacing.edgeEdge': String(20),

  // Hierarchy handling
  'elk.hierarchyHandling': 'INCLUDE_CHILDREN',

  // Compaction for tighter layout
  'elk.layered.compaction.postCompaction.strategy': 'EDGE_LENGTH',

  // Aspect ratio (prefer wider than tall)
  'elk.aspectRatio': '1.618',
};

/**
 * ELK Layered Layout - Minimizes edge crossings using Sugiyama algorithm
 *
 * This layout is ideal when you need to clearly see relationships
 * between nodes. It analyzes the graph structure and positions nodes
 * to minimize the number of crossing edges.
 */
export async function applyElkLayeredLayout(
  hierarchy: HierarchicalSchemaData
): Promise<SchemaLayoutResult> {
  // Build ELK graph from hierarchy
  const elkGraph = buildElkGraph(hierarchy);

  // Run ELK layout
  const layoutedGraph = await elk.layout(elkGraph);

  // Convert back to React Flow format
  return convertToReactFlow(layoutedGraph, hierarchy);
}

/**
 * Synchronous wrapper for use in synchronous contexts
 * Uses a smart edge-aware grid layout (barycenter heuristic)
 */
export function applyElkLayeredLayoutSync(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  // For synchronous usage, we use a smart grid layout that respects edges
  // The full ELK layout is available via applyElkLayeredLayout (async)
  return applyEdgeAwareGridLayout(hierarchy);
}

/**
 * Build ELK graph structure from hierarchy data
 */
function buildElkGraph(hierarchy: HierarchicalSchemaData): ElkNode {
  const children: ElkNode[] = [];
  const edges: ElkExtendedEdge[] = [];

  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];

  // Create scope groups as compound nodes with subcategory children
  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const subcatChildren: ElkNode[] = [];

    // Create subcategory groups
    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      if (subcatMeta.nodeTypes.length === 0) continue;

      const nodeChildren: ElkNode[] = [];
      for (const nodeType of subcatMeta.nodeTypes) {
        nodeChildren.push({
          id: `schema-${nodeType}`,
          width: NODE_WIDTH,
          height: NODE_HEIGHT,
          labels: [{ text: nodeType }],
        });
      }

      subcatChildren.push({
        id: `subcat-${scope}-${subcatName}`,
        children: nodeChildren,
        layoutOptions: {
          'elk.padding': `[top=${SUBCAT_PAD + SUBCAT_HEADER},left=${SUBCAT_PAD},bottom=${SUBCAT_PAD},right=${SUBCAT_PAD}]`,
        },
      });
    }

    if (subcatChildren.length > 0) {
      children.push({
        id: `scope-${scope}`,
        children: subcatChildren,
        layoutOptions: {
          'elk.padding': `[top=${SCOPE_PAD + SCOPE_HEADER},left=${SCOPE_PAD},bottom=${SCOPE_PAD},right=${SCOPE_PAD}]`,
        },
      });
    }
  }

  // Create edges
  hierarchy.edges.forEach((edge, index) => {
    // Handle both single and array source/target types
    const sources = Array.isArray(edge.sourceType)
      ? edge.sourceType.map(t => `schema-${t}`)
      : [`schema-${edge.sourceType}`];
    const targets = Array.isArray(edge.targetType)
      ? edge.targetType.map(t => `schema-${t}`)
      : [`schema-${edge.targetType}`];

    edges.push({
      id: `edge-${index}`,
      sources,
      targets,
    });
  });

  return {
    id: 'root',
    children,
    edges,
    layoutOptions: ELK_OPTIONS,
  };
}

/**
 * Convert ELK layout result back to React Flow nodes and edges
 */
function convertToReactFlow(
  elkGraph: ElkNode,
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Process scope groups
  for (const scopeNode of elkGraph.children || []) {
    const scopeId = scopeNode.id;
    const scope = scopeId.replace('scope-', '') as Scope;
    const scopeDef = hierarchy.scopes[scope];
    const config = SCOPE_CONFIGS.find(c => c.scope === scope);

    if (!scopeDef || !config) continue;

    // Count all nodes in this scope
    let nodeCount = 0;
    for (const subcatNode of scopeNode.children || []) {
      nodeCount += subcatNode.children?.length || 0;
    }

    // Add scope group node
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: scopeNode.x || 0, y: scopeNode.y || 0 },
      style: {
        width: scopeNode.width || 400,
        height: scopeNode.height || 300,
      },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount,
      },
    });

    // Process subcategory groups
    for (const subcatNode of scopeNode.children || []) {
      const subcatId = subcatNode.id;
      const subcatName = subcatId.replace(`subcat-${scope}-`, '');
      const subcatMeta = scopeDef.subcategories[subcatName as keyof typeof scopeDef.subcategories];

      if (!subcatMeta) continue;

      // Add subcategory group node
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: subcatNode.x || 0, y: subcatNode.y || 0 },
        style: {
          width: subcatNode.width || 200,
          height: subcatNode.height || 150,
        },
        data: {
          scope,
          subcategory: subcatName,
          label: subcatMeta.label,
          icon: subcatMeta.icon,
          nodeCount: subcatNode.children?.length || 0,
        },
      });

      // Add schema nodes
      for (const schemaNodeElk of subcatNode.children || []) {
        const nodeType = schemaNodeElk.id.replace('schema-', '');
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === nodeType);

        nodes.push({
          id: schemaNodeElk.id,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: true,
          position: {
            x: schemaNodeElk.x || 0,
            y: schemaNodeElk.y || 0,
          },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            scope,
            subcategory: subcatName,
          },
        });
      }
    }
  }

  // Create edges (handle both single and array source/target types)
  const validNodeIds = new Set(nodes.map(n => n.id));
  let edgeIndex = 0;
  for (const edge of hierarchy.edges) {
    const sources = Array.isArray(edge.sourceType)
      ? edge.sourceType.map(t => `schema-${t}`)
      : [`schema-${edge.sourceType}`];
    const targets = Array.isArray(edge.targetType)
      ? edge.targetType.map(t => `schema-${t}`)
      : [`schema-${edge.targetType}`];

    for (const sourceId of sources) {
      for (const targetId of targets) {
        if (validNodeIds.has(sourceId) && validNodeIds.has(targetId)) {
          edges.push({
            id: `edge-${edgeIndex++}`,
            source: sourceId,
            target: targetId,
            type: 'floating',
            data: {
              relationType: edge.relationType,
              label: edge.label,
            },
          });
        }
      }
    }
  }

  return { nodes, edges };
}

/**
 * Bottom-Up Hierarchical Grid Layout
 *
 * Uses a bottom-up approach to ensure nodes fit properly within containers:
 * 1. Calculate grid dimensions for each subcategory based on its nodes
 * 2. Calculate scope dimensions from its subcategories
 * 3. Position everything on the canvas
 *
 * This ensures containers are ALWAYS sized to fit their children.
 */
function applyEdgeAwareGridLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Build adjacency for edge-aware ordering
  const adjacency = new Map<string, Set<string>>();
  for (const edge of hierarchy.edges) {
    const sources = Array.isArray(edge.sourceType)
      ? edge.sourceType.map(String)
      : [String(edge.sourceType)];
    const targets = Array.isArray(edge.targetType)
      ? edge.targetType.map(String)
      : [String(edge.targetType)];

    for (const source of sources) {
      for (const target of targets) {
        if (!adjacency.has(source)) adjacency.set(source, new Set());
        if (!adjacency.has(target)) adjacency.set(target, new Set());
        adjacency.get(source)!.add(target);
        adjacency.get(target)!.add(source);
      }
    }
  }

  // ===========================================================================
  // PHASE 1: Calculate subcategory dimensions (bottom-up)
  // ===========================================================================

  interface SubcatLayout {
    subcatName: string;
    meta: { label: string; icon: string; nodeTypes: readonly string[] };
    orderedNodes: string[];
    cols: number;
    rows: number;
    width: number;
    height: number;
  }

  interface ScopeLayout {
    scope: Scope;
    scopeDef: typeof hierarchy.scopes[Scope];
    subcategories: SubcatLayout[];
    totalWidth: number;
    totalHeight: number;
  }

  const scopeLayouts: ScopeLayout[] = [];
  const MAX_COLS = 6;

  for (const scope of ['Project', 'Global', 'Shared'] as Scope[]) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const subcatLayouts: SubcatLayout[] = [];

    for (const [subcatName, meta] of Object.entries(scopeDef.subcategories)) {
      if (meta.nodeTypes.length === 0) continue;

      const orderedNodes = orderNodesByBarycenter([...meta.nodeTypes], adjacency);
      const nodeCount = orderedNodes.length;
      const cols = Math.min(nodeCount, MAX_COLS);
      const rows = Math.ceil(nodeCount / cols);

      // Calculate dimensions using local constants
      const contentWidth = cols * NODE_WIDTH + (cols - 1) * NODE_CELL_GAP;
      const contentHeight = rows * NODE_HEIGHT + (rows - 1) * NODE_CELL_GAP;
      const width = contentWidth + SUBCAT_PAD * 2;
      const height = contentHeight + SUBCAT_PAD * 2 + SUBCAT_HEADER;

      subcatLayouts.push({ subcatName, meta, orderedNodes, cols, rows, width, height });
    }

    if (subcatLayouts.length === 0) continue;

    // ===========================================================================
    // PHASE 2: Calculate scope dimensions from subcategories
    // ===========================================================================

    const MAX_SUBCATS_PER_ROW = 4;
    const subcatCols = Math.min(subcatLayouts.length, MAX_SUBCATS_PER_ROW);
    const subcatRows = Math.ceil(subcatLayouts.length / subcatCols);

    let maxRowWidths: number[] = [];
    let rowHeights: number[] = [];

    for (let r = 0; r < subcatRows; r++) {
      let rowWidth = 0;
      let maxHeight = 0;

      for (let c = 0; c < subcatCols; c++) {
        const idx = r * subcatCols + c;
        if (idx >= subcatLayouts.length) break;

        const subcat = subcatLayouts[idx];
        rowWidth += subcat.width + (c > 0 ? SUBCAT_SPACING : 0);
        maxHeight = Math.max(maxHeight, subcat.height);
      }

      maxRowWidths.push(rowWidth);
      rowHeights.push(maxHeight);
    }

    const totalContentWidth = Math.max(...maxRowWidths);
    const totalContentHeight = rowHeights.reduce((a, b) => a + b, 0) + (subcatRows - 1) * SUBCAT_SPACING;

    const totalWidth = totalContentWidth + SCOPE_PAD * 2;
    const totalHeight = totalContentHeight + SCOPE_PAD * 2 + SCOPE_HEADER;

    scopeLayouts.push({ scope, scopeDef, subcategories: subcatLayouts, totalWidth, totalHeight });
  }

  // ===========================================================================
  // PHASE 3: Position scopes on canvas
  // ===========================================================================

  let scopeX = LAYOUT_MARGIN;
  let scopeY = LAYOUT_MARGIN;

  for (const scopeLayout of scopeLayouts) {
    const { scope, scopeDef, subcategories, totalWidth, totalHeight } = scopeLayout;

    // Wrap to next row if needed
    if (scopeX + totalWidth > MAX_ROW_WIDTH + LAYOUT_MARGIN && scopeX > LAYOUT_MARGIN) {
      scopeX = LAYOUT_MARGIN;
      scopeY += maxHeightInRow + SCOPE_SPACING;
      maxHeightInRow = 0;
    }

    const scopeId = `scope-${scope}`;

    // Create scope group node
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: scopeX, y: scopeY },
      width: totalWidth,
      height: totalHeight,
      style: { width: totalWidth, height: totalHeight },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: subcategories.reduce((sum, s) => sum + s.orderedNodes.length, 0),
      },
    });

    // ===========================================================================
    // PHASE 4: Position subcategories within scope (HORIZONTALLY)
    // ===========================================================================

    const MAX_SUBCATS_PER_ROW = 4;
    const subcatColsInScope = Math.min(subcategories.length, MAX_SUBCATS_PER_ROW);
    let subcatX = SCOPE_PAD;
    let subcatY = SCOPE_PAD + SCOPE_HEADER;
    let colIndex = 0;
    let rowMaxHeight = 0;

    for (const subcat of subcategories) {
      const subcatId = `subcat-${scope}-${subcat.subcatName}`;

      // Create subcategory group node
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: subcatX, y: subcatY },
        width: subcat.width,
        height: subcat.height,
        style: { width: subcat.width, height: subcat.height },
        data: {
          scope,
          subcategory: subcat.subcatName,
          label: subcat.meta.label,
          icon: subcat.meta.icon,
          nodeCount: subcat.orderedNodes.length,
        },
      });

      // ===========================================================================
      // PHASE 5: Position nodes within subcategory
      // ===========================================================================

      subcat.orderedNodes.forEach((nodeType, idx) => {
        const row = Math.floor(idx / subcat.cols);
        const col = idx % subcat.cols;
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === nodeType);

        nodes.push({
          id: `schema-${nodeType}`,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: true,
          position: {
            x: SUBCAT_PAD + col * (NODE_WIDTH + NODE_CELL_GAP),
            y: SUBCAT_PAD + SUBCAT_HEADER + row * (NODE_HEIGHT + NODE_CELL_GAP),
          },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            scope,
            subcategory: subcat.subcatName,
          },
        });
      });

      // Move to next subcategory position (HORIZONTAL first)
      rowMaxHeight = Math.max(rowMaxHeight, subcat.height);
      colIndex++;

      if (colIndex >= subcatColsInScope) {
        // Move to next row (wrap)
        subcatX = SCOPE_PAD;
        subcatY += rowMaxHeight + SUBCAT_SPACING;
        colIndex = 0;
        rowMaxHeight = 0;
      } else {
        // Move to next column (horizontal)
        subcatX += subcat.width + SUBCAT_SPACING;
      }
    }

    // Move scope position for next scope
    scopeX += totalWidth + SCOPE_SPACING;
    maxHeightInRow = Math.max(maxHeightInRow, totalHeight);
  }

  // ===========================================================================
  // FALLBACK: If no nodes created from hierarchy (broken scopes)
  // ===========================================================================

  if (nodes.length === 0 && hierarchy.nodes.length > 0) {
    const FALLBACK_COLS = 5;

    hierarchy.nodes.forEach((schemaNode, idx) => {
      const col = idx % FALLBACK_COLS;
      const row = Math.floor(idx / FALLBACK_COLS);

      nodes.push({
        id: `schema-${schemaNode.nodeType}`,
        type: 'schemaNode',
        draggable: true,
        position: {
          x: LAYOUT_MARGIN + col * (NODE_WIDTH + NODE_CELL_GAP),
          y: LAYOUT_MARGIN + row * (NODE_HEIGHT + NODE_CELL_GAP),
        },
        data: {
          nodeType: schemaNode.nodeType,
          label: schemaNode.label,
          description: schemaNode.description || '',
          scope: schemaNode.scope,
          subcategory: schemaNode.subcategory,
        },
      });
    });
  }

  // ===========================================================================
  // PHASE 6: Create edges
  // ===========================================================================

  const validNodeIds = new Set(nodes.map(n => n.id));
  let edgeIdx = 0;

  for (const edge of hierarchy.edges) {
    const sources = Array.isArray(edge.sourceType)
      ? edge.sourceType.map(t => `schema-${t}`)
      : [`schema-${edge.sourceType}`];
    const targets = Array.isArray(edge.targetType)
      ? edge.targetType.map(t => `schema-${t}`)
      : [`schema-${edge.targetType}`];

    for (const sourceId of sources) {
      for (const targetId of targets) {
        if (validNodeIds.has(sourceId) && validNodeIds.has(targetId)) {
          edges.push({
            id: `edge-${edgeIdx++}`,
            source: sourceId,
            target: targetId,
            type: 'floating',
            data: {
              relationType: edge.relationType,
              label: edge.label,
            },
          });
        }
      }
    }
  }

  return { nodes, edges };
}

/**
 * Order nodes using barycenter heuristic to minimize edge crossings
 *
 * The barycenter heuristic places connected nodes closer together
 * by computing the average position of each node's neighbors.
 */
function orderNodesByBarycenter(
  nodeTypes: string[],
  adjacency: Map<string, Set<string>>
): string[] {
  if (nodeTypes.length <= 1) return nodeTypes;

  // Start with initial order
  let order = [...nodeTypes];

  // Iterate to improve order (fixed iterations to avoid infinite loop)
  for (let iteration = 0; iteration < 5; iteration++) {
    const positions = new Map<string, number>();
    order.forEach((node, idx) => positions.set(node, idx));

    // Calculate barycenter for each node
    const barycenters = new Map<string, number>();

    for (const nodeType of order) {
      const neighbors = adjacency.get(nodeType);
      if (!neighbors || neighbors.size === 0) {
        barycenters.set(nodeType, positions.get(nodeType)!);
        continue;
      }

      // Average position of neighbors
      let sum = 0;
      let count = 0;
      for (const neighbor of neighbors) {
        const pos = positions.get(neighbor);
        if (pos !== undefined) {
          sum += pos;
          count++;
        }
      }

      if (count > 0) {
        barycenters.set(nodeType, sum / count);
      } else {
        barycenters.set(nodeType, positions.get(nodeType)!);
      }
    }

    // Sort by barycenter
    order = [...order].sort((a, b) => {
      const ba = barycenters.get(a)!;
      const bb = barycenters.get(b)!;
      return ba - bb;
    });
  }

  return order;
}
