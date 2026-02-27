# Fix Focus Mode Visibility Bug - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix the bug where clicking a Concept node keeps ALL CONCEPT_L10N nodes visible instead of only directly connected nodes staying visible while others dim.

**Architecture:** The `useFocusMode` hook currently uses `adjacencyMap` from the full graphStore (all 19k nodes/edges). It should instead compute connected nodes from the **filtered edges** only (visible edges in current view). This requires passing filtered edges to useFocusMode and building a local adjacency lookup.

**Tech Stack:** React 19, Zustand 5, @xyflow/react, TypeScript 5.7

---

## Phase 1: Test Infrastructure

### Task 1.1: Create test file for useFocusMode hook

**Files:**
- Create: `src/hooks/__tests__/useFocusMode.test.ts`

**Step 1: Write the failing test for filtered adjacency**

```typescript
// src/hooks/__tests__/useFocusMode.test.ts
import { renderHook } from '@testing-library/react';
import { useFocusMode } from '../useFocusMode';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

// Mock stores
jest.mock('@/stores/graphStore');
jest.mock('@/stores/uiStore');

const mockUseGraphStore = useGraphStore as jest.MockedFunction<typeof useGraphStore>;
const mockUseUIStore = useUIStore as jest.MockedFunction<typeof useUIStore>;

describe('useFocusMode', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('with filtered edges', () => {
    it('should only consider connections from filtered edges, not full adjacencyMap', () => {
      // Setup: Full adjacencyMap has concept connected to 10 L10N nodes
      const fullAdjacencyMap = new Map<string, Set<string>>([
        ['concept-1', new Set(['l10n-1', 'l10n-2', 'l10n-3', 'l10n-4', 'l10n-5', 'l10n-6', 'l10n-7', 'l10n-8', 'l10n-9', 'l10n-10'])],
        ['l10n-1', new Set(['concept-1'])],
        ['l10n-2', new Set(['concept-1'])],
        // ... others would connect back
      ]);

      // But filtered edges only show 2 connections (e.g., only fr-FR and en-US visible)
      const filteredEdges: GraphEdge[] = [
        { id: 'e1', source: 'concept-1', target: 'l10n-1', type: 'HAS_L10N' },
        { id: 'e2', source: 'concept-1', target: 'l10n-2', type: 'HAS_L10N' },
      ];

      mockUseGraphStore.mockImplementation((selector) => {
        const state = { adjacencyMap: fullAdjacencyMap };
        return selector(state as never);
      });

      mockUseUIStore.mockImplementation((selector) => {
        const state = { selectedNodeId: 'concept-1' };
        return selector(state as never);
      });

      const { result } = renderHook(() => useFocusMode(filteredEdges));

      // Only l10n-1 and l10n-2 should be connected (from filtered edges)
      expect(result.current.connectedIds.has('l10n-1')).toBe(true);
      expect(result.current.connectedIds.has('l10n-2')).toBe(true);

      // l10n-3 through l10n-10 should NOT be connected (not in filtered edges)
      expect(result.current.connectedIds.has('l10n-3')).toBe(false);
      expect(result.current.connectedIds.has('l10n-10')).toBe(false);

      // Dimming should work correctly
      expect(result.current.isNodeDimmed('l10n-1')).toBe(false); // Connected
      expect(result.current.isNodeDimmed('l10n-3')).toBe(true);  // Not connected
    });
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --testPathPattern="useFocusMode" --verbose`
Expected: FAIL - useFocusMode doesn't accept filteredEdges parameter

**Step 3: Commit test**

```bash
git add src/hooks/__tests__/useFocusMode.test.ts
git commit -m "test(focus): add failing test for filtered adjacency"
```

---

## Phase 2: Fix useFocusMode Hook

### Task 2.1: Update useFocusMode to accept filtered edges

**Files:**
- Modify: `src/hooks/useFocusMode.ts`

**Step 1: Update hook signature and build local adjacency**

