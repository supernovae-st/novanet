'use client';

/**
 * useViewportInsets - Dynamic viewport padding for fitView
 *
 * Calculates pixel-based insets based on current UI state:
 * - Sidebar open/closed
 * - Details panel open/closed (node/edge selected)
 * - Focus mode (all UI hidden)
 *
 * Uses React Flow 12.5.0+ advanced padding syntax with pixels.
 *
 * @version 1.1.0 - Now uses shared calculateRawInsets from layoutConstants
 */

import { useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useUIStore } from '@/stores';
import {
  calculateRawInsets,
  LAYOUT_CONSTANTS,
  type UIState,
  type RawInsets,
} from '@/config/layoutConstants';

// =============================================================================
// Types
// =============================================================================

export interface ViewportInsets {
  /** Top padding in pixels */
  top: string;
  /** Right padding in pixels */
  right: string;
  /** Bottom padding in pixels */
  bottom: string;
  /** Left padding in pixels */
  left: string;
}

export interface FitViewConfig {
  /** Padding object for fitView */
  padding: ViewportInsets;
  /** Animation duration in ms */
  duration: number;
  /** Max zoom level */
  maxZoom: number;
}

// =============================================================================
// Helper: Convert raw insets to pixel strings
// =============================================================================

/**
 * Convert raw numeric insets to pixel string format for React Flow.
 */
function toPixelStrings(raw: RawInsets): ViewportInsets {
  return {
    top: `${raw.top}px`,
    right: `${raw.right}px`,
    bottom: `${raw.bottom}px`,
    left: `${raw.left}px`,
  };
}

// =============================================================================
// Hook
// =============================================================================

/**
 * Returns dynamic viewport insets based on current UI state.
 *
 * @returns ViewportInsets - { top, right, bottom, left } in pixel strings
 */
export function useViewportInsets(): ViewportInsets {
  // PERF: Calculate hasSelection in selector to avoid re-renders when
  // changing selected node (A→B) - only re-render when hasSelection toggles
  const { sidebarOpen, focusMode, hasSelection } = useUIStore(
    useShallow((state) => ({
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      hasSelection: state.selectedNodeId !== null || state.selectedEdgeId !== null,
    }))
  );

  return useMemo(() => {
    const raw = calculateRawInsets({ sidebarOpen, focusMode, hasSelection });
    return toPixelStrings(raw);
  }, [focusMode, sidebarOpen, hasSelection]);
}

/**
 * Returns complete fitView configuration with insets and animation.
 *
 * @param options - Override options
 * @returns FitViewConfig - Ready to spread into fitView()
 */
export function useFitViewConfig(options?: {
  duration?: number;
  maxZoom?: number;
}): FitViewConfig {
  const padding = useViewportInsets();

  return useMemo(
    () => ({
      padding,
      duration: options?.duration ?? 400,
      maxZoom: options?.maxZoom ?? 1.5,
    }),
    [padding, options?.duration, options?.maxZoom]
  );
}

// =============================================================================
// Standalone utility for non-hook contexts
// =============================================================================

/**
 * Pure function to calculate insets (for use outside React components).
 * Uses shared calculateRawInsets and converts to pixel strings.
 */
export function calculateViewportInsets(state: UIState): ViewportInsets {
  const raw = calculateRawInsets(state);
  return toPixelStrings(raw);
}

// Re-export for backwards compatibility
export { LAYOUT_CONSTANTS };
export type { UIState };
