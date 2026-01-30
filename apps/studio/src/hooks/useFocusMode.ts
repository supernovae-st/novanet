// src/hooks/useFocusMode.ts
/**
 * useFocusMode Hook
 *
 * Calculates which nodes/edges should be dimmed based on the selected node.
 * IMPORTANT: Uses filtered edges to determine connections, not full graph.
 * This ensures focus mode respects current view filters.
 */

import { useMemo, useCallback } from 'react';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

export interface FocusModeState {
  /** Currently selected node ID */
  selectedId: string | null;
  /** Set of node IDs that are directly connected (1-hop) */
  connectedIds: Set<string>;
  /** Set of node IDs that are 2-hops away */
  secondHopIds: Set<string>;
  /** Check if a node should be dimmed */
  isNodeDimmed: (nodeId: string) => boolean;
  /** Check if an edge should be dimmed */
  isEdgeDimmed: (sourceId: string, targetId: string) => boolean;
  /** Get opacity level for a node (1, 0.6, or 0.15) */
  getNodeOpacity: (nodeId: string) => number;
}

/**
 * Build adjacency map from edges
 * @param edges - The filtered edges to build adjacency from
 * @returns Map of nodeId -> Set of connected nodeIds
 */
function buildAdjacencyMap(edges: GraphEdge[]): Map<string, Set<string>> {
  const adjacencyMap = new Map<string, Set<string>>();

  for (const edge of edges) {
    // Bidirectional adjacency
    if (!adjacencyMap.has(edge.source)) {
      adjacencyMap.set(edge.source, new Set());
    }
    if (!adjacencyMap.has(edge.target)) {
      adjacencyMap.set(edge.target, new Set());
    }
    adjacencyMap.get(edge.source)!.add(edge.target);
    adjacencyMap.get(edge.target)!.add(edge.source);
  }

  return adjacencyMap;
}

/**
 * Hook for focus mode state
 * When a node is selected, calculates which nodes/edges should be dimmed
 *
 * @param filteredEdges - The currently visible edges (from useFilteredGraph)
 */
export function useFocusMode(filteredEdges: GraphEdge[]): FocusModeState {
  const selectedNodeId = useUIStore((state) => state.selectedNodeId);

  // Build adjacency map from FILTERED edges only
  const adjacencyMap = useMemo(() => {
    return buildAdjacencyMap(filteredEdges);
  }, [filteredEdges]);

  const { connectedIds, secondHopIds } = useMemo(() => {
    if (!selectedNodeId) {
      return { connectedIds: new Set<string>(), secondHopIds: new Set<string>() };
    }

    // Get 1-hop connections from filtered adjacency
    const firstHop = adjacencyMap.get(selectedNodeId) || new Set<string>();
    const connected = new Set(firstHop);

    // Get 2-hop connections
    const secondHop = new Set<string>();
    for (const nodeId of firstHop) {
      const neighbors = adjacencyMap.get(nodeId) || new Set<string>();
      for (const neighbor of neighbors) {
        if (neighbor !== selectedNodeId && !connected.has(neighbor)) {
          secondHop.add(neighbor);
        }
      }
    }

    return { connectedIds: connected, secondHopIds: secondHop };
  }, [selectedNodeId, adjacencyMap]);

  const isNodeDimmed = useCallback(
    (nodeId: string): boolean => {
      if (!selectedNodeId) return false;
      if (nodeId === selectedNodeId) return false;
      if (connectedIds.has(nodeId)) return false;
      if (secondHopIds.has(nodeId)) return false;
      return true;
    },
    [selectedNodeId, connectedIds, secondHopIds]
  );

  const isEdgeDimmed = useCallback(
    (sourceId: string, targetId: string): boolean => {
      if (!selectedNodeId) return false;
      // Edge is visible if it connects to selected node or between connected nodes
      const involvesSelected = sourceId === selectedNodeId || targetId === selectedNodeId;
      const bothConnected = connectedIds.has(sourceId) && connectedIds.has(targetId);
      return !involvesSelected && !bothConnected;
    },
    [selectedNodeId, connectedIds]
  );

  const getNodeOpacity = useCallback(
    (nodeId: string): number => {
      if (!selectedNodeId) return 1;
      if (nodeId === selectedNodeId) return 1;
      if (connectedIds.has(nodeId)) return 1;
      if (secondHopIds.has(nodeId)) return 0.6;
      return 0.15;
    },
    [selectedNodeId, connectedIds, secondHopIds]
  );

  return {
    selectedId: selectedNodeId,
    connectedIds,
    secondHopIds,
    isNodeDimmed,
    isEdgeDimmed,
    getNodeOpacity,
  };
}
