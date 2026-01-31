// src/lib/schemaLayouts/swimlanes.ts
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
} from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Swimlanes Layout - Horizontal bands per scope
 *
 * Uses unified spacing from types.ts (Golden Ratio system).
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

  // Derived spacing from unified constants
  const NODE_STEP_X = NODE_WIDTH + NODE_GAP;     // Horizontal step between nodes
  const NODE_STEP_Y = NODE_HEIGHT + NODE_GAP;    // Vertical step between nodes

  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];
  let currentY = 0;

  for (const scope of scopeOrder) {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) continue;

    const scopeId = `scope-${scope}`;
    const config = SCOPE_CONFIGS.find(c => c.scope === scope);
    if (!config) {
      console.error(`[swimlanes] Missing config for scope: ${scope}`);
      continue;
    }

    // Collect all nodes for this scope
    const scopeNodes: string[] = [];
    for (const [_, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      scopeNodes.push(...subcatMeta.nodeTypes);
    }

    // Lane dimensions from unified constants
    const laneWidth = Math.max(4000, scopeNodes.length * NODE_STEP_X + SCOPE_PADDING * 2);
    const laneHeight = Math.max(
      800,
      NODE_HEIGHT + SUBCAT_PADDING * 2 + SUBCAT_HEADER + SCOPE_PADDING * 2 + SCOPE_HEADER
    );

    // Scope group node (swimlane)
    nodes.push({
      id: scopeId,
      type: 'scopeGroup',
      position: { x: 0, y: currentY },
      style: { width: laneWidth, height: laneHeight },
      data: {
        scope,
        label: scopeDef.label,
        icon: scopeDef.icon,
        nodeCount: scopeNodes.length,
      },
    });

    // Layout subcategories horizontally within the lane
    let currentX = SCOPE_PADDING;
    let subcatY = SCOPE_PADDING + SCOPE_HEADER;

    for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
      if (subcatMeta.nodeTypes.length === 0) continue;

      const subcatId = `subcat-${scope}-${subcatName}`;
      const subcatWidth = subcatMeta.nodeTypes.length * NODE_STEP_X + SUBCAT_PADDING * 2;
      const subcatHeight = laneHeight - SCOPE_PADDING * 2 - SCOPE_HEADER;

      // Subcategory group
      nodes.push({
        id: subcatId,
        type: 'subcategoryGroup',
        parentId: scopeId,
        extent: 'parent',
        draggable: true,
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
      let nodeX = SUBCAT_PADDING;
      const nodeY = SUBCAT_PADDING + SUBCAT_HEADER + (subcatHeight - SUBCAT_PADDING * 2 - SUBCAT_HEADER - NODE_HEIGHT) / 2;

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

        nodeX += NODE_STEP_X;
      }

      currentX += subcatWidth + SUBCAT_GAP;
    }

    currentY += laneHeight + SCOPE_GAP;
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
