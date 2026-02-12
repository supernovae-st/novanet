'use client';

/**
 * MatrixCodeRain - Generation Arc Effect
 *
 * Semantic: LLM generation pipeline, AI processing, prompt->output flow
 *
 * Visual Concept:
 * - Data stream: Characters ("01" or "▓▒░") falling along arc
 * - Variable speed: Some fast, some slow (parallax depth)
 * - Processing burst: Periodic bright flash at midpoint
 * - Output glow: Target pulses when data "arrives"
 * - Scanline: Horizontal line sweeping across arc
 *
 * Metaphor: Data being processed through AI neural network
 */

import { memo, useMemo } from 'react';
import type { EffectTier } from '../EdgeVisibilityManager';
import { getRandomDelay } from '../EdgeUtils';

interface MatrixCodeRainProps {
  edgePath: string;
  colors: { primary: string; secondary: string; glow: string };
  state: 'default' | 'highlighted' | 'selected' | 'muted';
  effectTier: EffectTier;
  edgeId: string;
}

// Character sets for the matrix effect
const CHAR_SETS = {
  binary: ['0', '1'],
  blocks: ['▓', '▒', '░'],
  mixed: ['0', '1', '▓', '▒'],
};

/**
 * MatrixCodeRain Effect - Generation family signature effect
 *
 * TIER SCALING:
 * - ULTRA: 10 characters + 3 processing bursts + scanline + output pulse
 * - HIGH: 8 characters + 2 processing bursts + output pulse
 * - MEDIUM: 5 characters + 1 processing burst
 * - LOW: Fallback to SimplifiedEdgeEffect (not rendered here)
 */
