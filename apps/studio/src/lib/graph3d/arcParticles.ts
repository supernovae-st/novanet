/**
 * Arc Particle Effects for 3D Graph Visualization
 *
 * ATOMIC/NEURAL STYLE — Small, fast, numerous particles
 * Creates visible energy flow like fiber optics + synapses
 *
 * Each arc family has distinct particle behavior:
 * - ownership: Steady data stream, blue
 * - localization: Undulating flow, green
 * - semantic: Sparking synapses, orange
 * - generation: Fast cascade, violet
 * - mining: Radar pulse waves, pink
 */

export type ArcFamily = 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';

export interface ArcParticleConfig {
  particles: number;
  particleSpeed: number;
  particleWidth: number;
  particleColor: string;
  linkWidth: number;
  linkOpacity: number;
  curvature: number;
}

/**
 * Particle configuration for each arc family — NEURAL/FIBER OPTIC STYLE
 *
 * Key settings for visible energy transfer:
 * - particleWidth: 2-4 (tiny, like neurons firing)
 * - particles: 30-50 (creates continuous stream)
 * - particleSpeed: 0.02-0.05 (fast, visible flow)
 * - linkWidth: 1.5 (thin elegant line with glow)
 * - curvature: 0-0.3 for elegant arcs
 */
export const ARC_PARTICLE_CONFIG: Record<ArcFamily, ArcParticleConfig> = {
  ownership: {
    particles: 8,              // Fewer but more visible particles
    particleSpeed: 0.008,      // Slower for visibility (was 0.025)
    particleWidth: 6,          // Much larger (was 3)
    particleColor: '#60a5fa',  // Bright blue (blue-400)
    linkWidth: 1.5,            // Thin elegant line
    linkOpacity: 0.35,         // More visible (was 0.25)
    curvature: 0,              // Straight ownership links
  },
  localization: {
    particles: 6,              // Fewer particles
    particleSpeed: 0.006,      // Slower (was 0.02)
    particleWidth: 7,          // Larger (was 3.5)
    particleColor: '#4ade80',  // Bright green (green-400)
    linkWidth: 1.5,            // Thin line
    linkOpacity: 0.3,          // More visible (was 0.2)
    curvature: 0.2,            // Gentle curve
  },
  semantic: {
    particles: 10,             // Fewer sparking particles
    particleSpeed: 0.012,      // Slower (was 0.035)
    particleWidth: 5,          // Larger (was 2.5)
    particleColor: '#fb923c',  // Bright orange (orange-400)
    linkWidth: 1.2,            // Slightly thicker (was 1)
    linkOpacity: 0.3,          // More visible (was 0.2)
    curvature: 0.1,            // Slight curve
  },
  generation: {
    particles: 8,              // Fewer cascade particles
    particleSpeed: 0.015,      // Slower (was 0.045)
    particleWidth: 6,          // Larger (was 3)
    particleColor: '#a78bfa',  // Bright purple (violet-400)
    linkWidth: 1.8,            // Thicker (was 2)
    linkOpacity: 0.4,          // More visible (was 0.35)
    curvature: 0,              // Straight generation flow
  },
  mining: {
    particles: 5,              // Fewer radar pulses
    particleSpeed: 0.005,      // Much slower (was 0.015)
    particleWidth: 8,          // Much larger (was 4)
    particleColor: '#f472b6',  // Bright pink (pink-400)
    linkWidth: 1.5,            // Thin line
    linkOpacity: 0.3,          // More visible (was 0.2)
    curvature: 0.3,            // Curved sweep
  },
};

/**
 * Detect arc family from relation type
 */
export function detectArcFamily(relationType: string): ArcFamily {
  const type = relationType.toUpperCase();

  // Ownership family
  if (
    type.startsWith('HAS_') ||
    type.includes('BELONGS') ||
    type.includes('_OF') ||
    type === 'HAS_CONTENT' ||
    type === 'HAS_GENERATED'
  ) {
    return 'ownership';
  }

  // Localization family
  if (
    type.includes('LOCALE') ||
    type.includes('LOCALIZE') ||
    type.includes('L10N') ||
    type === 'CONTENT_OF'
  ) {
    return 'localization';
  }

  // Semantic family
  if (
    type.includes('USES') ||
    type.includes('LINKS') ||
    type.includes('RELATED') ||
    type.includes('SEMANTIC')
  ) {
    return 'semantic';
  }

  // Generation family
  if (
    type.includes('GENERATE') ||
    type.includes('PROMPT') ||
    type.includes('OUTPUT') ||
    type === 'GENERATED_FOR'
  ) {
    return 'generation';
  }

  // Mining family
  if (
    type.includes('SEO') ||
    type.includes('GEO') ||
    type.includes('KEYWORD') ||
    type.includes('METRIC')
  ) {
    return 'mining';
  }

  // Default to ownership
  return 'ownership';
}

/**
 * Get particle config for a relation type
 */
export function getArcParticleConfig(relationType: string): ArcParticleConfig {
  const family = detectArcFamily(relationType);
  return ARC_PARTICLE_CONFIG[family];
}
