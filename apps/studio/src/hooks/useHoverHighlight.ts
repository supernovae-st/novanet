// src/hooks/useHoverHighlight.ts
/**
 * useHoverHighlight Hook
 *
 * Calculates which nodes/edges should be highlighted or dimmed based on:
 * - Hovered node (spotlight on node + 1-hop connections)
 * - Hovered edge (spotlight on edge + its source/target nodes)
 *
 * Similar to useFocusMode but lighter opacity dimming.
 * This creates a subtle "spotlight" effect when hovering.
 */

import { useMemo, useCallback } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

export interface HoverHighlightState {
  /** Currently hovered node ID */
  hoveredId: string | null;
  /** Currently hovered edge ID */
  hoveredEdgeId: string | null;
  /** Set of node IDs that are directly connected (1-hop) or edge endpoints */
  connectedIds: Set<string>;
  /** Check if a node should be dimmed due to hover */
  isNodeHoverDimmed: (nodeId: string) => boolean;
  /** Check if an edge should be dimmed due to hover */
  isEdgeHoverDimmed: (sourceId: string, targetId: string, edgeId?: string) => boolean;
  /** Check if an edge is connected to hovered node */
  isEdgeHighlighted: (sourceId: string, targetId: string) => boolean;
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
 * Hook for hover-based connection highlighting
 * When a node or edge is hovered, calculates which nodes/edges should be highlighted or dimmed
 *
 * @param filteredEdges - The currently visible edges (from useFilteredGraph)
 */
export function useHoverHighlight(filteredEdges: GraphEdge[]): HoverHighlightState {
  // Combined selector with useShallow for object comparison (1 subscription instead of 3)
  const { hoveredNodeId, hoveredEdgeId, selectedNodeId } = useUIStore(
    useShallow((state) => ({
      hoveredNodeId: state.hoveredNodeId,
      hoveredEdgeId: state.hoveredEdgeId,
      selectedNodeId: state.selectedNodeId,
    }))
  );

  // Build adjacency map from FILTERED edges only
  const adjacencyMap = useMemo(() => {
    return buildAdjacencyMap(filteredEdges);
  }, [filteredEdges]);

  // Build edge map for quick lookup of edge endpoints
  const edgeMap = useMemo(() => {
    const map = new Map<string, { source: string; target: string }>();
    for (const edge of filteredEdges) {
      map.set(edge.id, { source: edge.source, target: edge.target });
    }
    return map;
  }, [filteredEdges]);

  // Get hovered edge endpoints (for edge hover highlighting)
  const hoveredEdgeEndpoints = useMemo(() => {
    if (!hoveredEdgeId) return null;
    return edgeMap.get(hoveredEdgeId) || null;
  }, [hoveredEdgeId, edgeMap]);

  const connectedIds = useMemo(() => {
    // Don't apply hover highlight when there's a selected node (focus mode takes precedence)
    if (selectedNodeId) {
      return new Set<string>();
    }

    // If edge is hovered, connected nodes are the edge endpoints
    if (hoveredEdgeEndpoints) {
      return new Set([hoveredEdgeEndpoints.source, hoveredEdgeEndpoints.target]);
    }

    // If node is hovered, get 1-hop connections from filtered adjacency
    if (hoveredNodeId) {
      const firstHop = adjacencyMap.get(hoveredNodeId) || new Set<string>();
      return new Set(firstHop);
    }

    return new Set<string>();
  }, [hoveredNodeId, hoveredEdgeEndpoints, selectedNodeId, adjacencyMap]);

  const isNodeHoverDimmed = useCallback(
    (nodeId: string): boolean => {
      // EDGE HOVER takes priority - dim all nodes except edge endpoints
      if (hoveredEdgeEndpoints) {
        return nodeId !== hoveredEdgeEndpoints.source && nodeId !== hoveredEdgeEndpoints.target;
      }

      // Don't apply node hover dimming when there's a selected node (focus mode takes precedence)
      if (selectedNodeId) return false;

      // Node hover mode: dim all nodes except hovered and 1-hop connected
      if (!hoveredNodeId) return false;
      if (nodeId === hoveredNodeId) return false;
      if (connectedIds.has(nodeId)) return false;
      return true;
    },
    [hoveredNodeId, hoveredEdgeEndpoints, selectedNodeId, connectedIds]
  );

  const isEdgeHoverDimmed = useCallback(
    (sourceId: string, targetId: string, edgeId?: string): boolean => {
      // EDGE HOVER takes priority - dim all edges except the hovered edge
      if (hoveredEdgeId) {
        return edgeId !== hoveredEdgeId;
      }

      // Don't apply node hover dimming when there's a selected node (focus mode takes precedence)
      if (selectedNodeId) return false;

      // Node hover mode: dim edges not connected to hovered node
      if (!hoveredNodeId) return false;
      const involvesHovered = sourceId === hoveredNodeId || targetId === hoveredNodeId;
      return !involvesHovered;
    },
    [hoveredNodeId, hoveredEdgeId, selectedNodeId]
  );

  const isEdgeHighlighted = useCallback(
    (sourceId: string, targetId: string): boolean => {
      // Edge hover: hovered edge is highlighted (handled by edge.hovered flag)
      if (hoveredEdgeId) return false;
      // Don't highlight when selected node (focus mode takes precedence)
      if (selectedNodeId) return false;
      // Node hover: edges connected to hovered node are highlighted
      if (!hoveredNodeId) return false;
      return sourceId === hoveredNodeId || targetId === hoveredNodeId;
    },
    [hoveredNodeId, hoveredEdgeId, selectedNodeId]
  );

  return {
    hoveredId: selectedNodeId ? null : hoveredNodeId, // Suppress node hover when selected
    hoveredEdgeId, // Edge hover always available (takes priority)
    connectedIds,
    isNodeHoverDimmed,
    isEdgeHoverDimmed,
    isEdgeHighlighted,
  };
}
