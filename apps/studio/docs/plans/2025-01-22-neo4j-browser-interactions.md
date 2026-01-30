# Neo4j Browser-Like Interactions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement Neo4j Browser-style node interactions (expand, context menu, keyboard nav) and enhanced sidebar filters with NovaNet-specific quick views.

**Architecture:** Event-driven with React Flow callbacks, Zustand state management, and Neo4j queries for neighbor expansion. Sidebar uses filter composition pattern.

**Tech Stack:** React 19, @xyflow/react, Zustand 5, Neo4j Driver, Tailwind CSS

---

## Task 1: Add fetchNodeNeighbors to Neo4j Client

**Files:**
- Modify: `src/lib/neo4j.ts`

**Step 1: Write the test**

```typescript
// src/lib/__tests__/neo4j.test.ts - add to existing file
describe('fetchNodeNeighbors', () => {
  it('should fetch neighbors for a given node ID', async () => {
    const result = await fetchNodeNeighbors('test-node-id', 10);
    expect(result).toHaveProperty('nodes');
    expect(result).toHaveProperty('edges');
    expect(Array.isArray(result.nodes)).toBe(true);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=neo4j`
Expected: FAIL with "fetchNodeNeighbors is not defined"

**Step 3: Write the implementation**

```typescript
// src/lib/neo4j.ts - add after fetchGraphData function

/**
 * Fetch neighbors of a specific node (for expansion)
 * Used by double-click expand feature (Neo4j Browser style)
 */
export async function fetchNodeNeighbors(
  nodeId: string,
  limit: number = 50
): Promise<QueryResult> {
  const cypher = `
    MATCH (n)-[r]-(m)
    WHERE elementId(n) = $nodeId
    RETURN n, r, m
    LIMIT $limit
  `;

  return executeQuery(cypher, {
    nodeId,
    limit: neo4j.int(limit)
  });
}
```

**Step 4: Run test to verify it passes**

Run: `npm test -- --testPathPattern=neo4j`
Expected: PASS

**Step 5: Commit**

```bash
git add src/lib/neo4j.ts src/lib/__tests__/neo4j.test.ts
git commit -m "feat(neo4j): add fetchNodeNeighbors for node expansion"
```

---

## Task 2: Create useNodeExpansion Hook

**Files:**
- Create: `src/hooks/useNodeExpansion.ts`
- Modify: `src/hooks/index.ts`

**Step 1: Write the test**

```typescript
// src/hooks/__tests__/useNodeExpansion.test.ts
import { renderHook, act } from '@testing-library/react';
import { useNodeExpansion } from '../useNodeExpansion';

// Mock neo4j
jest.mock('@/lib/neo4j', () => ({
  fetchNodeNeighbors: jest.fn().mockResolvedValue({
    nodes: [{ id: 'neighbor-1', type: 'Concept', key: 'test', displayName: 'Test' }],
    edges: [{ id: 'edge-1', source: 'node-1', target: 'neighbor-1', type: 'HAS_CONCEPT' }],
  }),
}));

describe('useNodeExpansion', () => {
  it('should expand node and return new nodes/edges', async () => {
    const { result } = renderHook(() => useNodeExpansion());

    let expansion;
    await act(async () => {
      expansion = await result.current.expandNode('node-1');
    });

    expect(expansion.nodes.length).toBeGreaterThan(0);
    expect(result.current.isExpanding).toBe(false);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern=useNodeExpansion`
Expected: FAIL

**Step 3: Write the implementation**

```typescript
// src/hooks/useNodeExpansion.ts
import { useState, useCallback } from 'react';
import { fetchNodeNeighbors } from '@/lib/neo4j';
import { useGraphStore } from '@/stores/graphStore';
import type { GraphNode, GraphEdge } from '@/types';

export interface ExpansionResult {
  nodes: GraphNode[];
  edges: GraphEdge[];
  addedCount: number;
}

export interface UseNodeExpansionReturn {
  expandNode: (nodeId: string, limit?: number) => Promise<ExpansionResult>;
  isExpanding: boolean;
  expandedNodes: Set<string>;
}

export function useNodeExpansion(): UseNodeExpansionReturn {
  const [isExpanding, setIsExpanding] = useState(false);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());
  const mergeGraphData = useGraphStore((state) => state.mergeGraphData);
  const existingNodeIds = useGraphStore((state) => new Set(state.nodes.map(n => n.id)));

  const expandNode = useCallback(async (nodeId: string, limit = 50): Promise<ExpansionResult> => {
    if (expandedNodes.has(nodeId)) {
      return { nodes: [], edges: [], addedCount: 0 };
    }

    setIsExpanding(true);
    try {
      const result = await fetchNodeNeighbors(nodeId, limit);

      // Filter out already existing nodes
      const newNodes = result.nodes.filter(n => !existingNodeIds.has(n.id));
      const newEdges = result.edges;

      // Merge into graph store
      if (newNodes.length > 0 || newEdges.length > 0) {
        mergeGraphData(newNodes, newEdges);
      }

      // Mark as expanded
      setExpandedNodes(prev => new Set([...prev, nodeId]));

      return {
        nodes: newNodes,
        edges: newEdges,
        addedCount: newNodes.length,
      };
    } finally {
      setIsExpanding(false);
    }
  }, [expandedNodes, existingNodeIds, mergeGraphData]);

  return { expandNode, isExpanding, expandedNodes };
}
```

