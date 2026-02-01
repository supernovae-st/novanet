# Magnetic Grouping Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace hardcoded UI containers with Neo4j-driven magnetic grouping where nodes gravitate toward their Scope/Subcategory via IN_SUBCATEGORY relationships.

**Architecture:**
- All instance nodes connect to their Subcategory via `:IN_SUBCATEGORY` relationship
- Scope/Subcategory nodes are displayed as visible "attractor" nodes in the graph
- Force-directed layout uses these relationships for magnetic clustering
- No more hardcoded containers - pure graph-driven visualization

**Tech Stack:** Neo4j (Cypher), TypeScript, React Flow, d3-force

---

## Phase 1: Neo4j - IN_SUBCATEGORY Relationships

### Task 1: Add IN_SUBCATEGORY relationship type to schema

**Files:**
- Modify: `packages/core/models/relations/in-subcategory.yaml` (create)
- Modify: `packages/db/seed/00-constraints.cypher`

**Step 1: Create relationship YAML definition**

Create `packages/core/models/relations/in-subcategory.yaml`:
```yaml
# IN_SUBCATEGORY - Links instance nodes to their taxonomy position
name: IN_SUBCATEGORY
description: Connects any instance node to its Subcategory for taxonomy grouping
from:
  - "*"  # Any node type
to:
  - Subcategory
properties: []  # No properties needed
cardinality: "N:1"  # Many instances to one subcategory
```

**Step 2: Add relationship index to constraints**

Add to `packages/db/seed/00-constraints.cypher`:
```cypher
// IN_SUBCATEGORY - Taxonomy grouping (v8.3.0)
// No index needed - traversal is always from instance to subcategory
```

**Step 3: Commit**
```bash
git add packages/core/models/relations/in-subcategory.yaml packages/db/seed/00-constraints.cypher
git commit -m "feat(schema): add IN_SUBCATEGORY relationship type"
```

---

### Task 2: Create auto-wiring Cypher script

**Files:**
- Create: `packages/db/seed/99-autowire-subcategories.cypher`

**Step 1: Write the auto-wiring script**

Create `packages/db/seed/99-autowire-subcategories.cypher`:
```cypher
// Auto-wire IN_SUBCATEGORY relationships
// This script runs AFTER all other seeds to connect instances to their Subcategory
// Based on: instance label → NodeTypeMeta → DEFINED_BY → Subcategory

// For each instance node that doesn't have IN_SUBCATEGORY yet,
// find its NodeTypeMeta (by matching label) and create the relationship

// Step 1: Get all labels that have NodeTypeMeta definitions
MATCH (ntm:NodeTypeMeta)-[:DEFINED_BY]->(sub:Subcategory)
WITH ntm.label AS nodeLabel, sub

// Step 2: For each defined label, wire all instances to their subcategory
CALL {
  WITH nodeLabel, sub
  CALL apoc.cypher.run(
    'MATCH (n:' + nodeLabel + ') WHERE NOT (n)-[:IN_SUBCATEGORY]->()
     MERGE (n)-[:IN_SUBCATEGORY]->(sub)
     RETURN count(n) as wired',
    {sub: sub}
  ) YIELD value
  RETURN value.wired AS wired
}
RETURN nodeLabel, sub.key AS subcategory, sum(wired) AS nodesWired;
```

**Step 2: Verify script is idempotent**

The script uses `WHERE NOT (n)-[:IN_SUBCATEGORY]->()` and `MERGE` to be idempotent.

**Step 3: Commit**
```bash
git add packages/db/seed/99-autowire-subcategories.cypher
git commit -m "feat(db): add auto-wiring script for IN_SUBCATEGORY"
```

---

### Task 3: Alternative auto-wiring (without APOC)

**Files:**
- Modify: `packages/db/seed/99-autowire-subcategories.cypher`

If APOC is not available, use static approach:

**Step 1: Rewrite without APOC**

