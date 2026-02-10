'use client';

/**
 * EnergyPulsePrimitive - Intense energy packets with glow trail
 *
 * Visual: Bright power pulses flowing from source to target
 * Purpose: Ownership family - power/authority flowing to children
 *
 * Layers:
 * 1. Outer glow (blur 12px, opacity 0.4)
 * 2. Middle glow (blur 6px, opacity 0.6)
 * 3. Core pulse (solid, opacity 0.9)
 * 4. White hot center (2px, opacity 1.0)
 * 5. Trail segments (decreasing size/opacity)
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { EASING_PRESETS } from '../../system/constants';

interface PulseConfig {
  id: number;
  delay: number;
  duration: number;
  size: number;
  isLeader: boolean;
}

interface TrailSegment {
  id: number;
  offsetRatio: number;
  size: number;
  opacity: number;
}

export interface EnergyPulsePrimitiveProps extends EffectPrimitiveProps {
  /** Number of energy pulses */
  pulseCount?: number;
  /** Base pulse size (radius in px) */
  pulseSize?: number;
  /** Glow intensity multiplier (0.8-1.0 for very bright) */
  glowIntensity?: number;
  /** Number of trail segments behind each pulse */
  trailLength?: number;
}

// Default configuration for energy pulse effect
const DEFAULTS = {
  pulseCount: 4,
  pulseSize: 10,
  glowIntensity: 0.9,
  trailLength: 5,
  duration: 1.5, // Fast speed
  leaderMultiplier: 1.4,
};

