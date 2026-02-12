import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type { Draft } from 'immer';
import type { GraphNode, GraphEdge, GraphData, NodeDetail } from '@/types';

/**
 * Detect if running in test environment (Jest/Vitest)
 * In tests, we need synchronous index building for assertions to work
 */
const IS_TEST_ENV = typeof process !== 'undefined' && process.env?.NODE_ENV === 'test';

/**
 * Polyfill for requestIdleCallback (Safari doesn't support it)
 * Falls back to setTimeout with 1ms delay (yields to event loop)
 * In test environment, executes synchronously for test predictability
 */
const requestIdleCallbackPolyfill: (cb: IdleRequestCallback) => number =
  IS_TEST_ENV
    ? (cb: IdleRequestCallback) => {
        // Synchronous execution in tests - immediately call with generous time budget
        cb({ didTimeout: false, timeRemaining: () => 50 });
        return 0;
      }
    : typeof requestIdleCallback !== 'undefined'
      ? requestIdleCallback
      : (cb: IdleRequestCallback) =>
          window.setTimeout(() => cb({ didTimeout: false, timeRemaining: () => 50 }), 1) as unknown as number;

const cancelIdleCallbackPolyfill: (id: number) => void =
  IS_TEST_ENV
    ? () => {} // No-op in tests since execution is synchronous
    : typeof cancelIdleCallback !== 'undefined'
      ? cancelIdleCallback
      : (id: number) => window.clearTimeout(id);

/** Batch size for chunked processing (tune for ~16ms frames) */
const INDEX_BATCH_SIZE = 2000;

/**
 * Builds graph indexes progressively using requestIdleCallback.
 * Non-blocking: yields to main thread between batches.
 *
 * Priority order (critical indexes first):
 * 1. nodeMap (20%) - Required for getNodeById, node lookups
 * 2. edgeMap (20%) - Required for getEdgeById
 * 3. edgesBySource (20%) - Required for getNodeDetail outgoing
 * 4. edgesByTarget (20%) - Required for getNodeDetail incoming
 * 5. adjacencyMap (20%) - Required for neighbor traversal
 */
