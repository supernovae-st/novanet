/**
 * Hierarchical Layout - Pure graph visualization without container nodes
 *
 * Generates Realm, Layer, and Kind as regular graph nodes with edges:
 * - Realm nodes (3): project, global, shared
 * - Layer nodes (9): foundation, structure, semantic, instruction, output, config, knowledge, seo, geo
 * - Kind nodes (35): all node types
 * - HAS_LAYER edges (Realm → Layer)
 * - HAS_KIND edges (Layer → Kind)
 * - Business edges (Kind → Kind)
 *
 * Uses Dagre for hierarchical LR layout (left-to-right).
 */

import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import type { Realm } from '@novanet/core/types';
import dagre from '@dagrejs/dagre';

// =============================================================================
// Constants
// =============================================================================

const REALM_CONFIGS: Record<Realm, { label: string; color: string }> = {
  project: { label: 'Project', color: '#8b5cf6' },
  global: { label: 'Global', color: '#10b981' },
  shared: { label: 'Shared', color: '#f59e0b' },
};

const LAYER_CONFIGS: Record<string, { label: string }> = {
  foundation: { label: 'Foundation' },
  structure: { label: 'Structure' },
  semantic: { label: 'Semantic' },
  instruction: { label: 'Instructions' },
  output: { label: 'Output' },
  config: { label: 'Config' },
  knowledge: { label: 'Knowledge' },
  seo: { label: 'SEO' },
  geo: { label: 'GEO' },
};

// Node sizes for Dagre layout
// MetaBadge nodes are compact (like relation badges)
const REALM_NODE_WIDTH = 200;
const REALM_NODE_HEIGHT = 44;
const LAYER_NODE_WIDTH = 200;
const LAYER_NODE_HEIGHT = 44;
const KIND_NODE_WIDTH = 180;
const KIND_NODE_HEIGHT = 90;

// Dagre layout config
const DAGRE_CONFIG = {
  rankdir: 'LR', // Left to right
  ranksep: 300, // Space between columns (Realm → Layer → Kind)
  nodesep: 50, // Space between nodes in same column
  edgesep: 20,
  marginx: 100,
  marginy: 100,
};

// =============================================================================
// Layout Function
// =============================================================================

/**
 * Apply hierarchical layout with Realm → Layer → Kind structure
 *
 * @param hierarchy - Schema hierarchy data from @novanet/core
 * @returns React Flow nodes and edges
 */
