'use client';

/**
 * TrailPrimitive - Comet tail effect following particles
 *
 * Visual: Fading trail segments behind moving particles
 * Purpose: Emphasizes direction and motion
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { TRAIL_CONFIG, PARTICLE_SIZES, EASING_PRESETS } from '../../system/constants';

interface TrailSegment {
  id: number;
  timeOffset: number;
  opacity: number;
  sizeRatio: number;
}

export interface TrailPrimitiveProps extends EffectPrimitiveProps {
  /** Number of particles to trail */
  particleCount?: number;
  /** Base particle size */
  particleSize?: number;
}

export const TrailPrimitive = memo(function TrailPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  particleCount = 4,
  particleSize,
}: TrailPrimitiveProps) {
  const isHighlighted = state === 'highlighted';
  const baseSize = particleSize ?? (isHighlighted ? PARTICLE_SIZES.highlighted : PARTICLE_SIZES.base);
  const adjustedSize = baseSize * intensity;

  // Generate trail segment configs - must be before early return
  const trailSegments = useMemo((): TrailSegment[] => {
    return Array.from({ length: TRAIL_CONFIG.segmentCount }, (_, i) => ({
      id: i,
      timeOffset: TRAIL_CONFIG.timeOffset * (i + 1),
      opacity: 0.6 - TRAIL_CONFIG.opacityDecay * i,
      sizeRatio: 1 - TRAIL_CONFIG.sizeDecay * i,
    }));
  }, []);

  const spline = EASING_PRESETS.energetic;

  // Generate trails for each particle - must be before early return
  const particleTrails = useMemo(() => {
    return Array.from({ length: particleCount }, (_, particleIdx) => {
      const particleDelay = (particleIdx / particleCount) * timing.duration;
      const durationVariation = 1 + (particleIdx % 3 === 0 ? 0.1 : particleIdx % 3 === 1 ? -0.05 : 0.03);
      const particleDuration = timing.duration * durationVariation;
      const isLeader = particleIdx === 0;

      return {
        particleIdx,
        particleDelay,
        particleDuration,
        size: isLeader ? adjustedSize * PARTICLE_SIZES.leaderMultiplier : adjustedSize,
      };
    });
  }, [particleCount, timing.duration, adjustedSize]);

  // Don't render if intensity too low
  if (intensity < 0.3) return null;

  return (
    <g className="trail-primitive">
      {particleTrails.map((particle) =>
        trailSegments.map((segment) => {
          const segmentSize = particle.size * segment.sizeRatio * PARTICLE_SIZES.trailMultiplier;
          const segmentDelay = particle.particleDelay + particle.particleDuration * segment.timeOffset;

          return (
            <circle
              key={`trail-${particle.particleIdx}-${segment.id}`}
              r={segmentSize}
              fill={colors.secondary}
              opacity={0}
            >
              <animateMotion
                dur={`${particle.particleDuration}s`}
                repeatCount="indefinite"
                begin={`${segmentDelay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animate
                attributeName="opacity"
                values={`0;${segment.opacity * intensity};${segment.opacity * intensity};0`}
                keyTimes="0;0.1;0.85;1"
                dur={`${particle.particleDuration}s`}
                begin={`${segmentDelay}s`}
                repeatCount="indefinite"
              />
            </circle>
          );
        })
      )}
    </g>
  );
});
