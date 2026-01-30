import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type { GraphNode, GraphEdge, GraphData, NodeDetail } from '@/types';

interface GraphState {
  // Data
  nodes: GraphNode[];
  edges: GraphEdge[];
  isLoading: boolean;
  error: string | null;
  lastFetchTime: number | null;

  // Indexed maps for O(1) lookups (critical for 19k nodes)
  nodeMap: Map<string, GraphNode>;
  adjacencyMap: Map<string, Set<string>>;
  /** Edges indexed by source node ID - for O(k) getNodeDetail */
  edgesBySource: Map<string, GraphEdge[]>;
  /** Edges indexed by target node ID - for O(k) getNodeDetail */
  edgesByTarget: Map<string, GraphEdge[]>;
  /** Edges indexed by ID - for O(1) getEdgeById */
  edgeMap: Map<string, GraphEdge>;

  // Hidden nodes (for hide/show functionality)
  hiddenNodeIds: Set<string>;

  // Stats
  totalNodes: number;
  totalEdges: number;
  nodeTypeCounts: Record<string, number>;

  // Actions
  setGraphData: (data: GraphData) => void;
  mergeGraphData: (newNodes: GraphNode[], newEdges: GraphEdge[]) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearGraph: () => void;
  hideNode: (nodeId: string) => void;

  // Computed (now O(1) instead of O(n))
  getNodeById: (id: string) => GraphNode | undefined;
  getEdgeById: (id: string) => GraphEdge | undefined;
  getNodeDetail: (nodeId: string) => NodeDetail | null;
}

