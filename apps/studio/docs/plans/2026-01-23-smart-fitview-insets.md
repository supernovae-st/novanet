# Smart FitView with Dynamic Insets - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement intelligent viewport fitting that accounts for UI overlays (sidebar, details panel, top/bottom bars) with smooth animations.

**Architecture:**
- `useViewportInsets()` hook calculates dynamic pixel-based insets based on UI state
- `useCenterOnNode()` hook centers and zooms on a specific node with offset compensation
- React Flow 12.5.0+ advanced padding API with pixel values
- All viewport changes animated with 400ms smooth transitions

**Tech Stack:** React 19, Zustand 5, @xyflow/react, TypeScript 5.7, Playwright (E2E tests)

---

## Layout Constants Reference

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ BASE_MARGIN      = 16px   (breathing room)                                  │
│ TOP_BAR_HEIGHT   = 120px  (QueryPill 48 + gap 12 + Stats 44 + gap 16)       │
│ BOTTOM_BAR_HEIGHT= 80px   (View controls 44 + padding 16 + margin 20)       │
│ SIDEBAR_WIDTH    = 288px  (w-72 = 18rem)                                    │
│ DETAILS_PANEL_W  = 420px  (w-[420px])                                       │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Insets Matrix

| State                          | TOP   | LEFT  | RIGHT | BOTTOM |
|--------------------------------|-------|-------|-------|--------|
| Focus Mode                     | 16    | 16    | 16    | 16     |
| Sidebar Open, Panel Closed     | 136   | 304   | 16    | 96     |
| Sidebar Open, Panel Open       | 136   | 304   | 436   | 96     |
| Sidebar Closed, Panel Closed   | 136   | 16    | 16    | 96     |
| Sidebar Closed, Panel Open     | 136   | 16    | 436   | 96     |

---

## Task 1: Create useViewportInsets Hook

**Files:**
- Create: `src/hooks/useViewportInsets.ts`
- Modify: `src/hooks/index.ts` (add export)
- Test: `src/hooks/__tests__/useViewportInsets.test.ts`

**Step 1: Write the failing test**

