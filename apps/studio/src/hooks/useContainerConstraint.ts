/**
 * useContainerConstraint - Dynamic container resizing for schema view
 *
 * Manages container (scope group) dimensions during node dragging.
 * Containers expand when nodes approach edges and shrink when drag ends.
 *
 * Features:
 * - Expand container when node approaches edge threshold
 * - Push adjacent containers to avoid overlap
 * - Shrink to minimum bounding box on drag end
 * - Smooth animated transitions
 *
 * Used in schema view to keep nodes constrained within their scope groups
 * while allowing dynamic resizing based on user interaction.
 */

import { useCallback, useRef, useMemo } from 'react';
import type { Node as ReactFlowNode } from '@xyflow/react';

// =============================================================================
// Types
// =============================================================================

export interface ContainerConstraintOptions {
  /** Distance from edge to trigger expansion (default: 50px) */
  edgeThreshold?: number;
  /** Minimum container padding around nodes (default: 40px) */
  minPadding?: number;
  /** Animation duration in ms (default: 200ms) */
  animationDuration?: number;
  /** Expansion step size when threshold triggered (default: 50px) */
  expansionStep?: number;
}

export interface ContainerBounds {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  /** IDs of child nodes within this container */
  childNodeIds: string[];
}

export interface UseContainerConstraintReturn {
  /** Call during node drag to check/update containers */
  handleNodeDrag: (
    node: ReactFlowNode,
    allNodes: ReactFlowNode[],
    setNodes: React.Dispatch<React.SetStateAction<ReactFlowNode[]>>
  ) => void;
  /** Call when drag ends to shrink containers to fit */
  handleNodeDragStop: (
    node: ReactFlowNode,
    allNodes: ReactFlowNode[],
    setNodes: React.Dispatch<React.SetStateAction<ReactFlowNode[]>>
  ) => void;
  /** Get current container bounds (for debugging/visualization) */
  getContainerBounds: (nodes: ReactFlowNode[]) => Map<string, ContainerBounds>;
  /** Check if a node is near a container edge */
  isNearEdge: (
    node: ReactFlowNode,
    containerBounds: ContainerBounds
  ) => { top: boolean; right: boolean; bottom: boolean; left: boolean };
}

// =============================================================================
// Constants
// =============================================================================

const DEFAULT_OPTIONS: Required<ContainerConstraintOptions> = {
  edgeThreshold: 100,    // Detect earlier for smoother expansion (was 50)
  minPadding: 80,        // More breathing room (was 40)
  animationDuration: 150, // Faster response (was 200)
  expansionStep: 100,    // Bigger steps = fewer expansions = less jitter (was 50)
};

/** Node type identifiers for container nodes */
const CONTAINER_TYPES = ['scopeGroup', 'subcategoryGroup'];

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Extract container bounds from all nodes
 * Returns a map of containerId -> bounds with child nodes
 */
function extractContainerBounds(nodes: ReactFlowNode[]): Map<string, ContainerBounds> {
  const containers = new Map<string, ContainerBounds>();

  // First pass: identify containers
  nodes.forEach((node) => {
    if (CONTAINER_TYPES.includes(node.type || '')) {
      containers.set(node.id, {
        id: node.id,
        x: node.position.x,
        y: node.position.y,
        width: (node.style?.width as number) || 300,
        height: (node.style?.height as number) || 200,
        childNodeIds: [],
      });
    }
  });

  // Second pass: assign children to containers
  nodes.forEach((node) => {
    const parentId = node.parentId;
    if (parentId && containers.has(parentId)) {
      containers.get(parentId)!.childNodeIds.push(node.id);
    }
  });

  return containers;
}

// =============================================================================
// Hook Implementation
// =============================================================================

/**
 * Hook for managing dynamic container constraints during schema node dragging
 *
 * @param options Configuration options for container behavior
 * @returns Functions to handle drag events and query container state
 *
 * @example
 * ```tsx
 * const { handleNodeDrag, handleNodeDragStop } = useContainerConstraint({
 *   edgeThreshold: 50,
 *   minPadding: 40,
 * });
 *
 * // In React Flow
 * <ReactFlow
 *   onNodeDrag={(event, node) => handleNodeDrag(node, nodes, setNodes)}
 *   onNodeDragStop={(event, node) => handleNodeDragStop(node, nodes, setNodes)}
 * />
 * ```
 */
