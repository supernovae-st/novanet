'use client';

/**
 * PowerConduit - Ownership Arc Effect
 *
 * Semantic: Parent-child hierarchy, structural authority, data containment
 *
 * Visual Concept:
 * - Main arc: Thick glowing cable (4px stroke + 3-layer glow)
 * - Inner core: Bright white-blue "hot wire" (1.5px, 100% opacity)
 * - Energy packets: 3 large orbs (20px) traveling in convoy
 * - Ambient hum: Subtle pulsing glow (0.98-1.02 scale, 3s cycle)
 *
 * Metaphor: High-voltage power line delivering authority
 */

import { memo } from 'react';
import type { EffectTier } from '../EdgeVisibilityManager';
import { getRandomDelay } from '../EdgeUtils';

interface PowerConduitProps {
  edgePath: string;
  colors: { primary: string; secondary: string; glow: string };
  state: 'default' | 'highlighted' | 'selected' | 'muted';
  effectTier: EffectTier;
  edgeId: string;
}

/**
 * PowerConduit Effect - Ownership family signature effect
 *
 * TIER SCALING:
 * - ULTRA: 3 orbs + 3 trail segments + full cable glow
 * - HIGH: 3 orbs + 2 trail segments
 * - MEDIUM: 2 orbs + 1 trail segment
 * - LOW: Fallback to SimplifiedEdgeEffect (not rendered here)
 */
export const PowerConduit = memo(function PowerConduit({
  edgePath,
  colors,
  state,
  effectTier,
  edgeId,
}: PowerConduitProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // TIER-BASED SCALING
  // Slower = more authoritative feel
  const baseDuration = effectTier === 'ultra' ? 10 : effectTier === 'medium' ? 7 : 8;
  const orbSize = effectTier === 'ultra' ? 24 : effectTier === 'medium' ? 16 : 20;
  const orbCount = effectTier === 'medium' ? 2 : 3;
  const trailCount = effectTier === 'ultra' ? 3 : effectTier === 'medium' ? 1 : 2;
  const showCablePulse = effectTier !== 'medium';

  // Orb spacing (evenly distributed across the path)
  const orbSpacing = baseDuration / orbCount;

  // Random delay to desynchronize animations across edges
  const baseDelay = getRandomDelay(edgeId, baseDuration);

  return (
    <g className="effect-power-conduit">
      {/* === CABLE GLOW (ambient pulsing) === */}
      {showCablePulse && (
        <>
          {/* Wide outer glow - creates "power field" effect */}
          <path
            d={edgePath}
            fill="none"
            stroke={colors.glow}
            strokeWidth={isHighlighted ? 18 : 14}
            strokeLinecap="round"
            style={{ filter: 'blur(8px)' }}
          >
            <animate
              attributeName="opacity"
              values="0.2;0.35;0.2"
              dur="4s"
              repeatCount="indefinite"
            />
          </path>
          {/* Middle glow layer */}
          <path
            d={edgePath}
            fill="none"
            stroke={colors.primary}
            strokeWidth={isHighlighted ? 10 : 8}
            strokeLinecap="round"
            style={{ filter: 'blur(4px)' }}
          >
            <animate
              attributeName="opacity"
              values="0.5;0.7;0.5"
              dur="4s"
              repeatCount="indefinite"
            />
          </path>
        </>
      )}

      {/* === CORE CABLE (thick glowing cable) === */}
      <path
        d={edgePath}
        fill="none"
        stroke={colors.primary}
        strokeWidth={isHighlighted ? 5 : 4}
        strokeLinecap="round"
        opacity={0.9}
      />

      {/* === HOT WIRE (bright white-blue center) === */}
      <path
        d={edgePath}
        fill="none"
        stroke="#ffffff"
        strokeWidth={isHighlighted ? 2 : 1.5}
        strokeLinecap="round"
        opacity={1}
        style={{ mixBlendMode: 'screen' }}
      />

      {/* === ENERGY ORB CONVOY === */}
      {Array.from({ length: orbCount }, (_, i) => {
        const orbDelay = baseDelay + i * orbSpacing;
        const orbRadius = orbSize * (1 - i * 0.08); // Slight size variation

        return (
          <g key={`orb-${i}`}>
            {/* Wide glow field around orb */}
            <circle
              r={orbRadius * 2.5}
              fill={colors.glow}
              opacity={0.25}
              style={{ filter: 'blur(12px)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${orbDelay}s`}
                path={edgePath}
                calcMode="spline"
                keySplines="0.4 0 0.2 1"
                keyTimes="0;1"
              />
            </circle>

            {/* Core glow */}
            <circle
              r={orbRadius * 1.5}
              fill={colors.primary}
              opacity={0.5}
              style={{ filter: 'blur(6px)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${orbDelay}s`}
                path={edgePath}
                calcMode="spline"
                keySplines="0.4 0 0.2 1"
                keyTimes="0;1"
              />
              {/* Ambient hum - subtle scale pulse */}
              <animate
                attributeName="r"
                values={`${orbRadius * 1.45};${orbRadius * 1.55};${orbRadius * 1.45}`}
                dur="3s"
                repeatCount="indefinite"
              />
            </circle>

            {/* Solid orb core */}
            <circle
              r={orbRadius}
              fill={colors.primary}
              opacity={0.95}
              style={{
                filter: `drop-shadow(0 0 ${orbRadius}px ${colors.glow})`,
              }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${orbDelay}s`}
                path={edgePath}
                calcMode="spline"
                keySplines="0.4 0 0.2 1"
                keyTimes="0;1"
              />
            </circle>

            {/* White-hot center */}
            <circle r={orbRadius * 0.4} fill="#ffffff" opacity={1}>
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${orbDelay}s`}
                path={edgePath}
                calcMode="spline"
                keySplines="0.4 0 0.2 1"
                keyTimes="0;1"
              />
              {/* Slight flicker */}
              <animate
                attributeName="opacity"
                values="1;0.85;1;0.95;1"
                dur="1.2s"
                repeatCount="indefinite"
              />
            </circle>

            {/* === TRAIL SEGMENTS behind each orb === */}
            {Array.from({ length: trailCount }, (_, t) => {
              // Trail follows slightly behind the orb
              const trailDelay = orbDelay + (t + 1) * 0.2;
              const trailSize = orbRadius * (0.7 - t * 0.15);
              const trailOpacity = 0.6 - t * 0.2;

              return (
                <circle
                  key={`trail-${i}-${t}`}
                  r={trailSize}
                  fill={colors.glow}
                  opacity={trailOpacity}
                  style={{
                    filter: `drop-shadow(0 0 ${trailSize}px ${colors.glow})`,
                  }}
                >
                  <animateMotion
                    dur={`${baseDuration}s`}
                    repeatCount="indefinite"
                    begin={`${trailDelay}s`}
                    path={edgePath}
                    calcMode="spline"
                    keySplines="0.4 0 0.2 1"
                    keyTimes="0;1"
                  />
                </circle>
              );
            })}
          </g>
        );
      })}
    </g>
  );
});

export default PowerConduit;
