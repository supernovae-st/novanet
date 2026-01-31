# Schema View Interactive - Design Plan

**Date:** 2026-01-31
**Status:** Approved
**Author:** Claude + Thibaut

## Overview

Make the Schema view interactive with draggable nodes, dynamic container resizing, and click interactions for viewing schema details.

## Goals

1. **Draggable nodes** - Schema nodes can be moved on canvas like data view
2. **Dynamic containers** - Containers grow/shrink when nodes approach edges
3. **Click interactions** - View node/edge schema details in right panel
4. **Unified style** - Same TurboNode visual style as data view

## Implementation Tasks

### Task 1: Enable Schema Node Dragging

**File:** `src/components/graph/Graph2D.tsx`

**Changes:**
- Change `nodesDraggable={false}` to `nodesDraggable={true}` in schema mode
- Add `onNodesChange` handler for schema mode
- Track schema node positions in local state (not persisted)

**Acceptance:**
- [ ] Schema nodes can be dragged
- [ ] Positions update smoothly during drag
- [ ] Other nodes don't move when one is dragged

---

### Task 2: Create useContainerConstraint Hook

**File:** `src/hooks/useContainerConstraint.ts` (NEW)

**Purpose:** Manage dynamic container resizing when nodes approach edges.

**React Flow API (from Context7):**
```typescript
// Drag event handlers
type OnNodeDrag = (event: React.MouseEvent, node: Node, nodes: Node[]) => void;
type OnNodeDragStop = (event: React.MouseEvent, node: Node) => void;

// Intersection detection
const { getIntersectingNodes } = useReactFlow();
```

**Interface:**
```typescript
interface ContainerConstraintOptions {
  /** Distance from edge to trigger expansion (default: 50px) */
  edgeThreshold?: number;
  /** Minimum container padding (default: 40px) */
  minPadding?: number;
  /** Animation duration (default: 200ms) */
  animationDuration?: number;
}

interface UseContainerConstraintReturn {
  /** Call during node drag to check/update containers */
  handleNodeDrag: (event: React.MouseEvent, node: Node) => void;
  /** Call when drag ends to potentially shrink containers */
  handleNodeDragStop: (event: React.MouseEvent, node: Node) => void;
  /** Current container dimensions (reactive) */
  containerSizes: Map<string, { width: number; height: number }>;
  /** Update schema nodes with new positions */
  schemaNodes: Node[];
  setSchemaNodes: React.Dispatch<React.SetStateAction<Node[]>>;
}
```

**Logic:**
1. Track all container bounds (scope groups)
2. On node drag:
   - Check if node is within `edgeThreshold` of any container edge
   - If yes, expand container in that direction
   - Push adjacent containers to avoid overlap
3. On drag end:
   - Calculate minimum bounding box for all nodes in container
   - Shrink container to fit (with padding)
   - Re-adjust adjacent containers

**Acceptance:**
- [ ] Hook exports correct interface
- [ ] Container expands when node approaches edge
- [ ] Adjacent containers are pushed smoothly
- [ ] Containers shrink when node is released

---

### Task 3: Integrate Container Constraint with Graph2D

**File:** `src/components/graph/Graph2D.tsx`

**Changes:**
- Import and use `useContainerConstraint` hook
- Wire up `onNodeDrag` and `onNodeDragEnd` callbacks
- Update schema node positions based on container sizes
- Add CSS transitions for smooth container animations

**Schema mode render updates:**
```tsx
// Before
nodesDraggable={false}

// After
nodesDraggable={true}
onNodesChange={handleSchemaNodesChange}
onNodeDrag={handleSchemaNodeDrag}
onNodeDragStop={handleSchemaNodeDragEnd}
```

**Acceptance:**
- [ ] Container resizing works during drag
- [ ] Animations are smooth (no jank)
- [ ] Multiple containers can be affected simultaneously

---

### Task 4: Click Interactions for Schema Details

**File:** `src/components/graph/Graph2D.tsx`

**Changes:**
- Add `onNodeClick` handler for schema mode
- Add `onEdgeClick` handler for schema mode
- Set `selectedNodeId` / `selectedEdgeId` in uiStore
- Panel will show schema details (existing InfoPanel logic)

**Schema node click data:**
```typescript
interface SchemaNodeDetails {
  type: string;           // "Concept", "Project", etc.
  scope: Scope;           // "Project" | "Global" | "Shared"
  properties: Property[]; // Schema properties
  incomingRelations: RelationType[];
  outgoingRelations: RelationType[];
}
```

**Schema edge click data:**
```typescript
interface SchemaEdgeDetails {
  relationType: string;   // "HAS_CONCEPT", etc.
  sourceType: string;
  targetType: string;
  cardinality?: string;   // "1:N", "N:M", etc.
}
```

**Acceptance:**
- [ ] Clicking schema node opens panel with details
- [ ] Clicking schema edge opens panel with relation details
- [ ] Selection state is visually indicated

---

### Task 5: Update Schema Node Styling

**Files:**
- `src/components/graph/schema/SchemaNode.tsx`
- `src/components/graph/schema/ScopeGroupNode.tsx`
- `src/components/graph/schema/SubcategoryGroupNode.tsx`

**Changes:**
- Apply TurboNode-like styling (glow, gradients)
- Use scope colors consistently:
  - Project: violet (`#8b5cf6`)
  - Global: emerald (`#10b981`)
  - Shared: amber (`#f59e0b`)
- Add hover/selected states matching data view
- Container glass effect with scope-colored border

**Acceptance:**
- [ ] Schema nodes look similar to data view nodes
- [ ] Scope colors are consistent
- [ ] Hover/selected states work
- [ ] Containers have glass effect

---

## File Changes Summary

| File | Action | Description |
|------|--------|-------------|
| `src/hooks/useContainerConstraint.ts` | CREATE | Dynamic container resizing logic |
| `src/hooks/index.ts` | MODIFY | Export new hook |
| `src/components/graph/Graph2D.tsx` | MODIFY | Enable dragging, add handlers |
| `src/components/graph/schema/SchemaNode.tsx` | MODIFY | TurboNode styling |
| `src/components/graph/schema/ScopeGroupNode.tsx` | MODIFY | Glass effect, colors |
| `src/components/graph/schema/SubcategoryGroupNode.tsx` | MODIFY | Glass effect, colors |

## Testing Plan

1. **Unit tests** for `useContainerConstraint` hook
2. **Visual testing** - drag nodes, verify container behavior
3. **E2E test** - schema view interaction flow

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Performance with many nodes | Use RAF for drag updates, debounce resize |
| Container overlap edge cases | Add minimum container size constraints |
| State sync issues | Keep container sizes in local component state |

## Definition of Done

- [ ] All 5 tasks completed
- [ ] Unit tests pass
- [ ] E2E tests pass
- [ ] Visual review approved
- [ ] No performance regression
