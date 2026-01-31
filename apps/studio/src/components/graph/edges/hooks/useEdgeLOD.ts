'use client';

/**
 * useEdgeLOD - Hook for Level of Detail management
 *
 * Calculates appropriate LOD tier based on distance, zoom, and edge state
 */

import { useMemo, useCallback } from 'react';
import type { LODTier, EffectPrimitive } from '../system/types';
import {
  calculateLODTier,
  filterEffectsForLOD,
  lodManager,
} from '../system/performance/LODController';

export interface UseEdgeLODOptions {
  /** Distance from viewport center */
  distanceFromCenter: number;
  /** Current zoom level */
  zoom: number;
  /** Whether edge is selected */
  isSelected?: boolean;
  /** Whether edge is highlighted */
  isHighlighted?: boolean;
  /** Whether connected to selected node */
  isConnectedToSelected?: boolean;
  /** Force specific LOD tier */
  forceTier?: LODTier;
}

export interface UseEdgeLODResult {
  /** Current LOD tier */
  tier: LODTier;
  /** Filter effects array based on LOD */
  filterEffects: (effects: EffectPrimitive[]) => EffectPrimitive[];
  /** Whether should render at all */
  shouldRender: boolean;
  /** Intensity multiplier based on LOD */
  intensityMultiplier: number;
}

/**
 * Hook to manage LOD for a single edge
 */
export function useEdgeLOD(options: UseEdgeLODOptions): UseEdgeLODResult {
  const {
    distanceFromCenter,
    zoom,
    isSelected = false,
    isHighlighted = false,
    isConnectedToSelected = false,
    forceTier,
  } = options;

  const tier = useMemo(() => {
    if (forceTier) return forceTier;

    return calculateLODTier(
      distanceFromCenter,
      zoom,
      isSelected,
      isHighlighted,
      isConnectedToSelected
    );
  }, [forceTier, distanceFromCenter, zoom, isSelected, isHighlighted, isConnectedToSelected]);

  const filterEffects = useCallback(
    (effects: EffectPrimitive[]) => filterEffectsForLOD(effects, tier),
    [tier]
  );

  const shouldRender = useMemo(() => {
    // Always render selected/highlighted
    if (isSelected || isHighlighted) return true;

    // For minimal LOD, don't render effects (but edge stroke still renders)
    return tier !== 'minimal';
  }, [tier, isSelected, isHighlighted]);

  const intensityMultiplier = useMemo(() => {
    switch (tier) {
      case 'high':
        return 1.0;
      case 'medium':
        return 0.8;
      case 'low':
        return 0.5;
      case 'minimal':
        return 0.3;
    }
  }, [tier]);

  return {
    tier,
    filterEffects,
    shouldRender,
    intensityMultiplier,
  };
}

/**
 * Hook to get global LOD statistics
 * Returns fresh stats on every call (no memoization - stats change frequently)
 */
export function useLODStats() {
  return lodManager.getStats();
}

