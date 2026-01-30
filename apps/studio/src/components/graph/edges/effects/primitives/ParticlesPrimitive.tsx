'use client';

/**
 * ParticlesPrimitive - Traveling data packets along edge
 *
 * Visual: Glowing orbs moving from source to target with comet trail
 * Purpose: Shows data/energy flow direction
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { PARTICLE_SIZES, EASING_PRESETS } from '../../system/constants';

interface ParticleConfig {
  id: number;
  delay: number;
  duration: number;
  size: number;
  isLeader: boolean;
}

export interface ParticlesPrimitiveProps extends EffectPrimitiveProps {
  /** Number of particles */
  particleCount?: number;
  /** Base particle size */
  particleSize?: number;
}

export const ParticlesPrimitive = memo(function ParticlesPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  particleCount = 4,
  particleSize,
}: ParticlesPrimitiveProps) {
  const isHighlighted = state === 'highlighted';
  const baseSize = particleSize ?? (isHighlighted ? PARTICLE_SIZES.highlighted : PARTICLE_SIZES.base);
  const adjustedSize = baseSize * intensity;

  // Generate particle configurations - must be before early return
  const particles = useMemo((): ParticleConfig[] => {
    return Array.from({ length: particleCount }, (_, i) => {
      const stagger = (i / particleCount) * timing.duration;
      // Vary duration slightly for organic feel
      const durationVariation = 1 + (i % 3 === 0 ? 0.1 : i % 3 === 1 ? -0.05 : 0.03);

      return {
        id: i,
        delay: stagger,
        duration: timing.duration * durationVariation,
        size: i === 0 ? adjustedSize * PARTICLE_SIZES.leaderMultiplier : adjustedSize,
        isLeader: i === 0,
      };
    });
  }, [particleCount, timing.duration, adjustedSize]);

  // Don't render if intensity too low
  if (intensity < 0.2) return null;

  const spline = EASING_PRESETS.energetic;

  return (
    <g className="particles-primitive">
      {particles.map((particle) => (
        <g key={`particle-${particle.id}`}>
          {/* Outer glow */}
          <circle
            r={particle.size * 2}
            fill={colors.glow}
            opacity={0}
            style={{ filter: 'blur(6px)' }}
          >
            <animateMotion
              dur={`${particle.duration}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.4 * intensity};${0.4 * intensity};0`}
              keyTimes="0;0.1;0.85;1"
              dur={`${particle.duration}s`}
              begin={`${particle.delay}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Main particle body */}
          <circle
            r={particle.size}
            fill={particle.isLeader ? colors.primary : colors.secondary}
            opacity={0}
          >
            <animateMotion
              dur={`${particle.duration}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.9 * intensity};${0.9 * intensity};0`}
              keyTimes="0;0.05;0.9;1"
              dur={`${particle.duration}s`}
              begin={`${particle.delay}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Bright core */}
          <circle
            r={particle.size * 0.5}
            fill="#ffffff"
            opacity={0}
          >
            <animateMotion
              dur={`${particle.duration}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.85 * intensity};${0.85 * intensity};0`}
              keyTimes="0;0.05;0.9;1"
              dur={`${particle.duration}s`}
              begin={`${particle.delay}s`}
              repeatCount="indefinite"
            />
          </circle>
        </g>
      ))}
    </g>
  );
});
