/**
 * Graph Store Tests
 *
 * Tests for the graph data store with O(1) indexed lookups.
 * Covers setGraphData, mergeGraphData, and index building.
 */

import { useGraphStore } from '../graphStore';
import type { GraphNode, GraphEdge, NodeType, RelationType } from '@/types';

// Helper to create test nodes
const createNode = (id: string, type: NodeType = 'Entity'): GraphNode => ({
  id,
  type,
  key: `test-${id}`,
  displayName: `Test ${id}`,
});

// Helper to create test edges
const createEdge = (id: string, source: string, target: string, type: RelationType = 'HAS_NATIVE'): GraphEdge => ({
  id,
  source,
  target,
  type,
});

describe('graphStore', () => {
  beforeEach(() => {
    // Reset store to initial state before each test
    useGraphStore.setState({
      nodes: [],
      edges: [],
      isLoading: false,
      error: null,
      lastFetchTime: null,
      nodeMap: new Map(),
      adjacencyMap: new Map(),
      edgesBySource: new Map(),
      edgesByTarget: new Map(),
      edgeMap: new Map(),
      hiddenNodeIds: new Set(),
      totalNodes: 0,
      totalArcs: 0,
      nodeTypeCounts: {},
    });
  });

  // ==========================================================================
  // setGraphData
  // ==========================================================================

  describe('setGraphData', () => {
    it('should set nodes and edges', () => {
      const nodes = [createNode('1'), createNode('2')];
      const edges = [createEdge('e1', '1', '2')];

      useGraphStore.getState().setGraphData({ nodes, edges });

      const state = useGraphStore.getState();
      expect(state.nodes).toEqual(nodes);
      expect(state.edges).toEqual(edges);
      expect(state.totalNodes).toBe(2);
      expect(state.totalArcs).toBe(1);
    });

    it('should build nodeMap index for O(1) lookup', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];

      useGraphStore.getState().setGraphData({ nodes, edges: [] });

      const state = useGraphStore.getState();
      expect(state.nodeMap.size).toBe(3);
      expect(state.nodeMap.get('1')).toEqual(nodes[0]);
      expect(state.nodeMap.get('2')).toEqual(nodes[1]);
      expect(state.nodeMap.get('3')).toEqual(nodes[2]);
    });

    it('should build adjacencyMap for connected nodes', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];
      const edges = [
        createEdge('e1', '1', '2'),
        createEdge('e2', '2', '3'),
      ];

      useGraphStore.getState().setGraphData({ nodes, edges });

      const state = useGraphStore.getState();
      // Adjacency is bidirectional
      expect(state.adjacencyMap.get('1')?.has('2')).toBe(true);
      expect(state.adjacencyMap.get('2')?.has('1')).toBe(true);
      expect(state.adjacencyMap.get('2')?.has('3')).toBe(true);
      expect(state.adjacencyMap.get('3')?.has('2')).toBe(true);
      // No direct connection between 1 and 3
      expect(state.adjacencyMap.get('1')?.has('3')).toBeFalsy();
    });

    it('should build edgesBySource index', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];
      const edges = [
        createEdge('e1', '1', '2'),
        createEdge('e2', '1', '3'),
      ];

      useGraphStore.getState().setGraphData({ nodes, edges });

      const state = useGraphStore.getState();
      expect(state.edgesBySource.get('1')?.length).toBe(2);
      expect(state.edgesBySource.get('2')).toBeUndefined();
    });

    it('should build edgesByTarget index', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];
      const edges = [
        createEdge('e1', '1', '2'),
        createEdge('e2', '3', '2'),
      ];

      useGraphStore.getState().setGraphData({ nodes, edges });

      const state = useGraphStore.getState();
      expect(state.edgesByTarget.get('2')?.length).toBe(2);
      expect(state.edgesByTarget.get('1')).toBeUndefined();
    });

    it('should build edgeMap for O(1) edge lookup', () => {
      const edges = [createEdge('e1', '1', '2'), createEdge('e2', '2', '3')];

      useGraphStore.getState().setGraphData({ nodes: [], edges });

      const state = useGraphStore.getState();
      expect(state.edgeMap.get('e1')).toEqual(edges[0]);
      expect(state.edgeMap.get('e2')).toEqual(edges[1]);
    });

    it('should count node types correctly', () => {
      const nodes = [
        createNode('1', 'Entity'),
        createNode('2', 'Entity'),
        createNode('3', 'Page'),
        createNode('4', 'Locale'),
      ];

      useGraphStore.getState().setGraphData({ nodes, edges: [] });

      const state = useGraphStore.getState();
      expect(state.nodeTypeCounts).toEqual({
        Entity: 2,
        Page: 1,
        Locale: 1,
      });
    });

    it('should set lastFetchTime', () => {
      const before = Date.now();
      useGraphStore.getState().setGraphData({ nodes: [], edges: [] });
      const after = Date.now();

      const state = useGraphStore.getState();
      expect(state.lastFetchTime).toBeGreaterThanOrEqual(before);
      expect(state.lastFetchTime).toBeLessThanOrEqual(after);
    });

    it('should clear error state', () => {
      useGraphStore.setState({ error: 'Previous error' });

      useGraphStore.getState().setGraphData({ nodes: [], edges: [] });

      expect(useGraphStore.getState().error).toBeNull();
    });
  });

  // ==========================================================================
  // mergeGraphData
  // ==========================================================================

  describe('mergeGraphData', () => {
    it('should add new nodes without duplicates', () => {
      const initialNodes = [createNode('1')];
      useGraphStore.getState().setGraphData({ nodes: initialNodes, edges: [] });

      useGraphStore.getState().mergeGraphData(
        [createNode('1'), createNode('2')], // 1 is duplicate
        []
      );

      const state = useGraphStore.getState();
      expect(state.nodes.length).toBe(2);
      expect(state.totalNodes).toBe(2);
    });

    it('should add new edges without duplicates', () => {
      const initialEdges = [createEdge('e1', '1', '2')];
      useGraphStore.getState().setGraphData({
        nodes: [createNode('1'), createNode('2'), createNode('3')],
        edges: initialEdges,
      });

      useGraphStore.getState().mergeGraphData(
        [],
        [createEdge('e1', '1', '2'), createEdge('e2', '2', '3')] // e1 is duplicate
      );

      const state = useGraphStore.getState();
      expect(state.edges.length).toBe(2);
      expect(state.totalArcs).toBe(2);
    });

    it('should update indexes when merging', () => {
      useGraphStore.getState().setGraphData({
        nodes: [createNode('1')],
        edges: [],
      });

      useGraphStore.getState().mergeGraphData(
        [createNode('2')],
        [createEdge('e1', '1', '2')]
      );

      const state = useGraphStore.getState();
      expect(state.nodeMap.get('2')).toBeDefined();
      expect(state.edgeMap.get('e1')).toBeDefined();
      expect(state.adjacencyMap.get('1')?.has('2')).toBe(true);
    });

    it('should update node type counts when merging', () => {
      useGraphStore.getState().setGraphData({
        nodes: [createNode('1', 'Entity')],
        edges: [],
      });

      useGraphStore.getState().mergeGraphData(
        [createNode('2', 'Page'), createNode('3', 'Entity')],
        []
      );

      const state = useGraphStore.getState();
      expect(state.nodeTypeCounts).toEqual({
        Entity: 2,
        Page: 1,
      });
    });

    it('should not modify state if nothing to add', () => {
      const initialNodes = [createNode('1')];
      useGraphStore.getState().setGraphData({ nodes: initialNodes, edges: [] });

      useGraphStore.getState().mergeGraphData([createNode('1')], []); // All duplicates

      // Node count should remain the same
      expect(useGraphStore.getState().nodes.length).toBe(1);
    });
  });

  // ==========================================================================
  // getNodeById (O(1) lookup)
  // ==========================================================================

  describe('getNodeById', () => {
    it('should return node by id in O(1)', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];
      useGraphStore.getState().setGraphData({ nodes, edges: [] });

      const node = useGraphStore.getState().getNodeById('2');

      expect(node).toEqual(nodes[1]);
    });

    it('should return undefined for non-existent id', () => {
      useGraphStore.getState().setGraphData({ nodes: [], edges: [] });

      const node = useGraphStore.getState().getNodeById('nonexistent');

      expect(node).toBeUndefined();
    });
  });

  // ==========================================================================
  // getEdgeById (O(1) lookup)
  // ==========================================================================

  describe('getEdgeById', () => {
    it('should return edge by id in O(1)', () => {
      const edges = [createEdge('e1', '1', '2'), createEdge('e2', '2', '3')];
      useGraphStore.getState().setGraphData({ nodes: [], edges });

      const edge = useGraphStore.getState().getEdgeById('e2');

      expect(edge).toEqual(edges[1]);
    });

    it('should return undefined for non-existent id', () => {
      useGraphStore.getState().setGraphData({ nodes: [], edges: [] });

      const edge = useGraphStore.getState().getEdgeById('nonexistent');

      expect(edge).toBeUndefined();
    });
  });

  // ==========================================================================
  // getNodeDetail (O(k) where k = connected edges)
  // ==========================================================================

  describe('getNodeDetail', () => {
    it('should return node with incoming and outgoing relations', () => {
      const nodes = [createNode('1'), createNode('2'), createNode('3')];
      const edges = [
        createEdge('e1', '1', '2', 'HAS_NATIVE'),
        createEdge('e2', '2', '3', 'HAS_PAGE'),
      ];
      useGraphStore.getState().setGraphData({ nodes, edges });

      const detail = useGraphStore.getState().getNodeDetail('2');

      expect(detail).not.toBeNull();
      expect(detail?.node.id).toBe('2');
      expect(detail?.relations.incoming.length).toBe(1);
      expect(detail?.relations.incoming[0].type).toBe('HAS_NATIVE');
      expect(detail?.relations.incoming[0].node.id).toBe('1');
      expect(detail?.relations.outgoing.length).toBe(1);
      expect(detail?.relations.outgoing[0].type).toBe('HAS_PAGE');
      expect(detail?.relations.outgoing[0].node.id).toBe('3');
    });

    it('should return null for non-existent node', () => {
      useGraphStore.getState().setGraphData({ nodes: [], edges: [] });

      const detail = useGraphStore.getState().getNodeDetail('nonexistent');

      expect(detail).toBeNull();
    });

    it('should handle node with no connections', () => {
      const nodes = [createNode('1')];
      useGraphStore.getState().setGraphData({ nodes, edges: [] });

      const detail = useGraphStore.getState().getNodeDetail('1');

      expect(detail).not.toBeNull();
      expect(detail?.relations.incoming).toEqual([]);
      expect(detail?.relations.outgoing).toEqual([]);
    });
  });

  // ==========================================================================
  // Other Actions
  // ==========================================================================

  describe('setLoading', () => {
    it('should set loading state', () => {
      useGraphStore.getState().setLoading(true);
      expect(useGraphStore.getState().isLoading).toBe(true);

      useGraphStore.getState().setLoading(false);
      expect(useGraphStore.getState().isLoading).toBe(false);
    });
  });

  describe('setError', () => {
    it('should set error and clear loading', () => {
      useGraphStore.setState({ isLoading: true });

      useGraphStore.getState().setError('Network error');

      const state = useGraphStore.getState();
      expect(state.error).toBe('Network error');
      expect(state.isLoading).toBe(false);
    });

    it('should clear error when set to null', () => {
      useGraphStore.setState({ error: 'Previous error' });

      useGraphStore.getState().setError(null);

      expect(useGraphStore.getState().error).toBeNull();
    });
  });

  describe('clearGraph', () => {
    it('should reset all state to initial values', () => {
      // Set up some state
      useGraphStore.getState().setGraphData({
        nodes: [createNode('1')],
        edges: [createEdge('e1', '1', '2')],
      });
      useGraphStore.getState().hideNode('1');

      // Clear
      useGraphStore.getState().clearGraph();

      const state = useGraphStore.getState();
      expect(state.nodes).toEqual([]);
      expect(state.edges).toEqual([]);
      expect(state.nodeMap.size).toBe(0);
      expect(state.edgeMap.size).toBe(0);
      expect(state.adjacencyMap.size).toBe(0);
      expect(state.hiddenNodeIds.size).toBe(0);
      expect(state.totalNodes).toBe(0);
      expect(state.totalArcs).toBe(0);
      expect(state.nodeTypeCounts).toEqual({});
    });
  });

  describe('hideNode', () => {
    it('should add node id to hiddenNodeIds', () => {
      useGraphStore.getState().hideNode('1');
      useGraphStore.getState().hideNode('2');

      const state = useGraphStore.getState();
      expect(state.hiddenNodeIds.has('1')).toBe(true);
      expect(state.hiddenNodeIds.has('2')).toBe(true);
    });
  });
});
