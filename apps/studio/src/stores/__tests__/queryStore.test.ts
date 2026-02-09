// src/stores/__tests__/queryStore.test.ts
import { useQueryStore } from '../queryStore';
import { useGraphStore } from '../graphStore';

// Mock fetch globally
const mockFetch = jest.fn();
global.fetch = mockFetch;

// Mock graphStore
jest.mock('../graphStore', () => ({
  useGraphStore: {
    getState: jest.fn(() => ({
      setGraphData: jest.fn(),
    })),
  },
}));

describe('queryStore', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    // Reset store state
    useQueryStore.setState({
      currentQuery: null,
      isExecuting: false,
      error: null,
      result: null,
      viewMode: 'graph',
    });
  });

  describe('executeQuery', () => {
    it('should set isExecuting to true during query execution', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { nodes: [], edges: [] },
          meta: { totalNodes: 0, totalArcs: 0 },
        }),
      });

      const executePromise = useQueryStore.getState().executeQuery('MATCH (n) RETURN n');

      // Check state immediately after call
      expect(useQueryStore.getState().isExecuting).toBe(true);
      expect(useQueryStore.getState().currentQuery).toBe('MATCH (n) RETURN n');

      await executePromise;

      // Check state after completion
      expect(useQueryStore.getState().isExecuting).toBe(false);
    });

    it('should handle successful query response', async () => {
      const mockNodes = [{ id: '1', type: 'Entity', key: 'test' }];
      const mockEdges = [{ id: 'e1', source: '1', target: '2', type: 'HAS_CONTENT' }];

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { nodes: mockNodes, edges: mockEdges },
          meta: { totalNodes: 1, totalArcs: 1, queryDuration: 50 },
        }),
      });

      await useQueryStore.getState().executeQuery('MATCH (n) RETURN n');

      const state = useQueryStore.getState();
      expect(state.error).toBeNull();
      expect(state.result).toBeDefined();
      expect(state.result?.nodes).toEqual(mockNodes);
      expect(state.result?.edges).toEqual(mockEdges);
      expect(state.result?.totalNodes).toBe(1);
      expect(state.result?.totalArcs).toBe(1);
    });

    it('should handle error response from server', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        json: async () => ({
          success: false,
          error: 'Invalid Cypher syntax',
        }),
      });

      await useQueryStore.getState().executeQuery('INVALID QUERY');

      const state = useQueryStore.getState();
      expect(state.error).toBe('Invalid Cypher syntax');
      expect(state.isExecuting).toBe(false);
    });

    it('should handle network errors', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      await useQueryStore.getState().executeQuery('MATCH (n) RETURN n');

      const state = useQueryStore.getState();
      expect(state.error).toBe('Network error');
      expect(state.isExecuting).toBe(false);
    });

    it('should cancel previous query when new query starts (race condition handling)', async () => {
      // First query - will be slow
      const slowResponse = new Promise((resolve) => {
        setTimeout(() => {
          resolve({
            ok: true,
            json: async () => ({
              success: true,
              data: { nodes: [{ id: 'old' }], edges: [] },
            }),
          });
        }, 100);
      });

      // Second query - fast response
      const fastResponse = {
        ok: true,
        json: async () => ({
          success: true,
          data: { nodes: [{ id: 'new' }], edges: [] },
        }),
      };

      mockFetch
        .mockImplementationOnce(() => slowResponse)
        .mockResolvedValueOnce(fastResponse);

      // Start first query (slow) - don't await, let it be cancelled
      void useQueryStore.getState().executeQuery('SLOW QUERY');

      // Immediately start second query (fast) - should abort first
      const secondPromise = useQueryStore.getState().executeQuery('FAST QUERY');

      // Wait for second query to complete
      await secondPromise;

      // First query should have been aborted (won't resolve normally)
      // The result should be from the second query
      const state = useQueryStore.getState();
      expect(state.currentQuery).toBe('FAST QUERY');
      expect(state.result?.nodes[0]?.id).toBe('new');
    });

    it('should ignore AbortError and not set error state', async () => {
      // Simulate AbortError
      const abortError = new Error('Aborted');
      abortError.name = 'AbortError';

      mockFetch.mockRejectedValueOnce(abortError);

      await useQueryStore.getState().executeQuery('MATCH (n) RETURN n');

      const state = useQueryStore.getState();
      // AbortError should NOT set error state
      expect(state.error).toBeNull();
    });

    it('should update graphStore with results', async () => {
      const mockSetGraphData = jest.fn();
      (useGraphStore.getState as jest.Mock).mockReturnValue({
        setGraphData: mockSetGraphData,
      });

      const mockNodes = [{ id: '1' }];
      const mockEdges = [{ id: 'e1' }];

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          success: true,
          data: { nodes: mockNodes, edges: mockEdges },
        }),
      });

      await useQueryStore.getState().executeQuery('MATCH (n) RETURN n');

      expect(mockSetGraphData).toHaveBeenCalledWith({
        nodes: mockNodes,
        edges: mockEdges,
      });
    });
  });

  describe('clear', () => {
    it('should reset all query state', () => {
      // Set some state
      useQueryStore.setState({
        currentQuery: 'MATCH (n) RETURN n',
        isExecuting: true,
        error: 'Some error',
        result: {
          nodes: [],
          edges: [],
          totalNodes: 0,
          totalArcs: 0,
          duration: 0,
          timestamp: new Date(),
        },
      });

      // Clear
      useQueryStore.getState().clear();

      const state = useQueryStore.getState();
      expect(state.currentQuery).toBeNull();
      expect(state.isExecuting).toBe(false);
      expect(state.error).toBeNull();
      expect(state.result).toBeNull();
    });
  });

  describe('setViewMode', () => {
    it('should update view mode', () => {
      useQueryStore.getState().setViewMode('table');
      expect(useQueryStore.getState().viewMode).toBe('table');

      useQueryStore.getState().setViewMode('raw');
      expect(useQueryStore.getState().viewMode).toBe('raw');

      useQueryStore.getState().setViewMode('graph');
      expect(useQueryStore.getState().viewMode).toBe('graph');
    });
  });
});
