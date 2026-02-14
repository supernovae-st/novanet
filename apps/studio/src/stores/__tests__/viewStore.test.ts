// src/stores/__tests__/viewStore.test.ts
// v12: Tests for view-based navigation store

import { useViewStore, selectActiveViewId, selectIsCustomQuery, selectIsExecuting, selectCategories, selectIsRegistryLoaded } from '../viewStore';
import { useQueryStore } from '../queryStore';
import type { ViewCategoryGroup } from '@novanet/core/filters';

// Mock fetch globally
const mockFetch = jest.fn();
global.fetch = mockFetch;

// Mock queryStore
jest.mock('../queryStore', () => ({
  useQueryStore: {
    getState: jest.fn(() => ({
      executeQuery: jest.fn().mockResolvedValue(undefined),
    })),
  },
}));

// Mock filterStore - v12.1: Query-First filter injection
jest.mock('../filterStore', () => ({
  useFilterStore: {
    getState: jest.fn(() => ({
      displayLimit: 250,
      selectedLocale: null,
    })),
  },
}));

// Mock logger to avoid console noise
jest.mock('@/lib/logger', () => ({
  logger: {
    debug: jest.fn(),
    info: jest.fn(),
    error: jest.fn(),
    warn: jest.fn(),
  },
}));

// Sample test data (v0.12.5 unified view system)
const mockCategories: ViewCategoryGroup[] = [
  {
    id: 'data',
    name: 'Data',
    views: [
      { id: 'data-complete', name: 'Complete Data', description: 'Complete data graph', cypher: 'MATCH (n) RETURN n', category: 'data' },
      { id: 'data-realm-shared', name: 'Shared Realm', description: 'Shared realm nodes', cypher: 'MATCH (n:Shared) RETURN n', category: 'data' },
    ],
  },
  {
    id: 'contextual',
    name: 'Contextual',
    views: [
      // v0.12.5: Updated to use new unified view ID
      { id: 'ctx-generation', name: 'Block Generation', description: 'Block generation view', cypher: 'MATCH (b:Block) RETURN b', category: 'contextual' },
    ],
  },
];

const mockRegistryResponse = {
  success: true,
  data: {
    categories: mockCategories,
    registry: {
      views: mockCategories.flatMap((c) => c.views),
    },
  },
};

