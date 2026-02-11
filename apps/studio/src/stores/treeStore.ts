import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type {
  UnifiedNode,
  UnifiedTreeState,
  UnifiedTreeActions,
  LazyChildrenState,
} from '@novanet/core';

/**
 * Unified Tree Store for v11.7
 *
 * Manages the hierarchical tree state where Realm, Layer, Kind, Instance,
 * ArcFamily, and ArcKind are all clickable nodes with detail panels.
 *
 * Principle: "If it's a node in Neo4j, it's a node everywhere"
 */

interface TreeStore extends UnifiedTreeState, UnifiedTreeActions {}

/** Default initial state for lazy children */
const NOT_LOADED: LazyChildrenState = { status: 'not_loaded' };

export const useTreeStore = create<TreeStore>()(
  immer((set, get) => ({
    // ==========================================================================
    // State
    // ==========================================================================

    /** All nodes by ID */
    nodes: new Map<string, UnifiedNode>(),

    /** Root node IDs in display order */
    rootOrder: [],

    /** Set of expanded node IDs */
    expanded: new Set<string>(),

    /** Set of currently loading node IDs */
    loading: new Set<string>(),

    /** Children state per node */
    children: new Map<string, LazyChildrenState>(),

    /** Currently selected node ID */
    selectedId: null,

    /** Currently focused node ID (keyboard navigation) */
    focusedId: null,

    // ==========================================================================
    // Actions
    // ==========================================================================

    toggleExpand: (nodeId: string) => {
      const state = get();
      const node = state.nodes.get(nodeId);

      // Cannot expand non-expandable nodes
      if (!node?.expandable) return;

      set((draft) => {
        if (draft.expanded.has(nodeId)) {
          draft.expanded.delete(nodeId);
        } else {
          draft.expanded.add(nodeId);

          // Trigger lazy loading if children not yet loaded
          const childrenState = draft.children.get(nodeId);
          if (!childrenState || childrenState.status === 'not_loaded') {
            // Mark as loading - actual load happens outside immer
            draft.loading.add(nodeId);
          }
        }
      });

      // If we just expanded and children aren't loaded, trigger load
      const childrenState = get().children.get(nodeId);
      if (
        get().expanded.has(nodeId) &&
        (!childrenState || childrenState.status === 'not_loaded')
      ) {
        get().loadChildren(nodeId);
      }
    },

    selectNode: (nodeId: string) => {
      set((draft) => {
        draft.selectedId = nodeId;
        draft.focusedId = nodeId;
      });
    },

    loadChildren: async (nodeId: string) => {
      const state = get();

      // Already loading or loaded
      const childrenState = state.children.get(nodeId);
      if (
        childrenState?.status === 'loading' ||
        childrenState?.status === 'loaded' ||
        childrenState?.status === 'leaf'
      ) {
        return;
      }

      // Mark as loading
      set((draft) => {
        draft.loading.add(nodeId);
        draft.children.set(nodeId, { status: 'loading' });
      });

      try {
        const response = await fetch(`/api/tree/${encodeURIComponent(nodeId)}/children`);

        if (!response.ok) {
          throw new Error(`Failed to load children: ${response.statusText}`);
        }

        const data = await response.json();

        set((draft) => {
          // Remove from loading
          draft.loading.delete(nodeId);

          // Handle empty response (leaf node)
          if (!data.children || data.children.length === 0) {
            draft.children.set(nodeId, { status: 'leaf' });
            return;
          }

          // Add child nodes to the nodes map
          const childIds: string[] = [];
          for (const child of data.children as UnifiedNode[]) {
            draft.nodes.set(child.id, child);
            childIds.push(child.id);

            // Initialize children state for each child
            if (!draft.children.has(child.id)) {
              draft.children.set(child.id, child.expandable ? NOT_LOADED : { status: 'leaf' });
            }
          }

          // Update children state with loaded items
          draft.children.set(nodeId, {
            status: 'loaded',
            items: childIds,
            total: data.total ?? childIds.length,
            hasMore: data.hasMore ?? false,
          });
        });
      } catch (error) {
        console.error(`[treeStore] Failed to load children for ${nodeId}:`, error);

        // Reset to not_loaded on error
        set((draft) => {
          draft.loading.delete(nodeId);
          draft.children.set(nodeId, NOT_LOADED);
        });
      }
    },

    loadMoreChildren: async (nodeId: string) => {
      const state = get();
      const childrenState = state.children.get(nodeId);

      // Can only load more if already loaded and has more
      if (childrenState?.status !== 'loaded' || !childrenState.hasMore) {
        return;
      }

      // Already loading
      if (state.loading.has(nodeId)) {
        return;
      }

      // Mark as loading
      set((draft) => {
        draft.loading.add(nodeId);
      });

      try {
        const offset = childrenState.items.length;
        const response = await fetch(
          `/api/tree/${encodeURIComponent(nodeId)}/children?offset=${offset}`
        );

        if (!response.ok) {
          throw new Error(`Failed to load more children: ${response.statusText}`);
        }

        const data = await response.json();

        set((draft) => {
          draft.loading.delete(nodeId);

          const currentState = draft.children.get(nodeId);
          if (currentState?.status !== 'loaded') return;

          // Add new child nodes
          const newChildIds: string[] = [];
          for (const child of data.children as UnifiedNode[]) {
            draft.nodes.set(child.id, child);
            newChildIds.push(child.id);

            if (!draft.children.has(child.id)) {
              draft.children.set(child.id, child.expandable ? NOT_LOADED : { status: 'leaf' });
            }
          }

          // Update children state with appended items
          draft.children.set(nodeId, {
            status: 'loaded',
            items: [...currentState.items, ...newChildIds],
            total: data.total ?? currentState.total,
            hasMore: data.hasMore ?? false,
          });
        });
      } catch (error) {
        console.error(`[treeStore] Failed to load more children for ${nodeId}:`, error);
        set((draft) => {
          draft.loading.delete(nodeId);
        });
      }
    },

    refreshTree: async () => {
      // Clear current state
      set((draft) => {
        draft.nodes.clear();
        draft.rootOrder = [];
        draft.expanded.clear();
        draft.loading.clear();
        draft.children.clear();
        draft.selectedId = null;
        draft.focusedId = null;
      });

      try {
        const response = await fetch('/api/tree/root');

        if (!response.ok) {
          throw new Error(`Failed to load tree root: ${response.statusText}`);
        }

        const data = await response.json();

        set((draft) => {
          // Add root nodes
          const rootIds: string[] = [];
          for (const node of data.root as UnifiedNode[]) {
            draft.nodes.set(node.id, node);
            rootIds.push(node.id);

            // Initialize children state
            draft.children.set(node.id, node.expandable ? NOT_LOADED : { status: 'leaf' });
          }

          draft.rootOrder = rootIds;
        });
      } catch (error) {
        console.error('[treeStore] Failed to refresh tree:', error);
      }
    },

    reset: () => {
      set((draft) => {
        draft.nodes.clear();
        draft.rootOrder = [];
        draft.expanded.clear();
        draft.loading.clear();
        draft.children.clear();
        draft.selectedId = null;
        draft.focusedId = null;
      });
    },
  }))
);
