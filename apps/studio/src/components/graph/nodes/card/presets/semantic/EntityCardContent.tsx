'use client';

/**
 * EntityCardContent - "Semantic Diamond" design for Entity nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = semantic) → orange #f97316
 * - Border style → solid (defined trait)
 * - Shows pillar status, schema.org type, semantic links
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ◆ ENTITY          ★ PILLAR      │  ← Diamond icon + pillar badge
 * │ ══════════════════════════════   │
 * │ qr-code-generator                │
 * │ ┌────────────────────────────┐   │
 * │ │ QR Code Generator          │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ schema:SoftwareApplication │   │  ← Schema.org type
 * │ │                            │   │
 * │ │ → 5  |  ← 12               │   │  ← Semantic link counts
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
import { PillarBadge, SchemaOrgBadge, SemanticLinkCounter } from './SemanticHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface EntityNodeData {
  id: string;
  type: 'Entity';
  key: string;
  displayName: string;
  /** Entity description */
  description?: string;
  /** Is this a pillar entity? */
  is_pillar?: boolean;
  /** Schema.org type for SEO */
  schema_org_type?: string;
  /** Entity category key */
  category_key?: string;
  /** Semantic link counts */
  semanticLinks?: {
    incoming?: number;
    outgoing?: number;
  };
  /** Content locales count */
  localeCount?: number;
}

export interface EntityCardContentProps extends CardContext {
  data: EntityNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const diamondVariants: Variants = {
  idle: {
    rotate: 0,
    scale: 1,
  },
  hover: {
    rotate: 45,
    scale: 1.1,
    transition: { duration: DURATIONS.normal, ease: 'easeOut' },
  },
  selected: {
    rotate: 45,
    scale: 1.15,
    transition: { duration: DURATIONS.normal },
  },
};

const infoVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const EntityCardContent = memo(function EntityCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: EntityCardContentProps) {
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

  const DiamondIcon = animationsEnabled ? motion.span : 'span';
  const InfoWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (semantic) + Realm (org) + Trait (defined) + Class (Entity) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="semantic"
          realm="org"
          trait="defined"
          className="Entity"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <PillarBadge isPillar={data.is_pillar || false} color={colors.primary} />
      </div>

      {/* Entity key */}
      <h3 className="text-base font-bold font-mono text-white mb-1" style={glowStyle}>
        {data.key}
      </h3>

      {/* Display name (if different from key) */}
      {data.displayName && data.displayName !== data.key && (
        <p className="text-sm text-white/80 mb-3">{data.displayName}</p>
      )}

      {/* Info section */}
      <InfoWrapper
        className="p-2 rounded-lg space-y-2"
        style={{
          background: `${colors.primary}08`,
          border: `1px solid ${colors.primary}20`,
        }}
        {...(animationsEnabled && {
          variants: infoVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Description (truncated) */}
        {data.description && (
          <p className="text-[10px] text-white/60 line-clamp-2 italic">{data.description}</p>
        )}

        {/* Schema.org type */}
        {data.schema_org_type && (
          <div className="flex items-center gap-2">
            <SchemaOrgBadge type={data.schema_org_type} color={colors.primary} />
          </div>
        )}

        {/* Category if present */}
        {data.category_key && (
          <div className="flex items-center gap-1 text-[9px]">
            <span className="text-white/50">category:</span>
            <span
              className="px-1.5 py-0.5 rounded font-mono"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {data.category_key}
            </span>
          </div>
        )}

        {/* Semantic link counts */}
        {data.semanticLinks && (
          <SemanticLinkCounter
            incoming={data.semanticLinks.incoming}
            outgoing={data.semanticLinks.outgoing}
            color={colors.primary}
          />
        )}

        {/* Locale count */}
        {typeof data.localeCount === 'number' && data.localeCount > 0 && (
          <div className="flex items-center gap-1 text-[9px]">
            <span className="text-white/50">🌐</span>
            <span className="text-white/70">{data.localeCount} locales</span>
          </div>
        )}
      </InfoWrapper>
    </div>
  );
});
