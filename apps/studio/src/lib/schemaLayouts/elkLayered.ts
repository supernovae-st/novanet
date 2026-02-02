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
  REALM_CONFIGS,
  NODE_WIDTH,
  NODE_HEIGHT,
  NODE_GAP,
  LAYER_GAP,
  LAYER_PADDING,
  LAYER_HEADER,
  REALM_GAP,
  REALM_PADDING,
  REALM_HEADER,
  EDGE_NODE_GAP,
  EDGE_EDGE_GAP,
  CANVAS_MARGIN,
  MAX_NODES_PER_ROW,
  MAX_LAYERS_PER_ROW,
  MAX_ROW_WIDTH,
} from './types';
import type { Realm } from '@novanet/core/types';

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

  // Spacing - unified from types.ts
  'elk.spacing.nodeNode': String(LAYER_GAP),
  'elk.layered.spacing.nodeNodeBetweenLayers': String(REALM_GAP),
  'elk.spacing.edgeNode': String(EDGE_NODE_GAP),
  'elk.spacing.edgeEdge': String(EDGE_EDGE_GAP),

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
  try {
    // Build ELK graph from hierarchy
    const elkGraph = buildElkGraph(hierarchy);

    // Run ELK layout
    const layoutedGraph = await elk.layout(elkGraph);

    // Convert back to React Flow format
    return convertToReactFlow(layoutedGraph, hierarchy);
  } catch (error) {
    console.error('[ELK Layout] Failed to compute layout, falling back to grid:', error);
    // Fallback to synchronous edge-aware layout
    return applyEdgeAwareGridLayout(hierarchy);
  }
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

  const realmOrder: Realm[] = ['project', 'global', 'shared'];

  // Create scope groups as compound nodes with subcategory children
  for (const realm of realmOrder) {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) continue;

    const layerChildren: ElkNode[] = [];

    // Create subcategory groups
    for (const [layerName, layerMeta] of Object.entries(realmDef.layers)) {
      if (layerMeta.nodeTypes.length === 0) continue;

      const nodeChildren: ElkNode[] = [];
      for (const nodeType of layerMeta.nodeTypes) {
        nodeChildren.push({
          id: `schema-${nodeType}`,
          width: NODE_WIDTH,
          height: NODE_HEIGHT,
          labels: [{ text: nodeType }],
        });
      }

      layerChildren.push({
        id: `layer-${realm}-${layerName}`,
        children: nodeChildren,
        layoutOptions: {
          'elk.padding': `[top=${LAYER_PADDING + LAYER_HEADER},left=${LAYER_PADDING},bottom=${LAYER_PADDING},right=${LAYER_PADDING}]`,
        },
      });
    }

    if (layerChildren.length > 0) {
      children.push({
        id: `scope-${realm}`,
        children: layerChildren,
        layoutOptions: {
          'elk.padding': `[top=${REALM_PADDING + REALM_HEADER},left=${REALM_PADDING},bottom=${REALM_PADDING},right=${REALM_PADDING}]`,
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
    const realmId = scopeNode.id;
    const realm = realmId.replace('scope-', '') as Realm;
    const realmDef = hierarchy.realms[realm];
    const config = REALM_CONFIGS.find(c => c.realm === realm);

    if (!realmDef || !config) continue;

    // Count all nodes in this scope
    let nodeCount = 0;
    for (const subcatNode of scopeNode.children || []) {
      nodeCount += subcatNode.children?.length || 0;
    }

    // Add scope group node
    nodes.push({
      id: realmId,
      type: 'realmGroup',
      position: { x: scopeNode.x || 0, y: scopeNode.y || 0 },
      style: {
        width: scopeNode.width || 400,
        height: scopeNode.height || 300,
      },
      data: {
        realm,
        label: realmDef.label,
        icon: realmDef.icon,
        nodeCount,
      },
    });

    // Process subcategory groups
    for (const subcatNode of scopeNode.children || []) {
      const layerId = subcatNode.id;
      const layerName = layerId.replace(`layer-${realm}-`, '');
      const layerMeta = realmDef.layers[layerName as keyof typeof realmDef.layers];

      if (!layerMeta) continue;

      // Add subcategory group node
      nodes.push({
        id: layerId,
        type: 'layerGroup',
        parentId: realmId,
        extent: 'parent',
        draggable: true,
        position: { x: subcatNode.x || 0, y: subcatNode.y || 0 },
        style: {
          width: subcatNode.width || 200,
          height: subcatNode.height || 150,
        },
        data: {
          realm,
          layer: layerName,
          label: layerMeta.label,
          icon: layerMeta.icon,
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
          parentId: layerId,
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
            realm,
            layer: layerName,
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
        adjacency.get(source)?.add(target);
        adjacency.get(target)?.add(source);
      }
    }
  }

  // ===========================================================================
  // PHASE 1: Calculate subcategory dimensions (bottom-up)
  // ===========================================================================

  interface SubcatLayout {
    layerName: string;
    meta: { label: string; icon: string; nodeTypes: readonly string[] };
    orderedNodes: string[];
    cols: number;
    rows: number;
    width: number;
    height: number;
  }

  interface RealmLayout {
    realm: Realm;
    realmDef: typeof hierarchy.realms[Realm];
    subcategories: SubcatLayout[];
    totalWidth: number;
    totalHeight: number;
  }

  const realmLayouts: RealmLayout[] = [];
  const MAX_COLS = MAX_NODES_PER_ROW;

  for (const realm of ['project', 'global', 'shared'] as Realm[]) {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) continue;

    const layerLayouts: SubcatLayout[] = [];

    for (const [layerName, meta] of Object.entries(realmDef.layers)) {
      if (meta.nodeTypes.length === 0) continue;

      const orderedNodes = orderNodesByBarycenter([...meta.nodeTypes], adjacency);
      const nodeCount = orderedNodes.length;
      const cols = Math.min(nodeCount, MAX_COLS);
      const rows = Math.ceil(nodeCount / cols);

      // Calculate dimensions using local constants
      const contentWidth = cols * NODE_WIDTH + (cols - 1) * NODE_GAP;
      const contentHeight = rows * NODE_HEIGHT + (rows - 1) * NODE_GAP;
      const width = contentWidth + LAYER_PADDING * 2;
      const height = contentHeight + LAYER_PADDING * 2 + LAYER_HEADER;

      layerLayouts.push({ layerName, meta, orderedNodes, cols, rows, width, height });
    }

    if (layerLayouts.length === 0) continue;

    // ===========================================================================
    // PHASE 2: Calculate scope dimensions from subcategories
    // ===========================================================================

    const layerCols = Math.min(layerLayouts.length, MAX_LAYERS_PER_ROW);
    const layerRows = Math.ceil(layerLayouts.length / layerCols);

    let maxRowWidths: number[] = [];
    let rowHeights: number[] = [];

    for (let r = 0; r < layerRows; r++) {
      let rowWidth = 0;
      let maxHeight = 0;

      for (let c = 0; c < layerCols; c++) {
        const idx = r * layerCols + c;
        if (idx >= layerLayouts.length) break;

        const subcat = layerLayouts[idx];
        rowWidth += subcat.width + (c > 0 ? LAYER_GAP : 0);
        maxHeight = Math.max(maxHeight, subcat.height);
      }

      maxRowWidths.push(rowWidth);
      rowHeights.push(maxHeight);
    }

    const totalContentWidth = Math.max(...maxRowWidths);
    const totalContentHeight = rowHeights.reduce((a, b) => a + b, 0) + (layerRows - 1) * LAYER_GAP;

    const totalWidth = totalContentWidth + REALM_PADDING * 2;
    const totalHeight = totalContentHeight + REALM_PADDING * 2 + REALM_HEADER;

    realmLayouts.push({ realm, realmDef, subcategories: layerLayouts, totalWidth, totalHeight });
  }

  // ===========================================================================
  // PHASE 3: Position scopes on canvas
  // ===========================================================================

  let maxHeightInRow = 0;      // Track tallest scope in current row
  let realmX = CANVAS_MARGIN;
  let realmY = CANVAS_MARGIN;

  for (const realmLayout of realmLayouts) {
    const { realm, realmDef, subcategories, totalWidth, totalHeight } = realmLayout;

    // Wrap to next row if needed
    if (realmX + totalWidth > MAX_ROW_WIDTH + CANVAS_MARGIN && realmX > CANVAS_MARGIN) {
      realmX = CANVAS_MARGIN;
      realmY += maxHeightInRow + REALM_GAP;
      maxHeightInRow = 0;
    }

    const realmId = `scope-${realm}`;

    // Create scope group node
    nodes.push({
      id: realmId,
      type: 'realmGroup',
      position: { x: realmX, y: realmY },
      width: totalWidth,
      height: totalHeight,
      style: { width: totalWidth, height: totalHeight },
      data: {
        realm,
        label: realmDef.label,
        icon: realmDef.icon,
        nodeCount: subcategories.reduce((sum, s) => sum + s.orderedNodes.length, 0),
      },
    });

    // ===========================================================================
    // PHASE 4: Position subcategories within scope (HORIZONTALLY)
    // ===========================================================================

    const layerColsInScope = Math.min(subcategories.length, MAX_LAYERS_PER_ROW);
    let layerX = REALM_PADDING;
    let layerY = REALM_PADDING + REALM_HEADER;
    let colIndex = 0;
    let rowMaxHeight = 0;

    for (const subcat of subcategories) {
      const layerId = `layer-${realm}-${subcat.layerName}`;

      // Create subcategory group node
      nodes.push({
        id: layerId,
        type: 'layerGroup',
        parentId: realmId,
        extent: 'parent',
        draggable: true,
        position: { x: layerX, y: layerY },
        width: subcat.width,
        height: subcat.height,
        style: { width: subcat.width, height: subcat.height },
        data: {
          realm,
          layer: subcat.layerName,
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
          parentId: layerId,
          extent: 'parent',
          draggable: true,
          position: {
            x: LAYER_PADDING + col * (NODE_WIDTH + NODE_GAP),
            y: LAYER_PADDING + LAYER_HEADER + row * (NODE_HEIGHT + NODE_GAP),
          },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: subcat.layerName,
          },
        });
      });

      // Move to next subcategory position (HORIZONTAL first)
      rowMaxHeight = Math.max(rowMaxHeight, subcat.height);
      colIndex++;

      if (colIndex >= layerColsInScope) {
        // Move to next row (wrap)
        layerX = REALM_PADDING;
        layerY += rowMaxHeight + LAYER_GAP;
        colIndex = 0;
        rowMaxHeight = 0;
      } else {
        // Move to next column (horizontal)
        layerX += subcat.width + LAYER_GAP;
      }
    }

    // Move scope position for next scope
    realmX += totalWidth + REALM_GAP;
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
          x: CANVAS_MARGIN + col * (NODE_WIDTH + NODE_GAP),
          y: CANVAS_MARGIN + row * (NODE_HEIGHT + NODE_GAP),
        },
        data: {
          nodeType: schemaNode.nodeType,
          label: schemaNode.label,
          description: schemaNode.description || '',
          realm: schemaNode.realm,
          layer: schemaNode.layer,
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
        barycenters.set(nodeType, positions.get(nodeType) ?? 0);
      }
    }

    // Sort by barycenter
    order = [...order].sort((a, b) => {
      const ba = barycenters.get(a) ?? positions.get(a) ?? 0;
      const bb = barycenters.get(b) ?? positions.get(b) ?? 0;
      return ba - bb;
    });
  }

  return order;
}
