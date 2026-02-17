'use client';

/**
 * MotionEffects - Framer Motion powered visual effects
 *
 * Performance-aware effect components that use Framer Motion for
 * GPU-accelerated animations with spring physics.
 *
 * Components:
 * - MotionTechCorners: L-shape corner decorations with path drawing
 * - NeonBorderGlow: Animated neon glow border
 * - FlowingParticles: Particle flow effect for arcs
 *
 * @example
 * ```tsx
 * <MotionTechCorners
 *   color={primaryColor}
 *   selected={selected}
 *   performanceConfig={config}
 * />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { SPRING_CONFIGS, DURATIONS } from '../card/animationPresets';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface MotionTechCornersProps {
  /** Primary color for corners */
  color: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Corner decoration size in pixels (default: 16) */
  size?: number;
  /** Performance configuration - disables animation on low tiers */
  performanceConfig?: PerformanceConfig;
}

export interface NeonBorderGlowProps {
  /** Primary color for glow */
  color: string;
  /** Secondary color for gradient */
  secondaryColor?: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Border radius in pixels */
  borderRadius?: number;
  /** Glow intensity multiplier (default: 1) */
  intensity?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

export interface FlowingParticlesProps {
  /** Primary color for particles */
  color: string;
  /** Whether particles are active */
  active: boolean;
  /** Direction of flow: 'horizontal' | 'vertical' */
  direction?: 'horizontal' | 'vertical';
  /** Particle count (default: 3) */
  particleCount?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// MotionTechCorners
// =============================================================================

/**
 * Enhanced L-shape tech corners with Framer Motion animations
 *
 * Features:
 * - SVG pathLength animation on hover/select
 * - Spring-based opacity transitions
 * - Pulsing dot effect on selection
 * - Performance-aware: CSS fallback on LOW/MINIMAL tiers
 */
export const MotionTechCorners = memo(function MotionTechCorners({
  color,
  selected,
  isHovered = false,
  size = 16,
  performanceConfig,
}: MotionTechCornersProps) {
  // Determine if animations should be enabled
  const animationsEnabled = performanceConfig?.effects?.techCorners ?? true;
  const useSpring = performanceConfig?.animation?.spring ?? true;

  // Animation state
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Opacity based on state
  const baseOpacity = selected ? 0.95 : isHovered ? 0.75 : 0.5;

  // Corner path: L-shape
  const cornerPath = `M0 ${size}L0 0L${size} 0`;

  // Corner positions and transforms
  const corners = useMemo(
    () => [
      { position: { top: 8, left: 8 }, transform: 'none' },
      { position: { top: 8, right: 8 }, transform: 'scaleX(-1)' },
      { position: { bottom: 8, left: 8 }, transform: 'scaleY(-1)' },
      { position: { bottom: 8, right: 8 }, transform: 'scale(-1)' },
    ],
    []
  );

  // Custom variants for corners
  const cornerVariants: Variants = useMemo(
    () => ({
      idle: {
        opacity: 0.5,
        pathLength: 0.8,
      },
      hover: {
        opacity: 0.75,
        pathLength: 1,
        transition: useSpring ? SPRING_CONFIGS.snappy : { duration: DURATIONS.fast },
      },
      selected: {
        opacity: 0.95,
        pathLength: 1,
        transition: useSpring ? SPRING_CONFIGS.smooth : { duration: DURATIONS.fast },
      },
    }),
    [useSpring]
  );

  // Dot pulse variants
  const dotVariants: Variants = useMemo(
    () => ({
      idle: { scale: 1, opacity: 0.6 },
      hover: { scale: 1.2, opacity: 0.8 },
      selected: {
        scale: [1, 1.4, 1],
        opacity: 1,
        transition: {
          scale: {
            duration: 1.2,
            repeat: Infinity,
            ease: 'easeInOut',
          },
        },
      },
    }),
    []
  );

  // If animations disabled, render static version
  if (!animationsEnabled) {
    return (
      <>
        {corners.map((corner, index) => (
          <div
            key={index}
            className="absolute pointer-events-none z-20"
            style={{ ...corner.position, color, opacity: baseOpacity, transform: corner.transform }}
          >
            <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
              <path d={cornerPath} stroke="currentColor" strokeWidth="1.5" />
              <circle cx="0" cy="0" r="2" fill="currentColor" />
            </svg>
          </div>
        ))}
      </>
    );
  }

  return (
    <>
      {corners.map((corner, index) => (
        <motion.div
          key={index}
          className="absolute pointer-events-none z-20"
          style={{ ...corner.position, color, transform: corner.transform }}
          initial="idle"
          animate={animationState}
        >
          <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} fill="none">
            <motion.path
              d={cornerPath}
              stroke="currentColor"
              strokeWidth="1.5"
              variants={cornerVariants}
              initial="idle"
              animate={animationState}
            />
            <motion.circle
              cx="0"
              cy="0"
              r="2"
              fill="currentColor"
              variants={dotVariants}
              initial="idle"
              animate={animationState}
            />
          </svg>
        </motion.div>
      ))}
    </>
  );
});

// =============================================================================
// NeonBorderGlow
// =============================================================================

/**
 * Animated neon glow border effect
 *
 * Features:
 * - Multi-layer glow with blur
 * - Gradient animation on selection
 * - Spring-based intensity transitions
 * - Performance-aware
 */
export const NeonBorderGlow = memo(function NeonBorderGlow({
  color,
  secondaryColor,
  selected,
  isHovered = false,
  borderRadius = 16,
  intensity = 1,
  performanceConfig,
}: NeonBorderGlowProps) {
  const animationsEnabled = performanceConfig?.effects?.outerGlow ?? true;
  const useSpring = performanceConfig?.animation?.spring ?? true;

  const secondary = secondaryColor ?? color;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Create CSS custom property for color
  const colorRgb = useMemo(() => {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(color);
    return result
      ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
      : '42, 161, 152';
  }, [color]);

  // Glow intensity values
  const glowValues = useMemo(
    () => ({
      idle: {
        boxShadow: `0 0 ${8 * intensity}px rgba(${colorRgb}, 0.2)`,
        opacity: 0.4,
      },
      hover: {
        boxShadow: `0 0 ${15 * intensity}px rgba(${colorRgb}, 0.3), 0 0 ${30 * intensity}px rgba(${colorRgb}, 0.15)`,
        opacity: 0.6,
      },
      selected: {
        boxShadow: `0 0 ${25 * intensity}px rgba(${colorRgb}, 0.5), 0 0 ${50 * intensity}px rgba(${colorRgb}, 0.3), 0 0 ${80 * intensity}px rgba(${colorRgb}, 0.15)`,
        opacity: 1,
      },
    }),
    [colorRgb, intensity]
  );

  const glowVariants: Variants = useMemo(
    () => ({
      idle: glowValues.idle,
      hover: {
        ...glowValues.hover,
        transition: useSpring ? SPRING_CONFIGS.gentle : { duration: DURATIONS.normal },
      },
      selected: {
        ...glowValues.selected,
        transition: useSpring ? SPRING_CONFIGS.smooth : { duration: DURATIONS.normal },
      },
    }),
    [glowValues, useSpring]
  );

  // Static fallback
  if (!animationsEnabled) {
    const currentGlow = glowValues[animationState];
    return (
      <div
        className="absolute -inset-1 pointer-events-none z-0 transition-all duration-300"
        style={{
          borderRadius: borderRadius + 4,
          boxShadow: currentGlow.boxShadow,
          opacity: currentGlow.opacity,
        }}
      />
    );
  }

  return (
    <>
      {/* Base glow layer */}
      <motion.div
        className="absolute -inset-1 pointer-events-none z-0"
        style={{ borderRadius: borderRadius + 4 }}
        variants={glowVariants}
        initial="idle"
        animate={animationState}
      />

      {/* Gradient rotation layer (selected only) */}
      {selected && (
        <motion.div
          className="absolute -inset-0.5 pointer-events-none z-0"
          style={{
            borderRadius: borderRadius + 2,
            background: `conic-gradient(from 0deg, ${color}, ${secondary}, ${color})`,
            opacity: 0.3,
          }}
          animate={{ rotate: 360 }}
          transition={{
            duration: 8,
            repeat: Infinity,
            ease: 'linear',
          }}
        />
      )}
    </>
  );
});

// =============================================================================
// FlowingParticles
// =============================================================================

/**
 * Flowing particle effect for arc connections
 *
 * Features:
 * - Particles flow along a path
 * - Staggered animation for natural look
 * - Direction control (horizontal/vertical)
 * - Performance-aware
 */
export const FlowingParticles = memo(function FlowingParticles({
  color,
  active,
  direction = 'horizontal',
  particleCount = 3,
  performanceConfig,
}: FlowingParticlesProps) {
  const animationsEnabled = performanceConfig?.effects?.particles ?? true;

  if (!active || !animationsEnabled) {
    return null;
  }

  const particles = Array.from({ length: particleCount }, (_, i) => i);

  const isHorizontal = direction === 'horizontal';
  const containerStyle = isHorizontal
    ? { height: 2, width: '100%' }
    : { width: 2, height: '100%' };

  return (
    <div
      className="absolute pointer-events-none overflow-hidden"
      style={{
        ...containerStyle,
        left: isHorizontal ? 0 : '50%',
        top: isHorizontal ? '50%' : 0,
        transform: isHorizontal ? 'translateY(-50%)' : 'translateX(-50%)',
      }}
    >
      {particles.map((i) => (
        <motion.div
          key={i}
          className="absolute rounded-full"
          style={{
            width: 4,
            height: 4,
            backgroundColor: color,
            boxShadow: `0 0 6px ${color}, 0 0 12px ${color}`,
            left: isHorizontal ? '-4px' : '50%',
            top: isHorizontal ? '50%' : '-4px',
            transform: isHorizontal ? 'translateY(-50%)' : 'translateX(-50%)',
          }}
          animate={
            isHorizontal
              ? { x: ['0%', 'calc(100vw + 4px)'], opacity: [0, 1, 1, 0] }
              : { y: ['0%', 'calc(100vh + 4px)'], opacity: [0, 1, 1, 0] }
          }
          transition={{
            duration: 2,
            repeat: Infinity,
            delay: i * 0.5,
            ease: 'linear',
          }}
        />
      ))}
    </div>
  );
});
