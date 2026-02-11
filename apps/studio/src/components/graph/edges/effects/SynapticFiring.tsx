'use client';

/**
 * SynapticFiring - Semantic Arc Effect
 *
 * Semantic: Meaning connections, entity relationships, concept links
 *
 * Visual Concept:
 * - Baseline: Dim, dormant arc (30% opacity, subtle glow)
 * - Firing pulse: Bright wave travels source->target (0.3s)
 * - Signal propagation: Leading edge white-hot, trailing decay
 * - Spark shower: 3-5 tiny particles ejected at target
 * - Residual glow: Arc stays brighter for 1s after firing
 *
 * Metaphor: Neurons firing and propagating signals
 */

import { memo, useMemo } from 'react';
import type { EffectTier } from '../EdgeVisibilityManager';

interface SynapticFiringProps {
  edgePath: string;
  colors: { primary: string; secondary: string; glow: string };
  state: 'default' | 'highlighted' | 'selected' | 'muted';
  effectTier: EffectTier;
}

/**
 * SynapticFiring Effect - Semantic family signature effect
 *
 * TIER SCALING:
 * - ULTRA: 3 firing pulses + 5 sparks + residual glow
 * - HIGH: 2 firing pulses + 4 sparks + residual glow
 * - MEDIUM: 1 firing pulse + 2 sparks
 * - LOW: Fallback to SimplifiedEdgeEffect (not rendered here)
 */