**Step 4: Export from index**

```typescript
// src/hooks/index.ts - add export
export { useNodeExpansion, type UseNodeExpansionReturn, type ExpansionResult } from './useNodeExpansion';
```

**Step 5: Run test and verify**

Run: `npm test -- --testPathPattern=useNodeExpansion`
Expected: PASS

**Step 6: Commit**

```bash
git add src/hooks/useNodeExpansion.ts src/hooks/index.ts src/hooks/__tests__/useNodeExpansion.test.ts
git commit -m "feat(hooks): add useNodeExpansion for double-click expand"
```

---

## Task 3: Add mergeGraphData to graphStore

**Files:**
- Modify: `src/stores/graphStore.ts`

**Step 1: Read existing graphStore**

Check current structure and add mergeGraphData action.

**Step 2: Add mergeGraphData action**

```typescript
// Add to graphStore interface and implementation
mergeGraphData: (newNodes: GraphNode[], newEdges: GraphEdge[]) => void;

// Implementation
mergeGraphData: (newNodes, newEdges) => {
  set((state) => {
    const existingNodeIds = new Set(state.nodes.map(n => n.id));
    const existingEdgeIds = new Set(state.edges.map(e => e.id));

    // Add only new nodes
    const nodesToAdd = newNodes.filter(n => !existingNodeIds.has(n.id));
    // Add only new edges
    const edgesToAdd = newEdges.filter(e => !existingEdgeIds.has(e.id));

    state.nodes = [...state.nodes, ...nodesToAdd];
    state.edges = [...state.edges, ...edgesToAdd];
  });
},
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/stores/graphStore.ts
git commit -m "feat(store): add mergeGraphData for node expansion"
```

---

## Task 4: Add Double-Click Handler to Graph2D

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Import useNodeExpansion**

```typescript
import { useFilteredGraph, useFocusMode, useNodeExpansion } from '@/hooks';
```

**Step 2: Add double-click handler**

```typescript
// Inside Graph2DInner component
const { expandNode, isExpanding } = useNodeExpansion();

const handleNodeDoubleClick: NodeMouseHandler<TurboNodeType> = useCallback(
  async (_, node) => {
    await expandNode(node.id);
  },
  [expandNode]
);
```

**Step 3: Add to ReactFlow**

```tsx
<ReactFlow
  // ... existing props
  onNodeDoubleClick={handleNodeDoubleClick}
>
```

**Step 4: Run type-check and test**

Run: `npm run type-check && npm test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/Graph2D.tsx
git commit -m "feat(graph): add double-click to expand neighbors"
```

---

## Task 5: Create SelectionHalo Component

**Files:**
- Create: `src/components/graph/SelectionHalo.tsx`

**Step 1: Create the component**

```typescript
// src/components/graph/SelectionHalo.tsx
'use client';

import { cn } from '@/lib/utils';

interface SelectionHaloProps {
  isSelected: boolean;
  color?: string;
  className?: string;
}

export function SelectionHalo({ isSelected, color = '#6366f1', className }: SelectionHaloProps) {
  if (!isSelected) return null;

  return (
    <div
      className={cn(
        'absolute inset-0 rounded-xl pointer-events-none',
        'animate-pulse',
        className
      )}
      style={{
        boxShadow: `0 0 0 3px ${color}, 0 0 20px ${color}40, 0 0 40px ${color}20`,
      }}
    />
  );
}
```

**Step 2: Integrate into BaseNodeWrapper**

```typescript
// src/components/graph/nodes/BaseNodeWrapper.tsx
import { SelectionHalo } from '../SelectionHalo';
import { useUIStore } from '@/stores/uiStore';

// Inside component
const selectedNodeId = useUIStore((state) => state.selectedNodeId);
const isSelected = selectedNodeId === nodeId;

// In render
<div className="relative">
  <SelectionHalo isSelected={isSelected} color={config.color} />
  {/* existing node content */}
</div>
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/graph/SelectionHalo.tsx src/components/graph/nodes/BaseNodeWrapper.tsx
git commit -m "feat(ui): add selection halo ring for selected nodes"
```

