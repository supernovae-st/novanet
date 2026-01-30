/**
 * Color Palette - Single source of truth for all colors
 *
 * All color hex values should be defined here and referenced elsewhere.
 * This prevents color drift and makes theme updates easier.
 *
 * Based on Tailwind CSS color palette with custom additions.
 */

// =============================================================================
// BASE PALETTE (Tailwind-inspired)
// =============================================================================

export const PALETTE = {
  // Violet/Purple family
  violet: {
    400: '#a78bfa',
    500: '#8b5cf6',
    600: '#7c3aed',
    700: '#6d28d9',
  },
  indigo: {
    400: '#818cf8',
    500: '#6366f1',
    600: '#4f46e5',
  },
  purple: {
    400: '#c084fc',
    500: '#a855f7',
    600: '#9333ea',
  },

  // Blue/Cyan family
  blue: {
    400: '#60a5fa',
    500: '#3b82f6',
    600: '#2563eb',
  },
  cyan: {
    400: '#22d3ee',
    500: '#06b6d4',
    600: '#0891b2',
  },

  // Green/Teal family
  emerald: {
    400: '#34d399',
    500: '#10b981',
    600: '#059669',
  },
  green: {
    300: '#86efac',
    400: '#4ade80',
    500: '#22c55e',
    600: '#16a34a',
  },
  teal: {
    400: '#2dd4bf',
    500: '#14b8a6',
    600: '#0d9488',
  },

  // Warm family
  amber: {
    400: '#fbbf24',
    500: '#f59e0b',
    600: '#d97706',
  },
  orange: {
    400: '#fb923c',
    500: '#f97316',
    600: '#ea580c',
  },
  red: {
    400: '#f87171',
    500: '#ef4444',
    600: '#dc2626',
  },

  // Pink/Rose family
  pink: {
    400: '#f472b6',
    500: '#ec4899',
    600: '#db2777',
  },
  rose: {
    400: '#fb7185',
    500: '#f43f5e',
    600: '#e11d48',
  },

  // Neutrals
  white: '#ffffff',
  black: '#000000',
  gray: {
    50: '#f9fafb',
    100: '#f3f4f6',
    200: '#e5e7eb',
    300: '#d1d5db',
    400: '#9ca3af',
    500: '#6b7280',
    600: '#4b5563',
    700: '#374151',
    800: '#1f2937',
    900: '#111827',
  },
} as const;

// =============================================================================
// SEMANTIC COLORS (Application-specific meanings)
// =============================================================================

export const SEMANTIC = {
  // Status colors
  success: PALETTE.emerald[500],
  warning: PALETTE.amber[500],
  error: PALETTE.red[500],
  info: PALETTE.blue[500],

  // Category colors (for node types)
  project: PALETTE.violet[500],
  content: PALETTE.amber[500],
  locale: PALETTE.emerald[500],
  generation: PALETTE.blue[500],
  seo: PALETTE.pink[500],
  geo: PALETTE.indigo[500],
  analytics: PALETTE.teal[500],

  // UI accent colors
  accent: PALETTE.indigo[500],
  accentHover: PALETTE.indigo[600],
  accentLight: PALETTE.indigo[400],
} as const;

// =============================================================================
// GRADIENT PAIRS (Primary + Secondary for node gradients)
// =============================================================================

export const GRADIENTS = {
  // Category gradients
  project: [PALETTE.violet[500], PALETTE.indigo[500]] as const,
  content: [PALETTE.amber[500], PALETTE.orange[500]] as const,
  locale: [PALETTE.emerald[500], PALETTE.green[500]] as const,
  generation: [PALETTE.blue[500], PALETTE.cyan[500]] as const,
  seo: [PALETTE.pink[500], PALETTE.rose[500]] as const,
  geo: [PALETTE.indigo[500], PALETTE.violet[500]] as const,
  analytics: [PALETTE.teal[500], PALETTE.emerald[500]] as const,

  // Special gradients
  success: [PALETTE.emerald[500], PALETTE.green[500]] as const,
  warning: [PALETTE.amber[500], PALETTE.orange[500]] as const,
  error: [PALETTE.red[500], PALETTE.rose[500]] as const,
  info: [PALETTE.blue[500], PALETTE.cyan[500]] as const,
} as const;

// =============================================================================
// TYPE EXPORTS
// =============================================================================

export type PaletteColor = typeof PALETTE;
export type SemanticColor = typeof SEMANTIC;
export type GradientName = keyof typeof GRADIENTS;
