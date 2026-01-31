# Schema View Layouts Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement 5 specialized layouts for NovaNet schema visualization (Treemap, Swimlanes, Stacked, Target, Force) accessible via existing toolbar buttons.

**Architecture:** Create a new `schemaLayouts.ts` module with pure layout functions for each algorithm. Modify `schemaLayoutELK.ts` to dispatch to the appropriate layout based on `layoutDirection`. Each layout receives hierarchical schema data and returns React Flow nodes/edges with proper parent-child relationships.

**Tech Stack:** TypeScript, React Flow (@xyflow/react), ELK.js (for some layouts), custom algorithms (Treemap, Target)

---

## Layout Mapping

| Toolbar Button | Layout Algorithm | Description |
|----------------|------------------|-------------|
| Dagre (⇧D) | **Treemap** (DEFAULT) | Rectangles proportional to node count |
| Horizontal (⇧H) | **Swimlanes** | Horizontal bands per scope |
| Vertical (⇧V) | **Stacked** | Vertical stacked scopes |
| Radial (⇧R) | **Target** | Concentric rings by scope |
| Force (⇧F) | **Force Clusters** | Physics-based with scope clustering |

---

## Task 1: Create Schema Layouts Module Structure

**Files:**
- Create: `src/lib/schemaLayouts.ts`
- Create: `src/lib/schemaLayouts/index.ts`
- Create: `src/lib/schemaLayouts/types.ts`

**Step 1: Create types file**

```typescript
// src/lib/schemaLayouts/types.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';

/** Layout result with React Flow nodes and edges */
export interface SchemaLayoutResult {
  nodes: Node[];
  edges: Edge[];
}

/** Layout function signature */
export type SchemaLayoutFn = (
  hierarchy: HierarchicalSchemaData
) => SchemaLayoutResult;

/** Layout direction from UI store */
export type LayoutDirection = 'LR' | 'TB' | 'dagre' | 'radial' | 'force';

/** Scope visual config */
export interface ScopeConfig {
  scope: 'Project' | 'Global' | 'Shared';
  color: string;
  order: number;
}

export const SCOPE_CONFIGS: ScopeConfig[] = [
  { scope: 'Project', color: '#8b5cf6', order: 0 },
  { scope: 'Global', color: '#10b981', order: 1 },
  { scope: 'Shared', color: '#f59e0b', order: 2 },
];

/** Node dimensions */
export const NODE_WIDTH = 140;
export const NODE_HEIGHT = 50;
export const GROUP_PADDING = 60;
```

**Step 2: Create index file with dispatcher**

```typescript
// src/lib/schemaLayouts/index.ts
export * from './types';
export { applyTreemapLayout } from './treemap';
export { applySwimlaneLayout } from './swimlanes';
export { applyStackedLayout } from './stacked';
export { applyTargetLayout } from './target';
export { applyForceClusterLayout } from './forceClusters';

import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { LayoutDirection, SchemaLayoutResult } from './types';
import { applyTreemapLayout } from './treemap';
import { applySwimlaneLayout } from './swimlanes';
import { applyStackedLayout } from './stacked';
import { applyTargetLayout } from './target';
import { applyForceClusterLayout } from './forceClusters';

/**
 * Apply schema layout based on direction
 * @param hierarchy - Schema hierarchy data
 * @param direction - Layout direction from toolbar
 */
export function applySchemaLayoutByDirection(
  hierarchy: HierarchicalSchemaData,
  direction: LayoutDirection
): SchemaLayoutResult {
  switch (direction) {
    case 'dagre':
      return applyTreemapLayout(hierarchy);
    case 'LR':
      return applySwimlaneLayout(hierarchy);
    case 'TB':
      return applyStackedLayout(hierarchy);
    case 'radial':
      return applyTargetLayout(hierarchy);
    case 'force':
      return applyForceClusterLayout(hierarchy);
    default:
      return applyTreemapLayout(hierarchy);
  }
}
```

**Step 3: Commit**

```bash
git add src/lib/schemaLayouts/
git commit -m "feat(schema): add layout module structure with types"
```

---

## Task 2: Implement Treemap Layout (Default)

**Files:**
- Create: `src/lib/schemaLayouts/treemap.ts`

**Step 1: Create treemap layout**

The treemap algorithm divides space proportionally based on node count per scope/subcategory.

```typescript
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
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayouts/treemap.ts
git commit -m "feat(schema): implement treemap layout algorithm"
```

---

## Task 3: Implement Swimlanes Layout (Horizontal)

**Files:**
- Create: `src/lib/schemaLayouts/swimlanes.ts`

**Step 1: Create swimlanes layout**

```typescript
// src/lib/schemaLayouts/swimlanes.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT, GROUP_PADDING } from './types';
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

  const LANE_HEIGHT = 400;
  const LANE_MARGIN = 40;
  const NODE_SPACING_X = 200;
  const NODE_SPACING_Y = 100;

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
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayouts/swimlanes.ts
git commit -m "feat(schema): implement swimlanes layout (horizontal)"
```

---