```cypher
// Auto-wire IN_SUBCATEGORY relationships (no APOC version)
// Run after all other seeds

// Global > Config
MATCH (n:Locale) WHERE NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'config'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Global > Knowledge
MATCH (n) WHERE (n:LocaleVoice OR n:LocaleCulture OR n:LocaleIdentity OR n:LocaleLexicon
  OR n:LocaleMarket OR n:LocaleRulesAdaptation OR n:LocaleRulesFormatting OR n:LocaleRulesSlug
  OR n:LocaleCultureReferences OR n:Expression OR n:Constraint OR n:Pattern OR n:Reference OR n:Metaphor)
  AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'knowledge'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Foundation
MATCH (n) WHERE (n:Project OR n:BrandIdentity OR n:ProjectL10n) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'foundation'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Structure
MATCH (n) WHERE (n:Page OR n:Block OR n:PageType OR n:BlockType) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'structure'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Semantic
MATCH (n) WHERE (n:Concept OR n:ConceptL10n) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'semantic'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Instruction
MATCH (n) WHERE (n:PagePrompt OR n:BlockPrompt OR n:BlockRules) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'instruction'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Project > Output
MATCH (n) WHERE (n:PageL10n OR n:BlockL10n) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'output'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Shared > SEO
MATCH (n) WHERE (n:SEOKeywordL10n OR n:SEOKeywordMetrics OR n:SEOMiningRun) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'seo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Shared > GEO
MATCH (n) WHERE (n:GEOSeedL10n OR n:GEOSeedMetrics OR n:GEOMiningRun) AND NOT (n)-[:IN_SUBCATEGORY]->()
MATCH (sub:Subcategory {key: 'geo'})
MERGE (n)-[:IN_SUBCATEGORY]->(sub);

// Summary
MATCH (sub:Subcategory)<-[r:IN_SUBCATEGORY]-()
RETURN sub.key AS subcategory, count(r) AS instanceCount
ORDER BY instanceCount DESC;
```

**Step 2: Commit**
```bash
git add packages/db/seed/99-autowire-subcategories.cypher
git commit -m "feat(db): auto-wiring script without APOC dependency"
```

---

### Task 4: Run seed and verify IN_SUBCATEGORY

**Files:**
- None (execution only)

**Step 1: Reset and seed database**
```bash
cd packages/db
pnpm reset  # or: pnpm down && pnpm up && pnpm seed
```

**Step 2: Verify relationships created**

Run in Neo4j Browser or cypher-shell:
```cypher
// Count IN_SUBCATEGORY by subcategory
MATCH (sub:Subcategory)<-[r:IN_SUBCATEGORY]-(n)
RETURN sub.key AS subcategory, count(n) AS instances
ORDER BY instances DESC;

// Verify a specific node has the relationship
MATCH (c:Concept {key: 'action-create-qr'})-[:IN_SUBCATEGORY]->(sub:Subcategory)
RETURN c.key, sub.key;
```

**Expected output:**
```
╒═════════════╤═══════════╕
│subcategory  │instances  │
╞═════════════╪═══════════╡
│semantic     │30         │  (5 Concepts + 25 ConceptL10n)
│knowledge    │...        │
│...          │...        │
└─────────────┴───────────┘
```

**Step 3: Commit (if any fixes needed)**
```bash
git add -A
git commit -m "fix(db): adjust auto-wiring script after testing"
```

---

## Phase 2: Studio - Magnetic Force Layout

### Task 5: Create new magnetic layout algorithm

**Files:**
- Create: `apps/studio/src/lib/schemaLayouts/magnetic.ts`
- Modify: `apps/studio/src/lib/schemaLayouts/index.ts`
- Modify: `apps/studio/src/lib/schemaLayouts/types.ts`

**Step 1: Add magnetic layout type**

Add to `apps/studio/src/lib/schemaLayouts/types.ts`:
```typescript
export type SchemaLayoutType =
  | 'swimlanes'
  | 'stacked'
  | 'elkLayered'
  | 'target'
  | 'forceClusters'
  | 'magnetic';  // NEW
```

**Step 2: Create magnetic.ts**

