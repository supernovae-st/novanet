'use client';

/**
 * useEdgeTheme - Hook for resolving edge themes
 *
 * Provides memoized theme resolution based on relation type and state
 */

import { useMemo } from 'react';
import type { EdgeState, ResolvedEdgeTheme } from '../system/types';
import { resolveThemeCached, getCategory } from '../system/registry';

export interface UseEdgeThemeOptions {
  /** Current edge state */
  state?: EdgeState;
  /** Whether edge is selected */
  isSelected?: boolean;
  /** Whether edge is highlighted (hover/connected) */
  isHighlighted?: boolean;
}

export interface UseEdgeThemeResult {
  /** Resolved theme with colors, timing, effects */
  theme: ResolvedEdgeTheme;
  /** Category of this relation */
  category: string;
  /** Whether theme has been overridden from category default */
  hasOverride: boolean;
}

/**
 * Hook to get resolved theme for an edge
 */
export function useEdgeTheme(
  relationType: string,
  options: UseEdgeThemeOptions = {}
): UseEdgeThemeResult {
  const { isSelected = false, isHighlighted = false } = options;

  const theme = useMemo(() => {
    return resolveThemeCached(relationType, {
      isSelected,
      isHovered: isHighlighted,
    });
  }, [relationType, isSelected, isHighlighted]);

  const category = useMemo(() => {
    return getCategory(relationType);
  }, [relationType]);

  const hasOverride = useMemo(() => {
    // Check if this specific relation has custom overrides
    return theme.relationType !== undefined && theme.category !== undefined;
  }, [theme]);

  return {
    theme,
    category,
    hasOverride,
  };
}

/**
 * Hook to get just the colors from a theme
 */
export function useEdgeColors(relationType: string, state?: EdgeState) {
  const { theme } = useEdgeTheme(relationType, { state });
  return theme.colors;
}

/**
 * Hook to get just the timing from a theme
 */
export function useEdgeTiming(relationType: string, state?: EdgeState) {
  const { theme } = useEdgeTheme(relationType, { state });
  return theme.timing;
}

/**
 * Hook to get just the effects from a theme
 */
export function useEdgeEffects(relationType: string, state?: EdgeState) {
  const { theme } = useEdgeTheme(relationType, { state });
  return theme.effects;
}
