# Graph Algorithm Improvements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance the knowledge graph visualization with physics-based layout, edge labels, and better clustering while preserving the beautiful Turbo-style design.

**Architecture:** Add d3-force simulation layer on top of dagre initial layout, create edge label component for relationship types, implement force-based clustering by node category.

**Tech Stack:** d3-force, @xyflow/react, SVG textPath for labels

---

## Task 1: Add d3-force Physics Simulation

**Files:**
- Create: `src/lib/forceSimulation.ts`
- Modify: `src/components/graph/Graph2D.tsx:166-178`

**Step 1: Install d3-force**

```bash
npm install d3-force d3-quadtree
npm install -D @types/d3-force
```

**Step 2: Create force simulation utility**

Create `src/lib/forceSimulation.ts`:

```typescript
/**
 * Force Simulation Utilities
 *
 * D3-force physics simulation for dynamic graph layout.
 * Uses velocity Verlet numerical integration for smooth animations.
 */

import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide,
  forceX,
  forceY,
  type Simulation,
  type SimulationNodeDatum,
  type SimulationLinkDatum,
} from 'd3-force';
import type { Node, Edge } from '@xyflow/react';

export interface ForceNode extends SimulationNodeDatum {
  id: string;
  x: number;
  y: number;
  fx?: number | null;
  fy?: number | null;
  category?: string;
}

export interface ForceLink extends SimulationLinkDatum<ForceNode> {
  source: string | ForceNode;
  target: string | ForceNode;
}

export interface ForceOptions {
  /** Strength of node repulsion (-1 to -500) */
  chargeStrength?: number;
  /** Link distance (50-300) */
  linkDistance?: number;
  /** Collision radius multiplier */
  collisionRadius?: number;
  /** Center force strength (0-1) */
  centerStrength?: number;
  /** Enable category clustering */
  clusterByCategory?: boolean;
  /** Alpha decay rate (0.01-0.05) */
  alphaDecay?: number;
  /** Velocity decay (0.1-0.6) */
  velocityDecay?: number;
}

const DEFAULT_OPTIONS: Required<ForceOptions> = {
  chargeStrength: -200,
  linkDistance: 120,
  collisionRadius: 1.2,
  centerStrength: 0.05,
  clusterByCategory: true,
  alphaDecay: 0.02,
  velocityDecay: 0.4,
};

// Category cluster positions (radial layout)
const CATEGORY_CENTERS: Record<string, { x: number; y: number }> = {
  invariant: { x: -300, y: 0 },
  l10n: { x: 0, y: -200 },
  output: { x: 300, y: 0 },
  knowledge: { x: 0, y: 200 },
  prompts: { x: -150, y: -150 },
  seo: { x: 150, y: -150 },
  geo: { x: -150, y: 150 },
  mining: { x: 150, y: 150 },
  metrics: { x: 0, y: 0 },
};

/**
 * Create force simulation from React Flow nodes/edges
 */
export function createForceSimulation<N extends Node, E extends Edge>(
  nodes: N[],
  edges: E[],
  options: ForceOptions = {}
): Simulation<ForceNode, ForceLink> {
  const opts = { ...DEFAULT_OPTIONS, ...options };

  // Convert to force nodes
  const forceNodes: ForceNode[] = nodes.map((n) => ({
    id: n.id,
    x: n.position.x,
    y: n.position.y,
    category: (n.data as { category?: string })?.category,
  }));

  // Convert to force links
  const forceLinks: ForceLink[] = edges.map((e) => ({
    source: e.source,
    target: e.target,
  }));

  // Create simulation
  const simulation = forceSimulation<ForceNode>(forceNodes)
    .alphaDecay(opts.alphaDecay)
    .velocityDecay(opts.velocityDecay);

  // Link force - keeps connected nodes together
  simulation.force(
    'link',
    forceLink<ForceNode, ForceLink>(forceLinks)
      .id((d) => d.id)
      .distance(opts.linkDistance)
      .strength(0.3)
  );

  // Charge force - repels nodes from each other
  simulation.force(
    'charge',
    forceManyBody<ForceNode>()
      .strength(opts.chargeStrength)
      .distanceMax(500)
  );

  // Center force - keeps graph centered
  simulation.force(
    'center',
    forceCenter<ForceNode>(0, 0).strength(opts.centerStrength)
  );

  // Collision force - prevents node overlap
  simulation.force(
    'collision',
    forceCollide<ForceNode>()
      .radius(80 * opts.collisionRadius)
      .strength(0.8)
  );

  // Category clustering force (optional)
  if (opts.clusterByCategory) {
    simulation.force(
      'x',
      forceX<ForceNode>()
        .x((d) => CATEGORY_CENTERS[d.category || 'metrics']?.x || 0)
        .strength(0.1)
    );
    simulation.force(
      'y',
      forceY<ForceNode>()
        .y((d) => CATEGORY_CENTERS[d.category || 'metrics']?.y || 0)
        .strength(0.1)
    );
  }

  return simulation;
}

/**
 * Run simulation until stable and return final positions
 */
export function runSimulationSync(
  simulation: Simulation<ForceNode, ForceLink>,
  maxIterations: number = 300
): Map<string, { x: number; y: number }> {
  // Run simulation to completion
  simulation.stop();
  for (let i = 0; i < maxIterations; i++) {
    simulation.tick();
    if (simulation.alpha() < 0.01) break;
  }

  // Extract final positions
  const positions = new Map<string, { x: number; y: number }>();
  for (const node of simulation.nodes()) {
    positions.set(node.id, { x: node.x || 0, y: node.y || 0 });
  }

  return positions;
}

/**
 * Apply force positions to React Flow nodes
 */
export function applyForcePositions<N extends Node>(
  nodes: N[],
  positions: Map<string, { x: number; y: number }>
): N[] {
  return nodes.map((node) => {
    const pos = positions.get(node.id);
    if (pos) {
      return {
        ...node,
        position: { x: pos.x, y: pos.y },
      };
    }
    return node;
  });
}
```

