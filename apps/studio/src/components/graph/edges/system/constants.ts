/**
 * Edge Animation System - Constants
 *
 * Centralized timing, sizing, and configuration values.
 * All magic numbers live here.
 */

import type {
  AnimationSpeed,
  LODTier,
  LODConfig,
  AnimationBudgetConfig,
  EdgePriority,
  EffectPrimitive,
} from './types';

// =============================================================================
// Animation Timing
// =============================================================================

/**
 * Base animation durations in seconds per speed preset
 */
export const ANIMATION_DURATIONS: Record<AnimationSpeed, number> = {
  slow: 5,
  normal: 3,
  fast: 1.5,
  ultra: 0.8,
} as const;

/**
 * Speed boost multiplier when edge is selected (< 1 = faster)
 */
export const SELECTED_SPEED_BOOST = 0.6;

/**
 * Speed boost multiplier when edge is hovered
 */
export const HOVERED_SPEED_BOOST = 0.8;

/**
 * Default stagger ratio between particles
 */
export const DEFAULT_STAGGER = 0.15;

/**
 * Default easing (SVG spline format)
 */
export const DEFAULT_EASING = '0.25 0.1 0.25 1';

/**
 * Easing presets
 */
export const EASING_PRESETS = {
  linear: '0 0 1 1',
  easeIn: '0.42 0 1 1',
  easeOut: '0 0 0.58 1',
  easeInOut: '0.42 0 0.58 1',
  energetic: '0.175 0.885 0.32 1.1',
  bounce: '0.68 -0.55 0.265 1.55',
} as const;

// =============================================================================
// Particle Sizing
// =============================================================================

/**
 * Base particle sizes
 */
export const PARTICLE_SIZES = {
  /** Default particle radius */
  base: 6,
  /** Particle radius when highlighted */
  highlighted: 9,
  /** Leader particle multiplier */
  leaderMultiplier: 1.4,
  /** Trail particle multiplier */
  trailMultiplier: 0.7,
  /** Minimum visible size */
  minimum: 2,
} as const;

/**
 * Particle counts per preset
 */
export const PARTICLE_COUNTS = {
  flow: 3,
  pulse: 2,
  wave: 5,
  spark: 4,
  plasma: 5,
  helix: 8,
  orbit: 4,
  aurora: 4,
} as const;

/**
 * Glow configuration
 */
export const GLOW_CONFIG = {
  /** Base blur radius */
  blurRadius: 8,
  /** Intensity multiplier for highlighted state */
  highlightedMultiplier: 1.5,
  /** Minimum opacity */
  minOpacity: 0.3,
  /** Maximum opacity */
  maxOpacity: 0.9,
} as const;

// =============================================================================
// Edge Geometry
// =============================================================================

/**
 * Edge stroke configuration
 */
export const STROKE_CONFIG = {
  /** Base stroke width */
  baseWidth: 2,
  /** Highlighted stroke width multiplier */
  highlightedMultiplier: 1.5,
  /** Dimmed opacity */
  dimmedOpacity: 0.3,
  /** Minimum stroke width */
  minimum: 1,
  /** Maximum stroke width */
  maximum: 6,
} as const;

/**
 * Path geometry configuration
 */
export const PATH_CONFIG = {
  /** Node padding at source */
  sourcePadding: 16,
  /** Node padding at target */
  targetPadding: 20,
  /** Curve offset ratio (proportion of edge length) */
  curveOffsetRatio: 0.25,
  /** Maximum curve offset in pixels */
  maxCurveOffset: 60,
} as const;

// =============================================================================
// Effect Sizing
// =============================================================================

/**
 * Emit effect configuration
 */
export const EMIT_CONFIG = {
  /** Base size */
  baseSize: 8,
  /** Pulse ring multiplier */
  ringMultiplier: 3,
  /** Highlighted scale */
  highlightedScale: 1.4,
  /** Number of pulse rings */
  ringCount: 2,
} as const;

/**
 * Impact effect configuration
 */
export const IMPACT_CONFIG = {
  /** Base size */
  baseSize: 12,
  /** Ripple expansion multiplier */
  rippleMultiplier: 4,
  /** Highlighted scale */
  highlightedScale: 1.5,
  /** Number of burst particles */
  burstCount: 8,
  /** Burst distance base */
  burstDistance: 3,
  /** Burst distance multiplier for intensity */
  burstDistanceMultiplier: 1.5,
} as const;

