'use client';

/**
 * NodeOrbitEffect - Orbiting particles for selected nodes
 *
 * Features:
 * - 4-6 orbiting particles at varied speeds and sizes
 * - Breathing glow (opacity oscillates)
 * - Periodic ring pulse expanding outward
 *
 * Visual metaphor: Active node processing/radiating knowledge
 */

import { memo } from 'react';

export interface NodeOrbitEffectProps {
  /** Center X position of the node */
  cx: number;
  /** Center Y position of the node */
  cy: number;
  /** Node radius (for orbit distance calculation) */
  nodeRadius: number;
  /** Primary color for particles and glow */
  primaryColor: string;
  /** Secondary color for ring pulse */
  secondaryColor?: string;
  /** Base animation duration in seconds */
  duration?: number;
  /** Intensity multiplier (0.5-2) */
  intensity?: number;
}

/**
 * NodeOrbitEffect - Creates orbiting particle system around selected nodes
 */
export const NodeOrbitEffect = memo(function NodeOrbitEffect({
  cx,
  cy,
  nodeRadius,
  primaryColor,
  secondaryColor,
  duration = 4,
  intensity = 1,
}: NodeOrbitEffectProps) {
  const secondaryCol = secondaryColor || primaryColor;
  const scale = Math.max(0.5, Math.min(2, intensity));

  // Orbit configuration - 5 particles with varied properties
  const orbitParticles = [
    { radius: nodeRadius * 1.4, size: 4 * scale, duration: duration * 0.8, delay: 0, clockwise: true },
    { radius: nodeRadius * 1.6, size: 3 * scale, duration: duration * 1.1, delay: 0.5, clockwise: false },
    { radius: nodeRadius * 1.3, size: 2.5 * scale, duration: duration * 0.6, delay: 1.2, clockwise: true },
    { radius: nodeRadius * 1.5, size: 3.5 * scale, duration: duration * 0.9, delay: 1.8, clockwise: true },
    { radius: nodeRadius * 1.7, size: 2 * scale, duration: duration * 1.3, delay: 2.5, clockwise: false },
  ];

  // Ring pulse configuration
  const ringPulseDuration = duration * 0.4;
  const ringMaxRadius = nodeRadius * 2.2;

  // Breathing glow configuration
  const glowRadius = nodeRadius * 1.8;
  const breatheDuration = duration * 0.7;

  return (
    <g className="node-orbit-effect">
      {/* ===== BREATHING GLOW ===== */}
      {/* Ambient glow that pulses with opacity */}
      <circle
        cx={cx}
        cy={cy}
        r={glowRadius}
        fill={primaryColor}
        opacity={0}
      >
        <animate
          attributeName="opacity"
          values={`${0.08 * scale};${0.2 * scale};${0.08 * scale}`}
          dur={`${breatheDuration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="r"
          values={`${glowRadius * 0.9};${glowRadius * 1.15};${glowRadius * 0.9}`}
          dur={`${breatheDuration}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Secondary breathing layer */}
      <circle
        cx={cx}
        cy={cy}
        r={glowRadius * 0.7}
        fill={secondaryCol}
        opacity={0}
      >
        <animate
          attributeName="opacity"
          values={`${0.1 * scale};${0.25 * scale};${0.1 * scale}`}
          dur={`${breatheDuration * 0.8}s`}
          repeatCount="indefinite"
          begin={`${breatheDuration * 0.25}s`}
        />
      </circle>

      {/* ===== RING PULSE ===== */}
      {/* Expanding ring that fades out */}
      <circle
        cx={cx}
        cy={cy}
        r={nodeRadius}
        fill="none"
        stroke={primaryColor}
        strokeWidth={2.5 * scale}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${nodeRadius};${ringMaxRadius};${ringMaxRadius * 1.2}`}
          keyTimes="0;0.7;1"
          dur={`${ringPulseDuration}s`}
          repeatCount="indefinite"
          begin={`${duration * 0.5}s`}
        />
        <animate
          attributeName="opacity"
          values={`${0.6 * scale};${0.3 * scale};0`}
          keyTimes="0;0.6;1"
          dur={`${ringPulseDuration}s`}
          repeatCount="indefinite"
          begin={`${duration * 0.5}s`}
        />
        <animate
          attributeName="stroke-width"
          values={`${3 * scale};${1.5 * scale};${0.5 * scale}`}
          keyTimes="0;0.5;1"
          dur={`${ringPulseDuration}s`}
          repeatCount="indefinite"
          begin={`${duration * 0.5}s`}
        />
      </circle>

      {/* Secondary ring pulse - offset timing */}
      <circle
        cx={cx}
        cy={cy}
        r={nodeRadius}
        fill="none"
        stroke={secondaryCol}
        strokeWidth={1.5 * scale}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${nodeRadius * 0.9};${ringMaxRadius * 0.85};${ringMaxRadius}`}
          keyTimes="0;0.65;1"
          dur={`${ringPulseDuration * 0.9}s`}
          repeatCount="indefinite"
          begin={`${duration * 0.6}s`}
        />
        <animate
          attributeName="opacity"
          values={`${0.4 * scale};${0.2 * scale};0`}
          keyTimes="0;0.5;1"
          dur={`${ringPulseDuration * 0.9}s`}
          repeatCount="indefinite"
          begin={`${duration * 0.6}s`}
        />
      </circle>

      {/* ===== ORBITING PARTICLES ===== */}
      {orbitParticles.map((particle, idx) => {
        // Create circular orbit path
        const orbitPathId = `orbit-path-${idx}-${cx}-${cy}`;
        const direction = particle.clockwise ? 1 : -1;

        return (
          <g key={idx}>
            {/* Hidden orbit path for animateMotion */}
            <defs>
              <circle
                id={orbitPathId}
                cx={cx}
                cy={cy}
                r={particle.radius}
                fill="none"
              />
            </defs>

            {/* Particle glow trail */}
            <circle
              r={particle.size * 2}
              fill={primaryColor}
              opacity={0.2 * scale}
            >
              <animateMotion
                dur={`${particle.duration}s`}
                repeatCount="indefinite"
                begin={`${particle.delay}s`}
                keyPoints={direction === 1 ? '0;1' : '1;0'}
                keyTimes="0;1"
              >
                <mpath href={`#${orbitPathId}`} />
              </animateMotion>
              <animate
                attributeName="opacity"
                values={`${0.15 * scale};${0.3 * scale};${0.15 * scale}`}
                dur={`${particle.duration * 0.3}s`}
                repeatCount="indefinite"
                begin={`${particle.delay}s`}
              />
            </circle>

            {/* Main particle */}
            <circle
              r={particle.size}
              fill={primaryColor}
              opacity={0.9}
            >
              <animateMotion
                dur={`${particle.duration}s`}
                repeatCount="indefinite"
                begin={`${particle.delay}s`}
                keyPoints={direction === 1 ? '0;1' : '1;0'}
                keyTimes="0;1"
              >
                <mpath href={`#${orbitPathId}`} />
              </animateMotion>
            </circle>

            {/* Hot center of particle */}
            <circle
              r={particle.size * 0.4}
              fill="#ffffff"
              opacity={0.95}
            >
              <animateMotion
                dur={`${particle.duration}s`}
                repeatCount="indefinite"
                begin={`${particle.delay}s`}
                keyPoints={direction === 1 ? '0;1' : '1;0'}
                keyTimes="0;1"
              >
                <mpath href={`#${orbitPathId}`} />
              </animateMotion>
              <animate
                attributeName="opacity"
                values="0.7;1;0.7"
                dur={`${particle.duration * 0.2}s`}
                repeatCount="indefinite"
                begin={`${particle.delay}s`}
              />
            </circle>
          </g>
        );
      })}
    </g>
  );
});
