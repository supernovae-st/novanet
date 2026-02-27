'use client';

/**
 * TraitGlow - Trait-based animated glow effect
 *
 * Each trait (Data Origin per ADR-024) has a unique animation mode:
 * - defined: stable pulse (solid, unchanging foundation)
 * - authored: breathing (organic, human touch)
 * - imported: color shift (external data flowing in)
 * - generated: horizontal flow (LLM generation stream)
 * - retrieved: rotation (fetching, loading)
 *
 * Layout:
 * ┌─────────────────────────────────────────┐
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │  ← Animated glow layer
 * │  ░   ┌─────────────────────────────┐ ░  │
 * │  ░   │                             │ ░  │
 * │  ░   │         Content             │ ░  │
 * │  ░   │                             │ ░  │
 * │  ░   └─────────────────────────────┘ ░  │
 * │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
 * └─────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { NodeTrait } from '../taxonomyColors';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../animationPresets';

// =============================================================================
// Types
// =============================================================================

export type GlowMode = 'pulse' | 'breathe' | 'colorShift' | 'flowHorizontal' | 'rotate' | 'static';

export interface TraitGlowProps {
  /** Node trait (determines animation mode) */
  trait: NodeTrait;
  /** Primary color for the glow */
  color: string;
  /** Whether the node is selected */
  selected?: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Glow intensity */
  intensity?: 'low' | 'medium' | 'high';
  /** Children to wrap */
  children?: React.ReactNode;
}

// =============================================================================
// Helper Functions
// =============================================================================

/** Get animation mode from trait */
export function getGlowMode(trait: NodeTrait): GlowMode {
  switch (trait) {
    case 'defined':
      return 'pulse';
    case 'authored':
      return 'breathe';
    case 'imported':
      return 'colorShift';
    case 'generated':
      return 'flowHorizontal';
    case 'retrieved':
      return 'rotate';
    default:
      return 'static';
  }
}

/** Convert hex to RGB for rgba usage */
const hexToRgb = (hex: string): string => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
    : '100, 100, 100';
};

// =============================================================================
// Animation Variants
// =============================================================================

const createPulseVariants = (color: string): Variants => ({
  idle: {
    boxShadow: `0 0 8px ${color}20, inset 0 0 4px ${color}10`,
  },
  hover: {
    boxShadow: `0 0 16px ${color}40, inset 0 0 8px ${color}20`,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    boxShadow: [
      `0 0 20px ${color}50, inset 0 0 10px ${color}25`,
      `0 0 30px ${color}60, inset 0 0 15px ${color}30`,
      `0 0 20px ${color}50, inset 0 0 10px ${color}25`,
    ],
    transition: { duration: 1.5, repeat: Infinity, ease: 'easeInOut' },
  },
});

const createBreatheVariants = (color: string): Variants => ({
  idle: {
    boxShadow: `0 0 10px ${color}25`,
    scale: 1,
  },
  hover: {
    boxShadow: `0 0 20px ${color}40`,
    scale: 1.01,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    boxShadow: [
      `0 0 15px ${color}35`,
      `0 0 25px ${color}50`,
      `0 0 15px ${color}35`,
    ],
    scale: [1, 1.015, 1],
    transition: { duration: 2.5, repeat: Infinity, ease: 'easeInOut' },
  },
});

// =============================================================================
// Subcomponents
// =============================================================================

/**
 * Flowing horizontal glow for generated trait (LLM stream effect)
 */
