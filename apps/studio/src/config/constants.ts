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

// =============================================================================
// UI TIMING
// =============================================================================

/** Duration to show "Copied!" feedback (ms) */
export const COPY_FEEDBACK_MS = 2000;

/** Duration to show error/failed feedback (ms) */
export const FEEDBACK_DURATION_MS = 2000;

/** Delay before focusing input after mount (ms) - allows DOM to settle */
export const FOCUS_DELAY_MS = 50;

/** Animation duration for transitions (ms) */
export const TRANSITION_DURATION_MS = 150;

// =============================================================================
// LAYOUT & SIMULATION
// =============================================================================

/** Default margin for graph layout */
export const LAYOUT_MARGIN = 50;

/** Base node radius for collision detection */
export const BASE_NODE_RADIUS = 100;

/** Default simulation iterations */
export const SIMULATION_ITERATIONS = 500;

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
// UI SIZES
// =============================================================================

/** Standard icon sizes */
export const ICON_SIZES = {
  /** Extra small icons */
  xs: 12,
  /** Small icons */
  sm: 14,
  /** Medium icons (default) */
  md: 16,
  /** Large icons */
  lg: 20,
  /** Extra large icons */
  xl: 24,
} as const;

// =============================================================================
// ANIMATION
// =============================================================================

/**
 * Animation timing constants
 * Use these for consistent animation durations across the app
 */
export const ANIMATION = {
  /** Duration classes */
  duration: {
    /** Fast animations (hover, micro-interactions) */
    fast: 'duration-100',
    /** Normal animations (default) */
    normal: 'duration-150',
    /** Slow animations (modals, page transitions) */
    slow: 'duration-200',
    /** Very slow animations (complex sequences) */
    slower: 'duration-300',
  },
  /** Easing classes */
  ease: {
    /** Default easing */
    default: 'ease-out',
    /** Smooth easing for entering */
    in: 'ease-in',
    /** Smooth easing for both */
    inOut: 'ease-in-out',
  },
  /** Common transition combinations */
  transition: {
    /** Color transitions (hover states) */
    colors: 'transition-colors duration-150',
    /** All properties */
    all: 'transition-all duration-150',
    /** Transform only */
    transform: 'transition-transform duration-150',
    /** Opacity */
    opacity: 'transition-opacity duration-150',
  },
} as const;
