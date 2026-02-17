'use client';

/**
 * OutputArtifactCardContent - "Release Bundle" design for OutputArtifact nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = output) → green #22c55e
 * - Border style → dotted (generated trait)
 * - Shows bundle stats, locale, checksum, deployment info
 *
 * Layout:
 * ┌──────────────────────────────────┐
 * │ ■ ARTIFACT         🇫🇷 fr-FR   │  ← Package icon + locale badge
 * │ ·····························   │  ← Dotted separator (generated)
 * │ outputartifact-qrcode-fr-FR-v1  │
 * │ ┌────────────────────────────┐   │
 * │ │ QRCode AI - French (v1)    │   │  ← Display name
 * │ │ ─────────────────────────  │   │
 * │ │ ● Published     v1         │   │  ← Status + version
 * │ │ ─────────────────────────  │   │
 * │ │    15       45      250KB  │   │  ← Bundle stats
 * │ │   pages   blocks    size   │   │
 * │ │ ─────────────────────────  │   │
 * │ │ sha256:abc123...           │   │  ← Checksum
 * │ │ bundled: Jan 15, 12:00     │   │  ← Bundle info
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
  BundleStats,
  ChecksumBadge,
} from './OutputHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface OutputArtifactNodeData {
  id: string;
  type: 'OutputArtifact';
  key: string; // outputartifact-{project}-{locale}-v{version}
  displayName: string;
  /** Description */
  description?: string;
  /** Target locale code */
  locale_code: string;
  /** Bundle version */
  version: number;
  /** Bundle status */
  status: 'draft' | 'validated' | 'published' | 'archived';
  /** Number of pages in bundle */
  pages_included: number;
  /** Number of blocks in bundle */
  blocks_included: number;
  /** Bundle timestamp */
  bundled_at?: string;
  /** Bundler version */
  bundler_version?: string;
  /** Total size in bytes */
  total_size_bytes?: number;
  /** Content integrity checksum */
  checksum?: string;
  /** Published timestamp */
  published_at?: string;
  /** Published by (user) */
  published_by?: string;
  /** Has previous version */
  has_previous_version?: boolean;
}

export interface OutputArtifactCardContentProps extends CardContext {
  data: OutputArtifactNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const packageVariants: Variants = {
  idle: { scale: 1 },
  hover: {
    scale: 1.1,
    y: -2,
    transition: { duration: DURATIONS.fast },
  },
  selected: {
    scale: 1.15,
    y: -4,
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

export const OutputArtifactCardContent = memo(function OutputArtifactCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: OutputArtifactCardContentProps) {
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

  const PackageIcon = animationsEnabled ? motion.span : 'span';
  const ContentWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="px-4 py-4">
      {/* Taxonomy Badge: Layer (output) + Realm (org) + Trait (generated) + Class (OutputArtifact) */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="output"
          realm="org"
          trait="generated"
          className="OutputArtifact"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
        <OutputLocaleBadge locale={data.locale_code} color={colors.primary} />
      </div>

      {/* Artifact key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Display name */}
      <h4 className="text-base font-bold text-white mb-3" style={glowStyle}>
        {data.displayName}
      </h4>

      {/* Content section */}
      <ContentWrapper
        className="space-y-3"
        {...(animationsEnabled && {
          variants: contentVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Status + Version */}
        <div className="flex items-center justify-between">
          <OutputStatusBadge status={data.status} />
          <VersionHistory
            version={data.version}
            hasPreviousVersion={data.has_previous_version}
            color={colors.primary}
          />
        </div>

        {/* Description */}
        {data.description && (
          <p className="text-[10px] text-white/60 line-clamp-2 italic">
            {data.description}
          </p>
        )}

        {/* Bundle stats */}
        <BundleStats
          pagesIncluded={data.pages_included}
          blocksIncluded={data.blocks_included}
          totalSizeBytes={data.total_size_bytes}
          color={colors.primary}
        />

        {/* Checksum */}
        {data.checksum && (
          <ChecksumBadge checksum={data.checksum} color={colors.primary} />
        )}

        {/* Bundle info */}
        <AssemblyInfo
          timestamp={data.bundled_at}
          version={data.bundler_version}
          color={colors.primary}
          label="bundled"
        />

        {/* Published info */}
        {data.published_at && (
          <div className="flex items-center gap-2 text-[8px]">
            <span className="text-white/50">published:</span>
            <span className="text-white/70">
              {new Date(data.published_at).toLocaleDateString()}
            </span>
            {data.published_by && (
              <>
                <span className="text-white/30">by</span>
                <span className="text-white/70">{data.published_by}</span>
              </>
            )}
          </div>
        )}
      </ContentWrapper>
    </div>
  );
});