const FlowingGlow = memo(function FlowingGlow({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;
  const rgb = hexToRgb(color);

  return (
    <motion.div
      className="absolute inset-0 pointer-events-none rounded-xl overflow-hidden"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <motion.div
        className="absolute inset-0"
        style={{
          background: `linear-gradient(
            90deg,
            transparent 0%,
            rgba(${rgb}, 0.15) 25%,
            rgba(${rgb}, 0.3) 50%,
            rgba(${rgb}, 0.15) 75%,
            transparent 100%
          )`,
          backgroundSize: '200% 100%',
        }}
        animate={{
          backgroundPosition: ['200% 0%', '-200% 0%'],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'linear',
        }}
      />
    </motion.div>
  );
});

/**
 * Rotating glow for retrieved trait (fetching effect)
 */
const RotatingGlow = memo(function RotatingGlow({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;
  const rgb = hexToRgb(color);

  return (
    <motion.div
      className="absolute inset-0 pointer-events-none rounded-xl overflow-hidden"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <motion.div
        className="absolute inset-0"
        style={{
          background: `conic-gradient(
            from 0deg,
            transparent 0deg,
            rgba(${rgb}, 0.25) 90deg,
            transparent 180deg,
            transparent 360deg
          )`,
        }}
        animate={{ rotate: 360 }}
        transition={{
          duration: 4,
          repeat: Infinity,
          ease: 'linear',
        }}
      />
    </motion.div>
  );
});

/**
 * Color shifting glow for imported trait (external data effect)
 */
const ColorShiftGlow = memo(function ColorShiftGlow({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;
  const rgb = hexToRgb(color);

  return (
    <motion.div
      className="absolute inset-0 pointer-events-none rounded-xl"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      style={{
        boxShadow: `0 0 20px rgba(${rgb}, 0.3)`,
      }}
    >
      <motion.div
        className="absolute inset-0 rounded-xl"
        animate={{
          boxShadow: [
            `inset 0 0 20px rgba(${rgb}, 0.2)`,
            `inset 0 0 30px rgba(${rgb}, 0.35)`,
            `inset 0 0 15px rgba(${rgb}, 0.15)`,
            `inset 0 0 20px rgba(${rgb}, 0.2)`,
          ],
        }}
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
    </motion.div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const TraitGlow = memo(function TraitGlow({
  trait,
  color,
  selected = false,
  isHovered = false,
  performanceConfig,
  intensity = 'medium',
  children,
}: TraitGlowProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const glowMode = getGlowMode(trait);
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Intensity multipliers
  const intensityMap = {
    low: 0.5,
    medium: 1,
    high: 1.5,
  };
  const _multiplier = intensityMap[intensity];

  // Choose variants based on glow mode
  const variants = useMemo(() => {
    switch (glowMode) {
      case 'pulse':
        return createPulseVariants(color);
      case 'breathe':
        return createBreatheVariants(color);
      default:
        return createPulseVariants(color);
    }
  }, [glowMode, color]);

  if (!animationsEnabled) {
    return <div className="relative">{children}</div>;
  }

  return (
    <motion.div
      className="relative"
      variants={variants}
      initial="idle"
      animate={animationState}
    >
      {/* Special effect layers for specific modes */}
      {glowMode === 'flowHorizontal' && (
        <FlowingGlow color={color} active={selected || isHovered} />
      )}
      {glowMode === 'rotate' && (
        <RotatingGlow color={color} active={selected} />
      )}
      {glowMode === 'colorShift' && (
        <ColorShiftGlow color={color} active={selected || isHovered} />
      )}

      {/* Children content */}
      {children}
    </motion.div>
  );
});

// =============================================================================
// Animated Trait Indicator
// =============================================================================

export interface TraitIndicatorAnimatedProps {
  trait: NodeTrait;
  color: string;
  selected?: boolean;
  isHovered?: boolean;
  performanceConfig?: PerformanceConfig;
  size?: 'sm' | 'md' | 'lg';
}

const indicatorSizes = {
  sm: 'w-2 h-2',
  md: 'w-3 h-3',
  lg: 'w-4 h-4',
};

/**
 * Animated dot indicator that represents the trait visually
 */
export const TraitIndicatorAnimated = memo(function TraitIndicatorAnimated({
  trait,
  color,
  selected = false,
  isHovered = false,
  performanceConfig,
  size = 'md',
}: TraitIndicatorAnimatedProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const glowMode = getGlowMode(trait);
  const sizeClass = indicatorSizes[size];

  const baseStyle = {
    backgroundColor: color,
    boxShadow: `0 0 ${selected ? '8px' : isHovered ? '5px' : '3px'} ${color}`,
  };

  if (!animationsEnabled) {
    return (
      <div
        className={`${sizeClass} rounded-full`}
        style={baseStyle}
      />
    );
  }

  // Animation based on trait - using type-safe animation objects
  const getAnimation = (): { scale?: number | number[]; opacity?: number | number[]; x?: number[]; rotate?: number; transition?: object } => {
    const base = { scale: 1, opacity: 1 };

    switch (glowMode) {
      case 'pulse':
        return selected
          ? {
              scale: [1, 1.2, 1],
              opacity: [1, 0.8, 1],
              transition: { duration: 1.2, repeat: Infinity },
            }
          : base;
      case 'breathe':
        return selected
          ? {
              scale: [1, 1.15, 1],
              transition: { duration: 2, repeat: Infinity, ease: 'easeInOut' as const },
            }
          : base;
      case 'colorShift':
        return selected
          ? {
              opacity: [0.7, 1, 0.7],
              transition: { duration: 1.5, repeat: Infinity },
            }
          : base;
      case 'flowHorizontal':
        return selected
          ? {
              x: [-2, 2, -2],
              transition: { duration: 0.8, repeat: Infinity, ease: 'easeInOut' as const },
            }
          : base;
      case 'rotate':
        return selected
          ? {
              rotate: 360,
              transition: { duration: 2, repeat: Infinity, ease: 'linear' as const },
            }
          : base;
      default:
        return base;
    }
  };

  return (
    <motion.div
      className={`${sizeClass} rounded-full`}
      style={baseStyle}
      animate={getAnimation()}
    />
  );
});