---

## Task 6: Create NodeContextMenu Component

**Files:**
- Create: `src/components/graph/NodeContextMenu.tsx`
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Create context menu component**

```typescript
// src/components/graph/NodeContextMenu.tsx
'use client';

import { useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useNodeExpansion } from '@/hooks';
import { useGraphStore } from '@/stores/graphStore';
import { copyToClipboard } from '@/lib/clipboard';

interface NodeContextMenuProps {
  nodeId: string;
  position: { x: number; y: number };
  onClose: () => void;
}

export function NodeContextMenu({ nodeId, position, onClose }: NodeContextMenuProps) {
  const { expandNode } = useNodeExpansion();
  const hideNode = useGraphStore((state) => state.hideNode);
  const node = useGraphStore((state) => state.nodes.find(n => n.id === nodeId));

  const handleExpand = useCallback(async () => {
    await expandNode(nodeId);
    onClose();
  }, [expandNode, nodeId, onClose]);

  const handleHide = useCallback(() => {
    hideNode(nodeId);
    onClose();
  }, [hideNode, nodeId, onClose]);

  const handleCopyId = useCallback(async () => {
    await copyToClipboard(node?.key || nodeId);
    onClose();
  }, [node, nodeId, onClose]);

  return (
    <div
      className={cn(
        'fixed z-50 min-w-[160px]',
        'bg-black/90 backdrop-blur-xl',
        'border border-white/10 rounded-lg shadow-2xl',
        'py-1'
      )}
      style={{ left: position.x, top: position.y }}
    >
      <button onClick={handleExpand} className="context-menu-item">
        <span>🔍</span> Expand Neighbors
      </button>
      <button onClick={handleHide} className="context-menu-item">
        <span>👁️</span> Hide Node
      </button>
      <div className="border-t border-white/10 my-1" />
      <button onClick={handleCopyId} className="context-menu-item">
        <span>📋</span> Copy ID
      </button>
    </div>
  );
}
```

**Step 2: Add CSS for context menu items**

```css
/* Add to globals.css or as Tailwind component */
.context-menu-item {
  @apply w-full px-3 py-2 text-left text-sm text-white/80
         hover:bg-white/10 hover:text-white
         flex items-center gap-2 transition-colors;
}
```

**Step 3: Integrate into Graph2D**

Add state for context menu position and node, handle onContextMenu event.

**Step 4: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 5: Commit**

```bash
git add src/components/graph/NodeContextMenu.tsx src/components/graph/Graph2D.tsx
git commit -m "feat(ui): add right-click context menu for nodes"
```

---

## Task 7: Add hideNode to graphStore

**Files:**
- Modify: `src/stores/graphStore.ts`

**Step 1: Add hideNode action**

```typescript
// Add to graphStore
hiddenNodeIds: Set<string>;
hideNode: (nodeId: string) => void;
showNode: (nodeId: string) => void;
clearHiddenNodes: () => void;

// Implementation
hideNode: (nodeId) => {
  set((state) => {
    state.hiddenNodeIds.add(nodeId);
  });
},
showNode: (nodeId) => {
  set((state) => {
    state.hiddenNodeIds.delete(nodeId);
  });
},
clearHiddenNodes: () => {
  set((state) => {
    state.hiddenNodeIds = new Set();
  });
},
```

**Step 2: Update useFilteredGraph to respect hidden nodes**

Filter out hidden nodes in the hook.

**Step 3: Commit**

```bash
git add src/stores/graphStore.ts src/hooks/useFilteredGraph.ts
git commit -m "feat(store): add node hiding functionality"
```

---

## Task 8: Add Keyboard Navigation

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`
- Modify: `src/config/shortcuts.ts`

**Step 1: Add keyboard event handlers**

```typescript
// Handle Tab to cycle through connected nodes
const handleKeyDown = useCallback((event: KeyboardEvent) => {
  if (!selectedNodeId) return;

  const connected = Array.from(connectedIds);
  if (connected.length === 0) return;

  if (event.key === 'Tab') {
    event.preventDefault();
    const currentIndex = connected.indexOf(selectedNodeId);
    const nextIndex = event.shiftKey
      ? (currentIndex - 1 + connected.length) % connected.length
      : (currentIndex + 1) % connected.length;
    setSelectedNode(connected[nextIndex]);
  }

  if (event.key === 'Enter') {
    expandNode(selectedNodeId);
  }

  if (event.key === 'Delete' || event.key === 'Backspace') {
    hideNode(selectedNodeId);
    setSelectedNode(null);
  }
}, [selectedNodeId, connectedIds, setSelectedNode, expandNode, hideNode]);
```

**Step 2: Register keyboard listener**

Use useEffect to add/remove event listener.

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/graph/Graph2D.tsx src/config/shortcuts.ts
git commit -m "feat(keyboard): add Tab/Enter/Delete navigation for nodes"
```

