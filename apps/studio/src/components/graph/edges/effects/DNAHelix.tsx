'use client';

/**
 * DNAHelix - Localization Arc Effect
 *
 * Semantic: Invariant<->Locale bridge, content adaptation, cultural DNA
 *
 * Visual Concept:
 * - Strand 1: Primary wave (+-25px amplitude, phase 0)
 * - Strand 2: Secondary wave (+-25px amplitude, phase 180)
 * - Base pairs: White connectors (3-4 per arc) appearing at crossings
 * - 3D illusion: Opacity gradient (closer strand brighter)
 * - Gene markers: Small bright dots at strand intersections
 *
 * Metaphor: Content DNA unwinding and adapting to locale
 */

import { memo } from 'react';
import type { EffectTier } from '../EdgeVisibilityManager';
import { getRandomDelay } from '../EdgeUtils';

interface DNAHelixProps {
  edgePath: string;
  colors: { primary: string; secondary: string; glow: string };
  state: 'default' | 'highlighted' | 'selected' | 'muted';
  effectTier: EffectTier;
  edgeId: string;
}

/**
 * DNAHelix Effect - Localization family signature effect
 *
 * TIER SCALING:
 * - ULTRA: 4 atoms/strand + 4 connectors + 3 gene markers
 * - HIGH: 3 atoms/strand + 3 connectors + 2 gene markers
 * - MEDIUM: 2 atoms/strand + 2 connectors + 1 gene marker
 * - LOW: Fallback to SimplifiedEdgeEffect (not rendered here)
 */
