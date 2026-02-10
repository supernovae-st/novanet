// =============================================================================
// USE CONTEXT VIEWS HOOK (v11.6)
// =============================================================================
// Returns available context views for a selected node with live stats

import { useMemo, useState, useEffect } from 'react';
import type { GraphNode, GraphEdge } from '@/types';
import type { NodeType } from '@novanet/core/types';
import { getViewsForNodeType, type ViewId, type ViewTypeConfig } from '@/config/viewTypes';

// =============================================================================
// TYPES
// =============================================================================

export interface ViewStats {
  nodeCount: number;
  arcCount: number;
  depth?: number;
  /** Breakdown by node type */
  nodesByType: Record<string, number>;
  /** Breakdown by arc type */
  arcsByType: Record<string, number>;
  /** For compact views: completion percentage */
  completion?: number;
}

export interface ContextView extends ViewTypeConfig {
  stats: ViewStats;
  /** Is data currently loading */
  isLoading: boolean;
}

export interface UseContextViewsResult {
  views: ContextView[];
  isLoading: boolean;
  error: Error | null;
}

// =============================================================================
// STATS CALCULATION HELPERS
// =============================================================================

/**
 * Calculate stats from edges for a specific view
 */
function calculateViewStats(
  node: GraphNode,
  edges: GraphEdge[],
  allNodes: GraphNode[],
  viewId: ViewId
): ViewStats {
  // Get edges connected to this node
  const connectedEdges = edges.filter(
    (e) => e.source === node.id || e.target === node.id
  );

  // Get connected node IDs
  const connectedNodeIds = new Set(
    connectedEdges.flatMap((e) => [e.source, e.target]).filter((id) => id !== node.id)
  );

  // Get connected nodes
  const connectedNodes = allNodes.filter((n) => connectedNodeIds.has(n.id));

  // Count by type
  const nodesByType: Record<string, number> = {};
  connectedNodes.forEach((n) => {
    nodesByType[n.type] = (nodesByType[n.type] || 0) + 1;
  });

  const arcsByType: Record<string, number> = {};
  connectedEdges.forEach((e) => {
    arcsByType[e.type] = (arcsByType[e.type] || 0) + 1;
  });

  // Filter based on view type
  let relevantNodes = connectedNodes;
  let relevantEdges = connectedEdges;

  switch (viewId) {
    case 'composition':
      relevantEdges = connectedEdges.filter((e) => e.type === 'HAS_BLOCK');
      break;
    case 'entities':
      relevantEdges = connectedEdges.filter((e) =>
        ['USES_ENTITY', 'HAS_ENTITY', 'TARGETS'].includes(e.type)
      );
      break;
    case 'knowledge':
      relevantEdges = connectedEdges.filter((e) =>
        e.type.startsWith('HAS_') || e.type.startsWith('CONTAINS_')
      );
      break;
    case 'locales':
      relevantEdges = connectedEdges.filter((e) =>
        ['HAS_CONTENT', 'HAS_GENERATED', 'FOR_LOCALE'].includes(e.type)
      );
      break;
    case 'geographic':
      relevantEdges = connectedEdges.filter((e) =>
        ['FOR_COUNTRY', 'IN_REGION', 'IN_CONTINENT'].includes(e.type)
      );
      break;
    case 'seo-intel':
      relevantEdges = connectedEdges.filter((e) =>
        ['TARGETS', 'IN_CLUSTER', 'HAS_METRICS'].includes(e.type)
      );
      break;
    case 'geo-intel':
      relevantEdges = connectedEdges.filter((e) =>
        ['MONITORS_GEO', 'HAS_ANSWER', 'IN_QUERY_SET'].includes(e.type)
      );
      break;
    case 'generation':
      relevantEdges = connectedEdges.filter((e) =>
        ['HAS_GENERATED', 'GENERATED_BY'].includes(e.type)
      );
      break;
    case 'categories':
      relevantEdges = connectedEdges.filter((e) => e.type === 'BELONGS_TO');
      break;
    case 'cross-realm':
      // Cross-realm arcs connect org to shared
      relevantEdges = connectedEdges.filter((e) =>
        ['BELONGS_TO', 'TARGETS', 'FOR_LOCALE', 'MONITORS_GEO', 'FOR_COUNTRY'].includes(e.type)
      );
      break;
  }

  // Recalculate connected nodes based on filtered edges
  const filteredNodeIds = new Set(
    relevantEdges.flatMap((e) => [e.source, e.target]).filter((id) => id !== node.id)
  );
  relevantNodes = allNodes.filter((n) => filteredNodeIds.has(n.id));

  // Recalculate nodesByType for relevant nodes
  const filteredNodesByType: Record<string, number> = {};
  relevantNodes.forEach((n) => {
    filteredNodesByType[n.type] = (filteredNodesByType[n.type] || 0) + 1;
  });

  const filteredArcsByType: Record<string, number> = {};
  relevantEdges.forEach((e) => {
    filteredArcsByType[e.type] = (filteredArcsByType[e.type] || 0) + 1;
  });

  return {
    nodeCount: relevantNodes.length + 1, // +1 for the node itself
    arcCount: relevantEdges.length,
    nodesByType: filteredNodesByType,
    arcsByType: filteredArcsByType,
    depth: calculateDepth(node, relevantEdges, allNodes),
    completion: calculateCompletion(viewId, relevantNodes, relevantEdges),
  };
}