describe('viewStore', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    // Reset store to initial state
    useViewStore.setState({
      categories: [],
      isRegistryLoaded: false,
      activeViewId: 'data-complete',
      isCustomQuery: false,
      customQueryText: null,
      params: {},
      isLoading: false,
      isExecuting: false,
      error: null,
      // v11.6.1: Added for AbortController support
      _abortController: null,
    });
  });

  // ============================================================================
  // INITIAL STATE
  // ============================================================================

  describe('initial state', () => {
    it('should have correct default values', () => {
      const state = useViewStore.getState();
      expect(state.categories).toEqual([]);
      expect(state.isRegistryLoaded).toBe(false);
      expect(state.activeViewId).toBe('data-complete');
      expect(state.isCustomQuery).toBe(false);
      expect(state.customQueryText).toBeNull();
      expect(state.params).toEqual({});
      expect(state.isLoading).toBe(false);
      expect(state.isExecuting).toBe(false);
      expect(state.error).toBeNull();
    });
  });

  // ============================================================================
  // loadRegistry
  // ============================================================================

  describe('loadRegistry', () => {
    it('should load categories from API on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockRegistryResponse,
      });

      await useViewStore.getState().loadRegistry();

      const state = useViewStore.getState();
      expect(state.categories).toEqual(mockCategories);
      expect(state.isRegistryLoaded).toBe(true);
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it('should set loading state during fetch', async () => {
      let resolvePromise: (value: unknown) => void;
      const promise = new Promise((resolve) => {
        resolvePromise = resolve;
      });
      mockFetch.mockImplementationOnce(() => promise);

      const loadPromise = useViewStore.getState().loadRegistry();

      // Check loading state immediately
      expect(useViewStore.getState().isLoading).toBe(true);

      // Resolve the fetch
      resolvePromise!({
        ok: true,
        json: async () => mockRegistryResponse,
      });

      await loadPromise;

      expect(useViewStore.getState().isLoading).toBe(false);
    });

    it('should not reload if registry is already loaded', async () => {
      // Pre-load registry
      useViewStore.setState({
        isRegistryLoaded: true,
        categories: mockCategories,
      });

      await useViewStore.getState().loadRegistry();

      // Fetch should not have been called
      expect(mockFetch).not.toHaveBeenCalled();
    });

    it('should handle API error response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: false,
          error: 'Failed to load views',
        }),
      });

      await useViewStore.getState().loadRegistry();

      const state = useViewStore.getState();
      expect(state.error).toBe('Failed to load views');
      expect(state.isLoading).toBe(false);
      expect(state.isRegistryLoaded).toBe(false);
    });

    it('should handle network errors', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      await useViewStore.getState().loadRegistry();

      const state = useViewStore.getState();
      expect(state.error).toBe('Network error');
      expect(state.isLoading).toBe(false);
      expect(state.isRegistryLoaded).toBe(false);
    });
  });

  // ============================================================================
  // selectView
  // ============================================================================

  describe('selectView', () => {
    it('should update activeViewId', () => {
      useViewStore.getState().selectView('data-realm-shared');

      const state = useViewStore.getState();
      expect(state.activeViewId).toBe('data-realm-shared');
    });

    it('should clear custom query state', () => {
      // Set custom query first
      useViewStore.setState({
        isCustomQuery: true,
        customQueryText: 'MATCH (n) RETURN n',
      });

      useViewStore.getState().selectView('data-complete');

      const state = useViewStore.getState();
      expect(state.isCustomQuery).toBe(false);
      expect(state.customQueryText).toBeNull();
    });

    it('should update params if provided', () => {
      useViewStore.getState().selectView('data-realm-shared', {
        key: 'my-project',
        locale: 'fr-FR',
      });

      const state = useViewStore.getState();
      expect(state.params).toEqual({
        key: 'my-project',
        locale: 'fr-FR',
      });
    });

    it('should not update params if not provided', () => {
      useViewStore.setState({
        params: { key: 'existing-key' },
      });

      useViewStore.getState().selectView('data-realm-shared');

      expect(useViewStore.getState().params).toEqual({ key: 'existing-key' });
    });
  });

  // ============================================================================
  // executeView
  // ============================================================================

  describe('executeView', () => {
    const mockViewResponse = {
      success: true,
      data: {
        id: 'data-complete',
        cypher: {
          query: 'MATCH (n) RETURN n LIMIT 100',
          params: { limit: 100 },
        },
      },
    };

    it('should set executing state during execution', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockViewResponse,
      });

      const promise = useViewStore.getState().executeView('data-complete');

      // Check executing state immediately
      expect(useViewStore.getState().isExecuting).toBe(true);

      await promise;

      expect(useViewStore.getState().isExecuting).toBe(false);
    });

    it('should update activeViewId and clear custom query', async () => {
      useViewStore.setState({
        isCustomQuery: true,
        customQueryText: 'CUSTOM',
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockViewResponse,
      });

      await useViewStore.getState().executeView('data-complete');

      const state = useViewStore.getState();
      expect(state.activeViewId).toBe('data-complete');
      expect(state.isCustomQuery).toBe(false);
      expect(state.customQueryText).toBeNull();
    });

    it('should call queryStore.executeQuery with Cypher from view (with filter injection)', async () => {
      const mockExecuteQuery = jest.fn().mockResolvedValue(undefined);
      (useQueryStore.getState as jest.Mock).mockReturnValue({
        executeQuery: mockExecuteQuery,
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockViewResponse,
      });

      await useViewStore.getState().executeView('data-complete');

      // v12.1: Query-First filter injection replaces LIMIT with displayLimit from filterStore
      // The original LIMIT 100 gets replaced with displayLimit=250
      expect(mockExecuteQuery).toHaveBeenCalledWith('MATCH (n) RETURN n LIMIT 250', { limit: 100 });
    });

    it('should include params in API URL', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockViewResponse,
      });

      await useViewStore.getState().executeView('data-realm-shared', {
        key: 'my-project',
        locale: 'en-US',
      });

      // v11.6.1: fetch now includes AbortController signal
      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/views/data-realm-shared'),
        expect.objectContaining({ signal: expect.any(AbortSignal) })
      );
      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('key=my-project'),
        expect.objectContaining({ signal: expect.any(AbortSignal) })
      );
      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('locale=en-US'),
        expect.objectContaining({ signal: expect.any(AbortSignal) })
      );
    });

    it('should handle API error', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: false,
          error: 'View not found',
        }),
      });

      await useViewStore.getState().executeView('invalid-view');

      const state = useViewStore.getState();
      expect(state.error).toBe('View not found');
      expect(state.isExecuting).toBe(false);
    });

    it('should handle missing Cypher in response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { id: 'test', cypher: {} }, // No query
        }),
      });

      await useViewStore.getState().executeView('test');

      expect(useViewStore.getState().error).toBe('View did not return a Cypher query');
    });

    it('should handle network errors', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Connection failed'));

      await useViewStore.getState().executeView('data-complete');

      const state = useViewStore.getState();
      expect(state.error).toBe('Connection failed');
      expect(state.isExecuting).toBe(false);
    });
  });

  // ============================================================================
  // executeCustomQuery
  // ============================================================================

  describe('executeCustomQuery', () => {
    it('should set custom query state', async () => {
      const mockExecuteQuery = jest.fn().mockResolvedValue(undefined);
      (useQueryStore.getState as jest.Mock).mockReturnValue({
        executeQuery: mockExecuteQuery,
      });

      await useViewStore.getState().executeCustomQuery('MATCH (n:Entity) RETURN n');

      const state = useViewStore.getState();
      expect(state.isCustomQuery).toBe(true);
      expect(state.customQueryText).toBe('MATCH (n:Entity) RETURN n');
    });

    it('should call queryStore.executeQuery', async () => {
      const mockExecuteQuery = jest.fn().mockResolvedValue(undefined);
      (useQueryStore.getState as jest.Mock).mockReturnValue({
        executeQuery: mockExecuteQuery,
      });

      await useViewStore.getState().executeCustomQuery('MATCH (n) RETURN n LIMIT 10');

      expect(mockExecuteQuery).toHaveBeenCalledWith('MATCH (n) RETURN n LIMIT 10');
    });

    it('should set executing state during query', async () => {
      let resolveQuery: () => void;
      const queryPromise = new Promise<void>((resolve) => {
        resolveQuery = resolve;
      });

      const mockExecuteQuery = jest.fn().mockReturnValue(queryPromise);
      (useQueryStore.getState as jest.Mock).mockReturnValue({
        executeQuery: mockExecuteQuery,
      });

      const executePromise = useViewStore.getState().executeCustomQuery('MATCH (n) RETURN n');

      expect(useViewStore.getState().isExecuting).toBe(true);

      resolveQuery!();
      await executePromise;

      expect(useViewStore.getState().isExecuting).toBe(false);
    });

    it('should handle query errors', async () => {
      const mockExecuteQuery = jest.fn().mockRejectedValue(new Error('Invalid query'));
      (useQueryStore.getState as jest.Mock).mockReturnValue({
        executeQuery: mockExecuteQuery,
      });

      await useViewStore.getState().executeCustomQuery('INVALID');

      const state = useViewStore.getState();
      expect(state.error).toBe('Invalid query');
      expect(state.isExecuting).toBe(false);
    });
  });

  // ============================================================================
  // loadDefaultView
  // ============================================================================

  describe('loadDefaultView', () => {
    it('should load registry if not loaded', async () => {
      mockFetch
        .mockResolvedValueOnce({
          ok: true,
          json: async () => mockRegistryResponse,
        })
        .mockResolvedValueOnce({
          ok: true,
          json: async () => ({
            success: true,
            data: { cypher: { query: 'MATCH (n) RETURN n', params: {} } },
          }),
        });

      await useViewStore.getState().loadDefaultView();

      expect(useViewStore.getState().isRegistryLoaded).toBe(true);
    });

    it('should not reload registry if already loaded', async () => {
      useViewStore.setState({
        isRegistryLoaded: true,
        categories: mockCategories,
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { cypher: { query: 'MATCH (n) RETURN n', params: {} } },
        }),
      });

      await useViewStore.getState().loadDefaultView();

      // Only one fetch for executeView, not for loadRegistry
      expect(mockFetch).toHaveBeenCalledTimes(1);
      // v11.6.1: fetch now includes AbortController signal
      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/views/data-complete'),
        expect.objectContaining({ signal: expect.any(AbortSignal) })
      );
    });

    it('should execute the persisted activeViewId', async () => {
      useViewStore.setState({
        isRegistryLoaded: true,
        categories: mockCategories,
        activeViewId: 'data-realm-shared',
      });

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { cypher: { query: 'MATCH (p:Project) RETURN p', params: {} } },
        }),
      });

      await useViewStore.getState().loadDefaultView();

      // v11.6.1: fetch now includes AbortController signal
      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/api/views/data-realm-shared'),
        expect.objectContaining({ signal: expect.any(AbortSignal) })
      );
    });
  });

  // ============================================================================
  // setParams
  // ============================================================================

  describe('setParams', () => {
    it('should update params', () => {
      useViewStore.getState().setParams({ key: 'test-key' });

      expect(useViewStore.getState().params).toEqual({ key: 'test-key' });
    });

    it('should merge with existing params', () => {
      useViewStore.setState({ params: { key: 'existing' } });

      useViewStore.getState().setParams({ locale: 'fr-FR' });

      expect(useViewStore.getState().params).toEqual({
        key: 'existing',
        locale: 'fr-FR',
      });
    });

    it('should override existing param values', () => {
      useViewStore.setState({ params: { key: 'old-key' } });

      useViewStore.getState().setParams({ key: 'new-key' });

      expect(useViewStore.getState().params.key).toBe('new-key');
    });
  });

  // ============================================================================
  // clearView
  // ============================================================================

  describe('clearView', () => {
    it('should reset to default view', () => {
      useViewStore.setState({
        activeViewId: 'data-realm-shared',
        isCustomQuery: true,
        customQueryText: 'CUSTOM QUERY',
        params: { key: 'test', locale: 'en-US' },
      });

      useViewStore.getState().clearView();

      const state = useViewStore.getState();
      expect(state.activeViewId).toBe('data-complete');
      expect(state.isCustomQuery).toBe(false);
      expect(state.customQueryText).toBeNull();
      expect(state.params).toEqual({});
    });
  });

  // ============================================================================
  // syncFromURL
  // ============================================================================

  describe('syncFromURL', () => {
    it('should parse view from URL', () => {
      const params = new URLSearchParams('view=data-realm-shared');

      useViewStore.getState().syncFromURL(params);

      expect(useViewStore.getState().activeViewId).toBe('data-realm-shared');
    });

    it('should parse all params from URL', () => {
      const params = new URLSearchParams('view=data-realm-shared&key=my-project&locale=fr-FR&project=proj1');

      useViewStore.getState().syncFromURL(params);

      const state = useViewStore.getState();
      expect(state.activeViewId).toBe('data-realm-shared');
      expect(state.params).toEqual({
        key: 'my-project',
        locale: 'fr-FR',
        project: 'proj1',
      });
    });

    it('should clear custom query state when syncing from URL', () => {
      useViewStore.setState({
        isCustomQuery: true,
        customQueryText: 'CUSTOM',
      });

      const params = new URLSearchParams('view=data-complete');
      useViewStore.getState().syncFromURL(params);

      expect(useViewStore.getState().isCustomQuery).toBe(false);
      expect(useViewStore.getState().customQueryText).toBeNull();
    });

    it('should not change state if view param is missing', () => {
      useViewStore.setState({ activeViewId: 'existing-view' });

      const params = new URLSearchParams('key=test');
      useViewStore.getState().syncFromURL(params);

      // Should not change activeViewId
      expect(useViewStore.getState().activeViewId).toBe('existing-view');
    });
  });

  // ============================================================================
  // toURLParams
  // ============================================================================

  describe('toURLParams', () => {
    it('should include view in URL', () => {
      useViewStore.setState({ activeViewId: 'data-realm-shared' });

      const params = useViewStore.getState().toURLParams();

      expect(params.get('view')).toBe('data-realm-shared');
    });

    it('should include all params in URL', () => {
      useViewStore.setState({
        activeViewId: 'data-realm-shared',
        params: { key: 'my-key', locale: 'en-US', project: 'proj1' },
      });

      const params = useViewStore.getState().toURLParams();

      expect(params.get('view')).toBe('data-realm-shared');
      expect(params.get('key')).toBe('my-key');
      expect(params.get('locale')).toBe('en-US');
      expect(params.get('project')).toBe('proj1');
    });

    it('should not include view when in custom query mode', () => {
      useViewStore.setState({
        activeViewId: 'data-complete',
        isCustomQuery: true,
      });

      const params = useViewStore.getState().toURLParams();

      expect(params.get('view')).toBeNull();
    });

    it('should not include undefined params', () => {
      useViewStore.setState({
        activeViewId: 'data-complete',
        params: { key: undefined, locale: 'en-US' },
      });

      const params = useViewStore.getState().toURLParams();

      expect(params.has('key')).toBe(false);
      expect(params.get('locale')).toBe('en-US');
    });
  });

  // ============================================================================
  // getViewById
  // ============================================================================

  describe('getViewById', () => {
    beforeEach(() => {
      useViewStore.setState({ categories: mockCategories });
    });

    it('should find view in first category', () => {
      const view = useViewStore.getState().getViewById('data-complete');

      expect(view).toBeDefined();
      expect(view?.id).toBe('data-complete');
    });

    it('should find view in second category', () => {
      // v11.6.1: Updated to use new unified view ID
      const view = useViewStore.getState().getViewById('ctx-generation');

      expect(view).toBeDefined();
      expect(view?.id).toBe('ctx-generation');
    });

    it('should return undefined for non-existent view', () => {
      const view = useViewStore.getState().getViewById('non-existent');

      expect(view).toBeUndefined();
    });

    it('should return undefined when categories are empty', () => {
      useViewStore.setState({ categories: [] });

      const view = useViewStore.getState().getViewById('data-complete');

      expect(view).toBeUndefined();
    });
  });

  // ============================================================================
  // getActiveView
  // ============================================================================

  describe('getActiveView', () => {
    beforeEach(() => {
      useViewStore.setState({ categories: mockCategories });
    });

    it('should return active view', () => {
      useViewStore.setState({ activeViewId: 'data-realm-shared' });

      const view = useViewStore.getState().getActiveView();

      expect(view).toBeDefined();
      expect(view?.id).toBe('data-realm-shared');
    });

    it('should return undefined when in custom query mode', () => {
      useViewStore.setState({
        activeViewId: 'data-complete',
        isCustomQuery: true,
      });

      const view = useViewStore.getState().getActiveView();

      expect(view).toBeUndefined();
    });

    it('should return undefined when activeViewId is not in registry', () => {
      useViewStore.setState({ activeViewId: 'non-existent' });

      const view = useViewStore.getState().getActiveView();

      expect(view).toBeUndefined();
    });
  });

  // ============================================================================
  // SELECTORS
  // ============================================================================

  describe('selectors', () => {
    it('should export selectActiveViewId', () => {
      useViewStore.setState({ activeViewId: 'test-view' });

      expect(selectActiveViewId(useViewStore.getState())).toBe('test-view');
    });

    it('should export selectIsCustomQuery', () => {
      useViewStore.setState({ isCustomQuery: true });

      expect(selectIsCustomQuery(useViewStore.getState())).toBe(true);
    });

    it('should export selectIsExecuting', () => {
      useViewStore.setState({ isExecuting: true });

      expect(selectIsExecuting(useViewStore.getState())).toBe(true);
    });

    it('should export selectCategories', () => {
      useViewStore.setState({ categories: mockCategories });

      expect(selectCategories(useViewStore.getState())).toEqual(mockCategories);
    });

    it('should export selectIsRegistryLoaded', () => {
      useViewStore.setState({ isRegistryLoaded: true });

      expect(selectIsRegistryLoaded(useViewStore.getState())).toBe(true);
    });
  });
});
