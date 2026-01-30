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

/**
 * Add opacity to hex color (returns hex with alpha channel)
 *
 * @example
 * withOpacity('#8b5cf6', 0.15) // '#8b5cf626' (15% opacity)
 * withOpacity('#8b5cf6', 0.25) // '#8b5cf640' (25% opacity)
 */
export function withOpacity(hex: string, opacity: number): string {
  // Clamp opacity to 0-1
  const clampedOpacity = Math.max(0, Math.min(1, opacity));

  // Convert opacity to hex (0-255 range, then to hex)
  const alphaHex = Math.round(clampedOpacity * 255)
    .toString(16)
    .padStart(2, '0');

  // Remove existing alpha if present (8-char hex)
  const baseHex = hex.replace(/^#/, '').slice(0, 6);

  return `#${baseHex}${alphaHex}`;
}

// =============================================================================
// STYLE HELPERS
// =============================================================================

/**
 * Create background + border style object with consistent opacity
 *
 * @example
 * coloredBadgeStyle('#8b5cf6')
 * // { backgroundColor: 'rgba(139, 92, 246, 0.15)', borderColor: 'rgba(139, 92, 246, 0.25)' }
 */
export function coloredBadgeStyle(color: string): {
  backgroundColor: string;
  borderColor: string;
} {
  return {
    backgroundColor: hexToRgba(color, 0.15),
    borderColor: hexToRgba(color, 0.25),
  };
}

/**
 * Create text + background style for chips/tags
 *
 * @example
 * coloredChipStyle('#8b5cf6')
 * // { color: '#8b5cf6', backgroundColor: 'rgba(139, 92, 246, 0.1)' }
 */
export function coloredChipStyle(color: string): {
  color: string;
  backgroundColor: string;
} {
  return {
    color,
    backgroundColor: hexToRgba(color, 0.1),
  };
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
