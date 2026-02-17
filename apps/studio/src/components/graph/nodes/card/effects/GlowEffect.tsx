'use client';

/**
 * GlowEffect - Layered glow effect for premium cards
 *
 * Creates a multi-layer glow effect with blur for depth.
 * Inspired by MagicUI GlassCard patterns.
 *
 * Performance: Requires MEDIUM+ tier (performanceConfig.effects.outerGlow)
 *
 * @example
 * ```tsx
 * <GlowEffect color="#8b5cf6" intensity="high" />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../animationPresets';

// =============================================================================
// Types
// =============================================================================

export type GlowIntensity = 'low' | 'medium' | 'high' | 'ultra';

export interface GlowEffectProps {
  /** Primary glow color */
  color: string;
  /** Glow intensity level */
  intensity?: GlowIntensity;
  /** Whether the element is selected */
  selected?: boolean;
  /** Whether the element is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Additional CSS classes */
  className?: string;
}

// =============================================================================
// Constants
// =============================================================================

const INTENSITY_CONFIG: Record<GlowIntensity, { blur1: number; blur2: number; opacity1: number; opacity2: number }> = {
  low: { blur1: 8, blur2: 4, opacity1: 0.15, opacity2: 0.25 },
  medium: { blur1: 16, blur2: 8, opacity1: 0.20, opacity2: 0.35 },
  high: { blur1: 24, blur2: 12, opacity1: 0.25, opacity2: 0.45 },
  ultra: { blur1: 32, blur2: 16, opacity1: 0.30, opacity2: 0.55 },
};

// =============================================================================
// Animation Variants
// =============================================================================

const glowLayerVariants: Variants = {
  idle: { scale: 1, opacity: 0.3 },
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

const pulseVariants: Variants = {
  idle: { scale: 1 },
  animate: {
    scale: [1, 1.05, 1],
    opacity: [0.4, 0.6, 0.4],
    transition: {
      duration: 2,
      ease: 'easeInOut',
      repeat: Infinity,
    },
  },
};

// =============================================================================
// Component
// =============================================================================

export const GlowEffect = memo(function GlowEffect({
  color,
  intensity = 'medium',
  selected = false,
  isHovered = false,
  performanceConfig,
  className,
}: GlowEffectProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  const config = INTENSITY_CONFIG[intensity];

  // Memoize styles
  const layerStyles = useMemo(
    () => ({
      outer: {
        background: `radial-gradient(ellipse at center, ${color}${Math.round(config.opacity1 * 255).toString(16).padStart(2, '0')} 0%, transparent 70%)`,
        filter: `blur(${config.blur1}px)`,
      },
      inner: {
        background: `radial-gradient(ellipse at center, ${color}${Math.round(config.opacity2 * 255).toString(16).padStart(2, '0')} 0%, transparent 60%)`,
        filter: `blur(${config.blur2}px)`,
      },
    }),
    [color, config]
  );

  const OuterLayer = animationsEnabled ? motion.div : 'div';
  const InnerLayer = animationsEnabled ? motion.div : 'div';

  return (
    <div className={cn('absolute inset-0 pointer-events-none overflow-hidden', className)}>
      {/* Outer glow layer - larger, more diffuse */}
      <OuterLayer
        className="absolute inset-[-20%] rounded-full"
        style={layerStyles.outer}
        {...(animationsEnabled && {
          variants: glowLayerVariants,
          initial: 'idle',
          animate: animationState,
        })}
      />

      {/* Inner glow layer - tighter, more intense */}
      <InnerLayer
        className="absolute inset-[-10%] rounded-full"
        style={layerStyles.inner}
        {...(animationsEnabled && selected && {
          variants: pulseVariants,
          initial: 'idle',
          animate: 'animate',
        })}
        {...(animationsEnabled && !selected && {
          variants: glowLayerVariants,
          initial: 'idle',
          animate: animationState,
        })}
      />
    </div>
  );
});
