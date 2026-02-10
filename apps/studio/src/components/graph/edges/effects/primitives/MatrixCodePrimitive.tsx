'use client';

/**
 * MatrixCodePrimitive - Flowing code characters along edge
 *
 * Visual: Matrix-style characters flowing along the path
 * Purpose: Generation family - AI processing data
 *
 * Technique:
 * - <text> elements with animateMotion
 * - useEffect interval to cycle characters randomly
 * - Staggered start times for wave effect
 * - Glow filter: drop-shadow for matrix aesthetic
 */

import { memo, useMemo, useState, useEffect, useRef } from 'react';
import type { EffectPrimitiveProps } from '../../system/types';
import { EASING_PRESETS } from '../../system/constants';

interface CharConfig {
  id: number;
  delay: number;
  duration: number;
  fontSize: number;
  isLeader: boolean;
}

export interface MatrixCodePrimitiveProps extends EffectPrimitiveProps {
  /** Character set to use for the effect */
  charSet?: string;
  /** Number of flowing characters */
  charCount?: number;
  /** Font size in pixels */
  charSize?: number;
  /** How often characters change (ms) */
  cycleSpeed?: number;
}

// Matrix character set: katakana + numbers + symbols
const DEFAULT_CHAR_SET = 'アイウエオカキクケコサシスセソタチツテト0123456789<>{}[]═║╔╗╚╝';

// Default configuration
const DEFAULTS = {
  charCount: 8,
  charSize: 11,
  cycleSpeed: 100,
  leaderMultiplier: 1.3,
};

