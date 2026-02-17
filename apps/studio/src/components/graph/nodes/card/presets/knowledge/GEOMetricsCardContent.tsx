'use client';

/**
 * GEOMetricsCardContent - "GEO Performance Dashboard" design for GEOMetrics nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> dotted (retrieved trait - fetched from external APIs)
 * - Shows visibility score, citation rate, brand mentions, competitor presence
 *
 * Premium Effects (v0.13.1):
 * - GridPattern: Data analytics matrix background
 * - BorderBeam: Animated border with DOTTED style (retrieved trait per ADR-005)
 * - Meteors: Data points "shooting in" on selection
 * - MotionTechCorners: Dashboard corners
 * - GlowEffect: Intensity based on visibility_score (low=red, medium=amber, high=green)
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ◈ GEO METRICS       2024-02-15      │  <- Icon + snapshot date
 * │ ........................................│  <- Dotted separator (retrieved)
 * │ ┌────────────────────────────────┐   │
 * │ │         VISIBILITY             │   │
 * │ │            [85]                │   │  <- Large score gauge
 * │ │ ─────────────────────────────  │   │
 * │ │ citation rate: 72% [███████░]  │   │  <- Progress bar
 * │ │ brand: 245 / 512 queries       │   │  <- Ratio
 * │ │ avg position: #3               │   │  <- Position indicator
 * │ │ ─────────────────────────────  │   │
 * │ │ ↑ rising                       │   │  <- Trend direction
 * │ │ competitors: 3                 │   │  <- Mini list
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { TrendBadge } from './KnowledgeHelpers';
import { GlowEffect, GridPattern, BorderBeam, Meteors } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { MotionTechCorners } from '../../../effects';

// =============================================================================
// Types
// =============================================================================

export interface GEOMetricsNodeData {
  id: string;
  type: 'GEOMetrics';
  key: string;
  displayName: string;
  /** When metrics were aggregated */
  snapshot_date: string;
  /** Total number of queries tracked */
  total_queries?: number;
  /** Count of queries where brand was mentioned */
  brand_mentions?: number;
  /** Percentage of queries where brand was cited as source (0-100) */
  citation_rate?: number;
  /** Map of competitor -> mention count */
  competitor_presence?: Record<string, number>;
  /** Average position in AI answers */
  average_position?: number;
  /** Overall GEO visibility score (0-100) */
  visibility_score?: number;
  /** Weekly/monthly trend direction */
  trends?: 'rising' | 'stable' | 'declining';
  /** Description */
  description?: string;
}

export interface GEOMetricsCardContentProps extends CardContext {
  data: GEOMetricsNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Helper Components
// =============================================================================

interface VisibilityGaugeProps {
  score: number;
  selected: boolean;
  animated?: boolean;
}

const VisibilityGauge = memo(function VisibilityGauge({
  score,
  selected,
  animated = true,
}: VisibilityGaugeProps) {
  // Color based on score: low=red, medium=amber, high=green
  const color = useMemo(() => {
    if (score >= 70) return '#22c55e'; // green
    if (score >= 40) return '#f59e0b'; // amber
    return '#ef4444'; // red
  }, [score]);

  const gaugeVariants: Variants = {
    idle: { scale: 1, opacity: 0.9 },
    hover: { scale: 1.02, opacity: 1 },
    selected: {
      scale: [1, 1.05, 1],
      opacity: 1,
      transition: {
        scale: {
          duration: 2,
          repeat: Infinity,
          ease: 'easeInOut',
        },
      },
    },
  };

  const GaugeWrapper = animated ? motion.div : 'div';

  return (
    <GaugeWrapper
      className="flex flex-col items-center gap-1 py-2"
      {...(animated && {
        variants: gaugeVariants,
        initial: 'idle',
        animate: selected ? 'selected' : 'idle',
      })}
    >
      <span className="text-[8px] text-white/50 uppercase tracking-wider">Visibility</span>
      <div
        className="relative flex items-center justify-center w-14 h-14 rounded-full"
        style={{
          background: `conic-gradient(${color} ${score}%, transparent ${score}%)`,
          boxShadow: selected
            ? `0 0 20px ${color}60, inset 0 0 20px rgba(0,0,0,0.5)`
            : `inset 0 0 15px rgba(0,0,0,0.4)`,
        }}
      >
        <div
          className="absolute inset-1 rounded-full bg-black/80 flex items-center justify-center"
          style={{
            boxShadow: `0 0 10px ${color}30`,
          }}
        >
          <span
            className="text-lg font-bold"
            style={{
              color,
              textShadow: selected ? `0 0 10px ${color}` : 'none',
            }}
          >
            {score}
          </span>
        </div>
      </div>
    </GaugeWrapper>
  );
});

interface CitationRateBarProps {
  rate: number;
  color: string;
  selected: boolean;
}

const CitationRateBar = memo(function CitationRateBar({
  rate,
  color,
  selected,
}: CitationRateBarProps) {
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center justify-between">
        <span className="text-[8px] text-white/50">citation rate</span>
        <span
          className="text-xs font-bold"
          style={{
            color,
            textShadow: selected ? `0 0 8px ${color}60` : 'none',
          }}
        >
          {rate}%
        </span>
      </div>
      <div className="h-1.5 rounded-full bg-white/10 overflow-hidden">
        <motion.div
          className="h-full rounded-full"
          style={{ backgroundColor: color }}
          initial={{ width: 0 }}
          animate={{ width: `${rate}%` }}
          transition={{ duration: 0.8, ease: 'easeOut' }}
        />
      </div>
    </div>
  );
});

