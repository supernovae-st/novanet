'use client';

/**
 * ScanlinePrimitive - Data scan/transfer visual effect
 *
 * Visual: Animated gradient sweep along the edge
 * Purpose: Shows data transfer/scanning operation
 */

import { memo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';

export const ScanlinePrimitive = memo(function ScanlinePrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
}: EffectPrimitiveProps) {
  const isHighlighted = state === 'highlighted';
  const strokeWidth = isHighlighted ? 4 : 2;

  // Don't render if intensity too low
  if (intensity < 0.3) return null;

  const scanDur = `${timing.duration * 0.8}s`;

  // Generate unique gradient ID
  const gradientId = `scanline-gradient-${pathId}`;

  return (
    <g className="scanline-primitive">
      {/* Define animated gradient */}
      <defs>
        <linearGradient id={gradientId} x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor={colors.primary} stopOpacity={0}>
            <animate
              attributeName="offset"
              values="-0.3;1"
              dur={scanDur}
              repeatCount="indefinite"
            />
          </stop>
          <stop offset="15%" stopColor={colors.primary} stopOpacity={0.8 * intensity}>
            <animate
              attributeName="offset"
              values="-0.15;1.15"
              dur={scanDur}
              repeatCount="indefinite"
            />
          </stop>
          <stop offset="30%" stopColor={colors.secondary} stopOpacity={0.6 * intensity}>
            <animate
              attributeName="offset"
              values="0;1.3"
              dur={scanDur}
              repeatCount="indefinite"
            />
          </stop>
          <stop offset="45%" stopColor={colors.primary} stopOpacity={0}>
            <animate
              attributeName="offset"
              values="0.15;1.45"
              dur={scanDur}
              repeatCount="indefinite"
            />
          </stop>
        </linearGradient>
      </defs>

      {/* Scanline stroke */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={`url(#${gradientId})`}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
      />

      {/* Secondary scanline (offset phase) */}
      {isHighlighted && (
        <>
          <defs>
            <linearGradient id={`${gradientId}-2`} x1="0%" y1="0%" x2="100%" y2="0%">
              <stop offset="0%" stopColor={colors.tertiary} stopOpacity={0}>
                <animate
                  attributeName="offset"
                  values="-0.3;1"
                  dur={scanDur}
                  begin={`${timing.duration * 0.4}s`}
                  repeatCount="indefinite"
                />
              </stop>
              <stop offset="15%" stopColor={colors.tertiary} stopOpacity={0.5 * intensity}>
                <animate
                  attributeName="offset"
                  values="-0.15;1.15"
                  dur={scanDur}
                  begin={`${timing.duration * 0.4}s`}
                  repeatCount="indefinite"
                />
              </stop>
              <stop offset="30%" stopColor={colors.tertiary} stopOpacity={0}>
                <animate
                  attributeName="offset"
                  values="0;1.3"
                  dur={scanDur}
                  begin={`${timing.duration * 0.4}s`}
                  repeatCount="indefinite"
                />
              </stop>
            </linearGradient>
          </defs>
          <use
            href={`#${pathId}`}
            fill="none"
            stroke={`url(#${gradientId}-2)`}
            strokeWidth={strokeWidth * 0.6}
            strokeLinecap="round"
          />
        </>
      )}
    </g>
  );
});
