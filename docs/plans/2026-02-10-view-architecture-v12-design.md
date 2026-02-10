# View Architecture v12 - Clean Slate Design

**Date**: 2026-02-10
**Version**: v12.0.0
**Status**: Ready for Implementation
**Philosophy**: No legacy, no backward compatibility, clean architecture

---

## Executive Summary

Replace the META/DATA navigation mode system with a unified VIEW-based architecture. Views become the single source of truth for graph visualization. Custom Cypher queries override views temporarily.

```
BEFORE (v11.x):                    AFTER (v12.0):
┌─────────────────────┐            ┌─────────────────────────────────────┐
│ [META] [DATA]       │            │ ◀ complete │ project │ block │ ▶   │
│ navigationMode      │     →      │     (horizontal scrollable views)   │
│ clearGraph()        │            │ ┌─────────────────────────────────┐ │
│ fetchSchemaData()   │            │ │ MATCH (n)... [click to edit]    │ │
└─────────────────────┘            └─────────────────────────────────────┘
```

---

## Architecture Overview

### Data Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ ViewScroll  │────►│  viewStore  │────►│ queryStore  │────►│ graphStore  │
│ CypherModal │     │ executeView │     │ executeQry  │     │ setGraphDat │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
                                                                   │
                                                                   ▼
                                                            ┌─────────────┐
                                                            │ GraphCanvas │
                                                            │ (re-render) │
                                                            └─────────────┘
```

### State Management

```typescript
// viewStore - SINGLE SOURCE OF TRUTH for navigation
interface ViewState {
  // Registry
  categories: ViewCategory[];
  views: Map<string, ViewDefinition>;

  // Current State
  activeViewId: string;           // 'complete-graph', 'block-generation', etc.
  isCustomQuery: boolean;         // true when custom Cypher executed
  customQueryText: string | null; // the custom query if any
  params: ViewParams;             // { key?, locale?, project? }

  // Loading
  isLoading: boolean;
  isExecuting: boolean;
  error: string | null;
}

// Actions
executeView(id: string, params?: ViewParams): Promise<void>
executeCustomQuery(cypher: string): Promise<void>
loadDefaultView(): Promise<void>
```

---

## Implementation Phases

### Phase 1: Remove Legacy (DELETE)

**Objective**: Clean slate - remove all META/DATA mode code.

#### 1.1 Delete from `uiStore.ts`

```typescript
// DELETE these lines:
export type NavigationMode = 'data' | 'meta';  // line 15

interface UIStoreState {
  navigationMode: NavigationMode;  // DELETE
  // ...
}

// DELETE selectors
export const selectNavigationMode = (state) => state.navigationMode;

// DELETE actions
setNavigationMode: (mode) => { ... }
cycleNavigationMode: () => { ... }

// DELETE from persist partialize
partialize: (state) => ({
  navigationMode: state.navigationMode,  // DELETE
  // ...
})
```

#### 1.2 Delete from `page.tsx`

```typescript
// DELETE imports
import { selectNavigationMode } from '@/stores/uiStore';

// DELETE state
const navigationMode = useUIStore(selectNavigationMode);
const setNavigationMode = useUIStore((state) => state.setNavigationMode);
const prevNavigationModeRef = useRef<typeof navigationMode | null>(null);

// DELETE useEffect (lines ~365-386)
useEffect(() => {
  if (modeChanged) {
    if (navigationMode === 'meta') {
      fetchSchemaData();
    } else {
      clearGraph();
    }
  }
}, [...]);

// DELETE keyboard handler for 'N' key (line ~501)
if (e.key === 'n') {
  const modes: typeof navigationMode[] = ['meta', 'data'];
  // ...
}

// DELETE Matrix transition mode logic
transitionActions.startTransition(nextMode);
```

#### 1.3 Delete from `useGraphData.ts`

```typescript
// DELETE
import { selectNavigationMode, type NavigationMode } from '@/stores/uiStore';
const navigationMode = useUIStore(selectNavigationMode);

// DELETE mode-dependent logic
const isMetaMode = navigationMode === 'meta';
```

#### 1.4 Delete from `useUrlSync.tsx`

```typescript
// DELETE all NavigationMode references
import { type NavigationMode } from '@/stores/uiStore';
const setNavigationMode = useUIStore((state) => state.setNavigationMode);
// DELETE URL sync for mode
```

#### 1.5 Delete from `useFilteredGraph.ts`

```typescript
// DELETE
const isMetaMode = navigationMode === 'meta';

