/**
 * Arc Particle Effects for 3D Graph Visualization
 *
 * ATOMIC/NEURAL STYLE — Small, fast, numerous particles
 * Creates visible energy flow like fiber optics + synapses
 *
 * Colors are sourced from the unified palette system (taxonomy.yaml).
 * Only particle-specific config (speed, width, count) is defined here.
 *
 * @see @/design/colors/palette.ts — Unified color system
 */

import {
  getArcFamily as getArcFamilyUnified,
  ARC_PARTICLE_COLORS,
  type ArcFamilyKey,
} from '@/design/colors/palette';

// Re-export the unified type for backwards compatibility
export type ArcFamily = ArcFamilyKey;

export interface ArcParticleConfig {
  particles: number;
  particleSpeed: number;
  particleWidth: number;
  particleColor: string;
  linkColor: string;      // Separate dark color for the link itself
  linkWidth: number;
  linkOpacity: number;
  curvature: number;
}

/**
 * Particle configuration for each arc family — PARTICLES ARE THE STAR
 *
 * MAXIMUM visibility for particles, links are near-invisible guides:
 * - particles: 15-30 (continuous visible stream)
 * - particleSpeed: 0.01-0.03 (fast but trackable)
 * - particleWidth: 4-7 (bright visible orbs)
 * - linkWidth: 0.2-0.4 (hair-thin, just a hint)
 * - linkOpacity: 0.05-0.08 (near transparent)
 * - curvature: 0-0.25 for elegant arcs
 *
 * Colors are derived from taxonomy.yaml via ARC_PARTICLE_COLORS
 */
export const ARC_PARTICLE_CONFIG: Record<ArcFamily, ArcParticleConfig> = {
  ownership: {
    particles: 20,             // Continuous stream
    particleSpeed: 0.012,      // Steady flow
    particleWidth: 5,          // Bright orbs
    particleColor: ARC_PARTICLE_COLORS.ownership.particleColor,
    linkColor: ARC_PARTICLE_COLORS.ownership.linkColor,
    linkWidth: 0.3,            // Hair-thin guide
    linkOpacity: 0.29,         // Bright tubes
    curvature: 0,              // Straight ownership links
  },
  localization: {
    particles: 15,             // Flowing particles
    particleSpeed: 0.008,      // Slower, undulating
    particleWidth: 6,          // Larger orbs
    particleColor: ARC_PARTICLE_COLORS.localization.particleColor,
    linkColor: ARC_PARTICLE_COLORS.localization.linkColor,
    linkWidth: 0.25,           // Hair-thin
    linkOpacity: 0.29,         // Bright tubes
    curvature: 0.2,            // Gentle curve
  },
  semantic: {
    particles: 25,             // Dense sparking stream
    particleSpeed: 0.02,       // Fast synaptic firing
    particleWidth: 4,          // Smaller, numerous
    particleColor: ARC_PARTICLE_COLORS.semantic.particleColor,
    linkColor: ARC_PARTICLE_COLORS.semantic.linkColor,
    linkWidth: 0.2,            // Thinnest - synapses don't need thick lines
    linkOpacity: 0.29,         // Bright tubes
    curvature: 0.1,            // Slight curve
  },
  generation: {
    particles: 30,             // Cascading energy
    particleSpeed: 0.025,      // Fast cascade
    particleWidth: 5,          // Medium orbs
    particleColor: ARC_PARTICLE_COLORS.generation.particleColor,
    linkColor: ARC_PARTICLE_COLORS.generation.linkColor,
    linkWidth: 0.35,           // Slightly thicker for generation flow
    linkOpacity: 0.38,         // Bright tubes
    curvature: 0,              // Straight generation flow
  },
  mining: {
    particles: 12,             // Radar pulses
    particleSpeed: 0.006,      // Slow sweep
    particleWidth: 7,          // Large pings - radar style
    particleColor: ARC_PARTICLE_COLORS.mining.particleColor,
    linkColor: ARC_PARTICLE_COLORS.mining.linkColor,
    linkWidth: 0.25,           // Thin
    linkOpacity: 0.29,         // Bright tubes
    curvature: 0.25,           // Curved sweep
  },
  schema: {
    particles: 10,             // Structured flow
    particleSpeed: 0.01,       // Steady pace
    particleWidth: 5,          // Medium orbs
    particleColor: ARC_PARTICLE_COLORS.schema.particleColor,
    linkColor: ARC_PARTICLE_COLORS.schema.linkColor,
    linkWidth: 0.3,            // Medium width
    linkOpacity: 0.25,         // Subtle tubes
    curvature: 0,              // Straight schema links
  },
};

/**
 * Detect arc family from relation type
 * Re-exports the unified getArcFamily function from palette.ts
 */
export function detectArcFamily(relationType: string): ArcFamily {
  return getArcFamilyUnified(relationType);
}

/**
 * Get particle config for a relation type
 */
export function getArcParticleConfig(relationType: string): ArcParticleConfig {
  const family = detectArcFamily(relationType);
  return ARC_PARTICLE_CONFIG[family];
}
