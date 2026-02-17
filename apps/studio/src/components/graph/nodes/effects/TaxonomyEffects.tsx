'use client';

/**
 * TaxonomyEffects - Visual effects for Level 1 (Taxonomy) nodes
 *
 * Each taxonomy variant has unique visual effects:
 * - Realm: Orbital rings (rotating decorative circles)
 * - Layer: Stacked planes (parallax depth effect)
 * - Trait: Border morphing (animated border style changes)
 * - ArcFamily: Radiating pulse (expanding ripple effect)
 *
 * All effects are performance-aware and use Framer Motion.
 *
 * @example
 * ```tsx
 * <OrbitalRings color={realmColor} selected={selected} />
 * <StackedPlanes color={layerColor} depth={3} />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { SPRING_CONFIGS, DURATIONS } from '../card/animationPresets';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';

// =============================================================================
// Types
// =============================================================================

export interface OrbitalRingsProps {
  /** Primary color for rings */
  color: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Number of orbital rings (default: 2) */
  ringCount?: number;
  /** Container size in pixels */
  size?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

export interface StackedPlanesProps {
  /** Primary color for planes */
  color: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Number of stacked layers (default: 3) */
  depth?: number;
  /** Container width */
  width?: number;
  /** Container height */
  height?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

export interface BorderMorphProps {
  /** Primary color for border */
  color: string;
  /** Current border style */
  borderStyle: 'solid' | 'dashed' | 'dotted' | 'double';
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** Border radius */
  borderRadius?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

export interface RadiatingPulseProps {
  /** Primary color for pulse */
  color: string;
  /** Whether the pulse is active */
  active: boolean;
  /** Number of pulse rings (default: 3) */
  pulseCount?: number;
  /** Container size */
  size?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// OrbitalRings - For Realm nodes
// =============================================================================

/**
 * Orbital rings that rotate around the node
 *
 * Visual metaphor: Realms are "universes" with orbiting elements
 */
export const OrbitalRings = memo(function OrbitalRings({
  color,
  selected,
  isHovered = false,
  ringCount = 2,
  size = 200,
  performanceConfig,
}: OrbitalRingsProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  const rings = useMemo(
    () =>
      Array.from({ length: ringCount }, (_, i) => ({
        index: i,
        radius: (size / 2) * (0.7 + i * 0.15),
        duration: 20 + i * 5,
        direction: i % 2 === 0 ? 1 : -1,
        opacity: selected ? 0.6 - i * 0.1 : isHovered ? 0.4 - i * 0.1 : 0.2 - i * 0.05,
      })),
    [ringCount, size, selected, isHovered]
  );

  // Static fallback
  if (!animationsEnabled) {
    return (
      <div className="absolute inset-0 pointer-events-none overflow-visible">
        {rings.map((ring) => (
          <div
            key={ring.index}
            className="absolute rounded-full border"
            style={{
              width: ring.radius * 2,
              height: ring.radius * 2,
              left: '50%',
              top: '50%',
              transform: 'translate(-50%, -50%)',
              borderColor: color,
              opacity: ring.opacity,
              borderStyle: 'dashed',
            }}
          />
        ))}
      </div>
    );
  }

  return (
    <div className="absolute inset-0 pointer-events-none overflow-visible">
      {rings.map((ring) => (
        <motion.div
          key={ring.index}
          className="absolute rounded-full border border-dashed"
          style={{
            width: ring.radius * 2,
            height: ring.radius * 2,
            left: '50%',
            top: '50%',
            borderColor: color,
          }}
          initial={{ rotate: 0, x: '-50%', y: '-50%', opacity: 0 }}
          animate={{
            rotate: ring.direction * 360,
            x: '-50%',
            y: '-50%',
            opacity: ring.opacity,
          }}
          transition={{
            rotate: {
              duration: ring.duration,
              repeat: Infinity,
              ease: 'linear',
            },
            opacity: {
              duration: DURATIONS.normal,
            },
          }}
        >
          {/* Small decorative dots on the orbit */}
          <motion.div
            className="absolute w-2 h-2 rounded-full"
            style={{
              backgroundColor: color,
              top: -4,
              left: '50%',
              marginLeft: -4,
            }}
            animate={{
              scale: selected ? [1, 1.3, 1] : 1,
            }}
            transition={{
              duration: 1.5,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />
        </motion.div>
      ))}
    </div>
  );
});

// =============================================================================
// StackedPlanes - For Layer nodes
// =============================================================================

/**
 * Stacked translucent planes creating a depth effect
 *
 * Visual metaphor: Layers are "strata" of the architecture
 */
export const StackedPlanes = memo(function StackedPlanes({
  color,
  selected,
  isHovered = false,
  depth = 3,
  width = 200,
  height = 80,
  performanceConfig,
}: StackedPlanesProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const useSpring = performanceConfig?.animation?.spring ?? true;

  const planes = useMemo(
    () =>
      Array.from({ length: depth }, (_, i) => ({
        index: i,
        offset: i * 4,
        opacity: 0.15 - i * 0.03,
        scale: 1 - i * 0.02,
      })),
    [depth]
  );

  const planeVariants: Variants = useMemo(
    () => ({
      idle: (i: number) => ({
        y: i * 4,
        scale: 1 - i * 0.02,
        opacity: 0.15 - i * 0.03,
      }),
      hover: (i: number) => ({
        y: i * 6,
        scale: 1 - i * 0.015,
        opacity: 0.2 - i * 0.03,
        transition: useSpring ? SPRING_CONFIGS.gentle : { duration: DURATIONS.fast },
      }),
      selected: (i: number) => ({
        y: i * 8,
        scale: 1 - i * 0.01,
        opacity: 0.25 - i * 0.03,
        transition: useSpring ? SPRING_CONFIGS.smooth : { duration: DURATIONS.fast },
      }),
    }),
    [useSpring]
  );

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Static fallback
  if (!animationsEnabled) {
    return (
      <div className="absolute inset-0 pointer-events-none">
        {planes.map((plane) => (
          <div
            key={plane.index}
            className="absolute rounded-lg"
            style={{
              width,
              height,
              left: 0,
              top: plane.offset,
              backgroundColor: color,
              opacity: plane.opacity,
              transform: `scale(${plane.scale})`,
            }}
          />
        ))}
      </div>
    );
  }

  return (
    <div className="absolute inset-0 pointer-events-none">
      {planes.map((plane) => (
        <motion.div
          key={plane.index}
          custom={plane.index}
          className="absolute rounded-lg"
          style={{
            width,
            height,
            left: 0,
            backgroundColor: color,
          }}
          variants={planeVariants}
          initial="idle"
          animate={animationState}
        />
      ))}
    </div>
  );
});

// =============================================================================
// BorderMorph - For Trait nodes
// =============================================================================

/**
 * Animated border that morphs between styles
 *
 * Visual metaphor: Traits change the "character" of nodes
 */
export const BorderMorph = memo(function BorderMorph({
  color,
  borderStyle,
  selected,
  isHovered = false,
  borderRadius = 12,
  performanceConfig,
}: BorderMorphProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const useSpring = performanceConfig?.animation?.spring ?? true;

  // Map border style to dash array values
  const dashArrayMap: Record<string, string> = {
    solid: '1000',
    dashed: '8 4',
    dotted: '2 4',
    double: '1000',
  };

  const dashArray = dashArrayMap[borderStyle] ?? '1000';
  const strokeWidth = borderStyle === 'double' ? 4 : 2;

  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  const pathVariants: Variants = useMemo(
    () => ({
      idle: {
        pathLength: 0.85,
        opacity: 0.5,
      },
      hover: {
        pathLength: 1,
        opacity: 0.7,
        transition: useSpring ? SPRING_CONFIGS.snappy : { duration: DURATIONS.fast },
      },
      selected: {
        pathLength: 1,
        opacity: 1,
        transition: useSpring ? SPRING_CONFIGS.smooth : { duration: DURATIONS.fast },
      },
    }),
    [useSpring]
  );

  // Static fallback
  if (!animationsEnabled) {
    return (
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius,
          border: `${strokeWidth}px ${borderStyle} ${color}`,
          opacity: selected ? 1 : isHovered ? 0.7 : 0.5,
        }}
      />
    );
  }

  return (
    <svg
      className="absolute inset-0 w-full h-full pointer-events-none"
      style={{ overflow: 'visible' }}
    >
      <motion.rect
        x={strokeWidth / 2}
        y={strokeWidth / 2}
        width={`calc(100% - ${strokeWidth}px)`}
        height={`calc(100% - ${strokeWidth}px)`}
        rx={borderRadius}
        ry={borderRadius}
        fill="none"
        stroke={color}
        strokeWidth={strokeWidth}
        strokeDasharray={dashArray}
        variants={pathVariants}
        initial="idle"
        animate={animationState}
      />
      {/* Double border inner line */}
      {borderStyle === 'double' && (
        <motion.rect
          x={strokeWidth / 2 + 3}
          y={strokeWidth / 2 + 3}
          width={`calc(100% - ${strokeWidth + 6}px)`}
          height={`calc(100% - ${strokeWidth + 6}px)`}
          rx={borderRadius - 3}
          ry={borderRadius - 3}
          fill="none"
          stroke={color}
          strokeWidth={1}
          variants={pathVariants}
          initial="idle"
          animate={animationState}
        />
      )}
    </svg>
  );
});

// =============================================================================
// RadiatingPulse - For ArcFamily nodes
// =============================================================================

/**
 * Radiating pulse rings that expand outward
 *
 * Visual metaphor: ArcFamilies are "connections" that radiate influence
 */
export const RadiatingPulse = memo(function RadiatingPulse({
  color,
  active,
  pulseCount = 3,
  size = 100,
  performanceConfig,
}: RadiatingPulseProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;

  if (!active || !animationsEnabled) {
    return null;
  }

  const pulses = Array.from({ length: pulseCount }, (_, i) => i);

  return (
    <div
      className="absolute pointer-events-none"
      style={{
        width: size,
        height: size,
        left: '50%',
        top: '50%',
        transform: 'translate(-50%, -50%)',
      }}
    >
      {pulses.map((i) => (
        <motion.div
          key={i}
          className="absolute inset-0 rounded-full border-2"
          style={{
            borderColor: color,
          }}
          initial={{ scale: 0.5, opacity: 0.8 }}
          animate={{
            scale: [0.5, 1.5],
            opacity: [0.8, 0],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            delay: i * 0.6,
            ease: 'easeOut',
          }}
        />
      ))}
    </div>
  );
});

// =============================================================================
// Combined TaxonomyEffects Component
// =============================================================================

export type TaxonomyVariant = 'realm' | 'layer' | 'trait' | 'arcFamily';

export interface TaxonomyEffectsProps {
  /** Taxonomy variant determines which effect to show */
  variant: TaxonomyVariant;
  /** Primary color */
  color: string;
  /** Whether the node is selected */
  selected: boolean;
  /** Whether the node is hovered */
  isHovered?: boolean;
  /** For trait variant: border style */
  borderStyle?: 'solid' | 'dashed' | 'dotted' | 'double';
  /** Container dimensions */
  width?: number;
  height?: number;
  /** Performance configuration */
  performanceConfig?: PerformanceConfig;
}

/**
 * Unified taxonomy effects component that renders the appropriate effect
 * based on the variant prop.
 */
export const TaxonomyEffects = memo(function TaxonomyEffects({
  variant,
  color,
  selected,
  isHovered = false,
  borderStyle = 'solid',
  width = 200,
  height = 100,
  performanceConfig,
}: TaxonomyEffectsProps) {
  switch (variant) {
    case 'realm':
      return (
        <OrbitalRings
          color={color}
          selected={selected}
          isHovered={isHovered}
          size={Math.max(width, height)}
          performanceConfig={performanceConfig}
        />
      );

    case 'layer':
      return (
        <StackedPlanes
          color={color}
          selected={selected}
          isHovered={isHovered}
          width={width}
          height={height}
          performanceConfig={performanceConfig}
        />
      );

    case 'trait':
      return (
        <BorderMorph
          color={color}
          borderStyle={borderStyle}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      );

    case 'arcFamily':
      return (
        <RadiatingPulse
          color={color}
          active={selected || isHovered}
          size={Math.max(width, height)}
          performanceConfig={performanceConfig}
        />
      );

    default:
      return null;
  }
});