Create `apps/studio/src/lib/schemaLayouts/magnetic.ts`:
```typescript
// src/lib/schemaLayouts/magnetic.ts
/**
 * Magnetic Layout - Force-directed with Neo4j-driven grouping
 *
 * Unlike other layouts, this one:
 * 1. Displays Scope/Subcategory as VISIBLE nodes (not containers)
 * 2. Uses IN_SUBCATEGORY relationships for magnetic attraction
 * 3. Runs d3-force simulation with custom forces
 *
 * Visual structure:
 *
 *           ◉ Scope:project
 *          ╱ ╲
 *         ╱   ╲ HAS_SUBCATEGORY
 *        ╱     ╲
 *   ◉ Sub:semantic  ◉ Sub:structure
 *    ╱╲              ╱╲
 *   ○ ○ ○          ○ ○ ○   ← attracted via IN_SUBCATEGORY
 */

import type { Node, Edge } from '@xyflow/react';
import type { SchemaLayoutResult } from './types';
import { NODE_WIDTH, NODE_HEIGHT, NODE_GAP } from './types';

export interface MagneticLayoutInput {
  // Organizing principle nodes from Neo4j
  scopes: Array<{
    key: string;
    displayName: string;
    emoji: string;
    color: string;
  }>;
  subcategories: Array<{
    key: string;
    displayName: string;
    emoji: string;
    scopeKey: string;  // Parent scope
  }>;
  // Instance nodes with their subcategory
  instances: Array<{
    id: string;
    label: string;
    nodeType: string;
    subcategoryKey: string;
    // Original Neo4j properties
    properties: Record<string, unknown>;
  }>;
  // Relationships between nodes
  relationships: Array<{
    source: string;
    target: string;
    type: string;
  }>;
}

/**
 * Initial positions before simulation
 * Places subcategories around their scope in a circle
 * Places instances near their subcategory
 */
function computeInitialPositions(input: MagneticLayoutInput): Map<string, { x: number; y: number }> {
  const positions = new Map<string, { x: number; y: number }>();

  // Scope positions (triangular arrangement)
  const scopePositions: Record<string, { x: number; y: number }> = {
    project: { x: 0, y: 0 },
    global: { x: 2000, y: 0 },
    shared: { x: 1000, y: 1500 },
  };

  // Place scopes
  for (const scope of input.scopes) {
    const pos = scopePositions[scope.key] || { x: 0, y: 0 };
    positions.set(`scope-${scope.key}`, pos);
  }

  // Place subcategories around their scope
  const subcatsByScope = new Map<string, typeof input.subcategories>();
  for (const sub of input.subcategories) {
    const list = subcatsByScope.get(sub.scopeKey) || [];
    list.push(sub);
    subcatsByScope.set(sub.scopeKey, list);
  }

  for (const [scopeKey, subs] of subcatsByScope) {
    const scopePos = positions.get(`scope-${scopeKey}`) || { x: 0, y: 0 };
    const radius = 400;

    subs.forEach((sub, i) => {
      const angle = (2 * Math.PI * i) / subs.length - Math.PI / 2;
      positions.set(`subcat-${sub.key}`, {
        x: scopePos.x + radius * Math.cos(angle),
        y: scopePos.y + radius * Math.sin(angle),
      });
    });
  }

  // Place instances near their subcategory (with jitter)
  for (const instance of input.instances) {
    const subcatPos = positions.get(`subcat-${instance.subcategoryKey}`) || { x: 0, y: 0 };
    positions.set(instance.id, {
      x: subcatPos.x + (Math.random() - 0.5) * 300,
      y: subcatPos.y + (Math.random() - 0.5) * 300,
    });
  }

  return positions;
}

/**
 * Apply magnetic layout
 * Returns nodes and edges for React Flow
 */
export function applyMagneticLayout(input: MagneticLayoutInput): SchemaLayoutResult {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  const positions = computeInitialPositions(input);

  // Create Scope nodes (large, prominent)
  for (const scope of input.scopes) {
    const pos = positions.get(`scope-${scope.key}`)!;
    nodes.push({
      id: `scope-${scope.key}`,
      type: 'scopeAttractor',  // New node type
      position: pos,
      data: {
        key: scope.key,
        label: scope.displayName,
        emoji: scope.emoji,
        color: scope.color,
        nodeCount: input.instances.filter(i =>
          input.subcategories.find(s => s.key === i.subcategoryKey)?.scopeKey === scope.key
        ).length,
      },
    });
  }

  // Create Subcategory nodes (medium size)
  for (const sub of input.subcategories) {
    const pos = positions.get(`subcat-${sub.key}`)!;
    const scope = input.scopes.find(s => s.key === sub.scopeKey);

    nodes.push({
      id: `subcat-${sub.key}`,
      type: 'subcategoryAttractor',  // New node type
      position: pos,
      data: {
        key: sub.key,
        label: sub.displayName,
        emoji: sub.emoji,
        scopeKey: sub.scopeKey,
        color: scope?.color || '#666',
        nodeCount: input.instances.filter(i => i.subcategoryKey === sub.key).length,
      },
    });

    // Edge from Scope to Subcategory (HAS_SUBCATEGORY)
    edges.push({
      id: `edge-scope-${sub.scopeKey}-to-${sub.key}`,
      source: `scope-${sub.scopeKey}`,
      target: `subcat-${sub.key}`,
      type: 'floating',
      data: { relationType: 'HAS_SUBCATEGORY' },
    });
  }

  // Create instance nodes
  for (const instance of input.instances) {
    const pos = positions.get(instance.id)!;
    const sub = input.subcategories.find(s => s.key === instance.subcategoryKey);
    const scope = input.scopes.find(s => s.key === sub?.scopeKey);

    nodes.push({
      id: instance.id,
      type: 'schemaNode',  // Reuse existing node type
      position: pos,
      draggable: true,
      data: {
        nodeType: instance.nodeType,
        label: instance.label,
        scope: scope?.key,
        subcategory: instance.subcategoryKey,
        ...instance.properties,
      },
    });

    // Edge from instance to Subcategory (IN_SUBCATEGORY) - rendered faintly
    edges.push({
      id: `edge-${instance.id}-to-subcat-${instance.subcategoryKey}`,
      source: instance.id,
      target: `subcat-${instance.subcategoryKey}`,
      type: 'magnetic',  // New edge type (faint, dashed)
      data: { relationType: 'IN_SUBCATEGORY' },
    });
  }

  // Add business relationships between instances
  for (const rel of input.relationships) {
    // Skip IN_SUBCATEGORY (already added above)
    if (rel.type === 'IN_SUBCATEGORY') continue;

    edges.push({
      id: `edge-${rel.source}-${rel.type}-${rel.target}`,
      source: rel.source,
      target: rel.target,
      type: 'floating',
      data: { relationType: rel.type },
    });
  }

  return { nodes, edges };
}
```