/**
 * Calculate hierarchy depth (for tree views)
 */
function calculateDepth(
  _node: GraphNode,
  edges: GraphEdge[],
  _allNodes: GraphNode[]
): number {
  // Simplified depth calculation based on edge count
  // Real implementation would traverse the graph
  const hierarchyEdges = edges.filter((e) =>
    ['HAS_BLOCK', 'HAS_PAGE', 'HAS_ENTITY', 'CONTAINS_TERM', 'CONTAINS_EXPRESSION'].includes(e.type)
  );
  return Math.min(hierarchyEdges.length, 5) + 1;
}

/**
 * Calculate completion percentage (for compact views)
 */
function calculateCompletion(
  viewId: ViewId,
  nodes: GraphNode[],
  edges: GraphEdge[]
): number | undefined {
  if (viewId !== 'locales' && viewId !== 'content') {
    return undefined;
  }

  // For locale views, calculate based on content nodes
  const contentEdges = edges.filter((e) =>
    ['HAS_CONTENT', 'HAS_GENERATED'].includes(e.type)
  );

  // Assume 12 locales as target (configurable)
  const targetLocales = 12;
  const coveredLocales = contentEdges.length;

  return Math.round((coveredLocales / targetLocales) * 100);
}

// =============================================================================
// MAIN HOOK
// =============================================================================

/**
 * Get available context views for a node with stats
 */
export function useContextViews(
  node: GraphNode | null,
  edges: GraphEdge[],
  allNodes: GraphNode[]
): UseContextViewsResult {
  // Get applicable views for this node type
  const applicableViews = useMemo(() => {
    if (!node) return [];
    return getViewsForNodeType(node.type as NodeType);
  }, [node?.type]);

  // Calculate stats for each view
  const viewsWithStats = useMemo((): ContextView[] => {
    if (!node) return [];

    return applicableViews.map((view) => ({
      ...view,
      stats: calculateViewStats(node, edges, allNodes, view.id),
      isLoading: false,
    }));
  }, [node, edges, allNodes, applicableViews]);

  return {
    views: viewsWithStats,
    isLoading: false,
    error: null,
  };
}

// =============================================================================
// VIEW DATA TYPES
// =============================================================================

export interface ViewData {
  nodes: GraphNode[];
  edges: GraphEdge[];
  view: {
    id: ViewId;
    label: string;
    style: 'tree' | 'flow' | 'compact';
    effect: string;
    transitionColor: string;
  };
}

export interface ViewDataResult {
  success: boolean;
  data?: ViewData;
  meta?: {
    totalNodes: number;
    totalArcs: number;
    queryDuration: number;
    requestDuration: number;
    description: string;
  };
  error?: string;
}

// =============================================================================
// VIEW DETAILS HOOK
// =============================================================================

/**
 * Fetch full view data from Neo4j for a specific view
 * Used when selecting a context view from Action Cards
 */
export function useViewDetails(
  node: GraphNode | null,
  viewId: ViewId | null
) {
  const [data, setData] = useState<ViewData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!node || !viewId) {
      setData(null);
      return;
    }

    // Create abort controller for cleanup
    const abortController = new AbortController();

    const fetchData = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const params = new URLSearchParams({
          nodeId: node.id,
          nodeKey: node.key,
          nodeType: node.type,
        });

        const response = await fetch(`/api/views/${viewId}/data?${params}`, {
          signal: abortController.signal,
        });

        if (!response.ok) {
          const result = await response.json();
          throw new Error(result.error || 'Failed to fetch view data');
        }

        const result: ViewDataResult = await response.json();

        if (!result.success || !result.data) {
          throw new Error(result.error || 'Invalid response');
        }

        setData(result.data);
      } catch (err) {
        // Ignore abort errors
        if (err instanceof Error && err.name === 'AbortError') {
          return;
        }
        setError(err instanceof Error ? err : new Error('Unknown error'));
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();

    // Cleanup: abort fetch on unmount or dependency change
    return () => {
      abortController.abort();
    };
  }, [node?.id, node?.key, node?.type, viewId]);

  return { data, isLoading, error };
}

/**
 * Fetch lightweight stats for a view (used for hovering/previews)
 */
export function useViewStats(
  node: GraphNode | null,
  viewId: ViewId | null
) {
  const [stats, setStats] = useState<{ nodeCount: number; arcCount: number } | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (!node || !viewId) {
      setStats(null);
      return;
    }

    const abortController = new AbortController();

    const fetchStats = async () => {
      setIsLoading(true);

      try {
        const params = new URLSearchParams({
          nodeId: node.id,
          nodeKey: node.key,
          nodeType: node.type,
          statsOnly: 'true',
        });

        const response = await fetch(`/api/views/${viewId}/data?${params}`, {
          signal: abortController.signal,
        });

        if (response.ok) {
          const result = await response.json();
          if (result.success && result.data?.stats) {
            setStats(result.data.stats);
          }
        }
      } catch {
        // Silently ignore stats errors (non-critical)
      } finally {
        setIsLoading(false);
      }
    };

    fetchStats();

    return () => {
      abortController.abort();
    };
  }, [node?.id, node?.key, node?.type, viewId]);

  return { stats, isLoading };
}

export default useContextViews;
