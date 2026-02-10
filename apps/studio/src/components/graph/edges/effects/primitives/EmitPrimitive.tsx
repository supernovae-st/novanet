'use client';

/**
 * EmitPrimitive - Pulse/burst effect at source node
 *
 * Visual: Expanding rings with glowing center
 * Purpose: Shows where energy originates from
 */

import { memo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { EMIT_CONFIG } from '../../system/constants';

export const EmitPrimitive = memo(function EmitPrimitive({
  colors,
  timing,
  intensity,
  state,
  sourcePosition,
}: EffectPrimitiveProps) {
  // Guard against undefined position
  if (!sourcePosition || typeof sourcePosition.x !== 'number') {
    return null;
  }

  const isHighlighted = state === 'highlighted';
  const scale = isHighlighted ? EMIT_CONFIG.highlightedScale : 1;
  const baseSize = EMIT_CONFIG.baseSize * intensity * scale;
  const ringSize = baseSize * EMIT_CONFIG.ringMultiplier;

  // Don't render if intensity too low
  if (intensity < 0.1) {
    return null;
  }

  const dur = `${timing.duration}s`;

  return (
    <g className="emit-primitive">
      {/* Outer pulse ring */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize}
        fill="none"
        stroke={colors.glow}
        strokeWidth={2}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize};${ringSize};${baseSize}`}
          dur={dur}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.6 * intensity};0;${0.6 * intensity}`}
          dur={dur}
          repeatCount="indefinite"
        />
      </circle>

      {/* Inner pulse ring (offset) */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize * 0.6}
        fill="none"
        stroke={colors.secondary}
        strokeWidth={1.5}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.5};${ringSize * 0.7};${baseSize * 0.5}`}
          dur={`${timing.duration * 0.7}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.5 * intensity};0;${0.5 * intensity}`}
          dur={`${timing.duration * 0.7}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Core glow */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize}
        fill={colors.glow}
        opacity={0.3 * intensity}
        style={{ filter: 'blur(4px)' }}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.8};${baseSize * 1.3};${baseSize * 0.8}`}
          dur={`${timing.duration * 0.5}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Bright center */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize * 0.5}
        fill={colors.primary}
        opacity={0.8 * intensity}
      >
        <animate
          attributeName="r"
          values={`${baseSize * 0.4};${baseSize * 0.6};${baseSize * 0.4}`}
          dur={`${timing.duration * 0.3}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.9 * intensity};${0.6 * intensity};${0.9 * intensity}`}
          dur={`${timing.duration * 0.3}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* White hot core */}
      <circle
        cx={sourcePosition.x}
        cy={sourcePosition.y}
        r={baseSize * 0.2}
        fill="#ffffff"
        opacity={0.9 * intensity}
      />
    </g>
  );
});