// DELETE mode-dependent filter bypass
if (isMetaMode) return unhiddenNodes;  // bypass filters in schema mode
```

#### 1.6 Delete Components

```bash
# Delete if exists
rm apps/studio/src/components/navigation/ModeToggle.tsx
rm apps/studio/src/components/navigation/ModeIndicator.tsx
```

#### 1.7 Clean YAML (optional)

```yaml
# In views/_registry.yaml - remove modes field entirely
# BEFORE:
- id: complete-graph
  modes: [data, meta, overlay, query]  # DELETE this line

# AFTER:
- id: complete-graph
  # no modes - all views available always
```

---

### Phase 2: Enhance viewStore (MODIFY)

**Objective**: Make viewStore the primary navigation controller.

#### 2.1 New viewStore Interface

```typescript
// apps/studio/src/stores/viewStore.ts

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import { useQueryStore } from './queryStore';
import { useGraphStore } from './graphStore';

// Types
export interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
}

export interface ViewDefinition {
  id: string;
  name: string;
  description: string;
  category: string;
  shortcut?: number;  // 1-9 for quick access
}

export interface ViewCategory {
  id: string;
  name: string;
  views: ViewDefinition[];
}

interface ViewState {
  // Registry (loaded once at startup)
  categories: ViewCategory[];
  isRegistryLoaded: boolean;

  // Current navigation state
  activeViewId: string;
  isCustomQuery: boolean;
  customQueryText: string | null;
  params: ViewParams;

  // Loading states
  isLoading: boolean;
  isExecuting: boolean;
  error: string | null;
}

interface ViewActions {
  // Registry
  loadRegistry(): Promise<void>;

  // Navigation
  executeView(id: string, params?: ViewParams): Promise<void>;
  executeCustomQuery(cypher: string): Promise<void>;
  loadDefaultView(): Promise<void>;

  // Params
  setParams(params: ViewParams): void;

  // Helpers
  getActiveView(): ViewDefinition | null;
  getViewById(id: string): ViewDefinition | null;
}

const DEFAULT_VIEW_ID = 'complete-graph';

export const useViewStore = create<ViewState & ViewActions>()(
  persist(
    immer((set, get) => ({
      // Initial state
      categories: [],
      isRegistryLoaded: false,
      activeViewId: DEFAULT_VIEW_ID,
      isCustomQuery: false,
      customQueryText: null,
      params: {},
      isLoading: false,
      isExecuting: false,
      error: null,

      // Load view registry from API
      loadRegistry: async () => {
        if (get().isRegistryLoaded) return;

        set({ isLoading: true, error: null });
        try {
          const res = await fetch('/api/views');
          const json = await res.json();

          if (!json.success) throw new Error(json.error);

          set({
            categories: json.data.categories,
            isRegistryLoaded: true,
            isLoading: false,
          });
        } catch (err) {
          set({
            error: err instanceof Error ? err.message : 'Failed to load views',
            isLoading: false,
          });
        }
      },

      // Execute a view by ID
      executeView: async (id, params) => {
        set({
          activeViewId: id,
          isCustomQuery: false,
          customQueryText: null,
          isExecuting: true,
          error: null,
        });

        if (params) set({ params });

        try {
          // Fetch Cypher for this view
          const viewParams = params || get().params;
          const queryString = new URLSearchParams(
            Object.entries(viewParams).filter(([_, v]) => v != null) as [string, string][]
          ).toString();

          const url = `/api/views/${id}${queryString ? `?${queryString}` : ''}`;
          const res = await fetch(url);
          const json = await res.json();

          if (!json.success) throw new Error(json.error);

          // Execute the Cypher query
          const { query, params: cypherParams } = json.data.cypher;
          await useQueryStore.getState().executeQuery(query, cypherParams);

          set({ isExecuting: false });
        } catch (err) {
          set({
            error: err instanceof Error ? err.message : 'Failed to execute view',
            isExecuting: false,
          });
        }
      },

      // Execute custom Cypher (overrides current view display)
      executeCustomQuery: async (cypher) => {
        set({
          isCustomQuery: true,
          customQueryText: cypher,
          isExecuting: true,
          error: null,
        });

        try {
          await useQueryStore.getState().executeQuery(cypher);
          set({ isExecuting: false });
        } catch (err) {
          set({
            error: err instanceof Error ? err.message : 'Query failed',
            isExecuting: false,
          });
        }
      },

      // Load default view on startup
      loadDefaultView: async () => {
        const { activeViewId, executeView, isRegistryLoaded, loadRegistry } = get();

        // Ensure registry is loaded first
        if (!isRegistryLoaded) {
          await loadRegistry();
        }

        // Execute the active view (persisted or default)
        await executeView(activeViewId || DEFAULT_VIEW_ID);
      },

      // Update params
      setParams: (params) => {
        set({ params: { ...get().params, ...params } });
      },

      // Get active view definition
      getActiveView: () => {
        const { activeViewId, categories } = get();
        for (const cat of categories) {
          const view = cat.views.find(v => v.id === activeViewId);
          if (view) return view;
        }
        return null;
      },

      // Get view by ID
      getViewById: (id) => {
        const { categories } = get();
        for (const cat of categories) {
          const view = cat.views.find(v => v.id === id);
          if (view) return view;
        }
        return null;
      },
    })),
    {
      name: 'novanet-view-store',
      partialize: (state) => ({
        activeViewId: state.activeViewId,
        params: state.params,
        // NOT persisted: isCustomQuery, customQueryText (reset on reload)
      }),
    }
  )
);

