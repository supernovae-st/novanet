/**
 * useFilteredGraph Hook
 *
 * Computed selector that combines graphStore nodes with filterStore settings.
 * Provides filtered nodes and edges based on enabled node types and locale.
 *
 * Performance: Uses useMemo to prevent unnecessary recalculations.
 * This is critical for 19k nodes.
 *
 * @example
 * const { nodes, edges, isFiltered } = useFilteredGraph();
 */

import { useMemo } from 'react';
import { useGraphStore } from '@/stores/graphStore';
import { useFilterStore } from '@/stores/filterStore';
import { useUIStore, selectNavigationMode } from '@/stores/uiStore';
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

export function useFilteredGraph(): FilteredGraphResult {
  // Get raw data from stores
  const allNodes = useGraphStore((state) => state.nodes);
  const allEdges = useGraphStore((state) => state.edges);
  const hiddenNodeIds = useGraphStore((state) => state.hiddenNodeIds);
  const enabledNodeTypes = useFilterStore((state) => state.enabledNodeTypes);
  const selectedLocale = useFilterStore((state) => state.selectedLocale);
  const searchQuery = useFilterStore((state) => state.searchQuery);

  // Navigation mode: meta mode bypasses filters to show all schema types
  const navigationMode = useUIStore(selectNavigationMode);
  const isMetaMode = navigationMode === 'meta';

  // Chained memos for optimal performance:
  // Each filter stage only recalculates when its dependencies change

  // Stage 1: Filter out hidden nodes
  const unhiddenNodes = useMemo(() => {
    if (hiddenNodeIds.size === 0) return allNodes;
    return allNodes.filter((node) => !hiddenNodeIds.has(node.id));
  }, [allNodes, hiddenNodeIds]);

  // Stage 2: Filter by node type (bypassed in schema mode to show all 35 types)
  const typeFilteredNodes = useMemo(() => {
    // In schema mode, show all types regardless of filter settings
    if (isMetaMode) return unhiddenNodes;
    if (enabledNodeTypes.size === 0) return unhiddenNodes;
    return unhiddenNodes.filter((node) => enabledNodeTypes.has(node.type));
  }, [unhiddenNodes, enabledNodeTypes, isMetaMode]);

  // Stage 3: Filter by locale (bypassed in schema mode - schema nodes have no locale)
  const localeFilteredNodes = useMemo(() => {
    // In schema mode, skip locale filtering (schema nodes don't have locales)
    if (isMetaMode) return typeFilteredNodes;
    if (!selectedLocale) return typeFilteredNodes;
    return typeFilteredNodes.filter((node) => {
      // Include nodes that match the locale or don't have a locale (global nodes)
      const nodeLocale = node.data?.locale_code || node.data?.code;
      return !nodeLocale || nodeLocale === selectedLocale;
    });
  }, [typeFilteredNodes, selectedLocale, isMetaMode]);

  // Stage 4: Filter by search query
  const filteredNodes = useMemo(() => {
    if (!searchQuery || !searchQuery.trim()) return localeFilteredNodes;
    const query = searchQuery.toLowerCase().trim();
    return localeFilteredNodes.filter(
      (node) =>
        node.key?.toLowerCase().includes(query) ||
        node.displayName?.toLowerCase().includes(query) ||
        node.description?.toLowerCase().includes(query) ||
        node.type.toLowerCase().includes(query)
    );
  }, [localeFilteredNodes, searchQuery]);

  // Compute visible node IDs for edge filtering
  const visibleNodeIds = useMemo(() => {
    return new Set(filteredNodes.map((node) => node.id));
  }, [filteredNodes]);

  // Compute filtered edges (only those connecting visible nodes)
  const filteredEdges = useMemo(() => {
    return allEdges.filter(
      (edge) => visibleNodeIds.has(edge.source) && visibleNodeIds.has(edge.target)
    );
  }, [allEdges, visibleNodeIds]);

  // Check if any filters are active (in schema mode, type/locale filters are bypassed)
  const isFiltered = useMemo(() => {
    // In schema mode, only search affects visible nodes
    if (isMetaMode) {
      return (
        hiddenNodeIds.size > 0 ||
        (searchQuery !== null && searchQuery.trim() !== '')
      );
    }
    return (
      hiddenNodeIds.size > 0 ||
      enabledNodeTypes.size !== ALL_NODE_TYPES.length ||
      selectedLocale !== null ||
      (searchQuery !== null && searchQuery.trim() !== '')
    );
  }, [hiddenNodeIds, enabledNodeTypes, selectedLocale, searchQuery, isMetaMode]);

  // Compute distinct relation types count
  const distinctRelationTypes = useMemo(() => {
    const types = new Set(filteredEdges.map((edge) => edge.type));
    return types.size;
  }, [filteredEdges]);

  // Compute scope counts (for schema mode breakdown) - v11.2: 2 realms
  const realmCounts = useMemo((): RealmCounts => {
    const counts: RealmCounts = { shared: 0, org: 0 };
    for (const node of filteredNodes) {
      const scope = NODE_REALMS[node.type as NodeType];
      if (scope && scope in counts) {
        counts[scope]++;
      }
    }
    return counts;
  }, [filteredNodes]);

  // Compute layer counts (for schema mode breakdown) - v11.4: 10 layers
  const layerCounts = useMemo((): LayerCounts => {
    const counts: LayerCounts = {
      // Shared realm (4) — v11.4: includes config
      config: 0, locale: 0, geography: 0, knowledge: 0,
      // Org realm (6) — v11.4: seo/geo removed
      foundation: 0, structure: 0, semantic: 0,
      instruction: 0, output: 0,
    };
    for (const node of filteredNodes) {
      const layer = NODE_LAYERS[node.type as NodeType];
      if (layer && layer in counts) {
        counts[layer]++;
      }
    }
    return counts;
  }, [filteredNodes]);

  return {
    nodes: filteredNodes,
    edges: filteredEdges,
    totalNodes: allNodes.length,
    totalArcs: allEdges.length,
    isFiltered,
    visibleNodeCount: filteredNodes.length,
    visibleEdgeCount: filteredEdges.length,
    distinctRelationTypes,
    realmCounts,
    layerCounts,
    isMetaMode,
  };
}