```typescript
// src/hooks/useFocusMode.ts
/**
 * useFocusMode Hook
 *
 * Calculates which nodes/edges should be dimmed based on the selected node.
 * IMPORTANT: Uses filtered edges to determine connections, not full graph.
 * This ensures focus mode respects current view filters.
 */

import { useMemo, useCallback } from 'react';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

export interface FocusModeState {
  /** Currently selected node ID */
  selectedId: string | null;
  /** Set of node IDs that are directly connected (1-hop) */
  connectedIds: Set<string>;
  /** Set of node IDs that are 2-hops away */
  secondHopIds: Set<string>;
  /** Check if a node should be dimmed */
  isNodeDimmed: (nodeId: string) => boolean;
  /** Check if an edge should be dimmed */
  isEdgeDimmed: (sourceId: string, targetId: string) => boolean;
  /** Get opacity level for a node (1, 0.6, or 0.15) */
  getNodeOpacity: (nodeId: string) => number;
}

/**
 * Build adjacency map from edges
 * @param edges - The filtered edges to build adjacency from
 * @returns Map of nodeId -> Set of connected nodeIds
 */
function buildAdjacencyMap(edges: GraphEdge[]): Map<string, Set<string>> {
  const adjacencyMap = new Map<string, Set<string>>();

  for (const edge of edges) {
    // Bidirectional adjacency
    if (!adjacencyMap.has(edge.source)) {
      adjacencyMap.set(edge.source, new Set());
    }
    if (!adjacencyMap.has(edge.target)) {
      adjacencyMap.set(edge.target, new Set());
    }
    adjacencyMap.get(edge.source)!.add(edge.target);
    adjacencyMap.get(edge.target)!.add(edge.source);
  }

  return adjacencyMap;
}

/**
 * Hook for focus mode state
 * When a node is selected, calculates which nodes/edges should be dimmed
 *
 * @param filteredEdges - The currently visible edges (from useFilteredGraph)
 */
export function useFocusMode(filteredEdges: GraphEdge[]): FocusModeState {
  const selectedNodeId = useUIStore((state) => state.selectedNodeId);

  // Build adjacency map from FILTERED edges only
  const adjacencyMap = useMemo(() => {
    return buildAdjacencyMap(filteredEdges);
  }, [filteredEdges]);

  const { connectedIds, secondHopIds } = useMemo(() => {
    if (!selectedNodeId) {
      return { connectedIds: new Set<string>(), secondHopIds: new Set<string>() };
    }

    // Get 1-hop connections from filtered adjacency
    const firstHop = adjacencyMap.get(selectedNodeId) || new Set<string>();
    const connected = new Set(firstHop);

    // Get 2-hop connections
    const secondHop = new Set<string>();
    for (const nodeId of firstHop) {
      const neighbors = adjacencyMap.get(nodeId) || new Set<string>();
      for (const neighbor of neighbors) {
        if (neighbor !== selectedNodeId && !connected.has(neighbor)) {
          secondHop.add(neighbor);
        }
      }
    }

    return { connectedIds: connected, secondHopIds: secondHop };
  }, [selectedNodeId, adjacencyMap]);

  const isNodeDimmed = useCallback(
    (nodeId: string): boolean => {
      if (!selectedNodeId) return false;
      if (nodeId === selectedNodeId) return false;
      if (connectedIds.has(nodeId)) return false;
      if (secondHopIds.has(nodeId)) return false;
      return true;
    },
    [selectedNodeId, connectedIds, secondHopIds]
  );

  const isEdgeDimmed = useCallback(
    (sourceId: string, targetId: string): boolean => {
      if (!selectedNodeId) return false;
      // Edge is visible if it connects to selected node or between connected nodes
      const involvesSelected = sourceId === selectedNodeId || targetId === selectedNodeId;
      const bothConnected = connectedIds.has(sourceId) && connectedIds.has(targetId);
      return !involvesSelected && !bothConnected;
    },
    [selectedNodeId, connectedIds]
  );

  const getNodeOpacity = useCallback(
    (nodeId: string): number => {
      if (!selectedNodeId) return 1;
      if (nodeId === selectedNodeId) return 1;
      if (connectedIds.has(nodeId)) return 1;
      if (secondHopIds.has(nodeId)) return 0.6;
      return 0.15;
    },
    [selectedNodeId, connectedIds, secondHopIds]
  );

  return {
    selectedId: selectedNodeId,
    connectedIds,
    secondHopIds,
    isNodeDimmed,
    isEdgeDimmed,
    getNodeOpacity,
  };
}
```

**Step 2: Run tests**

Run: `npm test -- --testPathPattern="useFocusMode" --verbose`
Expected: PASS

**Step 3: Commit**

```bash
git add src/hooks/useFocusMode.ts
git commit -m "fix(focus): use filtered edges for adjacency calculation"
```

---

## Phase 3: Update Hook Export

### Task 3.1: Update hooks index if needed

**Files:**
- Check: `src/hooks/index.ts`

**Step 1: Verify export is correct**