**Step 3: Export from index**

Add to `apps/studio/src/lib/schemaLayouts/index.ts`:
```typescript
export { applyMagneticLayout } from './magnetic';
export type { MagneticLayoutInput } from './magnetic';
```

**Step 4: Commit**
```bash
git add apps/studio/src/lib/schemaLayouts/magnetic.ts apps/studio/src/lib/schemaLayouts/index.ts apps/studio/src/lib/schemaLayouts/types.ts
git commit -m "feat(studio): add magnetic layout algorithm"
```

---

### Task 6: Create attractor node components

**Files:**
- Create: `apps/studio/src/components/graph/nodes/ScopeAttractorNode.tsx`
- Create: `apps/studio/src/components/graph/nodes/SubcategoryAttractorNode.tsx`
- Modify: `apps/studio/src/components/graph/nodes/index.ts`

**Step 1: Create ScopeAttractorNode**

Create `apps/studio/src/components/graph/nodes/ScopeAttractorNode.tsx`:
```typescript
'use client';

/**
 * ScopeAttractorNode - Visible scope node for magnetic grouping
 *
 * Unlike ScopeGroupNode (container), this is a regular node that
 * acts as a gravitational center for its child nodes.
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface ScopeAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  color: string;
  nodeCount: number;
}

export type ScopeAttractorNodeType = Node<ScopeAttractorData, 'scopeAttractor'>;

const SCOPE_SIZE = 120;

export const ScopeAttractorNode = memo(function ScopeAttractorNode({
  data,
  selected,
}: NodeProps<ScopeAttractorNodeType>) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center rounded-full',
        'border-4 transition-all duration-300',
        selected ? 'scale-110' : 'scale-100'
      )}
      style={{
        width: SCOPE_SIZE,
        height: SCOPE_SIZE,
        backgroundColor: `${data.color}20`,
        borderColor: data.color,
        boxShadow: selected
          ? `0 0 40px ${data.color}60, 0 0 80px ${data.color}30`
          : `0 0 20px ${data.color}40`,
      }}
    >
      {/* Emoji */}
      <span className="text-3xl">{data.emoji}</span>

      {/* Label */}
      <span
        className="text-sm font-bold mt-1"
        style={{ color: data.color }}
      >
        {data.label}
      </span>

      {/* Count badge */}
      <span
        className="text-xs mt-1 px-2 py-0.5 rounded-full"
        style={{
          backgroundColor: `${data.color}30`,
          color: data.color
        }}
      >
        {data.nodeCount}
      </span>

      {/* Handles for edges */}
      <Handle type="source" position={Position.Bottom} className="opacity-0" />
      <Handle type="target" position={Position.Top} className="opacity-0" />
    </div>
  );
});
```

