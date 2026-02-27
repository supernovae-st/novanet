'use client';

/**
 * SelectionEffect - Premium visual feedback for selected edges
 *
 * v11.6.10 - Dual Layer Particle System (Smooth Edition):
 * - LAYER 1: Small Orbs - 10 particles (3-7px, 2-4s) with smooth flowing wave
 * - LAYER 2: Matrix Rain - 25 Katakana/Cyrillic/Chinese chars with smooth drift
 *   - Alternating colors: white/primary/glow with white stroke outline
 *   - Smooth cubic-bezier animations (shonen anime style)
 * - Source: FOUNTAIN + COMET - 10 particles flying outward + 5 comets toward target (energy OUT)
 * - Target: VORTEX - 8 dots + 12 Matrix chars spiraling inward + contracting rings (energy IN)
 * - Clear visual distinction: Source particles LEAVE vs Target particles ARRIVE
 * - Zigzag: Both layers oscillate around arc path (not exact follow)
 *
 * @see docs/plans/2026-02-11-arc-effects-redesign.md
 */

import { memo, useMemo } from 'react';

/** Matrix character sets - NO emojis, NO shapes */
const MATRIX_CHARS = {
  katakana: 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン',
  cyrillic: 'ѦѪѮѰѲѴҀЋЂЄѢѤѨѬѰ',
  chinese: '电雷光力能气火水风云天地日月星辰龙凤',
};

/** Get random matrix character */
function getRandomMatrixChar(): string {
  const allChars = MATRIX_CHARS.katakana + MATRIX_CHARS.cyrillic + MATRIX_CHARS.chinese;
  return allChars[Math.floor(Math.random() * allChars.length)];
}

/** Generate array of matrix characters with random properties */
function generateMatrixChars(count: number, seed: string): Array<{ char: string; delay: number; duration: number; offsetY: number; size: number }> {
  // Use seed for deterministic but varied results
  const chars: Array<{ char: string; delay: number; duration: number; offsetY: number; size: number }> = [];
  for (let i = 0; i < count; i++) {
    const hash = (seed.charCodeAt(i % seed.length) + i * 17) % 100;
    // Mix of speeds: some fast, most medium-slow
    const speedType = hash % 4;
    let duration: number;
    if (speedType === 0) duration = 2 + (hash % 5) * 0.3; // Fast: 2-3.5s
    else if (speedType === 1) duration = 3.5 + (hash % 6) * 0.4; // Medium: 3.5-6s
    else duration = 5 + (hash % 8) * 0.5; // Slow: 5-9s

    chars.push({
      char: getRandomMatrixChar(),
      delay: (i * 0.25) + (hash % 6) * 0.2,
      duration,
      offsetY: ((hash % 40) - 20), // -20 to +20 random offset
      size: 12 + (hash % 8), // 12-20px (smaller, cleaner)
    });
  }
  return chars;
}

interface SelectionEffectProps {
  edgePath: string;
  sourcePoint: { x: number; y: number };
  targetPoint: { x: number; y: number };
  colors: { primary: string; glow: string };
  strokeWidth: number;
  /** Edge ID for unique animation identifiers */
  edgeId: string;
}

/**
 * Calculate angle in degrees from source to target
 */
function getDirectionAngle(
  source: { x: number; y: number },
  target: { x: number; y: number }
): number {
  const dx = target.x - source.x;
  const dy = target.y - source.y;
  return Math.atan2(dy, dx) * (180 / Math.PI);
}

/**
 * SelectionEffect - Maximum WOW when edge is selected
 *
 * Visual layers:
 * 1. Ultra wide outer energy field (pulsing)
 * 2. Wide pulse layer
 * 3. Core glow layer
 * 4. White hot inner core
 * 5. Direction particles (3 orbs flowing source → target)
 * 6. Source endpoint (LARGE sonar rings - origin)
 * 7. Target endpoint (LARGE arrow + rings - destination)
 */