// Selectors
export const selectActiveViewId = (state: ViewState) => state.activeViewId;
export const selectIsCustomQuery = (state: ViewState) => state.isCustomQuery;
export const selectIsExecuting = (state: ViewState) => state.isExecuting;
export const selectCategories = (state: ViewState) => state.categories;
```

---

### Phase 3: Update Startup Flow (MODIFY)

**Objective**: Load default view on app mount.

#### 3.1 Modify `page.tsx`

```typescript
// apps/studio/src/app/page.tsx

import { useViewStore } from '@/stores/viewStore';

export default function HomePage() {
  // View store
  const loadDefaultView = useViewStore((state) => state.loadDefaultView);
  const isRegistryLoaded = useViewStore((state) => state.isRegistryLoaded);

  // Startup: load default view
  useEffect(() => {
    loadDefaultView();
  }, [loadDefaultView]);

  // Show loading until view is loaded
  if (!isRegistryLoaded) {
    return <LoadingScreen />;
  }

  return (
    <main>
      <ViewScrollBar />
      <CypherBar />
      <GraphCanvas />
      {/* ... */}
    </main>
  );
}
```

---

### Phase 4: Create UI Components (CREATE)

#### 4.1 ViewScrollBar Component

```typescript
// apps/studio/src/components/views/ViewScrollBar.tsx

'use client';

import { useRef } from 'react';
import { ChevronLeft, ChevronRight, Zap } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useViewStore } from '@/stores/viewStore';

export function ViewScrollBar() {
  const scrollRef = useRef<HTMLDivElement>(null);

  const categories = useViewStore((state) => state.categories);
  const activeViewId = useViewStore((state) => state.activeViewId);
  const isCustomQuery = useViewStore((state) => state.isCustomQuery);
  const isExecuting = useViewStore((state) => state.isExecuting);
  const executeView = useViewStore((state) => state.executeView);

  // Flatten views for scroll bar
  const allViews = categories.flatMap(cat => cat.views);

  const scroll = (direction: 'left' | 'right') => {
    if (!scrollRef.current) return;
    const amount = 200;
    scrollRef.current.scrollBy({
      left: direction === 'left' ? -amount : amount,
      behavior: 'smooth',
    });
  };

  return (
    <div className="flex items-center gap-2 px-4 py-2 bg-zinc-900/80 border-b border-zinc-800">
      {/* Scroll Left */}
      <button
        onClick={() => scroll('left')}
        className="p-1 hover:bg-zinc-800 rounded"
      >
        <ChevronLeft className="w-4 h-4" />
      </button>

      {/* Scrollable Views */}
      <div
        ref={scrollRef}
        className="flex-1 flex items-center gap-2 overflow-x-auto scrollbar-hide"
      >
        {/* Custom Query indicator */}
        {isCustomQuery && (
          <button
            className={cn(
              'flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-medium',
              'bg-amber-500/20 text-amber-400 border border-amber-500/30'
            )}
          >
            <Zap className="w-3.5 h-3.5" />
            Custom Query
          </button>
        )}

        {/* View Pills */}
        {allViews.map((view) => (
          <button
            key={view.id}
            onClick={() => executeView(view.id)}
            disabled={isExecuting}
            className={cn(
              'px-3 py-1.5 rounded-full text-sm font-medium whitespace-nowrap',
              'transition-all duration-200',
              activeViewId === view.id && !isCustomQuery
                ? 'bg-violet-500/20 text-violet-300 border border-violet-500/30'
                : 'text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800'
            )}
          >
            {view.name}
          </button>
        ))}
      </div>

      {/* Scroll Right */}
      <button
        onClick={() => scroll('right')}
        className="p-1 hover:bg-zinc-800 rounded"
      >
        <ChevronRight className="w-4 h-4" />
      </button>
    </div>
  );
}
```

#### 4.2 CypherModal Component

```typescript
// apps/studio/src/components/query/CypherModal.tsx

