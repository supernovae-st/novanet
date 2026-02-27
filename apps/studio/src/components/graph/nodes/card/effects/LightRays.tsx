'use client';

/**
 * LightRays - Volumetric light beams effect
 *
 * Creates animated light rays emanating from the top of the card,
 * simulating volumetric lighting like in a sci-fi command center.
 *
 * Performance Requirements:
 * - HIGH+ tier (gradient animations)
 * - Uses CSS transforms for hardware acceleration
 */

import { memo, useMemo, useId } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface LightRaysProps {
  /** Number of light rays */
  count?: number;
  /** Primary color for the rays */
  color?: string;
  /** Blur amount in pixels */
  blur?: number;
  /** Base opacity of rays */
  opacity?: number;
  /** Animation speed multiplier */
  speed?: number;
  /** Ray length relative to container height (0-1) */
  length?: number;
  /** Origin point: 'top' | 'center' | 'bottom' */
  origin?: 'top' | 'center' | 'bottom';
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
// Single Light Ray Component
// =============================================================================

interface SingleRayProps {
  index: number;
  angle: number;
  color: string;
  blur: number;
  opacity: number;
  speed: number;
  length: number;
  width: number;
  origin: 'top' | 'center' | 'bottom';
  animationsEnabled: boolean;
}

const SingleRay = memo(function SingleRay({
  index,
  angle,
  color,
  blur,
  opacity,
  speed,
  length,
  width,
  origin,
  animationsEnabled,
}: SingleRayProps) {
  const RayWrapper = animationsEnabled ? motion.div : 'div';

  // Origin position
  const originY = origin === 'top' ? '0%' : origin === 'center' ? '50%' : '100%';

  const rayVariants: Variants = {
    animate: {
      opacity: [opacity * 0.3, opacity, opacity * 0.3],
      scaleY: [0.8, 1.1, 0.8],
      transition: {
        duration: 2 + (index % 3) * speed,
        repeat: Infinity,
        ease: 'easeInOut',
        delay: index * 0.2,
      },
    },
  };

  return (
    <RayWrapper
      className="absolute pointer-events-none"
      style={{
        top: originY,
        left: '50%',
        width: `${width}px`,
        height: `${length * 100}%`,
        transform: `translateX(-50%) rotate(${angle}deg)`,
        transformOrigin: origin === 'top' ? 'top center' : origin === 'bottom' ? 'bottom center' : 'center center',
        background: `linear-gradient(180deg,
          ${color}00 0%,
          ${color}60 20%,
          ${color}40 50%,
          ${color}10 80%,
          ${color}00 100%
        )`,
        filter: `blur(${blur}px)`,
        mixBlendMode: 'screen',
      }}
      {...(animationsEnabled && {
        variants: rayVariants,
        animate: 'animate',
      })}
    />
  );
});

// =============================================================================
// Component
// =============================================================================

export const LightRays = memo(function LightRays({
  count = 5,
  color = '#8b5cf6',
  blur = 20,
  opacity = 0.4,
  speed = 1,
  length = 1.2,
  origin = 'top',
  selected = false,
  isHovered = false,
  className,
  performanceConfig,
}: LightRaysProps) {
  const id = useId();

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;

  const effectiveOpacity = selected ? opacity * 1.5 : isHovered ? opacity * 1.2 : opacity;
  const effectiveCount = selected ? count + 2 : count;

  // Generate ray data with spread angles
  const rays = useMemo(() => {
    if (!effectsEnabled) return [];


    const spreadAngle = 60; // Total spread angle
    const startAngle = -spreadAngle / 2;

    return Array.from({ length: effectiveCount }, (_, i) => {
      const baseAngle = startAngle + (spreadAngle / (effectiveCount - 1)) * i;
      return {
        angle: baseAngle + (Math.random() - 0.5) * 10,
        width: 30 + Math.random() * 40,
        opacity: effectiveOpacity * (0.5 + Math.random() * 0.5),
      };
    });
  }, [effectsEnabled, effectiveCount, effectiveOpacity]);

  if (!effectsEnabled) return null;

  return (
    <div
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 overflow-hidden',
        className
      )}
    >
      {rays.map((ray, i) => (
        <SingleRay
          key={`ray-${id}-${i}`}
          index={i}
          angle={ray.angle}
          color={color}
          blur={blur}
          opacity={ray.opacity}
          speed={speed}
          length={length}
          width={ray.width}
          origin={origin}
          animationsEnabled={animationsEnabled}
        />
      ))}

      {/* Central bright spot at origin */}
      <div
        className="absolute pointer-events-none"
        style={{
          top: origin === 'top' ? '0' : origin === 'center' ? '50%' : '100%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          width: 100,
          height: 50,
          background: `radial-gradient(ellipse, ${color}40 0%, ${color}00 70%)`,
          filter: `blur(${blur / 2}px)`,
        }}
      />
    </div>
  );
});