export const MatrixCodeRain = memo(function MatrixCodeRain({
  edgePath,
  colors,
  state,
  effectTier,
  edgeId,
}: MatrixCodeRainProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // TIER-BASED SCALING
  const characterCount = effectTier === 'ultra' ? 10 : effectTier === 'medium' ? 5 : 8;
  const burstCount = effectTier === 'ultra' ? 3 : effectTier === 'medium' ? 1 : 2;
  const showScanline = effectTier === 'ultra';
  const showOutputPulse = effectTier !== 'medium';

  // Random delay to desynchronize animations across edges
  const baseDelay = getRandomDelay(edgeId, 3);

  // Character generation with varied speeds
  const characters = useMemo(() => {
    const chars = CHAR_SETS.mixed;
    return Array.from({ length: characterCount }, (_, i) => ({
      char: chars[i % chars.length],
      // Varied fall speeds for parallax depth (1.2s to 2.8s)
      speed: 1.2 + (i * 0.17) + ((i * 7) % 5) * 0.2,
      // Staggered start times with baseDelay
      delay: baseDelay + (i * 0.3) + ((i * 13) % 7) * 0.1,
      // Size variation
      size: isHighlighted ? 12 + (i % 3) * 2 : 10 + (i % 3) * 2,
      // Brightness variation (parallax depth)
      brightness: 0.5 + (i % 3) * 0.2,
    }));
  }, [characterCount, isHighlighted, baseDelay]);

  // Processing burst timing (every 2s with variation)
  const burstIntervals = useMemo(() => {
    return Array.from({ length: burstCount }, (_, i) => baseDelay + 1.8 + i * 2.3);
  }, [burstCount, baseDelay]);

  // Scanline sweep timing
  const scanlineDuration = 4;

  return (
    <g className="effect-matrix-code-rain">
      {/* === SCANLINE SWEEP === */}
      {showScanline && (
        <g>
          {/* Horizontal scanline sweeping along path */}
          <rect
            x={-20}
            y={-2}
            width={40}
            height={4}
            fill={colors.primary}
            opacity={0}
            style={{ filter: `blur(2px) drop-shadow(0 0 8px ${colors.glow})` }}
          >
            <animateMotion
              dur={`${scanlineDuration}s`}
              repeatCount="indefinite"
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0;0.7;0.7;0"
              dur={`${scanlineDuration}s`}
              repeatCount="indefinite"
              keyTimes="0;0.1;0.9;1"
            />
          </rect>
          {/* Scanline glow trail */}
          <rect
            x={-30}
            y={-6}
            width={60}
            height={12}
            fill={colors.glow}
            opacity={0}
            style={{ filter: 'blur(8px)' }}
          >
            <animateMotion
              dur={`${scanlineDuration}s`}
              repeatCount="indefinite"
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0;0.3;0.3;0"
              dur={`${scanlineDuration}s`}
              repeatCount="indefinite"
              keyTimes="0;0.1;0.9;1"
            />
          </rect>
        </g>
      )}

      {/* === FALLING CHARACTERS (data stream) === */}
      {characters.map((char, i) => (
        <g key={`char-${i}`}>
          {/* Character glow */}
          <text
            fontSize={char.size + 4}
            fontFamily="monospace"
            fontWeight="bold"
            fill={colors.glow}
            textAnchor="middle"
            dominantBaseline="middle"
            opacity={0}
            style={{ filter: 'blur(4px)' }}
          >
            <animateMotion
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values={`0;${char.brightness * 0.5};${char.brightness * 0.5};0`}
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              keyTimes="0;0.1;0.85;1"
            />
            {char.char}
          </text>

          {/* Main character */}
          <text
            fontSize={char.size}
            fontFamily="monospace"
            fontWeight="bold"
            fill={colors.primary}
            textAnchor="middle"
            dominantBaseline="middle"
            opacity={0}
            style={{
              filter: `drop-shadow(0 0 3px ${colors.glow})`,
            }}
          >
            <animateMotion
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values={`0;${char.brightness};${char.brightness};0`}
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              keyTimes="0;0.05;0.9;1"
            />
            {/* Character flicker (random character change illusion) */}
            <animate
              attributeName="fill"
              values={`${colors.primary};#ffffff;${colors.primary}`}
              dur="0.15s"
              repeatCount="indefinite"
              begin={`${char.delay + char.speed * 0.5}s`}
            />
            {char.char}
          </text>

          {/* Bright leading edge */}
          <text
            fontSize={char.size * 0.8}
            fontFamily="monospace"
            fontWeight="bold"
            fill="#ffffff"
            textAnchor="middle"
            dominantBaseline="middle"
            opacity={0}
          >
            <animateMotion
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0;1;0.3;0"
              dur={`${char.speed}s`}
              repeatCount="indefinite"
              begin={`${char.delay}s`}
              keyTimes="0;0.02;0.15;1"
            />
            {char.char}
          </text>
        </g>
      ))}

      {/* === PROCESSING BURSTS (at midpoint) === */}
      {burstIntervals.map((interval, i) => (
        <g key={`burst-${i}`}>
          {/* Bright flash */}
          <circle
            r={isHighlighted ? 25 : 20}
            fill="#ffffff"
            opacity={0}
            style={{ filter: 'blur(8px)' }}
          >
            <animateMotion
              dur="0.01s"
              repeatCount="indefinite"
              begin={`${interval}s`}
              path={edgePath}
              keyPoints="0.5;0.5"
              keyTimes="0;1"
            />
            <animate
              attributeName="opacity"
              values="0;1;0"
              dur="0.3s"
              repeatCount="indefinite"
              begin={`${interval}s`}
              keyTimes="0;0.1;1"
            />
            <animate
              attributeName="r"
              values="10;30;10"
              dur="0.3s"
              repeatCount="indefinite"
              begin={`${interval}s`}
            />
          </circle>

          {/* Color burst */}
          <circle
            r={isHighlighted ? 18 : 14}
            fill={colors.primary}
            opacity={0}
            style={{ filter: 'blur(4px)' }}
          >
            <animateMotion
              dur="0.01s"
              repeatCount="indefinite"
              begin={`${interval}s`}
              path={edgePath}
              keyPoints="0.5;0.5"
              keyTimes="0;1"
            />
            <animate
              attributeName="opacity"
              values="0;0.8;0"
              dur="0.4s"
              repeatCount="indefinite"
              begin={`${interval}s`}
              keyTimes="0;0.15;1"
            />
          </circle>
        </g>
      ))}

      {/* === OUTPUT PULSE (target glows when data arrives) === */}
      {showOutputPulse && (
        <g>
          {/* Expanding ring at end of path */}
          <circle
            r={10}
            fill="none"
            stroke={colors.glow}
            strokeWidth={3}
            opacity={0}
          >
            <animateMotion
              dur="0.01s"
              repeatCount="indefinite"
              begin="0s"
              path={edgePath}
              keyPoints="1;1"
              keyTimes="0;1"
            />
            {/* Pulse when characters arrive (every ~2s on average) */}
            <animate
              attributeName="opacity"
              values="0;0.8;0"
              dur="0.8s"
              repeatCount="indefinite"
              begin="1.5s"
            />
            <animate
              attributeName="r"
              values="8;25"
              dur="0.8s"
              repeatCount="indefinite"
              begin="1.5s"
            />
            <animate
              attributeName="stroke-width"
              values="4;1"
              dur="0.8s"
              repeatCount="indefinite"
              begin="1.5s"
            />
          </circle>

          {/* Central glow pulse */}
          <circle
            r={isHighlighted ? 12 : 8}
            fill={colors.primary}
            opacity={0}
            style={{ filter: `drop-shadow(0 0 10px ${colors.glow})` }}
          >
            <animateMotion
              dur="0.01s"
              repeatCount="indefinite"
              begin="0s"
              path={edgePath}
              keyPoints="1;1"
              keyTimes="0;1"
            />
            <animate
              attributeName="opacity"
              values="0;1;0.3;0"
              dur="0.6s"
              repeatCount="indefinite"
              begin="1.5s"
              keyTimes="0;0.1;0.5;1"
            />
          </circle>
        </g>
      )}
    </g>
  );
});

export default MatrixCodeRain;
