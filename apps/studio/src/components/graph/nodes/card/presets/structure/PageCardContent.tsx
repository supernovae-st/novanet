'use client';

/**
 * PageCardContent - "Page Blueprint" design for Page nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = structure) → blue #3b82f6
 * - Border style → solid
 * - Shows URL paths, block count, SEO score
 *
 * Layout:
 * ┌──────────────────────────────┐
 * │ 📄 PAGE              ★pillar │
 * │ ══════════════════════════   │
 * │ qr-code-generator            │
 * │ ┌────────────────────────┐   │
 * │ │ 🌐 /qr-code-generator  │   │ ← URL paths
 * │ │ ↳ /fr/generateur-qr    │   │
 * │ ├────────────────────────┤   │
 * │ │ blocks 8 │ SEO ●●●○○   │   │ ← Stats
 * │ │ 🔗 12 internal links   │   │
 * │ └────────────────────────┘   │
 * └──────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
// import { cn } from '@/lib/utils';
// import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
// import { DURATIONS } from '../../animationPresets';
import {
  URLPathDisplay,
  type LocalePath,
  SEOScoreDisplay,
  PillarBadge,
  StatCounter,
} from './StructureHelpers';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { FileText, Layers } from 'lucide-react';

// =============================================================================
// Types
// =============================================================================

export interface PageNodeData {
  id: string;
  type: 'Page';
  key: string;
  displayName: string;
  /** URL slug (invariant) */
  slug?: string;
  /** Is this a pillar page? */
  is_pillar?: boolean;
  /** Number of blocks */
  block_count?: number;
  /** Number of internal links */
  link_count?: number;
  /** SEO score (0-100) */
  seo_score?: number;
  /** Localized paths */
  locale_paths?: LocalePath[];
  /** Number of PageNative children */
  nativeCount?: number;
}

export interface PageCardContentProps extends CardContext {
  data: PageNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const statsVariants: Variants = {
  idle: { opacity: 0.8 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const stackVariants: Variants = {
  idle: { y: 0, opacity: 0.3 },
  hover: { y: -2, opacity: 0.5 },
  selected: { y: -3, opacity: 0.6 },
};

// =============================================================================
// Component
// =============================================================================

export const PageCardContent = memo(function PageCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: PageCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Build base path from slug
  const basePath = useMemo(() => {
    return data.slug ? `/${data.slug}` : `/${data.key}`;
  }, [data.slug, data.key]);

  // Glow style
  const glowStyle = useMemo(() => ({
    textShadow: selected
      ? `0 0 12px ${colors.primary}60`
      : isHovered
        ? `0 0 8px ${colors.primary}40`
        : 'none',
  }), [colors.primary, selected, isHovered]);

  const StatsWrapper = animationsEnabled ? motion.div : 'div';
  const StackLayer = animationsEnabled ? motion.div : 'div';
  const hasNatives = typeof data.nativeCount === 'number' && data.nativeCount > 0;

  return (
    <div className="px-4 py-4 relative">
      {/* === STACK EFFECT: Shows PageNatives behind this Page === */}
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

      {/* Taxonomy Badge + Pillar + Natives badge */}
      <div className="flex items-center justify-between mb-3">
        <TaxonomyBadge
          layer="structure"
          realm="org"
          className="Page"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
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
          <PillarBadge isPillar={data.is_pillar || false} color="#f59e0b" />
        </div>
      </div>

      {/* FileText icon + key row */}
      <div className="flex items-center gap-2 mb-3">
        <FileText size={16} style={{ color: colors.primary }} />
        <h3
          className="text-base font-bold font-mono text-white truncate"
          style={glowStyle}
        >
          {data.key}
        </h3>
      </div>

      {/* URL Paths section */}
      <div className="mb-2">
        <URLPathDisplay
          basePath={basePath}
          localePaths={data.locale_paths}
        />
      </div>

      {/* Stats section */}
      <StatsWrapper
        className="p-2 rounded-lg"
        style={{
          background: `${colors.primary}08`,
          border: `1px solid ${colors.primary}20`,
        }}
        {...(animationsEnabled && {
          variants: statsVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <StatCounter
              label="blocks"
              value={data.block_count ?? 0}
              color={colors.primary}
            />
            <StatCounter
              label=""
              value=""
              icon="🔗"
            />
            <span className="text-[9px] text-white/60">
              {data.link_count ?? 0} links
            </span>
          </div>

          {typeof data.seo_score === 'number' && (
            <SEOScoreDisplay score={data.seo_score} />
          )}
        </div>
      </StatsWrapper>
    </div>
  );
});
