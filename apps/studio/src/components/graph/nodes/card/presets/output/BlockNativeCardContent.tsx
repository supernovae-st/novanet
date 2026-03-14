'use client';

/**
 * BlockNativeCardContent - "Generated Block" design for BlockNative nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = output) → green #22c55e
 * - Border style → dotted (output layer)
 * - Shows generated content, locale, anchor slug, status
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ✦ BLOCK NATIVE     🇫🇷 fr-FR   │  ← Sparkle icon + locale badge
 * │ ·····························   │  ← Dotted separator (generated)
 * │ block:pricing-hero@fr-FR        │
 * │ ┌────────────────────────────┐   │
 * │ │ Pricing Hero (fr-FR)       │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ #comparaison-des-fonct...  │   │  ← Anchor slug
 * │ │ ● Published     v2         │   │  ← Status + version
 * │ │ ─────────────────────────  │   │
 * │ │ generated:                 │   │
 * │ │ title: "Des QR Codes..."   │   │  ← Content preview
 * │ │ subtitle: "Créez, pers..." │   │
 * │ │ ─────────────────────────  │   │
 * │ │ generated: Jan 15, 10:25   │   │  ← Generation info
 * │ └────────────────────────────┘   │
 * └──────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// import { cn } from '@/lib/utils';
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  OutputLocaleBadge,
  OutputStatusBadge,
  VersionHistory,
  AssemblyInfo,
  ContentPreview,
  AnchorSlugBadge,
} from './OutputHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { Box, ArrowUpRight } from 'lucide-react';

// =============================================================================
// Types
// =============================================================================

export interface BlockNativeNodeData {
  id: string;
  type: 'BlockNative';
  key: string; // block:{block_key}@{locale_key}
  displayName: string;
  /** Parent Block key (denormalized) */
  block_key: string;
  /** Target Locale key (denormalized) */
  locale_key: string;
  /** Description */
  description?: string;
  /** Generated content as JSON */
  generated?: Record<string, unknown>;
  /** Generation timestamp */
  generated_at?: string;
  /** Generator version */
  generator_version?: string;
  /** Publication status */
  status?: 'draft' | 'approved' | 'published';
  /** Anchor slug */
  anchor_slug?: string;
  /** Version number */
  version?: number;
  /** Published at timestamp */
  published_at?: string;
  /** Has previous version */
  has_previous_version?: boolean;
  /** Influenced by entities count */
  influenced_by_count?: number;
}

export interface BlockNativeCardContentProps extends CardContext {
  data: BlockNativeNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const _sparkleVariants: Variants = {
  idle: { scale: 1 },
  hover: {
    scale: [1, 1.2, 1],
    transition: { duration: DURATIONS.normal, repeat: Infinity },
  },
  selected: {
    scale: 1.2,
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

export const BlockNativeCardContent = memo(function BlockNativeCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: BlockNativeCardContentProps) {
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

  const _SparkleIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (output) + Realm (org) + Class (BlockNative) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="output"
          realm="org"
          className="BlockNative"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <OutputLocaleBadge locale={data.locale_key} color={colors.primary} />
      </div>

      {/* Block key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name */}
      <h4 className="text-base font-bold text-white mb-2" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* === PARENT REFERENCE: Shows this belongs to a Block === */}
      {data.block_key && (
        <div className="flex items-center gap-2 mb-3">
          {/* Dashed connector line */}
          <div
            className="flex-shrink-0 w-5 h-px"
            style={{
              background: `repeating-linear-gradient(90deg, ${colors.primary}40 0, ${colors.primary}40 3px, transparent 3px, transparent 6px)`,
            }}
          />
          {/* Parent badge */}
          <div
            className="flex items-center gap-1.5 px-2 py-0.5 rounded-md"
            style={{
              background: `${colors.primary}08`,
              border: `1px dashed ${colors.primary}25`,
            }}
          >
            <Box size={9} style={{ color: colors.primary }} />
            <span className="text-[8px] font-mono text-white/50">parent:</span>
            <span
              className="text-[8px] font-mono font-medium truncate max-w-[100px]"
              style={{ color: `${colors.primary}90` }}
            >
              {data.block_key}
            </span>
            <ArrowUpRight size={7} className="text-white/30" />
          </div>
        </div>
      )}

      {/* Content section */}
      <ContentWrapper
        className="space-y-2"
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Anchor slug */}
        {data.anchor_slug && (
          <AnchorSlugBadge slug={data.anchor_slug} color={colors.primary} />
        )}

        {/* Status + Version */}
        <div className="flex items-center justify-between">
          {data.status && <OutputStatusBadge status={data.status} />}
          {typeof data.version === 'number' && (
            <VersionHistory
              version={data.version}
              hasPreviousVersion={data.has_previous_version}
              color={colors.primary}
            />
          )}
        </div>

        {/* Description */}
        {data.description && (
          <p className="text-[10px] text-white/60 line-clamp-2 italic">
            {data.description}
          </p>
        )}

        {/* Generated content preview */}
        {data.generated && Object.keys(data.generated).length > 0 && (
          <div className="space-y-1">
            <span className="text-[9px] text-white/50 font-mono">generated:</span>
            <ContentPreview
              content={data.generated}
              maxKeys={3}
              color={colors.primary}
            />
          </div>
        )}

        {/* Generation info */}
        <AssemblyInfo
          timestamp={data.generated_at}
          version={data.generator_version}
          color={colors.primary}
          label="generated"
        />

        {/* Influenced by count */}
        {typeof data.influenced_by_count === 'number' && data.influenced_by_count > 0 && (
          <div className="flex items-center gap-1 text-[8px]">
            <span className="text-white/50">influenced by:</span>
            <span style={{ color: '#f97316' }}>{data.influenced_by_count} entities</span>
          </div>
        )}
      </ContentWrapper>
    </div>
  );
});