```typescript
// src/hooks/__tests__/useViewportInsets.test.ts
import { calculateViewportInsets, LAYOUT_CONSTANTS } from '../useViewportInsets';

describe('calculateViewportInsets', () => {
  const { BASE_MARGIN, TOP_BAR_HEIGHT, BOTTOM_BAR_HEIGHT, SIDEBAR_WIDTH, DETAILS_PANEL_WIDTH } = LAYOUT_CONSTANTS;

  describe('Focus Mode', () => {
    it('should return minimal insets in focus mode', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: true,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });
  });

  describe('Normal Mode - Sidebar Open', () => {
    it('should account for sidebar when open, no selection', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: false,
        hasSelection: false,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${SIDEBAR_WIDTH + BASE_MARGIN}px`,
      });
    });

    it('should account for sidebar and panel when both open', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: false,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${DETAILS_PANEL_WIDTH + BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${SIDEBAR_WIDTH + BASE_MARGIN}px`,
      });
    });
  });

  describe('Normal Mode - Sidebar Closed', () => {
    it('should use minimal left inset when sidebar closed', () => {
      const result = calculateViewportInsets({
        sidebarOpen: false,
        focusMode: false,
        hasSelection: false,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });

    it('should account for panel when sidebar closed but selection active', () => {
      const result = calculateViewportInsets({
        sidebarOpen: false,
        focusMode: false,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${DETAILS_PANEL_WIDTH + BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });
  });
});
```

**Step 2: Run test to verify it fails**

```bash
npm test -- src/hooks/__tests__/useViewportInsets.test.ts
```

Expected: FAIL with "Cannot find module '../useViewportInsets'"

**Step 3: Write the implementation**

```typescript
// src/hooks/useViewportInsets.ts
'use client';

/**
 * useViewportInsets - Dynamic viewport padding for fitView
 *
 * Calculates pixel-based insets based on current UI state:
 * - Sidebar open/closed
 * - Details panel open/closed (node/edge selected)
 * - Focus mode (all UI hidden)
 *
 * @version 1.0.0
 */

import { useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useUIStore } from '@/stores';

// =============================================================================
// Layout Constants (pixels)
// =============================================================================

const BASE_MARGIN = 16;
const TOP_BAR_HEIGHT = 120;
const BOTTOM_BAR_HEIGHT = 80;
const SIDEBAR_WIDTH = 288;
const DETAILS_PANEL_WIDTH = 420;

// =============================================================================
// Types
// =============================================================================

export interface ViewportInsets {
  top: string;
  right: string;
  bottom: string;
  left: string;
}

export interface UIState {
  sidebarOpen: boolean;
  focusMode: boolean;
  hasSelection: boolean;
}

// =============================================================================
// Pure calculation function (testable)
// =============================================================================

export function calculateViewportInsets(state: UIState): ViewportInsets {
  if (state.focusMode) {
    return {
      top: `${BASE_MARGIN}px`,
      right: `${BASE_MARGIN}px`,
      bottom: `${BASE_MARGIN}px`,
      left: `${BASE_MARGIN}px`,
    };
  }

  const top = TOP_BAR_HEIGHT + BASE_MARGIN;
  const bottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN;
  const left = state.sidebarOpen ? SIDEBAR_WIDTH + BASE_MARGIN : BASE_MARGIN;
  const right = state.hasSelection ? DETAILS_PANEL_WIDTH + BASE_MARGIN : BASE_MARGIN;

  return {
    top: `${top}px`,
    right: `${right}px`,
    bottom: `${bottom}px`,
    left: `${left}px`,
  };
}

// =============================================================================
// React Hook
// =============================================================================

export function useViewportInsets(): ViewportInsets {
  const { sidebarOpen, focusMode, selectedNodeId, selectedEdgeId } = useUIStore(
    useShallow((state) => ({
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      selectedNodeId: state.selectedNodeId,
      selectedEdgeId: state.selectedEdgeId,
    }))
  );

  const hasSelection = selectedNodeId !== null || selectedEdgeId !== null;

  return useMemo(
    () => calculateViewportInsets({ sidebarOpen, focusMode, hasSelection }),
    [sidebarOpen, focusMode, hasSelection]
  );
}

// =============================================================================
// Exported constants for testing
// =============================================================================

export const LAYOUT_CONSTANTS = {
  BASE_MARGIN,
  TOP_BAR_HEIGHT,
  BOTTOM_BAR_HEIGHT,
  SIDEBAR_WIDTH,
  DETAILS_PANEL_WIDTH,
} as const;
```

**Step 4: Run test to verify it passes**

```bash
npm test -- src/hooks/__tests__/useViewportInsets.test.ts
```

Expected: PASS (all 5 tests)

**Step 5: Add export to index**

```typescript
// Add to src/hooks/index.ts
export {
  useViewportInsets,
  calculateViewportInsets,
  LAYOUT_CONSTANTS,
  type ViewportInsets,
  type UIState,
} from './useViewportInsets';
```

**Step 6: Commit**

```bash
git add src/hooks/useViewportInsets.ts src/hooks/__tests__/useViewportInsets.test.ts src/hooks/index.ts
git commit -m "feat(viewport): add useViewportInsets hook with dynamic inset calculation

- Calculate pixel-based insets based on UI state
- Support focus mode, sidebar, and details panel states
- Export pure function for testing
- Add comprehensive unit tests

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Create useCenterOnNode Hook

**Files:**
- Create: `src/hooks/useCenterOnNode.ts`
- Modify: `src/hooks/index.ts` (add export)
- Test: `src/hooks/__tests__/useCenterOnNode.test.ts`

**Step 1: Write the failing test**

```typescript
// src/hooks/__tests__/useCenterOnNode.test.ts
import { calculateCenterOffset, LAYOUT_CONSTANTS } from '../useCenterOnNode';

describe('calculateCenterOffset', () => {
  const { BASE_MARGIN, TOP_BAR_HEIGHT, BOTTOM_BAR_HEIGHT, SIDEBAR_WIDTH, DETAILS_PANEL_WIDTH } = LAYOUT_CONSTANTS;

  it('should calculate offset to center node in visible area', () => {
    // With sidebar open (304px) and panel open (436px)
    // Horizontal: (304 - 436) / 2 = -66px (shift left because panel is wider)
    // Vertical: (136 - 96) / 2 = 20px (shift down because top is taller)
    const offset = calculateCenterOffset({
      sidebarOpen: true,
      focusMode: false,
      hasSelection: true,
    });

    const expectedLeft = SIDEBAR_WIDTH + BASE_MARGIN; // 304
    const expectedRight = DETAILS_PANEL_WIDTH + BASE_MARGIN; // 436
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // -66
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });

  it('should return zero offset in focus mode', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: true,
      focusMode: true,
      hasSelection: true,
    });

    expect(offset.x).toBe(0);
    expect(offset.y).toBe(0);
  });

  it('should calculate offset with sidebar closed and panel open', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: false,
      focusMode: false,
      hasSelection: true,
    });

    const expectedLeft = BASE_MARGIN; // 16
    const expectedRight = DETAILS_PANEL_WIDTH + BASE_MARGIN; // 436
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // -210
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });
});
```

**Step 2: Run test to verify it fails**

```bash
npm test -- src/hooks/__tests__/useCenterOnNode.test.ts
```

Expected: FAIL with "Cannot find module '../useCenterOnNode'"

**Step 3: Write the implementation**

```typescript
// src/hooks/useCenterOnNode.ts
'use client';

