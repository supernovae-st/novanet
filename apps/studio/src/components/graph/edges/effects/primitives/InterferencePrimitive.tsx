'use client';

/**
 * InterferencePrimitive - Wave interference pattern overlay
 *
 * Visual: Dashed lines moving in opposite directions
 * Purpose: Creates complex wave-like visual effect
 */

import { memo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';

export const InterferencePrimitive = memo(function InterferencePrimitive({
  pathId,
  colors,
  timing,
  intensity,
}: EffectPrimitiveProps) {
  // Don't render if intensity too low
  if (intensity < 0.3) return null;

  const dur1 = `${timing.duration * 0.3}s`;
  const dur2 = `${timing.duration * 0.4}s`;

  return (
    <g className="interference-primitive">
      {/* Wave 1 - forward */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.secondary}
        strokeWidth={1}
        strokeDasharray="2,8"
        opacity={0.3 * intensity}
      >
        <animate
          attributeName="stroke-dashoffset"
          values="0;20"
          dur={dur1}
          repeatCount="indefinite"
        />
      </use>

      {/* Wave 2 - offset */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.tertiary}
        strokeWidth={1}
        strokeDasharray="2,8"
        opacity={0.25 * intensity}
      >
        <animate
          attributeName="stroke-dashoffset"
          values="10;30"
          dur={dur2}
          repeatCount="indefinite"
        />
      </use>
    </g>
  );
});
