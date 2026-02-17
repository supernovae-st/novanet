'use client';

/**
 * SchemaBadgeNode - Premium Glassmorphism Badge for Realm & Layer
 *
 * Visual Encoding (ADR-005, visual-encoding.yaml):
 * - Primary color → from Realm (shared=cyan, org=violet) or Layer
 * - Glassmorphism background with blur
 * - Glowing border on hover/select
 * - Compact pill shape with icon + label
 * - Animated pulse for active states
 * - Shimmer effect on hover
 *
 * Layout:
 * ┌─────────────────────────────────────────────┐
 * │  [Icon]  REALM: SHARED          40 types   │
 * │          ═══════════════                   │
 * │          Universal locale knowledge        │
 * └─────────────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { LayerIcon, RealmIcon } from '@/components/ui/CategoryIcon';
import type { Realm, Layer } from '@novanet/core/types';
import { REALM_COLORS, LAYER_COLORS } from '@/design/colors/generated';
import { SPRING_CONFIGS, DURATIONS } from '../nodes/card/animationPresets';

// =============================================================================
// Types
// =============================================================================

export interface SchemaBadgeNodeData extends Record<string, unknown> {
  label: string;
  description: string;
  metaType: 'realm' | 'layer';
  color: string;
  typeCount?: number;
  realmKey?: Realm;
  layerKey?: Layer;
}

export type SchemaBadgeNodeType = Node<SchemaBadgeNodeData, 'schemaBadge'>;

// =============================================================================
// Helpers
// =============================================================================

/** Convert hex to RGB string for rgba usage */
const hexToRgb = (hex: string): string => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}`
    : '42, 161, 152';
};

// =============================================================================
// Animation Variants
// =============================================================================

const containerVariants: Variants = {
  idle: {
    scale: 1,
    y: 0,
  },
  hover: {
    scale: 1.02,
    y: -2,
    transition: SPRING_CONFIGS.gentle,
  },
  selected: {
    scale: 1.03,
    y: -3,
    transition: SPRING_CONFIGS.smooth,
  },
};

const iconContainerVariants: Variants = {
  idle: {
    scale: 1,
    rotate: 0,
  },
  hover: {
    scale: 1.1,
    rotate: [0, -5, 5, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.15,
    transition: SPRING_CONFIGS.snappy,
  },
};

const pulseVariants: Variants = {
  idle: {
    scale: 1,
    opacity: 0,
  },
  selected: {
    scale: [1, 1.5, 1],
    opacity: [0.6, 0, 0.6],
    transition: {
      duration: 2,
      ease: 'easeInOut',
      repeat: Infinity,
    },
  },
};

const shimmerVariants: Variants = {
  idle: {
    x: '-100%',
    opacity: 0,
  },
  hover: {
    x: '100%',
    opacity: 1,
    transition: {
      duration: 1.5,
      ease: 'easeInOut',
    },
  },
};

const glowRingVariants: Variants = {
  idle: {
    opacity: 0,
    scale: 0.95,
  },
  hover: {
    opacity: 0.5,
    scale: 1,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    opacity: 1,
    scale: 1.02,
    transition: SPRING_CONFIGS.gentle,
  },
};

// =============================================================================
// Subcomponents
// =============================================================================

/**
 * Premium glassmorphism background with subtle gradient
 */
const GlassBackground = memo(function GlassBackground({
  color,
  selected,
  isHovered,
}: {
  color: string;
  selected: boolean;
  isHovered: boolean;
}) {
  const rgb = hexToRgb(color);

  return (
    <div
      className="absolute inset-0 rounded-2xl transition-all duration-300"
      style={{
        background: `linear-gradient(
          135deg,
          rgba(${rgb}, ${selected ? 0.15 : isHovered ? 0.1 : 0.08}) 0%,
          rgba(${rgb}, ${selected ? 0.08 : isHovered ? 0.05 : 0.03}) 50%,
          rgba(0, 0, 0, 0.4) 100%
        )`,
        backdropFilter: 'blur(16px)',
        WebkitBackdropFilter: 'blur(16px)',
      }}
    />
  );
});

/**
 * Animated glow ring effect for selection
 */
const GlowRing = memo(function GlowRing({
  color,
  animationState,
}: {
  color: string;
  animationState: 'idle' | 'hover' | 'selected';
}) {
  return (
    <motion.div
      className="absolute -inset-1 rounded-[20px] pointer-events-none"
      variants={glowRingVariants}
      initial="idle"
      animate={animationState}
      style={{
        background: `radial-gradient(ellipse at center, ${color}40 0%, transparent 70%)`,
        filter: 'blur(8px)',
      }}
    />
  );
});

/**
 * Selection pulse rings
 */
const SelectionPulse = memo(function SelectionPulse({
  color,
  active,
}: {
  color: string;
  active: boolean;
}) {
  if (!active) return null;

  return (
    <>
      <motion.div
        className="absolute -inset-2 rounded-[24px] pointer-events-none"
        variants={pulseVariants}
        initial="idle"
        animate="selected"
        style={{
          border: `2px solid ${color}`,
        }}
      />
      <motion.div
        className="absolute -inset-3 rounded-[28px] pointer-events-none"
        variants={pulseVariants}
        initial="idle"
        animate="selected"
        style={{
          border: `1px solid ${color}`,
          animationDelay: '0.5s',
        }}
      />
    </>
  );
});

/**
 * Shimmer overlay effect on hover
 */
const ShimmerOverlay = memo(function ShimmerOverlay({
  color,
  animationState,
}: {
  color: string;
  animationState: 'idle' | 'hover' | 'selected';
}) {
  const rgb = hexToRgb(color);

  return (
    <div className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none">
      <motion.div
        className="absolute inset-0"
        variants={shimmerVariants}
        initial="idle"
        animate={animationState === 'hover' ? 'hover' : 'idle'}
        style={{
          background: `linear-gradient(
            90deg,
            transparent 0%,
            rgba(${rgb}, 0.3) 50%,
            transparent 100%
          )`,
          width: '50%',
        }}
      />
    </div>
  );
});

/**
 * Status indicator dot with pulse
 */
const StatusDot = memo(function StatusDot({
  color,
  selected,
}: {
  color: string;
  selected: boolean;
}) {
  return (
    <div className="relative">
      <div
        className={cn(
          'w-2 h-2 rounded-full transition-all duration-300',
          selected && 'animate-pulse'
        )}
        style={{
          background: color,
          boxShadow: `0 0 ${selected ? '12px' : '6px'} ${color}`,
        }}
      />
    </div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const SchemaBadgeNode = memo(function SchemaBadgeNode({
  data,
  selected = false,
}: NodeProps<SchemaBadgeNodeType>) {
  const { metaType, label, description, typeCount = 0, realmKey = 'shared', layerKey = 'foundation' } = data;

  // Get design system color based on meta type
  const isRealm = metaType === 'realm';
  const primaryColor = isRealm
    ? REALM_COLORS[realmKey]?.color || '#2aa198'
    : LAYER_COLORS[layerKey]?.color || '#64748b';

  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
  } = useNodeInteractions({ selected });

  // Animation state
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style for border
  const borderGlowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `0 0 30px ${primaryColor}40, 0 0 60px ${primaryColor}20, inset 0 1px 0 rgba(255,255,255,0.1)`
        : isHovered
          ? `0 0 20px ${primaryColor}30, inset 0 1px 0 rgba(255,255,255,0.08)`
          : `0 0 10px ${primaryColor}15, inset 0 1px 0 rgba(255,255,255,0.05)`,
    }),
    [primaryColor, selected, isHovered]
  );

  // Label text glow
  const labelGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 20px ${primaryColor}80, 0 0 40px ${primaryColor}40`
        : isHovered
          ? `0 0 12px ${primaryColor}50`
          : undefined,
    }),
    [primaryColor, selected, isHovered]
  );

  return (
    <motion.div
      className="relative"
      style={{ width: 280 }}
      variants={containerVariants}
      initial="idle"
      animate={animationState}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`${metaType} node: ${label}`}
    >
      {/* Hidden handles for edges */}
      <Handle type="target" position={Position.Left} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Right} className="!opacity-0 !w-1 !h-1" />

      {/* Outer glow ring */}
      <GlowRing color={primaryColor} animationState={animationState} />

      {/* Selection pulse rings */}
      <SelectionPulse color={primaryColor} active={selected} />

      {/* Main container - Premium Pill Shape */}
      <div
        className="relative overflow-hidden rounded-2xl transition-all duration-300"
        style={{
          background: 'rgba(13, 13, 18, 0.95)',
          border: `1.5px solid ${selected ? primaryColor : isHovered ? `${primaryColor}80` : `${primaryColor}40`}`,
          ...borderGlowStyle,
        }}
      >
        {/* Glassmorphism background */}
        <GlassBackground color={primaryColor} selected={selected} isHovered={isHovered} />

        {/* Shimmer effect on hover */}
        <ShimmerOverlay color={primaryColor} animationState={animationState} />

        {/* Content */}
        <div className="relative z-10 px-4 py-3">
          {/* Top row: Icon + Type badge + Count */}
          <div className="flex items-center gap-3 mb-2">
            {/* Icon container with glow */}
            <motion.div
              className="flex items-center justify-center w-10 h-10 rounded-xl transition-all duration-300"
              variants={iconContainerVariants}
              initial="idle"
              animate={animationState}
              style={{
                background: `linear-gradient(135deg, ${primaryColor}25, ${primaryColor}10)`,
                border: `1px solid ${primaryColor}50`,
                boxShadow: `0 0 ${selected ? '16px' : '8px'} ${primaryColor}30`,
              }}
            >
              {isRealm ? (
                <RealmIcon
                  realm={realmKey}
                  size={22}
                  strokeWidth={1.5}
                  style={{
                    color: primaryColor,
                    filter: `drop-shadow(0 0 ${selected ? '8px' : '4px'} ${primaryColor})`,
                  }}
                />
              ) : (
                <LayerIcon
                  layer={layerKey}
                  size={22}
                  strokeWidth={1.5}
                  style={{
                    color: primaryColor,
                    filter: `drop-shadow(0 0 ${selected ? '8px' : '4px'} ${primaryColor})`,
                  }}
                />
              )}
            </motion.div>

            {/* Type + Value labels */}
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2">
                {/* Type badge pill */}
                <span
                  className="inline-flex items-center gap-1 text-[9px] font-bold uppercase tracking-widest px-2 py-0.5 rounded-full font-mono"
                  style={{
                    background: `${primaryColor}20`,
                    color: primaryColor,
                    border: `1px solid ${primaryColor}35`,
                  }}
                >
                  {isRealm ? 'REALM' : 'LAYER'}
                </span>

                {/* Value pill */}
                <span
                  className="text-[10px] font-semibold uppercase tracking-wide px-2 py-0.5 rounded-full font-mono"
                  style={{
                    background: `${primaryColor}15`,
                    color: `${primaryColor}ee`,
                    border: `1px solid ${primaryColor}25`,
                  }}
                >
                  {isRealm ? realmKey : layerKey}
                </span>
              </div>
            </div>

            {/* Count badge + Status dot */}
            <div className="flex items-center gap-2">
              <span
                className="text-[10px] font-mono font-medium tabular-nums px-2 py-0.5 rounded"
                style={{
                  background: `${primaryColor}10`,
                  color: `${primaryColor}cc`,
                }}
              >
                {typeCount}
              </span>
              <StatusDot color={primaryColor} selected={selected} />
            </div>
          </div>

          {/* Gradient separator line */}
          <div
            className="h-px mb-2.5"
            style={{
              background: `linear-gradient(90deg, ${primaryColor}50, ${primaryColor}20, transparent)`,
            }}
          />

          {/* Label - hero element */}
          <h3
            className="text-lg font-bold text-white mb-0.5 transition-all duration-200"
            style={labelGlowStyle}
          >
            {label}
          </h3>

          {/* Description - muted subtitle */}
          <p
            className="text-[11px] leading-tight line-clamp-2"
            style={{ color: `${primaryColor}99` }}
          >
            {description || (isRealm ? `${typeCount} node types in this realm` : `${typeCount} types in this layer`)}
          </p>
        </div>
      </div>
    </motion.div>
  );
});
