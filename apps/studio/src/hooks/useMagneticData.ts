/**
 * useMagneticData - Fetch organizing principles for magnetic layout
 *
 * When layoutMode is 'magnetic', this hook fetches from Neo4j:
 * - 3 Scope nodes (global, project, shared) with display metadata
 * - 9 Subcategory nodes with display metadata
 * - 35 nodeType → subcategory mappings (from DEFINES_TYPE relationships)
 *
 * All display metadata (emoji, color) comes from Neo4j,
 * which is seeded from organizing-principles.yaml (the source of truth).
 */

import { useEffect, useState, useCallback } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useUIStore } from '@/stores/uiStore';
import { logger } from '@/lib/logger';

// Types for organizing principles - all data from Neo4j, no hardcoded metadata
export interface ScopeData {
  key: string;
  displayName: string;
  emoji: string;
  color: string;
}

export interface SubcategoryData {
  key: string;
  displayName: string;
  emoji: string;
  scopeKey: string;
}

export interface OrganizingPrinciples {
  scopes: ScopeData[];
  subcategories: SubcategoryData[];
  /** nodeType → subcategory key mapping (from DEFINES_TYPE in Neo4j) */
  nodeTypeMapping: Record<string, string>;
}

/**
 * Hook to fetch and manage organizing principles for magnetic layout.
 * All metadata comes from Neo4j (no hardcoded colors/emoji).
 */
export function useMagneticData() {
  const layoutMode = useUIStore(
    useShallow((state) => state.layoutMode)
  );

  const [data, setData] = useState<OrganizingPrinciples | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch organizing principles from API
  const fetchOrganizingPrinciples = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const res = await fetch('/api/graph/organizing-principles');
      const json = await res.json();

      if (!json.success) {
        throw new Error(json.error || 'Failed to fetch organizing principles');
      }

      // Transform API response - all display metadata comes from Neo4j
      const scopes: ScopeData[] = json.data.scopes.map(
        (s: { key: string; display_name: string; emoji: string; color: string }) => ({
          key: s.key,
          displayName: s.display_name,
          emoji: s.emoji,
          color: s.color,
        })
      );

      const subcategories: SubcategoryData[] = json.data.subcategories.map(
        (sub: { key: string; display_name: string; emoji: string; scope_key: string }) => ({
          key: sub.key,
          displayName: sub.display_name,
          emoji: sub.emoji,
          scopeKey: sub.scope_key,
        })
      );

      const nodeTypeMapping: Record<string, string> = json.data.nodeTypeMapping;

      setData({ scopes, subcategories, nodeTypeMapping });
      logger.info('MagneticData', 'Organizing principles loaded', {
        scopes: scopes.length,
        subcategories: subcategories.length,
        nodeTypes: Object.keys(nodeTypeMapping).length,
      });
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Unknown error';
      setError(message);
      logger.error('MagneticData', 'Failed to fetch organizing principles', { error: message });
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Fetch when switching to magnetic mode
  useEffect(() => {
    if (layoutMode === 'magnetic' && !data && !isLoading) {
      fetchOrganizingPrinciples();
    }
  }, [layoutMode, data, isLoading, fetchOrganizingPrinciples]);

  // Clear data when switching away from magnetic mode
  useEffect(() => {
    if (layoutMode !== 'magnetic') {
      setData(null);
      setError(null);
    }
  }, [layoutMode]);

  return {
    /** Organizing principles data (null if not in magnetic mode or loading) */
    data,
    /** Whether we're currently fetching */
    isLoading,
    /** Error message if fetch failed */
    error,
    /** Whether magnetic mode is active */
    isMagneticMode: layoutMode === 'magnetic',
    /** Refetch organizing principles */
    refetch: fetchOrganizingPrinciples,
  };
}

export type UseMagneticDataReturn = ReturnType<typeof useMagneticData>;
