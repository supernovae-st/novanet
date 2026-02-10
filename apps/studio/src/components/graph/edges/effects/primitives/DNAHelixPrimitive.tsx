'use client';

/**
 * DNAHelixPrimitive - Double helix spiral animation
 *
 * Visual: Two intertwined strands with connecting "rungs"
 * Purpose: Localization family - content DNA adapts per locale
 *
 * Technique:
 * - Two parallel particle streams with opposite phase oscillation
 * - Y offset via animateTransform type="translate"
 * - Opacity based on Z-depth for 3D illusion
 * - Connecting rungs between strands at intervals
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { EASING_PRESETS } from '../../system/constants';

interface NucleotideConfig {
  id: number;
  strand: 1 | 2;
  delay: number;
  duration: number;
  phaseOffset: number;
  size: number;
}

interface RungConfig {
  id: number;
  delay: number;
  duration: number;
}

export interface DNAHelixPrimitiveProps extends EffectPrimitiveProps {
  /** Number of nucleotides per strand */
  nucleotidesPerStrand?: number;
  /** Oscillation amplitude (height in px) */
  amplitude?: number;
  /** Number of waves per animation cycle */
  frequency?: number;
  /** Rotation speed for 3D effect */
  rotationSpeed?: number;
  /** Show connecting rungs between strands */
  showRungs?: boolean;
}

// Default configuration for DNA helix effect
const DEFAULTS = {
  nucleotidesPerStrand: 6,
  amplitude: 18,
  frequency: 3,
  rotationSpeed: 2,
  nucleotideSize: 5,
  rungInterval: 2, // Show rung every N nucleotides
};