**Step 3: Integrate force simulation into Graph2D**

Modify `src/components/graph/Graph2D.tsx` to use force layout:

```typescript
// Add imports at top
import { createForceSimulation, runSimulationSync, applyForcePositions } from '@/lib/forceSimulation';

// Replace layoutedNodes useMemo (lines 166-178) with:
const layoutedNodes = useMemo(() => {
  const turboNodes = graphNodes.map((n) => toTurboNode(n, compact));
  const turboEdges = graphEdges.map((e) => toOrganicEdge(e));

  // Step 1: Apply dagre for initial hierarchical positions
  const dagrePositioned = applyDagreLayout(turboNodes, turboEdges, {
    direction: 'LR',
    ranksep: 120,
    nodesep: 60,
    nodeWidth: compact ? 140 : 200,
    nodeHeight: compact ? 50 : 100,
  });

  // Step 2: Apply force simulation for physics-based refinement
  const simulation = createForceSimulation(dagrePositioned, turboEdges, {
    chargeStrength: -150,
    linkDistance: 100,
    clusterByCategory: true,
  });

  const positions = runSimulationSync(simulation);
  return applyForcePositions(dagrePositioned, positions);
}, [graphNodes, graphEdges, compact]);
```

**Step 4: Run tests and verify**

```bash
npm run type-check && npm run lint
npm run dev
```

Expected: Graph renders with physics-based positions, nodes cluster by category

**Step 5: Commit**

```bash
git add src/lib/forceSimulation.ts src/components/graph/Graph2D.tsx package.json package-lock.json
git commit -m "feat(graph): add d3-force physics simulation for better layout

- Create forceSimulation.ts with d3-force utilities
- Integrate force layout after dagre initial positioning
- Add category clustering for visual grouping
- Configure charge, link, collision forces

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 2: Add Edge Labels Component

**Files:**
- Modify: `src/components/graph/edges/OrganicEdge.tsx`

**Step 1: Add edge label with SVG textPath**

Modify `src/components/graph/edges/OrganicEdge.tsx` to add labels:

```typescript
'use client';

/**
 * OrganicEdge - Bezier edge with glow, animated particles, and labels
 *
 * Features:
 * - Gradient colors based on relation type
 * - Multi-layer glow effect
 * - Animated particle flowing along the edge
 * - Edge labels showing relationship type
 * - Focus mode dimming support
 */

import { memo, useMemo } from 'react';
import {
  BaseEdge,
  getBezierPath,
  type Edge,
  type EdgeProps,
} from '@xyflow/react';