**Step 2: Create SubcategoryAttractorNode**

Create `apps/studio/src/components/graph/nodes/SubcategoryAttractorNode.tsx`:
```typescript
'use client';

/**
 * SubcategoryAttractorNode - Visible subcategory node for magnetic grouping
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface SubcategoryAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  scopeKey: string;
  color: string;
  nodeCount: number;
}

export type SubcategoryAttractorNodeType = Node<SubcategoryAttractorData, 'subcategoryAttractor'>;

const SUBCAT_SIZE = 80;

export const SubcategoryAttractorNode = memo(function SubcategoryAttractorNode({
  data,
  selected,
}: NodeProps<SubcategoryAttractorNodeType>) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center rounded-full',
        'border-2 transition-all duration-300',
        selected ? 'scale-110' : 'scale-100'
      )}
      style={{
        width: SUBCAT_SIZE,
        height: SUBCAT_SIZE,
        backgroundColor: `${data.color}15`,
        borderColor: `${data.color}80`,
        boxShadow: selected
          ? `0 0 30px ${data.color}50`
          : `0 0 15px ${data.color}30`,
      }}
    >
      {/* Emoji */}
      <span className="text-xl">{data.emoji}</span>

      {/* Label */}
      <span
        className="text-xs font-semibold"
        style={{ color: data.color }}
      >
        {data.label}
      </span>

      {/* Count */}
      <span
        className="text-[10px]"
        style={{ color: `${data.color}90` }}
      >
        {data.nodeCount}
      </span>

      {/* Handles */}
      <Handle type="source" position={Position.Bottom} className="opacity-0" />
      <Handle type="target" position={Position.Top} className="opacity-0" />
    </div>
  );
});
```

**Step 3: Export from index**

Add to `apps/studio/src/components/graph/nodes/index.ts`:
```typescript
export { ScopeAttractorNode } from './ScopeAttractorNode';
export { SubcategoryAttractorNode } from './SubcategoryAttractorNode';
```

**Step 4: Commit**
```bash
git add apps/studio/src/components/graph/nodes/ScopeAttractorNode.tsx apps/studio/src/components/graph/nodes/SubcategoryAttractorNode.tsx apps/studio/src/components/graph/nodes/index.ts
git commit -m "feat(studio): add attractor node components for magnetic layout"
```

---

### Task 7: Create magnetic edge type

**Files:**
- Create: `apps/studio/src/components/graph/edges/MagneticEdge.tsx`
- Modify: `apps/studio/src/components/graph/edges/index.ts`

**Step 1: Create MagneticEdge**