function buildIndexesProgressively(
  nodes: GraphNode[],
  edges: GraphEdge[],
  set: (fn: (state: Draft<GraphState>) => void) => void
): void {
  // Pre-allocate maps with expected sizes for better performance
  const nodeMap = new Map<string, GraphNode>();
  const edgeMap = new Map<string, GraphEdge>();
  const edgesBySource = new Map<string, GraphEdge[]>();
  const edgesByTarget = new Map<string, GraphEdge[]>();
  const adjacencyMap = new Map<string, Set<string>>();
  const counts: Record<string, number> = {};

  let currentIdleHandle: number | null = null;

  // Phase 1: Build nodeMap (highest priority - enables getNodeById immediately)
  const buildNodeMap = (startIdx: number, deadline: IdleDeadline): void => {
    let i = startIdx;
    while (i < nodes.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
      const batchEnd = Math.min(i + INDEX_BATCH_SIZE, nodes.length);
      for (; i < batchEnd; i++) {
        const node = nodes[i];
        nodeMap.set(node.id, node);
        counts[node.type] = (counts[node.type] || 0) + 1;
      }
    }

    if (i < nodes.length) {
      // More work to do - schedule next batch
      const progress = Math.round((i / nodes.length) * 20);
      set((state) => {
        state.indexProgress.percent = progress;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildNodeMap(i, dl));
    } else {
      // Phase 1 complete - commit nodeMap and start phase 2
      set((state) => {
        state.nodeMap = nodeMap;
        state.nodeTypeCounts = counts;
        state.indexProgress.phase = 'edgeMap';
        state.indexProgress.percent = 20;
        state.indexProgress.ready.nodeMap = true;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgeMap(0, dl));
    }
  };

  // Phase 2: Build edgeMap (enables getEdgeById)
  const buildEdgeMap = (startIdx: number, deadline: IdleDeadline): void => {
    let i = startIdx;
    while (i < edges.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
      const batchEnd = Math.min(i + INDEX_BATCH_SIZE, edges.length);
      for (; i < batchEnd; i++) {
        edgeMap.set(edges[i].id, edges[i]);
      }
    }

    if (i < edges.length) {
      const progress = 20 + Math.round((i / edges.length) * 20);
      set((state) => {
        state.indexProgress.percent = progress;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgeMap(i, dl));
    } else {
      set((state) => {
        state.edgeMap = edgeMap;
        state.indexProgress.phase = 'edgesBySource';
        state.indexProgress.percent = 40;
        state.indexProgress.ready.edgeMap = true;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgesBySource(0, dl));
    }
  };

  // Phase 3: Build edgesBySource (enables getNodeDetail outgoing)
  const buildEdgesBySource = (startIdx: number, deadline: IdleDeadline): void => {
    let i = startIdx;
    while (i < edges.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
      const batchEnd = Math.min(i + INDEX_BATCH_SIZE, edges.length);
      for (; i < batchEnd; i++) {
        const edge = edges[i];
        const existing = edgesBySource.get(edge.source);
        if (existing) {
          existing.push(edge);
        } else {
          edgesBySource.set(edge.source, [edge]);
        }
      }
    }

    if (i < edges.length) {
      const progress = 40 + Math.round((i / edges.length) * 20);
      set((state) => {
        state.indexProgress.percent = progress;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgesBySource(i, dl));
    } else {
      set((state) => {
        state.edgesBySource = edgesBySource;
        state.indexProgress.phase = 'edgesByTarget';
        state.indexProgress.percent = 60;
        state.indexProgress.ready.edgesBySource = true;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgesByTarget(0, dl));
    }
  };

  // Phase 4: Build edgesByTarget (enables getNodeDetail incoming)
  const buildEdgesByTarget = (startIdx: number, deadline: IdleDeadline): void => {
    let i = startIdx;
    while (i < edges.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
      const batchEnd = Math.min(i + INDEX_BATCH_SIZE, edges.length);
      for (; i < batchEnd; i++) {
        const edge = edges[i];
        const existing = edgesByTarget.get(edge.target);
        if (existing) {
          existing.push(edge);
        } else {
          edgesByTarget.set(edge.target, [edge]);
        }
      }
    }

    if (i < edges.length) {
      const progress = 60 + Math.round((i / edges.length) * 20);
      set((state) => {
        state.indexProgress.percent = progress;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildEdgesByTarget(i, dl));
    } else {
      set((state) => {
        state.edgesByTarget = edgesByTarget;
        state.indexProgress.phase = 'adjacency';
        state.indexProgress.percent = 80;
        state.indexProgress.ready.edgesByTarget = true;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildAdjacencyMap(0, dl));
    }
  };

  // Phase 5: Build adjacencyMap (lowest priority - neighbor traversal)
  const buildAdjacencyMap = (startIdx: number, deadline: IdleDeadline): void => {
    let i = startIdx;
    while (i < edges.length && (deadline.timeRemaining() > 0 || deadline.didTimeout)) {
      const batchEnd = Math.min(i + INDEX_BATCH_SIZE, edges.length);
      for (; i < batchEnd; i++) {
        const edge = edges[i];
        let sourceSet = adjacencyMap.get(edge.source);
        if (!sourceSet) {
          sourceSet = new Set();
          adjacencyMap.set(edge.source, sourceSet);
        }
        sourceSet.add(edge.target);

        let targetSet = adjacencyMap.get(edge.target);
        if (!targetSet) {
          targetSet = new Set();
          adjacencyMap.set(edge.target, targetSet);
        }
        targetSet.add(edge.source);
      }
    }

    if (i < edges.length) {
      const progress = 80 + Math.round((i / edges.length) * 20);
      set((state) => {
        state.indexProgress.percent = progress;
      });
      currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildAdjacencyMap(i, dl));
    } else {
      // All phases complete
      set((state) => {
        state.adjacencyMap = adjacencyMap;
        state.indexProgress.phase = 'complete';
        state.indexProgress.percent = 100;
        state.indexProgress.ready.adjacencyMap = true;
      });
      currentIdleHandle = null;
    }
  };

  // Start the progressive build chain
  currentIdleHandle = requestIdleCallbackPolyfill((dl) => buildNodeMap(0, dl));

  // Note: We don't return a cleanup function here because:
  // 1. Index building should complete even if component unmounts
  // 2. The store is global, so cleanup is not typically needed
  // If needed, the caller can track currentIdleHandle and cancel via cancelIdleCallbackPolyfill
}

/** Index building progress phases */
type IndexPhase = 'idle' | 'nodeMap' | 'edgeMap' | 'edgesBySource' | 'edgesByTarget' | 'adjacency' | 'complete';

/** Index building progress state */
interface IndexProgress {
  phase: IndexPhase;
  /** Overall progress 0-100 */
  percent: number;
  /** Which indexes are ready for use */
  ready: {
    nodeMap: boolean;
    edgeMap: boolean;
    edgesBySource: boolean;
    edgesByTarget: boolean;
    adjacencyMap: boolean;
  };
}

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
  totalArcs: number;
  nodeTypeCounts: Record<string, number>;

  // Index building progress (for non-blocking builds)
  indexProgress: IndexProgress;

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
    totalArcs: 0,
    nodeTypeCounts: {},
    indexProgress: {
      phase: 'idle',
      percent: 0,
      ready: {
        nodeMap: false,
        edgeMap: false,
        edgesBySource: false,
        edgesByTarget: false,
        adjacencyMap: false,
      },
    },

    // Actions
    setGraphData: (data) => {
      // DEBUG: Log when graphStore receives data
      console.log('[graphStore] setGraphData called:', {
        nodeCount: data.nodes?.length ?? 0,
        edgeCount: data.edges?.length ?? 0,
      });

      // Phase 1: Immediately set raw data (renders can start)
      set((state) => {
        state.nodes = data.nodes;
        state.edges = data.edges;
        state.totalNodes = data.nodes.length;
        state.totalArcs = data.edges.length;
        state.lastFetchTime = Date.now();
        state.error = null;
        state.indexProgress = {
          phase: 'nodeMap',
          percent: 0,
          ready: {
            nodeMap: false,
            edgeMap: false,
            edgesBySource: false,
            edgesByTarget: false,
            adjacencyMap: false,
          },
        };
      });

      // Build indexes progressively using requestIdleCallback
      // Priority order: nodeMap (most critical) > edgeMap > edgesBySource/Target > adjacency
      buildIndexesProgressively(data.nodes, data.edges, set);
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
        state.totalArcs = state.edges.length;
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
        state.totalArcs = 0;
        state.nodeTypeCounts = {};
        state.indexProgress = {
          phase: 'idle',
          percent: 0,
          ready: {
            nodeMap: false,
            edgeMap: false,
            edgesBySource: false,
            edgesByTarget: false,
            adjacencyMap: false,
          },
        };
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

