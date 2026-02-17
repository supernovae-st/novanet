'use client';

/**
 * GridPattern - Animated grid background effect
 *
 * Inspired by Magic UI Grid Pattern component.
 * Creates an SVG-based grid pattern with optional highlighted squares
 * that can animate/flicker for a premium visual effect.
 *
 * Performance Requirements:
 * - MEDIUM+ tier (SVG is lightweight)
 * - Can be disabled via performanceConfig
 */

import { memo, useId, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface GridPatternProps {
  /** Width of each grid cell */
  width?: number;
  /** Height of each grid cell */
  height?: number;
  /** X offset of the pattern */
  x?: number;
  /** Y offset of the pattern */
  y?: number;
  /** Stroke dash array (0 = solid, "4" = dashed) */
  strokeDasharray?: string;
  /** Grid line color */
  color?: string;
  /** Opacity of grid lines */
  opacity?: number;
  /** Array of [x, y] coordinates for highlighted squares */
  squares?: [number, number][];
  /** Color for highlighted squares */
  squareColor?: string;
  /** Additional CSS classes */
  className?: string;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Enable flickering animation on squares */
  flicker?: boolean;
  /** Selected state - intensifies effect */
  selected?: boolean;
  /** Hovered state */
  isHovered?: boolean;
}

// =============================================================================
// Animation Variants
// =============================================================================

const squareVariants: Variants = {
  idle: (i: number) => ({
    opacity: [0.2, 0.6, 0.2],
    transition: {
      duration: 2 + (i % 3),
      repeat: Infinity,
      delay: i * 0.3,
    },
  }),
  selected: {
    opacity: 0.9,
    scale: 1.2,
    transition: { duration: 0.3 },
  },
};

// =============================================================================
// Component
// =============================================================================

export const GridPattern = memo(function GridPattern({
  width = 20,
  height = 20,
  x = -1,
  y = -1,
  strokeDasharray = '0',
  color = 'currentColor',
  opacity = 0.15,
  squares = [],
  squareColor,
  className,
  performanceConfig,
  flicker = true,
  selected = false,
  isHovered = false,
}: GridPatternProps) {
  const id = useId();
  const patternId = `grid-pattern-${id}`;

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;

  // Generate default squares if none provided
  const displaySquares = useMemo(() => {
    if (squares.length > 0) return squares;
    // Default: create a subtle pattern of 6 squares
    return [
      [1, 1],
      [3, 2],
      [5, 1],
      [2, 4],
      [4, 3],
      [6, 5],
    ] as [number, number][];
  }, [squares]);

  const effectiveSquareColor = squareColor ?? color;
  const effectiveOpacity = selected ? opacity * 1.5 : isHovered ? opacity * 1.2 : opacity;

  if (!effectsEnabled) return null;

  return (
    <svg
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 h-full w-full',
        className
      )}
      style={{ opacity: effectiveOpacity }}
    >
      <defs>
        <pattern
          id={patternId}
          width={width}
          height={height}
          patternUnits="userSpaceOnUse"
          x={x}
          y={y}
        >
          <path
            d={`M.5 ${height}V.5H${width}`}
            fill="none"
            stroke={color}
            strokeOpacity={0.4}
            strokeDasharray={strokeDasharray}
          />
        </pattern>
      </defs>

      {/* Grid pattern background */}
      <rect
        width="100%"
        height="100%"
        strokeWidth={0}
        fill={`url(#${patternId})`}
      />

      {/* Highlighted/flickering squares */}
      {displaySquares.map(([sqX, sqY], i) => {
        const SquareElement = animationsEnabled && flicker ? motion.rect : 'rect';
        return (
          <SquareElement
            key={`sq-${i}-${sqX}-${sqY}`}
            x={sqX * width + 1}
            y={sqY * height + 1}
            width={width - 2}
            height={height - 2}
            fill={effectiveSquareColor}
            rx={2}
            {...(animationsEnabled && flicker && {
              variants: squareVariants,
              initial: 'idle',
              animate: selected ? 'selected' : 'idle',
              custom: i,
            })}
            {...(!animationsEnabled && {
              opacity: 0.3,
            })}
          />
        );
      })}
    </svg>
  );
});

// =============================================================================
// DotPattern - Alternative pattern with dots instead of grid lines
// =============================================================================

export interface DotPatternProps {
  /** Spacing between dots */
  spacing?: number;
  /** Dot radius */
  radius?: number;
  /** Dot color */
  color?: string;
  /** Opacity */
  opacity?: number;
  /** Additional CSS classes */
  className?: string;
  /** Selected state */
  selected?: boolean;
  /** Hovered state */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

export const DotPattern = memo(function DotPattern({
  spacing = 16,
  radius = 1,
  color = 'currentColor',
  opacity = 0.3,
  className,
  selected = false,
  isHovered = false,
  performanceConfig,
}: DotPatternProps) {
  const id = useId();
  const patternId = `dot-pattern-${id}`;

  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;
  const effectiveOpacity = selected ? opacity * 1.5 : isHovered ? opacity * 1.2 : opacity;

  if (!effectsEnabled) return null;

  return (
    <svg
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 h-full w-full',
        className
      )}
      style={{ opacity: effectiveOpacity }}
    >
      <defs>
        <pattern
          id={patternId}
          width={spacing}
          height={spacing}
          patternUnits="userSpaceOnUse"
        >
          <circle
            cx={spacing / 2}
            cy={spacing / 2}
            r={radius}
            fill={color}
          />
        </pattern>
      </defs>
      <rect
        width="100%"
        height="100%"
        fill={`url(#${patternId})`}
      />
    </svg>
  );
});