---

## Task 9: Create LabelFilter Component

**Files:**
- Create: `src/components/sidebar/LabelFilter.tsx`

**Step 1: Create the component**

```typescript
// src/components/sidebar/LabelFilter.tsx
'use client';

import { useMemo } from 'react';
import { cn } from '@/lib/utils';
import { useGraphStore } from '@/stores/graphStore';
import { useFilterStore } from '@/stores/filterStore';
import { NODE_TYPE_CONFIG, type NodeTypeConfig } from '@/config/nodeTypes';
import type { NodeType, NodeCategory } from '@/types';

interface LabelFilterProps {
  className?: string;
}

export function LabelFilter({ className }: LabelFilterProps) {
  const nodes = useGraphStore((state) => state.nodes);
  const selectedTypes = useFilterStore((state) => state.selectedNodeTypes);
  const toggleNodeType = useFilterStore((state) => state.toggleNodeType);
  const setSelectedNodeTypes = useFilterStore((state) => state.setSelectedNodeTypes);

  // Count nodes by type
  const typeCounts = useMemo(() => {
    const counts = new Map<NodeType, number>();
    for (const node of nodes) {
      counts.set(node.type, (counts.get(node.type) || 0) + 1);
    }
    return counts;
  }, [nodes]);

  // Group by category
  const byCategory = useMemo(() => {
    const groups = new Map<NodeCategory, NodeTypeConfig[]>();
    for (const config of Object.values(NODE_TYPE_CONFIG)) {
      const list = groups.get(config.category) || [];
      list.push(config);
      groups.set(config.category, list);
    }
    return groups;
  }, []);

  const handleClick = (type: NodeType, event: React.MouseEvent) => {
    if (event.shiftKey) {
      // Shift+click = select ONLY this type
      setSelectedNodeTypes([type]);
    } else {
      // Regular click = toggle
      toggleNodeType(type);
    }
  };

  return (
    <div className={cn('space-y-4', className)}>
      <h3 className="text-sm font-medium text-white/70 uppercase tracking-wider">
        Node Labels
      </h3>
      {Array.from(byCategory.entries()).map(([category, types]) => (
        <div key={category} className="space-y-1">
          <div className="text-xs text-white/50 uppercase">{category}</div>
          {types.map((config) => {
            const count = typeCounts.get(config.type) || 0;
            const isSelected = selectedTypes.includes(config.type);
            return (
              <button
                key={config.type}
                onClick={(e) => handleClick(config.type, e)}
                className={cn(
                  'w-full flex items-center justify-between px-2 py-1.5 rounded',
                  'text-sm transition-colors',
                  isSelected ? 'bg-white/10 text-white' : 'text-white/60 hover:bg-white/5'
                )}
              >
                <span className="flex items-center gap-2">
                  <span>{config.icon}</span>
                  <span>{config.label}</span>
                </span>
                <span
                  className="px-1.5 py-0.5 rounded text-xs"
                  style={{ backgroundColor: config.color + '30', color: config.color }}
                >
                  {count}
                </span>
              </button>
            );
          })}
        </div>
      ))}
    </div>
  );
}
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/sidebar/LabelFilter.tsx
git commit -m "feat(sidebar): add Neo4j Browser style label filter"
```

---

## Task 10: Create QuickViews Component

**Files:**
- Create: `src/components/sidebar/QuickViews.tsx`
- Create: `src/config/quickViews.ts`

**Step 1: Create quick views config**

