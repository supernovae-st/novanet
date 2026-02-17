'use client';

/**
 * Perspective3D - 3D perspective transform wrapper
 *
 * Adds 3D perspective transforms on hover for premium depth effect.
 * Inspired by MagicUI GlassCard 3D transform patterns.
 *
 * Performance: Requires HIGH+ tier
 *
 * @example
 * ```tsx
 * <Perspective3D rotateX={10} rotateY={15}>
 *   <CardContent />
 * </Perspective3D>
 * ```
 */

import { memo, useMemo, type ReactNode } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS, SPRING_CONFIGS } from '../animationPresets';

// =============================================================================
// Types
// =============================================================================

export interface Perspective3DProps {
  /** Children to wrap with 3D effect */
  children: ReactNode;
  /** Max X-axis rotation in degrees (default: 10) */
  rotateX?: number;
  /** Max Y-axis rotation in degrees (default: 10) */
  rotateY?: number;
  /** Perspective distance in pixels (default: 1000) */
  perspective?: number;
  /** Scale on hover (default: 1.02) */
  hoverScale?: number;
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

const create3DVariants = (rotateX: number, rotateY: number, scale: number): Variants => ({
  idle: {
    rotateX: 0,
    rotateY: 0,
    scale: 1,
    z: 0,
  },
  hover: {
    rotateX: rotateX * 0.5,
    rotateY: rotateY * 0.5,
    scale: scale,
    z: 20,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    rotateX: 0,
    rotateY: 0,
    scale: scale * 1.02,
    z: 40,
    transition: SPRING_CONFIGS.smooth,
  },
});

// =============================================================================
// Component
// =============================================================================

export const Perspective3D = memo(function Perspective3D({
  children,
  rotateX = 10,
  rotateY = 10,
  perspective = 1000,
  hoverScale = 1.02,
  selected = false,
  isHovered = false,
  performanceConfig,
  className,
}: Perspective3DProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  const variants = useMemo(
    () => create3DVariants(rotateX, rotateY, hoverScale),
    [rotateX, rotateY, hoverScale]
  );

  const containerStyle = useMemo(
    () => ({
      perspective: `${perspective}px`,
      transformStyle: 'preserve-3d' as const,
    }),
    [perspective]
  );

  if (!animationsEnabled) {
    return <div className={className}>{children}</div>;
  }

  return (
    <div className={cn('relative', className)} style={containerStyle}>
      <motion.div
        className="relative"
        style={{ transformStyle: 'preserve-3d' }}
        variants={variants}
        initial="idle"
        animate={animationState}
      >
        {children}

        {/* Reflection layer for depth */}
        {isHovered && (
          <motion.div
            className="absolute inset-0 pointer-events-none"
            style={{
              background: 'linear-gradient(135deg, rgba(255,255,255,0.1) 0%, transparent 50%)',
              borderRadius: 'inherit',
            }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: DURATIONS.fast }}
          />
        )}
      </motion.div>
    </div>
  );
});
