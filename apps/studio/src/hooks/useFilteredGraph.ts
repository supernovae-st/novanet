/**
 * useFilteredGraph Hook
 *
 * Computed selector that combines graphStore nodes with filterStore settings.
 * Provides filtered nodes and edges based on enabled node types and locale.
 *
 * Performance optimizations (v12.1):
 * - Single-pass filtering with early exit on empty results
 * - Combined aggregation (realm/layer counts) during filter pass
 * - Memoized filter predicates to avoid closure recreation
 * - Early exit when no filters are active (identity return)
 *
 * This is critical for 19k nodes.
 *
 * @example
 * const { nodes, edges, isFiltered } = useFilteredGraph();
 */

import { useMemo, useCallback } from 'react';
import { useGraphStore } from '@/stores/graphStore';
import { useFilterStore } from '@/stores/filterStore';
import { useQueryStore } from '@/stores/queryStore';
import { ALL_NODE_TYPES } from '@/config/nodeTypes';
import { NODE_REALMS } from '@novanet/core/types';
import { NODE_LAYERS, type Layer } from '@novanet/core/graph';
import type { GraphNode, GraphEdge, NodeType } from '@/types';

/** Realm counts for schema mode breakdown (v11.2: 2 realms) */
export interface RealmCounts {
  shared: number;
  org: number;
}

/** Layer counts for schema mode breakdown */
export type LayerCounts = Record<Layer, number>;

export interface FilteredGraphResult {
  /** Filtered nodes based on enabled types and locale */
  nodes: GraphNode[];
  /** Filtered edges (only those connecting visible nodes) */
  edges: GraphEdge[];
  /** Total number of nodes before filtering */
  totalNodes: number;
  /** Total number of edges before filtering */
  totalArcs: number;
  /** Whether any filters are active */
  isFiltered: boolean;
  /** Count of visible nodes */
  visibleNodeCount: number;
  /** Count of visible edges */
  visibleEdgeCount: number;
  /** Number of distinct relation types (for schema mode stats) */
  distinctRelationTypes: number;
  /** Node counts by realm (for schema mode breakdown) */
  realmCounts: RealmCounts;
  /** Node counts by layer (for schema mode breakdown) */
  layerCounts: LayerCounts;
  /** Whether currently in meta or overlay mode (shows schema nodes) */
  isMetaMode: boolean;
}

/** Internal result from single-pass filter with aggregated counts */
interface FilterPassResult {
  nodes: GraphNode[];
  visibleNodeIds: Set<string>;
  realmCounts: RealmCounts;
  layerCounts: LayerCounts;
}

/**
 * Compute connection count for each node from edges.
 * Used for smart display limit (show most connected nodes first).
 * @returns Map of nodeId -> connection count
 */
function computeConnectionCounts(edges: GraphEdge[]): Map<string, number> {
  const counts = new Map<string, number>();
  for (const edge of edges) {
    counts.set(edge.source, (counts.get(edge.source) ?? 0) + 1);
    counts.set(edge.target, (counts.get(edge.target) ?? 0) + 1);
  }
  return counts;
}

/** Create empty layer counts object */
function createEmptyLayerCounts(): LayerCounts {
  return {
    // Shared realm (4) — v11.4: includes config
    config: 0, locale: 0, geography: 0, knowledge: 0,
    // Org realm (6) — v11.4: seo/geo removed
    foundation: 0, structure: 0, semantic: 0,
    instruction: 0, output: 0,
  };
}

/** Create empty realm counts object */
function createEmptyRealmCounts(): RealmCounts {
  return { shared: 0, org: 0 };
}

// Meta node types from the NovaNet meta-graph schema
// These are returned by meta-* views (meta-complete, meta-realm, etc.)
// Includes 'Meta' as a catch-all for nodes that only have the :Meta label
const META_NODE_TYPES = new Set([
  'Meta', 'Realm', 'Layer', 'Kind', 'Trait', 'ArcKind', 'ArcFamily', 'ArcScope', 'ArcCardinality',
]);

