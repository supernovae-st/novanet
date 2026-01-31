// src/lib/schemaLayouts/forceClusters.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT, GROUP_PADDING } from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Force Clusters Layout - Physics-based with scope clustering
 *
 * Uses a simple force simulation to cluster nodes by scope
 * with repulsion between different scopes.
 */
export function applyForceClusterLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Cluster centers for each scope
  const CLUSTER_CENTERS: Record<Scope, { x: number; y: number }> = {
    Project: { x: 600, y: 600 },
    Global: { x: 1400, y: 400 },
    Shared: { x: 1000, y: 1000 },
  };

  const CLUSTER_RADIUS = 400;
  const NODE_REPULSION = 150;

  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];

  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const scopeId = `scope-${scope}`;
    const center = CLUSTER_CENTERS[scope];

    // Collect all nodes for this scope
    const allNodes: { nodeType: string; subcatName: string }[] = [];
    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      for (const nodeType of subcatMeta.nodeTypes) {
        allNodes.push({ nodeType, subcatName });
      }
    }

    // Simple force simulation - place nodes in expanding spiral
    const nodePositions: { nodeType: string; x: number; y: number; subcatName: string }[] = [];

    allNodes.forEach((item, idx) => {
      // Golden angle spiral for even distribution
      const goldenAngle = Math.PI * (3 - Math.sqrt(5));
      const angle = idx * goldenAngle;
      const radius = Math.sqrt(idx + 1) * NODE_REPULSION * 0.5;

      nodePositions.push({
        nodeType: item.nodeType,
        x: center.x + radius * Math.cos(angle),
        y: center.y + radius * Math.sin(angle),
        subcatName: item.subcatName,
      });
    });

    // Calculate bounding box for scope group
    if (nodePositions.length > 0) {
      const minX = Math.min(...nodePositions.map(p => p.x)) - GROUP_PADDING - NODE_WIDTH / 2;
      const maxX = Math.max(...nodePositions.map(p => p.x)) + GROUP_PADDING + NODE_WIDTH / 2;
      const minY = Math.min(...nodePositions.map(p => p.y)) - GROUP_PADDING - NODE_HEIGHT / 2;
      const maxY = Math.max(...nodePositions.map(p => p.y)) + GROUP_PADDING + NODE_HEIGHT / 2;

      // Scope group node
      nodes.push({
        id: scopeId,
        type: 'scopeGroup',
        position: { x: minX, y: minY },
        style: { width: maxX - minX, height: maxY - minY },
        data: {
          scope,
          label: scopeDef.label,
          icon: scopeDef.icon,
          nodeCount: allNodes.length,
        },
      });

      // Create schema nodes (relative to scope group)
      for (const pos of nodePositions) {
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === pos.nodeType);

        nodes.push({
          id: `schema-${pos.nodeType}`,
          type: 'schemaNode',
          parentId: scopeId,
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
            scope,
            subcategory: pos.subcatName,
          },
        });
      }
    }
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
