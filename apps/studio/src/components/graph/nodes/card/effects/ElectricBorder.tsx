'use client';

/**
 * ElectricBorder - Animated gradient border effect
 *
 * Creates an animated border with flowing gradient effect.
 * Inspired by MagicUI ElectricCard SVG filter patterns.
 *
 * Performance: Requires ULTRA tier only (heavy GPU usage)
 *
 * @example
 * ```tsx
 * <ElectricBorder color="#8b5cf6" secondaryColor="#6366f1" />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export type ElectricStyle = 'flow' | 'pulse' | 'shimmer';

export interface ElectricBorderProps {
  /** Primary color */
  color: string;
  /** Secondary color for gradient */
  secondaryColor?: string;
  /** Border radius in pixels */
  borderRadius?: number;
  /** Border thickness in pixels */
  thickness?: number;
  /** Animation style */
  style?: ElectricStyle;
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
// Animation Variants
// =============================================================================

const flowVariants: Variants = {
  animate: {
    backgroundPosition: ['0% 50%', '100% 50%', '0% 50%'],
    transition: {
      duration: 3,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

const pulseVariants: Variants = {
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

const shimmerVariants: Variants = {
  animate: {
    backgroundPosition: ['-200% 0%', '200% 0%'],
    transition: {
      duration: 2,
      ease: 'linear',
      repeat: Infinity,
    },
  },
};

const STYLE_VARIANTS: Record<ElectricStyle, Variants> = {
  flow: flowVariants,
  pulse: pulseVariants,
  shimmer: shimmerVariants,
};

// =============================================================================
// Component
// =============================================================================

export const ElectricBorder = memo(function ElectricBorder({
  color,
  secondaryColor,
  borderRadius = 16,
  thickness = 2,
  style = 'flow',
  selected = false,
  isHovered = false,
  performanceConfig,
  className,
}: ElectricBorderProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.outerGlow ?? true;

  const secondary = secondaryColor || adjustColor(color, 30);

  // Gradient background for electric effect
  const gradientStyle = useMemo(
    () => ({
      background: style === 'shimmer'
        ? `linear-gradient(90deg, transparent 0%, ${color} 25%, ${secondary} 50%, ${color} 75%, transparent 100%)`
        : `linear-gradient(90deg, ${color}, ${secondary}, ${color}, ${secondary}, ${color})`,
      backgroundSize: style === 'shimmer' ? '200% 100%' : '300% 100%',
      borderRadius,
      padding: thickness,
    }),
    [color, secondary, borderRadius, thickness, style]
  );

  // Glow style for selected state
  const glowStyle = useMemo(
    () =>
      selected
        ? {
            boxShadow: `0 0 20px ${color}60, 0 0 40px ${color}30, inset 0 0 20px ${color}20`,
          }
        : isHovered
          ? {
              boxShadow: `0 0 15px ${color}40, 0 0 30px ${color}20`,
            }
          : {},
    [color, selected, isHovered]
  );

  if (!effectsEnabled) {
    return null;
  }

  const BorderWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <BorderWrapper
      className={cn('absolute inset-0 pointer-events-none', className)}
      style={{ ...gradientStyle, ...glowStyle }}
      {...(animationsEnabled && {
        variants: STYLE_VARIANTS[style],
        initial: false,
        animate: 'animate',
      })}
    >
      {/* Inner mask to create border effect */}
      <div
        className="w-full h-full"
        style={{
          backgroundColor: 'var(--card-bg, #0a0a0f)',
          borderRadius: borderRadius - thickness,
        }}
      />
    </BorderWrapper>
  );
});

// =============================================================================
// Utilities
// =============================================================================

/**
 * Adjust a hex color by a hue offset
 */
function adjustColor(hex: string, offset: number): string {
  // Simple hue rotation approximation
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);

  // Rotate through RGB channels
  const newR = Math.min(255, Math.max(0, r + offset));
  const newG = Math.min(255, Math.max(0, g + offset));
  const newB = Math.min(255, Math.max(0, b - offset));

  return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`;
}
