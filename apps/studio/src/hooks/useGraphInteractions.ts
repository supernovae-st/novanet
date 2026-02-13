'use client';

/**
 * useGraphInteractions - Unified z-index and interaction management for React Flow
 *
 * Handles:
 * - Z-index layering: clicked/hovered nodes come to front
 * - Edge interactions: source/target nodes highlighted on edge click
 * - Works for both data mode and schema mode
 *
 * Z-Index Strategy:
 * - Containers (realm/layer): 0 (background)
 * - Regular nodes: 1000 (middle layer)
 * - Hovered node: 9000 (front)
 * - Selected node: 10000 (always on top)
 */

import { useCallback } from 'react';
import type { Node as ReactFlowNode, Edge as ReactFlowEdge } from '@xyflow/react';

// =============================================================================
// Z-INDEX CONSTANTS
// =============================================================================

export const Z_INDEX = {
  /** Realm containers - layered by realm (Shared < Org) - v0.12.0: 2 realms */
  REALM_SHARED: 20,
  REALM_ORG: 30,
  /** Layer containers - slightly above their parent realm */
  LAYER_SHARED: 25,
  LAYER_ORG: 35,
  /** Regular nodes base level (always above containers) */
  BASE: 1000,
  /** Nodes connected to hovered edge */
  EDGE_CONNECTED: 8000,
  /** Hovered node (just below selected) */
  HOVERED: 9000,
  /** Selected/clicked node (always on top) */
  SELECTED: 10000,
} as const;

// =============================================================================
// TYPES
// =============================================================================

/**
 * Generic options interface that accepts any node type extending ReactFlowNode
 */
export interface UseGraphInteractionsOptions<T extends ReactFlowNode = ReactFlowNode> {
  /** Function to update nodes state */
  setNodes: React.Dispatch<React.SetStateAction<T[]>>;
}

export interface UseGraphInteractionsReturn {
  /** Bring a node to front (selected z-index) */
  bringToFront: (nodeId: string) => void;
  /** Set node to hover z-index (slightly below selected) */
  setHoverZIndex: (nodeId: string) => void;
  /** Reset node to base z-index */
  resetZIndex: (nodeId: string) => void;
  /** Bring edge's source and target nodes to front */
  bringEdgeNodesToFront: (edge: ReactFlowEdge) => void;
  /** Reset all nodes to their base z-index */
  resetAllZIndex: () => void;
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/**
 * Get base z-index for a node based on its type and realm
 *
 * Container hierarchy (back to front) - v0.12.0: 2 realms:
 * - Shared realm (20) < Org realm (30)
 * - Layers slightly above their parent realm
 * - Regular nodes always on top (1000+)
 */
function getBaseZIndex(node: ReactFlowNode): number {
  const id = node.id;

  // Realm containers: realm-{Realm}
  if (id.startsWith('realm-')) {
    const realm = id.replace('realm-', '');
    if (realm === 'shared') return Z_INDEX.REALM_SHARED;
    if (realm === 'org') return Z_INDEX.REALM_ORG;
    return Z_INDEX.REALM_SHARED; // fallback
  }

  // Layer containers: layer-{Realm}-{LayerName}
  if (id.startsWith('layer-')) {
    const parts = id.replace('layer-', '').split('-');
    const realm = parts[0];
    if (realm === 'shared') return Z_INDEX.LAYER_SHARED;
    if (realm === 'org') return Z_INDEX.LAYER_ORG;
    return Z_INDEX.LAYER_SHARED; // fallback
  }

  // Regular nodes - always above containers
  return Z_INDEX.BASE;
}

// =============================================================================
// HOOK
// =============================================================================

export function useGraphInteractions<T extends ReactFlowNode = ReactFlowNode>({
  setNodes,
}: UseGraphInteractionsOptions<T>): UseGraphInteractionsReturn {
  /**
   * Bring a node to front (selected z-index)
   * IMPORTANT: Containers (realm, layer) should NOT be brought to front
   * They must stay behind regular nodes to maintain the layering hierarchy
   */
  const bringToFront = useCallback(
    (nodeId: string) => {
      // Skip z-index change for containers - they should stay in their layer
      const isContainer = nodeId.startsWith('realm-') || nodeId.startsWith('layer-');
      if (isContainer) return;

      setNodes((nodes) =>
        nodes.map((node) => ({
          ...node,
          zIndex: node.id === nodeId ? Z_INDEX.SELECTED : getBaseZIndex(node),
        }))
      );
    },
    [setNodes]
  );

  /**
   * Set node to hover z-index (slightly below selected)
   * IMPORTANT: Containers (realm, layer) should NOT be brought to front on hover
   * They must stay behind regular nodes to maintain the layering hierarchy
   */
  const setHoverZIndex = useCallback(
    (nodeId: string) => {
      // Skip z-index change for containers - they should stay in their layer
      const isContainer = nodeId.startsWith('realm-') || nodeId.startsWith('layer-');
      if (isContainer) return;

      setNodes((nodes) =>
        nodes.map((node) => {
          if (node.id === nodeId) {
            // Don't lower z-index if already selected (higher)
            const currentZ = node.zIndex ?? getBaseZIndex(node);
            return {
              ...node,
              zIndex: Math.max(currentZ, Z_INDEX.HOVERED),
            };
          }
          return node;
        })
      );
    },
    [setNodes]
  );

  /**
   * Reset a specific node to its base z-index
   */
  const resetZIndex = useCallback(
    (nodeId: string) => {
      setNodes((nodes) =>
        nodes.map((node) =>
          node.id === nodeId
            ? { ...node, zIndex: getBaseZIndex(node) }
            : node
        )
      );
    },
    [setNodes]
  );

  /**
   * Bring edge's source and target nodes to front
   */
  const bringEdgeNodesToFront = useCallback(
    (edge: ReactFlowEdge) => {
      setNodes((nodes) =>
        nodes.map((node) => {
          if (node.id === edge.source || node.id === edge.target) {
            return { ...node, zIndex: Z_INDEX.EDGE_CONNECTED };
          }
          return { ...node, zIndex: getBaseZIndex(node) };
        })
      );
    },
    [setNodes]
  );

  /**
   * Reset all nodes to their base z-index
   */
  const resetAllZIndex = useCallback(() => {
    setNodes((nodes) =>
      nodes.map((node) => ({
        ...node,
        zIndex: getBaseZIndex(node),
      }))
    );
  }, [setNodes]);

  return {
    bringToFront,
    setHoverZIndex,
    resetZIndex,
    bringEdgeNodesToFront,
    resetAllZIndex,
  };
}
