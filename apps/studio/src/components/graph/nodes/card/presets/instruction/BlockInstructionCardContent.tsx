'use client';

/**
 * BlockInstructionCardContent - "Code Editor" design for BlockInstruction nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = instruction) → yellow #eab308
 * - Border style → solid (defined trait)
 * - Shows markdown content with @entity: and @link: references
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ⚡ INSTRUCTION        v2 ●      │  ← Bolt icon + version badge
 * │ ══════════════════════════════   │
 * │ homepage:hero:instruction        │
 * │ ┌────────────────────────────┐   │
 * │ │ [GENERATE] title           │   │  ← Syntax-highlighted content
 * │ │ Use @entity:qr-generator   │   │
 * │ │ Link to @link:pricing      │   │
 * │ │ ... +3 more lines          │   │
 * │ ├────────────────────────────┤   │
 * │ │ @entity: 2  |  @link: 1    │   │  ← Reference counts
 * │ │ 💡concepts 🗣️voice        │   │  ← Inclusion flags
 * │ └────────────────────────────┘   │
 * └──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  VersionBadge,
  ContentPreview,
  ReferenceCounter,
  InclusionFlags,
} from './InstructionHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface BlockInstructionNodeData {
  id: string;
  type: 'BlockInstruction';
  key: string; // {page_key}:{block_key}:instruction
  displayName: string;
  /** Block key reference */
  block_key: string;
  /** Markdown content with @ references */
  content?: string;
  /** Version number */
  version?: number;
  /** Active flag */
  is_active?: boolean;
  /** Include concepts in generation */
  include_concepts?: boolean;
  /** Include voice settings */
  include_voice?: boolean;
  /** Include culture context */
  include_culture?: boolean;
  /** Entity reference count (computed) */
  entity_refs?: number;
  /** Link reference count (computed) */
  link_refs?: number;
}

export interface BlockInstructionCardContentProps extends CardContext {
  data: BlockInstructionNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const boltVariants: Variants = {
  idle: { scale: 1, rotate: 0 },
  hover: {
    scale: 1.1,
    rotate: [-5, 5, -5, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.15,
    filter: 'drop-shadow(0 0 8px currentColor)',
  },
};

const contentVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const BlockInstructionCardContent = memo(function BlockInstructionCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BlockInstructionCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Glow style
  const glowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 12px ${colors.primary}60`
        : isHovered
          ? `0 0 8px ${colors.primary}40`
          : 'none',
    }),
    [colors.primary, selected, isHovered]
  );

  const BoltIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (instruction) + Realm (org) + Trait (defined) + Class (BlockInstruction) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="instruction"
          realm="org"
          trait="defined"
          className="BlockInstruction"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <VersionBadge
          version={data.version || 1}
          isActive={data.is_active !== false}
          color={colors.primary}
        />
      </div>

      {/* Instruction key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name */}
      <h4 className="text-base font-bold text-white mb-3" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* Content section */}
      <ContentWrapper
        className="space-y-2"
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Content preview */}
        {data.content && (
          <ContentPreview
            content={data.content}
            maxLines={4}
            color={colors.primary}
          />
        )}

        {/* Reference counts */}
        {(data.entity_refs || data.link_refs) && (
          <ReferenceCounter
            entityRefs={data.entity_refs}
            linkRefs={data.link_refs}
            color={colors.primary}
          />
        )}

        {/* Inclusion flags */}
        <InclusionFlags
          includeConcepts={data.include_concepts}
          includeVoice={data.include_voice}
          includeCulture={data.include_culture}
          color={colors.primary}
        />
      </ContentWrapper>
    </div>
  );
});
