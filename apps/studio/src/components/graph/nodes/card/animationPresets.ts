/**
 * Animation Presets for Card System
 *
 * Performance-aware Framer Motion variants and springs for card animations.
 * Uses spring physics for natural motion when enabled, instant transitions when not.
 *
 * Usage:
 * ```tsx
 * import { createCardVariants, SPRING_CONFIGS } from './animationPresets';
 *
 * const variants = createCardVariants(performanceConfig.animation.spring);
 * <motion.div variants={variants} initial="hidden" animate="visible" />
 * ```
 */

import type { Variants, Transition, SpringOptions } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Spring Configurations
// =============================================================================

export const SPRING_CONFIGS = {
  /** Bouncy, energetic spring for premium effects */
  bouncy: {
    type: 'spring',
    stiffness: 400,
    damping: 25,
    mass: 1,
  } as const,

  /** Smooth, controlled spring for UI elements */
  smooth: {
    type: 'spring',
    stiffness: 500,
    damping: 30,
    mass: 1,
  } as const,

  /** Gentle spring for subtle movements */
  gentle: {
    type: 'spring',
    stiffness: 300,
    damping: 35,
    mass: 1.2,
  } as const,

  /** Quick snap for selection states */
  snappy: {
    type: 'spring',
    stiffness: 600,
    damping: 40,
    mass: 0.8,
  } as const,

  /** Reduced motion spring (minimal bounce) */
  reduced: {
    type: 'spring',
    stiffness: 400,
    damping: 50,
    mass: 1,
  } as const,
} satisfies Record<string, SpringOptions & { type: 'spring' }>;

// =============================================================================
// Duration Presets
// =============================================================================

export const DURATIONS = {
  instant: 0,
  fast: 0.15,
  normal: 0.3,
  slow: 0.5,
  glacial: 0.8,
} as const;

// =============================================================================
// Easing Functions
// =============================================================================

export const EASINGS = {
  /** Natural ease out */
  out: [0.22, 1, 0.36, 1] as const,
  /** Smooth ease in-out */
  inOut: [0.4, 0, 0.2, 1] as const,
  /** Bouncy overshoot */
  overshoot: [0.175, 0.885, 0.32, 1.275] as const,
  /** Quick deceleration */
  decel: [0, 0, 0.2, 1] as const,
} as const;

// =============================================================================
// Card Variants Factory
// =============================================================================

/**
 * Creates animation variants based on performance configuration
 */
export function createCardVariants(
  springEnabled: boolean,
  duration: 'fast' | 'normal' | 'slow' | 'none' = 'normal'
): Variants {
  const actualDuration = duration === 'none' ? 0 : DURATIONS[duration];

  return {
    hidden: {
      opacity: 0,
      scale: 0.95,
      y: 10,
    },
    visible: {
      opacity: 1,
      scale: 1,
      y: 0,
      transition: springEnabled
        ? SPRING_CONFIGS.smooth
        : { duration: actualDuration, ease: EASINGS.out },
    },
    hover: {
      y: -2,
      transition: {
        duration: DURATIONS.fast,
        ease: EASINGS.out,
      },
    },
    selected: {
      scale: 1.02,
      transition: springEnabled
        ? SPRING_CONFIGS.snappy
        : { duration: DURATIONS.fast, ease: EASINGS.overshoot },
    },
    exit: {
      opacity: 0,
      scale: 0.9,
      transition: {
        duration: DURATIONS.fast,
        ease: EASINGS.out,
      },
    },
  };
}

// =============================================================================
// Effect-Specific Variants
// =============================================================================

/**
 * Variants for glow/pulse effects
 */
export const glowVariants: Variants = {
  idle: {
    opacity: 0.4,
    scale: 1,
  },
  hover: {
    opacity: 0.6,
    scale: 1.02,
    transition: {
      duration: DURATIONS.normal,
      ease: EASINGS.out,
    },
  },
  selected: {
    opacity: 0.8,
    scale: 1.05,
    transition: SPRING_CONFIGS.gentle,
  },
};

/**
 * Variants for floating/levitation effects
 */
export const floatVariants: Variants = {
  initial: { y: 0 },
  float: {
    y: [0, -4, 0],
    transition: {
      duration: 2,
      ease: 'easeInOut',
      repeat: Infinity,
      repeatType: 'reverse',
    },
  },
};

/**
 * Variants for pulse ring animations
 */
