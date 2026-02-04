/**
 * useGraphData Hook
 *
 * Handles fetching graph data from the API and updating the store.
 * Provides loading states, error handling, and refetch capabilities.
 *
 * @example
 * const { fetchData, executeQuery, isLoading, error } = useGraphData();
 * await fetchData({ nodeTypes: ['Project', 'Page'] });
 */

import { useCallback, useState, useRef } from 'react';
import { useGraphStore } from '@/stores/graphStore';
import { useFilterStore } from '@/stores/filterStore';
import { useUIStore, selectNavigationMode, type NavigationMode } from '@/stores/uiStore';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { logger } from '@/lib/logger';
import { fetchJSON, postJSON, getErrorMessage } from '@/lib/fetchClient';
import { toast } from '@/lib/toast';
import type { NodeType, GraphNode, GraphEdge } from '@/types';

export interface GraphDataResponse {
  success: boolean;
  data?: {
    nodes: GraphNode[];
    edges: GraphEdge[];
  };
  meta?: {
    totalNodes: number;
    totalEdges: number;
    duration: number;
    requestDuration: number;
  };
  error?: string;
}

export interface FetchOptions {
  /** Node types to fetch (empty = use filter store) */
  nodeTypes?: NodeType[];
  /** Locale filter */
  locale?: string | null;
  /** Search query */
  search?: string;
  /** Maximum nodes to fetch */
  limit?: number;
}

export interface UseGraphDataReturn {
  /** Fetch graph data from API (or schema based on navigationMode) */
  fetchData: (options?: FetchOptions) => Promise<GraphDataResponse>;
  /** Fetch schema graph (ontology) */
  fetchSchemaData: () => Promise<GraphDataResponse>;
  /** Execute a custom Cypher query */
  executeQuery: (cypher: string) => Promise<GraphDataResponse>;
  /** Fetch graph statistics */
  fetchStats: () => Promise<Record<string, number> | null>;
  /** Current navigation mode */
  navigationMode: NavigationMode;
  /** Loading state */
  isLoading: boolean;
  /** Error message */
  error: string | null;
  /** Clear error */
  clearError: () => void;
}

export interface UseGraphDataOptions {
  /** Show toast notifications for query results */
  showToast?: boolean;
}

