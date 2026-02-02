// src/lib/schemaLayouts/stacked.ts
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
  MAX_NODES_PER_ROW,
} from './types';
import type { Realm } from '@novanet/core/types';

/**
 * Stacked Layout - Vertical stacked realms
 *
 * Uses unified spacing from types.ts (Golden Ratio system).
 *
 * Visual structure:
 * ┌─────────────────────────────┐
 * │         PROJECT             │
 * │  ┌─────┐ ┌─────┐ ┌─────┐   │
 * │  │Node │ │Node │ │Node │   │
 * │  └─────┘ └─────┘ └─────┘   │
 * └─────────────────────────────┘
 *              ↓
 * ┌─────────────────────────────┐
 * │          GLOBAL             │
 * │  ┌─────┐ ┌─────┐ ┌─────┐   │
 * │  │Node │ │Node │ │Node │   │
 * │  └─────┘ └─────┘ └─────┘   │
 * └─────────────────────────────┘
 */
export function applyStackedLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  const realmOrder: Realm[] = ['project', 'global', 'shared'];
  let currentY = 0;

  for (const realm of realmOrder) {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) continue;

    const realmId = `scope-${realm}`;

    // Calculate scope height based on content
    let maxLayerHeight = 0;
    const layerEntries = Object.entries(realmDef.layers)
      .filter(([_, meta]) => meta.nodeTypes.length > 0);

    for (const [_, layerMeta] of layerEntries) {
      const rows = Math.ceil(layerMeta.nodeTypes.length / MAX_NODES_PER_ROW);
      const height = rows * (NODE_HEIGHT + NODE_GAP) + LAYER_PADDING * 2 + LAYER_HEADER;
      maxLayerHeight = Math.max(maxLayerHeight, height);
    }

    // Scope dimensions: content + padding + header
    const realmHeight = maxLayerHeight + REALM_PADDING * 2 + REALM_HEADER;
    const realmWidth = Math.max(
      4000,
      layerEntries.length * 1200 + (layerEntries.length - 1) * LAYER_GAP + REALM_PADDING * 2
    );

    // Scope group node
    nodes.push({
      id: realmId,
      type: 'realmGroup',
      position: { x: 0, y: currentY },
      style: { width: realmWidth, height: realmHeight },
      data: {
        realm,
        label: realmDef.label,
        icon: realmDef.icon,
        nodeCount: hierarchy.stats.nodesByRealm[realm] || 0,
      },
    });

    // Layout subcategories side by side
    let layerX = REALM_PADDING;
    const layerY = REALM_PADDING + REALM_HEADER;
    const layerWidth = (realmWidth - REALM_PADDING * 2 - (layerEntries.length - 1) * LAYER_GAP) / layerEntries.length;

    for (const [layerName, layerMeta] of layerEntries) {
      const layerId = `subcat-${realm}-${layerName}`;
      const rows = Math.ceil(layerMeta.nodeTypes.length / MAX_NODES_PER_ROW);
      const layerHeight = rows * (NODE_HEIGHT + NODE_GAP) + LAYER_PADDING * 2 + LAYER_HEADER;

      // Subcategory group
      nodes.push({
        id: layerId,
        type: 'layerGroup',
        parentId: realmId,
        extent: 'parent',
        draggable: true,
        position: { x: layerX, y: layerY },
        style: { width: layerWidth, height: layerHeight },
        data: {
          realm,
          layer: layerName,
          label: layerMeta.label,
          icon: layerMeta.icon,
          nodeCount: layerMeta.nodeTypes.length,
        },
      });

      // Layout nodes in grid
      const nodesPerRow = Math.min(
        MAX_NODES_PER_ROW,
        Math.floor((layerWidth - LAYER_PADDING * 2) / (NODE_WIDTH + NODE_GAP))
      );

      layerMeta.nodeTypes.forEach((nodeType, idx) => {
        const row = Math.floor(idx / nodesPerRow);
        const col = idx % nodesPerRow;
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
            layer: layerName,
          },
        });
      });

      layerX += layerWidth + LAYER_GAP;
    }

    currentY += realmHeight + REALM_GAP;
  }

  // Create edges
  const validNodeIds = new Set(nodes.map(n => n.id));
  hierarchy.edges.forEach((edge, index) => {
    const sourceId = `schema-${edge.sourceType}`;
    const targetId = `schema-${edge.targetType}`;

    if (validNodeIds.has(sourceId) && validNodeIds.has(targetId)) {
      edges.push({
        id: `edge-${index}`,
        source: sourceId,
        target: targetId,
        type: 'floating',
        data: {
          relationType: edge.relationType,
          label: edge.label,
        },
      });
    }
  });

  return { nodes, edges };
}