The export should already be correct since we only changed the function signature:
```typescript
export { useFocusMode, type FocusModeState } from './useFocusMode';
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: FAIL - Graph2D.tsx needs to pass filteredEdges

---

## Phase 4: Update Graph2D Consumer

### Task 4.1: Pass filtered edges to useFocusMode

**Files:**
- Modify: `src/components/graph/Graph2D.tsx:162`

**Step 1: Update useFocusMode call to pass graphEdges**

Find line 162:
```typescript
const { isNodeDimmed, isEdgeDimmed, selectedId: focusSelectedId } = useFocusMode();
```

Replace with:
```typescript
const { isNodeDimmed, isEdgeDimmed, selectedId: focusSelectedId } = useFocusMode(graphEdges);
```

**Step 2: Run type-check**

Run: `npm run type-check`
Expected: PASS

**Step 3: Run all tests**

Run: `npm test`
Expected: PASS

**Step 4: Commit**

```bash
git add src/components/graph/Graph2D.tsx
git commit -m "fix(graph): pass filtered edges to useFocusMode"
```

---

## Phase 5: E2E Testing

### Task 5.1: Create Playwright E2E test for focus behavior

**Files:**
- Create: `e2e/focus-mode.spec.ts`

**Step 1: Write E2E test**

```typescript
// e2e/focus-mode.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Focus Mode - Node Selection Visibility', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for graph to load
    await page.waitForSelector('[data-testid="react-flow"]', { timeout: 10000 });
  });

  test('selecting a Concept should only highlight directly connected visible nodes', async ({ page }) => {
    // Find and click a Concept node (type=invariant with "CONCEPT" label)
    const conceptNode = page.locator('.react-flow__node').filter({ hasText: 'CONCEPT' }).first();
    await conceptNode.click();

    // Wait for dimming to apply
    await page.waitForTimeout(300);

    // Get all nodes
    const allNodes = page.locator('.react-flow__node');
    const nodeCount = await allNodes.count();

    // Count dimmed vs visible nodes
    let dimmedCount = 0;
    let visibleCount = 0;

    for (let i = 0; i < nodeCount; i++) {
      const node = allNodes.nth(i);
      const opacity = await node.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return parseFloat(style.opacity);
      });

      if (opacity < 0.5) {
        dimmedCount++;
      } else {
        visibleCount++;
      }
    }

    // There should be some dimmed nodes (not all visible)
    expect(dimmedCount).toBeGreaterThan(0);
    // The selected node and its direct connections should be visible
    expect(visibleCount).toBeGreaterThan(0);
    // Most nodes should be dimmed (not all L10N visible)
    expect(dimmedCount).toBeGreaterThan(visibleCount);
  });

  test('clicking pane should clear selection and show all nodes at full opacity', async ({ page }) => {
    // First select a node
    const anyNode = page.locator('.react-flow__node').first();
    await anyNode.click();
    await page.waitForTimeout(300);

    // Now click the pane to deselect
    await page.locator('.react-flow__pane').click();
    await page.waitForTimeout(300);

    // All nodes should be full opacity
    const allNodes = page.locator('.react-flow__node');
    const nodeCount = await allNodes.count();

    for (let i = 0; i < Math.min(nodeCount, 10); i++) {
      const node = allNodes.nth(i);
      const opacity = await node.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return parseFloat(style.opacity);
      });
      expect(opacity).toBeGreaterThanOrEqual(0.9);
    }
  });
});
```

**Step 2: Run E2E test**

Run: `npx playwright test e2e/focus-mode.spec.ts --headed`
Expected: PASS (after starting dev server)

**Step 3: Commit**

```bash
git add e2e/focus-mode.spec.ts
git commit -m "test(e2e): add focus mode visibility tests"
```

---

## Phase 6: Manual Verification & Final Commit

### Task 6.1: Manual testing

**Step 1: Start dev server**

Run: `npm run dev`

**Step 2: Manual test checklist**

- [ ] Navigate to graph visualization
- [ ] Click on a CONCEPT node (like "Free Tier")
- [ ] Verify: Only DIRECTLY CONNECTED L10N nodes stay visible
- [ ] Verify: Unconnected L10N nodes are dimmed (opacity ~0.15)
- [ ] Verify: 2-hop nodes have medium opacity (~0.6)
- [ ] Click on empty pane to deselect
- [ ] Verify: All nodes return to full opacity

**Step 3: Run full test suite**

Run: `npm test && npm run type-check && npm run lint`
Expected: ALL PASS

**Step 4: Final commit**

```bash
git add -A
git commit -m "fix(focus): ensure focus mode respects filtered graph visibility

BREAKING CHANGE: useFocusMode now requires filteredEdges parameter.

Before: useFocusMode()
After:  useFocusMode(filteredEdges)

This fix ensures that when selecting a node, only nodes connected
via VISIBLE edges in the current filter are kept visible. Previously,
the hook used the full adjacencyMap which included edges hidden by
filters, causing all L10N nodes to stay visible when selecting a Concept.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Summary

| Phase | Tasks | Key Changes |
|-------|-------|-------------|
| 1. Test Infrastructure | 1 task | Create failing test for filtered adjacency |
| 2. Fix useFocusMode | 1 task | Build adjacency from filteredEdges param |
| 3. Hook Export | 1 task | Verify export (no change needed) |
| 4. Update Consumer | 1 task | Pass graphEdges to useFocusMode in Graph2D |
| 5. E2E Testing | 1 task | Playwright test for visibility behavior |
| 6. Verification | 1 task | Manual test + final commit |

**Total:** 6 tasks, ~6 commits

**Root Cause:** `useFocusMode` used `graphStore.adjacencyMap` (full graph) instead of building adjacency from filtered edges.

**Solution:** Accept `filteredEdges` as parameter, build local adjacency map, use that for connection calculations.
