// src/lib/schemaLayouts/swimlanes.ts
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
} from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Swimlanes Layout - Horizontal bands per scope
 *
 * Visual structure:
 * ┌─────────────────────────────────────────────────────┐
 * │ PROJECT  │ Page ──→ Block ──→ Concept ──→ L10n     │
 * ├──────────┼─────────────────────────────────────────┤
 * │ GLOBAL   │ Locale ──→ Identity ──→ Voice ──→ ...  │
 * ├──────────┼─────────────────────────────────────────┤
 * │ SHARED   │ SEO ──→ GEO ──→ Mining                  │
 * └──────────┴─────────────────────────────────────────┘
 */
export function applySwimlaneLayout(
  hierarchy: HierarchicalSchemaData
): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  // Using Golden Ratio spacing system
  const LANE_HEIGHT = 500;                        // Taller lanes for better visibility
  const LANE_MARGIN = SCOPE_GAP;                  // 293px between lanes (was 40)
  const NODE_SPACING_X = NODE_WIDTH + NODE_GAP;   // 252px horizontal (was 200)
  const NODE_SPACING_Y = NODE_HEIGHT + NODE_GAP;  // 162px vertical (was 100)

  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];
  let currentY = 0;

  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const scopeId = `scope-${scope}`;
    const config = SCOPE_CONFIGS.find(c => c.scope === scope)!;

    // Collect all nodes for this scope
    const scopeNodes: string[] = [];
    for (const [_, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      scopeNodes.push(...subcatMeta.nodeTypes);
    }

    const laneWidth = Math.max(1200, scopeNodes.length * NODE_SPACING_X + GROUP_PADDING * 2);

    // Scope group node (swimlane)
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: 0, y: currentY },
      style: { width: laneWidth, height: LANE_HEIGHT },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: scopeNodes.length,
      },
    });

    // Layout subcategories horizontally within the lane
    let currentX = GROUP_PADDING;
    let subcatY = GROUP_PADDING;

    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      if (subcatMeta.nodeTypes.length === 0) continue;

      const subcatId = `subcat-${scope}-${subcatName}`;
      const subcatWidth = subcatMeta.nodeTypes.length * NODE_SPACING_X + GROUP_PADDING;
      const subcatHeight = LANE_HEIGHT - GROUP_PADDING * 2;

      // Subcategory group
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        position: { x: currentX, y: subcatY },
        style: { width: subcatWidth, height: subcatHeight },
        data: {
          scope,
          subcategory: subcatName,
          label: subcatMeta.label,
          icon: subcatMeta.icon,
          nodeCount: subcatMeta.nodeTypes.length,
        },
      });

      // Layout nodes horizontally within subcategory
      let nodeX = GROUP_PADDING / 2;
      const nodeY = (subcatHeight - NODE_HEIGHT) / 2;

      for (const nodeType of subcatMeta.nodeTypes) {
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === nodeType);

        nodes.push({
          id: `schema-${nodeType}`,
          type: 'schemaNode',
          parentId: subcatId,
          extent: 'parent',
          draggable: true,
          position: { x: nodeX, y: nodeY },
          data: {
            nodeType,
            label: schemaNode?.label || nodeType,
            description: schemaNode?.description || '',
            scope,
            subcategory: subcatName,
          },
        });

        nodeX += NODE_SPACING_X;
      }

      currentX += subcatWidth + LANE_MARGIN;
    }

    currentY += LANE_HEIGHT + LANE_MARGIN;
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