## Task 4: Implement Stacked Layout (Vertical)

**Files:**
- Create: `src/lib/schemaLayouts/stacked.ts`

**Step 1: Create stacked layout**

```typescript
// src/lib/schemaLayouts/stacked.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT, GROUP_PADDING } from './types';
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

  const SCOPE_WIDTH = 1800;
  const SCOPE_MARGIN = 80;
  const SUBCAT_MARGIN = 30;
  const NODE_SPACING = 30;
  const NODES_PER_ROW = 8;

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
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayouts/stacked.ts
git commit -m "feat(schema): implement stacked layout (vertical)"
```

---

## Task 5: Implement Target Layout (Radial)

**Files:**
- Create: `src/lib/schemaLayouts/target.ts`

**Step 1: Create target/bullseye layout**

```typescript
// src/lib/schemaLayouts/target.ts
import type { Node, Edge } from '@xyflow/react';
import type { HierarchicalSchemaData } from '@novanet/core/graph';
import type { SchemaLayoutResult } from './types';
import { SCOPE_CONFIGS, NODE_WIDTH, NODE_HEIGHT } from './types';
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

  const CENTER_X = 1200;
  const CENTER_Y = 1000;
  const RING_SPACING = 350;
  const MIN_RADIUS = 200;

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

      let allNodes: { nodeType: string; subcatName: string; subcatMeta: any }[] = [];
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
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayouts/target.ts
git commit -m "feat(schema): implement target/bullseye layout (radial)"
```

---

## Task 6: Implement Force Clusters Layout

**Files:**
- Create: `src/lib/schemaLayouts/forceClusters.ts`

**Step 1: Create force clusters layout**

```typescript
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
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayouts/forceClusters.ts
git commit -m "feat(schema): implement force clusters layout"
```

---

## Task 7: Update schemaLayoutELK.ts to Use New Layouts

**Files:**
- Modify: `src/lib/schemaLayoutELK.ts`

**Step 1: Replace ELK layout with new dispatcher**

Update `schemaLayoutELK.ts` to import and use the new layout system:

```typescript
// At the top of the file, replace the entire content with:
/**
 * Schema Layout Dispatcher
 *
 * Routes layout requests to specialized layout algorithms based on direction.
 * Each layout is optimized for different visualization needs:
 * - Treemap (dagre): Default, proportional rectangles
 * - Swimlanes (LR): Horizontal bands per scope
 * - Stacked (TB): Vertical stacked scopes
 * - Target (radial): Concentric rings
 * - Force (force): Physics-based clustering
 */

import type { HierarchicalSchemaData } from '@novanet/core/graph';
import { applySchemaLayoutByDirection } from './schemaLayouts';
import type { SchemaLayoutResult } from './schemaLayouts/types';

/** Layout direction from UI store */
type LayoutDirection = 'LR' | 'TB' | 'dagre' | 'radial' | 'force';

/**
 * Apply schema layout based on direction.
 *
 * @param hierarchy - Hierarchical schema data from @novanet/core
 * @param layoutDirection - UI layout direction
 * @returns React Flow nodes and edges
 */
export async function applySchemaLayout(
  hierarchy: HierarchicalSchemaData,
  layoutDirection: LayoutDirection = 'dagre'
): Promise<SchemaLayoutResult> {
  // Delegate to the new layout system
  return applySchemaLayoutByDirection(hierarchy, layoutDirection);
}

// Re-export types for backward compatibility
export type { SchemaLayoutResult };
```

**Step 2: Commit**

```bash
git add src/lib/schemaLayoutELK.ts
git commit -m "refactor(schema): use new layout dispatcher instead of ELK-only"
```

---

## Task 8: Verify and Test

**Step 1: Run type-check**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq/apps/studio
npx tsc --noEmit
```

Expected: No errors

**Step 2: Test in browser**

1. Start dev server: `pnpm dev`
2. Open http://localhost:3000
3. Switch to Schema view
4. Test each layout button:
   - ⇧D (Dagre) → Treemap layout
   - ⇧H (Horizontal) → Swimlanes layout
   - ⇧V (Vertical) → Stacked layout
   - ⇧R (Radial) → Target layout
   - ⇧F (Force) → Force clusters layout

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat(schema): complete 5 specialized layout algorithms

- Treemap: Default layout with proportional rectangles
- Swimlanes: Horizontal bands per scope
- Stacked: Vertical stacked scopes
- Target: Concentric rings by scope hierarchy
- Force: Physics-based clustering

Layouts accessible via existing toolbar buttons (⇧D, ⇧H, ⇧V, ⇧R, ⇧F)"
```

---

## Summary

| Task | Description | Time |
|------|-------------|------|
| 1 | Module structure + types | 3 min |
| 2 | Treemap layout | 5 min |
| 3 | Swimlanes layout | 5 min |
| 4 | Stacked layout | 5 min |
| 5 | Target layout | 5 min |
| 6 | Force clusters layout | 5 min |
| 7 | Update dispatcher | 3 min |
| 8 | Verify & test | 5 min |

**Total: ~35 minutes**
