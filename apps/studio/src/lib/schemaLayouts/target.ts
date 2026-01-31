// src/lib/schemaLayouts/target.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData, SubcategoryMeta } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT, SCOPE_GAP, PHI } from './types';
import type { Scope } from '@novanet/core/types';

/**
 * Target/Bullseye Layout - Concentric rings by scope
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

  // Using Golden Ratio spacing: rings expand by φ factor
  const CENTER_X = 1600;
  const CENTER_Y = 1200;
  const RING_SPACING = Math.floor(SCOPE_GAP * PHI);  // ~474px between rings (was 350)
  const MIN_RADIUS = SCOPE_GAP;                       // 293px minimum radius (was 200)

  // Scope order from center outward
  const scopeOrder: Scope[] = ['Project', 'Global', 'Shared'];

  scopeOrder.forEach((scope, ringIndex) => {
    const scopeDef = hierarchy.scopes[scope];
    if (!scopeDef) return;

    const scopeId = `scope-${scope}`;
    const radius = MIN_RADIUS + ringIndex * RING_SPACING;
    const ringWidth = RING_SPACING - 50;

    // For center (Project), use a circle; for others, use a ring
    if (ringIndex === 0) {
      // Center scope - circular group
      const diameter = radius * 2;
      nodes.push({
        id: scopeId,
        type: 'scopeGroup',
        position: { x: CENTER_X - radius, y: CENTER_Y - radius },
        style: {
          width: diameter,
          height: diameter,
          borderRadius: '50%',
        },
        data: {
          scope,
          label: scopeDef.label,
          icon: scopeDef.icon,
          nodeCount: hierarchy.stats.nodesByScope[scope] || 0,
        },
      });

      // Place nodes in center cluster
      const subcatEntries = Object.entries(scopeDef.subcategories)
        .filter(([_, meta]) => meta.nodeTypes.length > 0);

      let allNodes: { nodeType: string; subcatName: string; subcatMeta: SubcategoryMeta }[] = [];
      for (const [subcatName, subcatMeta] of subcatEntries) {
        for (const nodeType of subcatMeta.nodeTypes) {
          allNodes.push({ nodeType, subcatName, subcatMeta });
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
          parentId: scopeId,
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
            scope,
            subcategory: item.subcatName,
          },
        });
      });
    } else {
      // Outer rings - approximate with large rectangle
      const outerRadius = radius + ringWidth / 2;
      const size = outerRadius * 2 + 100;

      nodes.push({
        id: scopeId,
        type: 'scopeGroup',
        position: { x: CENTER_X - outerRadius - 50, y: CENTER_Y - outerRadius - 50 },
        style: {
          width: size,
          height: size,
          borderRadius: '50%',
        },
        data: {
          scope,
          label: scopeDef.label,
          icon: scopeDef.icon,
          nodeCount: hierarchy.stats.nodesByScope[scope] || 0,
        },
      });

      // Collect all nodes for this scope
      const allNodes: { nodeType: string; subcatName: string }[] = [];
      for (const [subcatName, subcatMeta] of Object.entries(scopeDef.subcategories)) {
        for (const nodeType of subcatMeta.nodeTypes) {
          allNodes.push({ nodeType, subcatName });
        }
      }

      // Arrange nodes in a ring
      allNodes.forEach((item, idx) => {
        const angle = (2 * Math.PI * idx) / allNodes.length - Math.PI / 2;
        const nodeRadius = radius + ringWidth * 0.3;
        const schemaNode = hierarchy.nodes.find(n => n.nodeType === item.nodeType);

        // Position relative to scope group
        const groupOffset = outerRadius + 50;
        nodes.push({
          id: `schema-${item.nodeType}`,
          type: 'schemaNode',
          parentId: scopeId,
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
            scope,
            subcategory: item.subcatName,
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
