/**
 * useMagneticForce - Magnetic repulsion effect during node dragging
 *
 * Creates a dynamic visual effect where nearby nodes are pushed away
 * when another node is being dragged toward them.
 *
 * Features:
 * - Repulsion decreases with distance (inverse relationship)
 * - Maximum distance threshold for performance
 * - Tracks displaced nodes for visual feedback
 * - Respects animation settings
 *
 * Physics model:
 * - Repulsion = strength * (1 - distance/maxDistance)
 * - Direction: normalized vector from dragged to target
 */

import { useCallback, useRef } from 'react';

export interface MagneticForceOptions {
  /** Repulsion strength in pixels (default: 200) */
  strength?: number;
  /** Maximum repulsion distance in pixels (default: 300) */
  maxDistance?: number;
  /** Enable/disable magnetic force (default: true) */
  enabled?: boolean;
}

export interface NodePosition {
  id: string;
  x: number;
  y: number;
}

export interface UseMagneticForceReturn {
  /** Calculate displaced positions for all nodes */
  getDisplacedPositions: (
    nodes: NodePosition[],
    draggedNodeId: string,
    dragPosition: { x: number; y: number }
  ) => Map<string, { x: number; y: number }>;
  /** Check if a node is being displaced */
  isDisplaced: (nodeId: string) => boolean;
  /** Reset all displacements (call when drag ends) */
  reset: () => void;
  /** Get currently displaced node IDs */
  getDisplacedNodeIds: () => Set<string>;
}

const DEFAULT_OPTIONS: Required<MagneticForceOptions> = {
  strength: 200,
  maxDistance: 300,
  enabled: true,
};

/**
 * Hook for magnetic repulsion effect during node dragging
 *
 * @param options Configuration options for the magnetic force
 * @returns Functions to calculate displaced positions and track state
 *
 * @example
 * ```tsx
 * const { getDisplacedPositions, reset } = useMagneticForce({ strength: 200 });
 *
 * const onNodeDrag = (event, node, nodes) => {
 *   const displaced = getDisplacedPositions(nodes, node.id, node.position);
 *   // Apply displaced positions to nodes
 * };
 *
 * const onNodeDragStop = () => reset();
 * ```
 */
export function useMagneticForce(options: MagneticForceOptions = {}): UseMagneticForceReturn {
  const opts = { ...DEFAULT_OPTIONS, ...options };
  const displacedNodesRef = useRef<Set<string>>(new Set());

  const getDisplacedPositions = useCallback(
    (
      nodes: NodePosition[],
      draggedNodeId: string,
      dragPosition: { x: number; y: number }
    ): Map<string, { x: number; y: number }> => {
      const result = new Map<string, { x: number; y: number }>();
      const newDisplacedNodes = new Set<string>();

      // If disabled, just return original positions
      if (!opts.enabled) {
        nodes.forEach((n) => result.set(n.id, { x: n.x, y: n.y }));
        return result;
      }

      nodes.forEach((node) => {
        // Dragged node stays at drag position
        if (node.id === draggedNodeId) {
          result.set(node.id, dragPosition);
          return;
        }

        // Calculate distance from drag position to node
        const dx = node.x - dragPosition.x;
        const dy = node.y - dragPosition.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        // Apply repulsion if within maxDistance and not at same position
        if (distance < opts.maxDistance && distance > 0) {
          // Repulsion factor: stronger when closer (linear falloff)
          const repulsionFactor = 1 - distance / opts.maxDistance;
          const repulsionMagnitude = opts.strength * repulsionFactor;

          // Normalize direction vector
          const nx = dx / distance;
          const ny = dy / distance;

          // Apply repulsion: push node away from drag position
          result.set(node.id, {
            x: node.x + nx * repulsionMagnitude,
            y: node.y + ny * repulsionMagnitude,
          });
          newDisplacedNodes.add(node.id);
        } else {
          // Node is outside influence range - keep original position
          result.set(node.id, { x: node.x, y: node.y });
        }
      });

      // Update displaced nodes ref
      displacedNodesRef.current = newDisplacedNodes;

      return result;
    },
    [opts.enabled, opts.maxDistance, opts.strength]
  );

  const isDisplaced = useCallback((nodeId: string): boolean => {
    return displacedNodesRef.current.has(nodeId);
  }, []);

  const reset = useCallback(() => {
    displacedNodesRef.current.clear();
  }, []);

  const getDisplacedNodeIds = useCallback((): Set<string> => {
    return new Set(displacedNodesRef.current);
  }, []);

  return {
    getDisplacedPositions,
    isDisplaced,
    reset,
    getDisplacedNodeIds,
  };
}

export default useMagneticForce;