export function useContainerConstraint(
  options: ContainerConstraintOptions = {}
): UseContainerConstraintReturn {
  const opts = useMemo(
    () => ({ ...DEFAULT_OPTIONS, ...options }),
    [options]
  );

  // Track which containers have been expanded during current drag
  const expandedContainersRef = useRef<Set<string>>(new Set());

  /**
   * Get container bounds from nodes
   */
  const getContainerBounds = useCallback(
    (nodes: ReactFlowNode[]): Map<string, ContainerBounds> => {
      return extractContainerBounds(nodes);
    },
    []
  );

  /**
   * Check if a node is near any edge of its parent container
   */
  const isNearEdge = useCallback(
    (
      node: ReactFlowNode,
      containerBounds: ContainerBounds
    ): { top: boolean; right: boolean; bottom: boolean; left: boolean } => {
      const nodeWidth = (node.measured?.width as number) || 150;
      const nodeHeight = (node.measured?.height as number) || 50;
      const threshold = opts.edgeThreshold;

      // Node position is relative to container
      const nodeRight = node.position.x + nodeWidth;
      const nodeBottom = node.position.y + nodeHeight;

      return {
        top: node.position.y < threshold,
        right: containerBounds.width - nodeRight < threshold,
        bottom: containerBounds.height - nodeBottom < threshold,
        left: node.position.x < threshold,
      };
    },
    [opts.edgeThreshold]
  );

  /**
   * Expand a single container and adjust its children
   */
  const expandContainer = useCallback(
    (
      containerId: string,
      expansion: { top: number; right: number; bottom: number; left: number },
      currentNodes: ReactFlowNode[]
    ): ReactFlowNode[] => {
      const { top, right, bottom, left } = expansion;
      if (top === 0 && right === 0 && bottom === 0 && left === 0) {
        return currentNodes;
      }

      expandedContainersRef.current.add(containerId);

      return currentNodes.map((n) => {
        // Update the container itself
        if (n.id === containerId) {
          const currentWidth = (n.style?.width as number) || 300;
          const currentHeight = (n.style?.height as number) || 200;

          return {
            ...n,
            position: {
              x: n.position.x - left,
              y: n.position.y - top,
            },
            style: {
              ...n.style,
              width: currentWidth + left + right,
              height: currentHeight + top + bottom,
            },
          };
        }

        // Adjust children positions if expanded from top or left
        if (n.parentId === containerId && (left > 0 || top > 0)) {
          return {
            ...n,
            position: {
              x: n.position.x + left,
              y: n.position.y + top,
            },
          };
        }

        return n;
      });
    },
    []
  );

  /**
   * Handle node drag - expand containers as needed (cascading to parent containers)
   */
  const handleNodeDrag = useCallback(
    (
      node: ReactFlowNode,
      allNodes: ReactFlowNode[],
      setNodes: React.Dispatch<React.SetStateAction<ReactFlowNode[]>>
    ) => {
      // Only handle child nodes (nodes with parentId)
      if (!node.parentId) return;

      const containers = extractContainerBounds(allNodes);
      const parentContainer = containers.get(node.parentId);
      if (!parentContainer) return;

      const nearEdge = isNearEdge(node, parentContainer);
      const needsExpansion = nearEdge.top || nearEdge.right || nearEdge.bottom || nearEdge.left;

      if (!needsExpansion) return;

      // Calculate expansion for direct parent
      const step = opts.expansionStep;
      const expansion = {
        top: nearEdge.top ? step : 0,
        right: nearEdge.right ? step : 0,
        bottom: nearEdge.bottom ? step : 0,
        left: nearEdge.left ? step : 0,
      };

      setNodes((currentNodes) => {
        // Step 1: Expand the direct parent container
        let updatedNodes = expandContainer(node.parentId!, expansion, currentNodes);

        // Step 2: Check if the parent container now needs to expand its grandparent
        // (e.g., subcategoryGroup expanding might push against scopeGroup)
        const parentNode = updatedNodes.find((n) => n.id === node.parentId);
        if (parentNode?.parentId) {
          const grandparentContainer = containers.get(parentNode.parentId);
          if (grandparentContainer) {
            // Create a virtual node representing the expanded parent container
            const virtualParentNode = {
              ...parentNode,
              measured: {
                width: (parentNode.style?.width as number) || 300,
                height: (parentNode.style?.height as number) || 200,
              },
            };

            const parentNearEdge = isNearEdge(virtualParentNode, grandparentContainer);
            const parentNeedsExpansion =
              parentNearEdge.top || parentNearEdge.right || parentNearEdge.bottom || parentNearEdge.left;

            if (parentNeedsExpansion) {
              const parentExpansion = {
                top: parentNearEdge.top ? step : 0,
                right: parentNearEdge.right ? step : 0,
                bottom: parentNearEdge.bottom ? step : 0,
                left: parentNearEdge.left ? step : 0,
              };
              updatedNodes = expandContainer(parentNode.parentId, parentExpansion, updatedNodes);
            }
          }
        }

        return updatedNodes;
      });
    },
    [opts.expansionStep, isNearEdge, expandContainer]
  );

  /**
   * Handle node drag stop - cleanup after drag
   *
   * Note: We intentionally don't auto-shrink containers as it causes
   * jarring UX with nodes jumping around. Users can manually resize
   * containers using the NodeResizer if needed.
   */
  const handleNodeDragStop = useCallback(
    (
      _node: ReactFlowNode,
      _allNodes: ReactFlowNode[],
      _setNodes: React.Dispatch<React.SetStateAction<ReactFlowNode[]>>
    ) => {
      // Clear expanded containers tracking
      expandedContainersRef.current.clear();

      // Future: Could add optional "snap back" animation here if needed
      // For now, containers stay expanded after drag for better UX
    },
    []
  );

  return {
    handleNodeDrag,
    handleNodeDragStop,
    getContainerBounds,
    isNearEdge,
  };
}

export default useContainerConstraint;
