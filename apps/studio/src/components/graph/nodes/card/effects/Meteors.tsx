'use client';

/**
 * Meteors - Shooting stars effect
 *
 * Creates animated meteors streaking across the card from top-right to bottom-left.
 * Inspired by Magic UI Meteors component.
 *
 * Performance Requirements:
 * - HIGH+ tier (multiple animated elements)
 * - Can be disabled via performanceConfig
 */

import { memo, useMemo, useId } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface MeteorsProps {
  /** Number of meteors */
  count?: number;
  /** Primary color for meteor head */
  color?: string;
  /** Trail color (default: same as color) */
  trailColor?: string;
  /** Angle of meteor trajectory in degrees (default: 215 = top-right to bottom-left) */
  angle?: number;
  /** Minimum animation duration in seconds */
  minDuration?: number;
  /** Maximum animation duration in seconds */
  maxDuration?: number;
  /** Minimum delay between meteors */
  minDelay?: number;
  /** Maximum delay between meteors */
  maxDelay?: number;
  /** Meteor width in pixels */
  meteorWidth?: number;
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
// Single Meteor Component
// =============================================================================

interface SingleMeteorProps {
  index: number;
  style: React.CSSProperties;
  color: string;
  trailColor: string;
  duration: number;
  delay: number;
  angle: number;
  meteorWidth: number;
  animationsEnabled: boolean;
}

const SingleMeteor = memo(function SingleMeteor({
  index,
  style,
  color,
  trailColor,
  duration,
  delay,
  angle,
  meteorWidth,
  animationsEnabled,
}: SingleMeteorProps) {
  const MeteorWrapper = animationsEnabled ? motion.div : 'div';

  const meteorVariants: Variants = {
    animate: {
      x: ['0%', '200%'],
      y: ['0%', '200%'],
      opacity: [0, 1, 1, 0],
      transition: {
        duration,
        repeat: Infinity,
        delay,
        ease: 'linear',
      },
    },
  };

  return (
    <MeteorWrapper
      className="pointer-events-none absolute"
      style={{
        ...style,
        width: meteorWidth,
        height: 1,
        transform: `rotate(${angle}deg)`,
        background: `linear-gradient(90deg, ${trailColor}00 0%, ${trailColor}40 20%, ${color} 100%)`,
        boxShadow: `0 0 10px ${color}, 0 0 20px ${trailColor}80`,
        borderRadius: '100%',
      }}
      {...(animationsEnabled && {
        variants: meteorVariants,
        animate: 'animate',
      })}
    >
      {/* Meteor head (bright spot) */}
      <div
        className="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-2 rounded-full"
        style={{
          background: `radial-gradient(circle, white 0%, ${color} 50%, transparent 100%)`,
          boxShadow: `0 0 15px ${color}, 0 0 30px ${color}80`,
          filter: 'blur(0.5px)',
        }}
      />
    </MeteorWrapper>
  );
});

// =============================================================================
// Component
// =============================================================================

export const Meteors = memo(function Meteors({
  count = 8,
  color = '#8b5cf6',
  trailColor,
  angle = 215,
  minDuration = 1.5,
  maxDuration = 3,
  minDelay = 0,
  maxDelay = 4,
  meteorWidth = 80,
  selected = false,
  isHovered = false,
  className,
  performanceConfig,
}: MeteorsProps) {
  const id = useId();

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;

  if (!effectsEnabled) return null;

  const effectiveTrailColor = trailColor ?? color;
  const effectiveCount = selected ? count + 4 : isHovered ? count + 2 : count;

  // Generate meteor data
  const meteors = useMemo(() => {
    return Array.from({ length: effectiveCount }, (_, i) => ({
      // Random starting position (top-right area)
      style: {
        top: `${Math.random() * 40 - 10}%`,
        left: `${Math.random() * 60 + 60}%`,
      },
      duration: minDuration + Math.random() * (maxDuration - minDuration),
      delay: minDelay + Math.random() * (maxDelay - minDelay),
      width: meteorWidth * (0.5 + Math.random() * 0.5),
    }));
  }, [effectiveCount, minDuration, maxDuration, minDelay, maxDelay, meteorWidth]);

  return (
    <div
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 overflow-hidden',
        className
      )}
    >
      {meteors.map((meteor, i) => (
        <SingleMeteor
          key={`meteor-${id}-${i}`}
          index={i}
          style={meteor.style}
          color={color}
          trailColor={effectiveTrailColor}
          duration={meteor.duration}
          delay={meteor.delay}
          angle={angle}
          meteorWidth={meteor.width}
          animationsEnabled={animationsEnabled}
        />
      ))}
    </div>
  );
});
