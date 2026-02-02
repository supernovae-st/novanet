// src/lib/schemaLayouts/target.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData, LayerMeta } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import {
  NODE_WIDTH,
  NODE_HEIGHT,
  REALM_GAP,
  REALM_PADDING,
  PHI,
  CANVAS_WIDTH,
  CANVAS_HEIGHT,
} from './types';
import type { Realm } from '@novanet/core/types';

/**
 * Target/Bullseye Layout - Concentric rings by realm
 *
 * Uses unified spacing from types.ts (Golden Ratio system).
 * Ring spacing = REALM_GAP × φ for dramatic separation.
 *
 * Visual structure:
 *         ╭───────────────────────╮
 *       ╭─┤   ○ ○ SHARED ○ ○     ├─╮
 *      ╭──│ ╭────────────────╮   │──╮
 *      │  │ │  ● GLOBAL ●   │   │  │
 *      │  │ │ ╭────────────╮│   │  │
 *      │  │ │ │  PROJECT   ││   │  │
 *      │  │ │ ╰────────────╯│   │  │
 *      │  │ ╰────────────────╯   │  │
 *      ╰──│                      │──╯
 *         ╰───────────────────────╯
 */
export function applyTargetLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Derived from unified constants
  const CENTER_X = Math.round(CANVAS_WIDTH / 4);     // Canvas quarter = center
  const CENTER_Y = Math.round(CANVAS_HEIGHT / 4);
  const RING_SPACING = Math.round(REALM_GAP * PHI);  // φ × realm gap between rings
  const MIN_RADIUS = REALM_GAP;                       // Minimum inner radius

  // Realm order from center outward
  const realmOrder: Realm[] = ['project', 'global', 'shared'];

  realmOrder.forEach((realm, ringIndex) => {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) return;

    const realmId = `realm-${realm}`;
    const radius = MIN_RADIUS + ringIndex * RING_SPACING;
    const ringWidth = RING_SPACING - REALM_PADDING;

    // For center (Project), use a circle; for others, use a ring
    if (ringIndex === 0) {
      // Center realm - circular group
      const diameter = radius * 2;
      nodes.push({
        id: realmId,
        type: 'realmGroup',
        position: { x: CENTER_X - radius, y: CENTER_Y - radius },
        style: {
          width: diameter,
          height: diameter,
          borderRadius: '50%',
        },
        data: {
          realm,
          label: realmDef.label,
          icon: realmDef.icon,
          nodeCount: hierarchy.stats.nodesByRealm[realm] || 0,
        },
      });

      // Place nodes in center cluster
      const layerEntries = Object.entries(realmDef.layers)
        .filter(([_, meta]) => meta.nodeTypes.length > 0);

      const allNodes: { nodeType: string; layerName: string; layerMeta: LayerMeta }[] = [];
      for (const [layerName, layerMeta] of layerEntries) {
        for (const nodeType of layerMeta.nodeTypes) {
          allNodes.push({ nodeType, layerName, layerMeta });
        }
      }

      // Arrange in inner circle
      const innerRadius = radius * 0.6;
      allNodes.forEach((item, idx) => {
        const angle = (2 * Math.PI * idx) / allNodes.length - Math.PI / 2;
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === item.nodeType);

        nodes.push({
          id: `schema-${item.nodeType}`,
          type: 'schemaNode',
          parentId: realmId,
          extent: 'parent',
          draggable: true,
          position: {
            x: radius + innerRadius * Math.cos(angle) - NODE_WIDTH / 2,
            y: radius + innerRadius * Math.sin(angle) - NODE_HEIGHT / 2,
          },
          data: {
            nodeType: item.nodeType,
            label: schemaNode?.label || item.nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: item.layerName,
          },
        });
      });
    } else {
      // Outer rings - approximate with large rectangle
      const outerRadius = radius + ringWidth / 2;
      const size = outerRadius * 2 + REALM_PADDING;

      nodes.push({
        id: realmId,
        type: 'realmGroup',
        position: {
          x: CENTER_X - outerRadius - REALM_PADDING / 2,
          y: CENTER_Y - outerRadius - REALM_PADDING / 2,
        },
        style: {
          width: size,
          height: size,
          borderRadius: '50%',
        },
        data: {
          realm,
          label: realmDef.label,
          icon: realmDef.icon,
          nodeCount: hierarchy.stats.nodesByRealm[realm] || 0,
        },
      });

      // Collect all nodes for this realm
      const allNodes: { nodeType: string; layerName: string }[] = [];
      for (const [layerName, layerMeta] of Object.entries(realmDef.layers)) {
        for (const nodeType of layerMeta.nodeTypes) {
          allNodes.push({ nodeType, layerName });
        }
      }

      // Arrange nodes in a ring
      allNodes.forEach((item, idx) => {
        const angle = (2 * Math.PI * idx) / allNodes.length - Math.PI / 2;
        const nodeRadius = radius + ringWidth * 0.3;
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === item.nodeType);

        // Position relative to scope group
        const groupOffset = outerRadius + REALM_PADDING / 2;
        nodes.push({
          id: `schema-${item.nodeType}`,
          type: 'schemaNode',
          parentId: realmId,
          extent: 'parent',
          draggable: true,
          position: {
            x: groupOffset + nodeRadius * Math.cos(angle) - NODE_WIDTH / 2,
            y: groupOffset + nodeRadius * Math.sin(angle) - NODE_HEIGHT / 2,
          },
          data: {
            nodeType: item.nodeType,
            label: schemaNode?.label || item.nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: item.layerName,
          },
        });
      });
    }
  });

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