export interface OrganicEdgeData extends Record<string, unknown> {
  relationType: string;
  animated?: boolean;
  dimmed?: boolean;
  selected?: boolean;
  showLabel?: boolean;
}

export type OrganicEdgeType = Edge<OrganicEdgeData>;

/**
 * Get edge colors based on relation type
 */
function getEdgeColors(relationType: string): { primary: string; secondary: string } {
  // Structural relations
  if (relationType.includes('HAS_') || relationType.includes('CONTAINS')) {
    return { primary: '#3b82f6', secondary: '#06b6d4' }; // Blue → Cyan
  }
  // Localization relations
  if (relationType.includes('FOR_LOCALE') || relationType.includes('SUPPORTS')) {
    return { primary: '#10b981', secondary: '#22c55e' }; // Emerald → Green
  }
  // Output relations
  if (relationType.includes('OUTPUT') || relationType.includes('GENERATED')) {
    return { primary: '#f97316', secondary: '#ef4444' }; // Orange → Red
  }
  // Semantic relations
  if (relationType.includes('USES_CONCEPT') || relationType.includes('SEMANTIC')) {
    return { primary: '#ec4899', secondary: '#a855f7' }; // Pink → Purple
  }
  // Prompts relations (v7.2.0)
  if (relationType.includes('PROMPT') || relationType.includes('RULES')) {
    return { primary: '#3b82f6', secondary: '#60a5fa' }; // Blue palette
  }
  // Fallback relations
  if (relationType.includes('FALLBACK')) {
    return { primary: '#fbbf24', secondary: '#f59e0b' }; // Yellow → Amber
  }
  // Default
  return { primary: '#6366f1', secondary: '#8b5cf6' }; // Indigo → Violet
}

/**
 * Format relation type for display
 * HAS_BLOCK → Has Block
 */
function formatRelationType(type: string): string {
  return type
    .replace(/_/g, ' ')
    .toLowerCase()
    .replace(/\b\w/g, (c) => c.toUpperCase());
}

/**
 * OrganicEdge - Bezier edge with glow effects and labels
 */
export const OrganicEdge = memo(function OrganicEdge({
  id,
  data,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  selected,
}: EdgeProps<OrganicEdgeType>) {
  const colors = getEdgeColors(data?.relationType || '');
  const isDimmed = data?.dimmed === true;
  const isAnimated = data?.animated !== false;
  const isSelected = selected || data?.selected;
  const showLabel = data?.showLabel !== false; // Show labels by default

  const [edgePath, labelX, labelY] = useMemo(() => {
    return getBezierPath({
      sourceX,
      sourceY,
      sourcePosition,
      targetX,
      targetY,
      targetPosition,
      curvature: 0.5,
    });
  }, [sourceX, sourceY, sourcePosition, targetX, targetY, targetPosition]);

  const strokeWidth = isSelected ? 3 : isDimmed ? 1 : 2;
  const opacity = isDimmed ? 0.1 : 1;
  const labelText = formatRelationType(data?.relationType || '');

  return (
    <>
      {/* Glow layer */}
      {!isDimmed && (
        <path
          d={edgePath}
          fill="none"
          stroke={colors.primary}
          strokeWidth={strokeWidth + 6}
          strokeOpacity={isSelected ? 0.4 : 0.15}
          style={{
            filter: `blur(${isSelected ? 6 : 4}px)`,
          }}
        />
      )}

      {/* Main edge with path ID for text */}
      <path
        id={`edge-path-${id}`}
        d={edgePath}
        fill="none"
        stroke={`url(#edge-gradient-${id})`}
        strokeWidth={strokeWidth}
        opacity={opacity}
        strokeLinecap="round"
        markerEnd="url(#arrow)"
      />

      {/* Edge label using textPath */}
      {showLabel && !isDimmed && (
        <text
          className="pointer-events-none select-none"
          style={{
            fontSize: isSelected ? '11px' : '9px',
            fontWeight: isSelected ? 600 : 500,
            fill: isSelected ? colors.primary : 'rgba(255,255,255,0.6)',
            textShadow: '0 1px 3px rgba(0,0,0,0.8)',
            transition: 'all 0.2s ease',
          }}
        >
          <textPath
            href={`#edge-path-${id}`}
            startOffset="50%"
            textAnchor="middle"
            dominantBaseline="text-after-edge"
          >
            {labelText}
          </textPath>
        </text>
      )}

      {/* Alternative: Positioned label at midpoint (fallback) */}
      {showLabel && !isDimmed && isSelected && (
        <g transform={`translate(${labelX}, ${labelY})`}>
          <rect
            x={-labelText.length * 3.5}
            y={-10}
            width={labelText.length * 7}
            height={16}
            rx={4}
            fill="rgba(0,0,0,0.8)"
            stroke={colors.primary}
            strokeWidth={1}
          />
          <text
            textAnchor="middle"
            dominantBaseline="middle"
            style={{
              fontSize: '10px',
              fontWeight: 600,
              fill: colors.primary,
            }}
          >
            {labelText}
          </text>
        </g>
      )}

      {/* Animated particle */}
      {isAnimated && !isDimmed && (
        <circle r={isSelected ? 4 : 3} fill={colors.primary}>
          <animateMotion
            dur={isSelected ? '1.5s' : '3s'}
            repeatCount="indefinite"
            path={edgePath}
          />
        </circle>
      )}

      {/* Gradient definition */}
      <defs>
        <linearGradient id={`edge-gradient-${id}`} x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor={colors.primary} />
          <stop offset="100%" stopColor={colors.secondary} />
        </linearGradient>
        {/* Arrow marker */}
        <marker
          id="arrow"
          viewBox="0 0 10 10"
          refX="9"
          refY="5"
          markerWidth={isSelected ? 5 : 4}
          markerHeight={isSelected ? 5 : 4}
          orient="auto-start-reverse"
        >
          <path
            d="M 0 0 L 10 5 L 0 10 z"
            fill={colors.secondary}
            opacity={opacity}
          />
        </marker>
      </defs>
    </>
  );
});
```

**Step 2: Run tests and verify**

```bash
npm run type-check && npm run lint
npm run dev
```

Expected: Edge labels visible showing relationship types, highlighted on selection

**Step 3: Commit**

```bash
git add src/components/graph/edges/OrganicEdge.tsx
git commit -m "feat(graph): add edge labels showing relationship types