export const SynapticFiring = memo(function SynapticFiring({
  edgePath,
  colors,
  state,
  effectTier,
}: SynapticFiringProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // TIER-BASED SCALING
  const pulseCount = effectTier === 'ultra' ? 3 : effectTier === 'medium' ? 1 : 2;
  const sparkCount = effectTier === 'ultra' ? 5 : effectTier === 'medium' ? 2 : 4;
  const showResidualGlow = effectTier !== 'medium';

  // Firing interval - random-ish feel via prime numbers
  // Each pulse fires at different intervals to create organic timing
  const firingIntervals = useMemo(() => {
    return Array.from({ length: pulseCount }, (_, i) => {
      // Staggered intervals: 2.7s, 3.3s, 4.1s (prime-ish for organic feel)
      return 2.7 + i * 0.7 + (i % 2) * 0.4;
    });
  }, [pulseCount]);

  // Total cycle time (longest interval + pulse travel + residual)
  const cycleDuration = Math.max(...firingIntervals) + 1.5;

  // Pulse travel time (fast signal)
  const pulseTravelTime = 0.4;

  return (
    <g className="effect-synaptic-firing">
      {/* === DORMANT BASELINE (dim arc) === */}
      <path
        d={edgePath}
        fill="none"
        stroke={colors.primary}
        strokeWidth={isHighlighted ? 4 : 3}
        strokeLinecap="round"
        opacity={0.25}
        style={{ filter: 'blur(1px)' }}
      />

      {/* === RESIDUAL GLOW (brightens after each firing) === */}
      {showResidualGlow && (
        <path
          d={edgePath}
          fill="none"
          stroke={colors.glow}
          strokeWidth={isHighlighted ? 10 : 8}
          strokeLinecap="round"
          style={{ filter: 'blur(6px)' }}
        >
          {/* Glow pulses after each firing */}
          <animate
            attributeName="opacity"
            values="0.1;0.1;0.4;0.1"
            dur={`${cycleDuration}s`}
            repeatCount="indefinite"
            keyTimes="0;0.6;0.7;1"
          />
        </path>
      )}

      {/* === FIRING PULSES === */}
      {Array.from({ length: pulseCount }, (_, pulseIndex) => {
        const firingDelay = firingIntervals[pulseIndex];

        return (
          <g key={`pulse-${pulseIndex}`}>
            {/* Leading edge - white hot */}
            <circle
              r={isHighlighted ? 10 : 8}
              fill="#ffffff"
              opacity={0}
              style={{
                filter: `drop-shadow(0 0 12px #ffffff) drop-shadow(0 0 6px ${colors.glow})`,
              }}
            >
              <animateMotion
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                path={edgePath}
                fill="freeze"
              />
              <animate
                attributeName="opacity"
                values="0;1;0"
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                keyTimes="0;0.2;1"
              />
              <animate
                attributeName="r"
                values="4;10;6"
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
              />
            </circle>

            {/* Signal propagation - bright wave with trailing decay */}
            <circle
              r={isHighlighted ? 18 : 14}
              fill={colors.primary}
              opacity={0}
              style={{ filter: 'blur(4px)' }}
            >
              <animateMotion
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                path={edgePath}
                fill="freeze"
              />
              <animate
                attributeName="opacity"
                values="0;0.7;0.4;0"
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                keyTimes="0;0.1;0.5;1"
              />
            </circle>

            {/* Core pulse */}
            <circle
              r={isHighlighted ? 7 : 5}
              fill={colors.primary}
              opacity={0}
              style={{
                filter: `drop-shadow(0 0 8px ${colors.glow})`,
              }}
            >
              <animateMotion
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                path={edgePath}
                fill="freeze"
              />
              <animate
                attributeName="opacity"
                values="0;0.95;0.6;0"
                dur={`${pulseTravelTime}s`}
                repeatCount="indefinite"
                begin={`${firingDelay}s`}
                keyTimes="0;0.15;0.6;1"
              />
            </circle>

            {/* Trailing decay segments */}
            {[1, 2, 3].map((trailIndex) => {
              const trailDelay = firingDelay + trailIndex * 0.05;
              const trailOpacity = 0.6 - trailIndex * 0.15;
              const trailSize = (isHighlighted ? 5 : 4) - trailIndex * 0.8;

              return (
                <circle
                  key={`trail-${pulseIndex}-${trailIndex}`}
                  r={trailSize}
                  fill={colors.glow}
                  opacity={0}
                  style={{ filter: `blur(${trailIndex}px)` }}
                >
                  <animateMotion
                    dur={`${pulseTravelTime}s`}
                    repeatCount="indefinite"
                    begin={`${trailDelay}s`}
                    path={edgePath}
                    fill="freeze"
                  />
                  <animate
                    attributeName="opacity"
                    values={`0;${trailOpacity};0`}
                    dur={`${pulseTravelTime}s`}
                    repeatCount="indefinite"
                    begin={`${trailDelay}s`}
                    keyTimes="0;0.3;1"
                  />
                </circle>
              );
            })}
          </g>
        );
      })}

      {/* === SPARK SHOWER (at target on arrival) === */}
      {Array.from({ length: sparkCount }, (_, sparkIndex) => {
        // Sparks fire when pulse arrives (firingDelay + pulseTravelTime)
        const sparkDelay = firingIntervals[sparkIndex % pulseCount] + pulseTravelTime - 0.05;
        // Random-ish scatter directions using index
        const scatterX = ((sparkIndex * 37) % 30) - 15;
        const scatterY = ((sparkIndex * 23) % 30) - 15;
        const sparkSize = 3 + (sparkIndex % 3);

        return (
          <circle
            key={`spark-${sparkIndex}`}
            r={sparkSize}
            fill={colors.primary}
            opacity={0}
            style={{
              filter: `drop-shadow(0 0 4px ${colors.glow})`,
            }}
          >
            {/* Position at end of path (target) */}
            <animateMotion
              dur={`${pulseTravelTime}s`}
              begin={`${sparkDelay - pulseTravelTime * 0.9}s`}
              repeatCount="indefinite"
              path={edgePath}
              keyPoints="0.95;1"
              keyTimes="0;1"
            />
            {/* Ejection animation */}
            <animate
              attributeName="cx"
              values={`0;${scatterX}`}
              dur="0.5s"
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
            />
            <animate
              attributeName="cy"
              values={`0;${scatterY + 10}`}
              dur="0.5s"
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
            />
            <animate
              attributeName="opacity"
              values="0;0.9;0"
              dur="0.5s"
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
              keyTimes="0;0.2;1"
            />
            <animate
              attributeName="r"
              values={`${sparkSize};${sparkSize * 0.5}`}
              dur="0.5s"
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
            />
          </circle>
        );
      })}
    </g>
  );
});

export default SynapticFiring;
