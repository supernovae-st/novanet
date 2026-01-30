'use client';

/**
 * useCenterOnNode - Center viewport on a specific node
 *
 * Calculates offset compensation for UI overlays and provides
 * a function to smoothly center + zoom on a node.
 *
 * The offset formula:
 * - x = (left - right) / 2 (shift to center horizontally)
 * - y = (top - bottom) / 2 (shift to center vertically)
 *
 * @version 1.1.0 - Now uses shared calculateRawInsets from layoutConstants
 */

import { useCallback } from 'react';
import { useReactFlow } from '@xyflow/react';
import { useUIStore } from '@/stores';
import {
  calculateRawInsets,
  type UIState,
} from '@/config/layoutConstants';

// =============================================================================
// Types
// =============================================================================

export interface CenterOffset {
  /** Horizontal offset in pixels (positive = shift right, negative = shift left) */
  x: number;
  /** Vertical offset in pixels (positive = shift down, negative = shift up) */
  y: number;
}

export interface CenterOnNodeOptions {
  /** Target zoom level (default: max of current zoom or 1.2) */
  zoom?: number;
  /** Animation duration in ms (default: 400) */
  duration?: number;
}

// =============================================================================
// Pure calculation function (testable)
// =============================================================================

/**
 * Calculate the offset needed to center content in the visible area.
 *
 * The visible area is the viewport minus the UI overlays.
 * We need to shift the center point by half the difference between
 * left/right and top/bottom insets.
 *
 * Uses shared calculateRawInsets for DRY compliance.
 *
 * Example with sidebar (304px) and panel (436px):
 * - x offset = (304 - 436) / 2 = -66px (shift left because right panel is wider)
 * - y offset = (136 - 96) / 2 = 20px (shift down because top bar is taller)
 */
export function calculateCenterOffset(state: UIState): CenterOffset {
  // Focus mode: symmetric insets, no offset needed
  if (state.focusMode) {
    return { x: 0, y: 0 };
  }

  // Get raw insets from shared function
  const insets = calculateRawInsets(state);

  // Offset = (left - right) / 2 for horizontal centering
  // Positive x = shift right, Negative x = shift left
  // Positive y = shift down, Negative y = shift up
  return {
    x: (insets.left - insets.right) / 2,
    y: (insets.top - insets.bottom) / 2,
  };
}

// =============================================================================
// React Hook
// =============================================================================

/**
 * Hook that returns a function to center the viewport on a specific node,
 * accounting for UI overlays (sidebar, details panel, top/bottom bars).
 *
 * @returns Object with centerOnNode function
 */
export function useCenterOnNode() {
  const { setCenter, getZoom } = useReactFlow();

  // NOTE: We intentionally read state directly from store inside the callback
  // instead of using a selector. This avoids stale closure issues when
  // centerOnNode is called immediately after setSelectedNode (before React re-renders).

  const centerOnNode = useCallback(
    (
      nodeX: number,
      nodeY: number,
      nodeWidth: number,
      nodeHeight: number,
      options?: CenterOnNodeOptions
    ) => {
      // FIX: Read fresh state directly from store to avoid stale closure
      // This ensures we always use the latest UI state, even when called
      // immediately after setSelectedNode (before React re-renders)
      const freshState = useUIStore.getState();
      const freshHasSelection = freshState.selectedNodeId !== null || freshState.selectedEdgeId !== null;

      const offset = calculateCenterOffset({
        sidebarOpen: freshState.sidebarOpen,
        focusMode: freshState.focusMode,
        hasSelection: freshHasSelection,
      });

      const currentZoom = getZoom();
      const targetZoom = options?.zoom ?? Math.max(currentZoom, 1.2);
      const duration = options?.duration ?? 400;

      // Node center position
      const nodeCenterX = nodeX + nodeWidth / 2;
      const nodeCenterY = nodeY + nodeHeight / 2;

      // Compensate for UI overlays by adjusting the center point
      // We need to convert pixel offset to flow coordinates (divide by zoom)
      // SUBTRACT offset because: if panel is wider than sidebar (offset.x < 0),
      // we need to shift the view RIGHT so the node appears LEFT of window center
      const adjustedX = nodeCenterX - offset.x / targetZoom;
      const adjustedY = nodeCenterY - offset.y / targetZoom;

      setCenter(adjustedX, adjustedY, {
        zoom: targetZoom,
        duration,
      });
    },
    [setCenter, getZoom] // Removed state dependencies - now reads fresh from store
  );

  return { centerOnNode };
}

// Note: LAYOUT_CONSTANTS is re-exported from useViewportInsets to avoid duplicates
export type { UIState };
