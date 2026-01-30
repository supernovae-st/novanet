# NovaNet Visualizer Improvement Roadmap

**Created**: 2025-01-23
**Status**: Draft
**Authors**: Thibaut + Claude

---

## Executive Summary

After 22 rounds of code review and parallel exploration across 4 axes (Performance, Testing, Architecture, Code Quality), this plan consolidates all improvement opportunities into a prioritized roadmap.

**Key Metrics:**
- 22 bugs fixed in code review rounds
- ~400KB potential bundle savings identified
- Test coverage: ~15% → target 80%
- 51 type safety improvements identified

---

## Phase 1: Quick Wins (1-2 hours)

### 1.1 Fix DatabaseInfoPanel Effect Dependencies
**File**: `src/components/sidebar/DatabaseInfoPanel.tsx:55-71`
**Issue**: Missing `setSelectedLabels`, `setSelectedRelTypes` in useEffect deps
**Effort**: 5 min

```typescript
// Before
}, [schema]);

// After
}, [schema, setSelectedLabels, setSelectedRelTypes]);
```

### 1.2 Add Barrel Exports
**Effort**: 15 min

Create `src/lib/index.ts`:
```typescript
// Core utilities
export { cn, generateId } from './utils';
export { logger } from './logger';

// Data fetching
export { fetchJSON, postJSON, getErrorMessage, FetchError } from './fetchClient';

// Clipboard
export { copyToClipboard, copyNodeProperties } from './clipboard';

// Layout
export { applyDagreLayout } from './layout';
export { createForceSimulation, runSimulationSync, applyForcePositions } from './forceSimulation';

// Validation
export { validateCypher } from './cypherValidator';

// Search
export { fuzzyMatch, fuzzySearch } from './fuzzySearch';
```

Create `src/components/sidebar/index.ts`:
```typescript
export { DatabaseInfoPanel } from './DatabaseInfoPanel';
export { NodeDetailsPanel } from './NodeDetailsPanel';
export { EdgeDetailsPanel } from './EdgeDetailsPanel';
export { FilterPanel } from './FilterPanel';
// ... etc
```

### 1.3 Debounce Filter Search
**File**: `src/stores/filterStore.ts`
**Effort**: 15 min

Add debounced search action:
```typescript
// In filterStore
setSearchQueryDebounced: (query: string) => void;

// Implementation with useTimeout or lodash.debounce
```

### 1.4 Fix queryStore to Use fetchJSON
**File**: `src/stores/queryStore.ts:83-88`
**Effort**: 10 min

```typescript
// Before
const response = await fetch('/api/graph/query', { ... });

// After
import { postJSON } from '@/lib/fetchClient';
const result = await postJSON<QueryResponse>('/api/graph/query', { cypher: query }, { signal });
```

---

## Phase 2: Testing Foundation (4-6 hours)

### 2.1 Security-Critical: cypherValidator Tests
**File**: `src/lib/__tests__/cypherValidator.test.ts`
**Priority**: CRITICAL (security)
**Effort**: 1h

Test cases:
- Valid read queries pass
- Write operations blocked (CREATE, DELETE, SET, MERGE, REMOVE)
- Injection attempts blocked
- Unicode bypass attempts
- Edge cases in regex matching

```typescript
describe('cypherValidator', () => {
  describe('valid queries', () => {
    it('allows MATCH...RETURN', () => {});
    it('allows WITH clauses', () => {});
    it('allows OPTIONAL MATCH', () => {});
  });

  describe('blocked operations', () => {
    it('blocks CREATE', () => {});
    it('blocks DELETE', () => {});
    it('blocks SET', () => {});
    it('blocks CALL with write procedures', () => {});
  });

  describe('injection prevention', () => {
    it('blocks string injection attempts', () => {});
    it('blocks comment-based bypasses', () => {});
  });
});
```

### 2.2 Data Integrity: graphStore Tests
**File**: `src/stores/__tests__/graphStore.test.ts`
**Priority**: CRITICAL
**Effort**: 1.5h

Test cases:
- setGraphData builds correct indexes
- mergeGraphData deduplicates correctly
- Map consistency after operations
- getNodeDetail returns correct data
- hideNode removes from all indexes
- Empty data handling

### 2.3 Filter Logic: filterStore Tests
**File**: `src/stores/__tests__/filterStore.test.ts`
**Priority**: HIGH
**Effort**: 1.5h

Test cases:
- Preset application
- localStorage persistence/hydration
- Set serialization (toJSON/fromJSON)
- enabledNodeTypes toggle
- toCypher() generation
- toNovaNetFilter() adapter

