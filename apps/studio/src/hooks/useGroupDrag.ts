/**
 * useGroupDrag - Group-aware dragging for connected nodes
 *
 * When dragging a node, connected nodes follow with dampened movement.
 * Creates a cohesive "magnetic" group movement effect.
 *
 * Features:
 * - Connected nodes follow the dragged node
 * - Movement dampened by hop distance (1-hop = more movement)
 * - Configurable max hops and follow strength
 * - Bidirectional edge support
 *
 * Physics model:
 * - 1-hop neighbors: move by followStrength * delta
 * - 2-hop neighbors: move by (followStrength / 2) * delta
 * - Beyond maxHops: no movement
 */

import { useCallback, useMemo } from 'react';

export interface GroupDragOptions {
  /** How much connected nodes follow (0-1, default: 0.3) */
  followStrength?: number;
  /** Maximum hops to include (default: 1) */
  maxHops?: number;
  /** Enable/disable group drag (default: true) */
  enabled?: boolean;
}

export interface NodePosition {
  id: string;
  x: number;
  y: number;
}

export interface EdgeConnection {
  source: string;
  target: string;
}

export interface UseGroupDragReturn {
  /** Calculate new positions for all nodes when dragging */
  getGroupPositions: (
    nodes: NodePosition[],
    edges: EdgeConnection[],
    draggedNodeId: string,
    dragDelta: { dx: number; dy: number }
  ) => Map<string, { x: number; y: number }>;
  /** Get IDs of nodes connected within maxHops */
  getConnectedNodeIds: (nodeId: string, edges: EdgeConnection[]) => Set<string>;
}

const DEFAULT_OPTIONS: Required<GroupDragOptions> = {
  followStrength: 0.3,
  maxHops: 1,
  enabled: true,
};

/**
 * Build an adjacency map from edges (bidirectional)
 */
function buildAdjacencyMap(edges: EdgeConnection[]): Map<string, Set<string>> {
  const adj = new Map<string, Set<string>>();

  edges.forEach((edge) => {
    // Initialize sets if needed
    if (!adj.has(edge.source)) adj.set(edge.source, new Set());
    if (!adj.has(edge.target)) adj.set(edge.target, new Set());

    // Add bidirectional connections
    adj.get(edge.source)!.add(edge.target);
    adj.get(edge.target)!.add(edge.source);
  });

  return adj;
}

/**
 * Get all nodes within maxHops of the given node using BFS
 * Returns a Map of nodeId -> hopDistance
 */
function getNodesWithinHops(
  nodeId: string,
  edges: EdgeConnection[],
  maxHops: number
): Map<string, number> {
  const adjacency = buildAdjacencyMap(edges);
  const result = new Map<string, number>(); // nodeId -> hop distance
  const visited = new Set<string>([nodeId]);
  let frontier = [nodeId];

  for (let hop = 1; hop <= maxHops; hop++) {
    const nextFrontier: string[] = [];

    for (const current of frontier) {
      const neighbors = adjacency.get(current) || new Set();

      for (const neighbor of neighbors) {
        if (!visited.has(neighbor)) {
          visited.add(neighbor);
          result.set(neighbor, hop);
          nextFrontier.push(neighbor);
        }
      }
    }

    frontier = nextFrontier;
  }

  return result;
}

/**
 * Hook for group-aware dragging of connected nodes
 *
 * @param options Configuration options for group drag behavior
 * @returns Functions to calculate group positions and get connected nodes
 *
 * @example
 * ```tsx
 * const { getGroupPositions } = useGroupDrag({ followStrength: 0.3, maxHops: 1 });
 *
 * const onNodeDrag = (event, node) => {
 *   const delta = { dx: event.movementX, dy: event.movementY };
 *   const newPositions = getGroupPositions(nodes, edges, node.id, delta);
 *   // Apply new positions to nodes
 * };
 * ```
 */
export function useGroupDrag(options: GroupDragOptions = {}): UseGroupDragReturn {
  const opts = useMemo(
    () => ({ ...DEFAULT_OPTIONS, ...options }),
    [options]
  );

  const getGroupPositions = useCallback(
    (
      nodes: NodePosition[],
      edges: EdgeConnection[],
      draggedNodeId: string,
      dragDelta: { dx: number; dy: number }
    ): Map<string, { x: number; y: number }> => {
      const result = new Map<string, { x: number; y: number }>();

      // If disabled, only move the dragged node
      if (!opts.enabled) {
        nodes.forEach((node) => {
          if (node.id === draggedNodeId) {
            result.set(node.id, {
              x: node.x + dragDelta.dx,
              y: node.y + dragDelta.dy,
            });
          } else {
            result.set(node.id, { x: node.x, y: node.y });
          }
        });
        return result;
      }

      // Get connected nodes with their hop distances
      const connectedNodes = getNodesWithinHops(draggedNodeId, edges, opts.maxHops);

      nodes.forEach((node) => {
        if (node.id === draggedNodeId) {
          // Dragged node moves by full delta
          result.set(node.id, {
            x: node.x + dragDelta.dx,
            y: node.y + dragDelta.dy,
          });
        } else if (connectedNodes.has(node.id)) {
          // Connected nodes move with dampening based on hop distance
          const hopDistance = connectedNodes.get(node.id)!;
          const dampening = opts.followStrength / hopDistance;

          result.set(node.id, {
            x: node.x + dragDelta.dx * dampening,
            y: node.y + dragDelta.dy * dampening,
          });
        } else {
          // Unconnected nodes stay in place
          result.set(node.id, { x: node.x, y: node.y });
        }
      });

      return result;
    },
    [opts.enabled, opts.followStrength, opts.maxHops]
  );

  const getConnectedNodeIds = useCallback(
    (nodeId: string, edges: EdgeConnection[]): Set<string> => {
      const connected = getNodesWithinHops(nodeId, edges, opts.maxHops);
      return new Set(connected.keys());
    },
    [opts.maxHops]
  );

  return {
    getGroupPositions,
    getConnectedNodeIds,
  };
}

export default useGroupDrag;