- Display formatted relation type on each edge
- Use SVG textPath for curved label alignment
- Show badge on selected edges with background
- Add arrow markers to indicate direction
- Labels respect dimming state

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 3: Add Edge Label Toggle Control

**Files:**
- Modify: `src/stores/uiStore.ts`
- Modify: `src/components/graph/Graph2D.tsx`
- Modify: `src/config/shortcuts.ts`

**Step 1: Add showEdgeLabels to UI store**

In `src/stores/uiStore.ts`, add:

```typescript
// In interface UIState, add:
showEdgeLabels: boolean;
toggleEdgeLabels: () => void;

// In create(), add:
showEdgeLabels: true,
toggleEdgeLabels: () => set((state) => ({ showEdgeLabels: !state.showEdgeLabels })),
```

**Step 2: Pass showLabel to edges in Graph2D**

Modify `src/components/graph/Graph2D.tsx`:

```typescript
// Add to imports and hooks
const showEdgeLabels = useUIStore((state) => state.showEdgeLabels);

// In initialEdges useMemo, add showLabel:
data: {
  relationType: e.type,
  dimmed,
  animated: !dimmed,
  showLabel: showEdgeLabels, // Add this
},
```

**Step 3: Add keyboard shortcut (L key)**

In `src/config/shortcuts.ts`, add:

```typescript
{ key: 'l', description: 'Toggle edge labels', action: 'toggleEdgeLabels' },
```

**Step 4: Run tests and verify**

```bash
npm run type-check && npm run lint
```

**Step 5: Commit**

```bash
git add src/stores/uiStore.ts src/components/graph/Graph2D.tsx src/config/shortcuts.ts
git commit -m "feat(graph): add toggle for edge labels (L key)

- Add showEdgeLabels state to UI store
- Pass showLabel prop to OrganicEdge
- Add L keyboard shortcut for toggle

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 4: Enhance Force Simulation with Interactive Physics

**Files:**
- Modify: `src/lib/forceSimulation.ts`
- Modify: `src/components/graph/Graph2D.tsx`
- Create: `src/hooks/useForceSimulation.ts`

**Step 1: Create interactive simulation hook**

Create `src/hooks/useForceSimulation.ts`:

```typescript
/**
 * useForceSimulation - Interactive D3 force simulation for React Flow
 *
 * Provides:
 * - Real-time physics simulation
 * - Dragging updates simulation
 * - Smooth position interpolation
 */

