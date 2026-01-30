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
import { ALL_NODE_TYPES } from '@/config/nodeTypes';
import type { GraphNode, GraphEdge } from '@/types';

export interface FilteredGraphResult {
  /** Filtered nodes based on enabled types and locale */
  nodes: GraphNode[];
  /** Filtered edges (only those connecting visible nodes) */
  edges: GraphEdge[];
  /** Total number of nodes before filtering */
  totalNodes: number;
  /** Total number of edges before filtering */
  totalEdges: number;
  /** Whether any filters are active */
  isFiltered: boolean;
  /** Count of visible nodes */
  visibleNodeCount: number;
  /** Count of visible edges */
  visibleEdgeCount: number;
}

export function useFilteredGraph(): FilteredGraphResult {
  // Get raw data from stores
  const allNodes = useGraphStore((state) => state.nodes);
  const allEdges = useGraphStore((state) => state.edges);
  const hiddenNodeIds = useGraphStore((state) => state.hiddenNodeIds);
  const enabledNodeTypes = useFilterStore((state) => state.enabledNodeTypes);
  const selectedLocale = useFilterStore((state) => state.selectedLocale);
  const searchQuery = useFilterStore((state) => state.searchQuery);

  // Chained memos for optimal performance:
  // Each filter stage only recalculates when its dependencies change

  // Stage 1: Filter out hidden nodes
  const unhiddenNodes = useMemo(() => {
    if (hiddenNodeIds.size === 0) return allNodes;
    return allNodes.filter((node) => !hiddenNodeIds.has(node.id));
  }, [allNodes, hiddenNodeIds]);

  // Stage 2: Filter by node type
  const typeFilteredNodes = useMemo(() => {
    if (enabledNodeTypes.size === 0) return unhiddenNodes;
    return unhiddenNodes.filter((node) => enabledNodeTypes.has(node.type));
  }, [unhiddenNodes, enabledNodeTypes]);

  // Stage 3: Filter by locale
  const localeFilteredNodes = useMemo(() => {
    if (!selectedLocale) return typeFilteredNodes;
    return typeFilteredNodes.filter((node) => {
      // Include nodes that match the locale or don't have a locale (global nodes)
      const nodeLocale = node.data?.locale_code || node.data?.code;
      return !nodeLocale || nodeLocale === selectedLocale;
    });
  }, [typeFilteredNodes, selectedLocale]);

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

  // Check if any filters are active
  const isFiltered = useMemo(() => {
    return (
      hiddenNodeIds.size > 0 ||
      enabledNodeTypes.size !== ALL_NODE_TYPES.length ||
      selectedLocale !== null ||
      (searchQuery !== null && searchQuery.trim() !== '')
    );
  }, [hiddenNodeIds, enabledNodeTypes, selectedLocale, searchQuery]);

  return {
    nodes: filteredNodes,
    edges: filteredEdges,
    totalNodes: allNodes.length,
    totalEdges: allEdges.length,
    isFiltered,
    visibleNodeCount: filteredNodes.length,
    visibleEdgeCount: filteredEdges.length,
  };
}
