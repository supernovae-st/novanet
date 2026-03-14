'use client';

/**
 * BlockTypeCardContent - "Schema Blueprint" design for BlockType nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = instruction) → yellow #eab308
 * - Border style → solid
 * - Shows JSON schema, category, block_type
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ◧ TYPE           ▲ HEADER       │  ← Grid icon + category badge
 * │ ══════════════════════════════   │
 * │ hero                             │
 * │ ┌────────────────────────────┐   │
 * │ │ Hero Section               │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ schema:                    │   │
 * │ │ *title: string             │   │  ← Schema properties
 * │ │ *description: string       │   │
 * │ │  cta_url: string           │   │
 * │ │  +3 more                   │   │
 * │ └────────────────────────────┘   │
 * └──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// cn reserved for future use
// import { cn } from '@/lib/utils';
// gapTokens reserved for future use
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import { CategoryBadge, SchemaPropertyList } from './InstructionHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface BlockTypeNodeData {
  id: string;
  type: 'BlockType';
  key: string;
  displayName: string;
  /** Block type identifier */
  block_type?: string;
  /** Category: header, body, footer, sidebar */
  category?: 'header' | 'body' | 'footer' | 'sidebar';
  /** Description */
  description?: string;
  /** JSON schema for block content */
  schema?: Record<string, { type: string; required?: boolean }>;
  /** Number of blocks using this type */
  usage_count?: number;
}

export interface BlockTypeCardContentProps extends CardContext {
  data: BlockTypeNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const _gridVariants: Variants = {
  idle: { scale: 1 },
  hover: {
    scale: 1.1,
    transition: { duration: DURATIONS.fast },
  },
  selected: {
    scale: 1.15,
    filter: 'drop-shadow(0 0 6px currentColor)',
  },
};

const schemaVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const BlockTypeCardContent = memo(function BlockTypeCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BlockTypeCardContentProps) {
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

  const _GridIcon = animationsEnabled ? motion.span : 'span';
  const SchemaWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (instruction) + Realm (org) + Class (BlockType) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="instruction"
          realm="org"
          className="BlockType"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        {data.category && (
          <CategoryBadge category={data.category} color={colors.primary} />
        )}
      </div>

      {/* Block type key */}
      <h3 className="text-base font-bold font-mono text-white mb-1" style={glowStyle}>
        {data.block_type || data.key}
      </h3>

      {/* Display name (if different from key) */}
      {data.displayName && data.displayName !== data.key && (
        <p className="text-sm text-white/80 mb-2">{data.displayName}</p>
      )}

      {/* Description */}
      {data.description && (
        <p className="text-[10px] text-white/60 line-clamp-2 italic mb-3">
          {data.description}
        </p>
      )}

      {/* Schema section */}
      <SchemaWrapper
        {...(animationsEnabled && {
          variants: schemaVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {data.schema && Object.keys(data.schema).length > 0 && (
          <SchemaPropertyList
            properties={data.schema}
            maxDisplay={4}
            color={colors.primary}
          />
        )}

        {/* Usage count */}
        {typeof data.usage_count === 'number' && data.usage_count > 0 && (
          <div className="flex items-center gap-1 text-[9px] mt-2">
            <span className="text-white/50">used by:</span>
            <span
              className="px-1.5 py-0.5 rounded font-mono font-bold"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {data.usage_count} blocks
            </span>
          </div>
        )}
      </SchemaWrapper>
    </div>
  );
});