export const pulseRingVariants: Variants = {
  initial: {
    scale: 1,
    opacity: 0.8,
  },
  pulse: {
    scale: [1, 1.1, 1],
    opacity: [0.8, 0.4, 0.8],
    transition: {
      duration: 1.5,
      ease: 'easeInOut',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for rotating gradient borders
 */
export const gradientRotateVariants: Variants = {
  initial: { rotate: 0 },
  animate: {
    rotate: 360,
    transition: {
      duration: 8,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for shimmer/sparkle effects
 */
export const shimmerVariants: Variants = {
  initial: { x: '-100%' },
  animate: {
    x: '100%',
    transition: {
      duration: 2,
      ease: 'easeInOut',
      repeat: Infinity,
      repeatDelay: 1,
    },
  },
};

/**
 * Variants for electric border flow (MagicUI inspired)
 */
export const electricFlowVariants: Variants = {
  initial: { backgroundPosition: '0% 50%' },
  animate: {
    backgroundPosition: ['0% 50%', '100% 50%', '0% 50%'],
    transition: {
      duration: 3,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for electric pulse effect
 */
export const electricPulseVariants: Variants = {
  idle: { opacity: 0.5, scale: 1 },
  animate: {
    opacity: [0.5, 1, 0.5],
    scale: [1, 1.02, 1],
    transition: {
      duration: 1.5,
      ease: 'easeInOut',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for 3D perspective transforms (MagicUI GlassCard inspired)
 */
export const perspective3DVariants: Variants = {
  idle: {
    rotateX: 0,
    rotateY: 0,
    scale: 1,
  },
  hover: {
    rotateX: 5,
    rotateY: 5,
    scale: 1.02,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    rotateX: 0,
    rotateY: 0,
    scale: 1.03,
    transition: SPRING_CONFIGS.smooth,
  },
};

/**
 * Variants for layered glow effect
 */
export const layeredGlowVariants: Variants = {
  idle: {
    scale: 1,
    opacity: 0.3,
  },
  hover: {
    scale: 1.05,
    opacity: 0.5,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.1,
    opacity: 0.7,
    transition: { duration: DURATIONS.fast },
  },
};

// =============================================================================
// Taxonomy-Specific Variants
// =============================================================================

/**
 * Variants for orbital ring animations (Realm nodes)
 */
export const orbitalVariants: Variants = {
  initial: { rotate: 0 },
  animate: {
    rotate: 360,
    transition: {
      duration: 20,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for stacked planes effect (Layer nodes)
 */
export const stackedPlaneVariants: Variants = {
  initial: { z: 0, opacity: 0.5 },
  hover: {
    z: 10,
    opacity: 0.8,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    z: 20,
    opacity: 1,
    transition: SPRING_CONFIGS.smooth,
  },
};

/**
 * Variants for border morph effect (Trait nodes)
 * Note: strokeDasharray is handled via CSS, these variants control opacity/scale
 */
export const borderMorphVariants: Variants = {
  solid: { pathLength: 1, opacity: 1 },
  dashed: { pathLength: 1, opacity: 0.9 },
  dotted: { pathLength: 1, opacity: 0.8 },
  double: { pathLength: 1, scale: 1.02 },
};

/**
 * Variants for radiating pulse effect (ArcFamily nodes)
 */
export const radiatingPulseVariants: Variants = {
  initial: {
    scale: 0.5,
    opacity: 0.8,
  },
  animate: {
    scale: [0.5, 1.5, 0.5],
    opacity: [0.8, 0, 0.8],
    transition: {
      duration: 2,
      ease: 'easeOut',
      repeat: Infinity,
    },
  },
};

// =============================================================================
// Schema-Specific Variants
// =============================================================================

/**
 * Variants for tech corner animations (Class nodes)
 */
export const techCornerVariants: Variants = {
  idle: {
    opacity: 0.6,
    pathLength: 0.8,
  },
  hover: {
    opacity: 0.9,
    pathLength: 1,
    transition: {
      duration: DURATIONS.fast,
      ease: EASINGS.out,
    },
  },
  selected: {
    opacity: 1,
    pathLength: 1,
    filter: 'drop-shadow(0 0 4px currentColor)',
    transition: SPRING_CONFIGS.snappy,
  },
};

/**
 * Variants for flowing particles (Arc nodes)
 */
export const flowingParticleVariants: Variants = {
  initial: { x: 0, opacity: 0 },
  animate: {
    x: '100%',
    opacity: [0, 1, 1, 0],
    transition: {
      duration: 2,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

/**
 * Variants for neon border glow
 */
export const neonGlowVariants: Variants = {
  idle: {
    boxShadow: '0 0 0 rgba(var(--color), 0)',
  },
  hover: {
    boxShadow: '0 0 15px rgba(var(--color), 0.3)',
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    boxShadow: '0 0 25px rgba(var(--color), 0.5), 0 0 50px rgba(var(--color), 0.3)',
    transition: SPRING_CONFIGS.gentle,
  },
};

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Get appropriate transition based on performance config
 */
export function getTransition(
  config: PerformanceConfig,
  preset: keyof typeof SPRING_CONFIGS = 'smooth'
): Transition {
  if (!config.animation.enabled) {
    return { duration: 0 };
  }

  if (config.animation.spring) {
    return SPRING_CONFIGS[preset];
  }

  const duration =
    config.animation.duration === 'none'
      ? 0
      : DURATIONS[config.animation.duration];

  return { duration, ease: EASINGS.out };
}

/**
 * Check if animations should be enabled
 */
export function shouldAnimate(config: PerformanceConfig): boolean {
  return config.animation.enabled && config.animation.duration !== 'none';
}

/**
 * Get reduced motion variants (for accessibility)
 */
export function getReducedMotionVariants(): Variants {
  return {
    hidden: { opacity: 0 },
    visible: { opacity: 1, transition: { duration: DURATIONS.fast } },
    hover: {},
    selected: {},
    exit: { opacity: 0, transition: { duration: DURATIONS.fast } },
  };
}