export const EnergyPulsePrimitive = memo(function EnergyPulsePrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  pulseCount = DEFAULTS.pulseCount,
  pulseSize = DEFAULTS.pulseSize,
  glowIntensity = DEFAULTS.glowIntensity,
  trailLength = DEFAULTS.trailLength,
}: EnergyPulsePrimitiveProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  const baseSize = isHighlighted ? pulseSize * 1.3 : pulseSize;
  const adjustedSize = baseSize * intensity;
  const adjustedGlow = glowIntensity * intensity;

  // Generate pulse configurations
  const pulses = useMemo((): PulseConfig[] => {
    return Array.from({ length: pulseCount }, (_, i) => {
      const stagger = (i / pulseCount) * timing.duration;
      const durationVariation = 1 + (i % 3 === 0 ? 0.08 : i % 3 === 1 ? -0.04 : 0.02);

      return {
        id: i,
        delay: stagger,
        duration: timing.duration * durationVariation,
        size: i === 0 ? adjustedSize * DEFAULTS.leaderMultiplier : adjustedSize,
        isLeader: i === 0,
      };
    });
  }, [pulseCount, timing.duration, adjustedSize]);

  // Generate trail segments for each pulse
  const trailSegments = useMemo((): TrailSegment[] => {
    return Array.from({ length: trailLength }, (_, i) => ({
      id: i,
      offsetRatio: (i + 1) * 0.03, // Time offset behind main pulse
      size: adjustedSize * (1 - (i + 1) * 0.15), // Decreasing size
      opacity: adjustedGlow * (1 - (i + 1) * 0.18), // Decreasing opacity
    }));
  }, [trailLength, adjustedSize, adjustedGlow]);

  // Don't render if intensity too low
  if (intensity < 0.15) return null;

  const spline = EASING_PRESETS.energetic;
  const filterId = `energy-glow-${pathId}`;

  return (
    <g className="energy-pulse-primitive">
      {/* Simple visible animated pulses that we KNOW work */}
      {pulses.map((pulse) => (
        <circle
          key={`simple-pulse-${pulse.id}`}
          r={pulse.size}
          fill={colors.primary}
          opacity={0.9}
          style={{ filter: `drop-shadow(0 0 ${pulse.size}px ${colors.glow})` }}
        >
          <animateMotion
            dur={`${pulse.duration}s`}
            repeatCount="indefinite"
            begin={`${pulse.delay}s`}
          >
            <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
          </animateMotion>
        </circle>
      ))}

      {/* SVG Filter for intense glow */}
      <defs>
        <filter id={filterId} x="-100%" y="-100%" width="300%" height="300%">
          <feGaussianBlur in="SourceGraphic" stdDeviation="4" result="blur1" />
          <feGaussianBlur in="SourceGraphic" stdDeviation="8" result="blur2" />
          <feMerge>
            <feMergeNode in="blur2" />
            <feMergeNode in="blur1" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      {pulses.map((pulse) => (
        <g key={`pulse-${pulse.id}`}>
          {/* Trail segments (rendered first, behind pulse) */}
          {trailSegments.map((segment) => (
            <circle
              key={`trail-${pulse.id}-${segment.id}`}
              r={segment.size}
              fill={colors.secondary}
              opacity={0}
            >
              <animateMotion
                dur={`${pulse.duration}s`}
                repeatCount="indefinite"
                begin={`${pulse.delay + segment.offsetRatio}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
              </animateMotion>
              <animate
                attributeName="opacity"
                values={`0;${segment.opacity};${segment.opacity};0`}
                keyTimes="0;0.08;0.88;1"
                dur={`${pulse.duration}s`}
                begin={`${pulse.delay + segment.offsetRatio}s`}
                repeatCount="indefinite"
              />
            </circle>
          ))}

          {/* Layer 1: Outer glow (blur 12px) */}
          <circle
            r={pulse.size * 2.5}
            fill={colors.glow}
            opacity={0}
            style={{ filter: 'blur(12px)' }}
          >
            <animateMotion
              dur={`${pulse.duration}s`}
              repeatCount="indefinite"
              begin={`${pulse.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.4 * adjustedGlow};${0.4 * adjustedGlow};0`}
              keyTimes="0;0.08;0.85;1"
              dur={`${pulse.duration}s`}
              begin={`${pulse.delay}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Layer 2: Middle glow (blur 6px) */}
          <circle
            r={pulse.size * 1.8}
            fill={colors.primary}
            opacity={0}
            style={{ filter: 'blur(6px)' }}
          >
            <animateMotion
              dur={`${pulse.duration}s`}
              repeatCount="indefinite"
              begin={`${pulse.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.6 * adjustedGlow};${0.6 * adjustedGlow};0`}
              keyTimes="0;0.06;0.88;1"
              dur={`${pulse.duration}s`}
              begin={`${pulse.delay}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Layer 3: Core pulse (solid) */}
          <circle
            r={pulse.size}
            fill={pulse.isLeader ? colors.primary : colors.secondary}
            opacity={0}
            filter={`url(#${filterId})`}
          >
            <animateMotion
              dur={`${pulse.duration}s`}
              repeatCount="indefinite"
              begin={`${pulse.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.9 * intensity};${0.9 * intensity};0`}
              keyTimes="0;0.04;0.92;1"
              dur={`${pulse.duration}s`}
              begin={`${pulse.delay}s`}
              repeatCount="indefinite"
            />
            {/* Pulsing size animation for leader */}
            {pulse.isLeader && (
              <animate
                attributeName="r"
                values={`${pulse.size};${pulse.size * 1.2};${pulse.size}`}
                keyTimes="0;0.5;1"
                dur="0.4s"
                begin={`${pulse.delay}s`}
                repeatCount="indefinite"
              />
            )}
          </circle>

          {/* Layer 4: White hot center */}
          <circle
            r={pulse.size * 0.35}
            fill="#ffffff"
            opacity={0}
          >
            <animateMotion
              dur={`${pulse.duration}s`}
              repeatCount="indefinite"
              begin={`${pulse.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath xlinkHref={`#${pathId}`} href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${1.0 * intensity};${1.0 * intensity};0`}
              keyTimes="0;0.04;0.92;1"
              dur={`${pulse.duration}s`}
              begin={`${pulse.delay}s`}
              repeatCount="indefinite"
            />
          </circle>
        </g>
      ))}
    </g>
  );
});
