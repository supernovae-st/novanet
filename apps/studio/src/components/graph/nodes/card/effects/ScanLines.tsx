'use client';

/**
 * ScanLines - CRT/Hologram scan line effect
 *
 * Creates horizontal scan lines with optional moving scan beam
 * for a retro-futuristic holographic display aesthetic.
 *
 * Performance Requirements:
 * - MEDIUM+ tier (CSS-only with optional animation)
 * - Lightweight SVG pattern
 */

import { memo, useId } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface ScanLinesProps {
  /** Line spacing in pixels */
  spacing?: number;
  /** Line thickness in pixels */
  thickness?: number;
  /** Line color */
  color?: string;
  /** Base opacity */
  opacity?: number;
  /** Show animated scan beam */
  showScanBeam?: boolean;
  /** Scan beam color (default: brighter version of color) */
  scanBeamColor?: string;
  /** Scan beam animation duration in seconds */
  scanDuration?: number;
  /** Direction of scan beam: 'down' | 'up' */
  direction?: 'down' | 'up';
  /** Flicker effect (CRT authenticity) */
  flicker?: boolean;
  /** Selected state - intensifies effect */
  selected?: boolean;
  /** Hovered state */
  isHovered?: boolean;
  /** Additional CSS classes */
  className?: string;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const scanBeamVariants: Variants = {
  animate: (direction: 'down' | 'up') => ({
    y: direction === 'down' ? ['0%', '100%'] : ['100%', '0%'],
    opacity: [0, 0.8, 0.8, 0],
    transition: {
      y: {
        duration: 3,
        repeat: Infinity,
        ease: 'linear',
      },
      opacity: {
        duration: 3,
        repeat: Infinity,
        ease: 'linear',
        times: [0, 0.1, 0.9, 1],
      },
    },
  }),
};

const flickerVariants: Variants = {
  animate: {
    opacity: [1, 0.95, 1, 0.98, 1, 0.96, 1],
    transition: {
      duration: 0.15,
      repeat: Infinity,
      repeatDelay: 2 + Math.random() * 3,
    },
  },
};

// =============================================================================
// Component
// =============================================================================

export const ScanLines = memo(function ScanLines({
  spacing = 3,
  thickness = 1,
  color = 'rgba(255, 255, 255, 0.03)',
  opacity = 1,
  showScanBeam = true,
  scanBeamColor,
  scanDuration: _scanDuration = 3,
  direction = 'down',
  flicker = true,
  selected = false,
  isHovered: _isHovered = false,
  className,
  performanceConfig,
}: ScanLinesProps) {
  const id = useId();
  const patternId = `scanlines-${id}`;

  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const effectsEnabled = performanceConfig?.effects?.premiumEffects ?? true;

  if (!effectsEnabled) return null;

  const effectiveOpacity = selected ? opacity * 1.2 : opacity;
  const effectiveScanBeamColor = scanBeamColor ?? 'rgba(255, 255, 255, 0.15)';

  const Wrapper = animationsEnabled && flicker ? motion.div : 'div';
  const ScanBeam = animationsEnabled ? motion.div : 'div';

  return (
    <Wrapper
      aria-hidden="true"
      className={cn(
        'pointer-events-none absolute inset-0 overflow-hidden',
        className
      )}
      style={{ opacity: effectiveOpacity }}
      {...(animationsEnabled && flicker && {
        variants: flickerVariants,
        animate: 'animate',
      })}
    >
      {/* SVG Pattern for scan lines */}
      <svg
        className="absolute inset-0 w-full h-full"
        style={{ mixBlendMode: 'overlay' }}
      >
        <defs>
          <pattern
            id={patternId}
            width={1}
            height={spacing + thickness}
            patternUnits="userSpaceOnUse"
          >
            <rect
              x={0}
              y={0}
              width={1}
              height={thickness}
              fill={color}
            />
          </pattern>
        </defs>
        <rect
          width="100%"
          height="100%"
          fill={`url(#${patternId})`}
        />
      </svg>

      {/* Animated scan beam */}
      {showScanBeam && animationsEnabled && (
        <ScanBeam
          className="absolute left-0 right-0 pointer-events-none"
          style={{
            height: 60,
            background: `linear-gradient(${direction === 'down' ? '180deg' : '0deg'},
              ${effectiveScanBeamColor}00 0%,
              ${effectiveScanBeamColor} 40%,
              ${effectiveScanBeamColor} 60%,
              ${effectiveScanBeamColor}00 100%
            )`,
            boxShadow: `0 0 40px ${effectiveScanBeamColor}`,
          }}
          variants={scanBeamVariants}
          animate="animate"
          custom={direction}
        />
      )}

      {/* CRT vignette effect */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          background: `radial-gradient(ellipse at center,
            transparent 0%,
            transparent 50%,
            rgba(0, 0, 0, 0.15) 100%
          )`,
        }}
      />

      {/* Corner darkening for CRT curve simulation */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          background: `
            radial-gradient(ellipse at top left, rgba(0,0,0,0.1) 0%, transparent 40%),
            radial-gradient(ellipse at top right, rgba(0,0,0,0.1) 0%, transparent 40%),
            radial-gradient(ellipse at bottom left, rgba(0,0,0,0.1) 0%, transparent 40%),
            radial-gradient(ellipse at bottom right, rgba(0,0,0,0.1) 0%, transparent 40%)
          `,
        }}
      />
    </Wrapper>
  );
});