export const DNAHelixPrimitive = memo(function DNAHelixPrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  nucleotidesPerStrand = DEFAULTS.nucleotidesPerStrand,
  amplitude = DEFAULTS.amplitude,
  frequency = DEFAULTS.frequency,
  showRungs = true,
}: DNAHelixPrimitiveProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  const baseSize = isHighlighted ? DEFAULTS.nucleotideSize * 1.3 : DEFAULTS.nucleotideSize;
  const adjustedSize = baseSize * intensity;
  const adjustedAmplitude = amplitude * intensity;

  // Generate nucleotide configurations for both strands
  const nucleotides = useMemo((): NucleotideConfig[] => {
    const configs: NucleotideConfig[] = [];

    for (let strand = 1; strand <= 2; strand++) {
      for (let i = 0; i < nucleotidesPerStrand; i++) {
        const stagger = (i / nucleotidesPerStrand) * timing.duration;
        // Strand 2 is 180° out of phase
        const phaseOffset = strand === 1 ? 0 : Math.PI;

        configs.push({
          id: configs.length,
          strand: strand as 1 | 2,
          delay: stagger,
          duration: timing.duration,
          phaseOffset,
          size: adjustedSize,
        });
      }
    }

    return configs;
  }, [nucleotidesPerStrand, timing.duration, adjustedSize]);

  // Generate rung configurations (connect strands at intervals)
  const rungs = useMemo((): RungConfig[] => {
    if (!showRungs) return [];

    const rungConfigs: RungConfig[] = [];
    const rungCount = Math.floor(nucleotidesPerStrand / DEFAULTS.rungInterval);

    for (let i = 0; i < rungCount; i++) {
      const stagger = (i / rungCount) * timing.duration;
      rungConfigs.push({
        id: i,
        delay: stagger,
        duration: timing.duration,
      });
    }

    return rungConfigs;
  }, [showRungs, nucleotidesPerStrand, timing.duration]);

  // Don't render if intensity too low
  if (intensity < 0.15) return null;

  const spline = EASING_PRESETS.easeInOut;

  // Create oscillation keyframes for Y translation
  // This creates the wave effect perpendicular to the path
  const createOscillationValues = (phaseOffset: number): string => {
    const steps = 20;
    const values: string[] = [];

    for (let i = 0; i <= steps; i++) {
      const t = i / steps;
      const y = adjustedAmplitude * Math.sin(2 * Math.PI * frequency * t + phaseOffset);
      values.push(`0 ${y.toFixed(1)}`);
    }

    return values.join(';');
  };

  // Create opacity values for 3D depth effect
  const createDepthOpacity = (phaseOffset: number): string => {
    const steps = 20;
    const values: string[] = [];

    for (let i = 0; i <= steps; i++) {
      const t = i / steps;
      const z = Math.cos(2 * Math.PI * frequency * t + phaseOffset);
      // Map z from [-1, 1] to [0.3, 1.0] for opacity
      const opacity = 0.4 + (z + 1) * 0.3;
      values.push((opacity * intensity).toFixed(2));
    }

    return values.join(';');
  };

  const createKeyTimes = (): string => {
    const steps = 20;
    const times: string[] = [];
    for (let i = 0; i <= steps; i++) {
      times.push((i / steps).toFixed(2));
    }
    return times.join(';');
  };

  const keyTimes = createKeyTimes();

  return (
    <g className="dna-helix-primitive">
      {/* Strand 1 nucleotides */}
      {nucleotides
        .filter((n) => n.strand === 1)
        .map((nucleotide) => (
          <g key={`nucleotide-${nucleotide.id}`}>
            {/* Glow layer */}
            <circle
              r={nucleotide.size * 2}
              fill={colors.glow}
              opacity={0}
              style={{ filter: 'blur(4px)' }}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>

            {/* Core nucleotide */}
            <circle
              r={nucleotide.size}
              fill={colors.primary}
              opacity={0}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>

            {/* Bright center */}
            <circle
              r={nucleotide.size * 0.4}
              fill="#ffffff"
              opacity={0}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>
          </g>
        ))}

      {/* Strand 2 nucleotides (opposite phase) */}
      {nucleotides
        .filter((n) => n.strand === 2)
        .map((nucleotide) => (
          <g key={`nucleotide-${nucleotide.id}`}>
            {/* Glow layer */}
            <circle
              r={nucleotide.size * 2}
              fill={colors.glow}
              opacity={0}
              style={{ filter: 'blur(4px)' }}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>

            {/* Core nucleotide (secondary color for strand 2) */}
            <circle
              r={nucleotide.size}
              fill={colors.secondary}
              opacity={0}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>

            {/* Bright center */}
            <circle
              r={nucleotide.size * 0.4}
              fill="#ffffff"
              opacity={0}
            >
              <animateMotion
                dur={`${nucleotide.duration}s`}
                repeatCount="indefinite"
                begin={`${nucleotide.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animateTransform
                attributeName="transform"
                type="translate"
                values={createOscillationValues(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
              <animate
                attributeName="opacity"
                values={createDepthOpacity(nucleotide.phaseOffset)}
                keyTimes={keyTimes}
                dur={`${nucleotide.duration}s`}
                begin={`${nucleotide.delay}s`}
                repeatCount="indefinite"
              />
            </circle>
          </g>
        ))}

      {/* Connecting rungs between strands */}
      {rungs.map((rung) => (
        <line
          key={`rung-${rung.id}`}
          x1={0}
          y1={-adjustedAmplitude * 0.8}
          x2={0}
          y2={adjustedAmplitude * 0.8}
          stroke={colors.tertiary}
          strokeWidth={1.5 * intensity}
          strokeLinecap="round"
          opacity={0}
        >
          <animateMotion
            dur={`${rung.duration}s`}
            repeatCount="indefinite"
            begin={`${rung.delay}s`}
            calcMode="spline"
            keySplines={spline}
            keyTimes="0;1"
          >
            <mpath href={`#${pathId}`} />
          </animateMotion>
          <animate
            attributeName="opacity"
            values={`0;${0.6 * intensity};${0.6 * intensity};0`}
            keyTimes="0;0.1;0.85;1"
            dur={`${rung.duration}s`}
            begin={`${rung.delay}s`}
            repeatCount="indefinite"
          />
        </line>
      ))}
    </g>
  );
});
