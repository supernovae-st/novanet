// src/lib/schemaLayouts/stacked.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import {
  SCOPE_CONFIGS,
  NODE_WIDTH,
  NODE_HEIGHT,
  GROUP_PADDING,
  NODE_GAP,
  SCOPE_GAP,
  SUBCAT_GAP,
} from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Stacked Layout - Vertical stacked scopes
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
 *              ↓
 * ┌─────────────────────────────┐
 * │          SHARED             │
 * └─────────────────────────────┘
 */
export function applyStackedLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Using Golden Ratio spacing system
  const SCOPE_WIDTH = 3000;            // Wider to accommodate larger spacing
  const SCOPE_MARGIN = SCOPE_GAP;      // 293px between scopes (was 80)
  const SUBCAT_MARGIN = SUBCAT_GAP;    // 181px between subcategories (was 30)
  const NODE_SPACING = NODE_GAP;       // 112px between nodes (was 30)
  const NODES_PER_ROW = 5;             // Fewer per row for wider spacing (was 8)

  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];
  let currentY = 0;

  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const scopeId = `scope-${scope}`;

    // Calculate scope height based on content
    let maxSubcatHeight = 0;
    const subcatEntries = Object.entries(scopeDef.subcategories)
      .filter(([_, meta]) => meta.nodeTypes.length > 0);

    for (const [_, subcatMeta] of subcatEntries) {
      const rows = Math.ceil(subcatMeta.nodeTypes.length / NODES_PER_ROW);
      const height = rows * (NODE_HEIGHT + NODE_SPACING) + GROUP_PADDING * 2;
      maxSubcatHeight = Math.max(maxSubcatHeight, height);
    }

    const scopeHeight = maxSubcatHeight + GROUP_PADDING * 2 + 40; // Extra for label

    // Scope group node
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: 0, y: currentY },
      style: { width: SCOPE_WIDTH, height: scopeHeight },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: hierarchy.stats.nodesByScope[scope] || 0,
      },
    });

    // Layout subcategories side by side
    let subcatX = GROUP_PADDING;
    const subcatY = GROUP_PADDING + 20; // Below label
    const subcatWidth = (SCOPE_WIDTH - GROUP_PADDING * 2 - (subcatEntries.length - 1) * SUBCAT_MARGIN) / subcatEntries.length;

    for (const [subcatName, subcatMeta] of subcatEntries) {
      const subcatId = `subcat-${scope}-${subcatName}`;
      const rows = Math.ceil(subcatMeta.nodeTypes.length / NODES_PER_ROW);
      const subcatHeight = rows * (NODE_HEIGHT + NODE_SPACING) + GROUP_PADDING;

      // Subcategory group
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: subcatX, y: subcatY },
        style: { width: subcatWidth, height: subcatHeight },
        data: {
          scope,
          subcategory: subcatName,
          label: subcatMeta.label,
          icon: subcatMeta.icon,
          nodeCount: subcatMeta.nodeTypes.length,
        },
      });

      // Layout nodes in grid
      const nodesPerRow = Math.min(NODES_PER_ROW, Math.floor((subcatWidth - GROUP_PADDING) / (NODE_WIDTH + NODE_SPACING)));

      subcatMeta.nodeTypes.forEach((nodeType, idx) => {
        const row = Math.floor(idx / nodesPerRow);
        const col = idx % nodesPerRow;
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === nodeType);

        nodes.push({
          id: `schema-${nodeType}`,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: true,
          position: {
            x: GROUP_PADDING / 2 + col * (NODE_WIDTH + NODE_SPACING),
            y: GROUP_PADDING / 2 + row * (NODE_HEIGHT + NODE_SPACING),
          },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            scope,
            subcategory: subcatName,
          },
        });
      });

      subcatX += subcatWidth + SUBCAT_MARGIN;
    }

    currentY += scopeHeight + SCOPE_MARGIN;
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