export function applyHierarchicalLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Initialize Dagre graph
  const g = new dagre.graphlib.Graph();
  g.setGraph(DAGRE_CONFIG);
  g.setDefaultEdgeLabel(() => ({}));

  // Track layer → realm mapping for edge creation
  const layerToRealm: Record<string, Realm> = {};

  // ==========================================================================
  // 1. Create Realm Nodes (3)
  // ==========================================================================

  for (const [realmKey, realmData] of Object.entries(hierarchy.realms)) {
    const realm = realmKey as Realm;
    const config = REALM_CONFIGS[realm];
    const nodeId = `meta-realm-${realm}`;

    // Add to Dagre for layout calculation
    g.setNode(nodeId, { width: REALM_NODE_WIDTH, height: REALM_NODE_HEIGHT });

    // Count types in this realm
    let realmTypeCount = 0;
    for (const layerData of Object.values(realmData.layers)) {
      realmTypeCount += layerData.nodeTypes.length;
    }

    // Create React Flow node (will position later)
    // Use metaBadge type for compact display
    nodes.push({
      id: nodeId,
      type: 'metaBadge',
      position: { x: 0, y: 0 }, // Will be set by Dagre
      data: {
        label: config.label,
        description: `${realmData.label} realm - contains ${Object.keys(realmData.layers).length} layers`,
        metaType: 'realm',
        color: config.color,
        typeCount: realmTypeCount,
        realmKey: realm, // For icon selection
      },
    });

    // ==========================================================================
    // 2. Create Layer Nodes (9) for this Realm
    // ==========================================================================

    for (const [layerKey, layerData] of Object.entries(realmData.layers)) {
      const layerConfig = LAYER_CONFIGS[layerKey];
      if (!layerConfig) continue;

      const layerNodeId = `meta-layer-${realm}-${layerKey}`;
      layerToRealm[layerKey] = realm;

      // Add to Dagre
      g.setNode(layerNodeId, { width: LAYER_NODE_WIDTH, height: LAYER_NODE_HEIGHT });

      // Create HAS_LAYER edge (Realm → Layer)
      g.setEdge(nodeId, layerNodeId);
      edges.push({
        id: `edge-has-layer-${realm}-${layerKey}`,
        source: nodeId,
        target: layerNodeId,
        type: 'floating',
        data: {
          relationType: 'HAS_LAYER',
          label: 'HAS_LAYER',
          isMetaEdge: true,
        },
      });

      // Create React Flow node
      // Use metaBadge type for compact display
      nodes.push({
        id: layerNodeId,
        type: 'metaBadge',
        position: { x: 0, y: 0 },
        data: {
          label: layerConfig.label,
          description: `${layerData.label} - ${layerData.nodeTypes.length} node types`,
          metaType: 'layer',
          color: REALM_CONFIGS[realm].color,
          typeCount: layerData.nodeTypes.length,
          realmKey: realm,
          layerKey: layerKey, // For icon selection
        },
      });

      // ==========================================================================
      // 3. Create Kind Nodes (35) and HAS_KIND edges
      // ==========================================================================

      for (const nodeType of layerData.nodeTypes) {
        const kindNodeId = `schema-${nodeType}`;
        const schemaNode = hierarchy.nodes.find((n) => n.nodeType === nodeType);

        // Add to Dagre
        g.setNode(kindNodeId, { width: KIND_NODE_WIDTH, height: KIND_NODE_HEIGHT });

        // Create HAS_KIND edge (Layer → Kind)
        g.setEdge(layerNodeId, kindNodeId);
        edges.push({
          id: `edge-has-kind-${layerKey}-${nodeType}`,
          source: layerNodeId,
          target: kindNodeId,
          type: 'floating',
          data: {
            relationType: 'HAS_KIND',
            label: 'HAS_KIND',
            isMetaEdge: true,
          },
        });

        // Create React Flow node
        nodes.push({
          id: kindNodeId,
          type: 'schemaNode',
          position: { x: 0, y: 0 },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: layerKey,
            trait: schemaNode?.trait || 'invariant',
            isMetaNode: false,
            metaType: 'kind',
          },
        });
      }
    }
  }

  // ==========================================================================
  // 4. Add Business Edges (Kind → Kind) - existing schema edges
  // ==========================================================================

  const validNodeIds = new Set(nodes.map((n) => n.id));

  for (const [index, edge] of hierarchy.edges.entries()) {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    // Only add edges between existing nodes
    if (validNodeIds.has(sourceId) && validNodeIds.has(targetId)) {
      // Add to Dagre for layout consideration
      g.setEdge(sourceId, targetId);

      edges.push({
        id: `edge-business-${index}`,
        source: sourceId,
        target: targetId,
        type: 'floating',
        data: {
          relationType: edge.relationType,
          label: edge.label,
          isMetaEdge: false,
        },
      });
    }
  }

  // ==========================================================================
  // 5. Run Dagre Layout
  // ==========================================================================

  dagre.layout(g);

  // Apply calculated positions to nodes
  for (const node of nodes) {
    const dagreNode = g.node(node.id);
    if (dagreNode) {
      // Dagre gives center position, convert to top-left
      node.position = {
        x: dagreNode.x - (dagreNode.width ?? 0) / 2,
        y: dagreNode.y - (dagreNode.height ?? 0) / 2,
      };
    }
  }

  return { nodes, edges };
}
