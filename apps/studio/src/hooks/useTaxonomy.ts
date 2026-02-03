'use client';

/**
 * useTaxonomy - Hook for loading taxonomy data from the server
 *
 * Fetches and caches taxonomy data including realms, layers, traits,
 * and arc families with their visual encoding from Neo4j.
 *
 * @example
 * const { taxonomy, isLoading, error, getArcFamilyColor } = useTaxonomy();
 *
 * // Get color for a relation's arc family
 * const color = getArcFamilyColor('ownership'); // '#3b82f6'
 */

import { useState, useEffect, useCallback, useMemo } from 'react';
import type {
  TaxonomyResponse,
  TaxonomyRealm,
  TaxonomyTrait,
  TaxonomyArcFamily,
} from '@/app/api/graph/taxonomy/route';

// =============================================================================
// Types
// =============================================================================

export interface UseTaxonomyResult {
  /** Full taxonomy data */
  taxonomy: TaxonomyResponse | null;
  /** Loading state */
  isLoading: boolean;
  /** Error if fetch failed */
  error: Error | null;
  /** Refetch taxonomy data */
  refetch: () => Promise<void>;
  /** Get realm by key */
  getRealm: (key: string) => TaxonomyRealm | undefined;
  /** Get trait by key */
  getTrait: (key: string) => TaxonomyTrait | undefined;
  /** Get arc family by key */
  getArcFamily: (key: string) => TaxonomyArcFamily | undefined;
  /** Get color for an arc family */
  getArcFamilyColor: (familyKey: string) => string;
  /** Get color for a trait */
  getTraitColor: (traitKey: string) => string;
  /** Get border style for a trait */
  getTraitBorderStyle: (traitKey: string) => string | null;
  /** Get layer for a node type */
  getNodeTypeLayer: (nodeType: string) => string | undefined;
}

// =============================================================================
// Cache
// =============================================================================

let taxonomyCache: TaxonomyResponse | null = null;
let fetchPromise: Promise<TaxonomyResponse> | null = null;

// =============================================================================
// Fallback colors (from taxonomy.yaml)
// =============================================================================

const FALLBACK_ARC_FAMILY_COLORS: Record<string, string> = {
  ownership: '#3b82f6',
  localization: '#22c55e',
  semantic: '#f97316',
  generation: '#8b5cf6',
  mining: '#ec4899',
};

const FALLBACK_TRAIT_COLORS: Record<string, string> = {
  invariant: '#3b82f6',
  localized: '#22c55e',
  knowledge: '#8b5cf6',
  derived: '#9ca3af',
  job: '#6b7280',
};

const FALLBACK_TRAIT_BORDER_STYLES: Record<string, string> = {
  invariant: 'solid',
  localized: 'dashed',
  knowledge: 'dotted',
  derived: 'double',
  job: 'none',
};

// =============================================================================
// Fetch function
// =============================================================================

async function fetchTaxonomy(): Promise<TaxonomyResponse> {
  // Return cached data if available
  if (taxonomyCache) {
    return taxonomyCache;
  }

  // Deduplicate concurrent requests
  if (fetchPromise) {
    return fetchPromise;
  }

  fetchPromise = fetch('/api/graph/taxonomy')
    .then((res) => {
      if (!res.ok) {
        throw new Error(`Failed to fetch taxonomy: ${res.status}`);
      }
      return res.json();
    })
    .then((json) => {
      if (!json.success || !json.data) {
        throw new Error('Invalid taxonomy response');
      }
      taxonomyCache = json.data as TaxonomyResponse;
      return taxonomyCache;
    })
    .finally(() => {
      fetchPromise = null;
    });

  return fetchPromise;
}

// =============================================================================
// Hook
// =============================================================================

export function useTaxonomy(): UseTaxonomyResult {
  const [taxonomy, setTaxonomy] = useState<TaxonomyResponse | null>(taxonomyCache);
  const [isLoading, setIsLoading] = useState(!taxonomyCache);
  const [error, setError] = useState<Error | null>(null);

  const refetch = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    taxonomyCache = null; // Clear cache

    try {
      const data = await fetchTaxonomy();
      setTaxonomy(data);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch taxonomy'));
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!taxonomy && !error) {
      fetchTaxonomy()
        .then(setTaxonomy)
        .catch((err) => setError(err instanceof Error ? err : new Error('Failed to fetch taxonomy')))
        .finally(() => setIsLoading(false));
    }
  }, [taxonomy, error]);

  // Memoized lookup maps
  const realmMap = useMemo(() => {
    if (!taxonomy) return new Map<string, TaxonomyRealm>();
    return new Map(taxonomy.realms.map((r) => [r.key, r]));
  }, [taxonomy]);

  const traitMap = useMemo(() => {
    if (!taxonomy) return new Map<string, TaxonomyTrait>();
    return new Map(taxonomy.traits.map((t) => [t.key, t]));
  }, [taxonomy]);

  const arcFamilyMap = useMemo(() => {
    if (!taxonomy) return new Map<string, TaxonomyArcFamily>();
    return new Map(taxonomy.arcFamilies.map((f) => [f.key, f]));
  }, [taxonomy]);

  // Lookup functions
  const getRealm = useCallback(
    (key: string) => realmMap.get(key),
    [realmMap]
  );

  const getTrait = useCallback(
    (key: string) => traitMap.get(key),
    [traitMap]
  );

  const getArcFamily = useCallback(
    (key: string) => arcFamilyMap.get(key),
    [arcFamilyMap]
  );

  const getArcFamilyColor = useCallback(
    (familyKey: string): string => {
      const family = arcFamilyMap.get(familyKey);
      return family?.color ?? FALLBACK_ARC_FAMILY_COLORS[familyKey] ?? '#64748b';
    },
    [arcFamilyMap]
  );

  const getTraitColor = useCallback(
    (traitKey: string): string => {
      const trait = traitMap.get(traitKey);
      return trait?.color ?? FALLBACK_TRAIT_COLORS[traitKey] ?? '#64748b';
    },
    [traitMap]
  );

  const getTraitBorderStyle = useCallback(
    (traitKey: string): string | null => {
      const trait = traitMap.get(traitKey);
      return trait?.border_style ?? FALLBACK_TRAIT_BORDER_STYLES[traitKey] ?? null;
    },
    [traitMap]
  );

  const getNodeTypeLayer = useCallback(
    (nodeType: string): string | undefined => {
      return taxonomy?.nodeTypeMapping[nodeType];
    },
    [taxonomy]
  );

  return {
    taxonomy,
    isLoading,
    error,
    refetch,
    getRealm,
    getTrait,
    getArcFamily,
    getArcFamilyColor,
    getTraitColor,
    getTraitBorderStyle,
    getNodeTypeLayer,
  };
}

// =============================================================================
// Static access (for non-component code)
// =============================================================================

/**
 * Get cached taxonomy data (or null if not loaded)
 */
export function getCachedTaxonomy(): TaxonomyResponse | null {
  return taxonomyCache;
}

/**
 * Clear taxonomy cache (useful for testing or refresh)
 */
export function clearTaxonomyCache(): void {
  taxonomyCache = null;
}
