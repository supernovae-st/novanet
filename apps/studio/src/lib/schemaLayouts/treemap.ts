// src/lib/schemaLayouts/treemap.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult, ScopeConfig } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT, GROUP_PADDING } from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Treemap Layout - Rectangles proportional to node count
 *
 * Visual structure:
 * ┌────────────────────────────────────────┐
 * │ Global (15 nodes)          │ Project   │
 * │ ┌──────┬──────┬──────────┐ │ (14)      │
 * │ │Locale│Know  │Rules     │ │┌────┬────┐│
 * │ │      │ledge │          │ ││Core│L10n││
 * │ └──────┴──────┴──────────┘ │└────┴────┘│
 * ├────────────────────────────┴───────────┤
 * │ Shared (6 nodes)                       │
 * └────────────────────────────────────────┘
 */
export function applyTreemapLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Calculate total nodes per scope for proportional sizing
  const scopeNodeCounts = new Map<Scope, number>();
  let totalNodes = 0;

  for (const scope of ['Project', 'Global', 'Shared'] as Scope[]) {
    const count = hierarchy.stats.nodesByScope[scope] || 0;
    scopeNodeCounts.set(scope, count);
    totalNodes += count;
  }

  // Canvas dimensions
  const CANVAS_WIDTH = 2400;
  const CANVAS_HEIGHT = 1600;
  const MARGIN = 40;

  // Sort scopes by node count (largest first for better treemap)
  const sortedScopes = [...scopeNodeCounts.entries()]
    .sort((a, b) => b[1] - a[1])
    .map(([scope]) => scope);

  // Simple treemap: split horizontally for first 2, then vertically
  let currentX = MARGIN;
  let currentY = MARGIN;
  const availableWidth = CANVAS_WIDTH - MARGIN * 2;
  const availableHeight = CANVAS_HEIGHT - MARGIN * 2;

  // Calculate scope rectangles using squarified treemap algorithm (simplified)
  const scopeRects: Map<Scope, { x: number; y: number; width: number; height: number }> = new Map();

  if (sortedScopes.length >= 2) {
    const firstScopeRatio = scopeNodeCounts.get(sortedScopes[0])! / totalNodes;
    const secondScopeRatio = scopeNodeCounts.get(sortedScopes[1])! / totalNodes;
    const thirdScopeRatio = sortedScopes[2] ? scopeNodeCounts.get(sortedScopes[2])! / totalNodes : 0;

    // Top row: first two scopes side by side
    const topHeight = availableHeight * (1 - thirdScopeRatio);
    const firstWidth = availableWidth * (firstScopeRatio / (firstScopeRatio + secondScopeRatio));

    scopeRects.set(sortedScopes[0], {
      x: currentX,
      y: currentY,
      width: firstWidth - MARGIN / 2,
      height: topHeight - MARGIN / 2,
    });

    scopeRects.set(sortedScopes[1], {
      x: currentX + firstWidth + MARGIN / 2,
      y: currentY,
      width: availableWidth - firstWidth - MARGIN / 2,
      height: topHeight - MARGIN / 2,
    });

    // Bottom row: third scope full width
    if (sortedScopes[2]) {
      scopeRects.set(sortedScopes[2], {
        x: currentX,
        y: currentY + topHeight + MARGIN / 2,
        width: availableWidth,
        height: availableHeight - topHeight - MARGIN / 2,
      });
    }
  }

  // Create scope group nodes and their children
  for (const [scope, rect] of scopeRects) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const scopeId = `scope-${scope}`;
    const config = SCOPE_CONFIGS.find(c => c.scope === scope)!;

    // Scope group node
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: rect.x, y: rect.y },
      style: { width: rect.width, height: rect.height },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: scopeNodeCounts.get(scope) || 0,
      },
    });

    // Layout subcategories within scope using nested treemap
    const subcatEntries = Object.entries(scopeDef.subcategories)
      .filter(([_, meta]) => meta.nodeTypes.length > 0)
      .sort((a, b) => b[1].nodeTypes.length - a[1].nodeTypes.length);

    const innerPadding = GROUP_PADDING;
    const innerWidth = rect.width - innerPadding * 2;
    const innerHeight = rect.height - innerPadding * 2;

    let subcatY = innerPadding;
    const subcatHeight = (innerHeight - (subcatEntries.length - 1) * 20) / subcatEntries.length;

    for (const [subcatName, subcatMeta] of subcatEntries) {
      const subcatId = `subcat-${scope}-${subcatName}`;

      // Subcategory group node (relative to parent)
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: innerPadding, y: subcatY },
        style: { width: innerWidth, height: subcatHeight },
        data: {
          scope,
          subcategory: subcatName,
          label: subcatMeta.label,
          icon: subcatMeta.icon,
          nodeCount: subcatMeta.nodeTypes.length,
        },
      });

      // Layout nodes within subcategory in a grid
      const nodeInnerPadding = 30;
      const nodeSpacing = 20;
      const nodesPerRow = Math.max(1, Math.floor((innerWidth - nodeInnerPadding * 2) / (NODE_WIDTH + nodeSpacing)));

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
            x: nodeInnerPadding + col * (NODE_WIDTH + nodeSpacing),
            y: nodeInnerPadding + row * (NODE_HEIGHT + nodeSpacing),
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

      subcatY += subcatHeight + 20;
    }
  }

  // Fallback: if no nodes created from hierarchy (broken scopes), create nodes directly
  if (nodes.length === 0 && hierarchy.nodes.length > 0) {
    const FALLBACK_SPACING = 200;
    const FALLBACK_COLS = 6;

    hierarchy.nodes.forEach((schemaNode, idx) => {
      const col = idx % FALLBACK_COLS;
      const row = Math.floor(idx / FALLBACK_COLS);

      nodes.push({
        id: `schema-${schemaNode.nodeType}`,
        type: 'schemaNode',
        draggable: true,
        position: {
          x: 50 + col * FALLBACK_SPACING,
          y: 50 + row * FALLBACK_SPACING,
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
