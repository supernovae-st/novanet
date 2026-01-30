/**
 * useNodeExpansion Hook
 *
 * Manages node expansion state for Neo4j Browser-style double-click interactions.
 * Fetches neighbors for a given node and merges them into the graph store.
 *
 * Features:
 * - Tracks which nodes have been expanded (prevents re-expansion)
 * - Filters out already-existing nodes before merging
 * - Provides loading state during expansion
 *
 * @example
 * const { expandNode, isExpanding, expandedNodes } = useNodeExpansion();
 * await expandNode('node-element-id', 50);
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { useGraphStore } from '@/stores/graphStore';
import { postJSON } from '@/lib/fetchClient';
import { toast } from '@/lib/toast';
import type { GraphNode, GraphEdge } from '@/types';

/**
 * Response type for node expansion API
 */
interface ExpandNodeResponse {
  nodes: GraphNode[];
  edges: GraphEdge[];
  totalNodes: number;
  totalEdges: number;
  duration: number;
}

/**
 * Fetch node neighbors via API endpoint
 * (Neo4j can only be accessed server-side)
 */
async function fetchNodeNeighborsAPI(nodeId: string, limit: number): Promise<ExpandNodeResponse> {
  return postJSON<ExpandNodeResponse>('/api/graph/expand', { nodeId, limit });
}

/**
 * Result of a node expansion operation
 */
export interface ExpansionResult {
  /** New nodes added to the graph */
  nodes: GraphNode[];
  /** New edges added to the graph */
  edges: GraphEdge[];
  /** Number of nodes actually added (after filtering existing) */
  addedCount: number;
}

/**
 * Return type of the useNodeExpansion hook
 */
export interface UseNodeExpansionReturn {
  /** Expand a node's neighbors and merge into graph */
  expandNode: (nodeId: string, limit?: number) => Promise<ExpansionResult>;
  /** Whether an expansion is currently in progress */
  isExpanding: boolean;
  /** Set of node IDs that have been expanded */
  expandedNodes: Set<string>;
}

export interface UseNodeExpansionOptions {
  /** Show toast notifications for expansion results */
  showToast?: boolean;
}

/**
 * Hook for managing node expansion (double-click to expand neighbors)
 *
 * @param options - Configuration options
 * @returns Object with expandNode function, isExpanding state, and expandedNodes set
 */
export function useNodeExpansion(options: UseNodeExpansionOptions = {}): UseNodeExpansionReturn {
  const { showToast = true } = options;
  const [isExpanding, setIsExpanding] = useState(false);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());

  // Ref to avoid stale closure in async callback
  const expandedNodesRef = useRef(expandedNodes);
  expandedNodesRef.current = expandedNodes;

  // Track nodes currently being expanded (prevents rapid double-click race condition)
  const expandingNodesRef = useRef<Set<string>>(new Set());

  // Get store actions and state (stable references)
  const mergeGraphData = useGraphStore((state) => state.mergeGraphData);
  const totalNodes = useGraphStore((state) => state.totalNodes);

  // Reset expanded nodes when graph is cleared
  useEffect(() => {
    if (totalNodes === 0 && expandedNodes.size > 0) {
      setExpandedNodes(new Set());
    }
  }, [totalNodes, expandedNodes.size]);

  const expandNode = useCallback(
    async (nodeId: string, limit = 50): Promise<ExpansionResult> => {
      // Use ref for fresh value (avoids stale closure)
      // Also check if node is currently being expanded (prevents rapid double-click race)
      if (expandedNodesRef.current.has(nodeId) || expandingNodesRef.current.has(nodeId)) {
        return { nodes: [], edges: [], addedCount: 0 };
      }

      // Mark node as being expanded
      expandingNodesRef.current.add(nodeId);
      setIsExpanding(true);

      try {
        // Fetch neighbors via API (Neo4j runs server-side only)
        const result = await fetchNodeNeighborsAPI(nodeId, limit);

        // Get fresh nodes from store (avoids stale closure)
        const currentNodes = useGraphStore.getState().nodes;
        const existingNodeIds = new Set(currentNodes.map((n) => n.id));

        // Filter out already existing nodes
        const newNodes = result.nodes.filter((n) => !existingNodeIds.has(n.id));
        const newEdges = result.edges;

        // Merge into graph store (store handles edge deduplication)
        if (newNodes.length > 0 || newEdges.length > 0) {
          mergeGraphData(newNodes, newEdges);
        }

        // Mark node as expanded
        setExpandedNodes((prev) => new Set([...prev, nodeId]));

        // Show toast with expansion result
        if (showToast) {
          toast.nodeExpansion(newNodes.length);
        }

        return {
          nodes: newNodes,
          edges: newEdges,
          addedCount: newNodes.length,
        };
      } finally {
        // Remove from expanding set
        expandingNodesRef.current.delete(nodeId);
        setIsExpanding(false);
      }
    },
    [mergeGraphData, showToast]
  );

  return { expandNode, isExpanding, expandedNodes };
}
