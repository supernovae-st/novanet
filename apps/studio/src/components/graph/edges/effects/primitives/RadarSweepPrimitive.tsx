'use client';

/**
 * RadarSweepPrimitive - Gradient sweep like radar scanning
 *
 * Visual: Bright gradient sweeping along the path like a radar beam
 * Purpose: Mining family - scanning for SEO/GEO intelligence
 *
 * Technique:
 * - Animated linearGradient with moving stops
 * - Gradient: transparent → color → bright → color → transparent
 * - Animate offset attribute from -0.3 to 1.3
 * - Optional pulse emission at sweep completion
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';

interface SweepConfig {
  id: number;
  delay: number;
  duration: number;
}

export interface RadarSweepPrimitiveProps extends EffectPrimitiveProps {
  /** Width of the sweep gradient (0-1 ratio of path) */
  sweepWidth?: number;
  /** Duration of one complete sweep */
  sweepSpeed?: number;
  /** Number of concurrent sweeps */
  sweepCount?: number;
  /** Emit pulse particles at sweep end */
  pulseOnComplete?: boolean;
}

// Default configuration
const DEFAULTS = {
  sweepWidth: 0.25,
  sweepSpeed: 2.5,
  sweepCount: 2,
  strokeWidth: 4,
};

export const RadarSweepPrimitive = memo(function RadarSweepPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  sweepWidth = DEFAULTS.sweepWidth,
  sweepCount = DEFAULTS.sweepCount,
  pulseOnComplete = true,
}: RadarSweepPrimitiveProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  const strokeWidth = isHighlighted ? DEFAULTS.strokeWidth * 1.4 : DEFAULTS.strokeWidth;
  const adjustedStroke = strokeWidth * intensity;

  // Generate sweep configurations with staggered timing
  const sweeps = useMemo((): SweepConfig[] => {
    return Array.from({ length: sweepCount }, (_, i) => ({
      id: i,
      delay: (i / sweepCount) * timing.duration,
      duration: timing.duration,
    }));
  }, [sweepCount, timing.duration]);

  // Don't render if intensity too low
  if (intensity < 0.15) return null;

  // Unique IDs for gradients and filters
  const gradientId = `radar-gradient-${pathId}`;
  const glowFilterId = `radar-glow-${pathId}`;

  // Calculate gradient stop positions based on sweep width
  const w = sweepWidth;
  const stops = {
    start: -w,
    rampUp: -w / 2,
    peak: 0,
    rampDown: w / 2,
    end: w,
  };

  return (
    <g className="radar-sweep-primitive">
      <defs>
        {/* Glow filter for the sweep */}
        <filter id={glowFilterId} x="-50%" y="-50%" width="200%" height="200%">
          <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>

        {/* Animated gradient for each sweep */}
        {sweeps.map((sweep) => (
          <linearGradient
            key={`gradient-${sweep.id}`}
            id={`${gradientId}-${sweep.id}`}
            gradientUnits="objectBoundingBox"
            x1="0%"
            y1="0%"
            x2="100%"
            y2="0%"
          >
            {/* Leading edge - transparent */}
            <stop offset="0%" stopColor={colors.primary} stopOpacity="0">
              <animate
                attributeName="offset"
                values={`${stops.start};${1 + stops.start}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>

            {/* Ramp up */}
            <stop offset="0%" stopColor={colors.primary} stopOpacity={0.4 * intensity}>
              <animate
                attributeName="offset"
                values={`${stops.rampUp};${1 + stops.rampUp}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>

            {/* Peak - bright white */}
            <stop offset="0%" stopColor="#ffffff" stopOpacity={1.0 * intensity}>
              <animate
                attributeName="offset"
                values={`${stops.peak};${1 + stops.peak}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>

            {/* Ramp down */}
            <stop offset="0%" stopColor={colors.primary} stopOpacity={0.6 * intensity}>
              <animate
                attributeName="offset"
                values={`${stops.rampDown};${1 + stops.rampDown}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>

            {/* Trailing edge - fade with glow color */}
            <stop offset="0%" stopColor={colors.glow} stopOpacity={0.3 * intensity}>
              <animate
                attributeName="offset"
                values={`${stops.end};${1 + stops.end}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>

            {/* Final transparent */}
            <stop offset="0%" stopColor={colors.glow} stopOpacity="0">
              <animate
                attributeName="offset"
                values={`${stops.end + 0.05};${1 + stops.end + 0.05}`}
                dur={`${sweep.duration}s`}
                begin={`${sweep.delay}s`}
                repeatCount="indefinite"
              />
            </stop>
          </linearGradient>
        ))}
      </defs>

      {/* Background glow layer */}
      <use
        href={`#${pathId}`}
        fill="none"
        stroke={colors.glow}
        strokeWidth={adjustedStroke * 2.5}
        strokeLinecap="round"
        opacity={0.2 * intensity}
        style={{ filter: 'blur(6px)' }}
      />

      {/* Sweep layers - one per sweep config */}
      {sweeps.map((sweep) => (
        <use
          key={`sweep-${sweep.id}`}
          href={`#${pathId}`}
          fill="none"
          stroke={`url(#${gradientId}-${sweep.id})`}
          strokeWidth={adjustedStroke}
          strokeLinecap="round"
          filter={`url(#${glowFilterId})`}
        />
      ))}

      {/* Pulse particles at target when sweep completes */}
      {pulseOnComplete && sweeps.map((sweep) => (
        <g key={`pulse-${sweep.id}`}>
          {/* Central pulse */}
          <circle
            r={6 * intensity}
            fill={colors.primary}
            opacity={0}
          >
            <animateMotion
              dur={`${sweep.duration}s`}
              repeatCount="indefinite"
              begin={`${sweep.delay}s`}
              keyPoints="1;1"
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values="0;0;0;1;0"
              keyTimes="0;0.85;0.9;0.95;1"
              dur={`${sweep.duration}s`}
              begin={`${sweep.delay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="r"
              values={`${4 * intensity};${10 * intensity}`}
              keyTimes="0;1"
              dur="0.15s"
              begin={`${sweep.delay + sweep.duration * 0.9}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Outer ring pulse */}
          <circle
            r={8 * intensity}
            fill="none"
            stroke={colors.glow}
            strokeWidth={2 * intensity}
            opacity={0}
          >
            <animateMotion
              dur={`${sweep.duration}s`}
              repeatCount="indefinite"
              begin={`${sweep.delay}s`}
              keyPoints="1;1"
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values="0;0;0;0.8;0"
              keyTimes="0;0.85;0.9;0.95;1"
              dur={`${sweep.duration}s`}
              begin={`${sweep.delay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="r"
              values={`${6 * intensity};${18 * intensity}`}
              keyTimes="0;1"
              dur="0.2s"
              begin={`${sweep.delay + sweep.duration * 0.9}s`}
              repeatCount="indefinite"
            />
          </circle>
        </g>
      ))}
    </g>
  );
});