```typescript
// src/config/quickViews.ts
import type { NodeType } from '@/types';

export interface QuickView {
  id: string;
  name: string;
  description: string;
  icon: string;
  nodeTypes: NodeType[];
}

export const QUICK_VIEWS: QuickView[] = [
  {
    id: 'content-pipeline',
    name: 'Content Pipeline',
    description: 'Page → Block → Output',
    icon: '📄',
    nodeTypes: ['Page', 'Block', 'BlockType', 'PageOutput', 'BlockOutput'],
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Identity/Voice/Culture/Market/Lexicon',
    icon: '🌍',
    nodeTypes: ['Locale', 'LocaleIdentity', 'LocaleVoice', 'LocaleCulture', 'LocaleMarket', 'LocaleLexicon', 'Expression'],
  },
  {
    id: 'seo-geo-targeting',
    name: 'SEO/GEO Targeting',
    description: 'Concepts → Keywords/Seeds',
    icon: '🎯',
    nodeTypes: ['Concept', 'ConceptL10n', 'SEOKeyword', 'SEOVariation', 'GEOSeed', 'GEOReformulation'],
  },
  {
    id: 'project-structure',
    name: 'Project Structure',
    description: 'Project → Brand → Audiences',
    icon: '🏢',
    nodeTypes: ['Project', 'BrandIdentity', 'Audience', 'ProjectL10n', 'AudienceL10n', 'ValuePropL10n', 'SocialProofL10n'],
  },
  {
    id: 'generation-pipeline',
    name: 'Generation Pipeline',
    description: 'Prompts → Rules → Outputs',
    icon: '🤖',
    nodeTypes: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageOutput', 'BlockOutput'],
  },
];
```

**Step 2: Create QuickViews component**

```typescript
// src/components/sidebar/QuickViews.tsx
'use client';

import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import { QUICK_VIEWS, type QuickView } from '@/config/quickViews';

interface QuickViewsProps {
  className?: string;
}

export function QuickViews({ className }: QuickViewsProps) {
  const setSelectedNodeTypes = useFilterStore((state) => state.setSelectedNodeTypes);
  const selectedTypes = useFilterStore((state) => state.selectedNodeTypes);

  const handleSelect = (view: QuickView) => {
    setSelectedNodeTypes(view.nodeTypes);
  };

  const isActive = (view: QuickView) => {
    return view.nodeTypes.every(t => selectedTypes.includes(t)) &&
           selectedTypes.every(t => view.nodeTypes.includes(t));
  };

  return (
    <div className={cn('space-y-2', className)}>
      <h3 className="text-sm font-medium text-white/70 uppercase tracking-wider">
        Quick Views
      </h3>
      {QUICK_VIEWS.map((view) => (
        <button
          key={view.id}
          onClick={() => handleSelect(view)}
          className={cn(
            'w-full text-left p-3 rounded-lg transition-colors',
            isActive(view)
              ? 'bg-indigo-500/20 border border-indigo-500/50'
              : 'bg-white/5 hover:bg-white/10 border border-transparent'
          )}
        >
          <div className="flex items-center gap-2">
            <span className="text-lg">{view.icon}</span>
            <span className="font-medium text-white">{view.name}</span>
          </div>
          <p className="text-xs text-white/50 mt-1">{view.description}</p>
        </button>
      ))}
    </div>
  );
}
```

**Step 3: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 4: Commit**

```bash
git add src/config/quickViews.ts src/components/sidebar/QuickViews.tsx
git commit -m "feat(sidebar): add NovaNet-specific quick views"
```

---

## Task 11: Integrate Sidebar Components

**Files:**
- Modify: `src/components/sidebar/FilterPanel.tsx` (or equivalent)

**Step 1: Import and add components**

```typescript
import { LabelFilter } from './LabelFilter';
import { QuickViews } from './QuickViews';

// In render
<div className="space-y-6 p-4">
  <QuickViews />
  <div className="border-t border-white/10 pt-4">
    <LabelFilter />
  </div>
</div>
```

**Step 2: Run type-check and lint**

Run: `npm run type-check && npm run lint`
Expected: PASS

**Step 3: Commit**

```bash
git add src/components/sidebar/
git commit -m "feat(sidebar): integrate label filter and quick views"
```

---

## Task 12: Final Verification

**Step 1: Run all checks**

```bash
npm run type-check
npm run lint
npm test
npm run build
```

**Step 2: Manual testing checklist**

- [ ] Single click selects node with halo
- [ ] Double click expands neighbors
- [ ] Right-click shows context menu
- [ ] Tab cycles through connected nodes
- [ ] Enter expands selected node
- [ ] Delete hides selected node
- [ ] Label filter toggles work
- [ ] Shift+click selects only one label
- [ ] Quick views apply correct filters
- [ ] Badge counts update correctly

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat: complete Neo4j Browser-like interactions

- Double-click to expand node neighbors
- Selection halo with pulsing animation
- Right-click context menu (expand, hide, copy)
- Keyboard navigation (Tab, Enter, Delete)
- Neo4j Browser style label filters
- NovaNet-specific quick views

Co-Authored-By: Claude <noreply@anthropic.com>"
```
