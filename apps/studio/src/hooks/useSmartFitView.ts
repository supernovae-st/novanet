'use client';

/**
 * useSmartFitView - Intelligent fitView with dynamic insets
 *
 * Wraps React Flow's fitView with automatic inset calculation
 * based on current UI state (sidebar, details panel, focus mode).
 *
 * @version 1.0.0
 */

import { useCallback } from 'react';
import { useReactFlow, type FitViewOptions } from '@xyflow/react';
import { useViewportInsets, calculateViewportInsets, type ViewportInsets } from './useViewportInsets';
import { useUIStore } from '@/stores';

// =============================================================================
// Types
// =============================================================================

export interface SmartFitViewOptions {
  /** Animation duration in ms (default: 400) */
  duration?: number;
  /** Maximum zoom level (default: 1.5) */
  maxZoom?: number;
  /** Minimum zoom level (default: 0.1) */
  minZoom?: number;
  /** Include hidden nodes in fit calculation (default: false) */
  includeHiddenNodes?: boolean;
}

export interface UseSmartFitViewReturn {
  /** Call to fit view with dynamic insets */
  smartFitView: (options?: SmartFitViewOptions) => void;
  /** Current viewport insets */
  insets: ViewportInsets;
}

// =============================================================================
// Hook
// =============================================================================

/**
 * Hook that wraps React Flow's fitView with automatic inset calculation.
 *
 * @returns Object with smartFitView function and current insets
 *
 * @example
 * ```tsx
 * const { smartFitView } = useSmartFitView();
 *
 * // Fit with defaults (400ms, maxZoom 1.5)
 * smartFitView();
 *
 * // Custom options
 * smartFitView({ duration: 200, maxZoom: 2 });
 * ```
 */
export function useSmartFitView(): UseSmartFitViewReturn {
  const { fitView } = useReactFlow();

  // NOTE: We still use the hook for the return value (for consumers that need it),
  // but the callback reads fresh state directly from the store.
  const insets = useViewportInsets();

  const smartFitView = useCallback(
    (options?: SmartFitViewOptions) => {
      // FIX: Read fresh state directly from store to avoid stale closure
      // This ensures we always use the latest UI state
      const freshState = useUIStore.getState();
      const freshHasSelection = freshState.selectedNodeId !== null || freshState.selectedEdgeId !== null;

      const freshInsets = calculateViewportInsets({
        sidebarOpen: freshState.sidebarOpen,
        focusMode: freshState.focusMode,
        hasSelection: freshHasSelection,
      });

      // Cast insets to FitViewOptions['padding'] since our string format
      // matches React Flow's PaddingWithUnit type (`${number}px`)
      fitView({
        padding: freshInsets as FitViewOptions['padding'],
        duration: options?.duration ?? 400,
        maxZoom: options?.maxZoom ?? 1.5,
        minZoom: options?.minZoom ?? 0.1,
        includeHiddenNodes: options?.includeHiddenNodes ?? false,
      });
    },
    [fitView] // Removed insets dependency - now reads fresh from store
  );

  return { smartFitView, insets };
}
