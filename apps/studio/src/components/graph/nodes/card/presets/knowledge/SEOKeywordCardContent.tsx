'use client';

/**
 * SEOKeywordCardContent - "Search Analytics" design for SEOKeyword nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> double (imported trait)
 * - Shows keyword, volume, difficulty, traffic potential, SERP features
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ⌕ SEO KEYWORD     ◆ transactional   │  <- Search icon + intent badge
 * │ ═════════════════════════════════   │  <- Double separator (imported)
 * │ seo-creer-qr-code-gratuit-fr         │
 * │ ┌────────────────────────────────┐   │
 * │ │ "créer qr code gratuit"        │   │  <- Keyword value
 * │ │ ─────────────────────────────  │   │
 * │ │ volume                  12.1K  │   │  <- Volume bar
 * │ │ [████████████░░░░░░]           │   │
 * │ │ ─────────────────────────────  │   │
 * │ │ difficulty: 35  ↑ rising       │   │  <- Difficulty + trend
 * │ │ traffic potential: 8.5K        │   │
 * │ │ ─────────────────────────────  │   │
 * │ │ ◆ ◈ ? ▣ ▶                      │   │  <- SERP features icons
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { DURATIONS } from '../../animationPresets';
import {
  VolumeDisplay,
  DifficultyBadge,
  TrafficPotential,
  IntentBadge,
  SerpFeatures,
  TrendBadge,
} from './KnowledgeHelpers';
import { GlowEffect } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';

// =============================================================================
// Types
// =============================================================================

export interface SEOKeywordNodeData {
  id: string;
  type: 'SEOKeyword';
  key: string;
  displayName: string;
  /** The keyword string */
  value: string;
  /** Description */
  description?: string;
  /** Monthly search volume */
  volume?: number;
  /** SEO difficulty score (0-100) */
  difficulty?: number;
  /** Cost per click (USD) */
  cpc?: number;
  /** Search intent */
  intent?: 'transactional' | 'informational' | 'navigational' | 'commercial';
  /** Search platform */
  platform?: 'google' | 'bing' | 'youtube';
  /** Data source */
  source?: 'ahrefs' | 'semrush';
  /** Estimated traffic if ranking #1 */
  traffic_potential?: number;
  /** Estimated monthly clicks */
  clicks?: number;
  /** Clicks/volume ratio */
  clicks_per_search?: number;
  /** SERP features present */
  serp_features?: string[];
  /** Organic competition level (0-1) */
  competition?: number;
  /** Search trend direction */
  trend?: 'rising' | 'stable' | 'declining';
  /** 12-month volume index */
  seasonality?: number[];
}

export interface SEOKeywordCardContentProps extends CardContext {
  data: SEOKeywordNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const searchVariants: Variants = {
  idle: { scale: 1 },
  hover: {
    scale: 1.1,
    rotate: [0, -10, 10, 0],
    transition: { duration: DURATIONS.normal },
  },
  selected: {
    scale: 1.15,
    filter: 'drop-shadow(0 0 8px currentColor)',
  },
};

const metricsVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const SEOKeywordCardContent = memo(function SEOKeywordCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: SEOKeywordCardContentProps) {
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

  const SearchIcon = animationsEnabled ? motion.span : 'span';
  const MetricsWrapper = animationsEnabled ? motion.div : 'div';

  // Check if glow should be shown (MEDIUM+ tier)
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;

  return (
    <div className="relative px-4 py-4">
      {/* Premium glow effect (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (imported) + Class (SEOKeyword) */}
      <div className="mb-3">
        <TaxonomyBadge
          layer="knowledge"
          realm="shared"
          trait="imported"
          className="SEOKeyword"
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          size="sm"
        />
      </div>

      {/* Intent badge */}
      <div className="flex justify-end mb-2">
        {data.intent && <IntentBadge intent={data.intent} />}
      </div>

      {/* Keyword key */}
      <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

      {/* Keyword value */}
      <h4
        className="text-base font-bold text-white mb-3"
        style={glowStyle}
      >
        "{data.value}"
      </h4>

      {/* Metrics section */}
      <MetricsWrapper
        className="space-y-3"
        {...(animationsEnabled && {
          variants: metricsVariants,
          initial: 'idle',
          animate: animationState,
        })}
      >
        {/* Volume display */}
        {typeof data.volume === 'number' && (
          <VolumeDisplay
            volume={data.volume}
            color={colors.primary}
            animated={animationsEnabled}
          />
        )}

        {/* Difficulty + Trend row */}
        <div className="flex items-center justify-between">
          {typeof data.difficulty === 'number' && (
            <DifficultyBadge difficulty={data.difficulty} color={colors.primary} />
          )}
          {data.trend && <TrendBadge trend={data.trend} />}
        </div>

        {/* Traffic potential */}
        {typeof data.traffic_potential === 'number' && (
          <TrafficPotential
            trafficPotential={data.traffic_potential}
            color={colors.primary}
          />
        )}

        {/* CPC */}
        {typeof data.cpc === 'number' && (
          <div className="flex items-center gap-1.5 text-[8px]">
            <span className="text-white/50">CPC:</span>
            <span className="font-bold" style={{ color: '#22c55e' }}>
              ${data.cpc.toFixed(2)}
            </span>
          </div>
        )}

        {/* SERP Features */}
        {data.serp_features && data.serp_features.length > 0 && (
          <div className="space-y-1">
            <span className="text-[8px] text-white/50">SERP features:</span>
            <SerpFeatures features={data.serp_features} color={colors.primary} />
          </div>
        )}

        {/* Platform + Source */}
        <div className="flex items-center gap-2 text-[8px]">
          {data.platform && (
            <span
              className="px-1 py-0.5 rounded"
              style={{
                backgroundColor: `${colors.primary}15`,
                color: `${colors.primary}cc`,
              }}
            >
              {data.platform}
            </span>
          )}
          {data.source && (
            <span className="text-white/40">via {data.source}</span>
          )}
        </div>
      </MetricsWrapper>
    </div>
  );
});