// Character cycling hook - returns array of characters that update periodically
function useMatrixChars(
  charSet: string,
  charCount: number,
  cycleSpeed: number,
  isActive: boolean,
): string[] {
  const [chars, setChars] = useState<string[]>(() =>
    Array.from({ length: charCount }, () =>
      charSet[Math.floor(Math.random() * charSet.length)]
    )
  );

  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  useEffect(() => {
    if (!isActive) {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      return;
    }

    intervalRef.current = setInterval(() => {
      setChars((prev) =>
        prev.map((_, i) => {
          // Each character has 30% chance to change per tick
          if (Math.random() < 0.3) {
            return charSet[Math.floor(Math.random() * charSet.length)];
          }
          return prev[i];
        })
      );
    }, cycleSpeed);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [charSet, charCount, cycleSpeed, isActive]);

  return chars;
}

export const MatrixCodePrimitive = memo(function MatrixCodePrimitive({
  pathId,
  colors,
  timing,
  intensity,
  state,
  charSet = DEFAULT_CHAR_SET,
  charCount = DEFAULTS.charCount,
  charSize = DEFAULTS.charSize,
  cycleSpeed = DEFAULTS.cycleSpeed,
}: MatrixCodePrimitiveProps) {
  const isHighlighted = state === 'highlighted' || state === 'selected';
  const baseFontSize = isHighlighted ? charSize * 1.2 : charSize;
  const adjustedFontSize = baseFontSize * intensity;

  // Cycle characters (only when intensity is sufficient)
  const isActive = intensity >= 0.15;
  const chars = useMatrixChars(charSet, charCount, cycleSpeed, isActive);

  // Generate character configurations
  const charConfigs = useMemo((): CharConfig[] => {
    return Array.from({ length: charCount }, (_, i) => {
      const stagger = (i / charCount) * timing.duration;
      const durationVariation = 1 + (i % 4 === 0 ? 0.1 : i % 4 === 1 ? -0.05 : 0.03);

      return {
        id: i,
        delay: stagger,
        duration: timing.duration * durationVariation,
        fontSize: i === 0 ? adjustedFontSize * DEFAULTS.leaderMultiplier : adjustedFontSize,
        isLeader: i === 0,
      };
    });
  }, [charCount, timing.duration, adjustedFontSize]);

  // Don't render if intensity too low
  if (!isActive) return null;

  const spline = EASING_PRESETS.linear;
  const filterId = `matrix-glow-${pathId}`;

  return (
    <g className="matrix-code-primitive">
      {/* SVG Filter for matrix glow effect */}
      <defs>
        <filter id={filterId} x="-50%" y="-50%" width="200%" height="200%">
          <feDropShadow
            dx="0"
            dy="0"
            stdDeviation="3"
            floodColor={colors.primary}
            floodOpacity={0.8}
          />
          <feDropShadow
            dx="0"
            dy="0"
            stdDeviation="6"
            floodColor={colors.glow}
            floodOpacity={0.4}
          />
        </filter>
      </defs>

      {charConfigs.map((config, index) => (
        <g key={`char-${config.id}`}>
          {/* Background glow layer */}
          <text
            fontSize={config.fontSize}
            fill={colors.glow}
            fontFamily="'Courier New', 'Consolas', monospace"
            fontWeight="bold"
            textAnchor="middle"
            dominantBaseline="central"
            opacity={0}
            style={{ filter: 'blur(4px)' }}
          >
            {chars[index] || '0'}
            <animateMotion
              dur={`${config.duration}s`}
              repeatCount="indefinite"
              begin={`${config.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.5 * intensity};${0.5 * intensity};0`}
              keyTimes="0;0.08;0.88;1"
              dur={`${config.duration}s`}
              begin={`${config.delay}s`}
              repeatCount="indefinite"
            />
          </text>

          {/* Main character */}
          <text
            fontSize={config.fontSize}
            fill={config.isLeader ? colors.primary : colors.secondary}
            fontFamily="'Courier New', 'Consolas', monospace"
            fontWeight="bold"
            textAnchor="middle"
            dominantBaseline="central"
            opacity={0}
            filter={`url(#${filterId})`}
          >
            {chars[index] || '0'}
            <animateMotion
              dur={`${config.duration}s`}
              repeatCount="indefinite"
              begin={`${config.delay}s`}
              calcMode="spline"
              keySplines={spline}
              keyTimes="0;1"
            >
              <mpath href={`#${pathId}`} />
            </animateMotion>
            <animate
              attributeName="opacity"
              values={`0;${0.95 * intensity};${0.95 * intensity};0`}
              keyTimes="0;0.06;0.9;1"
              dur={`${config.duration}s`}
              begin={`${config.delay}s`}
              repeatCount="indefinite"
            />
          </text>

          {/* Bright overlay for leader */}
          {config.isLeader && (
            <text
              fontSize={config.fontSize}
              fill="#ffffff"
              fontFamily="'Courier New', 'Consolas', monospace"
              fontWeight="bold"
              textAnchor="middle"
              dominantBaseline="central"
              opacity={0}
            >
              {chars[index] || '0'}
              <animateMotion
                dur={`${config.duration}s`}
                repeatCount="indefinite"
                begin={`${config.delay}s`}
                calcMode="spline"
                keySplines={spline}
                keyTimes="0;1"
              >
                <mpath href={`#${pathId}`} />
              </animateMotion>
              <animate
                attributeName="opacity"
                values={`0;${0.6 * intensity};${0.4 * intensity};0`}
                keyTimes="0;0.1;0.8;1"
                dur={`${config.duration}s`}
                begin={`${config.delay}s`}
                repeatCount="indefinite"
              />
            </text>
          )}
        </g>
      ))}

      {/* Trailing fade effect - small dots that follow */}
      {charConfigs.slice(0, 3).map((config) => (
        <circle
          key={`trail-${config.id}`}
          r={2 * intensity}
          fill={colors.tertiary}
          opacity={0}
        >
          <animateMotion
            dur={`${config.duration}s`}
            repeatCount="indefinite"
            begin={`${config.delay + 0.1}s`}
            calcMode="spline"
            keySplines={spline}
            keyTimes="0;1"
          >
            <mpath href={`#${pathId}`} />
          </animateMotion>
          <animate
            attributeName="opacity"
            values={`0;${0.4 * intensity};${0.3 * intensity};0`}
            keyTimes="0;0.15;0.85;1"
            dur={`${config.duration}s`}
            begin={`${config.delay + 0.1}s`}
            repeatCount="indefinite"
          />
        </circle>
      ))}
    </g>
  );
});
