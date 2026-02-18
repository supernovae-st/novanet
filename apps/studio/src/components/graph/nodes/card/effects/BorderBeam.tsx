'use client';

/**
 * BorderBeam - Rotating spotlight effect that travels around the border
 *
 * Creates a premium "beam of light" that continuously rotates around
 * the card border, creating a magical scanning effect.
 *
 * Inspired by Aceternity UI and MagicUI border beam patterns.
 *
 * @example
 * ```tsx
 * <BorderBeam
 *   color="#8b5cf6"
 *   borderRadius={16}
 *   duration={6}
 * />
 * ```
 */

import { memo, useMemo, useId } from 'react';
import { motion } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface BorderBeamProps {
  /** Beam color */
  color: string;
  /** Secondary color for gradient (optional) */
  secondaryColor?: string;
  /** Border radius in pixels */
  borderRadius?: number;
  /** Border thickness in pixels */
  thickness?: number;
  /** Animation duration in seconds */
  duration?: number;
  /** Whether the element is selected */
  selected?: boolean;
  /** Whether the element is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Beam length as percentage of perimeter (0-1) */
  beamLength?: number;
}

// =============================================================================
// Component
// =============================================================================

export const BorderBeam = memo(function BorderBeam({
  color,
  secondaryColor,
  borderRadius = 16,
  thickness = 2,
  duration = 6,
  selected = false,
  isHovered = false,
  performanceConfig,
  beamLength = 0.15,
}: BorderBeamProps) {
  const id = useId();
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  const secondary = secondaryColor ?? color;

  // Glow intensity based on state
  const glowIntensity = selected ? 1.5 : isHovered ? 1.2 : 1;
  const currentDuration = selected ? duration * 0.6 : isHovered ? duration * 0.8 : duration;

  // Convert beamLength to degrees (percentage of 360)
  const beamDegrees = beamLength * 360;

  if (!animationsEnabled) {
    // Static fallback - simple gradient border
    return (
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius,
          border: `${thickness}px solid ${color}40`,
          zIndex: -1,
        }}
      />
    );
  }

  return (
    <div
      className="absolute inset-0 pointer-events-none overflow-hidden"
      style={{ borderRadius, zIndex: -1 }}
    >
      {/* Rotating conic gradient */}
      <motion.div
        className="absolute"
        style={{
          inset: -thickness * 2,
          borderRadius: borderRadius + thickness * 2,
          background: `conic-gradient(
            from 0deg,
            transparent 0deg,
            transparent ${180 - beamDegrees / 2}deg,
            ${color}00 ${180 - beamDegrees / 2}deg,
            ${color} ${180}deg,
            ${secondary} ${180}deg,
            ${color}00 ${180 + beamDegrees / 2}deg,
            transparent ${180 + beamDegrees / 2}deg,
            transparent 360deg
          )`,
          filter: `blur(${thickness}px)`,
          opacity: glowIntensity * 0.6,
        }}
        animate={{
          rotate: [0, 360],
        }}
        transition={{
          duration: currentDuration,
          repeat: Infinity,
          ease: 'linear',
        }}
      />

      {/* Inner sharp beam */}
      <motion.div
        className="absolute"
        style={{
          inset: -thickness,
          borderRadius: borderRadius + thickness,
          background: `conic-gradient(
            from 0deg,
            transparent 0deg,
            transparent ${180 - beamDegrees / 4}deg,
            ${color} ${180}deg,
            transparent ${180 + beamDegrees / 4}deg,
            transparent 360deg
          )`,
          opacity: glowIntensity * 0.8,
        }}
        animate={{
          rotate: [0, 360],
        }}
        transition={{
          duration: currentDuration,
          repeat: Infinity,
          ease: 'linear',
        }}
      />

      {/* Inner mask to create border effect */}
      <div
        className="absolute bg-[var(--card-bg,#0a0a0f)]"
        style={{
          inset: thickness,
          borderRadius: borderRadius - thickness,
        }}
      />

      {/* Glow overlay on selection */}
      {selected && (
        <motion.div
          className="absolute inset-0 pointer-events-none"
          style={{
            borderRadius,
            boxShadow: `
              0 0 ${20 * glowIntensity}px ${color}40,
              0 0 ${40 * glowIntensity}px ${color}20,
              inset 0 0 ${20 * glowIntensity}px ${color}10
            `,
          }}
          animate={{
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}
    </div>
  );
});
