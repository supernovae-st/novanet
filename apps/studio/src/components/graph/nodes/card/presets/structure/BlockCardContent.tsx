'use client';

/**
 * BlockCardContent - "Block Unit" design for Block nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = structure) → blue #3b82f6
 * - Border style → solid (defined trait)
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
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { BlockOrderIndicator, StatCounter } from './StructureHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

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

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (structure) + Realm (org) + Trait (defined) + Class (Block) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="structure"
          realm="org"
          trait="defined"
          className="Block"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
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

      {/* Block key */}
      <h3
        className="text-base font-bold font-mono text-white mb-3 truncate"
        style={glowStyle}
      >
        {data.key}
      </h3>

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
