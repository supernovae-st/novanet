'use client';

/**
 * PageNativeCardContent - "Assembled Page" design for PageNative nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = output) → green #22c55e
 * - Border style → dotted (generated trait)
 * - Shows assembled content, locale, status, version
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ★ PAGE NATIVE      🇫🇷 fr-FR   │  ← Star icon + locale badge
 * │ ·····························   │  ← Dotted separator (generated)
 * │ page:pricing@fr-FR              │
 * │ ┌────────────────────────────┐   │
 * │ │ Page Tarifs (fr-FR)        │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ ● Published     v3         │   │  ← Status + version
 * │ │ ─────────────────────────  │   │
 * │ │ assembled:                 │   │
 * │ │ headers: {...}             │   │  ← Content preview
 * │ │ sections: [3]              │   │
 * │ │ ─────────────────────────  │   │
 * │ │ assembled: Jan 15, 10:30   │   │  ← Assembly info
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
  OutputLocaleBadge,
  OutputStatusBadge,
  VersionHistory,
  AssemblyInfo,
  ContentPreview,
} from './OutputHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface PageNativeNodeData {
  id: string;
  type: 'PageNative';
  key: string; // page:{page_key}@{locale_key}
  displayName: string;
  /** Parent Page key (denormalized) */
  page_key: string;
  /** Target Locale key (denormalized) */
  locale_key: string;
  /** Description */
  description?: string;
  /** Assembled content as JSON */
  assembled?: Record<string, unknown>;
  /** Assembly timestamp */
  assembled_at?: string;
  /** Assembler version */
  assembler_version?: string;
  /** Publication status */
  status?: 'draft' | 'published' | 'archived';
  /** Version number */
  version?: number;
  /** Published at timestamp */
  published_at?: string;
  /** Has previous version */
  has_previous_version?: boolean;
}

export interface PageNativeCardContentProps extends CardContext {
  data: PageNativeNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const starVariants: Variants = {
  idle: { rotate: 0, scale: 1 },
  hover: {
    rotate: 72,
    scale: 1.15,
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    rotate: 144,
    scale: 1.2,
    filter: 'drop-shadow(0 0 8px currentColor)',
    transition: { duration: DURATIONS.slow },
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

export const PageNativeCardContent = memo(function PageNativeCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: PageNativeCardContentProps) {
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

  const StarIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (output) + Realm (org) + Trait (generated) + Class (PageNative) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="output"
          realm="org"
          trait="generated"
          className="PageNative"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <OutputLocaleBadge locale={data.locale_key} color={colors.primary} />
      </div>

      {/* Page key */}
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

        {/* Assembled content preview */}
        {data.assembled && Object.keys(data.assembled).length > 0 && (
          <div className="space-y-1">
            <span className="text-[9px] text-white/50 font-mono">assembled:</span>
            <ContentPreview
              content={data.assembled}
              maxKeys={3}
              color={colors.primary}
            />
          </div>
        )}

        {/* Assembly info */}
        <AssemblyInfo
          timestamp={data.assembled_at}
          version={data.assembler_version}
          color={colors.primary}
          label="assembled"
        />

        {/* Published at */}
        {data.published_at && (
          <div className="flex items-center gap-1 text-[8px]">
            <span className="text-white/50">published:</span>
            <span className="text-white/70">
              {new Date(data.published_at).toLocaleDateString()}
            </span>
          </div>
        )}
      </ContentWrapper>
    </div>
  );
});