/**
 * useCenterOnNode - Center viewport on a specific node
 *
 * Calculates offset compensation for UI overlays and provides
 * a function to smoothly center + zoom on a node.
 *
 * @version 1.0.0
 */

import { useCallback } from 'react';
import { useReactFlow } from '@xyflow/react';
import { useShallow } from 'zustand/react/shallow';
import { useUIStore } from '@/stores';

// =============================================================================
// Layout Constants (same as useViewportInsets)
// =============================================================================

const BASE_MARGIN = 16;
const TOP_BAR_HEIGHT = 120;
const BOTTOM_BAR_HEIGHT = 80;
const SIDEBAR_WIDTH = 288;
const DETAILS_PANEL_WIDTH = 420;

// =============================================================================
// Types
// =============================================================================

export interface CenterOffset {
  x: number;
  y: number;
}

export interface UIState {
  sidebarOpen: boolean;
  focusMode: boolean;
  hasSelection: boolean;
}

export interface CenterOnNodeOptions {
  zoom?: number;
  duration?: number;
}

// =============================================================================
// Pure calculation function (testable)
// =============================================================================

/**
 * Calculate the offset needed to center content in the visible area.
 *
 * The visible area is the viewport minus the UI overlays.
 * We need to shift the center point by half the difference between
 * left/right and top/bottom insets.
 */
export function calculateCenterOffset(state: UIState): CenterOffset {
  if (state.focusMode) {
    return { x: 0, y: 0 };
  }

  const top = TOP_BAR_HEIGHT + BASE_MARGIN;
  const bottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN;
  const left = state.sidebarOpen ? SIDEBAR_WIDTH + BASE_MARGIN : BASE_MARGIN;
  const right = state.hasSelection ? DETAILS_PANEL_WIDTH + BASE_MARGIN : BASE_MARGIN;

  // Offset = (left - right) / 2 for horizontal centering
  // Positive = shift right, Negative = shift left
  return {
    x: (left - right) / 2,
    y: (top - bottom) / 2,
  };
}

// =============================================================================
// React Hook
// =============================================================================

export function useCenterOnNode() {
  const { setCenter, getZoom } = useReactFlow();

  const { sidebarOpen, focusMode, selectedNodeId, selectedEdgeId } = useUIStore(
    useShallow((state) => ({
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      selectedNodeId: state.selectedNodeId,
      selectedEdgeId: state.selectedEdgeId,
    }))
  );

  const centerOnNode = useCallback(
    (
      nodeX: number,
      nodeY: number,
      nodeWidth: number,
      nodeHeight: number,
      options?: CenterOnNodeOptions
    ) => {
      const hasSelection = selectedNodeId !== null || selectedEdgeId !== null;
      const offset = calculateCenterOffset({ sidebarOpen, focusMode, hasSelection });

      const currentZoom = getZoom();
      const targetZoom = options?.zoom ?? Math.max(currentZoom, 1.2);
      const duration = options?.duration ?? 400;

      // Node center position
      const nodeCenterX = nodeX + nodeWidth / 2;
      const nodeCenterY = nodeY + nodeHeight / 2;

      // Compensate for UI overlays by adjusting the center point
      // We need to convert pixel offset to flow coordinates (divide by zoom)
      const adjustedX = nodeCenterX + offset.x / targetZoom;
      const adjustedY = nodeCenterY + offset.y / targetZoom;

      setCenter(adjustedX, adjustedY, {
        zoom: targetZoom,
        duration,
      });
    },
    [setCenter, getZoom, sidebarOpen, focusMode, selectedNodeId, selectedEdgeId]
  );

  return { centerOnNode };
}