export const SelectionEffect = memo(function SelectionEffect({
  edgePath,
  sourcePoint,
  targetPoint,
  colors,
  strokeWidth,
  edgeId,
}: SelectionEffectProps) {
  // Calculate direction angle for target arrow
  const _targetAngle = useMemo(
    () => getDirectionAngle(sourcePoint, targetPoint),
    [sourcePoint, targetPoint]
  );

  // Unique animation name suffix to prevent conflicts with other selected edges
  const _animId = useMemo(() => edgeId.replace(/[^a-zA-Z0-9]/g, '_'), [edgeId]);

  // Generate matrix characters for this edge (memoized for stability)
  // More chars for dense Matrix effect
  const matrixChars = useMemo(() => generateMatrixChars(25, edgeId), [edgeId]);

  return (
    <g className="selection-effect" style={{ animation: 'selectionFadeIn 0.3s ease-out' }}>
      {/* === GLOW LAYERS (from outer to inner) === */}

      {/* Ultra wide outer energy field - dramatic pulsing */}
      <path
        d={edgePath}
        fill="none"
        stroke={colors.glow}
        strokeWidth={strokeWidth + 50}
        strokeLinecap="round"
        style={{ filter: 'blur(25px)' }}
      >
        <animate
          attributeName="opacity"
          values="0.15;0.4;0.15"
          dur="2s"
          repeatCount="indefinite"
        />
      </path>

      {/* Wide pulse layer */}
      <path
        d={edgePath}
        fill="none"
        stroke={colors.glow}
        strokeWidth={strokeWidth + 30}
        strokeLinecap="round"
        style={{ filter: 'blur(15px)' }}
      >
        <animate
          attributeName="opacity"
          values="0.4;1;0.4"
          dur="1.5s"
          repeatCount="indefinite"
        />
      </path>

      {/* Core glow layer */}
      <path
        d={edgePath}
        fill="none"
        stroke={colors.primary}
        strokeWidth={strokeWidth + 16}
        strokeLinecap="round"
        style={{ filter: 'blur(8px)' }}
      >
        <animate
          attributeName="opacity"
          values="0.4;1;0.4"
          dur="1.2s"
          repeatCount="indefinite"
        />
      </path>

      {/* White hot inner core */}
      <path
        d={edgePath}
        fill="none"
        stroke="#ffffff"
        strokeWidth={strokeWidth + 4}
        strokeLinecap="round"
        opacity={0.9}
        style={{ filter: 'blur(2px)' }}
      />

      {/* === SOURCE ENDPOINT (FOUNTAIN + COMET LAUNCH - particles flying OUT) === */}
      <g style={{ filter: `drop-shadow(0 0 15px ${colors.glow})` }}>
        {/* FOUNTAIN: Particles spawning at center, flying outward in all directions */}
        {Array.from({ length: 10 }, (_, i) => {
          const angle = (i * 36) * (Math.PI / 180); // 10 particles, evenly spaced
          const endX = sourcePoint.x + Math.cos(angle) * 55;
          const endY = sourcePoint.y + Math.sin(angle) * 55;
          const duration = 1.8 + (i % 3) * 0.4;
          const delay = i * 0.15;
          const size = 3 + (i % 3);
          return (
            <g key={`fountain-${i}`}>
              {/* Particle glow */}
              <circle r={size + 3} fill={colors.glow} opacity={0}>
                <animate attributeName="cx" values={`${sourcePoint.x};${endX}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="cy" values={`${sourcePoint.y};${endY}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="opacity" values="0;0.7;0" keyTimes="0;0.3;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </circle>
              {/* Particle core */}
              <circle r={size} fill={i % 2 === 0 ? '#ffffff' : colors.primary} opacity={0}>
                <animate attributeName="cx" values={`${sourcePoint.x};${endX}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="cy" values={`${sourcePoint.y};${endY}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="opacity" values="0;1;0" keyTimes="0;0.2;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </circle>
            </g>
          );
        })}

        {/* COMET LAUNCH: Particles flying toward the arc direction */}
        {Array.from({ length: 5 }, (_, i) => {
          // Calculate direction toward target
          const dx = targetPoint.x - sourcePoint.x;
          const dy = targetPoint.y - sourcePoint.y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          const dirX = dx / dist;
          const dirY = dy / dist;
          // Spread slightly around the main direction
          const spreadAngle = ((i - 2) * 15) * (Math.PI / 180);
          const cosSpread = Math.cos(spreadAngle);
          const sinSpread = Math.sin(spreadAngle);
          const finalDirX = dirX * cosSpread - dirY * sinSpread;
          const finalDirY = dirX * sinSpread + dirY * cosSpread;
          const endX = sourcePoint.x + finalDirX * 70;
          const endY = sourcePoint.y + finalDirY * 70;
          const duration = 1.5 + (i % 2) * 0.3;
          const delay = i * 0.25;
          const size = 4 + (i % 2) * 2;
          return (
            <g key={`comet-${i}`}>
              {/* Comet trail */}
              <line
                x1={sourcePoint.x}
                y1={sourcePoint.y}
                x2={sourcePoint.x}
                y2={sourcePoint.y}
                stroke={colors.glow}
                strokeWidth={size - 1}
                strokeLinecap="round"
                opacity={0}
              >
                <animate attributeName="x1" values={`${sourcePoint.x};${sourcePoint.x + finalDirX * 20};${endX - finalDirX * 15}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1;0.4 0 0.2 1" />
                <animate attributeName="y1" values={`${sourcePoint.y};${sourcePoint.y + finalDirY * 20};${endY - finalDirY * 15}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1;0.4 0 0.2 1" />
                <animate attributeName="x2" values={`${sourcePoint.x};${sourcePoint.x + finalDirX * 35};${endX}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1;0.4 0 0.2 1" />
                <animate attributeName="y2" values={`${sourcePoint.y};${sourcePoint.y + finalDirY * 35};${endY}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1;0.4 0 0.2 1" />
                <animate attributeName="opacity" values="0;0.6;0" keyTimes="0;0.3;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </line>
              {/* Comet head */}
              <circle r={size} fill="#ffffff" opacity={0}>
                <animate attributeName="cx" values={`${sourcePoint.x};${endX}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="cy" values={`${sourcePoint.y};${endY}`} dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} calcMode="spline" keySplines="0.4 0 0.2 1" />
                <animate attributeName="opacity" values="0;1;0" keyTimes="0;0.15;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </circle>
            </g>
          );
        })}

        {/* Soft expanding pulse */}
        <circle cx={sourcePoint.x} cy={sourcePoint.y} r={15} fill="none" stroke={colors.glow} strokeWidth={2} opacity={0}>
          <animate attributeName="r" values="15;45;15" dur="2s" repeatCount="indefinite" />
          <animate attributeName="opacity" values="0;0.5;0" dur="2s" repeatCount="indefinite" />
        </circle>

        {/* Core orb */}
        <circle cx={sourcePoint.x} cy={sourcePoint.y} r={14} fill={colors.primary} opacity={0.9}>
          <animate attributeName="r" values="14;16;14" dur="1s" repeatCount="indefinite" />
        </circle>
        <circle cx={sourcePoint.x} cy={sourcePoint.y} r={7} fill="#ffffff" opacity={0.95} />
      </g>

      {/* === LAYER 1: SMALL ORBS (10 particles) === */}
      {Array.from({ length: 10 }, (_, i) => {
        const size = 3 + (i % 3) * 2; // 3-7px (small)
        const duration = 2 + (i % 4) * 0.5; // 2-4s (fast)
        const delay = i * 0.3;
        return (
          <g key={`small-orb-${i}`} className={`atom-orb-${i % 3}`}>
            {/* Glow */}
            <circle r={size + 3} fill={colors.glow} style={{ filter: 'blur(4px)' }}>
              <animateMotion dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} path={edgePath} />
              <animate attributeName="opacity" values="0;0.6;0.6;0" keyTimes="0;0.1;0.8;0.9" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
            </circle>
            {/* Core */}
            <circle r={size} fill={colors.primary}>
              <animateMotion dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} path={edgePath} />
              <animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.1;0.8;0.9" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
            </circle>
            {/* White dot */}
            <circle r={size * 0.35} fill="#ffffff">
              <animateMotion dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} path={edgePath} />
              <animate attributeName="opacity" values="0;0.95;0.95;0" keyTimes="0;0.1;0.8;0.9" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
            </circle>
          </g>
        );
      })}

      {/* === LAYER 2: MATRIX RAIN (25 characters, dense Matrix effect, varied colors) === */}
      {matrixChars.map((m, i) => {
        // Alternate colors: white, primary, glow
        const colorType = i % 3;
        const fillColor = colorType === 0 ? '#ffffff' : colorType === 1 ? colors.primary : colors.glow;
        const glowColor = colorType === 0 ? colors.primary : '#ffffff';
        return (
          <text
            key={`matrix-${i}`}
            className={`matrix-char-${i % 4}`}
            textAnchor="middle"
            dominantBaseline="central"
            fill={fillColor}
            stroke={glowColor}
            strokeWidth={0.5}
            fontSize={m.size}
            fontFamily="'Courier New', monospace"
            fontWeight={i % 2 === 0 ? 'bold' : 'normal'}
            style={{
              filter: `drop-shadow(0 0 4px ${glowColor})`,
            }}
          >
            <animateMotion
              dur={`${m.duration}s`}
              repeatCount="indefinite"
              begin={`${m.delay}s`}
              path={edgePath}
            />
            <animate
              attributeName="opacity"
              values="0;0.95;0.95;0"
              keyTimes="0;0.08;0.78;0.88"
              dur={`${m.duration}s`}
              repeatCount="indefinite"
              begin={`${m.delay}s`}
            />
            {m.char}
          </text>
        );
      })}

      {/* === TARGET ENDPOINT (Implosion Vortex - dots spiral inward) === */}
      <g style={{ filter: `drop-shadow(0 0 25px ${colors.glow})` }}>
        {/* Outer glow halo */}
        <circle cx={targetPoint.x} cy={targetPoint.y} r={35} fill={colors.glow} opacity={0.25} style={{ filter: 'blur(15px)' }}>
          <animate attributeName="opacity" values="0.25;0.5;0.25" dur="1.5s" repeatCount="indefinite" />
        </circle>

        {/* Vortex particles - 8 dots spiraling inward */}
        {Array.from({ length: 8 }, (_, i) => {
          const angle = (i * 45) * (Math.PI / 180); // Evenly spaced
          const startRadius = 45 + (i % 3) * 10; // 45-65px start radius
          const duration = 1.5 + (i % 4) * 0.3; // 1.5-2.7s varied speeds
          const delay = i * 0.2;
          const size = 4 + (i % 3) * 2; // 4-8px dots
          // Start position on outer ring
          const startX = Math.cos(angle) * startRadius;
          const startY = Math.sin(angle) * startRadius;
          return (
            <g key={`vortex-${i}`} transform={`translate(${targetPoint.x}, ${targetPoint.y})`}>
              {/* Dot glow */}
              <circle
                cx={startX}
                cy={startY}
                r={size + 4}
                fill={colors.glow}
                className={`vortex-particle-${i % 4}`}
                style={{ filter: 'blur(4px)', transformOrigin: '0 0' }}
              >
                <animate attributeName="opacity" values="0;0.8;0.8;0" keyTimes="0;0.1;0.7;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </circle>
              {/* Dot core */}
              <circle
                cx={startX}
                cy={startY}
                r={size}
                fill={i % 2 === 0 ? '#ffffff' : colors.primary}
                className={`vortex-particle-${i % 4}`}
                style={{ transformOrigin: '0 0' }}
              >
                <animate attributeName="opacity" values="0;1;1;0" keyTimes="0;0.1;0.7;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              </circle>
            </g>
          );
        })}

        {/* Vortex Matrix chars - 12 characters spiraling inward */}
        {Array.from({ length: 12 }, (_, i) => {
          const angle = (i * 30) * (Math.PI / 180); // 30° apart
          const startRadius = 50 + (i % 4) * 8; // 50-74px start radius
          const duration = 2 + (i % 3) * 0.5; // 2-3s
          const delay = i * 0.15;
          const size = 10 + (i % 3) * 2; // 10-14px
          const startX = Math.cos(angle) * startRadius;
          const startY = Math.sin(angle) * startRadius;
          const char = getRandomMatrixChar();
          const colorType = i % 3;
          const fillColor = colorType === 0 ? '#ffffff' : colorType === 1 ? colors.primary : colors.glow;
          const glowColor = colorType === 0 ? colors.primary : '#ffffff';
          return (
            <text
              key={`vortex-char-${i}`}
              x={targetPoint.x + startX}
              y={targetPoint.y + startY}
              textAnchor="middle"
              dominantBaseline="central"
              fill={fillColor}
              stroke={glowColor}
              strokeWidth={0.5}
              fontSize={size}
              fontFamily="'Courier New', monospace"
              fontWeight="bold"
              className={`vortex-particle-${i % 4}`}
              style={{ filter: `drop-shadow(0 0 3px ${glowColor})`, transformOrigin: `${targetPoint.x}px ${targetPoint.y}px` }}
            >
              <animate attributeName="opacity" values="0;0.95;0.95;0" keyTimes="0;0.1;0.6;1" dur={`${duration}s`} repeatCount="indefinite" begin={`${delay}s`} />
              {char}
            </text>
          );
        })}

        {/* Inner attraction ring - pulses inward */}
        <circle cx={targetPoint.x} cy={targetPoint.y} r={30} fill="none" stroke={colors.glow} strokeWidth={2} opacity={0.6}>
          <animate attributeName="r" values="40;20;40" dur="1.2s" repeatCount="indefinite" />
          <animate attributeName="opacity" values="0;0.8;0" dur="1.2s" repeatCount="indefinite" />
        </circle>
        <circle cx={targetPoint.x} cy={targetPoint.y} r={30} fill="none" stroke={colors.primary} strokeWidth={2} opacity={0.5}>
          <animate attributeName="r" values="35;15;35" dur="1.2s" repeatCount="indefinite" begin="0.4s" />
          <animate attributeName="opacity" values="0;0.7;0" dur="1.2s" repeatCount="indefinite" begin="0.4s" />
        </circle>

        {/* Core attractor - pulsing center */}
        <circle cx={targetPoint.x} cy={targetPoint.y} r={18} fill={colors.primary} opacity={0.9}>
          <animate attributeName="r" values="18;14;18" dur="0.8s" repeatCount="indefinite" />
          <animate attributeName="opacity" values="0.9;1;0.9" dur="0.8s" repeatCount="indefinite" />
        </circle>

        {/* White hot center */}
        <circle cx={targetPoint.x} cy={targetPoint.y} r={10} fill="#ffffff" opacity={0.95}>
          <animate attributeName="r" values="10;7;10" dur="0.8s" repeatCount="indefinite" />
        </circle>
      </g>

      {/* === CSS ANIMATIONS === */}
      <style>{`
        @keyframes selectionFadeIn {
          from { opacity: 0; transform: scale(0.95); }
          to { opacity: 1; transform: scale(1); }
        }

        /* Atom orbs - smooth flowing wave (shonen anime style) */
        .atom-orb-0 { animation: atomWave0 2.5s cubic-bezier(0.45, 0.05, 0.55, 0.95) infinite; }
        .atom-orb-1 { animation: atomWave1 3s cubic-bezier(0.45, 0.05, 0.55, 0.95) infinite; }
        .atom-orb-2 { animation: atomWave2 2.8s cubic-bezier(0.45, 0.05, 0.55, 0.95) infinite; }

        @keyframes atomWave0 {
          0%, 100% { transform: translateY(-10px) translateX(-3px); }
          25% { transform: translateY(-2px) translateX(5px); }
          50% { transform: translateY(10px) translateX(2px); }
          75% { transform: translateY(3px) translateX(-4px); }
        }
        @keyframes atomWave1 {
          0%, 100% { transform: translateY(8px) translateX(4px); }
          25% { transform: translateY(2px) translateX(-3px); }
          50% { transform: translateY(-8px) translateX(-5px); }
          75% { transform: translateY(-1px) translateX(3px); }
        }
        @keyframes atomWave2 {
          0%, 100% { transform: translateY(-6px) translateX(-5px); }
          25% { transform: translateY(4px) translateX(2px); }
          50% { transform: translateY(8px) translateX(6px); }
          75% { transform: translateY(-3px) translateX(-2px); }
        }

        /* Matrix chars - smooth flowing drift (not choppy zigzag) */
        .matrix-char-0 { animation: matrixFlow0 3s cubic-bezier(0.37, 0, 0.63, 1) infinite; }
        .matrix-char-1 { animation: matrixFlow1 3.5s cubic-bezier(0.37, 0, 0.63, 1) infinite; }
        .matrix-char-2 { animation: matrixFlow2 4s cubic-bezier(0.37, 0, 0.63, 1) infinite; }
        .matrix-char-3 { animation: matrixFlow3 2.8s cubic-bezier(0.37, 0, 0.63, 1) infinite; }

        @keyframes matrixFlow0 {
          0%, 100% { transform: translate(-8px, -12px); }
          25% { transform: translate(4px, -4px); }
          50% { transform: translate(10px, 14px); }
          75% { transform: translate(-2px, 6px); }
        }
        @keyframes matrixFlow1 {
          0%, 100% { transform: translate(10px, 8px); }
          25% { transform: translate(2px, -6px); }
          50% { transform: translate(-10px, -14px); }
          75% { transform: translate(-4px, 2px); }
        }
        @keyframes matrixFlow2 {
          0%, 100% { transform: translate(-6px, -16px); }
          20% { transform: translate(2px, -6px); }
          40% { transform: translate(8px, 6px); }
          60% { transform: translate(4px, 16px); }
          80% { transform: translate(-4px, 4px); }
        }
        @keyframes matrixFlow3 {
          0%, 100% { transform: translate(6px, 10px); }
          33% { transform: translate(-4px, -4px); }
          66% { transform: translate(-8px, -12px); }
        }

        /* Vortex particles - smooth spiral inward */
        .vortex-particle-0 {
          animation: vortexSpiral0 2.5s cubic-bezier(0.25, 0.46, 0.45, 0.94) infinite;
        }
        .vortex-particle-1 {
          animation: vortexSpiral1 3s cubic-bezier(0.25, 0.46, 0.45, 0.94) infinite;
        }
        .vortex-particle-2 {
          animation: vortexSpiral2 3.5s cubic-bezier(0.25, 0.46, 0.45, 0.94) infinite;
        }
        .vortex-particle-3 {
          animation: vortexSpiral3 2.8s cubic-bezier(0.25, 0.46, 0.45, 0.94) infinite;
        }

        @keyframes vortexSpiral0 {
          0% { transform: rotate(0deg) scale(1); }
          50% { transform: rotate(180deg) scale(0.5); }
          100% { transform: rotate(360deg) scale(0.1); }
        }
        @keyframes vortexSpiral1 {
          0% { transform: rotate(0deg) scale(1); }
          50% { transform: rotate(-150deg) scale(0.55); }
          100% { transform: rotate(-300deg) scale(0.15); }
        }
        @keyframes vortexSpiral2 {
          0% { transform: rotate(0deg) scale(1); }
          50% { transform: rotate(210deg) scale(0.5); }
          100% { transform: rotate(420deg) scale(0.1); }
        }
        @keyframes vortexSpiral3 {
          0% { transform: rotate(0deg) scale(1); }
          50% { transform: rotate(-135deg) scale(0.6); }
          100% { transform: rotate(-270deg) scale(0.2); }
        }
      `}</style>
    </g>
  );
});

export default SelectionEffect;