export const DNAHelix = memo(function DNAHelix({
  edgePath,
  colors,
  state,
  effectTier,
  edgeId,
}: DNAHelixProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // TIER-BASED SCALING
  const baseDuration = effectTier === 'ultra' ? 12 : effectTier === 'medium' ? 8 : 10;
  const amplitude = effectTier === 'ultra' ? 28 : effectTier === 'medium' ? 20 : 25;
  const atomsPerStrand = effectTier === 'ultra' ? 4 : effectTier === 'medium' ? 2 : 3;
  const connectorCount = effectTier === 'ultra' ? 4 : effectTier === 'medium' ? 2 : 3;
  const geneMarkerCount = effectTier === 'ultra' ? 3 : effectTier === 'medium' ? 1 : 2;
  const orbSize = isHighlighted ? 14 : 12;

  // Oscillation period for the helix rotation feel
  const oscillationPeriod = 2.5;

  // Random delay to desynchronize animations across edges
  const baseDelay = getRandomDelay(edgeId, baseDuration);

  return (
    <g className="effect-dna-helix">
      {/* === STRAND 1 (Primary wave - phase 0) === */}
      {Array.from({ length: atomsPerStrand }, (_, i) => {
        const atomDelay = baseDelay + (i * baseDuration) / atomsPerStrand;
        // 3D illusion: atoms near the "front" are brighter
        const depthPhase = (i / atomsPerStrand) * Math.PI * 2;
        const brightness = 0.7 + 0.3 * Math.cos(depthPhase);

        return (
          <g key={`strand1-${i}`}>
            {/* Glow field */}
            <circle
              r={orbSize * 1.8}
              fill={colors.glow}
              opacity={0.2 * brightness}
              style={{ filter: 'blur(6px)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${atomDelay}s`}
                path={edgePath}
              />
              {/* Primary sine wave - strand 1 phase 0 */}
              <animate
                attributeName="cy"
                values={`${-amplitude};${amplitude};${-amplitude}`}
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
              />
            </circle>

            {/* Core atom */}
            <circle
              r={orbSize}
              fill={colors.primary}
              opacity={0.9 * brightness}
              style={{
                filter: `drop-shadow(0 0 ${orbSize}px ${colors.glow})`,
              }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${atomDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="cy"
                values={`${-amplitude};${amplitude};${-amplitude}`}
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
              />
            </circle>

            {/* White center */}
            <circle r={orbSize * 0.35} fill="#ffffff" opacity={brightness}>
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${atomDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="cy"
                values={`${-amplitude};${amplitude};${-amplitude}`}
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
              />
            </circle>
          </g>
        );
      })}

      {/* === STRAND 2 (Secondary wave - phase 180) === */}
      {Array.from({ length: atomsPerStrand }, (_, i) => {
        const atomDelay = baseDelay + (i * baseDuration) / atomsPerStrand;
        // Opposite phase for crossing effect
        const depthPhase = ((i / atomsPerStrand) * Math.PI * 2) + Math.PI;
        const brightness = 0.5 + 0.3 * Math.cos(depthPhase);

        return (
          <g key={`strand2-${i}`}>
            {/* Glow field - slightly smaller for depth */}
            <circle
              r={orbSize * 1.4}
              fill={colors.secondary}
              opacity={0.15 * brightness}
              style={{ filter: 'blur(5px)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${atomDelay}s`}
                path={edgePath}
              />
              {/* Opposite phase (180 degrees) */}
              <animate
                attributeName="cy"
                values={`${amplitude};${-amplitude};${amplitude}`}
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
              />
            </circle>

            {/* Core atom - smaller for depth illusion */}
            <circle
              r={orbSize * 0.8}
              fill={colors.secondary}
              opacity={0.85 * brightness}
              style={{
                filter: `drop-shadow(0 0 8px ${colors.secondary})`,
              }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${atomDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="cy"
                values={`${amplitude};${-amplitude};${amplitude}`}
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
                begin={`${i * 0.2}s`}
              />
            </circle>
          </g>
        );
      })}

      {/* === BASE PAIR CONNECTORS (white lines at crossings) === */}
      {Array.from({ length: connectorCount }, (_, i) => {
        const connectorDelay = baseDelay + (i * baseDuration) / connectorCount + baseDuration * 0.1;

        return (
          <g key={`connector-${i}`}>
            {/* Connector line - fades in/out at crossings */}
            <line
              x1={0}
              y1={-amplitude * 0.5}
              x2={0}
              y2={amplitude * 0.5}
              stroke="#ffffff"
              strokeWidth={2}
              strokeLinecap="round"
              opacity={0}
              style={{ filter: 'drop-shadow(0 0 3px #ffffff)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${connectorDelay}s`}
                path={edgePath}
              />
              {/* Fade in at crossings (when strands meet) */}
              <animate
                attributeName="opacity"
                values="0;0.8;0;0.8;0"
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
              />
              {/* Shrink at crossings */}
              <animate
                attributeName="y1"
                values={`${-amplitude * 0.6};0;${-amplitude * 0.6}`}
                dur={`${oscillationPeriod / 2}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="y2"
                values={`${amplitude * 0.6};0;${amplitude * 0.6}`}
                dur={`${oscillationPeriod / 2}s`}
                repeatCount="indefinite"
              />
            </line>

            {/* Central dot at crossing point */}
            <circle
              r={4}
              fill="#ffffff"
              opacity={0}
              style={{ filter: 'drop-shadow(0 0 4px #ffffff)' }}
            >
              <animateMotion
                dur={`${baseDuration}s`}
                repeatCount="indefinite"
                begin={`${connectorDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="opacity"
                values="0;1;0;1;0"
                dur={`${oscillationPeriod}s`}
                repeatCount="indefinite"
              />
            </circle>
          </g>
        );
      })}

      {/* === GENE MARKERS (bright dots at intersections) === */}
      {Array.from({ length: geneMarkerCount }, (_, i) => {
        const markerDelay = baseDelay + (i * baseDuration) / geneMarkerCount + baseDuration * 0.25;

        return (
          <circle
            key={`gene-${i}`}
            r={6}
            fill={colors.primary}
            opacity={0}
            style={{
              filter: `drop-shadow(0 0 8px ${colors.glow})`,
            }}
          >
            <animateMotion
              dur={`${baseDuration}s`}
              repeatCount="indefinite"
              begin={`${markerDelay}s`}
              path={edgePath}
            />
            {/* Pulse brighter at strand intersections */}
            <animate
              attributeName="opacity"
              values="0;0;0.9;0;0"
              dur={`${oscillationPeriod}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="r"
              values="4;4;8;4;4"
              dur={`${oscillationPeriod}s`}
              repeatCount="indefinite"
            />
          </circle>
        );
      })}
    </g>
  );
});

export default DNAHelix;
