// src/hooks/useFocusMode.ts
/**
 * useFocusMode Hook
 *
 * Calculates which nodes/edges should be dimmed based on selection.
 * Supports both NODE selection and EDGE selection/hover.
 *
 * IMPORTANT: Uses filtered edges to determine connections, not full graph.
 * This ensures focus mode respects current view filters.
 *
 * v11.6.3: Added edge selection/hover support for consistent focus behavior.
 */

import { useMemo, useCallback } from 'react';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

export interface FocusModeState {
  /** Currently selected node ID */
  selectedId: string | null;
  /** Currently selected edge ID */
  selectedEdgeId: string | null;
  /** Currently hovered edge ID */
  hoveredEdgeId: string | null;
  /** Set of node IDs that are directly connected (1-hop) */
  connectedIds: Set<string>;
  /** Set of node IDs that are 2-hops away */
  secondHopIds: Set<string>;
  /** Check if a node should be dimmed (considers both node and edge selection) */
  isNodeDimmed: (nodeId: string) => boolean;
  /** Check if an edge should be dimmed (considers both node and edge selection) */
  isEdgeDimmed: (sourceId: string, targetId: string) => boolean;
  /** Check if an edge should be dimmed by edge ID */
  isEdgeDimmedById: (edgeId: string) => boolean;
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
 * When a node OR edge is selected/hovered, calculates which elements should be dimmed
 *
 * @param filteredEdges - The currently visible edges (from useFilteredGraph)
 */
export function useFocusMode(filteredEdges: GraphEdge[]): FocusModeState {
  const selectedNodeId = useUIStore((state) => state.selectedNodeId);
  const selectedEdgeId = useUIStore((state) => state.selectedEdgeId);
  const hoveredEdgeId = useUIStore((state) => state.hoveredEdgeId);

  // Build adjacency map from FILTERED edges only
  const adjacencyMap = useMemo(() => {
    return buildAdjacencyMap(filteredEdges);
  }, [filteredEdges]);

  // Build edge lookup map for quick access to edge source/target
  const edgeMap = useMemo(() => {
    const map = new Map<string, { source: string; target: string }>();
    for (const edge of filteredEdges) {
      map.set(edge.id, { source: edge.source, target: edge.target });
    }
    return map;
  }, [filteredEdges]);

  // Get the "active" edge (selected takes priority over hovered)
  const activeEdgeId = selectedEdgeId || hoveredEdgeId;
  const activeEdge = activeEdgeId ? edgeMap.get(activeEdgeId) : null;

  // Nodes connected to the active edge
  const edgeConnectedNodes = useMemo(() => {
    if (!activeEdge) return new Set<string>();
    return new Set([activeEdge.source, activeEdge.target]);
  }, [activeEdge]);

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
      // Edge focus mode: dim nodes not connected to the active edge
      if (activeEdge && !selectedNodeId) {
        return !edgeConnectedNodes.has(nodeId);
      }
      // Node focus mode (original behavior)
      if (!selectedNodeId) return false;
      if (nodeId === selectedNodeId) return false;
      if (connectedIds.has(nodeId)) return false;
      if (secondHopIds.has(nodeId)) return false;
      return true;
    },
    [selectedNodeId, connectedIds, secondHopIds, activeEdge, edgeConnectedNodes]
  );

  const isEdgeDimmed = useCallback(
    (sourceId: string, targetId: string): boolean => {
      // Edge focus mode: dim edges that are not the active edge
      if (activeEdge && !selectedNodeId) {
        const isActiveEdge =
          (sourceId === activeEdge.source && targetId === activeEdge.target) ||
          (sourceId === activeEdge.target && targetId === activeEdge.source);
        return !isActiveEdge;
      }
      // Node focus mode (original behavior)
      if (!selectedNodeId) return false;
      // Edge is visible if it connects to selected node or between connected nodes
      const involvesSelected = sourceId === selectedNodeId || targetId === selectedNodeId;
      const bothConnected = connectedIds.has(sourceId) && connectedIds.has(targetId);
      return !involvesSelected && !bothConnected;
    },
    [selectedNodeId, connectedIds, activeEdge]
  );

  const isEdgeDimmedById = useCallback(
    (edgeId: string): boolean => {
      // Edge focus mode: dim edges that are not the active edge
      if (activeEdgeId && !selectedNodeId) {
        return edgeId !== activeEdgeId;
      }
      // For node focus mode, need to look up edge source/target
      const edge = edgeMap.get(edgeId);
      if (!edge) return false;
      return isEdgeDimmed(edge.source, edge.target);
    },
    [activeEdgeId, selectedNodeId, edgeMap, isEdgeDimmed]
  );

  const getNodeOpacity = useCallback(
    (nodeId: string): number => {
      // Edge focus mode: full opacity for connected nodes, dimmed for others
      if (activeEdge && !selectedNodeId) {
        return edgeConnectedNodes.has(nodeId) ? 1 : 0.15;
      }
      // Node focus mode (original behavior)
      if (!selectedNodeId) return 1;
      if (nodeId === selectedNodeId) return 1;
      if (connectedIds.has(nodeId)) return 1;
      if (secondHopIds.has(nodeId)) return 0.6;
      return 0.15;
    },
    [selectedNodeId, connectedIds, secondHopIds, activeEdge, edgeConnectedNodes]
  );

  return {
    selectedId: selectedNodeId,
    selectedEdgeId,
    hoveredEdgeId,
    connectedIds,
    secondHopIds,
    isNodeDimmed,
    isEdgeDimmed,
    isEdgeDimmedById,
    getNodeOpacity,
  };
}