/**
 * Trail effect configuration
 */
export const TRAIL_CONFIG = {
  /** Number of trail segments */
  segmentCount: 4,
  /** Opacity decay per segment */
  opacityDecay: 0.2,
  /** Size decay per segment */
  sizeDecay: 0.15,
  /** Time offset between segments */
  timeOffset: 0.03,
} as const;

/**
 * Zigzag (neural) effect configuration
 */
export const ZIGZAG_CONFIG = {
  /** Base amplitude */
  amplitude: 15,
  /** Number of zigzag segments */
  segments: 6,
  /** Branch probability (0-1) */
  branchProbability: 0.5,
  /** Branch angle range in degrees */
  branchAngleRange: 40,
} as const;

// =============================================================================
// LOD (Level of Detail) Configuration
// =============================================================================

/**
 * Core effects (always rendered in reduced mode)
 */
export const CORE_EFFECTS: EffectPrimitive[] = ['particles', 'glow'];

/**
 * LOD configuration per tier
 */
export const LOD_CONFIGS: Record<LODTier, LODConfig> = {
  high: {
    effects: 'ALL',
    maxParticles: 8,
    enableGlow: true,
    targetFPS: 60,
  },
  medium: {
    effects: 'ALL',  // Changed: show all effects by default
    maxParticles: 4,
    enableGlow: true,
    targetFPS: 45,
  },
  low: {
    effects: 'CORE',  // Changed: particles + glow instead of just glow
    maxParticles: 2,
    enableGlow: true,
    targetFPS: 30,
  },
  minimal: {
    effects: 'GLOW',  // Changed: at least glow instead of none
    maxParticles: 1,
    enableGlow: true,
    targetFPS: 15,
  },
} as const;

/**
 * Distance thresholds for LOD tiers (in pixels, adjusted for zoom)
 */
export const LOD_DISTANCES = {
  full: 800,      // Increased: more edges get full effects
  reduced: 1500,  // Increased: larger medium zone
  minimal: 3000,  // Increased: effects visible further
  static: 5000,   // Increased: edges stay animated longer
  // Beyond static distance = hidden
} as const;

// =============================================================================
// Animation Budget Configuration
// =============================================================================

/**
 * Priority values per edge state
 */
export const EDGE_PRIORITIES: Record<EdgePriority, number> = {
  selected: 100,
  highlighted: 80,
  connected: 60,
  default: 40,
} as const;

/**
 * Default animation budget configuration
 */
export const DEFAULT_ANIMATION_BUDGET: AnimationBudgetConfig = {
  maxConcurrent: 150,  // Increased: allow more concurrent animations
  priorities: EDGE_PRIORITIES,
} as const;

// =============================================================================
// Effect Pool Configuration
// =============================================================================

/**
 * Pre-warm counts for effect pool
 */
export const POOL_PREWARM_COUNTS: Partial<Record<EffectPrimitive, number>> = {
  particles: 50,   // Increased for always-on animations
  glow: 60,
  trail: 40,
  emit: 30,
  impact: 25,
} as const;

/**
 * Maximum pool size per primitive type
 */
export const POOL_MAX_SIZE = 200;  // Increased for always-on animations

// =============================================================================
// Duration Helpers
// =============================================================================

/**
 * Get duration for a speed preset with optional state modifiers
 */
export function getDuration(
  speed: AnimationSpeed,
  isSelected: boolean = false,
  isHovered: boolean = false,
): number {
  let duration = ANIMATION_DURATIONS[speed];

  if (isSelected) {
    duration *= SELECTED_SPEED_BOOST;
  } else if (isHovered) {
    duration *= HOVERED_SPEED_BOOST;
  }

  return duration;
}

/**
 * Get particle count for a preset with LOD adjustment
 */
export function getParticleCount(
  preset: keyof typeof PARTICLE_COUNTS,
  lodTier: LODTier = 'high',
): number {
  const base = PARTICLE_COUNTS[preset];
  const lodConfig = LOD_CONFIGS[lodTier];

  return Math.min(base, lodConfig.maxParticles);
}
