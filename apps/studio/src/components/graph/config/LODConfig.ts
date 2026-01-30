/**
 * Level of Detail (LOD) Configuration - Task 3.1 Performance Optimization
 *
 * Provides zoom-based visual detail scaling for optimal graph performance.
 * At low zoom levels, expensive effects are disabled to maintain 60fps.
 *
 * Part of Phase 3: Advanced Performance Optimizations.
 *
 * Tiers (zoom thresholds):
 *   ULTRA   > 1.5   Full quality, all effects enabled
 *   HIGH    > 0.8   Reduced glow, still interactive
 *   MEDIUM  > 0.4   No particles, simplified energy effects
 *   LOW     > 0.2   Minimal rendering, no glow
 *   MINIMAL <= 0.2  Bare minimum for overview
 */

/** Configuration for a single LOD tier */
export interface LODTier {
  /** Enable particle animations on edges */
  particles: boolean;
  /** Energy effect mode: true (full), 'simplified' (reduced), or false (disabled) */
  energyEffects: boolean | 'simplified';
  /** Number of glow layers (0-3) */
  glowLayers: number;
  /** Show node labels */
  labels: boolean;
  /** Edge label abbreviation level */
  labelAbbreviation: 'full' | 'short' | 'initials' | 'icon';
  /** Blur filter quality */
  blurQuality: 'high' | 'medium' | 'low' | 'none';
}

/** Available LOD tier names */
export type LODTierName = 'ULTRA' | 'HIGH' | 'MEDIUM' | 'LOW' | 'MINIMAL';

/**
 * LOD tier configurations for each zoom level.
 * Higher tiers enable more visual effects at performance cost.
 */
export const LOD_TIERS = {
  /** Full quality - zoom > 1.5 */
  ULTRA: {
    particles: true,
    energyEffects: true,
    glowLayers: 3,
    labels: true,
    labelAbbreviation: 'full',
    blurQuality: 'high',
  },
  /** High quality - zoom 0.8-1.5 */
  HIGH: {
    particles: true,
    energyEffects: true,
    glowLayers: 2,
    labels: true,
    labelAbbreviation: 'full',
    blurQuality: 'medium',
  },
  /** Medium quality - zoom 0.4-0.8 */
  MEDIUM: {
    particles: true,
    energyEffects: 'simplified',
    glowLayers: 1,
    labels: true,
    labelAbbreviation: 'short',
    blurQuality: 'low',
  },
  /** Low quality - zoom 0.2-0.4 */
  LOW: {
    particles: true,
    energyEffects: 'simplified',
    glowLayers: 0,
    labels: true,
    labelAbbreviation: 'initials',
    blurQuality: 'none',
  },
  /** Minimal quality - zoom <= 0.2 */
  MINIMAL: {
    particles: true,
    energyEffects: false,
    glowLayers: 0,
    labels: true,
    labelAbbreviation: 'icon',
    blurQuality: 'none',
  },
} as const satisfies Record<LODTierName, LODTier>;

/**
 * Get the appropriate LOD tier for a given zoom level.
 *
 * @param zoom - Current viewport zoom level (0.0 to infinity)
 * @returns The LOD tier configuration for that zoom level
 *
 * @example
 * const tier = getLODTier(1.2); // Returns HIGH tier
 * if (tier.particles) {
 *   renderParticles();
 * }
 */
export function getLODTier(zoom: number): LODTier {
  if (zoom > 1.5) return LOD_TIERS.ULTRA;
  if (zoom > 0.8) return LOD_TIERS.HIGH;
  if (zoom > 0.4) return LOD_TIERS.MEDIUM;
  if (zoom > 0.2) return LOD_TIERS.LOW;
  return LOD_TIERS.MINIMAL;
}

/**
 * Get the LOD tier name for a given zoom level.
 * Useful for logging and debugging.
 *
 * @param zoom - Current viewport zoom level
 * @returns The tier name as a string
 */
export function getLODTierName(zoom: number): LODTierName {
  if (zoom > 1.5) return 'ULTRA';
  if (zoom > 0.8) return 'HIGH';
  if (zoom > 0.4) return 'MEDIUM';
  if (zoom > 0.2) return 'LOW';
  return 'MINIMAL';
}
