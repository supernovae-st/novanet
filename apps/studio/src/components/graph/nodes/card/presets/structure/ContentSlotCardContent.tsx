'use client';

/**
 * ContentSlotCardContent - "Placeholder" design for ContentSlot nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = structure) → blue #3b82f6
 * - Border style → dashed (placeholder feel)
 * - Shows allowed block types as pills
 *
 * Layout:
 * ┌┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┐
 * │ ┌ ─ ─ ┐ SLOT                │  ← Dashed border + interior dashed box
 * │         ═══════════════     │    (visual "empty placeholder" feel)
 * │ │     │ sidebar-cta         │
 * │         ┌────────────────┐  │
 * │ │     │ │ allowed: [CTA] │  │  ← Allowed block types as pills
 * │         └────────────────┘  │
 * │ └ ─ ─ ┘                     │
 * └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface ContentSlotNodeData {
  id: string;
  type: 'ContentSlot';
  key: string;
  displayName: string;
  /** Allowed block types for this slot */
  allowed_types?: string[];
  /** Is this slot required? */
  required?: boolean;
  /** Description of slot purpose */
  description?: string;
}

export interface ContentSlotCardContentProps extends CardContext {
  data: ContentSlotNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const placeholderVariants: Variants = {
  idle: {
    opacity: 0.6,
    borderColor: 'rgba(255,255,255,0.2)',
  },
  hover: {
    opacity: 0.9,
    borderColor: 'rgba(255,255,255,0.4)',
    transition: { duration: DURATIONS.fast },
  },
  selected: {
    opacity: 1,
    borderColor: 'rgba(255,255,255,0.6)',
  },
};

const pulseVariants: Variants = {
  idle: { opacity: 0.3 },
  selected: {
    opacity: [0.3, 0.6, 0.3],
    transition: { duration: 2, repeat: Infinity },
  },
};

// =============================================================================
// Component
// =============================================================================

export const ContentSlotCardContent = memo(function ContentSlotCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: ContentSlotCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style (subtle for placeholder)
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 8px ${colors.primary}40`
      : 'none',
  }), [colors.primary, selected]);

  const PlaceholderBox = animationsEnabled ? motion.div : 'div';
  const PulseOverlay = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (structure) + Realm (org) + Trait (defined) + Class (ContentSlot) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="structure"
          realm="org"
          trait="defined"
          className="ContentSlot"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        {/* Required badge */}
        {data.required && (
          <span
            className="px-1.5 py-0.5 rounded text-[8px] font-mono"
            style={{
              color: '#ef4444',
              backgroundColor: 'rgba(239, 68, 68, 0.15)',
              border: '1px solid rgba(239, 68, 68, 0.3)',
            }}
          >
            required
          </span>
        )}
      </div>

      {/* Slot name */}
      <h3
        className="text-base font-bold font-mono text-white/80 mb-3"
        style={glowStyle}
      >
        {data.key}
      </h3>

      {/* Placeholder visualization */}
      <PlaceholderBox
        className="relative p-3 rounded-lg border-2 border-dashed"
        style={{
          background: `${colors.primary}05`,
        }}
        {...(animationsEnabled && {
          variants: placeholderVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Pulse overlay for selected state */}
        {selected && (
          <PulseOverlay
            className="absolute inset-0 rounded-lg"
            style={{
              background: `radial-gradient(circle at center, ${colors.primary}20 0%, transparent 70%)`,
            }}
            {...(animationsEnabled && {
              variants: pulseVariants,
              initial: 'idle',
              animate: animationState,
            })}
          />
        )}

        {/* Inner dashed box (placeholder feel) */}
        <div className="relative">
          <div
            className="w-8 h-8 border-2 border-dashed rounded mb-2"
            style={{
              borderColor: `${colors.primary}30`,
            }}
          />

          {/* Allowed types */}
          {data.allowed_types && data.allowed_types.length > 0 && (
            <div className="mt-2">
              <span className="text-[9px] text-white/40 font-mono">allowed:</span>
              <div className="flex flex-wrap gap-1 mt-1">
                {data.allowed_types.map((type, idx) => (
                  <span
                    key={idx}
                    className="px-1.5 py-0.5 rounded text-[8px] font-mono"
                    style={{
                      color: colors.primary,
                      backgroundColor: `${colors.primary}15`,
                      border: `1px dashed ${colors.primary}30`,
                    }}
                  >
                    {type}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      </PlaceholderBox>

      {/* Description */}
      {data.description && (
        <p className="mt-2 text-[9px] text-white/40 italic">
          {data.description}
        </p>
      )}
    </div>
  );
});
