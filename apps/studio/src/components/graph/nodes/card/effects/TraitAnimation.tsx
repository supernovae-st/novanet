'use client';

/**
 * TraitAnimation - Trait-specific animations based on ADR-024 Data Origin
 *
 * Each trait has a distinctive animation that reflects its data origin:
 *
 * | Trait     | Animation     | Visual Metaphor                      |
 * |-----------|---------------|--------------------------------------|
 * | defined   | Solid glow    | Stable, foundational (human ONCE)    |
 * | authored  | Writing pulse | Editorial heartbeat (human/locale)   |
 * | imported  | Incoming flow | Data streaming in (external source)  |
 * | generated | AI spark      | LLM magic sparkle (our LLM)          |
 * | retrieved | Fetch wave    | API ping/response (external APIs)    |
 *
 * Usage:
 * ```tsx
 * <TraitAnimation trait="generated" active={true}>
 *   <div className="status-indicator" />
 * </TraitAnimation>
 * ```
 */

import { memo, type ReactNode } from 'react';
import { motion, type Variants } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { TRAIT_COLORS, type TraitKey } from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export type AnimatableTrait = TraitKey;

export interface TraitAnimationProps {
  /** The trait determining animation style */
  trait: AnimatableTrait;
  /** Whether animation is active */
  active?: boolean;
  /** Whether element is selected (intensifies animation) */
  selected?: boolean;
  /** Children to wrap */
  children: ReactNode;
  /** Performance config */
  performanceConfig?: PerformanceConfig;
  /** Additional class names */
  className?: string;
}

// =============================================================================
// Animation Variants per Trait
// =============================================================================

const TRAIT_ANIMATIONS: Record<AnimatableTrait, Variants> = {
  // DEFINED: Stable, subtle breathing glow (human creates ONCE)
  defined: {
    idle: {
      scale: 1,
      opacity: 1,
    },
    active: {
      scale: [1, 1.02, 1],
      opacity: [1, 0.95, 1],
      transition: {
        duration: 3,
        repeat: Infinity,
        ease: 'easeInOut',
      },
    },
  },

  // AUTHORED: Rhythmic pulse like writing (human writes PER locale)
  authored: {
    idle: {
      scale: 1,
      opacity: 1,
    },
    active: {
      scale: [1, 1.05, 1],
      opacity: [1, 0.9, 1],
      transition: {
        duration: 1.5,
        repeat: Infinity,
        ease: [0.4, 0, 0.2, 1],
      },
    },
  },

  // IMPORTED: Streaming flow effect (external data brought in)
  imported: {
    idle: {
      scale: 1,
      x: 0,
      opacity: 1,
    },
    active: {
      scale: [1, 1.03, 1],
      x: [0, 2, 0, -2, 0],
      opacity: [1, 0.95, 1, 0.95, 1],
      transition: {
        duration: 2,
        repeat: Infinity,
        ease: 'easeInOut',
      },
    },
  },

  // GENERATED: Sparkle/magic effect (our LLM produces)
  generated: {
    idle: {
      scale: 1,
      opacity: 1,
      rotate: 0,
    },
    active: {
      scale: [1, 1.08, 0.98, 1.04, 1],
      opacity: [1, 0.8, 1, 0.9, 1],
      rotate: [0, 1, -1, 0.5, 0],
      transition: {
        duration: 1.2,
        repeat: Infinity,
        ease: [0.25, 0.1, 0.25, 1],
      },
    },
  },

  // RETRIEVED: Ping/fetch wave (fetched from external APIs)
  retrieved: {
    idle: {
      scale: 1,
      opacity: 1,
    },
    active: {
      scale: [1, 1.1, 1],
      opacity: [1, 0.7, 1],
      transition: {
        duration: 0.8,
        repeat: Infinity,
        ease: 'easeOut',
        repeatDelay: 1.2,
      },
    },
  },
};