Create `apps/studio/src/components/graph/edges/MagneticEdge.tsx`:
```typescript
'use client';

/**
 * MagneticEdge - Faint edge for IN_SUBCATEGORY relationships
 *
 * These edges represent the taxonomy grouping but should be
 * visually subtle to not overwhelm the actual business relationships.
 */

import { memo } from 'react';
import { BaseEdge, getStraightPath, type EdgeProps } from '@xyflow/react';

export const MagneticEdge = memo(function MagneticEdge({
  sourceX,
  sourceY,
  targetX,
  targetY,
  style,
}: EdgeProps) {
  const [edgePath] = getStraightPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  return (
    <BaseEdge
      path={edgePath}
      style={{
        ...style,
        stroke: '#ffffff15',
        strokeWidth: 1,
        strokeDasharray: '4 4',
      }}
    />
  );
});
```

**Step 2: Export from index**

Add to `apps/studio/src/components/graph/edges/index.ts`:
```typescript
export { MagneticEdge } from './MagneticEdge';
```

**Step 3: Commit**
```bash
git add apps/studio/src/components/graph/edges/MagneticEdge.tsx apps/studio/src/components/graph/edges/index.ts
git commit -m "feat(studio): add magnetic edge type for taxonomy relationships"
```

---

### Task 8: Add d3-force simulation hook

**Files:**
- Create: `apps/studio/src/hooks/useMagneticSimulation.ts`
- Modify: `apps/studio/package.json` (add d3-force if not present)

**Step 1: Check/add d3-force dependency**
```bash
cd apps/studio
pnpm add d3-force @types/d3-force
```

**Step 2: Create useMagneticSimulation hook**

Create `apps/studio/src/hooks/useMagneticSimulation.ts`:
```typescript
'use client';

/**
 * useMagneticSimulation - d3-force simulation for magnetic grouping
 *
 * Forces applied:
 * 1. Attraction to subcategory (via IN_SUBCATEGORY)
 * 2. Repulsion between same-type nodes
 * 3. Collision detection
 * 4. Center gravity (weak)
 */

import { useCallback, useRef, useEffect } from 'react';
import {
  forceSimulation,
  forceLink,
  forceManyBody,
  forceCollide,
  forceX,
  forceY,
  type Simulation,
  type SimulationNodeDatum,
  type SimulationLinkDatum,
} from 'd3-force';
import type { Node } from '@xyflow/react';

interface SimNode extends SimulationNodeDatum {
  id: string;
  fx?: number | null;
  fy?: number | null;
  isAttractor?: boolean;
}

interface SimLink extends SimulationLinkDatum<SimNode> {
  source: string | SimNode;
  target: string | SimNode;
  strength?: number;
}

export interface UseMagneticSimulationOptions {
  /** Strength of attraction to subcategory (0-1) */
  attractionStrength?: number;
  /** Strength of repulsion between nodes */
  repulsionStrength?: number;
  /** Node collision radius */
  collisionRadius?: number;
  /** Whether simulation is running */
  enabled?: boolean;
}

export function useMagneticSimulation(
  nodes: Node[],
  edges: Array<{ source: string; target: string; type?: string }>,
  options: UseMagneticSimulationOptions = {}
) {
  const {
    attractionStrength = 0.3,
    repulsionStrength = -100,
    collisionRadius = 60,
    enabled = true,
  } = options;

  const simulationRef = useRef<Simulation<SimNode, SimLink> | null>(null);
  const nodesRef = useRef<Map<string, SimNode>>(new Map());

  const updatePositions = useCallback((callback: (positions: Map<string, { x: number; y: number }>) => void) => {
    if (!simulationRef.current) return;

    const positions = new Map<string, { x: number; y: number }>();
    nodesRef.current.forEach((node, id) => {
      if (node.x !== undefined && node.y !== undefined) {
        positions.set(id, { x: node.x, y: node.y });
      }
    });
    callback(positions);
  }, []);

  const initSimulation = useCallback(() => {
    // Convert React Flow nodes to simulation nodes
    const simNodes: SimNode[] = nodes.map(n => ({
      id: n.id,
      x: n.position.x,
      y: n.position.y,
      // Fix attractor positions (scopes and subcategories)
      fx: n.type?.includes('Attractor') ? n.position.x : null,
      fy: n.type?.includes('Attractor') ? n.position.y : null,
      isAttractor: n.type?.includes('Attractor'),
    }));

    // Store in ref for position updates
    nodesRef.current.clear();
    simNodes.forEach(n => nodesRef.current.set(n.id, n));

    // Convert edges to simulation links
    // IN_SUBCATEGORY edges have stronger attraction
    const simLinks: SimLink[] = edges
      .filter(e => e.type === 'magnetic' || e.type === 'IN_SUBCATEGORY')
      .map(e => ({
        source: e.source,
        target: e.target,
        strength: attractionStrength,
      }));

    // Create simulation
    const simulation = forceSimulation<SimNode>(simNodes)
      .force('link', forceLink<SimNode, SimLink>(simLinks)
        .id(d => d.id)
        .distance(150)
        .strength(d => d.strength || attractionStrength)
      )
      .force('charge', forceManyBody<SimNode>()
        .strength(d => d.isAttractor ? 0 : repulsionStrength)
      )
      .force('collide', forceCollide<SimNode>(collisionRadius))
      .force('x', forceX<SimNode>().strength(0.01))
      .force('y', forceY<SimNode>().strength(0.01))
      .alphaDecay(0.02)
      .velocityDecay(0.3);

    simulationRef.current = simulation;

    return simulation;
  }, [nodes, edges, attractionStrength, repulsionStrength, collisionRadius]);

  const startSimulation = useCallback(() => {
    if (!enabled) return;

    const simulation = initSimulation();
    simulation.alpha(1).restart();
  }, [enabled, initSimulation]);

  const stopSimulation = useCallback(() => {
    simulationRef.current?.stop();
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      simulationRef.current?.stop();
    };
  }, []);

  return {
    startSimulation,
    stopSimulation,
    updatePositions,
    simulation: simulationRef.current,
  };
}
```

