/**
 * Color Utilities
 *
 * Type-safe color manipulation functions for consistent styling.
 * Works with hex colors from palette.ts.
 */

import { logger } from './logger';

// =============================================================================
// TYPES
// =============================================================================

/** Hex color string (e.g., "#8b5cf6") */
export type HexColor = `#${string}`;

// =============================================================================
// CONVERSION FUNCTIONS
// =============================================================================

/**
 * Convert hex color to RGB components
 *
 * @example
 * hexToRgb('#8b5cf6') // { r: 139, g: 92, b: 246 }
 */
export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
  // Remove # if present
  const cleanHex = hex.replace(/^#/, '');

  // Validate hex format
  if (!/^[0-9A-Fa-f]{6}$/.test(cleanHex)) {
    return null;
  }

  return {
    r: parseInt(cleanHex.slice(0, 2), 16),
    g: parseInt(cleanHex.slice(2, 4), 16),
    b: parseInt(cleanHex.slice(4, 6), 16),
  };
}

/**
 * Convert hex color to RGBA string
 *
 * @example
 * hexToRgba('#8b5cf6', 0.5) // 'rgba(139, 92, 246, 0.5)'
 */
export function hexToRgba(hex: string, alpha: number): string {
  const rgb = hexToRgb(hex);
  if (!rgb) {
    logger.warn('ColorUtils', `Invalid hex color: ${hex}`);
    return `rgba(0, 0, 0, ${alpha})`;
  }
  return `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, ${alpha})`;
}

// =============================================================================
// OPACITY PRESETS (common values used in the app)
// =============================================================================

export const OPACITY = {
  /** Very subtle background (5%) */
  SUBTLE: 0.05,
  /** Light background (10%) */
  LIGHT: 0.1,
  /** Medium background (15%) */
  MEDIUM: 0.15,
  /** Border/divider (25%) */
  BORDER: 0.25,
  /** Hover state (30%) */
  HOVER: 0.3,
  /** Active/pressed state (40%) */
  ACTIVE: 0.4,
  /** Semi-transparent (50%) */
  HALF: 0.5,
} as const;
