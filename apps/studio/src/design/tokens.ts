/**
 * NovaNet Design Tokens
 *
 * Centralized design system tokens for consistent styling across the app.
 * These tokens align with the CSS variables defined in globals.css
 */

// ============================================================================
// SPACING
// ============================================================================

export const spacing = {
  px: '1px',
  0: '0',
  0.5: '0.125rem', // 2px
  1: '0.25rem',    // 4px
  1.5: '0.375rem', // 6px
  2: '0.5rem',     // 8px
  2.5: '0.625rem', // 10px
  3: '0.75rem',    // 12px
  4: '1rem',       // 16px
  5: '1.25rem',    // 20px
  6: '1.5rem',     // 24px
  8: '2rem',       // 32px
  10: '2.5rem',    // 40px
  12: '3rem',      // 48px
  16: '4rem',      // 64px
} as const;

// ============================================================================
// BORDER RADIUS
// ============================================================================

export const radius = {
  none: '0',
  sm: '0.375rem',   // 6px
  md: '0.5rem',     // 8px
  DEFAULT: '0.75rem', // 12px - Our primary radius
  lg: '1rem',       // 16px
  xl: '1.25rem',    // 20px
  '2xl': '1.5rem',  // 24px
  full: '9999px',
} as const;

// ============================================================================
// SHADOWS
// ============================================================================

export const shadows = {
  none: 'none',
  sm: '0 1px 2px 0 rgb(0 0 0 / 0.3)',
  DEFAULT: '0 1px 3px 0 rgb(0 0 0 / 0.3), 0 1px 2px -1px rgb(0 0 0 / 0.3)',
  md: '0 4px 6px -1px rgb(0 0 0 / 0.3), 0 2px 4px -2px rgb(0 0 0 / 0.3)',
  lg: '0 10px 15px -3px rgb(0 0 0 / 0.3), 0 4px 6px -4px rgb(0 0 0 / 0.3)',
  xl: '0 20px 25px -5px rgb(0 0 0 / 0.3), 0 8px 10px -6px rgb(0 0 0 / 0.3)',
  '2xl': '0 25px 50px -12px rgb(0 0 0 / 0.5)',
  // Glass-specific shadows
  glass: '0 25px 50px -12px rgba(0, 0, 0, 0.8), 0 0 0 1px rgba(255, 255, 255, 0.05) inset',
  glow: (color: string) => `0 0 20px ${color}`,
  glowLg: (color: string) => `0 0 40px ${color}, 0 0 80px ${color}`,
} as const;

// ============================================================================
// Z-INDEX SCALE
// ============================================================================

export const zIndex = {
  hide: -1,
  auto: 'auto',
  base: 0,
  dropdown: 100,
  sticky: 200,
  fixed: 300,
  modalBackdrop: 400,
  modal: 450,
  popover: 500,
  tooltip: 600,
  toast: 700,
  overlay: 9999,
} as const;

// ============================================================================
// TIMING & EASING (defined first so transitions can reference)
// ============================================================================

export const durations = {
  faster: 100,  // Micro-interactions (button press feedback)
  fast: 150,
  normal: 200,
  slow: 300,
  slower: 500,
} as const;

export const easing = {
  /** Standard easing for most transitions */
  default: 'ease-out',
  /** Overshoot bounce for playful interactions (buttons, toggles) */
  spring: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
  /** Smooth deceleration for entrances */
  out: 'cubic-bezier(0, 0, 0.2, 1)',
  /** Acceleration for exits */
  in: 'cubic-bezier(0.4, 0, 1, 1)',
  /** Symmetric for reversible animations */
  inOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
} as const;

// Composed transitions (duration + easing) - DRY: references primitives
export const transitions = {
  // Standard transitions (ease-out)
  faster: `${durations.faster}ms ${easing.default}`,
  fast: `${durations.fast}ms ${easing.default}`,
  normal: `${durations.normal}ms ${easing.default}`,
  slow: `${durations.slow}ms ${easing.default}`,
  slower: `${durations.slower}ms ${easing.inOut}`,
  // Spring transitions (bouncy overshoot) - for buttons, toggles
  springFaster: `${durations.faster}ms ${easing.spring}`,
  springFast: `${durations.fast}ms ${easing.spring}`,
  springNormal: `${durations.normal}ms ${easing.spring}`,
} as const;

// ============================================================================
// INTERACTIVE CONTROLS (buttons, toolbars, action bars)
// ============================================================================

export const controls = {
  /** Standard button size (px) - use for toolbar buttons, icon buttons */
  buttonSize: 44,
  buttonSizePx: '44px',
  /** Icon size for Lucide icons (px) */
  iconSize: 18,
  iconSizePx: '18px',
  /** Gap between grouped buttons (px) */
  gap: 8,
  gapPx: '8px',
} as const;

// ============================================================================
// GLASS MORPHISM - Premium Nika-Inspired System
// ============================================================================

