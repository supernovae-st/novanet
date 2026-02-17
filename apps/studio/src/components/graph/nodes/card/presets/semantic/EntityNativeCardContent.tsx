'use client';

/**
 * EntityNativeCardContent - "Native Content" design for EntityNative nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = semantic) → orange #f97316
 * - Border style → dashed (authored trait)
 * - Shows locale, curation status, content stats
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ● NATIVE          🇫🇷 fr-FR     │  ← Native icon + locale badge
 * │ ══════════════════════════════   │
 * │ entity:qr-code@fr-FR             │
 * │ ┌────────────────────────────┐   │
 * │ │ Générateur de QR Code      │   │  ← Localized display name
 * │ │ ─────────────────────────  │   │
 * │ │ ✍️ Human  |  ● Published   │   │  ← Curation + Status
 * │ │ ─────────────────────────  │   │
 * │ │ benefits:                  │   │
 * │ │ ✓ Création rapide          │   │  ← Benefits preview
 * │ │ ✓ Gratuit                  │   │
 * │ │ ▼ +2 more                  │   │
 * │ │ ─────────────────────────  │   │
 * │ │ ✓ 4  📝 2         v1       │   │  ← Content stats
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
  LocaleBadge,
  CurationBadge,
  StatusBadge,
  BenefitsList,
  ContentStats,
} from './SemanticHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface EntityNativeNodeData {
  id: string;
  type: 'EntityNative';
  key: string; // entity:{entity_key}@{locale}
  displayName: string;
  /** Parent entity key */
  entity_key: string;
  /** Locale key */
  locale_key: string;
  /** Localized description */
  description?: string;
  /** Content curation status */
  curation_status?: 'human_authored' | 'machine_translated' | 'ai_generated' | 'ai_generated_reviewed';
  /** Publication status */
  status?: 'draft' | 'reviewed' | 'published';
  /** Localized definition */
  definition?: string;
  /** Value propositions */
  benefits?: string[];
  /** Usage examples */
  usage_examples?: string[];
  /** Content version */
  version?: number;
  /** Audience segment */
  audience_segment?: string;
  /** Cultural notes */
  cultural_notes?: string;
}

export interface EntityNativeCardContentProps extends CardContext {
  data: EntityNativeNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const pulseVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: {
    opacity: [0.8, 1, 0.8],
    transition: { duration: 2, repeat: Infinity },
  },
};

const contentVariants: Variants = {
  idle: { opacity: 0.9 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const EntityNativeCardContent = memo(function EntityNativeCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: EntityNativeCardContentProps) {
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

  const NativeIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (semantic) + Realm (org) + Trait (authored) + Class (EntityNative) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="semantic"
          realm="org"
          trait="authored"
          className="EntityNative"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <LocaleBadge locale={data.locale_key} color={colors.primary} />
      </div>

      {/* Composite key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name (localized) */}
      <h4 className="text-base font-bold text-white mb-3" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* Content section */}
      <ContentWrapper
        className="p-2 rounded-lg space-y-2"
        style={{
          background: `${colors.primary}08`,
          border: `1px dashed ${colors.primary}30`,
        }}
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Description (truncated) */}
        {data.description && (
          <p className="text-[10px] text-white/60 line-clamp-2 italic">{data.description}</p>
        )}

        {/* Curation + Status badges */}
        <div className="flex items-center gap-2 flex-wrap">
          {data.curation_status && (
            <CurationBadge status={data.curation_status} animate={animationsEnabled} />
          )}
          {data.status && <StatusBadge status={data.status} />}
        </div>

        {/* Audience segment */}
        {data.audience_segment && (
          <div className="flex items-center gap-1 text-[9px]">
            <span className="text-white/50">audience:</span>
            <span
              className="px-1.5 py-0.5 rounded font-mono"
              style={{
                color: colors.primary,
                backgroundColor: `${colors.primary}15`,
              }}
            >
              {data.audience_segment}
            </span>
          </div>
        )}

        {/* Benefits list */}
        {data.benefits && data.benefits.length > 0 && (
          <BenefitsList
            benefits={data.benefits}
            maxDisplay={2}
            color={colors.primary}
            animate={animationsEnabled}
          />
        )}

        {/* Cultural notes */}
        {data.cultural_notes && (
          <div className="text-[9px] text-white/50 italic border-l-2 pl-2" style={{ borderColor: `${colors.primary}40` }}>
            🌍 {data.cultural_notes}
          </div>
        )}

        {/* Content stats */}
        <ContentStats
          benefitsCount={data.benefits?.length || 0}
          examplesCount={data.usage_examples?.length || 0}
          version={data.version || 1}
          color={colors.primary}
        />
      </ContentWrapper>
    </div>
  );
});
