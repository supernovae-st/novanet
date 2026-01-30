/**
 * useNodeExpansion Hook Tests
 *
 * Tests for Neo4j Browser-style node expansion (double-click to expand neighbors).
 *
 * Note: Internal state testing (expandedNodes Set) is limited due to React 19 /
 * testing-library interaction. Tests focus on observable behavior:
 * - API calls and parameters
 * - Return values
 * - Store interactions (mergeGraphData)
 * - Concurrent expansion prevention
 */

import { renderHook, act } from '@testing-library/react';
import { useNodeExpansion } from '../useNodeExpansion';
import * as fetchClient from '@/lib/fetchClient';
import type { GraphNode, GraphEdge } from '@/types';

// Mock fetchClient
jest.mock('@/lib/fetchClient', () => ({
  postJSON: jest.fn(),
}));

// Mock graphStore - create a more realistic mock
const mockMergeGraphData = jest.fn();
let mockNodes: GraphNode[] = [];
let mockTotalNodes = 0;

// Create the mock store state getter
const createMockState = () => ({
  nodes: mockNodes,
  totalNodes: mockTotalNodes,
  mergeGraphData: mockMergeGraphData,
});

jest.mock('@/stores/graphStore', () => {
  return {
    useGraphStore: Object.assign(
      // The hook selector function
      (selector: (state: ReturnType<typeof createMockState>) => unknown) => {
        return selector(createMockState());
      },
      // The getState() static method
      {
        getState: () => createMockState(),
      }
    ),
  };
});

const mockPostJSON = fetchClient.postJSON as jest.MockedFunction<typeof fetchClient.postJSON>;

