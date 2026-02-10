/**
 * Arc Particle Effects for 3D Graph Visualization
 *
 * Each arc family has distinct particle behavior:
 * - ownership: Pulse dots flowing source→target
 * - localization: Globe orbs with slow orbit
 * - semantic: Sparks with random zigzag
 * - generation: Matrix rain cascade
 * - mining: Radar sweep pulse
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

// Particle configuration for each arc family
export const ARC_PARTICLE_CONFIG: Record<ArcFamily, ArcParticleConfig> = {
  ownership: {
    particles: 4,
    particleSpeed: 0.005,
    particleWidth: 2,
    particleColor: '#3b82f6',
    linkWidth: 1.5,
    linkOpacity: 0.4,
    curvature: 0,
  },
  localization: {
    particles: 2,
    particleSpeed: 0.002,
    particleWidth: 3,
    particleColor: '#22c55e',
    linkWidth: 1,
    linkOpacity: 0.3,
    curvature: 0.2,
  },
  semantic: {
    particles: 6,
    particleSpeed: 0.008,
    particleWidth: 1.5,
    particleColor: '#f97316',
    linkWidth: 1,
    linkOpacity: 0.3,
    curvature: 0.1,
  },
  generation: {
    particles: 8,
    particleSpeed: 0.01,
    particleWidth: 2.5,
    particleColor: '#8b5cf6',
    linkWidth: 2,
    linkOpacity: 0.5,
    curvature: 0,
  },
  mining: {
    particles: 3,
    particleSpeed: 0.003,
    particleWidth: 1,
    particleColor: '#ec4899',
    linkWidth: 0.5,
    linkOpacity: 0.2,
    curvature: 0.3,
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
