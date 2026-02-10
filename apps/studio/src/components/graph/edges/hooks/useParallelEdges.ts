'use client';

/**
 * useParallelEdges - Group edges between same node pairs
 *
 * Detects multiple edges connecting the same two nodes and groups them
 * for bundled rendering. Handles both directions (A→B and B→A).
 *
 * Strategy:
 * - 2-3 edges: Render individually with curved fan-out
 * - 4+ edges: Collapse into bundle with count badge, expand on hover
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */

import { useMemo } from 'react';
import type { Edge } from '@xyflow/react';

// =============================================================================
// Types
// =============================================================================

export interface ParallelEdgeGroup {
  /** Canonical key: 'nodeA::nodeB' (sorted alphabetically) */
  key: string;
  /** All edges in this group */
  edges: Edge[];
  /** Number of edges */
  count: number;
  /** True if should be rendered as collapsed bundle (count >= 4) */
  isBundled: boolean;
  /** Source node ID (from canonical key) */
  sourceNode: string;
  /** Target node ID (from canonical key) */
  targetNode: string;
}

export interface UseParallelEdgesResult {
  /** Map of canonical key to edge group */
  groups: Map<string, ParallelEdgeGroup>;
  /** Edges that are not part of any parallel group (single edges) */
  singleEdges: Edge[];
  /** Edges that are part of parallel groups (2-3 edges, need fan-out) */
  fanOutEdges: ParallelEdgeGroup[];
  /** Edges that should be bundled (4+ edges) */
  bundledEdges: ParallelEdgeGroup[];
  /** Total number of groups */
  totalGroups: number;
}

// =============================================================================
// Constants
// =============================================================================

/** Threshold for bundling (4+ edges between same nodes) */
export const BUNDLE_THRESHOLD = 4;

/** Maximum fan-out offset in pixels */
export const MAX_FAN_OUT_OFFSET = 75;

/** Base offset per edge in fan-out mode */
export const FAN_OUT_OFFSET_PER_EDGE = 25;

// =============================================================================
// Hook
// =============================================================================

/**
 * Group edges by their source-target node pairs
 *
 * @param edges - Array of React Flow edges
 * @returns Grouped edges with bundling information
 */
export function useParallelEdges(edges: Edge[]): UseParallelEdgesResult {
  return useMemo(() => {
    const groups = new Map<string, Edge[]>();

    // Group edges by canonical key (sorted node IDs)
    for (const edge of edges) {
      // Create canonical key by sorting node IDs
      // This ensures A→B and B→A are grouped together
      const [first, second] = [edge.source, edge.target].sort();
      const key = `${first}::${second}`;

      if (!groups.has(key)) {
        groups.set(key, []);
      }
      groups.get(key)!.push(edge);
    }

    // Transform to ParallelEdgeGroup format
    const parallelGroups = new Map<string, ParallelEdgeGroup>();
    const singleEdges: Edge[] = [];
    const fanOutEdges: ParallelEdgeGroup[] = [];
    const bundledEdges: ParallelEdgeGroup[] = [];

    for (const [key, groupEdges] of groups) {
      const [sourceNode, targetNode] = key.split('::');
      const count = groupEdges.length;

      if (count === 1) {
        // Single edge - no grouping needed
        singleEdges.push(groupEdges[0]);
      } else {
        const group: ParallelEdgeGroup = {
          key,
          edges: groupEdges,
          count,
          isBundled: count >= BUNDLE_THRESHOLD,
          sourceNode,
          targetNode,
        };

        parallelGroups.set(key, group);

        if (group.isBundled) {
          bundledEdges.push(group);
        } else {
          fanOutEdges.push(group);
        }
      }
    }

    return {
      groups: parallelGroups,
      singleEdges,
      fanOutEdges,
      bundledEdges,
      totalGroups: parallelGroups.size,
    };
  }, [edges]);
}

/**
 * Get the index of an edge within its parallel group
 *
 * @param edge - The edge to find
 * @param groups - The parallel edge groups map
 * @returns Object with index and total count, or null if not in a group
 */
export function getEdgeIndexInGroup(
  edge: Edge,
  groups: Map<string, ParallelEdgeGroup>
): { index: number; total: number } | null {
  const [first, second] = [edge.source, edge.target].sort();
  const key = `${first}::${second}`;

  const group = groups.get(key);
  if (!group || group.count === 1) {
    return null;
  }

  const index = group.edges.findIndex((e) => e.id === edge.id);
  if (index === -1) {
    return null;
  }

  return { index, total: group.count };
}
