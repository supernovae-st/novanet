---
name: zustand-patterns
description: State management with Zustand 5, persist, and immer middlewares. Use when creating stores, implementing selectors, or handling Set serialization for filters.
user-invocable: false
---

# Zustand Patterns

> NovaNet state management with Zustand 5

## Use When

- Creating new stores (graph, filter, ui, chat)
- Implementing persist middleware for user preferences
- Using immer for immutable updates
- Writing efficient selectors to prevent re-renders
- Handling Set serialization for localStorage

## Quick Reference

| Store | Persist | Purpose |
|-------|---------|---------|
| `graphStore` | No | Nodes, edges, loading |
| `filterStore` | Yes | Node types, locale, presets |
| `uiStore` | Partial | View mode, panels, selection |
| `chatStore` | No | AI messages, streaming |
| `queryStore` | No | Cypher query state |

---

## Store with Persist + Immer

```typescript
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';

interface FilterState {
  enabledNodeTypes: Set<string>;
  selectedLocale: string | null;
  searchQuery: string;
  activePresetId: string | null;

  setEnabledNodeTypes: (types: Set<string>) => void;
  toggleNodeType: (type: string) => void;
  setSelectedLocale: (locale: string | null) => void;
  setSearchQuery: (query: string) => void;
  applyPreset: (preset: FilterPreset) => void;
  clearFilters: () => void;
}

export const useFilterStore = create<FilterState>()(
  persist(
    immer((set) => ({
      enabledNodeTypes: new Set(ALL_NODE_TYPES),
      selectedLocale: null,
      searchQuery: '',
      activePresetId: null,

      setEnabledNodeTypes: (types) => set((state) => {
        state.enabledNodeTypes = types;
        state.activePresetId = null;
      }),

      toggleNodeType: (type) => set((state) => {
        if (state.enabledNodeTypes.has(type)) {
          state.enabledNodeTypes.delete(type);
        } else {
          state.enabledNodeTypes.add(type);
        }
        state.activePresetId = null;
      }),

      setSelectedLocale: (locale) => set((state) => {
        state.selectedLocale = locale;
      }),

      setSearchQuery: (query) => set((state) => {
        state.searchQuery = query;
      }),

      applyPreset: (preset) => set((state) => {
        state.enabledNodeTypes = new Set(preset.nodeTypes);
        state.selectedLocale = preset.locale ?? null;
        state.activePresetId = preset.id;
      }),

      clearFilters: () => set((state) => {
        state.enabledNodeTypes = new Set(ALL_NODE_TYPES);
        state.selectedLocale = null;
        state.searchQuery = '';
        state.activePresetId = null;
      }),
    })),
    {
      name: 'novanet-filter-storage',
      // Handle Set serialization
      storage: {
        getItem: (name) => {
          const str = localStorage.getItem(name);
          if (!str) return null;
          const parsed = JSON.parse(str);
          return {
            ...parsed,
            state: {
              ...parsed.state,
              enabledNodeTypes: new Set(parsed.state.enabledNodeTypes),
            },
          };
        },
        setItem: (name, value) => {
          const toStore = {
            ...value,
            state: {
              ...value.state,
              enabledNodeTypes: Array.from(value.state.enabledNodeTypes),
            },
          };
          localStorage.setItem(name, JSON.stringify(toStore));
        },
        removeItem: (name) => localStorage.removeItem(name),
      },
    }
  )
);
```

## Computed Values (Selectors)

```typescript
// In the store
const useGraphStore = create<GraphState>()((set, get) => ({
  nodes: [],
  edges: [],

  // Computed getters
  getNodeById: (id: string) => {
    return get().nodes.find((n) => n.id === id);
  },

  getConnectedNodes: (nodeId: string) => {
    const edges = get().edges;
    const connectedIds = new Set<string>();

    edges.forEach((edge) => {
      if (edge.source === nodeId) connectedIds.add(edge.target);
      if (edge.target === nodeId) connectedIds.add(edge.source);
    });

    return get().nodes.filter((n) => connectedIds.has(n.id));
  },
}));

// Usage with selector for performance
const node = useGraphStore((state) => state.getNodeById(selectedId));
const connected = useGraphStore((state) => state.getConnectedNodes(selectedId));
```

## Subscribing to Changes

```typescript
// Subscribe outside React
const unsubscribe = useFilterStore.subscribe(
  (state) => state.enabledNodeTypes,
  (enabledTypes) => {
    console.log('Filter changed:', enabledTypes);
  }
);

// In useEffect
useEffect(() => {
  const unsub = useGraphStore.subscribe(
    (state) => state.isLoading,
    (isLoading) => {
      if (!isLoading) {
        // Graph finished loading
        fitView();
      }
    }
  );
  return unsub;
}, []);
```

## Combining Stores

```typescript
// Hook that combines multiple stores
function useGraphWithFilters() {
  const nodes = useGraphStore((s) => s.nodes);
  const edges = useGraphStore((s) => s.edges);
  const enabledTypes = useFilterStore((s) => s.enabledNodeTypes);
  const locale = useFilterStore((s) => s.selectedLocale);

  return useMemo(() => {
    const filteredNodes = nodes.filter((n) => {
      if (!enabledTypes.has(n.type)) return false;
      if (locale && n.data.locale && n.data.locale !== locale) return false;
      return true;
    });

    const nodeIds = new Set(filteredNodes.map((n) => n.id));
    const filteredEdges = edges.filter(
      (e) => nodeIds.has(e.source) && nodeIds.has(e.target)
    );

    return { nodes: filteredNodes, edges: filteredEdges };
  }, [nodes, edges, enabledTypes, locale]);
}
```