'use client';

import { useState, useCallback, useEffect } from 'react';
import { X, Play, ChevronLeft, ChevronRight } from 'lucide-react';
import { useViewStore } from '@/stores/viewStore';
import { useQueryStore } from '@/stores/queryStore';
import { CodeEditor } from '@/components/ui/CodeEditor';
import { cn } from '@/lib/utils';

interface CypherModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function CypherModal({ isOpen, onClose }: CypherModalProps) {
  const categories = useViewStore((state) => state.categories);
  const executeView = useViewStore((state) => state.executeView);
  const executeCustomQuery = useViewStore((state) => state.executeCustomQuery);
  const isExecuting = useViewStore((state) => state.isExecuting);

  const currentQuery = useQueryStore((state) => state.currentQuery);

  const [editValue, setEditValue] = useState(currentQuery || '');
  const scrollRef = useRef<HTMLDivElement>(null);

  // Sync with current query when opening
  useEffect(() => {
    if (isOpen) {
      setEditValue(currentQuery || '');
    }
  }, [isOpen, currentQuery]);

  // Flatten views
  const allViews = categories.flatMap(cat => cat.views);

  // Insert view query into editor
  const handleViewClick = async (viewId: string) => {
    try {
      const res = await fetch(`/api/views/${viewId}`);
      const json = await res.json();
      if (json.success) {
        setEditValue(json.data.cypher.query);
      }
    } catch (err) {
      console.error('Failed to load view query:', err);
    }
  };

