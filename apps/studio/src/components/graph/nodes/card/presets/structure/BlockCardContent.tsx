'use client';

/**
 * BlockCardContent - "Block Unit" design for Block nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = structure) → blue #3b82f6
 * - Border style → solid
 * - Shows block type, order position, property count
 *
 * Layout:
 * ┌────────────────────────┐
 * │ 🧱 BLOCK          #1   │
 * │ ════════════════════   │
 * │ homepage:hero:1        │
 * │ ┌────────────────────┐ │
 * │ │ type  HeroBlock    │ │
 * │ │ ┌────────────────┐ │ │ ← Order visual
 * │ │ │ 1 │ 2 │ 3 │ 4 │ │ │
 * │ │ └────────────────┘ │ │
 * │ │ ⊞ 5 props          │ │
 * │ └────────────────────┘ │
 * └────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// import { cn } from '@/lib/utils';
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { BlockOrderIndicator } from './StructureHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { Box, Layers } from 'lucide-react';

// =============================================================================
// Types
// =============================================================================

export interface BlockNodeData {
  id: string;
  type: 'Block';
  key: string;
  displayName: string;
  /** Block type (e.g., HeroBlock, CTABlock) */
  block_type?: string;
  /** Order in parent page (1-indexed) */
  order?: number;
  /** Total blocks in parent page */
  total_blocks?: number;
  /** Number of properties */
  prop_count?: number;
  /** Parent page key */
  page_key?: string;
  /** Number of BlockNative children */
  nativeCount?: number;
}

export interface BlockCardContentProps extends CardContext {
  data: BlockNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const orderVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: {
    opacity: 1,
    scale: [1, 1.02, 1],
    transition: { duration: DURATIONS.normal },
  },
};

const stackVariants: Variants = {
  idle: { y: 0, opacity: 0.3 },
  hover: { y: -2, opacity: 0.5 },
  selected: { y: -3, opacity: 0.6 },
};

// =============================================================================
// Component
// =============================================================================

export const BlockCardContent = memo(function BlockCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BlockCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  const OrderWrapper = animationsEnabled ? motion.div : 'div';
  const StackLayer = animationsEnabled ? motion.div : 'div';
  const hasNatives = typeof data.nativeCount === 'number' && data.nativeCount > 0;

  return (
    <div className="px-4 py-4 relative">
      {/* === STACK EFFECT: Shows BlockNatives behind this Block === */}
      {hasNatives && (
        <>
          {/* Stack layer 2 (furthest back) */}
          <StackLayer
            className="absolute inset-x-2 top-1 bottom-1 rounded-lg -z-20"
            style={{
              background: `${colors.primary}08`,
              border: `1px dashed ${colors.primary}20`,
              transform: 'translateY(6px) scale(0.96)',
            }}
            {...(animationsEnabled && {
              variants: stackVariants,
              initial: 'idle',
              animate: animationState,
            })}
          />
          {/* Stack layer 1 (behind main card) */}
          <StackLayer
            className="absolute inset-x-1 top-0.5 bottom-0.5 rounded-lg -z-10"
            style={{
              background: `${colors.primary}12`,
              border: `1px dashed ${colors.primary}30`,
              transform: 'translateY(3px) scale(0.98)',
            }}
            {...(animationsEnabled && {
              variants: stackVariants,
              initial: 'idle',
              animate: animationState,
            })}
          />
        </>
      )}

      {/* Taxonomy Badge: Layer (structure) + Realm (org) + Class (Block) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="structure"
          realm="org"
          className="Block"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        {/* Right side badges: natives count + order */}
        <div className="flex items-center gap-1.5">
          {/* Natives badge */}
          {hasNatives && (
            <span
              className="flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-mono"
              style={{
                color: '#22c55e',
                backgroundColor: 'rgba(34, 197, 94, 0.15)',
                border: '1px dashed rgba(34, 197, 94, 0.3)',
              }}
            >
              <Layers size={10} />
              {data.nativeCount}
            </span>
          )}
          {/* Order badge */}
          {typeof data.order === 'number' && (
            <span
              className="px-1.5 py-0.5 rounded text-[9px] font-mono font-bold"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}20`,
                border: `1px solid ${colors.primary}40`,
              }}
            >
              #{data.order}
            </span>
          )}
        </div>
      </div>

      {/* Block icon + key */}
      <div className="flex items-center gap-2 mb-3">
        <Box size={16} style={{ color: colors.primary }} />
        <h3
          className="text-base font-bold font-mono text-white truncate"
          style={glowStyle}
        >
          {data.key}
        </h3>
      </div>

      {/* Block info section */}
      <OrderWrapper
        className="p-2 rounded-lg"
        style={{
          background: `${colors.primary}08`,
          border: `1px solid ${colors.primary}20`,
        }}
        {...(animationsEnabled && {
          variants: orderVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        <div className="space-y-2">
          {/* Block type */}
          {data.block_type && (
            <div className="flex items-center justify-between text-[10px]">
              <span className="text-white/50 font-mono">type</span>
              <span
                className="px-1.5 py-0.5 rounded font-mono"
                style={{
                  color: colors.primary,
                  backgroundColor: `${colors.primary}15`,
                }}
              >
                {data.block_type}
              </span>
            </div>
          )}

          {/* Order indicator */}
          {typeof data.order === 'number' && typeof data.total_blocks === 'number' && (
            <div className="flex items-center justify-center py-1">
              <BlockOrderIndicator
                order={data.order}
                total={data.total_blocks}
                color={colors.primary}
                animate={animationsEnabled}
              />
            </div>
          )}

          {/* Property count */}
          {typeof data.prop_count === 'number' && (
            <div className="flex items-center gap-1 text-[9px]">
              <span className="text-white/50">⊞</span>
              <span className="text-white/70">{data.prop_count} props</span>
            </div>
          )}
        </div>
      </OrderWrapper>
    </div>
  );
});
