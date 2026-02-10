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
 */
export const ARC_PARTICLE_CONFIG: Record<ArcFamily, ArcParticleConfig> = {
  ownership: {
    particles: 20,             // Continuous stream
    particleSpeed: 0.012,      // Steady flow
    particleWidth: 5,          // Bright orbs
    particleColor: '#60a5fa',  // Bright blue (blue-400)
    linkColor: '#1e3a5f',      // Very dark blue - barely visible guide
    linkWidth: 0.3,            // Hair-thin guide
    linkOpacity: 0.15,         // Low opacity
    curvature: 0,              // Straight ownership links
  },
  localization: {
    particles: 15,             // Flowing particles
    particleSpeed: 0.008,      // Slower, undulating
    particleWidth: 6,          // Larger orbs
    particleColor: '#4ade80',  // Bright green (green-400)
    linkColor: '#134e3a',      // Very dark green
    linkWidth: 0.25,           // Hair-thin
    linkOpacity: 0.15,         // Low opacity
    curvature: 0.2,            // Gentle curve
  },
  semantic: {
    particles: 25,             // Dense sparking stream
    particleSpeed: 0.02,       // Fast synaptic firing
    particleWidth: 4,          // Smaller, numerous
    particleColor: '#fb923c',  // Bright orange (orange-400)
    linkColor: '#431407',      // Very dark orange/brown
    linkWidth: 0.2,            // Thinnest - synapses don't need thick lines
    linkOpacity: 0.15,         // Low opacity
    curvature: 0.1,            // Slight curve
  },
  generation: {
    particles: 30,             // Cascading energy
    particleSpeed: 0.025,      // Fast cascade
    particleWidth: 5,          // Medium orbs
    particleColor: '#a78bfa',  // Bright purple (violet-400)
    linkColor: '#2e1065',      // Very dark violet
    linkWidth: 0.35,           // Slightly thicker for generation flow
    linkOpacity: 0.2,          // Slightly more visible
    curvature: 0,              // Straight generation flow
  },
  mining: {
    particles: 12,             // Radar pulses
    particleSpeed: 0.006,      // Slow sweep
    particleWidth: 7,          // Large pings - radar style
    particleColor: '#f472b6',  // Bright pink (pink-400)
    linkColor: '#500724',      // Very dark pink/maroon
    linkWidth: 0.25,           // Thin
    linkOpacity: 0.15,         // Low opacity
    curvature: 0.25,           // Curved sweep
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
