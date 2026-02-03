// src/lib/schemaLayouts/forceClusters.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { NODE_WIDTH, NODE_HEIGHT, REALM_PADDING, NODE_GAP, REALM_GAP } from './types';
import type { Realm } from '@novanet/core/types';

/**
 * Force Clusters Layout - Physics-based with realm clustering
 *
 * Uses unified spacing from types.ts (Golden Ratio system).
 * Cluster centers derived from REALM_GAP; spiral spacing from NODE_GAP.
 *
 * Visual structure (triangular cluster arrangement):
 *
 *      [PROJECT]          [GLOBAL]
 *            \            /
 *             \          /
 *              [SHARED]
 */
export function applyForceClusterLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Derived from unified constants
  const BASE_OFFSET = REALM_GAP * 2;               // ~1100px from origin
  const CLUSTER_SPACING = REALM_GAP * 3;            // ~1650px between cluster centers
  const CLUSTER_CENTERS: Record<Realm, { x: number; y: number }> = {
    project: { x: BASE_OFFSET, y: BASE_OFFSET },
    global:  { x: BASE_OFFSET + CLUSTER_SPACING, y: BASE_OFFSET - CLUSTER_SPACING * 0.3 },
    shared:  { x: BASE_OFFSET + CLUSTER_SPACING / 2, y: BASE_OFFSET + CLUSTER_SPACING * 0.7 },
  };

  const NODE_REPULSION = NODE_GAP * 1.5;            // ~120px spiral spacing

  const realmOrder: Realm[] = ['project', 'global', 'shared'];

  for (const realm of realmOrder) {
    const realmDef = hierarchy.realms[realm];
    if (!realmDef) continue;

    const realmId = `realm-${realm}`;
    const center = CLUSTER_CENTERS[realm];

    // Collect all nodes for this realm
    const allNodes: { nodeType: string; layerName: string }[] = [];
    for (const [layerName, layerMeta] of Object.entries(realmDef.layers)) {
      for (const nodeType of layerMeta.nodeTypes) {
        allNodes.push({ nodeType, layerName });
      }
    }

    // Simple force simulation - place nodes in expanding spiral
    const nodePositions: { nodeType: string; x: number; y: number; layerName: string }[] = [];

    allNodes.forEach((item, idx) => {
      // Golden angle spiral for even distribution
      const goldenAngle = Math.PI * (3 - Math.sqrt(5));
      const angle = idx * goldenAngle;
      const radius = Math.sqrt(idx + 1) * NODE_REPULSION * 0.5;

      nodePositions.push({
        nodeType: item.nodeType,
        x: center.x + radius * Math.cos(angle),
        y: center.y + radius * Math.sin(angle),
        layerName: item.layerName,
      });
    });

    // Calculate bounding box for realm group
    if (nodePositions.length > 0) {
      const minX = Math.min(...nodePositions.map(p => p.x)) - REALM_PADDING - NODE_WIDTH / 2;
      const maxX = Math.max(...nodePositions.map(p => p.x)) + REALM_PADDING + NODE_WIDTH / 2;
      const minY = Math.min(...nodePositions.map(p => p.y)) - REALM_PADDING - NODE_HEIGHT / 2;
      const maxY = Math.max(...nodePositions.map(p => p.y)) + REALM_PADDING + NODE_HEIGHT / 2;

      // Realm group node
      nodes.push({
        id: realmId,
        type: 'realmGroup',
        position: { x: minX, y: minY },
        style: { width: maxX - minX, height: maxY - minY },
        data: {
          realm,
          label: realmDef.label,
          icon: realmDef.icon,
          nodeCount: allNodes.length,
        },
      });

      // Create schema nodes (relative to realm group)
      for (const pos of nodePositions) {
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === pos.nodeType);

        nodes.push({
          id: `schema-${pos.nodeType}`,
          type: 'schemaNode',
          parentId: realmId,
          extent: 'parent',
          draggable: true,
          position: {
            x: pos.x - minX - NODE_WIDTH / 2,
            y: pos.y - minY - NODE_HEIGHT / 2,
          },
          data: {
            nodeType: pos.nodeType,
            label: schemaNode?.label || pos.nodeType,
            description: schemaNode?.description || '',
            realm,
            layer: pos.layerName,
          },
        });
      }
    }
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
