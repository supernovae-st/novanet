'use client';

/**
 * LiquidGlass - Premium glassmorphism with SVG distortion effect
 *
 * Creates a "liquid glass" effect using SVG turbulence filters
 * that makes the card appear as if viewed through warped glass.
 *
 * Inspired by the Electric Card pattern from 21st.dev.
 *
 * @example
 * ```tsx
 * <LiquidGlass
 *   color="#8b5cf6"
 *   selected={selected}
 * />
 * ```
 */

import { memo, useId } from 'react';
import { motion } from 'motion/react';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface LiquidGlassProps {
  /** Accent color */
  color: string;
  /** Whether the element is selected */
  selected?: boolean;
  /** Whether the element is hovered */
  isHovered?: boolean;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
  /** Border radius to match parent */
  borderRadius?: number;
  /** Enable turbulence distortion (expensive) */
  enableDistortion?: boolean;
}

// =============================================================================
// Component
// =============================================================================

export const LiquidGlass = memo(function LiquidGlass({
  color,
  selected = false,
  isHovered = false,
  performanceConfig,
  borderRadius = 16,
  enableDistortion = true,
}: LiquidGlassProps) {
  const id = useId();
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  // Only enable distortion on ULTRA tier
  const useDistortion = enableDistortion && animationsEnabled;

  return (
    <div
      className="absolute inset-0 pointer-events-none overflow-hidden"
      style={{ borderRadius }}
    >
      {/* SVG filter definitions */}
      {useDistortion && (
        <svg className="absolute w-0 h-0" aria-hidden="true">
          <defs>
            {/* Liquid turbulence filter */}
            <filter
              id={`liquid-${id}`}
              colorInterpolationFilters="sRGB"
              x="-20%"
              y="-20%"
              width="140%"
              height="140%"
            >
              <feTurbulence
                type="turbulence"
                baseFrequency="0.015"
                numOctaves="3"
                result="noise"
                seed="1"
              />
              <feDisplacementMap
                in="SourceGraphic"
                in2="noise"
                scale={selected ? 8 : isHovered ? 5 : 3}
                xChannelSelector="R"
                yChannelSelector="G"
              />
            </filter>

            {/* Glow filter */}
            <filter id={`glow-${id}`} x="-50%" y="-50%" width="200%" height="200%">
              <feGaussianBlur in="SourceGraphic" stdDeviation="4" result="blur" />
              <feComposite in="SourceGraphic" in2="blur" operator="over" />
            </filter>
          </defs>
        </svg>
      )}

      {/* Glassmorphism base layer */}
      <div
        className="absolute inset-0"
        style={{
          borderRadius,
          background: `
            radial-gradient(
              ellipse at 30% 20%,
              rgba(255, 255, 255, 0.15) 0%,
              transparent 50%
            ),
            radial-gradient(
              ellipse at 70% 80%,
              ${color}10 0%,
              transparent 50%
            )
          `,
          backdropFilter: 'blur(8px)',
          WebkitBackdropFilter: 'blur(8px)',
        }}
      />

      {/* Inner glow border */}
      <div
        className="absolute inset-0"
        style={{
          borderRadius,
          border: `1px solid rgba(255, 255, 255, ${selected ? 0.3 : isHovered ? 0.2 : 0.1})`,
          boxShadow: `
            inset 0 1px 0 rgba(255, 255, 255, ${selected ? 0.2 : 0.1}),
            inset 0 -1px 0 rgba(0, 0, 0, 0.1)
          `,
        }}
      />

      {/* Animated light reflection */}
      {animationsEnabled && (
        <motion.div
          className="absolute inset-0 overflow-hidden"
          style={{ borderRadius }}
        >
          <motion.div
            className="absolute w-[150%] h-[200%]"
            style={{
              background: `
                linear-gradient(
                  120deg,
                  transparent 0%,
                  transparent 40%,
                  rgba(255, 255, 255, 0.05) 45%,
                  rgba(255, 255, 255, 0.1) 50%,
                  rgba(255, 255, 255, 0.05) 55%,
                  transparent 60%,
                  transparent 100%
                )
              `,
              transform: 'rotate(-30deg)',
              transformOrigin: 'center center',
            }}
            animate={{
              x: ['-150%', '150%'],
              y: ['-100%', '100%'],
            }}
            transition={{
              duration: selected ? 3 : 6,
              repeat: Infinity,
              ease: 'easeInOut',
              repeatDelay: selected ? 1 : 3,
            }}
          />
        </motion.div>
      )}

      {/* Color accent glow */}
      <motion.div
        className="absolute"
        style={{
          inset: -20,
          borderRadius: borderRadius + 20,
          background: `radial-gradient(ellipse at center, ${color}20, transparent 70%)`,
          filter: useDistortion ? `url(#liquid-${id})` : undefined,
        }}
        animate={
          animationsEnabled
            ? {
                opacity: selected ? [0.6, 0.8, 0.6] : isHovered ? [0.4, 0.5, 0.4] : 0.3,
                scale: selected ? [1, 1.05, 1] : 1,
              }
            : undefined
        }
        transition={{
          duration: 3,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />

      {/* Top reflection highlight */}
      <div
        className="absolute left-2 right-2 h-px top-2"
        style={{
          background: `linear-gradient(
            90deg,
            transparent 0%,
            rgba(255, 255, 255, ${selected ? 0.4 : 0.2}) 30%,
            rgba(255, 255, 255, ${selected ? 0.5 : 0.3}) 50%,
            rgba(255, 255, 255, ${selected ? 0.4 : 0.2}) 70%,
            transparent 100%
          )`,
          borderRadius: 1,
        }}
      />

      {/* Corner accent dots */}
      {selected && (
        <>
          <motion.div
            className="absolute w-1 h-1 rounded-full"
            style={{
              top: 8,
              left: 8,
              backgroundColor: color,
              boxShadow: `0 0 6px ${color}`,
            }}
            animate={{
              scale: [1, 1.5, 1],
              opacity: [0.8, 1, 0.8],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />
          <motion.div
            className="absolute w-1 h-1 rounded-full"
            style={{
              top: 8,
              right: 8,
              backgroundColor: color,
              boxShadow: `0 0 6px ${color}`,
            }}
            animate={{
              scale: [1, 1.5, 1],
              opacity: [0.8, 1, 0.8],
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: 'easeInOut',
              delay: 0.5,
            }}
          />
        </>
      )}
    </div>
  );
});
