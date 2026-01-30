'use client';

/**
 * ZigzagPrimitive - Neural branching electric effect
 *
 * Visual: Particles with zigzag oscillation and branching sparks
 * Purpose: Brain electricity / neural firing metaphor
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { ZIGZAG_CONFIG, PARTICLE_SIZES, EASING_PRESETS } from '../../system/constants';

export const ZigzagPrimitive = memo(function ZigzagPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
}: EffectPrimitiveProps) {
  const isHighlighted = state === 'highlighted';
  const scale = isHighlighted ? 1.4 : 1;
  const baseSize = PARTICLE_SIZES.base * intensity * scale;
  const amplitude = ZIGZAG_CONFIG.amplitude * intensity * scale;

  // Generate zigzag oscillation values - must be before early return
  const zigzagValues = useMemo(() => {
    const segments = ZIGZAG_CONFIG.segments;
    const values: string[] = [];
    for (let i = 0; i <= segments * 2; i++) {
      const offset = (i % 2 === 0 ? -1 : 1) * amplitude * (1 - i / (segments * 2) * 0.5);
      values.push(`0,${offset.toFixed(1)}`);
    }
    return values.join(';');
  }, [amplitude]);

  // Flicker values for electric effect
  const flickerValues = '1;0.6;1;0.8;1;0.7;1';
  const flickerTimes = '0;0.15;0.3;0.45;0.6;0.75;1';

  // Generate 3-4 zigzag particles with stagger - must be before early return
  const particles = useMemo(() => {
    const count = isHighlighted ? 4 : 3;
    return Array.from({ length: count }, (_, i) => ({
      id: i,
      delay: (i / count) * timing.duration,
      duration: timing.duration * (1 + (i % 2 === 0 ? 0.1 : -0.05)),
      hasBranch: i % 2 === 0 && Math.random() < ZIGZAG_CONFIG.branchProbability,
    }));
  }, [timing.duration, isHighlighted]);

  // Don't render if intensity too low
  if (intensity < 0.4) return null;

  const spline = EASING_PRESETS.energetic;

  return (
    <g className="zigzag-primitive">
      {particles.map((particle) => (
        <g key={`zigzag-${particle.id}`}>
          {/* Glow envelope */}
          <circle
            r={baseSize * 2}
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
              values={flickerValues.split(';').map(v => `${parseFloat(v) * 0.3 * intensity}`).join(';')}
              keyTimes={flickerTimes}
              dur={`${particle.duration * 0.15}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
            <animateTransform
              attributeName="transform"
              type="translate"
              values={zigzagValues}
              dur={`${particle.duration * 0.2}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
          </circle>

          {/* Main particle */}
          <circle
            r={baseSize}
            fill={colors.primary}
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
              values={flickerValues.split(';').map(v => `${parseFloat(v) * 0.9 * intensity}`).join(';')}
              keyTimes={flickerTimes}
              dur={`${particle.duration * 0.12}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
            <animateTransform
              attributeName="transform"
              type="translate"
              values={zigzagValues}
              dur={`${particle.duration * 0.18}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
          </circle>

          {/* White hot core */}
          <circle
            r={baseSize * 0.5}
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
              values={flickerValues.split(';').map(v => `${parseFloat(v) * 0.8 * intensity}`).join(';')}
              keyTimes={flickerTimes}
              dur={`${particle.duration * 0.08}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
            <animateTransform
              attributeName="transform"
              type="translate"
              values={zigzagValues}
              dur={`${particle.duration * 0.15}s`}
              repeatCount="indefinite"
              begin={`${particle.delay}s`}
            />
          </circle>
        </g>
      ))}
    </g>
  );
});
