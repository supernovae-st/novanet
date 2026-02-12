'use client';

/**
 * SonarPulse - Mining Arc Effect
 *
 * Semantic: SEO/GEO data extraction, discovery, market intelligence
 *
 * Visual Concept:
 * - Ping source: Bright pulse emitted from source node
 * - Wave propagation: 4 concentric rings expanding along arc
 * - Echo return: Faint reflection wave traveling back
 * - Data blip: Small bright dot at target when wave arrives
 * - Ambient scan: Slow sweeping highlight (10s cycle)
 *
 * Metaphor: Sonar pinging and discovering data in the depths
 */

import { memo } from 'react';
import type { EffectTier } from '../EdgeVisibilityManager';
import { getRandomDelay } from '../EdgeUtils';

interface SonarPulseProps {
  edgePath: string;
  colors: { primary: string; secondary: string; glow: string };
  state: 'default' | 'highlighted' | 'selected' | 'muted';
  effectTier: EffectTier;
  edgeId: string;
}

/**
 * SonarPulse Effect - Mining family signature effect
 *
 * TIER SCALING:
 * - ULTRA: 4 rings + echo return + ambient scan + data blips
 * - HIGH: 3 rings + echo return + data blips
 * - MEDIUM: 2 rings + data blip
 * - LOW: Fallback to SimplifiedEdgeEffect (not rendered here)
 */
