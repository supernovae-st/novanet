'use client';

/**
 * AuroraBackground - Northern lights effect with animated gradient layers
 *
 * Creates a mesmerizing aurora borealis effect using multiple animated
 * gradient layers that shift and flow organically.
 *
 * TAXONOMY level visual treatment (ADR-005 maximum wow).
 *
 * @example
 * ```tsx
 * <AuroraBackground
 *   primaryColor="#8b5cf6"
 *   secondaryColor="#6366f1"
 *   tertiaryColor="#06b6d4"
 *   selected={selected}
 * />
 * ```
 */

import { memo, useMemo, useId } from 'react';
import { motion } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface AuroraBackgroundProps {
  /** Primary aurora color */
  primaryColor: string;
  /** Secondary aurora color */
  secondaryColor?: string;
  /** Tertiary aurora color */
  tertiaryColor?: string;
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
// Component
// =============================================================================

export const AuroraBackground = memo(function AuroraBackground({
  primaryColor,
  secondaryColor,
  tertiaryColor,
  selected = false,
  isHovered = false,
  performanceConfig,
  intensity = 'medium',
  borderRadius = 16,
}: AuroraBackgroundProps) {
  const id = useId();
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  // Derive secondary/tertiary if not provided
  const secondary = secondaryColor ?? shiftHue(primaryColor, 40);
  const tertiary = tertiaryColor ?? shiftHue(primaryColor, -30);

  // Intensity multipliers
  const intensityConfig = {
    subtle: { opacity: 0.15, blur: 60, scale: 1.2 },
    medium: { opacity: 0.25, blur: 80, scale: 1.4 },
    intense: { opacity: 0.4, blur: 100, scale: 1.6 },
  };
  const config = intensityConfig[intensity];

  // Aurora gradient styles
  const auroraLayers = useMemo(
    () => [
      {
        background: `radial-gradient(ellipse 80% 50% at 50% 0%, ${primaryColor}${Math.round(config.opacity * 255).toString(16).padStart(2, '0')}, transparent 50%)`,
        animationDelay: '0s',
      },
      {
        background: `radial-gradient(ellipse 60% 40% at 70% 20%, ${secondary}${Math.round(config.opacity * 0.8 * 255).toString(16).padStart(2, '0')}, transparent 50%)`,
        animationDelay: '-2s',
      },
      {
        background: `radial-gradient(ellipse 70% 35% at 30% 10%, ${tertiary}${Math.round(config.opacity * 0.6 * 255).toString(16).padStart(2, '0')}, transparent 50%)`,
        animationDelay: '-4s',
      },
    ],
    [primaryColor, secondary, tertiary, config.opacity]
  );

  if (!animationsEnabled) {
    // Static fallback
    return (
      <div
        className="absolute inset-0 pointer-events-none overflow-hidden"
        style={{ borderRadius }}
      >
        <div
          className="absolute inset-[-20%]"
          style={{
            background: `radial-gradient(ellipse 80% 50% at 50% 0%, ${primaryColor}20, transparent 50%)`,
            filter: `blur(${config.blur}px)`,
          }}
        />
      </div>
    );
  }

  return (
    <div
      className="absolute inset-0 pointer-events-none overflow-hidden"
      style={{ borderRadius }}
    >
      {/* SVG filter for enhanced blur/glow */}
      <svg className="absolute w-0 h-0" aria-hidden="true">
        <defs>
          <filter id={`aurora-blur-${id}`} x="-50%" y="-50%" width="200%" height="200%">
            <feGaussianBlur in="SourceGraphic" stdDeviation={config.blur / 4} result="blur" />
            <feColorMatrix
              in="blur"
              type="saturate"
              values={selected ? '1.5' : isHovered ? '1.2' : '1'}
            />
          </filter>
        </defs>
      </svg>

      {/* Aurora layers with staggered animations */}
      {auroraLayers.map((layer, index) => (
        <motion.div
          key={index}
          className="absolute"
          style={{
            inset: `-${20 + index * 10}%`,
            background: layer.background,
            filter: `blur(${config.blur}px) url(#aurora-blur-${id})`,
            transformOrigin: 'center top',
          }}
          animate={{
            scale: [1, config.scale, 1],
            x: [0, index % 2 === 0 ? 30 : -30, 0],
            y: [0, 20, 0],
            opacity: selected
              ? [config.opacity, config.opacity * 1.5, config.opacity]
              : isHovered
                ? [config.opacity * 0.8, config.opacity * 1.2, config.opacity * 0.8]
                : [config.opacity * 0.6, config.opacity, config.opacity * 0.6],
          }}
          transition={{
            duration: 8 + index * 2,
            repeat: Infinity,
            ease: 'easeInOut',
            delay: index * 1.5,
          }}
        />
      ))}

      {/* Shimmer overlay when selected */}
      {selected && (
        <motion.div
          className="absolute inset-0"
          style={{
            background: `linear-gradient(135deg, transparent 30%, ${primaryColor}15 50%, transparent 70%)`,
            backgroundSize: '200% 200%',
            borderRadius,
          }}
          animate={{
            backgroundPosition: ['0% 0%', '100% 100%', '0% 0%'],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'linear',
          }}
        />
      )}
    </div>
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
