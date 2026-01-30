'use client';

/**
 * useDatabaseSchema - Hook for fetching Neo4j database schema
 *
 * Features:
 * - Fetches node labels, relationship types, and property keys
 * - Tracks loading state and errors
 * - Auto-fetches on mount
 * - Manual refresh capability
 */

import { useState, useEffect, useCallback, useMemo, useRef } from 'react';
import { fetchJSON, FetchError } from '@/lib/fetchClient';

// =============================================================================
// TYPES
// =============================================================================

export interface NodeLabel {
  label: string;
  count: number;
}

export interface RelationType {
  type: string;
  count: number;
}

export interface SchemaData {
  nodeLabels: NodeLabel[];
  relationshipTypes: RelationType[];
  propertyKeys: string[];
  totalNodes: number;
  totalRelationships: number;
}

interface SchemaApiResponse {
  success: boolean;
  data: SchemaData;
  error?: string;
}

export interface UseDatabaseSchemaReturn {
  /** The fetched schema data */
  schema: SchemaData | null;
  /** Whether the schema is currently being fetched */
  isLoading: boolean;
  /** Error message if fetch failed */
  error: string | null;
  /** Timestamp of last successful fetch */
  lastUpdate: Date | null;
  /** Manually refresh the schema */
  refresh: () => Promise<void>;
  /** Map of label -> count for quick lookup */
  labelCounts: Map<string, number>;
  /** Maximum node count (for progress bars) */
  maxNodeCount: number;
  /** Maximum relationship count (for progress bars) */
  maxRelCount: number;
}

// =============================================================================
// HOOK
// =============================================================================

export function useDatabaseSchema(): UseDatabaseSchemaReturn {
  const [schema, setSchema] = useState<SchemaData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdate, setLastUpdate] = useState<Date | null>(null);

  // Track if component is mounted to avoid state updates after unmount
  const isMounted = useRef(true);

  // Fetch schema from API
  const refresh = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await fetchJSON<SchemaApiResponse>('/api/graph/schema');

      if (isMounted.current) {
        setSchema(result.data);
        setLastUpdate(new Date());
      }
    } catch (err) {
      if (isMounted.current) {
        if (err instanceof FetchError) {
          setError(err.message);
        } else {
          setError(err instanceof Error ? err.message : 'Unknown error');
        }
      }
    } finally {
      if (isMounted.current) {
        setIsLoading(false);
      }
    }
  }, []);

  // Fetch on mount, cleanup on unmount
  useEffect(() => {
    isMounted.current = true;
    refresh();

    return () => {
      isMounted.current = false;
    };
  }, [refresh]);

  // Build counts map (with null safety)
  const labelCounts = useMemo(() => {
    const map = new Map<string, number>();
    schema?.nodeLabels?.forEach((item) => map.set(item.label, item.count));
    return map;
  }, [schema]);

  // Calculate max count for progress bars (with null safety)
  const maxNodeCount = useMemo(() => {
    if (!schema?.nodeLabels?.length) return 1;
    return Math.max(...schema.nodeLabels.map((l) => l.count), 1);
  }, [schema]);

  const maxRelCount = useMemo(() => {
    if (!schema?.relationshipTypes?.length) return 1;
    return Math.max(...schema.relationshipTypes.map((r) => r.count), 1);
  }, [schema]);

  return {
    schema,
    isLoading,
    error,
    lastUpdate,
    refresh,
    labelCounts,
    maxNodeCount,
    maxRelCount,
  };
}
