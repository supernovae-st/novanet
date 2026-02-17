'use client';

/**
 * HolographicOverlay - Prismatic color-shifting shimmer effect
 *
 * Creates a holographic/iridescent overlay that shifts colors
 * as if light is reflecting off a holographic surface.
 *
 * Perfect for TAXONOMY level nodes to create maximum "wow" factor.
 *
 * @example
 * ```tsx
 * <HolographicOverlay
 *   baseColor="#8b5cf6"
 *   selected={selected}
 *   isHovered={isHovered}
 * />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface HolographicOverlayProps {
  /** Base color for the holographic effect */
  baseColor?: string;
  /** Whether the element is selected */
  selected?: boolean;
  /** Whether the element is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Intensity: subtle, medium, intense */
  intensity?: 'subtle' | 'medium' | 'intense';
  /** Border radius to match parent */
  borderRadius?: number;
}

// =============================================================================
// Constants
// =============================================================================

// Holographic color stops (rainbow spectrum)
const HOLO_COLORS = [
  'rgba(255, 0, 128, 0.15)',   // Pink
  'rgba(128, 0, 255, 0.15)',   // Purple
  'rgba(0, 128, 255, 0.15)',   // Blue
  'rgba(0, 255, 128, 0.15)',   // Cyan
  'rgba(128, 255, 0, 0.15)',   // Green
  'rgba(255, 128, 0, 0.15)',   // Orange
  'rgba(255, 0, 128, 0.15)',   // Pink (loop)
];

// =============================================================================
// Component
// =============================================================================

export const HolographicOverlay = memo(function HolographicOverlay({
  baseColor = '#8b5cf6',
  selected = false,
  isHovered = false,
  performanceConfig,
  intensity = 'medium',
  borderRadius = 16,
}: HolographicOverlayProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  // Intensity multipliers
  const intensityConfig = {
    subtle: { opacity: 0.08, saturation: 0.8 },
    medium: { opacity: 0.15, saturation: 1 },
    intense: { opacity: 0.25, saturation: 1.3 },
  };
  const config = intensityConfig[intensity];

  // Current opacity based on state
  const currentOpacity = selected
    ? config.opacity * 1.5
    : isHovered
      ? config.opacity * 1.2
      : config.opacity;

  // Gradient with color stops
  const holographicGradient = useMemo(
    () => `linear-gradient(
      135deg,
      ${HOLO_COLORS.join(', ')}
    )`,
    []
  );

  if (!animationsEnabled) {
    // Static fallback
    return (
      <div
        className="absolute inset-0 pointer-events-none mix-blend-overlay"
        style={{
          borderRadius,
          background: holographicGradient,
          backgroundSize: '400% 400%',
          opacity: currentOpacity,
        }}
      />
    );
  }

  return (
    <>
      {/* Main holographic layer */}
      <motion.div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius,
          background: holographicGradient,
          backgroundSize: '400% 400%',
          mixBlendMode: 'overlay',
        }}
        animate={{
          backgroundPosition: ['0% 0%', '100% 100%', '0% 0%'],
          opacity: [currentOpacity, currentOpacity * 1.2, currentOpacity],
        }}
        transition={{
          duration: selected ? 4 : 8,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Prismatic shimmer sweep */}
      <motion.div
        className="absolute inset-0 pointer-events-none overflow-hidden"
        style={{ borderRadius }}
      >
        <motion.div
          className="absolute w-[200%] h-full"
          style={{
            background: `linear-gradient(
              90deg,
              transparent 0%,
              rgba(255, 255, 255, 0.1) 10%,
              rgba(255, 255, 255, 0.2) 20%,
              rgba(255, 255, 255, 0.3) 30%,
              rgba(255, 255, 255, 0.2) 40%,
              rgba(255, 255, 255, 0.1) 50%,
              transparent 60%,
              transparent 100%
            )`,
            transform: 'skewX(-20deg)',
          }}
          animate={{
            x: ['-100%', '100%'],
          }}
          transition={{
            duration: selected ? 2 : isHovered ? 3 : 5,
            repeat: Infinity,
            ease: 'easeInOut',
            repeatDelay: selected ? 0.5 : isHovered ? 1 : 2,
          }}
        />
      </motion.div>

      {/* Color shift layer (selected only) */}
      {selected && (
        <motion.div
          className="absolute inset-0 pointer-events-none"
          style={{
            borderRadius,
            mixBlendMode: 'color',
          }}
          animate={{
            background: [
              `linear-gradient(45deg, ${baseColor}30, transparent)`,
              `linear-gradient(135deg, ${shiftHue(baseColor, 60)}30, transparent)`,
              `linear-gradient(225deg, ${shiftHue(baseColor, 120)}30, transparent)`,
              `linear-gradient(315deg, ${shiftHue(baseColor, 180)}30, transparent)`,
              `linear-gradient(45deg, ${baseColor}30, transparent)`,
            ],
          }}
          transition={{
            duration: 6,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />
      )}
    </>
  );
});

// =============================================================================
// Utilities
// =============================================================================

/**
 * Shift hue of a hex color
 */
function shiftHue(hex: string, degrees: number): string {
  const r = parseInt(hex.slice(1, 3), 16) / 255;
  const g = parseInt(hex.slice(3, 5), 16) / 255;
  const b = parseInt(hex.slice(5, 7), 16) / 255;

  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h = 0;
  const s = max === 0 ? 0 : (max - min) / max;
  const v = max;

  if (max !== min) {
    const d = max - min;
    switch (max) {
      case r:
        h = (g - b) / d + (g < b ? 6 : 0);
        break;
      case g:
        h = (b - r) / d + 2;
        break;
      case b:
        h = (r - g) / d + 4;
        break;
    }
    h /= 6;
  }

  // Shift hue
  h = (h + degrees / 360 + 1) % 1;

  // HSV to RGB
  const i = Math.floor(h * 6);
  const f = h * 6 - i;
  const p = v * (1 - s);
  const q = v * (1 - f * s);
  const t = v * (1 - (1 - f) * s);

  let r2: number, g2: number, b2: number;
  switch (i % 6) {
    case 0:
      r2 = v; g2 = t; b2 = p;
      break;
    case 1:
      r2 = q; g2 = v; b2 = p;
      break;
    case 2:
      r2 = p; g2 = v; b2 = t;
      break;
    case 3:
      r2 = p; g2 = q; b2 = v;
      break;
    case 4:
      r2 = t; g2 = p; b2 = v;
      break;
    default:
      r2 = v; g2 = p; b2 = q;
      break;
  }

  return `#${Math.round(r2 * 255).toString(16).padStart(2, '0')}${Math.round(g2 * 255).toString(16).padStart(2, '0')}${Math.round(b2 * 255).toString(16).padStart(2, '0')}`;
}
