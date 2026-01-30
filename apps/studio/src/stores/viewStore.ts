// stores/viewStore.ts
// YAML Views state management for Studio
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { ViewCategoryGroup, ViewRegistryEntry } from '@novanet/core/filters';
import { logger } from '@/lib/logger';
import { useQueryStore } from './queryStore';

// ============================================================================
// TYPES
// ============================================================================

export interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
}

interface ViewStoreState {
  // Data
  categories: ViewCategoryGroup[];
  activeViewId: string | null;
  params: ViewParams;
  loading: boolean;
  executing: boolean;
  error: string | null;

  // Actions
  loadRegistry: () => Promise<void>;
  selectView: (id: string, params?: ViewParams) => void;
  executeView: (id: string, params?: ViewParams) => Promise<void>;
  setParams: (params: Partial<ViewParams>) => void;
  clearView: () => void;

  // URL sync helpers
  syncFromURL: (searchParams: URLSearchParams) => void;
  toURLParams: () => URLSearchParams;

  // Getters
  getViewById: (id: string) => ViewRegistryEntry | undefined;
  getActiveView: () => ViewRegistryEntry | undefined;
}

// ============================================================================
// STORE
// ============================================================================

export const useViewStore = create<ViewStoreState>()(
  persist(
    immer((set, get) => ({
      // Initial state
      categories: [],
      activeViewId: null,
      params: {},
      loading: false,
      executing: false,
      error: null,

      // Load registry from API
      loadRegistry: async () => {
        // Don't reload if already loaded
        if (get().categories.length > 0) {
          logger.debug('ViewStore', 'Registry already loaded, skipping');
          return;
        }

        set({ loading: true, error: null });
        logger.debug('ViewStore', 'Loading registry...');

        try {
          const res = await fetch('/api/views');
          const json = await res.json();

          if (json.success) {
            set((state) => {
              state.categories = json.data.categories;
              state.loading = false;
            });
            logger.info('ViewStore', 'Registry loaded', {
              categoryCount: json.data.categories.length,
              viewCount: json.data.registry.views.length,
            });
          } else {
            set({ error: json.error, loading: false });
            logger.error('ViewStore', 'Failed to load registry', { error: json.error });
          }
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Unknown error';
          set({ error: message, loading: false });
          logger.error('ViewStore', 'Registry load error', { error: message });
        }
      },

      // Select a view (just sets state, doesn't execute)
      selectView: (id, params) => {
        set((state) => {
          state.activeViewId = id;
          if (params) {
            state.params = params;
          }
        });
        logger.info('ViewStore', 'View selected', { id, params });
      },

      // Select and execute a view (fetches Cypher and runs query)
      executeView: async (id, params) => {
        // First select the view
        set((state) => {
          state.activeViewId = id;
          state.executing = true;
          state.error = null;
          if (params) {
            state.params = params;
          }
        });

        logger.info('ViewStore', 'Executing view', { id, params });

        try {
          // Build query params for the API
          const queryParams = new URLSearchParams();
          const viewParams = params || get().params;
          if (viewParams.key) queryParams.set('key', viewParams.key);
          if (viewParams.locale) queryParams.set('locale', viewParams.locale);
          if (viewParams.project) queryParams.set('project', viewParams.project);

          const url = `/api/views/${id}${queryParams.toString() ? `?${queryParams.toString()}` : ''}`;
          const res = await fetch(url);
          const json = await res.json();

          if (!json.success) {
            throw new Error(json.error || 'Failed to load view');
          }

          // Extract the Cypher query from the view
          const cypher = json.data.cypher?.query;
          if (!cypher) {
            throw new Error('View did not return a Cypher query');
          }

          logger.debug('ViewStore', 'View Cypher loaded', { id, cypher: cypher.substring(0, 100) + '...' });

          // Execute the query via queryStore
          await useQueryStore.getState().executeQuery(cypher);

          set({ executing: false });
          logger.info('ViewStore', 'View executed successfully', { id });
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Unknown error';
          set({ error: message, executing: false });
          logger.error('ViewStore', 'View execution failed', { id, error: message });
        }
      },

      // Update params
      setParams: (params) => {
        set((state) => {
          state.params = { ...state.params, ...params };
        });
        logger.debug('ViewStore', 'Params updated', { params });
      },

      // Clear active view
      clearView: () => {
        set({ activeViewId: null, params: {} });
        logger.info('ViewStore', 'View cleared');
      },

      // Sync from URL search params
      syncFromURL: (searchParams) => {
        const view = searchParams.get('view');
        if (view) {
          set({
            activeViewId: view,
            params: {
              key: searchParams.get('key') || undefined,
              locale: searchParams.get('locale') || undefined,
              project: searchParams.get('project') || undefined,
            },
          });
          logger.debug('ViewStore', 'Synced from URL', { view });
        }
      },

      // Convert state to URL params
      toURLParams: () => {
        const { activeViewId, params } = get();
        const urlParams = new URLSearchParams();

        if (activeViewId) {
          urlParams.set('view', activeViewId);
          if (params.key) urlParams.set('key', params.key);
          if (params.locale) urlParams.set('locale', params.locale);
          if (params.project) urlParams.set('project', params.project);
        }

        return urlParams;
      },

      // Get view by ID
      getViewById: (id) => {
        const { categories } = get();
        for (const category of categories) {
          const view = category.views.find((v) => v.id === id);
          if (view) return view;
        }
        return undefined;
      },

      // Get active view
      getActiveView: () => {
        const { activeViewId, getViewById } = get();
        return activeViewId ? getViewById(activeViewId) : undefined;
      },
    })),
    {
      name: 'novanet-view-store',
      // Only persist view selection, not categories (they're fetched)
      partialize: (state) => ({
        activeViewId: state.activeViewId,
        params: state.params,
      }),
    }
  )
);