export function useGraphData(options: UseGraphDataOptions = {}): UseGraphDataReturn {
  const { showToast = true } = options;
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Track loading toast ID for dismissal
  const loadingToastRef = useRef<string | number | null>(null);

  // Store actions
  const setGraphData = useGraphStore((state) => state.setGraphData);
  const setLoading = useGraphStore((state) => state.setLoading);
  const setStoreError = useGraphStore((state) => state.setError);

  // Filter store values (for default fetch)
  const enabledNodeTypes = useFilterStore((state) => state.enabledNodeTypes);
  const selectedLocale = useFilterStore((state) => state.selectedLocale);
  const searchQuery = useFilterStore((state) => state.searchQuery);

  // Navigation mode (data vs meta vs overlay vs query)
  const navigationMode = useUIStore(selectNavigationMode);

  /**
   * Fetch graph data from API
   */
  const fetchData = useCallback(
    async (options: FetchOptions = {}): Promise<GraphDataResponse> => {
      setIsLoading(true);
      setLoading(true);
      setError(null);

      try {
        // Use provided options or fall back to filter store values
        const nodeTypes = options.nodeTypes || Array.from(enabledNodeTypes);
        const locale = options.locale !== undefined ? options.locale : selectedLocale;
        const search = options.search !== undefined ? options.search : searchQuery;
        const limit = options.limit || DEFAULT_FETCH_LIMIT;

        // Build query params
        const params = new URLSearchParams();
        if (nodeTypes.length > 0) {
          params.set('nodeTypes', nodeTypes.join(','));
        }
        if (locale) {
          params.set('locale', locale);
        }
        if (search) {
          params.set('search', search);
        }
        params.set('limit', limit.toString());

        const data = await fetchJSON<GraphDataResponse>(`/api/graph?${params.toString()}`);

        // Update store with fetched data (with defensive null check)
        if (data.success && data.data) {
          setGraphData(data.data);
        } else if (!data.success) {
          throw new Error(data.error || 'Failed to fetch graph data');
        }

        setIsLoading(false);
        setLoading(false);
        return data;
      } catch (err) {
        const errorMessage = getErrorMessage(err);
        setError(errorMessage);
        setStoreError(errorMessage);
        setIsLoading(false);
        setLoading(false);
        return { success: false, error: errorMessage };
      }
    },
    // Note: setIsLoading and setError are stable (useState setters) but included for consistency
    [enabledNodeTypes, selectedLocale, searchQuery, setGraphData, setLoading, setStoreError, setIsLoading, setError]
  );

  /**
   * Fetch schema graph (ontology) - 46 node types and their relationships
   */
  const fetchSchemaData = useCallback(
    async (): Promise<GraphDataResponse> => {
      setIsLoading(true);
      setLoading(true);
      setError(null);

      try {
        const data = await fetchJSON<GraphDataResponse>('/api/graph/ontology');

        // Update store with schema data
        if (data.success && data.data) {
          setGraphData(data.data);
        } else if (!data.success) {
          throw new Error(data.error || 'Failed to fetch schema graph');
        }

        setIsLoading(false);
        setLoading(false);
        return data;
      } catch (err) {
        const errorMessage = getErrorMessage(err);
        setError(errorMessage);
        setStoreError(errorMessage);
        setIsLoading(false);
        setLoading(false);
        return { success: false, error: errorMessage };
      }
    },
    [setGraphData, setLoading, setStoreError, setIsLoading, setError]
  );

  /**
   * Execute a custom Cypher query
   */
  const executeQuery = useCallback(
    async (cypher: string): Promise<GraphDataResponse> => {
      setIsLoading(true);
      setLoading(true);
      setError(null);

      // Show loading toast
      if (showToast) {
        loadingToastRef.current = toast.queryExecuting();
      }

      try {
        const data = await postJSON<GraphDataResponse>('/api/graph/query', { cypher });

        // Dismiss loading toast
        if (loadingToastRef.current) {
          toast.dismiss(loadingToastRef.current);
          loadingToastRef.current = null;
        }

        // Update store with query results (with defensive null check)
        if (data.success && data.data) {
          setGraphData(data.data);

          // Show success toast with results count
          if (showToast) {
            const nodeCount = data.data.nodes?.length || 0;
            const edgeCount = data.data.edges?.length || 0;
            toast.queryResult(nodeCount, edgeCount);
          }
        } else if (!data.success) {
          throw new Error(data.error || 'Query execution failed');
        }

        setIsLoading(false);
        setLoading(false);
        return data;
      } catch (err) {
        // Dismiss loading toast
        if (loadingToastRef.current) {
          toast.dismiss(loadingToastRef.current);
          loadingToastRef.current = null;
        }

        const errorMessage = getErrorMessage(err);
        setError(errorMessage);
        setStoreError(errorMessage);
        setIsLoading(false);
        setLoading(false);

        // Show error toast
        if (showToast) {
          toast.queryError(errorMessage);
        }

        return { success: false, error: errorMessage };
      }
    },
    // Note: setIsLoading and setError are stable (useState setters) but included for consistency
    [setGraphData, setLoading, setStoreError, setIsLoading, setError, showToast]
  );

  /**
   * Fetch graph statistics
   */
  const fetchStats = useCallback(async (): Promise<Record<string, number> | null> => {
    try {
      const data = await fetchJSON<{ success: boolean; data: { byType: Record<string, number> } }>(
        '/api/graph/stats'
      );
      return data.data.byType;
    } catch (err) {
      logger.error('Graph', 'Failed to fetch stats', err);
      return null;
    }
  }, []);

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
    setStoreError(null);
  }, [setStoreError, setError]);

  return {
    fetchData,
    fetchSchemaData,
    executeQuery,
    fetchStats,
    navigationMode,
    isLoading,
    error,
    clearError,
  };
}
