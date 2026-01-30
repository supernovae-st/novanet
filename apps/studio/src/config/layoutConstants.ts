/**
 * Layout Constants - Shared dimensions for viewport calculations
 *
 * These constants define the pixel dimensions of UI overlays
 * that affect the visible graph area.
 *
 * Used by:
 * - useViewportInsets (dynamic padding for fitView)
 * - useCenterOnNode (offset compensation for centering)
 * - Graph2D (minimap and toolbar positioning)
 *
 * @version 1.0.0
 */

// =============================================================================
// Base Spacing
// =============================================================================

/** Minimum breathing room around content (pixels) */
export const BASE_MARGIN = 16;

// =============================================================================
// Top Bar Dimensions
// =============================================================================

/** QueryPill height */
export const QUERY_PILL_HEIGHT = 48;

/** Gap between QueryPill and Stats row */
export const TOP_BAR_GAP = 12;

/** Stats row height */
export const STATS_ROW_HEIGHT = 44;

/** Additional gap below stats */
export const TOP_BAR_BOTTOM_GAP = 16;

/** Total top bar height: QueryPill (48) + gap (12) + Stats (44) + gap (16) = 120px */
export const TOP_BAR_HEIGHT = QUERY_PILL_HEIGHT + TOP_BAR_GAP + STATS_ROW_HEIGHT + TOP_BAR_BOTTOM_GAP;

// =============================================================================
// Bottom Bar Dimensions
// =============================================================================

/** View controls pill height */
export const VIEW_CONTROLS_HEIGHT = 44;

/** Padding around bottom controls */
export const BOTTOM_BAR_PADDING = 16;

/** Additional margin from edge */
export const BOTTOM_BAR_MARGIN = 20;

/** Total bottom bar height: Controls (44) + padding (16) + margin (20) = 80px */
export const BOTTOM_BAR_HEIGHT = VIEW_CONTROLS_HEIGHT + BOTTOM_BAR_PADDING + BOTTOM_BAR_MARGIN;

// =============================================================================
// Sidebar Dimensions
// =============================================================================

/** Left sidebar width: w-72 = 18rem = 288px */
export const SIDEBAR_WIDTH = 288;

// =============================================================================
// Details Panel Dimensions
// =============================================================================

/** Right details panel width: w-[420px] */
export const DETAILS_PANEL_WIDTH = 420;

// =============================================================================
// Minimap & Toolbar Dimensions
// =============================================================================

/** Minimap width (React Flow default) */
export const MINIMAP_WIDTH = 200;

/** Minimap height */
export const MINIMAP_HEIGHT = 160;

/** Toolbar bottom offset from viewport edge */
export const TOOLBAR_BOTTOM_OFFSET = 184;

// =============================================================================
// Aggregated Constants for Hooks
// =============================================================================

/**
 * All layout constants as a single object for easy import.
 * Used by useViewportInsets and useCenterOnNode hooks.
 */
export const LAYOUT_CONSTANTS = {
  BASE_MARGIN,
  TOP_BAR_HEIGHT,
  BOTTOM_BAR_HEIGHT,
  SIDEBAR_WIDTH,
  DETAILS_PANEL_WIDTH,
  MINIMAP_WIDTH,
  MINIMAP_HEIGHT,
  TOOLBAR_BOTTOM_OFFSET,
} as const;

export type LayoutConstants = typeof LAYOUT_CONSTANTS;

// =============================================================================
// Shared Types
// =============================================================================

/** UI state for inset calculations */
export interface UIState {
  sidebarOpen: boolean;
  focusMode: boolean;
  hasSelection: boolean;
}

/** Raw insets in pixels (numbers, not strings) */
export interface RawInsets {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

// =============================================================================
// Pre-computed Insets Lookup Table (PERF)
// =============================================================================

/**
 * Pre-computed insets for all 8 possible UI state combinations.
 * Key format: `${sidebarOpen}-${focusMode}-${hasSelection}`
 *
 * Benefits:
 * - Zero allocation at runtime (returns same object reference)
 * - O(1) lookup vs O(n) calculation
 * - Deterministic and cache-friendly
 */

// Focus mode insets (same for all combinations when focusMode=true)
const FOCUS_INSETS: RawInsets = {
  top: BASE_MARGIN,
  right: BASE_MARGIN,
  bottom: BASE_MARGIN,
  left: BASE_MARGIN,
} as const;

// Pre-computed normal mode insets
const INSETS_LOOKUP: Record<string, RawInsets> = {
  // Focus mode (4 combinations - all return same insets)
  'true-true-true': FOCUS_INSETS,
  'true-true-false': FOCUS_INSETS,
  'false-true-true': FOCUS_INSETS,
  'false-true-false': FOCUS_INSETS,

  // Normal mode: sidebar=true, hasSelection=true
  'true-false-true': {
    top: TOP_BAR_HEIGHT + BASE_MARGIN,      // 136
    right: DETAILS_PANEL_WIDTH + BASE_MARGIN, // 436
    bottom: BOTTOM_BAR_HEIGHT + BASE_MARGIN,  // 96
    left: SIDEBAR_WIDTH + BASE_MARGIN,        // 304
  },

  // Normal mode: sidebar=true, hasSelection=false
  'true-false-false': {
    top: TOP_BAR_HEIGHT + BASE_MARGIN,      // 136
    right: BASE_MARGIN,                      // 16
    bottom: BOTTOM_BAR_HEIGHT + BASE_MARGIN, // 96
    left: SIDEBAR_WIDTH + BASE_MARGIN,       // 304
  },

  // Normal mode: sidebar=false, hasSelection=true
  'false-false-true': {
    top: TOP_BAR_HEIGHT + BASE_MARGIN,      // 136
    right: DETAILS_PANEL_WIDTH + BASE_MARGIN, // 436
    bottom: BOTTOM_BAR_HEIGHT + BASE_MARGIN,  // 96
    left: BASE_MARGIN,                        // 16
  },

  // Normal mode: sidebar=false, hasSelection=false
  'false-false-false': {
    top: TOP_BAR_HEIGHT + BASE_MARGIN,      // 136
    right: BASE_MARGIN,                      // 16
    bottom: BOTTOM_BAR_HEIGHT + BASE_MARGIN, // 96
    left: BASE_MARGIN,                       // 16
  },
} as const;

// =============================================================================
// Pure Calculation Functions
// =============================================================================

/**
 * Get raw viewport insets based on UI state.
 * Uses pre-computed lookup table for O(1) performance with zero allocation.
 *
 * Used by:
 * - useViewportInsets (converts to strings for React Flow)
 * - useCenterOnNode (uses numbers for offset math)
 *
 * @param state - Current UI state
 * @returns RawInsets - { top, right, bottom, left } in pixels
 */
export function calculateRawInsets(state: UIState): RawInsets {
  const key = `${state.sidebarOpen}-${state.focusMode}-${state.hasSelection}`;
  return INSETS_LOOKUP[key];
}