// Selected state intensifiers
const _SELECTED_INTENSIFIERS: Record<AnimatableTrait, Partial<Variants['active']>> = {
  defined: {
    scale: [1, 1.04, 1],
    filter: ['brightness(1)', 'brightness(1.2)', 'brightness(1)'],
  },
  authored: {
    scale: [1, 1.08, 1],
  },
  imported: {
    x: [0, 4, 0, -4, 0],
  },
  generated: {
    scale: [1, 1.12, 0.95, 1.08, 1],
    rotate: [0, 2, -2, 1, 0],
  },
  retrieved: {
    scale: [1, 1.15, 1],
  },
};

// =============================================================================
// Component
// =============================================================================

export const TraitAnimation = memo(function TraitAnimation({
  trait,
  active = false,
  selected = false,
  children,
  performanceConfig,
  className,
}: TraitAnimationProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  if (!animationsEnabled) {
    return <div className={className}>{children}</div>;
  }

  const variants = TRAIT_ANIMATIONS[trait];
  const animationState = active ? 'active' : 'idle';

  // Get trait color for glow effect
  const traitColor = TRAIT_COLORS[trait]?.color ?? '#ffffff';

  return (
    <motion.div
      className={className}
      variants={variants}
      initial="idle"
      animate={animationState}
      style={{
        // Add subtle glow when active
        filter: active
          ? `drop-shadow(0 0 ${selected ? 8 : 4}px ${traitColor}60)`
          : undefined,
      }}
    >
      {children}
    </motion.div>
  );
});

// =============================================================================
// Trait Status Indicator (animated dot)
// =============================================================================

export interface TraitStatusDotProps {
  trait: AnimatableTrait;
  size?: number;
  selected?: boolean;
  performanceConfig?: PerformanceConfig;
}

/**
 * Animated status dot that visualizes trait type
 */
export const TraitStatusDot = memo(function TraitStatusDot({
  trait,
  size = 8,
  selected = false,
  performanceConfig,
}: TraitStatusDotProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const traitColor = TRAIT_COLORS[trait]?.color ?? '#ffffff';

  if (!animationsEnabled) {
    return (
      <div
        className="rounded-full"
        style={{
          width: size,
          height: size,
          background: traitColor,
          boxShadow: `0 0 ${size / 2}px ${traitColor}`,
        }}
      />
    );
  }

  return (
    <TraitAnimation
      trait={trait}
      active={true}
      selected={selected}
      performanceConfig={performanceConfig}
    >
      <div
        className="rounded-full"
        style={{
          width: size,
          height: size,
          background: traitColor,
          boxShadow: `0 0 ${selected ? size : size / 2}px ${traitColor}`,
        }}
      />
    </TraitAnimation>
  );
});

// =============================================================================
// Trait Badge with Animation
// =============================================================================

export interface TraitBadgeAnimatedProps {
  trait: AnimatableTrait;
  selected?: boolean;
  isHovered?: boolean;
  performanceConfig?: PerformanceConfig;
  showLabel?: boolean;
}

/**
 * Trait badge with embedded animation
 */
export const TraitBadgeAnimated = memo(function TraitBadgeAnimated({
  trait,
  selected = false,
  isHovered = false,
  performanceConfig,
  showLabel = true,
}: TraitBadgeAnimatedProps) {
  const traitColor = TRAIT_COLORS[trait]?.color ?? '#ffffff';
  const _active = selected || isHovered;

  return (
    <div
      className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded text-[9px] font-semibold uppercase tracking-wide"
      style={{
        background: `${traitColor}20`,
        color: traitColor,
        border: `1px solid ${traitColor}35`,
      }}
    >
      <TraitStatusDot
        trait={trait}
        size={6}
        selected={selected}
        performanceConfig={performanceConfig}
      />
      {showLabel && <span>{trait}</span>}
    </div>
  );
});

export default TraitAnimation;