import { useRef, useEffect, useCallback, useState } from 'react';
import { useReactFlow, type Node } from '@xyflow/react';
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCenter,
  forceCollide,
  forceX,
  forceY,
  type Simulation,
} from 'd3-force';
import type { ForceNode, ForceLink, ForceOptions } from '@/lib/forceSimulation';

const CATEGORY_CENTERS: Record<string, { x: number; y: number }> = {
  invariant: { x: -400, y: 0 },
  l10n: { x: 0, y: -300 },
  output: { x: 400, y: 0 },
  knowledge: { x: 0, y: 300 },
  prompts: { x: -200, y: -200 },
  seo: { x: 200, y: -200 },
  geo: { x: -200, y: 200 },
  mining: { x: 200, y: 200 },
  metrics: { x: 0, y: 0 },
};

interface UseForceSimulationOptions extends ForceOptions {
  enabled?: boolean;
}

export function useForceSimulation<N extends Node>(
  initialNodes: N[],
  edges: { source: string; target: string }[],
  options: UseForceSimulationOptions = {}
) {
  const { setNodes } = useReactFlow();
  const simulationRef = useRef<Simulation<ForceNode, ForceLink> | null>(null);
  const [isSimulating, setIsSimulating] = useState(false);

  const {
    enabled = true,
    chargeStrength = -150,
    linkDistance = 100,
    collisionRadius = 1.2,
    clusterByCategory = true,
  } = options;

  // Initialize simulation
  useEffect(() => {
    if (!enabled || initialNodes.length === 0) {
      simulationRef.current?.stop();
      return;
    }

    // Convert nodes to force nodes
    const forceNodes: ForceNode[] = initialNodes.map((n) => ({
      id: n.id,
      x: n.position.x,
      y: n.position.y,
      category: (n.data as { category?: string })?.category,
    }));

    // Convert edges to force links
    const forceLinks: ForceLink[] = edges.map((e) => ({
      source: e.source,
      target: e.target,
    }));

    // Create simulation
    const simulation = forceSimulation<ForceNode>(forceNodes)
      .alphaDecay(0.02)
      .velocityDecay(0.4);

    // Add forces
    simulation.force(
      'link',
      forceLink<ForceNode, ForceLink>(forceLinks)
        .id((d) => d.id)
        .distance(linkDistance)
        .strength(0.3)
    );

    simulation.force(
      'charge',
      forceManyBody<ForceNode>().strength(chargeStrength).distanceMax(500)
    );

    simulation.force('center', forceCenter<ForceNode>(0, 0).strength(0.05));

    simulation.force(
      'collision',
      forceCollide<ForceNode>()
        .radius(80 * collisionRadius)
        .strength(0.8)
    );

    if (clusterByCategory) {
      simulation.force(
        'x',
        forceX<ForceNode>()
          .x((d) => CATEGORY_CENTERS[d.category || 'metrics']?.x || 0)
          .strength(0.08)
      );
      simulation.force(
        'y',
        forceY<ForceNode>()
          .y((d) => CATEGORY_CENTERS[d.category || 'metrics']?.y || 0)
          .strength(0.08)
      );
    }

    // On tick, update React Flow nodes
    simulation.on('tick', () => {
      setIsSimulating(simulation.alpha() > 0.01);

      setNodes((nds) =>
        nds.map((node) => {
          const forceNode = forceNodes.find((fn) => fn.id === node.id);
          if (forceNode && forceNode.x !== undefined && forceNode.y !== undefined) {
            return {
              ...node,
              position: { x: forceNode.x, y: forceNode.y },
            };
          }
          return node;
        })
      );
    });

    simulationRef.current = simulation;

    return () => {
      simulation.stop();
    };
  }, [
    enabled,
    initialNodes,
    edges,
    chargeStrength,
    linkDistance,
    collisionRadius,
    clusterByCategory,
    setNodes,
  ]);

  // Handle node drag - fix position during drag
  const onNodeDragStart = useCallback(
    (nodeId: string) => {
      const simulation = simulationRef.current;
      if (!simulation) return;

      const node = simulation.nodes().find((n) => n.id === nodeId);
      if (node) {
        node.fx = node.x;
        node.fy = node.y;
        simulation.alpha(0.3).restart();
      }
    },
    []
  );

  const onNodeDrag = useCallback(
    (nodeId: string, position: { x: number; y: number }) => {
      const simulation = simulationRef.current;
      if (!simulation) return;

      const node = simulation.nodes().find((n) => n.id === nodeId);
      if (node) {
        node.fx = position.x;
        node.fy = position.y;
      }
    },
    []
  );

  const onNodeDragStop = useCallback(
    (nodeId: string) => {
      const simulation = simulationRef.current;
      if (!simulation) return;

      const node = simulation.nodes().find((n) => n.id === nodeId);
      if (node) {
        node.fx = null;
        node.fy = null;
      }
    },
    []
  );

  // Reheat simulation (for manual trigger)
  const reheat = useCallback(() => {
    simulationRef.current?.alpha(0.3).restart();
  }, []);

  return {
    isSimulating,
    onNodeDragStart,
    onNodeDrag,
    onNodeDragStop,
    reheat,
  };
}
```

**Step 2: Integrate hook into Graph2D**

Modify `src/components/graph/Graph2D.tsx` to use interactive simulation:

```typescript
// Add import
import { useForceSimulation } from '@/hooks/useForceSimulation';

