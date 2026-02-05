// src/lib/schemaLayouts/swimlanes.ts
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
} from './types';
import type { Realm } from '@novanet/core/types';

/**
 * Swimlanes Layout - Horizontal bands per realm (v10.6: 2 realms)
 *
 * Uses unified spacing from types.ts (Golden Ratio system).
 *
 * Visual structure:
 * ┌─────────────────────────────────────────────────────┐
 * │ TENANT   │ Page ──→ Block ──→ Entity ──→ L10n      │
 * ├──────────┼─────────────────────────────────────────┤
 * │ GLOBAL   │ Locale ──→ Identity ──→ SEO ──→ ...     │
 * └──────────┴─────────────────────────────────────────┘
 */
export function applySwimlaneLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Derived spacing from unified constants
  const NODE_STEP_X = NODE_WIDTH + NODE_GAP;     // Horizontal step between nodes
  // const NODE_STEP_Y = NODE_HEIGHT + NODE_GAP; // Vertical step (reserved for multi-row)

  const realmOrder: Realm[] = ['tenant', 'global'];
  let currentY = 0;

  for (const realm of realmOrder) {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) continue;

    const realmId = `realm-${realm}`;
    const config = REALM_CONFIGS.find(c => c.realm === realm);
    if (!config) {
      console.error(`[swimlanes] Missing config for realm: ${realm}`);
      continue;
    }

    // Collect all nodes for this realm
    const realmNodes: string[] = [];
    for (const [_, layerMeta] of Object.entries(realmDef.layers)) {
      realmNodes.push(...layerMeta.nodeTypes);
    }

    // Lane dimensions from unified constants
    const laneWidth = Math.max(4000, realmNodes.length * NODE_STEP_X + REALM_PADDING * 2);
    const laneHeight = Math.max(
      800,
      NODE_HEIGHT + LAYER_PADDING * 2 + LAYER_HEADER + REALM_PADDING * 2 + REALM_HEADER
    );

    // Realm group node (swimlane)
    nodes.push({
      id: realmId,
      type: 'realmGroup',
      position: { x: 0, y: currentY },
      style: { width: laneWidth, height: laneHeight },
      data: {
        realm,
        label: realmDef.label,
        icon: realmDef.icon,
        nodeCount: realmNodes.length,
      },
    });

    // Layout layers horizontally within the lane
    let currentX = REALM_PADDING;
    const layerY = REALM_PADDING + REALM_HEADER;

    for (const [layerName, layerMeta] of Object.entries(realmDef.layers)) {
      if (layerMeta.nodeTypes.length === 0) continue;

      const layerId = `layer-${realm}-${layerName}`;
      const layerWidth = layerMeta.nodeTypes.length * NODE_STEP_X + LAYER_PADDING * 2;
      const layerHeight = laneHeight - REALM_PADDING * 2 - REALM_HEADER;

      // Layer group
      nodes.push({
        id: layerId,
        type: 'layerGroup',
        parentId: realmId,
        extent: 'parent',
        draggable: true,
        position: { x: currentX, y: layerY },
        style: { width: layerWidth, height: layerHeight },
        data: {
          realm,
          layer: layerName,
          label: layerMeta.label,
          icon: layerMeta.icon,
          nodeCount: layerMeta.nodeTypes.length,
        },
      });

      // Layout nodes horizontally within layer
      let nodeX = LAYER_PADDING;
      const nodeY = LAYER_PADDING + LAYER_HEADER + (layerHeight - LAYER_PADDING * 2 - LAYER_HEADER - NODE_HEIGHT) / 2;

      for (const nodeType of layerMeta.nodeTypes) {
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === nodeType);

        nodes.push({
          id: `schema-${nodeType}`,
          type: 'schemaNode',
          parentId: layerId,
          extent: 'parent',
          draggable: true,
          position: { x: nodeX, y: nodeY },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: layerName,
          },
        });

        nodeX += NODE_STEP_X;
      }

      currentX += layerWidth + LAYER_GAP;
    }

    currentY += laneHeight + REALM_GAP;
  }

  // Create edges
  const validNodeIds = new Set(nodes.map(n => n.id));
  hierarchy.arcs.forEach((edge, index) => {
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