  // Execute and close
  const handleRun = async () => {
    if (!editValue.trim()) return;
    await executeCustomQuery(editValue.trim());
    onClose();
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      if (e.key === 'Escape') {
        onClose();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        e.preventDefault();
        handleRun();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, editValue]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/80 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="relative w-full max-w-4xl mx-4 bg-zinc-900 rounded-lg border border-zinc-700 shadow-2xl">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-zinc-800">
          <h2 className="text-lg font-semibold text-zinc-100">Cypher Editor</h2>
          <button onClick={onClose} className="p-1 hover:bg-zinc-800 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* View Shortcuts */}
        <div className="flex items-center gap-2 px-4 py-2 border-b border-zinc-800 bg-zinc-900/50">
          <span className="text-xs text-zinc-500">Views:</span>
          <div
            ref={scrollRef}
            className="flex-1 flex items-center gap-1.5 overflow-x-auto scrollbar-hide"
          >
            {allViews.slice(0, 10).map((view) => (
              <button
                key={view.id}
                onClick={() => handleViewClick(view.id)}
                className="px-2 py-1 text-xs rounded bg-zinc-800 hover:bg-zinc-700 text-zinc-300 whitespace-nowrap"
              >
                {view.name}
              </button>
            ))}
          </div>
        </div>

        {/* Editor */}
        <div className="p-4">
          <CodeEditor
            value={editValue}
            onChange={setEditValue}
            language="cypher"
            placeholder="MATCH (n) RETURN n LIMIT 100"
            minHeight={300}
            className="border border-zinc-700 rounded-lg"
          />
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between px-4 py-3 border-t border-zinc-800">
          <div className="text-xs text-zinc-500">
            <kbd className="px-1.5 py-0.5 bg-zinc-800 rounded">⌘</kbd>
            <span className="mx-1">+</span>
            <kbd className="px-1.5 py-0.5 bg-zinc-800 rounded">Enter</kbd>
            <span className="ml-2">to run</span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              className="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-200"
            >
              Cancel
            </button>
            <button
              onClick={handleRun}
              disabled={isExecuting || !editValue.trim()}
              className={cn(
                'flex items-center gap-2 px-4 py-2 text-sm font-medium rounded-lg',
                'bg-violet-600 hover:bg-violet-500 text-white',
                'disabled:opacity-50 disabled:cursor-not-allowed'
              )}
            >
              <Play className="w-4 h-4" />
              Run Query
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
```

#### 4.3 Update CypherBar (trigger for modal)

```typescript
// apps/studio/src/components/query/CypherBar.tsx

'use client';

import { useState } from 'react';
import { Terminal, Maximize2 } from 'lucide-react';
import { useQueryStore } from '@/stores/queryStore';
import { useViewStore } from '@/stores/viewStore';
import { CypherModal } from './CypherModal';
import { cn } from '@/lib/utils';

export function CypherBar() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const currentQuery = useQueryStore((state) => state.currentQuery);
  const isExecuting = useViewStore((state) => state.isExecuting);

  // Keyboard shortcut to open modal
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsModalOpen(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return (
    <>
      <div
        onClick={() => setIsModalOpen(true)}
        className={cn(
          'flex items-center gap-2 px-4 py-2 mx-4 my-2',
          'bg-zinc-900/80 border border-zinc-800 rounded-lg',
          'cursor-pointer hover:border-zinc-700 transition-colors'
        )}
      >
        <Terminal className="w-4 h-4 text-emerald-500" />
        <span className="text-sm text-zinc-500">neo4j$</span>
        <span className="flex-1 text-sm text-zinc-300 truncate">
          {currentQuery || 'Click to edit query...'}
        </span>
        <div className="flex items-center gap-2 text-zinc-500">
          <kbd className="px-1.5 py-0.5 text-xs bg-zinc-800 rounded">⌘K</kbd>
          <Maximize2 className="w-4 h-4" />
        </div>
      </div>

      <CypherModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
      />
    </>
  );
}
```

---

### Phase 5: Cleanup & Verification

#### 5.1 Files to Delete

```bash
# Components that may exist for mode toggle
rm -f apps/studio/src/components/navigation/ModeToggle.tsx
rm -f apps/studio/src/components/navigation/ModeIndicator.tsx
rm -f apps/studio/src/components/navigation/NavigationModeSwitch.tsx
```

#### 5.2 Type Cleanup

```typescript
// Delete from types if exists
// types/index.ts or similar
export type NavigationMode = 'data' | 'meta';  // DELETE
```

#### 5.3 Test Updates

```bash
# Update any tests referencing navigationMode
grep -r "navigationMode" apps/studio/src/**/*.test.ts
# Update or delete those tests
```

---

## File Change Summary

| File | Action | Description |
|------|--------|-------------|
| `stores/uiStore.ts` | MODIFY | Remove NavigationMode, keep other UI state |
| `stores/viewStore.ts` | REWRITE | New view-based navigation system |
| `stores/queryStore.ts` | KEEP | Already works correctly |
| `stores/graphStore.ts` | KEEP | Already works correctly |
| `app/page.tsx` | MODIFY | Remove mode logic, add loadDefaultView() |
| `hooks/useGraphData.ts` | MODIFY | Remove mode dependencies |
| `hooks/useFilteredGraph.ts` | MODIFY | Remove mode-based filter bypass |
| `hooks/useUrlSync.tsx` | MODIFY | Sync view instead of mode |
| `components/views/ViewScrollBar.tsx` | CREATE | Horizontal scrollable views |
| `components/query/CypherBar.tsx` | MODIFY | Trigger for modal |
| `components/query/CypherModal.tsx` | CREATE | Full Cypher editor |

---

## Testing Checklist

- [ ] App loads with `complete-graph` view
- [ ] Clicking a view in scroll bar updates graph
- [ ] Custom query shows "Custom Query" indicator
- [ ] Clicking view after custom query replaces it
- [ ] `⌘K` opens Cypher modal
- [ ] `⌘Enter` in modal runs query
- [ ] View persists in localStorage across reloads
- [ ] URL reflects current view (`?view=block-generation`)
- [ ] No console errors about navigationMode

---

## Migration Notes

**Breaking Changes**:
- `navigationMode` state removed entirely
- `'N'` key no longer toggles modes
- META/DATA UI indicators removed
- URL param `?mode=meta` no longer works (use `?view=complete-graph`)

**No Backward Compatibility**:
- This is a clean v12 release
- Users must adapt to view-based navigation
- Old bookmarks with `?mode=` will be ignored

---

## Timeline

| Phase | Duration | Tasks |
|-------|----------|-------|
| Phase 1 | 1h | Delete legacy mode code |
| Phase 2 | 2h | Rewrite viewStore |
| Phase 3 | 30min | Update startup flow |
| Phase 4 | 2h | Create UI components |
| Phase 5 | 30min | Cleanup & verification |
| **Total** | **6h** | |

---

## Approval

- [x] Architecture validated with user
- [x] No backward compatibility required
- [x] Clean slate approach confirmed
- [ ] Ready for implementation