export const glass = {
  // Surface colors - 4-step hierarchy (0%, 5%, 8%, 12%, 16%)
  surface: {
    0: 'hsl(0, 0%, 0%)',             // #000000 - Background
    1: 'hsl(240, 8%, 5%)',           // #0D0D10 - Cards, sidebars
    2: 'hsl(240, 6%, 8%)',           // #141418 - Elevated panels
    3: 'hsl(240, 5%, 12%)',          // #1C1C22 - Hover states
    4: 'hsl(240, 5%, 16%)',          // #24242C - Active/pressed
    // Legacy aliases
    base: 'hsl(0, 0%, 0%)',
    hover: 'hsl(240, 5%, 12%)',
  },
  // Background opacity levels (for overlays)
  bg: {
    light: 'rgba(13, 13, 16, 0.6)',
    medium: 'rgba(13, 13, 16, 0.8)',
    heavy: 'rgba(13, 13, 16, 0.95)',
  },
  // Border opacity levels - Calibrated for contrast
  border: {
    subtle: 'rgba(255, 255, 255, 0.08)',
    light: 'rgba(255, 255, 255, 0.12)',
    medium: 'rgba(255, 255, 255, 0.15)',
    heavy: 'rgba(255, 255, 255, 0.20)',
    active: 'rgba(255, 255, 255, 0.30)',
  },
  // Blur intensity
  blur: {
    sm: '8px',
    md: '12px',
    lg: '16px',
    xl: '24px',
  },
  // Inner highlights for depth
  highlight: {
    subtle: 'rgba(255, 255, 255, 0.04)',
    medium: 'rgba(255, 255, 255, 0.06)',
    strong: 'rgba(255, 255, 255, 0.10)',
  },
} as const;

// ============================================================================
// TYPOGRAPHY
// ============================================================================

export const typography = {
  fontFamily: {
    sans: 'var(--font-geist-sans), system-ui, -apple-system, sans-serif',
    mono: 'var(--font-geist-mono), ui-monospace, monospace',
  },
  fontSize: {
    xs: ['0.75rem', { lineHeight: '1rem' }],      // 12px
    sm: ['0.875rem', { lineHeight: '1.25rem' }],  // 14px
    base: ['1rem', { lineHeight: '1.5rem' }],     // 16px
    lg: ['1.125rem', { lineHeight: '1.75rem' }],  // 18px
    xl: ['1.25rem', { lineHeight: '1.75rem' }],   // 20px
    '2xl': ['1.5rem', { lineHeight: '2rem' }],    // 24px
    '3xl': ['1.875rem', { lineHeight: '2.25rem' }], // 30px
    '4xl': ['2.25rem', { lineHeight: '2.5rem' }],   // 36px
  },
  fontWeight: {
    normal: '400',
    medium: '500',
    semibold: '600',
    bold: '700',
  },
} as const;

// ============================================================================
// SEMANTIC COLORS (CSS variable references)
// ============================================================================

export const colors = {
  // Base
  background: 'hsl(var(--background))',
  foreground: 'hsl(var(--foreground))',

  // Primary - NovaNet brand
  primary: {
    DEFAULT: 'hsl(var(--primary))',
    foreground: 'hsl(var(--primary-foreground))',
  },

  // Secondary
  secondary: {
    DEFAULT: 'hsl(var(--secondary))',
    foreground: 'hsl(var(--secondary-foreground))',
  },

  // Muted
  muted: {
    DEFAULT: 'hsl(var(--muted))',
    foreground: 'hsl(var(--muted-foreground))',
  },

  // Accent
  accent: {
    DEFAULT: 'hsl(var(--accent))',
    foreground: 'hsl(var(--accent-foreground))',
  },

  // Semantic
  destructive: {
    DEFAULT: 'hsl(var(--destructive))',
    foreground: 'hsl(var(--destructive-foreground))',
  },
  success: {
    DEFAULT: 'hsl(var(--success))',
    foreground: 'hsl(var(--success-foreground))',
  },
  warning: {
    DEFAULT: 'hsl(var(--warning))',
    foreground: 'hsl(var(--warning-foreground))',
  },

  // Border & Input
  border: 'hsl(var(--border))',
  input: 'hsl(var(--input))',
  ring: 'hsl(var(--ring))',
} as const;

// ============================================================================
// TAILWIND CLASS HELPERS
// ============================================================================

/**
 * Glass morphism utility classes - Premium Nika-Inspired
 */
export const glassClasses = {
  subtle: 'bg-[hsl(240,8%,5%)] backdrop-blur-sm border border-white/8',
  light: 'bg-[hsl(240,8%,5%)] backdrop-blur-md border border-white/12',
  medium: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/12',
  heavy: 'bg-[hsl(240,5%,12%)] backdrop-blur-xl border border-white/15',
  floating: 'bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/15 rounded-2xl shadow-2xl shadow-black/50 ring-1 ring-white/4 ring-inset',
} as const;

/**
 * Common button variant classes - HIGH CONTRAST
 */
export const buttonClasses = {
  base: 'inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200',
  ghost: 'text-white/60 hover:bg-white/10 hover:text-white',
  outline: 'border border-white/20 bg-white/8 hover:bg-white/15 text-white/90',
  primary: 'bg-primary text-primary-foreground hover:bg-primary/90 shadow-lg shadow-primary/30',
} as const;

/**
 * Text opacity utilities - Premium contrast levels
 *
 * Based on research from Nika Draft, Linear, Raycast:
 * - Primary: 95% (headings, important content)
 * - Secondary: 80% (body text, readable)
 * - Muted: 65% (captions, hints)
 * - Disabled: 45% (non-interactive)
 */
export const textOpacity = {
  disabled: 'text-white/45',
  muted: 'text-white/65',
  secondary: 'text-white/80',
  primary: 'text-white/95',
  full: 'text-white',
} as const;

// ============================================================================
// EXPORT ALL
// ============================================================================

export const tokens = {
  spacing,
  radius,
  shadows,
  zIndex,
  durations,
  easing,
  transitions,
  controls,
  glass,
  typography,
  colors,
  glassClasses,
  buttonClasses,
  textOpacity,
} as const;

export default tokens;
