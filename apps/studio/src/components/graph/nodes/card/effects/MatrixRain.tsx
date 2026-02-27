'use client';

/**
 * MatrixRain - Digital rain effect inspired by The Matrix
 *
 * Creates falling character columns with glowing trails.
 * Performance-aware: Uses CSS animations with hardware acceleration.
 *
 * Performance Requirements:
 * - ULTRA tier only (complex animation)
 * - Automatically disabled at lower tiers
 */

import { memo, useMemo, useId } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface MatrixRainProps {
  /** Primary color for the rain characters */
  color?: string;
  /** Secondary glow color */
  glowColor?: string;
  /** Number of rain columns */
  columns?: number;
  /** Characters to use (default: katakana + numbers) */
  characters?: string;
  /** Speed multiplier (1 = normal) */
  speed?: number;
  /** Opacity of the rain */
  opacity?: number;
  /** Selected state - intensifies effect */
  selected?: boolean;
  /** Hovered state */
  isHovered?: boolean;
  /** Additional CSS classes */
  className?: string;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Constants
// =============================================================================

// Katakana + numbers + symbols for authentic Matrix feel
const DEFAULT_CHARS = 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ<>{}[]|\\/:;!@#$%^&*()+-=~`';

// =============================================================================
// Animation Variants
// =============================================================================

const columnVariants: Variants = {
  animate: (i: number) => ({
    y: ['0%', '100%'],
    transition: {
      y: {
        duration: 3 + (i % 4) * 0.5,
        repeat: Infinity,
        ease: 'linear',
        delay: i * 0.1,
      },
    },
  }),
};

const charVariants: Variants = {
  animate: (i: number) => ({
    opacity: [0.3, 1, 0.3],
    scale: [0.8, 1.1, 0.8],
    transition: {
      duration: 0.5 + (i % 3) * 0.2,
      repeat: Infinity,
      repeatType: 'reverse',
      delay: i * 0.05,
    },
  }),
};

// =============================================================================
// Single Rain Column Component
// =============================================================================

interface RainColumnProps {
  index: number;
  x: number;
  color: string;
  glowColor: string;
  characters: string;
  charCount: number;
  animationsEnabled: boolean;
}

const RainColumn = memo(function RainColumn({
  index,
  x,
  color,
  glowColor,
  characters,
  charCount,
  animationsEnabled,
}: RainColumnProps) {
  // Generate random characters for this column
  const chars = useMemo(() => {
    const result: string[] = [];
    for (let i = 0; i < charCount; i++) {
      result.push(characters[Math.floor(Math.random() * characters.length)]);
    }
    return result;
  }, [characters, charCount]);

  const ColumnWrapper = animationsEnabled ? motion.div : 'div';
  const CharWrapper = animationsEnabled ? motion.span : 'span';

  return (
    <ColumnWrapper
      className="absolute flex flex-col items-center pointer-events-none"
      style={{
        left: `${x}%`,
        top: '-20%',
        writingMode: 'vertical-rl',
        textOrientation: 'upright',
        fontFamily: 'monospace',
        fontSize: '10px',
        lineHeight: 1.2,
      }}
      {...(animationsEnabled && {
        variants: columnVariants,
        animate: 'animate',
        custom: index,
      })}
    >
      {chars.map((char, i) => {
        // First char is brightest (head of the trail)
        const isHead = i === 0;
        const fadeOpacity = isHead ? 1 : Math.max(0.1, 1 - (i / charCount) * 0.8);

        return (
          <CharWrapper
            key={`${index}-${i}`}
            style={{
              color: isHead ? '#ffffff' : color,
              opacity: fadeOpacity,
              textShadow: isHead
                ? `0 0 20px ${glowColor}, 0 0 40px ${glowColor}, 0 0 60px ${color}`
                : `0 0 10px ${glowColor}`,
              filter: isHead ? 'brightness(1.5)' : undefined,
            }}
            {...(animationsEnabled && isHead && {
              variants: charVariants,
              animate: 'animate',
              custom: i,
            })}
          >
            {char}
          </CharWrapper>
        );
      })}
    </ColumnWrapper>
  );
});

// =============================================================================
// Component
// =============================================================================

export const MatrixRain = memo(function MatrixRain({
  color = '#22c55e',
  glowColor,
  columns = 12,
  characters = DEFAULT_CHARS,
  speed: _speed = 1,
  opacity = 0.6,
  selected = false,
  isHovered = false,
  className,
  performanceConfig,
}: MatrixRainProps) {
  const id = useId();

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;

  const effectiveGlowColor = glowColor ?? color;
  const effectiveOpacity = selected ? opacity * 1.3 : isHovered ? opacity * 1.1 : opacity;
  const effectiveColumns = Math.min(columns, 20); // Cap for performance

  // Generate column positions
  const columnData = useMemo(() => {
    // Only show in ULTRA tier or when premium effects are enabled
    if (!effectsEnabled) return [];

    return Array.from({ length: effectiveColumns }, (_item, i) => ({
      x: (i / effectiveColumns) * 100 + Math.random() * 5,
      charCount: 8 + Math.floor(Math.random() * 8),
    }));
  }, [effectsEnabled, effectiveColumns]);

  if (!effectsEnabled) return null;

  return (
    <div
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 overflow-hidden',
        className
      )}
      style={{
        opacity: effectiveOpacity,
        maskImage: 'linear-gradient(to bottom, transparent 0%, black 10%, black 90%, transparent 100%)',
        WebkitMaskImage: 'linear-gradient(to bottom, transparent 0%, black 10%, black 90%, transparent 100%)',
      }}
    >
      {columnData.map((col, i) => (
        <RainColumn
          key={`rain-${id}-${i}`}
          index={i}
          x={col.x}
          color={color}
          glowColor={effectiveGlowColor}
          characters={characters}
          charCount={col.charCount}
          animationsEnabled={animationsEnabled}
        />
      ))}
    </div>
  );
});
