'use client';

/**
 * NodeHoverEffect - Glow and pulse effect for hovered nodes
 *
 * Features:
 * - Border glow intensifies with pulse
 * - Expanding highlight ring
 * - Connected edge trigger effect
 *
 * Note: CSS bounce animation should be applied via className.
 * This component handles the SVG glow effects only.
 *
 * Visual metaphor: Node "waking up" when user focuses attention
 */

import { memo } from 'react';

export interface NodeHoverEffectProps {
  /** Center X position of the node */
  cx: number;
  /** Center Y position of the node */
  cy: number;
  /** Node radius for effect positioning */
  nodeRadius: number;
  /** Primary glow color */
  primaryColor: string;
  /** Secondary color for outer ring */
  secondaryColor?: string;
  /** Animation duration in seconds */
  duration?: number;
  /** Intensity multiplier (0.5-2) */
  intensity?: number;
  /** Whether hover is active (controls animation state) */
  isActive?: boolean;
}

/**
 * NodeHoverEffect - Creates hover glow effects around nodes
 */
export const NodeHoverEffect = memo(function NodeHoverEffect({
  cx,
  cy,
  nodeRadius,
  primaryColor,
  secondaryColor,
  duration = 1.5,
  intensity = 1,
  isActive = true,
}: NodeHoverEffectProps) {
  const secondaryCol = secondaryColor || primaryColor;
  const scale = Math.max(0.5, Math.min(2, intensity));

  // Don't render if not active
  if (!isActive) return null;

  const glowRadius = nodeRadius * 1.5;
  const ringRadius = nodeRadius * 1.2;
  const pulseDuration = duration * 0.6;

  return (
    <g className="node-hover-effect">
      {/* ===== AMBIENT GLOW ===== */}
      {/* Soft diffuse glow around the node */}
      <circle
        cx={cx}
        cy={cy}
        r={glowRadius}
        fill={primaryColor}
        opacity={0.15 * scale}
        style={{ filter: 'blur(8px)' }}
      >
        <animate
          attributeName="opacity"
          values={`${0.1 * scale};${0.25 * scale};${0.1 * scale}`}
          dur={`${pulseDuration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="r"
          values={`${glowRadius * 0.95};${glowRadius * 1.15};${glowRadius * 0.95}`}
          dur={`${pulseDuration}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* ===== BORDER GLOW ===== */}
      {/* Ring that hugs the node border with pulse */}
      <circle
        cx={cx}
        cy={cy}
        r={nodeRadius}
        fill="none"
        stroke={primaryColor}
        strokeWidth={3 * scale}
        opacity={0.5 * scale}
      >
        <animate
          attributeName="opacity"
          values={`${0.4 * scale};${0.8 * scale};${0.4 * scale}`}
          dur={`${pulseDuration * 0.8}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="stroke-width"
          values={`${2 * scale};${4 * scale};${2 * scale}`}
          dur={`${pulseDuration * 0.8}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* ===== HIGHLIGHT RING ===== */}
      {/* Expanding ring that signals "active" state */}
      <circle
        cx={cx}
        cy={cy}
        r={ringRadius}
        fill="none"
        stroke={secondaryCol}
        strokeWidth={1.5 * scale}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${nodeRadius};${ringRadius * 1.3};${ringRadius * 1.5}`}
          keyTimes="0;0.7;1"
          dur={`${duration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`${0.5 * scale};${0.25 * scale};0`}
          keyTimes="0;0.6;1"
          dur={`${duration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="stroke-width"
          values={`${2 * scale};${1 * scale};${0.3 * scale}`}
          keyTimes="0;0.5;1"
          dur={`${duration}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* ===== SPARKLE POINTS ===== */}
      {/* Small bright points around the node */}
      {[0, 90, 180, 270].map((angle, idx) => {
        const rad = (angle * Math.PI) / 180;
        const sparkX = cx + Math.cos(rad) * nodeRadius * 1.15;
        const sparkY = cy + Math.sin(rad) * nodeRadius * 1.15;
        const sparkDelay = idx * pulseDuration * 0.15;

        return (
          <circle
            key={angle}
            cx={sparkX}
            cy={sparkY}
            r={2 * scale}
            fill="#ffffff"
            opacity={0}
          >
            <animate
              attributeName="opacity"
              values={`0;${0.9 * scale};0`}
              keyTimes="0;0.3;1"
              dur={`${pulseDuration * 0.5}s`}
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
            />
            <animate
              attributeName="r"
              values={`${1 * scale};${3 * scale};${1 * scale}`}
              keyTimes="0;0.3;1"
              dur={`${pulseDuration * 0.5}s`}
              repeatCount="indefinite"
              begin={`${sparkDelay}s`}
            />
          </circle>
        );
      })}
    </g>
  );
});

/**
 * NodeImpactBounce - Micro-bounce effect when edge impact occurs
 *
 * This is a simpler effect for when a particle "hits" the node
 */
export interface NodeImpactBounceProps {
  /** Center X position of the node */
  cx: number;
  /** Center Y position of the node */
  cy: number;
  /** Node radius */
  nodeRadius: number;
  /** Impact color (usually edge color) */
  impactColor: string;
  /** Animation duration in seconds */
  duration?: number;
  /** Intensity multiplier */
  intensity?: number;
}

export const NodeImpactBounce = memo(function NodeImpactBounce({
  cx,
  cy,
  nodeRadius,
  impactColor,
  duration = 0.3,
  intensity = 1,
}: NodeImpactBounceProps) {
  const scale = Math.max(0.5, Math.min(2, intensity));

  return (
    <g className="node-impact-bounce">
      {/* Flash ring at impact */}
      <circle
        cx={cx}
        cy={cy}
        r={nodeRadius}
        fill="none"
        stroke={impactColor}
        strokeWidth={2 * scale}
        opacity={0}
      >
        <animate
          attributeName="r"
          values={`${nodeRadius};${nodeRadius * 1.15};${nodeRadius}`}
          dur={`${duration}s`}
          repeatCount="indefinite"
        />
        <animate
          attributeName="opacity"
          values={`0;${0.7 * scale};0`}
          dur={`${duration}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Inner glow pulse */}
      <circle
        cx={cx}
        cy={cy}
        r={nodeRadius * 0.8}
        fill={impactColor}
        opacity={0}
      >
        <animate
          attributeName="opacity"
          values={`0;${0.3 * scale};0`}
          dur={`${duration * 0.6}s`}
          repeatCount="indefinite"
        />
      </circle>
    </g>
  );
});