interface PositionIndicatorProps {
  position: number;
  color: string;
  selected: boolean;
}

const PositionIndicator = memo(function PositionIndicator({
  position,
  color,
  selected,
}: PositionIndicatorProps) {
  // Color coding: #1-3 green, #4-7 amber, #8+ red
  const posColor = position <= 3 ? '#22c55e' : position <= 7 ? '#f59e0b' : '#ef4444';

  return (
    <div className="flex items-center gap-1.5">
      <span className="text-[8px] text-white/50">avg position:</span>
      <span
        className="text-xs font-bold px-1.5 py-0.5 rounded"
        style={{
          backgroundColor: `${posColor}20`,
          color: posColor,
          boxShadow: selected ? `0 0 8px ${posColor}40` : 'none',
        }}
      >
        #{position}
      </span>
    </div>
  );
});

interface CompetitorListProps {
  competitors: Record<string, number>;
  color: string;
  maxShow?: number;
}

const CompetitorList = memo(function CompetitorList({
  competitors,
  color,
  maxShow = 3,
}: CompetitorListProps) {
  const entries = Object.entries(competitors)
    .sort((a, b) => b[1] - a[1])
    .slice(0, maxShow);

  const remaining = Object.keys(competitors).length - maxShow;

  return (
    <div className="flex flex-col gap-1">
      <span className="text-[8px] text-white/50">competitors:</span>
      <div className="flex flex-wrap gap-1">
        {entries.map(([name, count]) => (
          <span
            key={name}
            className="text-[7px] font-mono px-1 py-0.5 rounded"
            style={{
              backgroundColor: `${color}15`,
              color: `${color}cc`,
              border: `1px solid ${color}30`,
            }}
          >
            {name}: {count}
          </span>
        ))}
        {remaining > 0 && (
          <span className="text-[7px] text-white/40">+{remaining}</span>
        )}
      </div>
    </div>
  );
});

// =============================================================================
// Animation Variants
// =============================================================================

const metricsVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

// =============================================================================
// Component
// =============================================================================