// Inside Graph2DInner, after initialNodes/initialEdges, add:
const {
  isSimulating,
  onNodeDragStart,
  onNodeDrag,
  onNodeDragStop,
} = useForceSimulation(initialNodes, graphEdges, {
  enabled: true,
  chargeStrength: -150,
  linkDistance: 100,
  clusterByCategory: true,
});

// Update node drag handlers
const handleNodeDragStart = useCallback(
  (event: React.MouseEvent, node: TurboNodeType) => {
    onNodeDragStart(node.id);
  },
  [onNodeDragStart]
);

const handleNodeDrag = useCallback(
  (event: React.MouseEvent, node: TurboNodeType) => {
    onNodeDrag(node.id, node.position);
  },
  [onNodeDrag]
);

const handleNodeDragStop = useCallback(
  (event: React.MouseEvent, node: TurboNodeType) => {
    onNodeDragStop(node.id);
  },
  [onNodeDragStop]
);

// In ReactFlow component, add drag handlers:
onNodeDragStart={handleNodeDragStart}
onNodeDrag={handleNodeDrag}
onNodeDragStop={handleNodeDragStop}

// In Stats Panel, add simulation indicator:
{isSimulating && (
  <div className="flex items-center gap-2">
    <div className="w-2 h-2 rounded-full bg-blue-400 animate-pulse" />
    <span className="text-white/40 text-xs">simulating</span>
  </div>
)}
```

**Step 3: Export hook from hooks index**

Add to `src/hooks/index.ts`:

```typescript
export { useForceSimulation } from './useForceSimulation';
```

**Step 4: Run tests and verify**

```bash
npm run type-check && npm run lint
npm run dev
```

Expected: Nodes can be dragged and physics simulation runs, other nodes react

**Step 5: Commit**

```bash
git add src/hooks/useForceSimulation.ts src/hooks/index.ts src/components/graph/Graph2D.tsx
git commit -m "feat(graph): add interactive force simulation with drag support

- Create useForceSimulation hook for real-time physics
- Nodes react to dragging with physics
- Add simulation indicator in stats panel
- Category-based clustering preserves visual grouping

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Task 5: Final Testing and Polish

**Step 1: Verify all features work together**

```bash
npm run dev
```

Test checklist:
- [ ] Graph renders with physics-based layout
- [ ] Nodes cluster by category visually
- [ ] Edge labels show relationship types
- [ ] L key toggles edge labels
- [ ] Dragging nodes triggers physics simulation
- [ ] All existing Turbo effects still work (glow, particles, gradients)
- [ ] Dimming/focus mode still works
- [ ] Selection highlights edges and labels

**Step 2: Run full test suite**

```bash
npm run build
npm run lint
npm run type-check
```

**Step 3: Final commit**

```bash
git add -A
git commit -m "chore: polish graph algorithm improvements

Co-Authored-By: Nika <agent@nika.sh>
```
