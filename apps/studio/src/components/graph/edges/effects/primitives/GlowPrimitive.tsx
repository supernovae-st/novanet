'use client';

/**
 * GlowPrimitive - Edge glow/blur effect
 *
 * Visual: Soft glow layer behind edge stroke
 * Purpose: Adds depth and visual prominence
 */

import { memo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { GLOW_CONFIG } from '../../system/constants';

export const GlowPrimitive = memo(function GlowPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
}: EffectPrimitiveProps) {
  const isHighlighted = state === 'highlighted';
  const glowMultiplier = isHighlighted ? GLOW_CONFIG.highlightedMultiplier : 1;
  const baseOpacity = GLOW_CONFIG.minOpacity + (GLOW_CONFIG.maxOpacity - GLOW_CONFIG.minOpacity) * intensity;
  const blurAmount = GLOW_CONFIG.blurRadius * glowMultiplier;

  // Don't render if intensity too low
  if (intensity < 0.1) return null;

  const breatheDur = `${timing.duration * 1.5}s`;

  return (
    <g className="glow-primitive">
      {/* Outer glow layer */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.glow}
        strokeWidth={12 * glowMultiplier}
        strokeLinecap="round"
        opacity={baseOpacity * 0.3}
        style={{ filter: `blur(${blurAmount}px)` }}
      >
        <animate
          attributeName="opacity"
          values={`${baseOpacity * 0.25};${baseOpacity * 0.4};${baseOpacity * 0.25}`}
          dur={breatheDur}
          repeatCount="indefinite"
        />
      </use>

      {/* Middle glow layer */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.secondary}
        strokeWidth={8 * glowMultiplier}
        strokeLinecap="round"
        opacity={baseOpacity * 0.5}
        style={{ filter: `blur(${blurAmount * 0.6}px)` }}
      >
        <animate
          attributeName="opacity"
          values={`${baseOpacity * 0.4};${baseOpacity * 0.6};${baseOpacity * 0.4}`}
          dur={`${timing.duration * 1.2}s`}
          repeatCount="indefinite"
        />
      </use>

      {/* Inner glow layer */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.primary}
        strokeWidth={4 * glowMultiplier}
        strokeLinecap="round"
        opacity={baseOpacity * 0.7}
        style={{ filter: `blur(${blurAmount * 0.3}px)` }}
      />
    </g>
  );
});
