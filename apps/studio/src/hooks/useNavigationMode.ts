'use client';

/**
 * useNavigationMode - Hook for navigation mode state and mode-aware data fetching
 *
 * Provides:
 * - Current mode, setMode, cycleMode
 * - fetchForMode() - dispatches to correct fetch per mode
 * - Boolean helpers: includesMeta, includesData, usesFacets
 */

import { useCallback } from 'react';
import { useUIStore, selectNavigationMode, type NavigationMode } from '@/stores/uiStore';
import { useFilterStore } from '@/stores/filterStore';
import { useGraphData, type GraphDataResponse } from './useGraphData';
import { useGraphStore } from '@/stores/graphStore';
import { generateSchemaGraph } from '@/lib/schemaGenerator';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { fetchJSON, getErrorMessage } from '@/lib/fetchClient';
import { logger } from '@/lib/logger';

export interface UseNavigationModeReturn {
  /** Current navigation mode */
  mode: NavigationMode;
  /** Set navigation mode directly */
  setMode: (mode: NavigationMode) => void;
  /** Cycle to next mode (meta -> data -> overlay -> query -> meta) */
  cycleMode: () => void;
  /** Fetch data appropriate for the current mode */
  fetchForMode: () => Promise<GraphDataResponse>;
  /** Whether current mode shows meta/schema nodes */
  includesMeta: boolean;
  /** Whether current mode shows data nodes */
  includesData: boolean;
  /** Whether current mode uses faceted filters */
  usesFacets: boolean;
}

export function useNavigationMode(): UseNavigationModeReturn {
  const mode = useUIStore(selectNavigationMode);
  const setMode = useUIStore((s) => s.setNavigationMode);
  const cycleMode = useUIStore((s) => s.cycleNavigationMode);

  const { fetchData, fetchSchemaData } = useGraphData({ showToast: false });

  // Facet state for query mode
  const realmFilter = useFilterStore((s) => s.realmFilter);
  const traitFilter = useFilterStore((s) => s.traitFilter);
  const layerFilter = useFilterStore((s) => s.layerFilter);
  const arcFamilyFilter = useFilterStore((s) => s.arcFamilyFilter);

  // Graph store for overlay merging
  const setGraphData = useGraphStore((s) => s.setGraphData);

  const fetchForMode = useCallback(async (): Promise<GraphDataResponse> => {
    switch (mode) {
      case 'data':
        return fetchData({ limit: DEFAULT_FETCH_LIMIT });

      case 'meta':
        return fetchSchemaData();

      case 'overlay': {
        // Fetch real data first, then merge schema nodes
        const dataResult = await fetchData({ limit: DEFAULT_FETCH_LIMIT });
        if (dataResult.success && dataResult.data) {
          try {
            const schemaGraph = generateSchemaGraph();
            // Merge schema nodes with data nodes (schema nodes get isMetaMode flag)
            const mergedNodes = [
              ...dataResult.data.nodes,
              ...schemaGraph.nodes.map((n) => ({ ...n, isMetaMode: true })),
            ];
            const mergedEdges = [
              ...dataResult.data.edges,
              ...schemaGraph.edges,
            ];
            setGraphData({ nodes: mergedNodes, edges: mergedEdges });
          } catch (err) {
            logger.warn('NavigationMode', 'Failed to merge schema in overlay mode', err);
          }
        }
        return dataResult;
      }

      case 'query': {
        // Build facet query params
        const params = new URLSearchParams();
        if (realmFilter.length > 0) params.set('realms', realmFilter.join(','));
        if (layerFilter.length > 0) params.set('layers', layerFilter.join(','));
        if (traitFilter.length > 0) params.set('traits', traitFilter.join(','));
        if (arcFamilyFilter.length > 0) params.set('arcFamilies', arcFamilyFilter.join(','));

        try {
          const data = await fetchJSON<GraphDataResponse>(
            `/api/graph/navigation?${params.toString()}`
          );
          if (data.success && data.data) {
            setGraphData(data.data);
          }
          return data;
        } catch (err) {
          const errorMessage = getErrorMessage(err);
          return { success: false, error: errorMessage };
        }
      }

      default:
        return fetchData({ limit: DEFAULT_FETCH_LIMIT });
    }
  }, [
    mode, fetchData, fetchSchemaData, setGraphData,
    realmFilter, traitFilter, layerFilter, arcFamilyFilter,
  ]);

  return {
    mode,
    setMode,
    cycleMode,
    fetchForMode,
    includesMeta: mode === 'meta' || mode === 'overlay',
    includesData: mode === 'data' || mode === 'overlay' || mode === 'query',
    usesFacets: mode === 'query',
  };
}