export const useGraphStore = create<GraphState>()(
  immer((set, get) => ({
    // Initial state
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
    totalEdges: 0,
    nodeTypeCounts: {},

    // Actions
    setGraphData: (data) => {
      set((state) => {
        state.nodes = data.nodes;
        state.edges = data.edges;
        state.totalNodes = data.nodes.length;
        state.totalEdges = data.edges.length;
        state.lastFetchTime = Date.now();
        state.error = null;

        // Build indexed maps for O(1) lookups (critical for 19k nodes)
        const nodeMap = new Map<string, GraphNode>();
        const adjacencyMap = new Map<string, Set<string>>();
        const edgesBySource = new Map<string, GraphEdge[]>();
        const edgesByTarget = new Map<string, GraphEdge[]>();
        const edgeMap = new Map<string, GraphEdge>();
        const counts: Record<string, number> = {};

        // Index nodes
        data.nodes.forEach((node) => {
          nodeMap.set(node.id, node);
          counts[node.type] = (counts[node.type] || 0) + 1;
        });

        // Build adjacency map and edge indexes
        data.edges.forEach((edge) => {
          // Adjacency map (bidirectional)
          if (!adjacencyMap.has(edge.source)) {
            adjacencyMap.set(edge.source, new Set());
          }
          if (!adjacencyMap.has(edge.target)) {
            adjacencyMap.set(edge.target, new Set());
          }
          adjacencyMap.get(edge.source)!.add(edge.target);
          adjacencyMap.get(edge.target)!.add(edge.source);

          // Edge indexes by source and target (for O(k) getNodeDetail)
          if (!edgesBySource.has(edge.source)) {
            edgesBySource.set(edge.source, []);
          }
          if (!edgesByTarget.has(edge.target)) {
            edgesByTarget.set(edge.target, []);
          }
          edgesBySource.get(edge.source)!.push(edge);
          edgesByTarget.get(edge.target)!.push(edge);

          // Edge map for O(1) getEdgeById
          edgeMap.set(edge.id, edge);
        });

        state.nodeMap = nodeMap;
        state.adjacencyMap = adjacencyMap;
        state.edgesBySource = edgesBySource;
        state.edgesByTarget = edgesByTarget;
        state.edgeMap = edgeMap;
        state.nodeTypeCounts = counts;
      });
    },

    mergeGraphData: (newNodes, newEdges) => {
      set((state) => {
        // Get existing IDs for deduplication
        const existingNodeIds = new Set(state.nodes.map((n) => n.id));
        const existingEdgeIds = new Set(state.edges.map((e) => e.id));

        // Filter out duplicates
        const nodesToAdd = newNodes.filter((n) => !existingNodeIds.has(n.id));
        const edgesToAdd = newEdges.filter((e) => !existingEdgeIds.has(e.id));

        // Early return if nothing to add
        if (nodesToAdd.length === 0 && edgesToAdd.length === 0) {
          return;
        }

        // Add new nodes to array and update indexes
        for (const node of nodesToAdd) {
          state.nodes.push(node);
          state.nodeMap.set(node.id, node);
          state.nodeTypeCounts[node.type] = (state.nodeTypeCounts[node.type] || 0) + 1;
        }

        // Add new edges and update indexes
        for (const edge of edgesToAdd) {
          state.edges.push(edge);

          // Update adjacency map (bidirectional)
          if (!state.adjacencyMap.has(edge.source)) {
            state.adjacencyMap.set(edge.source, new Set());
          }
          if (!state.adjacencyMap.has(edge.target)) {
            state.adjacencyMap.set(edge.target, new Set());
          }
          state.adjacencyMap.get(edge.source)!.add(edge.target);
          state.adjacencyMap.get(edge.target)!.add(edge.source);

          // Update edge indexes
          if (!state.edgesBySource.has(edge.source)) {
            state.edgesBySource.set(edge.source, []);
          }
          if (!state.edgesByTarget.has(edge.target)) {
            state.edgesByTarget.set(edge.target, []);
          }
          state.edgesBySource.get(edge.source)!.push(edge);
          state.edgesByTarget.get(edge.target)!.push(edge);

          // Update edge map for O(1) getEdgeById
          state.edgeMap.set(edge.id, edge);
        }

        // Update totals
        state.totalNodes = state.nodes.length;
        state.totalEdges = state.edges.length;
      });
    },

    setLoading: (loading) => {
      set((state) => {
        state.isLoading = loading;
      });
    },

    setError: (error) => {
      set((state) => {
        state.error = error;
        state.isLoading = false;
      });
    },

    clearGraph: () => {
      set((state) => {
        state.nodes = [];
        state.edges = [];
        state.nodeMap = new Map();
        state.adjacencyMap = new Map();
        state.edgesBySource = new Map();
        state.edgesByTarget = new Map();
        state.edgeMap = new Map();
        state.hiddenNodeIds = new Set();
        state.totalNodes = 0;
        state.totalEdges = 0;
        state.nodeTypeCounts = {};
      });
    },

    hideNode: (nodeId) => {
      set((state) => {
        state.hiddenNodeIds.add(nodeId);
      });
    },

    // Computed - Now O(1) using indexed maps
    getNodeById: (id) => {
      return get().nodeMap.get(id);
    },

    getEdgeById: (id) => {
      return get().edgeMap.get(id);
    },

    getNodeDetail: (nodeId) => {
      const { nodeMap, edgesBySource, edgesByTarget } = get();
      const node = nodeMap.get(nodeId);
      if (!node) return null;

      const incoming: NodeDetail['relations']['incoming'] = [];
      const outgoing: NodeDetail['relations']['outgoing'] = [];

      // O(k) using edge indexes where k = number of edges connected to this node
      // Incoming edges: edges where this node is the target
      const incomingEdges = edgesByTarget.get(nodeId) || [];
      for (const edge of incomingEdges) {
        const sourceNode = nodeMap.get(edge.source);
        if (sourceNode) {
          incoming.push({ type: edge.type, node: sourceNode });
        }
      }

      // Outgoing edges: edges where this node is the source
      const outgoingEdges = edgesBySource.get(nodeId) || [];
      for (const edge of outgoingEdges) {
        const targetNode = nodeMap.get(edge.target);
        if (targetNode) {
          outgoing.push({ type: edge.type, node: targetNode });
        }
      }

      return {
        node,
        relations: { incoming, outgoing },
      };
    },
  }))
);

// =============================================================================
// SELECTORS - Use these for optimal re-render performance
// =============================================================================

/** Select nodes array */
export const selectNodes = (state: GraphState) => state.nodes;

/** Select edges array */
export const selectEdges = (state: GraphState) => state.edges;

/** Select loading state */
export const selectIsLoading = (state: GraphState) => state.isLoading;

/** Select error state */
export const selectError = (state: GraphState) => state.error;

/** Select total counts */
export const selectTotals = (state: GraphState) => ({
  nodes: state.totalNodes,
  edges: state.totalEdges,
});

/** Select node type counts */
export const selectNodeTypeCounts = (state: GraphState) => state.nodeTypeCounts;

/** Select hidden node IDs */
export const selectHiddenNodeIds = (state: GraphState) => state.hiddenNodeIds;

/** Select graph data (nodes + edges) */
export const selectGraphData = (state: GraphState) => ({
  nodes: state.nodes,
  edges: state.edges,
});
