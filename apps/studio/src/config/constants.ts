/**
 * Application Constants
 *
 * Centralized magic numbers and configuration values.
 * "When you see a number, give it a name!" - Miss Hoover
 */

// =============================================================================
// DATA FETCHING LIMITS
// =============================================================================

/** Default limit for initial graph fetch */
export const DEFAULT_FETCH_LIMIT = 500;

/** Maximum nodes Neo4j will return (hard cap) */
export const MAX_NODES = 1000;

/** Default limit for Cypher queries (Neo4j Browser style) */
export const DEFAULT_QUERY_LIMIT = 25;

/** Limit for node expansion queries (double-click) */
export const EXPAND_QUERY_LIMIT = 50;

/** Display limit options for graph visualization (v12.1: added 25 for Query-First sync) */
export const DISPLAY_LIMIT_OPTIONS = [25, 50, 100, 200, 250, 500, 1000] as const;

/** Default display limit for graph visualization */
export const DEFAULT_DISPLAY_LIMIT = 250;

// =============================================================================
// PERFORMANCE THRESHOLDS
// =============================================================================

/** Threshold above which simplified edge effects are used */
export const EDGE_SIMPLIFICATION_THRESHOLD = 200;

/** Threshold above which edge animations are disabled */
export const EDGE_ANIMATION_THRESHOLD = 500;

/** Threshold above which node clustering is suggested */
export const NODE_CLUSTERING_THRESHOLD = 300;

/** Maximum concurrent edge animations for 60fps target */
export const MAX_CONCURRENT_EDGE_ANIMATIONS = 50;

// =============================================================================
// UI TIMING
// =============================================================================

/** Duration to show "Copied!" feedback (ms) */
export const COPY_FEEDBACK_MS = 2000;

/** Delay before focusing input after mount (ms) - allows DOM to settle */
export const FOCUS_DELAY_MS = 50;

/** Animation duration for transitions (ms) */
export const TRANSITION_DURATION_MS = 150;

/** Minimum time (ms) to show the matrix executing animation for UX feedback */
export const MIN_EXECUTION_ANIMATION_MS = 2000;

// =============================================================================
// NODE BACKGROUNDS
// =============================================================================

/** Node card background colors (dark theme) */
export const NODE_BG = {
  /** Background when node is selected */
  selected: '#1a1a24',
  /** Background when node is not selected */
  default: '#18181f',
} as const;

// =============================================================================
// ACCENT COLORS
// =============================================================================

/**
 * Linear-dark design system colors
 * Use these instead of hardcoding hex values like '#0d0d12'
 */
export const ACCENT_COLORS = {
  /** Darkest background (modals, dropdowns) */
  darker: '#0d0d12',
  /** Dark background (cards, panels) */
  dark: '#111118',
  /** Base background (main content area) */
  base: '#0a0a0f',
  /** Elevated surface (hover states) */
  elevated: '#16161f',
  /** Subtle surface (borders, dividers as bg) */
  subtle: '#1a1a24',
} as const;

// =============================================================================
// NODE DESIGN SYSTEM
// =============================================================================

/**
 * Shared design tokens for all node components
 * Provides consistency across StructuralNode, SharedLayerNode, SchemaNode, ProjectNode
 */
export const NODE_DESIGN = {
  /** Border radius values */
  radius: {
    /** Outer wrapper radius */
    outer: 14,
    /** Inner card default radius */
    inner: 12,
    /** Inner card selected radius */
    innerSelected: 10,
    /** Circular nodes (SharedLayer) */
    circular: 9999,
  },

  /** Border widths */
  border: {
    /** Default gradient border */
    default: 2,
    /** Selected gradient border */
    selected: 3,
    /** Inner card border when selected */
    innerSelected: 3,
  },

  /** Scale transforms */
  scale: {
    /** Hover scale */
    hover: 1.03,
    /** Selected scale */
    selected: 1.05,
    /** Circular node selected scale */
    circularSelected: 1.10,
    /** Press feedback scale */
    pressed: 0.98,
    /** Circular press scale */
    circularPressed: 0.96,
    /** Dimmed scale */
    dimmed: 0.90,
    /** Circular dimmed scale */
    circularDimmed: 0.75,
  },

  /** Opacity values */
  opacity: {
    /** Dimmed node opacity */
    dimmed: 0.15,
  },

  /** Handle sizes */
  handle: {
    /** Default handle size */
    size: 12,
    /** Small handle size (circular nodes) */
    small: 10,
    /** Handle border width */
    border: 2,
  },

  /** Selected state background with glassmorphism */
  selectedBg: 'rgba(18, 16, 30, 0.92)',

  /** Box shadow configurations */
  shadows: {
    /** Glow shadow for unselected state */
    glow: (color: string) =>
      `0 0 20px 4px ${color}40, 0 0 40px 8px ${color}20`,
    /** Hover glow shadow */
    glowHover: (color: string) =>
      `0 0 30px 6px ${color}50, 0 0 60px 12px ${color}25`,
    /** Selected glow shadow (intense) */
    glowSelected: (color: string) =>
      `0 0 40px 8px ${color}70, 0 0 80px 16px ${color}40, 0 0 120px 24px ${color}20`,
    /** Selection pulse shadow */
    selectionPulse: (color: string) => `0 0 20px ${color}60`,
    /** Selection pulse shadow (delayed) */
    selectionPulseDelayed: (color: string) => `0 0 15px ${color}40`,
    /** Skeuomorphic inner card shadow */
    skeuomorphic: (color: string) => `
      inset 0 2px 0 0 rgba(255, 255, 255, 0.15),
      inset 0 -2px 0 0 rgba(0, 0, 0, 0.4),
      inset 0 0 30px ${color}25,
      0 12px 40px rgba(0, 0, 0, 0.5),
      0 4px 12px rgba(0, 0, 0, 0.4),
      0 0 0 1px rgba(255, 255, 255, 0.05)
    `,
    /** Handle glow when selected */
    handleGlow: (color: string) => `0 0 8px ${color}`,
  },

  /** Gradient configurations */
  gradients: {
    /** Gradient border for selected state */
    borderSelected: (primary: string, secondary: string) =>
      `linear-gradient(135deg, ${primary}, ${secondary}, ${primary})`,
    /** Gradient border for hover state */
    borderHover: (primary: string, secondary: string) =>
      `linear-gradient(135deg, ${primary}, ${secondary})`,
    /** Gradient border for default state */
    borderDefault: (primary: string, secondary: string) =>
      `linear-gradient(135deg, ${primary}, ${secondary}90)`,
    /** Shimmer effect gradient */
    shimmer: `linear-gradient(
      115deg,
      transparent 30%,
      rgba(255, 255, 255, 0.03) 45%,
      rgba(255, 255, 255, 0.06) 50%,
      rgba(255, 255, 255, 0.03) 55%,
      transparent 70%
    )`,
    /** Top bevel highlight */
    bevelHighlight: 'linear-gradient(to right, transparent, rgba(255, 255, 255, 0.2), transparent)',
    /** Top glass reflection */
    glassReflection: 'linear-gradient(to bottom, rgba(255, 255, 255, 0.05), transparent)',
  },

  /** Animation durations (ms) */
  timing: {
    /** Transform transitions */
    transform: 200,
    /** All property transitions */
    all: 300,
    /** Complex transitions */
    complex: 500,
  },
} as const;