// =============================================================================
// Exported constants for testing
// =============================================================================

export const LAYOUT_CONSTANTS = {
  BASE_MARGIN,
  TOP_BAR_HEIGHT,
  BOTTOM_BAR_HEIGHT,
  SIDEBAR_WIDTH,
  DETAILS_PANEL_WIDTH,
} as const;
```

**Step 4: Run test to verify it passes**

```bash
npm test -- src/hooks/__tests__/useCenterOnNode.test.ts
```

Expected: PASS (all 3 tests)

**Step 5: Add export to index**

```typescript
// Add to src/hooks/index.ts
export {
  useCenterOnNode,
  calculateCenterOffset,
  type CenterOffset,
  type CenterOnNodeOptions,
} from './useCenterOnNode';
```

**Step 6: Commit**

```bash
git add src/hooks/useCenterOnNode.ts src/hooks/__tests__/useCenterOnNode.test.ts src/hooks/index.ts
git commit -m "feat(viewport): add useCenterOnNode hook for node-focused navigation

- Calculate offset compensation for UI overlays
- Center + zoom on node with smooth animation
- Export pure function for testing
- Add unit tests for offset calculations

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Create useSmartFitView Hook

**Files:**
- Create: `src/hooks/useSmartFitView.ts`
- Modify: `src/hooks/index.ts` (add export)

**Step 1: Write the implementation**

```typescript
// src/hooks/useSmartFitView.ts
'use client';

/**
 * useSmartFitView - Intelligent fitView with dynamic insets
 *
 * Wraps React Flow's fitView with automatic inset calculation
 * based on current UI state.
 *
 * @version 1.0.0
 */

import { useCallback } from 'react';
import { useReactFlow } from '@xyflow/react';
import { useViewportInsets } from './useViewportInsets';

export interface SmartFitViewOptions {
  duration?: number;
  maxZoom?: number;
  minZoom?: number;
  includeHiddenNodes?: boolean;
}

export function useSmartFitView() {
  const { fitView } = useReactFlow();
  const insets = useViewportInsets();

  const smartFitView = useCallback(
    (options?: SmartFitViewOptions) => {
      fitView({
        padding: insets,
        duration: options?.duration ?? 400,
        maxZoom: options?.maxZoom ?? 1.5,
        minZoom: options?.minZoom ?? 0.1,
        includeHiddenNodes: options?.includeHiddenNodes ?? false,
      });
    },
    [fitView, insets]
  );

  return { smartFitView, insets };
}
```

**Step 2: Add export to index**

```typescript
// Add to src/hooks/index.ts
export { useSmartFitView, type SmartFitViewOptions } from './useSmartFitView';
```

**Step 3: Commit**

```bash
git add src/hooks/useSmartFitView.ts src/hooks/index.ts
git commit -m "feat(viewport): add useSmartFitView hook wrapping fitView with insets

- Automatically apply dynamic insets to fitView
- Configurable duration, zoom limits
- Smooth 400ms default animation

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Update GraphToolbar to Use Smart FitView

**Files:**
- Modify: `src/components/graph/GraphToolbar.tsx`

**Step 1: Update the component**

```typescript
// In src/components/graph/GraphToolbar.tsx

// Add import
import { useSmartFitView } from '@/hooks';

// Replace in GraphToolbar component:
// OLD:
// const { zoomIn, zoomOut, fitView } = useReactFlow();
// NEW:
const { zoomIn, zoomOut } = useReactFlow();
const { smartFitView } = useSmartFitView();

// Replace handleFitView:
// OLD:
// const handleFitView = useCallback(() => {
//   fitView({ padding: 0.3, duration: 400 });
// }, [fitView]);
// NEW:
const handleFitView = useCallback(() => {
  smartFitView({ duration: 400 });
}, [smartFitView]);
```

**Step 2: Commit**

```bash
git add src/components/graph/GraphToolbar.tsx
git commit -m "feat(toolbar): use smartFitView with dynamic insets