### 2.4 Performance: useFilteredGraph Tests
**File**: `src/hooks/__tests__/useFilteredGraph.test.ts`
**Priority**: HIGH
**Effort**: 1h

Test cases:
- 4-stage filter pipeline
- Edge filtering by visible nodes
- Empty graph handling
- All nodes filtered out
- Locale filtering

---

## Phase 3: Performance Optimizations (3-4 hours)

### 3.1 Lazy Load Graph2D Component
**File**: `src/app/page.tsx`
**Impact**: ~400KB bundle reduction
**Effort**: 30 min

```typescript
import dynamic from 'next/dynamic';

const Graph2D = dynamic(
  () => import('@/components/graph/Graph2D').then(mod => ({ default: mod.Graph2D })),
  {
    loading: () => <GraphLoadingSkeleton />,
    ssr: false
  }
);
```

Create `src/components/graph/GraphLoadingSkeleton.tsx`:
```typescript
export function GraphLoadingSkeleton() {
  return (
    <div className="h-full w-full flex items-center justify-center bg-black/50">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="w-8 h-8 animate-spin text-novanet-400" />
        <span className="text-white/60 text-sm">Loading graph...</span>
      </div>
    </div>
  );
}
```

### 3.2 Virtualize Node Labels List
**File**: `src/components/sidebar/database/NodeLabelsSection.tsx`
**Impact**: Smooth scrolling with 100+ items
**Effort**: 1h

```typescript
import { useVirtualizer } from '@tanstack/react-virtual';

function NodeLabelsSection({ labels }: Props) {
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: labels.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 36, // row height
  });

  return (
    <div ref={parentRef} className="h-[300px] overflow-auto">
      <div style={{ height: virtualizer.getTotalSize() }}>
        {virtualizer.getVirtualItems().map((virtualRow) => (
          <NodeLabelRow
            key={labels[virtualRow.index].label}
            label={labels[virtualRow.index]}
            style={{ transform: `translateY(${virtualRow.start}px)` }}
          />
        ))}
      </div>
    </div>
  );
}
```

### 3.3 Lazy Compute Relations in NodeDetailsPanel
**File**: `src/components/sidebar/NodeDetailsPanel.tsx`
**Impact**: Faster panel open
**Effort**: 30 min

```typescript
// Only compute when expanded
const relatedData = useMemo(() => {
  if (!expandedSections.has('relations')) return null;

  const incomingEdges = edgesByTarget.get(node.id) || [];
  const outgoingEdges = edgesBySource.get(node.id) || [];
  // ... rest of computation
}, [expandedSections, node.id, edgesByTarget, edgesBySource]);
```

### 3.4 Debounce Command Palette Search
**File**: `src/components/ui/CommandPalette.tsx`
**Impact**: No input lag on slower devices
**Effort**: 15 min

```typescript
import { useDeferredValue } from 'react';

const deferredQuery = useDeferredValue(query);
const filteredCommands = useMemo(() => {
  if (!deferredQuery) return commands;
  return fuzzySearch(commands, deferredQuery, (cmd) => cmd.label);
}, [commands, deferredQuery]);
```

### 3.5 Virtualize Query Results Table
**File**: `src/components/query/QueryResultsTabs.tsx`
**Impact**: Handle 500+ row results
**Effort**: 1h

Similar approach to 3.2 using `@tanstack/react-virtual`.

---

## Phase 4: Architecture Improvements (2-3 hours)

### 4.1 Consolidate CopyButton Components
**Current**: 2 implementations with different APIs
**Effort**: 45 min

1. Keep `src/components/dx/CopyButton.tsx` as source of truth
2. Refactor to use `useCopyFeedback` hook internally
3. Export both controlled and uncontrolled variants
4. Update `src/components/ui/detail-panel/CopyButton.tsx` to re-export
5. Update all imports

### 4.2 Add Missing Store Selectors
**Files**: `src/stores/queryStore.ts`, `src/stores/aiQueryStore.ts`
**Effort**: 30 min

```typescript
// queryStore selectors
export const selectCurrentQuery = (state: QueryState) => state.currentQuery;
export const selectIsExecuting = (state: QueryState) => state.isExecuting;
export const selectResult = (state: QueryState) => state.result;
export const selectError = (state: QueryState) => state.error;

// Usage with useShallow
const { currentQuery, isExecuting } = useQueryStore(
  useShallow(state => ({
    currentQuery: selectCurrentQuery(state),
    isExecuting: selectIsExecuting(state),
  }))
);
```

### 4.3 Add Error Boundaries
**Effort**: 45 min

Wrap critical components:

