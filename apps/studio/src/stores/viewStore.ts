// stores/viewStore.ts
// v12: View-based navigation - views are the single source of truth
// v12.1: Query-First filter injection

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { ViewCategoryGroup, ViewRegistryEntry } from '@novanet/core/filters';
import { logger } from '@/lib/logger';
import { useQueryStore } from './queryStore';
import { useFilterStore } from './filterStore';
import { injectFilters } from '@/lib/cypher/injectFilters';

// ============================================================================
// TYPES
// ============================================================================

export interface ViewParams {
  key?: string;
  locale?: string;
  project?: string;
}

interface ViewStoreState {
  // Registry (loaded once at startup)
  categories: ViewCategoryGroup[];
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

  // Internal: AbortController for canceling in-flight requests (not persisted)
  _abortController: AbortController | null;
}

interface ViewStoreActions {
  // Registry
  loadRegistry: () => Promise<void>;

  // Navigation
  selectView: (id: string, params?: ViewParams) => void;
  executeView: (id: string, params?: ViewParams) => Promise<void>;
  executeCustomQuery: (cypher: string) => Promise<void>;
  loadDefaultView: () => Promise<void>;

  // Params
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
// CONSTANTS
// ============================================================================

const DEFAULT_VIEW_ID = 'data-complete';

// ============================================================================
// STORE
// ============================================================================

export const useViewStore = create<ViewStoreState & ViewStoreActions>()(
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
      _abortController: null,

      // Load registry from API
      loadRegistry: async () => {
        // Don't reload if already loaded
        if (get().isRegistryLoaded) {
          logger.debug('ViewStore', 'Registry already loaded, skipping');
          return;
        }

        set({ isLoading: true, error: null });
        logger.debug('ViewStore', 'Loading registry...');

        try {
          const res = await fetch('/api/views');

          // v11.6.1: Check HTTP status before parsing JSON
          if (!res.ok) {
            throw new Error(`HTTP ${res.status}: ${res.statusText}`);
          }

          const json = await res.json();

          if (json.success) {
            set((state) => {
              state.categories = json.data.categories;
              state.isRegistryLoaded = true;
              state.isLoading = false;
            });
            logger.info('ViewStore', 'Registry loaded', {
              categoryCount: json.data.categories.length,
              viewCount: json.data.registry.views.length,
            });
          } else {
            set({ error: json.error, isLoading: false });
            logger.error('ViewStore', 'Failed to load registry', { error: json.error });
          }
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Unknown error';
          set({ error: message, isLoading: false });
          logger.error('ViewStore', 'Registry load error', { error: message });
        }
      },

      // Select a view (just sets state, doesn't execute)
      selectView: (id, params) => {
        set((state) => {
          state.activeViewId = id;
          state.isCustomQuery = false;
          state.customQueryText = null;
          if (params) {
            state.params = params;
          }
        });
        logger.info('ViewStore', 'View selected', { id, params });
      },

      // Select and execute a view (fetches Cypher and runs query)
      executeView: async (id, params) => {
        // v11.6.1: Cancel any in-flight request to prevent race conditions
        const currentAbort = get()._abortController;
        if (currentAbort) {
          currentAbort.abort();
          logger.debug('ViewStore', 'Aborted previous request', { id });
        }

        // Create new AbortController for this request
        const abortController = new AbortController();

        // First select the view and clear custom query flag
        set((state) => {
          state.activeViewId = id;
          state.isCustomQuery = false;
          state.customQueryText = null;
          state.isExecuting = true;
          state.error = null;
          state._abortController = abortController;
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
          const res = await fetch(url, { signal: abortController.signal });

          // v11.6.1: Check HTTP status before parsing JSON
          if (!res.ok) {
            throw new Error(`HTTP ${res.status}: ${res.statusText}`);
          }

          const json = await res.json();

          if (!json.success) {
            throw new Error(json.error || 'Failed to load view');
          }

          // Extract the Cypher query and params from the view
          const baseCypherQuery = json.data.cypher?.query;
          const cypherParams = json.data.cypher?.params || {};
          if (!baseCypherQuery) {
            throw new Error('View did not return a Cypher query');
          }

          // v12.1: Query-First - inject filters into the Cypher query
          // This ensures what's displayed = what's executed
          const { displayLimit, selectedLocale } = useFilterStore.getState();
          const cypherQuery = injectFilters(baseCypherQuery, {
            displayLimit,
            localeKey: selectedLocale || undefined,
          });

          logger.debug('ViewStore', 'View Cypher loaded', {
            id,
            baseCypher: baseCypherQuery.substring(0, 100) + '...',
            finalCypher: cypherQuery.substring(0, 100) + '...',
            filters: { displayLimit, selectedLocale },
            params: cypherParams,
          });

          // DEBUG: Log full cypher before executing
          console.log('[viewStore] Executing Cypher:', {
            id,
            cypher: cypherQuery,
            params: cypherParams,
            filters: { displayLimit, selectedLocale },
          });

          // Execute the query via queryStore with params
          const result = await useQueryStore.getState().executeQuery(cypherQuery, cypherParams);

          // DEBUG: Log result from queryStore
          console.log('[viewStore] Query result:', {
            id,
            hasResult: !!result,
            nodeCount: result?.nodes?.length ?? 0,
            edgeCount: result?.edges?.length ?? 0,
          });

          set((state) => {
            state.isExecuting = false;
            state._abortController = null;
          });
          logger.info('ViewStore', 'View executed successfully', { id });
        } catch (error) {
          // v11.6.1: Ignore aborted requests (user clicked another view)
          if (error instanceof Error && error.name === 'AbortError') {
            logger.debug('ViewStore', 'Request aborted', { id });
            return;
          }

          // v11.6.10: Enhanced error logging for debugging
          const message = error instanceof Error ? error.message : String(error) || 'Unknown error';
          const errorDetails = error instanceof Error
            ? { name: error.name, message: error.message, stack: error.stack?.split('\n').slice(0, 3).join('\n') }
            : { raw: String(error) };
          set((state) => {
            state.error = message;
            state.isExecuting = false;
            state._abortController = null;
          });
          logger.error('ViewStore', `View execution failed: ${id}`, errorDetails);
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

        logger.info('ViewStore', 'Executing custom query', { cypher: cypher.substring(0, 100) + '...' });

        try {
          await useQueryStore.getState().executeQuery(cypher);
          set({ isExecuting: false });
          logger.info('ViewStore', 'Custom query executed successfully');
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Query failed';
          set({ error: message, isExecuting: false });
          logger.error('ViewStore', 'Custom query failed', { error: message });
        }
      },

      // Load default view on startup
      loadDefaultView: async () => {
        const { activeViewId, executeView, isRegistryLoaded, loadRegistry, getViewById } = get();

        // Ensure registry is loaded first
        if (!isRegistryLoaded) {
          await loadRegistry();
        }

        // Validate that persisted view still exists in registry, fallback to default
        const viewIdToLoad = activeViewId && getViewById(activeViewId)
          ? activeViewId
          : DEFAULT_VIEW_ID;

        // Execute the active view (persisted or default)
        await executeView(viewIdToLoad);
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
        set({
          activeViewId: DEFAULT_VIEW_ID,
          isCustomQuery: false,
          customQueryText: null,
          params: {},
        });
        logger.info('ViewStore', 'View cleared, reset to default');
      },

      // Sync from URL search params
      syncFromURL: (searchParams) => {
        const view = searchParams.get('view');
        if (view) {
          set({
            activeViewId: view,
            isCustomQuery: false,
            customQueryText: null,
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
        const { activeViewId, params, isCustomQuery } = get();
        const urlParams = new URLSearchParams();

        // Don't include view in URL if it's a custom query
        if (!isCustomQuery && activeViewId) {
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
        const { activeViewId, getViewById, isCustomQuery } = get();
        if (isCustomQuery) return undefined;
        return activeViewId ? getViewById(activeViewId) : undefined;
      },
    })),
    {
      name: 'novanet-view-store',
      version: 12, // v12: View-based navigation
      // Only persist view selection, not registry or custom query state
      partialize: (state) => ({
        activeViewId: state.activeViewId,
        params: state.params,
        // NOT persisted: isCustomQuery, customQueryText, categories (reset on reload)
      }),
    }
  )
);

// ============================================================================
// SELECTORS (for useShallow)
// ============================================================================

export const selectActiveViewId = (state: ViewStoreState) => state.activeViewId;
export const selectIsCustomQuery = (state: ViewStoreState) => state.isCustomQuery;
export const selectIsExecuting = (state: ViewStoreState) => state.isExecuting;
export const selectCategories = (state: ViewStoreState) => state.categories;
export const selectIsRegistryLoaded = (state: ViewStoreState) => state.isRegistryLoaded;
