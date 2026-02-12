/**
 * Graph Layout Configuration
 *
 * Centralized configuration for graph layout algorithms.
 * Extracted from Graph2D.tsx magic numbers.
 */

// =============================================================================
// DAGRE LAYOUT (Hierarchical)
// =============================================================================

export const DAGRE_CONFIG = {
  /** Node width for layout calculation (px) */
  NODE_WIDTH: 240,
  /** Node height for layout calculation (px) */
  NODE_HEIGHT: 140,
  /** Vertical separation between ranks/levels (px) */
  RANK_SEP: 250,
  /** Horizontal separation between nodes in same rank (px) */
  NODE_SEP: 150,
} as const;

// =============================================================================
// FORCE SIMULATION (Pure force-directed layout)
// =============================================================================

export const FORCE_CONFIG = {
  /** Repulsion strength between nodes (negative = repel, higher magnitude = more spread) */
  CHARGE_STRENGTH: -1500,
  /** Ideal distance between connected nodes (px) */
  LINK_DISTANCE: 280,
  /** Multiplier for collision detection radius (node radii) */
  COLLISION_RADIUS: 3.0,
  /** Gentle center pull strength */
  CENTER_STRENGTH: 0.03,
  /** Rate at which simulation cools down (lower = slower, more stable) */
  ALPHA_DECAY: 0.008,
  /** Friction/drag on node movement (0-1, higher = more friction) */
  VELOCITY_DECAY: 0.25,
  /** Number of simulation iterations to run synchronously */
  ITERATIONS: 1000,
} as const;

// =============================================================================
// RADIAL LAYOUT
// =============================================================================

export const RADIAL_CONFIG = {
  /** Minimum radius for radial layout (px) */
  MIN_RADIUS: 250,
  /** Radius scaling factor per node */
  RADIUS_PER_NODE: 20,
} as const;

// =============================================================================
// COMBINED LAYOUT (Dagre + Force refinement)
// =============================================================================

export const COMBINED_LAYOUT_CONFIG = {
  /** Charge strength for combined layout (weaker than pure force) */
  CHARGE_STRENGTH: -1000,
  /** Link distance for combined layout */
  LINK_DISTANCE: 320,
  /** Collision radius for combined layout */
  COLLISION_RADIUS: 2.8,
  /** Center strength for combined layout */
  CENTER_STRENGTH: 0.015,
  /** Alpha decay for combined layout (faster settling) */
  ALPHA_DECAY: 0.010,
  /** Velocity decay for combined layout */
  VELOCITY_DECAY: 0.30,
  /** Iterations for combined layout simulation */
  ITERATIONS: 700,
} as const;

// =============================================================================
// INITIAL POSITIONING
// =============================================================================

export const INITIAL_POSITION_CONFIG = {
  /** Fallback grid spacing when layout fails (px) */
  FALLBACK_GRID_SPACING: 300,
  /** Spiral radius for initial force layout positioning (px) */
  SPIRAL_RADIUS: 400,
  /** Random jitter for spiral positioning (px) */
  SPIRAL_JITTER: 150,
  /** Spiral angle increment per node (radians) */
  SPIRAL_ANGLE_INCREMENT: 0.4,
} as const;

// =============================================================================
// ANIMATION & INTERACTION (2D/3D unified)
// =============================================================================

export const GRAPH_ANIMATION = {
  // -------------------------------------------------------------------------
  // 2D Animation Timings (React Flow)
  // -------------------------------------------------------------------------
  /** Duration for fitView animation (ms) */
  FIT_VIEW_DURATION: 400,
  /** Minimum zoom level for fitView */
  MIN_ZOOM: 0.1,
  /** Maximum zoom level for fitView */
  MAX_ZOOM: 2,
  /** Default zoom when centering on node via keyboard */
  DEFAULT_CENTER_ZOOM: 0.8,
  /** Zoom level when double-clicking a node */
  NODE_DOUBLE_CLICK_ZOOM: 1.3,
  /** Delay before auto-fitView on UI changes (ms) */
  UI_CHANGE_DELAY: 50,
  /** Delay before initial fitView on mount (ms) */
  INITIAL_FIT_DELAY: 100,

  // -------------------------------------------------------------------------
  // 3D Animation Timings (Three.js / react-force-graph-3d)
  // Slightly longer than 2D for cinematic feel in 3D space
  // -------------------------------------------------------------------------
  /** Duration for 3D fitView/zoomToFit animation (ms) */
  FIT_VIEW_DURATION_3D: 600,
  /** Duration for 3D node focus zoom animation (ms) */
  NODE_FOCUS_DURATION_3D: 800,
  /** Duration for 3D camera reset animation (ms) - faster for responsiveness */
  RESET_DURATION_3D: 400,
  /** Duration for 3D edge midpoint zoom animation (ms) */
  EDGE_FOCUS_DURATION_3D: 600,
  /** Camera distance from node when focused */
  NODE_FOCUS_DISTANCE_3D: 180,
  /** Camera distance from edge midpoint when focused */
  EDGE_FOCUS_DISTANCE_3D: 200,
} as const;
