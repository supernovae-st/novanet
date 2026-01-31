// src/lib/schemaLayouts/stacked.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import {
  SCOPE_CONFIGS,
  NODE_WIDTH,
  NODE_HEIGHT,
  NODE_GAP,
  SUBCAT_GAP,
  SUBCAT_PADDING,
  SUBCAT_HEADER,
  SCOPE_GAP,
  SCOPE_PADDING,
  SCOPE_HEADER,
  MAX_NODES_PER_ROW,
} from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Stacked Layout - Vertical stacked scopes
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
      const rows = Math.ceil(subcatMeta.nodeTypes.length / MAX_NODES_PER_ROW);
      const height = rows * (NODE_HEIGHT + NODE_GAP) + SUBCAT_PADDING * 2 + SUBCAT_HEADER;
      maxSubcatHeight = Math.max(maxSubcatHeight, height);
    }

    // Scope dimensions: content + padding + header
    const scopeHeight = maxSubcatHeight + SCOPE_PADDING * 2 + SCOPE_HEADER;
    const scopeWidth = Math.max(
      4000,
      subcatEntries.length * 1200 + (subcatEntries.length - 1) * SUBCAT_GAP + SCOPE_PADDING * 2
    );

    // Scope group node
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: 0, y: currentY },
      style: { width: scopeWidth, height: scopeHeight },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: hierarchy.stats.nodesByScope[scope] || 0,
      },
    });

    // Layout subcategories side by side
    let subcatX = SCOPE_PADDING;
    const subcatY = SCOPE_PADDING + SCOPE_HEADER;
    const subcatWidth = (scopeWidth - SCOPE_PADDING * 2 - (subcatEntries.length - 1) * SUBCAT_GAP) / subcatEntries.length;

    for (const [subcatName, subcatMeta] of subcatEntries) {
      const subcatId = `subcat-${scope}-${subcatName}`;
      const rows = Math.ceil(subcatMeta.nodeTypes.length / MAX_NODES_PER_ROW);
      const subcatHeight = rows * (NODE_HEIGHT + NODE_GAP) + SUBCAT_PADDING * 2 + SUBCAT_HEADER;

      // Subcategory group
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        draggable: true,
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
      const nodesPerRow = Math.min(
        MAX_NODES_PER_ROW,
        Math.floor((subcatWidth - SUBCAT_PADDING * 2) / (NODE_WIDTH + NODE_GAP))
      );

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
            x: SUBCAT_PADDING + col * (NODE_WIDTH + NODE_GAP),
            y: SUBCAT_PADDING + SUBCAT_HEADER + row * (NODE_HEIGHT + NODE_GAP),
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

      subcatX += subcatWidth + SUBCAT_GAP;
    }

    currentY += scopeHeight + SCOPE_GAP;
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
