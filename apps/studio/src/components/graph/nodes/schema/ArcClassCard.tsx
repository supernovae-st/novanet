'use client';

/**
 * ArcClassCard - "Flow Conduit" premium design for ArcClass nodes
 *
 * Visual Encoding (ADR-005, visual-encoding.yaml):
 * - Primary color (from ArcFamily) → accents, glow, particles
 * - FlowingParticles effect → visualizes arc direction
 * - Premium effects: L-corners, border glow, particle streams
 *
 * Used for: ArcClass schema nodes (relationships between NodeClasses)
 *
 * Layout:
 * ┌─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┐
 * ╎ L─                               ─L ╎  ← Tech corners
 * ╎  → ARC CLASS                        ╎  ← Arrow icon (family color)
 * ╎  ════════════════════════════       ╎  ← Double line
 * ╎  HAS_NATIVE                         ╎  ← Arc name (family glow)
 * ╎  ════════════════════════════       ╎
 * ╎   Entity ──────────→ EntityNative   ╎  ← Source → Target
 * ╎  ════════════════════════════       ╎
 * ╎  family: ownership    cardinality   ╎  ← Metadata badges
 * ╎ L─                               ─L ╎  ← Tech corners
 * └─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─┘
 *     ≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋≋    ← FlowingParticles (below card)
 *
 * @example
 * ```tsx
 * <ArcClassCard
 *   data={{ key: 'ArcClass:HAS_NATIVE', source: 'Entity', target: 'EntityNative', family: 'ownership' }}
 *   selected={true}
 * />
 * ```
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { SPRING_CONFIGS, DURATIONS } from '../card/animationPresets';
import {
  ARC_FAMILY_COLORS,
  type ArcFamilyKey,
} from '@/design/colors/generated';
import {
  GridPattern,
  MotionTechCorners,
  NeonBorderGlow,
  FlowingParticles,
} from '../effects';

// =============================================================================
// Types
// =============================================================================

export interface ArcClassNodeData {
  id: string;
  type: 'ArcClass';
  key: string;
  displayName: string;
  /** Arc family: ownership, localization, semantic, generation, mining */
  family?: string;
  /** Source node type */
  source?: string;
  /** Target node type */
  target?: string;
  /** Cardinality: 1:1, 1:N, N:M */
  cardinality?: string;
  /** Arc scope: intra_realm, cross_realm */
  scope?: string;
}

export interface ArcClassCardProps {
  data: ArcClassNodeData;
  selected?: boolean;
  isHovered?: boolean;
  width?: number;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Constants
// =============================================================================

const DEFAULT_COLOR = '#6c71c4'; // violet (fallback)

// =============================================================================
// Animation Variants
// =============================================================================

const cardVariants: Variants = {
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
    scale: 1.04,
    y: -4,
    transition: SPRING_CONFIGS.smooth,
  },
};

const arrowVariants: Variants = {
  idle: {
    x: 0,
    opacity: 0.7,
  },
  hover: {
    x: 4,
    opacity: 1,
    transition: { duration: DURATIONS.fast },
  },
  selected: {
    x: [0, 8, 0],
    opacity: 1,
    transition: {
      x: { duration: 1.2, repeat: Infinity, ease: 'easeInOut' },
    },
  },
};

const flowLineVariants: Variants = {
  idle: {
    pathLength: 0.6,
    opacity: 0.5,
  },
  hover: {
    pathLength: 1,
    opacity: 0.8,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    pathLength: 1,
    opacity: 1,
    transition: { duration: DURATIONS.normal },
  },
};

// =============================================================================
// Main Component
// =============================================================================

