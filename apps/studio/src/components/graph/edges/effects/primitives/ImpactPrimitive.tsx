'use client';

/**
 * ImpactPrimitive - Burst effect at target node
 *
 * Visual: Flash + expanding ripples + particle burst
 * Purpose: Shows where energy arrives/impacts
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { IMPACT_CONFIG } from '../../system/constants';

export const ImpactPrimitive = memo(function ImpactPrimitive({
  colors,
  timing,
  intensity,
  state,
  targetPosition,
}: EffectPrimitiveProps) {
  // Guard against undefined position
  if (!targetPosition || typeof targetPosition.x !== 'number') {
    return null;
  }

  const isHighlighted = state === 'highlighted';
  const scale = isHighlighted ? IMPACT_CONFIG.highlightedScale : 1;
  const baseSize = IMPACT_CONFIG.baseSize * intensity * scale;
  const rippleSize = baseSize * IMPACT_CONFIG.rippleMultiplier;

  // Delay impact to sync with particle arrival (~85% of duration)
  const delay = timing.duration * 0.85;

  // Burst particle angles - must be before early return
  const burstAngles = useMemo(() => {
    const count = IMPACT_CONFIG.burstCount;
    return Array.from({ length: count }, (_, i) => (i * 360) / count);
  }, []);

  // Don't render if intensity too low
  if (intensity < 0.1) return null;

  const dur = `${timing.duration}s`;
  const burstDur = `${timing.duration * 0.4}s`;

  return (
    <g className="impact-primitive">
      {/* Outer ripple */}
      <circle
        cx={targetPosition.x}
        cy={targetPosition.y}
        r={baseSize}
        fill="none"
        stroke={colors.glow}
        strokeWidth={2}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize};${rippleSize};${baseSize}`}
          dur={dur}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.7 * intensity};0;0`}
          keyTimes="0;0.7;1"
          dur={dur}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Inner ripple */}
      <circle
        cx={targetPosition.x}
        cy={targetPosition.y}
        r={baseSize * 0.5}
        fill="none"
        stroke={colors.secondary}
        strokeWidth={1.5}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.3};${rippleSize * 0.6};${baseSize * 0.3}`}
          dur={`${timing.duration * 0.8}s`}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.6 * intensity};0;0`}
          keyTimes="0;0.6;1"
          dur={`${timing.duration * 0.8}s`}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Core flash */}
      <circle
        cx={targetPosition.x}
        cy={targetPosition.y}
        r={baseSize}
        fill={colors.primary}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.5};${baseSize * 1.5};${baseSize * 0.5}`}
          dur={burstDur}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.8 * intensity};0;0`}
          keyTimes="0;0.5;1"
          dur={burstDur}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* White flash */}
      <circle
        cx={targetPosition.x}
        cy={targetPosition.y}
        r={baseSize * 0.3}
        fill="#ffffff"
        opacity={0}
      >
        <animate
          attributeName="opacity"
          values={`${0.9 * intensity};0;0`}
          keyTimes="0;0.3;1"
          dur={burstDur}
          begin={`${delay}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Burst particles */}
      {burstAngles.map((angle, i) => {
        const radians = (angle * Math.PI) / 180;
        const distance = baseSize * IMPACT_CONFIG.burstDistanceMultiplier;
        const endX = targetPosition.x + Math.cos(radians) * distance;
        const endY = targetPosition.y + Math.sin(radians) * distance;
        const particleSize = baseSize * 0.3 * (1 - i * 0.05);

        return (
          <circle
            key={`burst-${i}`}
            cx={targetPosition.x}
            cy={targetPosition.y}
            r={particleSize}
            fill={i % 2 === 0 ? colors.secondary : colors.tertiary}
            opacity={0}
          >
            <animate
              attributeName="cx"
              values={`${targetPosition.x};${endX}`}
              dur={burstDur}
              begin={`${delay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="cy"
              values={`${targetPosition.y};${endY}`}
              dur={burstDur}
              begin={`${delay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="opacity"
              values={`${0.8 * intensity};0`}
              dur={burstDur}
              begin={`${delay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="r"
              values={`${particleSize};${particleSize * 0.3}`}
              dur={burstDur}
              begin={`${delay}s`}
              repeatCount="indefinite"
            />
          </circle>
        );
      })}
    </g>
  );
});