- Replace static padding with dynamic inset calculation
- FitView now respects sidebar and panel states

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Update Graph2D for Double-Click Behavior

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Update imports and add hook**

```typescript
// Add to imports
import { useSmartFitView, useCenterOnNode } from '@/hooks';

// Inside Graph2DInner component, add:
const { smartFitView } = useSmartFitView();
const { centerOnNode } = useCenterOnNode();
```

**Step 2: Update onNodeDoubleClick handler**

The current `onNodeDoubleClick` expands neighbors. We need to ALSO center on the node.

```typescript
// Find the existing onNodeDoubleClick handler and wrap it:
const handleNodeDoubleClick = useCallback(
  (event: React.MouseEvent, node: TurboNodeType) => {
    // Select the node (opens panel)
    setSelectedNode(node.id);

    // Center on the node with zoom
    const nodeWidth = node.measured?.width ?? 240;
    const nodeHeight = node.measured?.height ?? 140;

    // Small delay to let panel animation start
    requestAnimationFrame(() => {
      centerOnNode(
        node.position.x,
        node.position.y,
        nodeWidth,
        nodeHeight,
        { zoom: 1.3, duration: 400 }
      );
    });

    // Call original expand handler if provided
    onNodeDoubleClick?.(node.id);
  },
  [setSelectedNode, centerOnNode, onNodeDoubleClick]
);
```

**Step 3: Update ReactFlow props**

```typescript
<ReactFlow
  // ... existing props
  onNodeDoubleClick={handleNodeDoubleClick}
  // Remove onNodeClick from opening panel (single click = drag only)
>
```

**Step 4: Commit**

```bash
git add src/components/graph/Graph2D.tsx
git commit -m "feat(graph): double-click centers + zooms on node

- Double-click opens panel AND centers on node
- Single-click no longer opens panel (drag only)
- Smooth 400ms animation with 1.3x zoom

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Add Auto-FitView on UI State Changes

**Files:**
- Modify: `src/components/graph/Graph2D.tsx`

**Step 1: Add effect for sidebar toggle**

```typescript
// Add refs to track previous state
const prevSidebarOpenRef = useRef(sidebarOpen);
const prevFocusModeRef = useRef(focusMode);
const prevHasSelectionRef = useRef(false);

// Add effect to auto-fitView on UI changes
useEffect(() => {
  const hasSelection = selectedNodeId !== null;
  const prevHasSelection = prevHasSelectionRef.current;

  // Check if any relevant state changed
  const sidebarChanged = prevSidebarOpenRef.current !== sidebarOpen;
  const focusModeChanged = prevFocusModeRef.current !== focusMode;
  const selectionClosed = prevHasSelection && !hasSelection;

  // Update refs
  prevSidebarOpenRef.current = sidebarOpen;
  prevFocusModeRef.current = focusMode;
  prevHasSelectionRef.current = hasSelection;

  // Trigger fitView on relevant changes
  if (sidebarChanged || focusModeChanged || selectionClosed) {
    // Small delay to let CSS animations start
    const timeoutId = setTimeout(() => {
      smartFitView({ duration: 400 });
    }, 50);

    return () => clearTimeout(timeoutId);
  }
}, [sidebarOpen, focusMode, selectedNodeId, smartFitView]);
```

**Step 2: Commit**

```bash
git add src/components/graph/Graph2D.tsx
git commit -m "feat(graph): auto-fitView on sidebar/panel/focus changes

- Trigger smooth fitView when sidebar toggles
- Trigger smooth fitView when focus mode changes
- Trigger smooth fitView when panel closes

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Update Page.tsx for Single-Click Behavior

**Files:**
- Modify: `src/app/page.tsx`

**Step 1: Update node click handler**

```typescript
// Change handleNodeClick to NOT open panel
// OLD:
// const handleNodeClick = useCallback(
//   (nodeId: string) => {
//     uiActions.setSelectedNode(nodeId);
//   },
//   [uiActions]
// );

// NEW: Single click does nothing (handled by Graph2D for selection highlight only)
// Remove handleNodeClick or make it a no-op for panel
```

**Step 2: Verify double-click handler stays for expand**

The `handleExpandNode` should remain for expanding neighbors, but Graph2D will also center on the node.

**Step 3: Commit**