export function useFilteredGraph(): FilteredGraphResult {
  // Get raw data from stores
  const allNodes = useGraphStore((state) => state.nodes);
  const allEdges = useGraphStore((state) => state.edges);
  const hiddenNodeIds = useGraphStore((state) => state.hiddenNodeIds);
  const enabledNodeTypes = useFilterStore((state) => state.enabledNodeTypes);
  const selectedLocale = useFilterStore((state) => state.selectedLocale);
  const searchQuery = useFilterStore((state) => state.searchQuery);
  const displayLimit = useFilterStore((state) => state.displayLimit);

  // v12.1: Query-First - when a query is active, bypass type filtering
  // The query defines what's displayed, not the enabledNodeTypes filter
  const currentQuery = useQueryStore((state) => state.currentQuery);
  const hasActiveQuery = currentQuery !== null && currentQuery.trim().length > 0;

  // Detect meta mode based on node types (Realm, Layer, Kind, etc.)
  // v12.1: Changed from ID prefix detection to type-based detection
  // Meta mode is true if ALL nodes are meta types (Realm, Layer, Kind, etc.)
  const isMetaMode = useMemo(() => {
    if (allNodes.length === 0) return false;
    // Check if all nodes are meta types
    for (const node of allNodes) {
      if (!META_NODE_TYPES.has(node.type)) {
        return false; // Early exit: if any non-meta node, not meta mode
      }
    }
    return true;
  }, [allNodes]);

  // Memoize normalized search query to avoid repeated toLowerCase/trim
  const normalizedSearchQuery = useMemo(() => {
    if (!searchQuery || !searchQuery.trim()) return null;
    return searchQuery.toLowerCase().trim();
  }, [searchQuery]);

  // Memoize filter predicate functions to avoid closure recreation
  const matchesSearch = useCallback((node: GraphNode, query: string): boolean => {
    return (
      node.key?.toLowerCase().includes(query) ||
      node.displayName?.toLowerCase().includes(query) ||
      node.description?.toLowerCase().includes(query) ||
      node.type.toLowerCase().includes(query)
    );
  }, []);

  // Check if any filters are active (computed once, used for early exit)
  // v12.1: Query-First - type filter is bypassed when a query is active
  const hasActiveFilters = useMemo(() => {
    if (isMetaMode || hasActiveQuery) {
      // In meta mode or query-first mode, only check hidden/search filters (not type filter)
      return hiddenNodeIds.size > 0 || normalizedSearchQuery !== null;
    }
    return (
      hiddenNodeIds.size > 0 ||
      enabledNodeTypes.size !== ALL_NODE_TYPES.length ||
      selectedLocale !== null ||
      normalizedSearchQuery !== null
    );
  }, [hiddenNodeIds, enabledNodeTypes, selectedLocale, normalizedSearchQuery, isMetaMode, hasActiveQuery]);

  // Single-pass filter with aggregation
  // Combines all 5 filter stages + realm/layer counting into one pass
  const filterResult = useMemo((): FilterPassResult => {
    const realmCounts = createEmptyRealmCounts();
    const layerCounts = createEmptyLayerCounts();

    // Early exit: no nodes
    if (allNodes.length === 0) {
      return {
        nodes: allNodes,
        visibleNodeIds: new Set(),
        realmCounts,
        layerCounts,
      };
    }

    // Early exit: no filters active and no display limit
    if (!hasActiveFilters && (!displayLimit || displayLimit <= 0 || allNodes.length <= displayLimit)) {
      // Still need to compute counts
      const visibleNodeIds = new Set<string>();
      for (const node of allNodes) {
        visibleNodeIds.add(node.id);
        const realm = NODE_REALMS[node.type as NodeType];
        if (realm && realm in realmCounts) {
          realmCounts[realm]++;
        }
        const layer = NODE_LAYERS[node.type as NodeType];
        if (layer && layer in layerCounts) {
          layerCounts[layer]++;
        }
      }
      return { nodes: allNodes, visibleNodeIds, realmCounts, layerCounts };
    }

    // Pre-compute filter conditions for the loop
    const hasHiddenNodes = hiddenNodeIds.size > 0;
    // v12.1: Query-First - bypass type filter when a query is active
    // The query defines what's displayed, not the enabledNodeTypes filter
    const hasTypeFilter = !isMetaMode && !hasActiveQuery && enabledNodeTypes.size > 0;
    const hasLocaleFilter = !isMetaMode && selectedLocale !== null;
    const hasSearchFilter = normalizedSearchQuery !== null;
    const hasDisplayLimit = displayLimit && displayLimit > 0;

    // =========================================================================
    // SMART DISPLAY LIMIT: Rank nodes by connection count
    // =========================================================================
    // Instead of arbitrary slice, keep the most connected nodes when limiting.
    // This ensures hub nodes (high connectivity) are always visible.
    // =========================================================================

    // First pass: collect all nodes that pass filters (without limit)
    const candidateNodes: GraphNode[] = [];
    for (const node of allNodes) {
      // Stage 1: Hidden nodes filter
      if (hasHiddenNodes && hiddenNodeIds.has(node.id)) {
        continue;
      }

      // Stage 2: Type filter (bypassed in schema mode)
      if (hasTypeFilter && !enabledNodeTypes.has(node.type)) {
        continue;
      }

      // Stage 3: Locale filter (bypassed in schema mode)
      if (hasLocaleFilter) {
        const nodeLocale = node.data?.locale_code || node.data?.code;
        if (nodeLocale && nodeLocale !== selectedLocale) {
          continue;
        }
      }

      // Stage 4: Search filter
      if (hasSearchFilter && !matchesSearch(node, normalizedSearchQuery)) {
        continue;
      }

      candidateNodes.push(node);
    }

    // Apply display limit with connection-based ranking
    let filteredNodes: GraphNode[];
    if (hasDisplayLimit && candidateNodes.length > displayLimit) {
      // Compute connection counts and sort by most connected
      const connectionCounts = computeConnectionCounts(allEdges);
      const sortedNodes = [...candidateNodes].sort((a, b) => {
        const countA = connectionCounts.get(a.id) ?? 0;
        const countB = connectionCounts.get(b.id) ?? 0;
        return countB - countA; // Descending order (most connected first)
      });
      filteredNodes = sortedNodes.slice(0, displayLimit);
    } else {
      filteredNodes = candidateNodes;
    }

    const visibleNodeIds = new Set<string>();

    // Aggregate counts for visible nodes
    for (const node of filteredNodes) {
      visibleNodeIds.add(node.id);

      // Aggregate realm count
      const realm = NODE_REALMS[node.type as NodeType];
      if (realm && realm in realmCounts) {
        realmCounts[realm]++;
      }

      // Aggregate layer count
      const layer = NODE_LAYERS[node.type as NodeType];
      if (layer && layer in layerCounts) {
        layerCounts[layer]++;
      }
    }

    return { nodes: filteredNodes, visibleNodeIds, realmCounts, layerCounts };
  }, [
    allNodes,
    allEdges, // Added for connection-based ranking
    hasActiveFilters,
    hiddenNodeIds,
    enabledNodeTypes,
    selectedLocale,
    normalizedSearchQuery,
    displayLimit,
    isMetaMode,
    hasActiveQuery, // v12.1: Query-First - bypass type filter when query is active
    matchesSearch,
  ]);

  // Extract results from single-pass filter
  const { nodes: filteredNodes, visibleNodeIds, realmCounts, layerCounts } = filterResult;

  // Compute filtered edges (only those connecting visible nodes)
  // Early exit if no visible nodes
  const filteredEdges = useMemo(() => {
    if (visibleNodeIds.size === 0) return [];
    return allEdges.filter(
      (edge) => visibleNodeIds.has(edge.source) && visibleNodeIds.has(edge.target)
    );
  }, [allEdges, visibleNodeIds]);

  // Compute distinct relation types count
  // Early exit if no edges
  const distinctRelationTypes = useMemo(() => {
    if (filteredEdges.length === 0) return 0;
    const types = new Set(filteredEdges.map((edge) => edge.type));
    return types.size;
  }, [filteredEdges]);

  return {
    nodes: filteredNodes,
    edges: filteredEdges,
    totalNodes: allNodes.length,
    totalArcs: allEdges.length,
    isFiltered: hasActiveFilters,
    visibleNodeCount: filteredNodes.length,
    visibleEdgeCount: filteredEdges.length,
    distinctRelationTypes,
    realmCounts,
    layerCounts,
    isMetaMode,
  };
}