```typescript
// src/app/page.tsx
<ErrorBoundary fallback={<GraphErrorFallback />}>
  <Graph2D />
</ErrorBoundary>

<ErrorBoundary fallback={<SidebarErrorFallback />}>
  <Sidebar />
</ErrorBoundary>

<ErrorBoundary fallback={<ChatErrorFallback />}>
  <AiChat />
</ErrorBoundary>
```

### 4.4 Type Safety: Replace Record<string, unknown>
**Effort**: 1h (incremental)

Priority files:
1. `src/types/index.ts` - Define NodeProperties union type
2. `src/stores/graphStore.ts` - Type Neo4j property bags
3. `src/lib/clipboard.ts` - Type property serialization

```typescript
// src/types/node-properties.ts
export interface BaseNodeProperties {
  key: string;
  display_name: string;
  llm_context?: string;
  priority?: 'critical' | 'high' | 'medium' | 'low';
  freshness?: 'realtime' | 'hourly' | 'daily' | 'static';
  created_at?: string;
  updated_at?: string;
}

export interface ProjectProperties extends BaseNodeProperties {
  description?: string;
  default_locale?: string;
}

// ... other node type properties

export type NodeProperties =
  | ProjectProperties
  | ConceptProperties
  | LocaleProperties
  | GenericProperties;
```

---

## Phase 5: Extended Testing (6-8 hours)

### 5.1 Hook Tests (Priority Order)
1. `useKeyboardHandler` - Event cleanup, memory leaks
2. `useCopyFeedback` - Clipboard + toast logic
3. `useTriStateSelection` - Complex tri-state logic
4. `useModal` - Dialog lifecycle
5. `useSmartFitView` - Viewport calculation
6. `useDatabaseSchema` - API call + caching
7. `useTimeout` - Timer cleanup
8. `useEscapeKey` - Event cleanup

### 5.2 API Route Tests
1. `/api/graph/route.ts` - GET/POST validation
2. `/api/chat/route.ts` - Claude integration
3. `/api/graph/expand/route.ts` - Neighbor expansion
4. `/api/graph/stats/route.ts` - Statistics

### 5.3 Component Tests (Priority Order)
1. `CommandPalette` - Fuzzy search, keyboard nav
2. `DatabaseInfoPanel` - Multi-select logic
3. `QueryResultsTabs` - Tab switching
4. `GraphToolbar` - Action buttons
5. `NodeContextMenu` - Context menu logic

---

## Phase 6: Documentation (1 hour)

### 6.1 Update CLAUDE.md
Add sections:
- Store patterns (when to use persist, Map/Set serialization)
- Hook composition patterns
- Testing conventions
- Performance optimization patterns

### 6.2 Create ARCHITECTURE.md
Document:
- Data flow diagram
- Store relationships
- Component hierarchy
- API route structure

---

## Implementation Schedule

| Phase | Effort | Priority | Dependencies |
|-------|--------|----------|--------------|
| Phase 1: Quick Wins | 1-2h | NOW | None |
| Phase 2: Testing Foundation | 4-6h | HIGH | None |
| Phase 3: Performance | 3-4h | MEDIUM | Phase 1 |
| Phase 4: Architecture | 2-3h | MEDIUM | Phase 1 |
| Phase 5: Extended Testing | 6-8h | LOW | Phase 2 |
| Phase 6: Documentation | 1h | LOW | Phase 4 |

**Total Estimated Effort**: 17-24 hours

---

## Success Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Test Coverage | ~15% | 80% |
| Bundle Size (initial) | ~800KB | ~400KB |
| Lighthouse Performance | TBD | 90+ |
| Type Safety (any usage) | 14 files | 0 files |
| Code Review Issues | 23 fixed | 0 pending |

---

## Appendix A: Files by Priority

### Critical (test immediately)
- `src/lib/cypherValidator.ts` - Security
- `src/stores/graphStore.ts` - Data integrity
- `src/stores/filterStore.ts` - User preferences
- `src/lib/filterAdapter.ts` - Cypher generation

### High (next sprint)
- `src/hooks/useFilteredGraph.ts` - Performance
- `src/hooks/useKeyboardHandler.ts` - Event handling
- `src/lib/fuzzySearch.ts` - Search quality
- `src/app/api/chat/route.ts` - AI integration

### Medium (technical debt)
- All remaining hooks (14)
- Component tests (21)
- Type safety improvements

---

## Appendix B: Dependencies to Add

```bash
# Virtualization (if not present)
npm install @tanstack/react-virtual

# Testing utilities (if needed)
npm install -D @testing-library/react-hooks
```

---

*This plan will be updated as work progresses. Use `/commit` to track completed phases.*