describe('useNodeExpansion', () => {
  // Test data
  const mockNeighborNodes: GraphNode[] = [
    { id: 'neighbor-1', type: 'Concept', key: 'concept-1', displayName: 'Concept 1' },
    { id: 'neighbor-2', type: 'Expression', key: 'expr-1', displayName: 'Expression 1' },
  ];

  const mockNeighborEdges: GraphEdge[] = [
    { id: 'edge-1', source: 'node-1', target: 'neighbor-1', type: 'USES_CONCEPT' },
    { id: 'edge-2', source: 'node-1', target: 'neighbor-2', type: 'HAS_EXPRESSION' },
  ];

  beforeEach(() => {
    jest.clearAllMocks();
    mockNodes = [];
    mockTotalNodes = 0;

    // Default mock: successful expansion
    mockPostJSON.mockResolvedValue({
      nodes: mockNeighborNodes,
      edges: mockNeighborEdges,
      totalNodes: 2,
      totalEdges: 2,
      duration: 50,
    });
  });

  describe('expandNode', () => {
    it('should call API with correct parameters', async () => {
      const { result } = renderHook(() => useNodeExpansion());

      await act(async () => {
        await result.current.expandNode('node-1');
      });

      // Verify API was called correctly
      expect(mockPostJSON).toHaveBeenCalledWith('/api/graph/expand', {
        nodeId: 'node-1',
        limit: 50,
      });
    });

    it('should return expansion result with nodes and edges', async () => {
      const { result } = renderHook(() => useNodeExpansion());

      let expansion: Awaited<ReturnType<typeof result.current.expandNode>>;
      await act(async () => {
        expansion = await result.current.expandNode('node-1');
      });

      // Verify result (return value, not state)
      expect(expansion!.nodes).toHaveLength(2);
      expect(expansion!.nodes[0].id).toBe('neighbor-1');
      expect(expansion!.nodes[1].id).toBe('neighbor-2');
      expect(expansion!.edges).toHaveLength(2);
      expect(expansion!.addedCount).toBe(2);
    });

    it('should call mergeGraphData with new nodes and edges', async () => {
      const { result } = renderHook(() => useNodeExpansion());

      await act(async () => {
        await result.current.expandNode('node-1');
      });

      // Verify mergeGraphData was called
      expect(mockMergeGraphData).toHaveBeenCalledWith(mockNeighborNodes, mockNeighborEdges);
    });

    it('should respect custom limit parameter', async () => {
      const { result } = renderHook(() => useNodeExpansion());

      await act(async () => {
        await result.current.expandNode('node-1', 100);
      });

      expect(mockPostJSON).toHaveBeenCalledWith('/api/graph/expand', {
        nodeId: 'node-1',
        limit: 100,
      });
    });

    it('should filter out existing nodes before merging', async () => {
      // Setup: graph already has one of the neighbor nodes
      mockNodes = [{ id: 'neighbor-1', type: 'Concept', key: 'existing', displayName: 'Existing' }];

      const { result } = renderHook(() => useNodeExpansion());

      let expansion: Awaited<ReturnType<typeof result.current.expandNode>>;
      await act(async () => {
        expansion = await result.current.expandNode('node-1');
      });

      // Only new nodes should be in result (neighbor-2, not neighbor-1)
      expect(expansion!.nodes).toHaveLength(1);
      expect(expansion!.nodes[0].id).toBe('neighbor-2');
      expect(expansion!.addedCount).toBe(1);

      // mergeGraphData should be called with filtered nodes
      expect(mockMergeGraphData).toHaveBeenCalledWith(
        [mockNeighborNodes[1]], // Only neighbor-2
        mockNeighborEdges       // All edges (store handles edge deduplication)
      );
    });

    it('should handle API errors gracefully', async () => {
      mockPostJSON.mockRejectedValue(new Error('Network error'));

      const { result } = renderHook(() => useNodeExpansion());

      await expect(
        act(async () => {
          await result.current.expandNode('node-1');
        })
      ).rejects.toThrow('Network error');

      // isExpanding should be false after error
      expect(result.current.isExpanding).toBe(false);
    });

    it('should have initial state with empty expandedNodes', () => {
      const { result } = renderHook(() => useNodeExpansion());

      expect(result.current.isExpanding).toBe(false);
      expect(result.current.expandedNodes.size).toBe(0);
    });
  });

  describe('isExpanding state', () => {
    it('should be true during expansion and false after', async () => {
      // Create a controllable promise
      let resolveExpansion: (value: unknown) => void;
      const pendingExpansion = new Promise((resolve) => {
        resolveExpansion = resolve;
      });
      mockPostJSON.mockReturnValue(pendingExpansion as Promise<unknown>);

      const { result } = renderHook(() => useNodeExpansion());

      // Start expansion (don't await)
      let expansionPromise: Promise<unknown>;
      act(() => {
        expansionPromise = result.current.expandNode('node-1');
      });

      // Should be expanding (state update happens synchronously in act)
      expect(result.current.isExpanding).toBe(true);

      // Resolve the expansion
      await act(async () => {
        resolveExpansion!({
          nodes: mockNeighborNodes,
          edges: mockNeighborEdges,
          totalNodes: 2,
          totalEdges: 2,
          duration: 50,
        });
        await expansionPromise;
      });

      // Should no longer be expanding
      expect(result.current.isExpanding).toBe(false);
    });
  });

  describe('expandedNodes behavior', () => {
    it('should reset expandedNodes when graph is cleared (totalNodes becomes 0)', async () => {
      // Start with some data
      mockTotalNodes = 10;

      const { result, rerender } = renderHook(() => useNodeExpansion());

      // Initial state
      expect(result.current.expandedNodes.size).toBe(0);

      // Expand a node
      await act(async () => {
        await result.current.expandNode('node-1');
      });

      // Simulate graph clear (totalNodes becomes 0)
      mockTotalNodes = 0;
      mockNodes = [];

      // Rerender to trigger the effect
      await act(async () => {
        rerender();
      });

      // The effect should have cleared expanded nodes
      expect(result.current.expandedNodes.size).toBe(0);
    });
  });

  describe('no changes scenario', () => {
    it('should not call mergeGraphData when no new nodes or edges', async () => {
      // Setup: all neighbor nodes already exist
      mockNodes = [...mockNeighborNodes];
      mockPostJSON.mockResolvedValue({
        nodes: mockNeighborNodes,
        edges: [], // No new edges either
        totalNodes: 2,
        totalEdges: 0,
        duration: 50,
      });

      const { result } = renderHook(() => useNodeExpansion());

      await act(async () => {
        await result.current.expandNode('node-1');
      });

      // mergeGraphData should NOT be called (nothing new to add)
      expect(mockMergeGraphData).not.toHaveBeenCalled();
    });

    it('should return empty result when all nodes exist', async () => {
      mockNodes = [...mockNeighborNodes];
      mockPostJSON.mockResolvedValue({
        nodes: mockNeighborNodes,
        edges: [],
        totalNodes: 2,
        totalEdges: 0,
        duration: 50,
      });

      const { result } = renderHook(() => useNodeExpansion());

      let expansion: Awaited<ReturnType<typeof result.current.expandNode>>;
      await act(async () => {
        expansion = await result.current.expandNode('node-1');
      });

      // No new nodes to add
      expect(expansion!.nodes).toHaveLength(0);
      expect(expansion!.addedCount).toBe(0);
    });
  });

  describe('concurrent expansion prevention', () => {
    it('should prevent rapid double-click from triggering multiple expansions', async () => {
      // Create a slow promise
      let resolveFirst: (value: unknown) => void;
      const slowExpansion = new Promise((resolve) => {
        resolveFirst = resolve;
      });
      mockPostJSON.mockReturnValueOnce(slowExpansion as Promise<unknown>);

      const { result } = renderHook(() => useNodeExpansion());

      // Start first expansion (don't await)
      let firstPromise: Promise<unknown>;
      act(() => {
        firstPromise = result.current.expandNode('node-1');
      });

      // Should be expanding
      expect(result.current.isExpanding).toBe(true);

      // Try to expand same node again while first is in progress
      let secondResult: Awaited<ReturnType<typeof result.current.expandNode>>;
      await act(async () => {
        secondResult = await result.current.expandNode('node-1');
      });

      // Second call should return empty immediately (node is being expanded)
      expect(secondResult!.nodes).toHaveLength(0);
      expect(secondResult!.addedCount).toBe(0);

      // Only one API call should have been made
      expect(mockPostJSON).toHaveBeenCalledTimes(1);

      // Clean up: resolve the first expansion
      await act(async () => {
        resolveFirst!({
          nodes: mockNeighborNodes,
          edges: mockNeighborEdges,
          totalNodes: 2,
          totalEdges: 2,
          duration: 50,
        });
        await firstPromise;
      });
    });

    it('should allow expanding different nodes concurrently', async () => {
      const { result } = renderHook(() => useNodeExpansion());

      // First expansion
      await act(async () => {
        await result.current.expandNode('node-1');
      });

      mockPostJSON.mockClear();

      // Second expansion of different node (should work)
      await act(async () => {
        await result.current.expandNode('node-2');
      });

      // API should have been called for second node
      expect(mockPostJSON).toHaveBeenCalledWith('/api/graph/expand', {
        nodeId: 'node-2',
        limit: 50,
      });
    });
  });
});