export const ArcClassCard = memo(function ArcClassCard({
  data,
  selected = false,
  isHovered = false,
  width = 280,
  performanceConfig,
}: ArcClassCardProps) {
  // Performance flags
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;
  const showOuterGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showParticles = performanceConfig?.effects?.particles ?? true;
  const showGridPattern = performanceConfig?.effects?.gridPattern ?? true;

  // Extract arc name from key (e.g., "ArcClass:HAS_NATIVE" → "HAS_NATIVE")
  const arcName = useMemo(() => {
    const parts = data.key.split(':');
    return parts.length > 1 ? parts[1] : data.displayName;
  }, [data.key, data.displayName]);

  // Get family color
  const familyColor = useMemo(() => {
    const familyKey = (data.family || 'semantic') as ArcFamilyKey;
    return ARC_FAMILY_COLORS[familyKey]?.color ?? DEFAULT_COLOR;
  }, [data.family]);

  // Animation state
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';
  const intensity = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style for arc name
  const nameGlowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 20px ${familyColor}80, 0 0 40px ${familyColor}40`
        : isHovered
          ? `0 0 12px ${familyColor}60`
          : `0 0 6px ${familyColor}30`,
    }),
    [familyColor, selected, isHovered]
  );

  // Border glow style
  const borderGlowStyle = useMemo(
    () => ({
      boxShadow: selected
        ? `0 0 20px ${familyColor}40, inset 0 0 20px ${familyColor}10`
        : `0 0 8px ${familyColor}20`,
    }),
    [familyColor, selected]
  );

  // Wrapper component - motion or static based on performance
  const CardWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="relative" style={{ width }}>
      <CardWrapper
        className="relative p-4 rounded-xl overflow-hidden"
        style={{
          ...borderGlowStyle,
          background: 'rgba(0, 0, 0, 0.9)',
          border: `2px solid ${selected ? familyColor : `${familyColor}50`}`,
        }}
        {...(animationsEnabled && {
          variants: cardVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Background effects */}
        {showGridPattern && <GridPattern color={familyColor} intensity={intensity} />}

        {/* Tech corners */}
        {showTechCorners && (
          <MotionTechCorners
            color={familyColor}
            selected={selected}
            isHovered={isHovered}
            size={14}
            performanceConfig={performanceConfig}
          />
        )}

        {/* Neon border glow */}
        {showOuterGlow && (selected || isHovered) && (
          <NeonBorderGlow
            color={familyColor}
            selected={selected}
            isHovered={isHovered}
            borderRadius={12}
            performanceConfig={performanceConfig}
          />
        )}

        {/* Content */}
        <div className="relative z-10">
          {/* Header: Arrow + ARC CLASS label */}
          <div className={cn('flex items-center justify-between mb-2', gapTokens.default)}>
            <div className={cn('flex items-center', gapTokens.default)}>
              {animationsEnabled ? (
                <motion.span
                  className="text-base font-bold"
                  style={{
                    color: familyColor,
                    filter: `drop-shadow(0 0 4px ${familyColor})`,
                  }}
                  variants={arrowVariants}
                  initial="idle"
                  animate={animationState}
                >
                  →
                </motion.span>
              ) : (
                <span
                  className="text-base font-bold"
                  style={{
                    color: familyColor,
                    filter: `drop-shadow(0 0 4px ${familyColor})`,
                  }}
                >
                  →
                </span>
              )}
              <span
                className="text-[9px] font-bold uppercase tracking-widest font-mono"
                style={{ color: familyColor }}
              >
                ARC CLASS
              </span>
            </div>

            {/* Family badge */}
            {data.family && (
              <span
                className="px-1.5 py-0.5 rounded text-[8px] font-mono uppercase"
                style={{
                  color: familyColor,
                  backgroundColor: `${familyColor}20`,
                  border: `1px solid ${familyColor}40`,
                }}
              >
                {data.family}
              </span>
            )}
          </div>

          {/* Double line separator */}
          <div className="mb-3">
            <div
              className="h-[2px] mb-[2px]"
              style={{
                background: `linear-gradient(90deg, ${familyColor}60, ${familyColor}20, transparent)`,
              }}
            />
            <div
              className="h-[1px]"
              style={{
                background: `linear-gradient(90deg, ${familyColor}40, transparent)`,
              }}
            />
          </div>

          {/* Arc name - hero element */}
          <h3
            className="text-lg font-bold font-mono text-white mb-4 text-center"
            style={nameGlowStyle}
          >
            {arcName}
          </h3>

          {/* Source → Target visualization */}
          <div
            className="relative p-3 rounded-lg mb-3"
            style={{
              background: `${familyColor}08`,
              border: `1px solid ${familyColor}20`,
            }}
          >
            <div className="flex items-center justify-center gap-3 text-sm font-mono">
              {/* Source node */}
              <span
                className="px-2 py-1 rounded bg-white/5 border border-white/10"
              >
                {data.source || '?'}
              </span>

              {/* Flow arrow with animation */}
              <div className="relative flex-1 flex items-center justify-center">
                {animationsEnabled ? (
                  <svg
                    width="100%"
                    height="20"
                    className="overflow-visible"
                    style={{ maxWidth: 80 }}
                  >
                    <motion.line
                      x1="0"
                      y1="10"
                      x2="70"
                      y2="10"
                      stroke={familyColor}
                      strokeWidth="2"
                      strokeDasharray="4 2"
                      variants={flowLineVariants}
                      initial="idle"
                      animate={animationState}
                    />
                    <motion.polygon
                      points="70,5 80,10 70,15"
                      fill={familyColor}
                      variants={{
                        idle: { opacity: 0.7 },
                        hover: { opacity: 1 },
                        selected: { opacity: 1 },
                      }}
                      initial="idle"
                      animate={animationState}
                    />
                  </svg>
                ) : (
                  <svg
                    width="100%"
                    height="20"
                    style={{ maxWidth: 80 }}
                  >
                    <line
                      x1="0"
                      y1="10"
                      x2="70"
                      y2="10"
                      stroke={familyColor}
                      strokeWidth="2"
                      strokeDasharray="4 2"
                    />
                    <polygon
                      points="70,5 80,10 70,15"
                      fill={familyColor}
                    />
                  </svg>
                )}
              </div>

              {/* Target node */}
              <span
                className="px-2 py-1 rounded bg-white/5 border border-white/10"
              >
                {data.target || '?'}
              </span>
            </div>
          </div>

          {/* Metadata row */}
          <div className="flex justify-between items-center text-[9px] font-mono">
            <div className="flex items-center gap-2">
              <span className="text-white/40">cardinality:</span>
              <span className="text-white/70">{data.cardinality || '?'}</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-white/40">scope:</span>
              <span
                className="px-1 rounded"
                style={{
                  color: data.scope === 'cross_realm' ? '#f59e0b' : '#22c55e',
                  backgroundColor: data.scope === 'cross_realm' ? '#f59e0b20' : '#22c55e20',
                }}
              >
                {data.scope || 'intra'}
              </span>
            </div>
          </div>
        </div>
      </CardWrapper>

      {/* FlowingParticles effect below the card */}
      {showParticles && (selected || isHovered) && (
        <div
          className="absolute left-0 right-0 -bottom-8 h-8 overflow-hidden pointer-events-none"
          style={{ opacity: selected ? 1 : 0.6 }}
        >
          <FlowingParticles
            color={familyColor}
            active={selected || isHovered}
            direction="horizontal"
            particleCount={selected ? 5 : 3}
            performanceConfig={performanceConfig}
          />
        </div>
      )}
    </div>
  );
});