export const GEOMetricsCardContent = memo(function GEOMetricsCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: GEOMetricsCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Performance tier checks
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  // Glow color based on visibility score
  const glowColor = useMemo(() => {
    const score = data.visibility_score ?? 0;
    if (score >= 70) return '#22c55e'; // green - high visibility
    if (score >= 40) return '#f59e0b'; // amber - medium visibility
    return '#ef4444'; // red - low visibility
  }, [data.visibility_score]);

  // Format snapshot date
  const formattedDate = useMemo(() => {
    if (!data.snapshot_date) return '';
    const date = new Date(data.snapshot_date);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
    });
  }, [data.snapshot_date]);

  // Grid pattern squares for dashboard effect
  const gridSquares = useMemo((): [number, number][] => {
    return [
      [0, 0], [2, 1], [4, 0], [6, 2],
      [1, 3], [3, 2], [5, 4], [7, 1],
      [0, 5], [4, 4], [6, 5], [8, 3],
    ];
  }, []);

  const MetricsWrapper = animationsEnabled ? motion.div : 'div';

  return (
    <div className="relative px-4 py-4">
      {/* Layer 0: GridPattern - Dashboard matrix background (MEDIUM+ tier) */}
      {showPremiumEffects && (
        <GridPattern
          width={14}
          height={14}
          color={colors.primary}
          opacity={selected ? 0.25 : isHovered ? 0.18 : 0.12}
          squares={gridSquares}
          squareColor={glowColor}
          flicker={animationsEnabled}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 1: BorderBeam - Animated dotted border (retrieved trait) (MEDIUM+ tier) */}
      {showPremiumEffects && (selected || isHovered) && (
        <BorderBeam
          color={glowColor}
          secondaryColor={colors.primary}
          borderRadius={12}
          thickness={2}
          duration={selected ? 5 : 7}
          beamLength={selected ? 0.15 : 0.1}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 2: GlowEffect - Color based on visibility score (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={glowColor}
          intensity={selected ? 'ultra' : isHovered ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 3: Meteors - Data points "shooting in" (HIGH+ tier, selected only) */}
      {showPremiumEffects && selected && (
        <Meteors
          count={5}
          color={glowColor}
          trailColor={colors.primary}
          angle={135}
          minDuration={1.5}
          maxDuration={3}
          minDelay={0.3}
          maxDelay={2.5}
          meteorWidth={50}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 4: MotionTechCorners - Dashboard corners (MEDIUM+ tier) */}
      {showTechCorners && (selected || isHovered) && (
        <MotionTechCorners
          color={glowColor}
          selected={selected}
          isHovered={isHovered}
          size={12}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content Layer */}
      <div className="relative z-10">
        {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (retrieved) + Class (GEOMetrics) */}
        <div className="mb-3">
          <TaxonomyBadge
            layer="knowledge"
            realm="shared"
            trait="retrieved"
            className="GEOMetrics"
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
          />
        </div>

        {/* Header: Icon + Title + Date */}
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-2">
            <span
              className="text-sm"
              style={{
                color: colors.primary,
                textShadow: selected ? `0 0 8px ${colors.primary}60` : 'none',
              }}
            >
              ◈
            </span>
            <span
              className="text-[10px] font-bold uppercase tracking-wider"
              style={{
                color: colors.primary,
                textShadow: selected ? `0 0 6px ${colors.primary}40` : 'none',
              }}
            >
              GEO Metrics
            </span>
          </div>
          {formattedDate && (
            <span className="text-[8px] text-white/50 font-mono">
              {formattedDate}
            </span>
          )}
        </div>

        {/* Dotted separator (retrieved trait visual) */}
        <div
          className="mb-3"
          style={{
            borderBottom: `1px dotted ${colors.primary}40`,
          }}
        />

        {/* Metrics section */}
        <MetricsWrapper
          className="space-y-3"
          {...(animationsEnabled && {
            variants: metricsVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Visibility Score Gauge */}
          {typeof data.visibility_score === 'number' && (
            <VisibilityGauge
              score={data.visibility_score}
              selected={selected}
              animated={animationsEnabled}
            />
          )}

          {/* Citation Rate */}
          {typeof data.citation_rate === 'number' && (
            <CitationRateBar
              rate={data.citation_rate}
              color={glowColor}
              selected={selected}
            />
          )}

          {/* Brand Mentions / Total Queries */}
          {typeof data.brand_mentions === 'number' && typeof data.total_queries === 'number' && (
            <div className="flex items-center gap-1.5 text-[8px]">
              <span className="text-white/50">brand:</span>
              <span
                className="font-bold"
                style={{
                  color: colors.primary,
                  textShadow: selected ? `0 0 6px ${colors.primary}40` : 'none',
                }}
              >
                {data.brand_mentions.toLocaleString()}
              </span>
              <span className="text-white/40">/</span>
              <span className="text-white/60">
                {data.total_queries.toLocaleString()} queries
              </span>
            </div>
          )}

          {/* Average Position */}
          {typeof data.average_position === 'number' && (
            <PositionIndicator
              position={Math.round(data.average_position)}
              color={colors.primary}
              selected={selected}
            />
          )}

          {/* Trend Direction */}
          {data.trends && (
            <div className="flex items-center gap-2">
              <span className="text-[8px] text-white/50">trend:</span>
              <TrendBadge trend={data.trends} />
            </div>
          )}

          {/* Competitor Presence */}
          {data.competitor_presence && Object.keys(data.competitor_presence).length > 0 && (
            <CompetitorList
              competitors={data.competitor_presence}
              color={colors.primary}
            />
          )}
        </MetricsWrapper>
      </div>
    </div>
  );
});