**Step 3: Commit**
```bash
git add apps/studio/src/hooks/useMagneticSimulation.ts apps/studio/package.json pnpm-lock.yaml
git commit -m "feat(studio): add d3-force magnetic simulation hook"
```

---

### Task 9: Create API endpoint for organizing principles

**Files:**
- Create: `apps/studio/src/app/api/graph/organizing-principles/route.ts`

**Step 1: Create API route**

Create `apps/studio/src/app/api/graph/organizing-principles/route.ts`:
```typescript
import { NextResponse } from 'next/server';
import { getDriver } from '@/lib/neo4j';

/**
 * GET /api/graph/organizing-principles
 *
 * Returns Scope and Subcategory nodes with their relationships
 * for the magnetic layout visualization.
 */
export async function GET() {
  const driver = getDriver();
  const session = driver.session();

  try {
    const result = await session.run(`
      // Get all scopes with their subcategories
      MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)
      OPTIONAL MATCH (sub)-[:DEFINES_TYPE]->(ntm:NodeTypeMeta)
      RETURN
        s.key AS scopeKey,
        s.display_name AS scopeDisplayName,
        s.emoji AS scopeEmoji,
        s.color AS scopeColor,
        s.llm_context AS scopeLlmContext,
        collect(DISTINCT {
          key: sub.key,
          displayName: sub.display_name,
          emoji: sub.emoji,
          llmContext: sub.llm_context,
          nodeTypes: collect(ntm.label)
        }) AS subcategories
      ORDER BY s.key
    `);

    const scopes = result.records.map(record => ({
      key: record.get('scopeKey'),
      displayName: record.get('scopeDisplayName'),
      emoji: record.get('scopeEmoji'),
      color: record.get('scopeColor'),
      llmContext: record.get('scopeLlmContext'),
      subcategories: record.get('subcategories'),
    }));

    return NextResponse.json({ scopes });
  } catch (error) {
    console.error('Failed to fetch organizing principles:', error);
    return NextResponse.json(
      { error: 'Failed to fetch organizing principles' },
      { status: 500 }
    );
  } finally {
    await session.close();
  }
}
```

**Step 2: Commit**
```bash
git add apps/studio/src/app/api/graph/organizing-principles/route.ts
git commit -m "feat(studio): add API endpoint for organizing principles"
```

---

### Task 10: Integrate magnetic layout into Graph2D