export const SonarPulse = memo(function SonarPulse({
  edgePath,
  colors,
  state,
  effectTier,
  edgeId,
}: SonarPulseProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';

  // TIER-BASED SCALING
  const ringCount = effectTier === 'ultra' ? 4 : effectTier === 'medium' ? 2 : 3;
  const showEchoReturn = effectTier !== 'medium';
  const showAmbientScan = effectTier === 'ultra';
  const dataBlipCount = effectTier === 'ultra' ? 3 : effectTier === 'medium' ? 1 : 2;

  // Random delay to desynchronize animations across edges
  const baseDelay = getRandomDelay(edgeId, 3);

  // Ping timing (with baseDelay offset)
  const pingInterval = 3; // Every 3s (methodical scanning)
  const ringExpansionTime = 1.5; // Time for ring to expand source->target
  const echoReturnTime = 2; // Slower return

  // Ambient scan timing (slow sweep)
  const ambientScanDuration = 10;

  return (
    <g className="effect-sonar-pulse">
      {/* === AMBIENT SCAN (slow sweeping highlight) === */}
      {showAmbientScan && (
        <>
          {/* Wide sweeping glow */}
          <circle
            r={isHighlighted ? 35 : 28}
            fill={colors.glow}
            opacity={0}
            style={{ filter: 'blur(15px)' }}
          >
            <animateMotion
              dur={`${ambientScanDuration}s`}
              repeatCount="indefinite"
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0.1;0.25;0.1"
              dur={`${ambientScanDuration}s`}
              repeatCount="indefinite"
            />
          </circle>

          {/* Inner scan highlight */}
          <circle
            r={isHighlighted ? 18 : 14}
            fill={colors.primary}
            opacity={0}
            style={{ filter: 'blur(6px)' }}
          >
            <animateMotion
              dur={`${ambientScanDuration}s`}
              repeatCount="indefinite"
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0.2;0.4;0.2"
              dur={`${ambientScanDuration}s`}
              repeatCount="indefinite"
            />
          </circle>
        </>
      )}

      {/* === PING SOURCE (emitted from source node) === */}
      <g>
        {/* Ping origin flash */}
        <circle
          r={isHighlighted ? 14 : 10}
          fill="#ffffff"
          opacity={0}
          style={{ filter: `drop-shadow(0 0 12px #ffffff)` }}
        >
          <animateMotion
            dur="0.01s"
            repeatCount="indefinite"
            begin={`${baseDelay}s`}
            path={edgePath}
            keyPoints="0;0"
            keyTimes="0;1"
          />
          <animate
            attributeName="opacity"
            values="0;1;0"
            dur="0.3s"
            repeatCount="indefinite"
            begin={`${baseDelay}s;${baseDelay + pingInterval}s`}
            keyTimes="0;0.15;1"
          />
          <animate
            attributeName="r"
            values="6;16;8"
            dur="0.3s"
            repeatCount="indefinite"
            begin={`${baseDelay}s;${baseDelay + pingInterval}s`}
          />
        </circle>

        {/* Ping glow burst */}
        <circle
          r={isHighlighted ? 20 : 16}
          fill={colors.primary}
          opacity={0}
          style={{ filter: 'blur(6px)' }}
        >
          <animateMotion
            dur="0.01s"
            repeatCount="indefinite"
            begin={`${baseDelay}s`}
            path={edgePath}
            keyPoints="0;0"
            keyTimes="0;1"
          />
          <animate
            attributeName="opacity"
            values="0;0.6;0"
            dur="0.4s"
            repeatCount="indefinite"
            begin={`${baseDelay}s;${baseDelay + pingInterval}s`}
          />
        </circle>
      </g>

      {/* === WAVE PROPAGATION (concentric rings expanding along arc) === */}
      {Array.from({ length: ringCount }, (_, i) => {
        const ringDelay = baseDelay + i * 0.15; // Staggered rings with base offset

        return (
          <g key={`ring-${i}`}>
            {/* Expanding ring */}
            <circle
              r={isHighlighted ? 12 : 10}
              fill="none"
              stroke={colors.primary}
              strokeWidth={3 - i * 0.5}
              opacity={0}
            >
              <animateMotion
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="opacity"
                values="0;0.8;0.5;0"
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
                keyTimes="0;0.1;0.7;1"
              />
              <animate
                attributeName="r"
                values={`${8 + i * 4};${18 + i * 6};${25 + i * 8}`}
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
              />
              <animate
                attributeName="stroke-width"
                values={`${3 - i * 0.5};${2 - i * 0.3};${1}`}
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
              />
            </circle>

            {/* Ring glow */}
            <circle
              r={isHighlighted ? 14 : 12}
              fill="none"
              stroke={colors.glow}
              strokeWidth={8 - i * 1.5}
              opacity={0}
              style={{ filter: 'blur(4px)' }}
            >
              <animateMotion
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
                path={edgePath}
              />
              <animate
                attributeName="opacity"
                values="0;0.4;0.2;0"
                dur={`${ringExpansionTime}s`}
                repeatCount="indefinite"
                begin={`${ringDelay}s;${pingInterval + ringDelay}s`}
                keyTimes="0;0.1;0.6;1"
              />
            </circle>
          </g>
        );
      })}

      {/* === ECHO RETURN (faint reflection traveling back) === */}
      {showEchoReturn && (
        <g>
          {/* Echo wave */}
          <circle
            r={isHighlighted ? 10 : 8}
            fill="none"
            stroke={colors.secondary}
            strokeWidth={1.5}
            opacity={0}
          >
            <animateMotion
              dur={`${echoReturnTime}s`}
              repeatCount="indefinite"
              begin={`${baseDelay + ringExpansionTime + 0.1}s;${baseDelay + pingInterval + ringExpansionTime + 0.1}s`}
              path={edgePath}
              keyPoints="1;0"
              keyTimes="0;1"
            />
            <animate
              attributeName="opacity"
              values="0;0.5;0.3;0"
              dur={`${echoReturnTime}s`}
              repeatCount="indefinite"
              begin={`${baseDelay + ringExpansionTime + 0.1}s;${baseDelay + pingInterval + ringExpansionTime + 0.1}s`}
              keyTimes="0;0.1;0.7;1"
            />
          </circle>

          {/* Echo glow */}
          <circle
            r={isHighlighted ? 16 : 12}
            fill={colors.secondary}
            opacity={0}
            style={{ filter: 'blur(6px)' }}
          >
            <animateMotion
              dur={`${echoReturnTime}s`}
              repeatCount="indefinite"
              begin={`${baseDelay + ringExpansionTime + 0.1}s;${baseDelay + pingInterval + ringExpansionTime + 0.1}s`}
              path={edgePath}
              keyPoints="1;0"
              keyTimes="0;1"
            />
            <animate
              attributeName="opacity"
              values="0;0.3;0.1;0"
              dur={`${echoReturnTime}s`}
              repeatCount="indefinite"
              begin={`${baseDelay + ringExpansionTime + 0.1}s;${baseDelay + pingInterval + ringExpansionTime + 0.1}s`}
              keyTimes="0;0.1;0.6;1"
            />
          </circle>
        </g>
      )}

      {/* === DATA BLIPS (bright dots when wave arrives at target) === */}
      {Array.from({ length: dataBlipCount }, (_, i) => {
        const blipDelay = baseDelay + ringExpansionTime + i * 0.2;

        return (
          <g key={`blip-${i}`}>
            {/* Blip flash */}
            <circle
              r={isHighlighted ? 8 : 6}
              fill={colors.primary}
              opacity={0}
              style={{
                filter: `drop-shadow(0 0 8px ${colors.glow})`,
              }}
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
                values="0;1;0"
                dur="0.5s"
                repeatCount="indefinite"
                begin={`${blipDelay}s;${pingInterval + blipDelay}s`}
                keyTimes="0;0.2;1"
              />
              <animate
                attributeName="r"
                values="4;10;6"
                dur="0.5s"
                repeatCount="indefinite"
                begin={`${blipDelay}s;${pingInterval + blipDelay}s`}
              />
            </circle>

            {/* Blip glow expansion */}
            <circle
              r={isHighlighted ? 15 : 12}
              fill={colors.glow}
              opacity={0}
              style={{ filter: 'blur(6px)' }}
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
                values="0;0.5;0"
                dur="0.6s"
                repeatCount="indefinite"
                begin={`${blipDelay}s;${pingInterval + blipDelay}s`}
              />
              <animate
                attributeName="r"
                values="8;20"
                dur="0.6s"
                repeatCount="indefinite"
                begin={`${blipDelay}s;${pingInterval + blipDelay}s`}
              />
            </circle>
          </g>
        );
      })}
    </g>
  );
});

export default SonarPulse;
