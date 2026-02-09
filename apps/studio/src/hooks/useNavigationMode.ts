'use client';

/**
 * useNavigationMode - Hook for navigation mode state and mode-aware data fetching
 *
 * v11.0: Simplified to Meta and Data modes only
 *
 * Provides:
 * - Current mode, setMode, cycleMode
 * - fetchForMode() - dispatches to correct fetch per mode
 * - Boolean helpers: includesMeta, includesData
 */

import { useCallback } from 'react';
import { useUIStore, selectNavigationMode, type NavigationMode } from '@/stores/uiStore';
import { useGraphData, type GraphDataResponse } from './useGraphData';
import { useGraphStore } from '@/stores/graphStore';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';

export interface UseNavigationModeReturn {
  /** Current navigation mode */
  mode: NavigationMode;
  /** Set navigation mode directly */
  setMode: (mode: NavigationMode) => void;
  /** Cycle to next mode (meta -> data -> meta) */
  cycleMode: () => void;
  /** Fetch data appropriate for the current mode */
  fetchForMode: () => Promise<GraphDataResponse>;
  /** Whether current mode shows meta/schema nodes */
  includesMeta: boolean;
  /** Whether current mode shows data nodes */
  includesData: boolean;
}

export function useNavigationMode(): UseNavigationModeReturn {
  const mode = useUIStore(selectNavigationMode);
  const setMode = useUIStore((s) => s.setNavigationMode);
  const cycleMode = useUIStore((s) => s.cycleNavigationMode);

  const { fetchData, fetchSchemaData } = useGraphData({ showToast: false });

  // Graph store for clearing
  const clearGraph = useGraphStore((s) => s.clearGraph);

  const fetchForMode = useCallback(async (): Promise<GraphDataResponse> => {
    switch (mode) {
      case 'meta':
        return fetchSchemaData();

      case 'data':
        // v11.0: Data view intentionally shows empty state
        clearGraph();
        return { success: true, data: { nodes: [], edges: [] } };

      default:
        return fetchData({ limit: DEFAULT_FETCH_LIMIT });
    }
  }, [mode, fetchData, fetchSchemaData, clearGraph]);

  return {
    mode,
    setMode,
    cycleMode,
    fetchForMode,
    includesMeta: mode === 'meta',
    includesData: mode === 'data',
  };
}