**Files:**
- Modify: `apps/studio/src/components/graph/Graph2D.tsx`
- Modify: `apps/studio/src/stores/uiStore.ts`

**Step 1: Add layout mode to uiStore**

Add to `apps/studio/src/stores/uiStore.ts`:
```typescript
// Add to UIState interface
layoutMode: 'containers' | 'magnetic';

// Add to initial state
layoutMode: 'containers',

// Add action
setLayoutMode: (mode: 'containers' | 'magnetic') => set({ layoutMode: mode }),
```

**Step 2: Add keyboard shortcut for layout toggle**

Add to `apps/studio/src/config/shortcuts.ts`:
```typescript
{
  key: 'Shift+M',
  action: 'Toggle magnetic layout',
  category: 'Layout',
}
```

**Step 3: Modify Graph2D to support magnetic layout**

The integration into Graph2D requires:
1. Detect layoutMode from uiStore
2. If 'magnetic', use applyMagneticLayout instead of current layouts
3. Register new node types (scopeAttractor, subcategoryAttractor)
4. Register new edge type (magnetic)
5. Optionally run d3-force simulation

This is a larger change - the exact implementation depends on the current Graph2D structure.

**Step 4: Commit**
```bash
git add apps/studio/src/components/graph/Graph2D.tsx apps/studio/src/stores/uiStore.ts apps/studio/src/config/shortcuts.ts
git commit -m "feat(studio): integrate magnetic layout mode"
```

---

### Task 11: Add toggle UI for layout mode

**Files:**
- Modify: `apps/studio/src/components/graph/GraphToolbar.tsx`

**Step 1: Add layout toggle button**

Add to GraphToolbar:
```typescript
<button
  onClick={() => setLayoutMode(layoutMode === 'containers' ? 'magnetic' : 'containers')}
  className={cn(
    'px-3 py-1.5 rounded-lg text-sm font-medium',
    'transition-colors duration-200',
    layoutMode === 'magnetic'
      ? 'bg-violet-500/20 text-violet-300 border border-violet-500/40'
      : 'bg-white/5 text-white/60 hover:text-white/80'
  )}
  title="Toggle magnetic grouping (⇧M)"
>
  {layoutMode === 'magnetic' ? '🧲 Magnetic' : '📦 Containers'}
</button>
```

**Step 2: Commit**
```bash
git add apps/studio/src/components/graph/GraphToolbar.tsx
git commit -m "feat(studio): add layout mode toggle in toolbar"
```

---

### Task 12: Test and verify

**Files:**
- None (execution only)

**Step 1: Start dev server**
```bash
pnpm dev
```

**Step 2: Verify in browser**
1. Open http://localhost:3000
2. Press `⇧M` or click the layout toggle
3. Verify:
   - Scope nodes visible as circles
   - Subcategory nodes visible as smaller circles
   - Instance nodes gravitate toward their subcategory
   - IN_SUBCATEGORY edges are faint/dashed
   - Business relationships are normal edges

**Step 3: Run tests**
```bash
pnpm test
pnpm type-check
pnpm lint
```

**Step 4: Final commit**
```bash
git add -A
git commit -m "test(studio): verify magnetic layout implementation"
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| **Phase 1** | 1-4 | Neo4j: Add IN_SUBCATEGORY relationships |
| **Phase 2** | 5-12 | Studio: Magnetic force layout |

**Total: 12 tasks**

**Key files created:**
- `packages/db/seed/99-autowire-subcategories.cypher`
- `apps/studio/src/lib/schemaLayouts/magnetic.ts`
- `apps/studio/src/components/graph/nodes/ScopeAttractorNode.tsx`
- `apps/studio/src/components/graph/nodes/SubcategoryAttractorNode.tsx`
- `apps/studio/src/components/graph/edges/MagneticEdge.tsx`
- `apps/studio/src/hooks/useMagneticSimulation.ts`
- `apps/studio/src/app/api/graph/organizing-principles/route.ts`

**Outcome:**
- No more hardcoded containers
- Scope/Subcategory are visible Neo4j nodes
- Instances gravitate magnetically via IN_SUBCATEGORY
- Toggle between container and magnetic views