```bash
git add src/app/page.tsx
git commit -m "refactor(page): single-click no longer opens details panel

- Double-click now handles panel opening + centering
- Single-click reserved for drag & drop only

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 8: Write E2E Tests with Playwright

**Files:**
- Create: `e2e/smart-fitview.spec.ts`

**Step 1: Write E2E tests**

```typescript
// e2e/smart-fitview.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Smart FitView with Dynamic Insets', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for graph to load
    await page.waitForSelector('[data-testid="react-flow"]', { timeout: 10000 });
  });

  test('fitView respects sidebar state', async ({ page }) => {
    // Get initial viewport
    const initialViewport = await page.evaluate(() => {
      const rf = document.querySelector('.react-flow__viewport');
      return rf?.getAttribute('style');
    });

    // Close sidebar
    await page.keyboard.press('[');
    await page.waitForTimeout(500);

    // Get new viewport after fitView
    const newViewport = await page.evaluate(() => {
      const rf = document.querySelector('.react-flow__viewport');
      return rf?.getAttribute('style');
    });

    // Viewport should have changed (more space available)
    expect(newViewport).not.toBe(initialViewport);
  });

  test('double-click on node opens panel and centers', async ({ page }) => {
    // Find a node and double-click
    const node = page.locator('.react-flow__node').first();
    await node.dblclick();

    // Panel should open
    await expect(page.locator('text=Node Details')).toBeVisible({ timeout: 1000 });

    // Node should be roughly centered (check transform)
    await page.waitForTimeout(500);
    // Verify viewport changed
  });

  test('closing panel triggers fitView', async ({ page }) => {
    // Open panel first
    const node = page.locator('.react-flow__node').first();
    await node.dblclick();
    await expect(page.locator('text=Node Details')).toBeVisible();

    // Get viewport with panel open
    const viewportWithPanel = await page.evaluate(() => {
      const rf = document.querySelector('.react-flow__viewport');
      return rf?.getAttribute('style');
    });

    // Close panel with ]
    await page.keyboard.press(']');
    await page.waitForTimeout(500);

    // Get viewport after panel closed
    const viewportAfterClose = await page.evaluate(() => {
      const rf = document.querySelector('.react-flow__viewport');
      return rf?.getAttribute('style');
    });

    // Viewport should have changed
    expect(viewportAfterClose).not.toBe(viewportWithPanel);
  });

  test('focus mode uses minimal insets', async ({ page }) => {
    // Enter focus mode
    await page.keyboard.press('g');
    await page.waitForTimeout(500);

    // Verify sidebar is hidden
    await expect(page.locator('aside').first()).not.toBeVisible();

    // Exit focus mode
    await page.keyboard.press('g');
    await page.waitForTimeout(500);

    // Verify sidebar is back
    await expect(page.locator('aside').first()).toBeVisible();
  });
});
```

**Step 2: Run tests**

```bash
npx playwright test e2e/smart-fitview.spec.ts
```

**Step 3: Commit**

```bash
git add e2e/smart-fitview.spec.ts
git commit -m "test(e2e): add Playwright tests for smart fitView

- Test sidebar toggle triggers fitView
- Test double-click centers on node
- Test panel close triggers fitView
- Test focus mode uses minimal insets

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 9: Final Integration Test & Code Review

**Step 1: Run all tests**

```bash
npm test
npx playwright test
```

**Step 2: Run type check**

```bash
npm run type-check
```

**Step 3: Manual testing checklist**

- [ ] Press F or click FitView button → graph fits with correct insets
- [ ] Press [ to toggle sidebar → graph re-fits smoothly
- [ ] Double-click node → panel opens, node centers with zoom
- [ ] Press ] to close panel → graph re-fits smoothly
- [ ] Press G for focus mode → graph uses full screen
- [ ] Change layout (⇧H/V/D/R/F) → graph re-fits after layout

**Step 4: Code review**

Use `superpowers:code-reviewer` to review the implementation.

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(viewport): complete smart fitView implementation

Summary:
- useViewportInsets: dynamic inset calculation
- useCenterOnNode: node-focused navigation with offset
- useSmartFitView: fitView wrapper with auto-insets
- Auto-fitView on sidebar/panel/focus changes
- Double-click centers + zooms on node
- E2E tests for all scenarios

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Verification Commands

```bash
# Unit tests
npm test

# E2E tests
npx playwright test

# Type check
npm run type-check

# Lint
npm run lint

# Dev server for manual testing
npm run dev
```
