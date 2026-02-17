'use client';

/**
 * SEOKeywordMetricsCardContent - "Analytics Dashboard" design for SEOKeywordMetrics nodes
 *
 * Visual Encoding (ADR-005):
 * - Primary color (from Layer = knowledge) -> purple #8b5cf6
 * - Border style -> dotted (retrieved trait per ADR-024)
 * - Shows snapshot date, trends, position rank, CTR, clicks, impressions
 *
 * Premium Effects (v0.13.1):
 * - GridPattern: Analytics matrix background (data flowing in feel)
 * - BorderBeam: Animated border respecting dotted style (retrieved trait)
 * - Meteors: Data "streaming in" from external APIs on selection
 * - MotionTechCorners: Metrics dashboard corners
 * - Enhanced GlowEffect: Analytics-themed intensity
 *
 * Layout:
 * ┌──────────────────────────────────────┐
 * │ ⌕ SEO METRICS     ◆ 2024-01-15      │  <- Search icon + snapshot date badge
 * │ ........... (dotted separator)      │  <- Dotted line (retrieved trait)
 * │ seo-metrics:creer-qr-code@fr-FR     │
 * │ ┌────────────────────────────────┐   │
 * │ │ volume trend  ▁▂▃▅▇▆▄         │   │  <- Mini sparkline
 * │ │ ─────────────────────────────  │   │
 * │ │ position: #3  ↑2              │   │  <- Position with delta
 * │ │ ─────────────────────────────  │   │
 * │ │ clicks: 1.2K  impr: 45K       │   │  <- Click stats
 * │ │ CTR: 2.67%                    │   │
 * │ │ ─────────────────────────────  │   │
 * │ │ difficulty: Δ+5  ↑ rising     │   │  <- Difficulty delta badge
 * │ └────────────────────────────────┘   │
 * └──────────────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { motion, type Variants } from 'motion/react';
import type { CardContext } from '../../CardShell';
import type { PerformanceConfig } from '@/contexts/PerformanceContext';
import { TrendBadge, DifficultyBadge } from './KnowledgeHelpers';
import { GlowEffect, GridPattern, BorderBeam, Meteors } from '../../effects';
import { TaxonomyBadge } from '../../TaxonomyBadge';
import { MotionTechCorners } from '../../../effects';

// =============================================================================
// Types
// =============================================================================

/** Single data point for trend history */
export interface TrendDataPoint {
  date: string;
  value: number;
}

export interface SEOKeywordMetricsNodeData {
  id: string;
  type: 'SEOKeywordMetrics';
  key: string;
  displayName: string;
  /** Description */
  description?: string;
  /** When metrics were retrieved */
  snapshot_date?: string;
  /** Array of volume data points */
  volume_trend?: TrendDataPoint[];
  /** Array of difficulty changes */
  difficulty_history?: TrendDataPoint[];
  /** Array of CPC data */
  cpc_history?: TrendDataPoint[];
  /** Current SERP position */
  position_rank?: number;
  /** Array of position changes */
  position_history?: TrendDataPoint[];
  /** Estimated clicks */
  clicks?: number;
  /** Estimated impressions */
  impressions?: number;
  /** Click-through rate (0-100) */
  ctr?: number;
  /** Overall trend direction */
  trend?: 'rising' | 'stable' | 'declining';
  /** Current difficulty value */
  difficulty?: number;
  /** Difficulty change since last snapshot */
  difficulty_delta?: number;
}

export interface SEOKeywordMetricsCardContentProps extends CardContext {
  data: SEOKeywordMetricsNodeData;
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Animation Variants
// =============================================================================

const metricsVariants: Variants = {
  idle: { opacity: 0.85 },
  hover: { opacity: 1 },
  selected: { opacity: 1 },
};

const sparklineVariants: Variants = {
  idle: { opacity: 0.7, scale: 1 },
  hover: { opacity: 0.9, scale: 1.02 },
  selected: {
    opacity: 1,
    scale: 1.05,
    transition: { duration: 0.3 },
  },
};

// =============================================================================
// Helper Components
// =============================================================================

interface SparklineProps {
  data: TrendDataPoint[];
  color: string;
  animated?: boolean;
  selected?: boolean;
  isHovered?: boolean;
}

const Sparkline = memo(function Sparkline({
  data,
  color,
  animated = false,
  selected = false,
  isHovered = false,
}: SparklineProps) {
  const SparkWrapper = animated ? motion.div : 'div';
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Get min/max for scaling
  const values = data.map((d) => d.value);
  const min = Math.min(...values);
  const max = Math.max(...values);
  const range = max - min || 1;

  // Generate bar heights
  const barCount = Math.min(data.length, 12);
  const bars = data.slice(-barCount).map((d) => ({
    height: ((d.value - min) / range) * 100,
    value: d.value,
  }));

  return (
    <SparkWrapper
      className="flex items-end gap-0.5 h-4"
      {...(animated && {
        variants: sparklineVariants,
        initial: 'idle',
        animate: animationState,
      })}
    >
      {bars.map((bar, i) => (
        <div
          key={i}
          className="w-1.5 rounded-sm transition-all duration-200"
          style={{
            height: `${Math.max(bar.height, 10)}%`,
            backgroundColor: color,
            opacity: 0.4 + (i / bars.length) * 0.6,
            boxShadow: selected ? `0 0 4px ${color}60` : 'none',
          }}
          title={`${bar.value}`}
        />
      ))}
    </SparkWrapper>
  );
});

interface PositionIndicatorProps {
  position: number;
  history?: TrendDataPoint[];
  color: string;
  selected?: boolean;
}

const PositionIndicator = memo(function PositionIndicator({
  position,
  history,
  color,
  selected = false,
}: PositionIndicatorProps) {
  // Calculate position delta from history
  const delta = useMemo(() => {
    if (!history || history.length < 2) return 0;
    const current = history[history.length - 1]?.value ?? position;
    const previous = history[history.length - 2]?.value ?? position;
    return previous - current; // Positive = improved (lower rank is better)
  }, [history, position]);

  const deltaColor = delta > 0 ? '#22c55e' : delta < 0 ? '#ef4444' : '#f59e0b';
  const deltaIcon = delta > 0 ? '↑' : delta < 0 ? '↓' : '→';

  return (
    <div className="flex items-center gap-2">
      <span className="text-[8px] text-white/50">position:</span>
      <span
        className="text-sm font-bold px-1.5 py-0.5 rounded"
        style={{
          backgroundColor: `${color}20`,
          color: color,
          boxShadow: selected ? `0 0 8px ${color}40` : 'none',
        }}
      >
        #{position}
      </span>
      {delta !== 0 && (
        <span
          className="text-xs font-bold px-1 py-0.5 rounded"
          style={{
            backgroundColor: `${deltaColor}20`,
            color: deltaColor,
          }}
        >
          {deltaIcon}{Math.abs(delta)}
        </span>
      )}
    </div>
  );
});

interface StatsRowProps {
  clicks?: number;
  impressions?: number;
  ctr?: number;
  color: string;
  selected?: boolean;
}

const StatsRow = memo(function StatsRow({
  clicks,
  impressions,
  ctr,
  color,
  selected = false,
}: StatsRowProps) {
  const formatNumber = (num: number) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num.toString();
  };

  return (
    <div className="flex items-center justify-between gap-2 text-[8px]">
      {typeof clicks === 'number' && (
        <div className="flex items-center gap-1">
          <span className="text-white/50">clicks:</span>
          <span
            className="font-bold"
            style={{
              color: '#22c55e',
              textShadow: selected ? '0 0 6px #22c55e40' : 'none',
            }}
          >
            {formatNumber(clicks)}
          </span>
        </div>
      )}
      {typeof impressions === 'number' && (
        <div className="flex items-center gap-1">
          <span className="text-white/50">impr:</span>
          <span
            className="font-bold"
            style={{
              color: '#3b82f6',
              textShadow: selected ? '0 0 6px #3b82f640' : 'none',
            }}
          >
            {formatNumber(impressions)}
          </span>
        </div>
      )}
      {typeof ctr === 'number' && (
        <div className="flex items-center gap-1">
          <span className="text-white/50">CTR:</span>
          <span
            className="font-bold"
            style={{
              color: color,
              textShadow: selected ? `0 0 6px ${color}40` : 'none',
            }}
          >
            {ctr.toFixed(2)}%
          </span>
        </div>
      )}
    </div>
  );
});

interface DifficultyDeltaBadgeProps {
  difficulty: number;
  delta?: number;
  color: string;
  selected?: boolean;
}

const DifficultyDeltaBadge = memo(function DifficultyDeltaBadge({
  difficulty,
  delta,
  color,
  selected = false,
}: DifficultyDeltaBadgeProps) {
  const deltaColor = delta && delta > 0 ? '#ef4444' : delta && delta < 0 ? '#22c55e' : '#f59e0b';
  const deltaPrefix = delta && delta > 0 ? '+' : '';

  return (
    <div className="flex items-center gap-2">
      <DifficultyBadge difficulty={difficulty} color={color} />
      {typeof delta === 'number' && delta !== 0 && (
        <span
          className="text-[8px] font-bold px-1 py-0.5 rounded"
          style={{
            backgroundColor: `${deltaColor}20`,
            color: deltaColor,
            boxShadow: selected ? `0 0 6px ${deltaColor}30` : 'none',
          }}
        >
          {'\u0394'}{deltaPrefix}{delta}
        </span>
      )}
    </div>
  );
});

// =============================================================================
// Component
// =============================================================================

export const SEOKeywordMetricsCardContent = memo(function SEOKeywordMetricsCardContent({
  data,
  colors,
  selected = false,
  isHovered = false,
  performanceConfig,
}: SEOKeywordMetricsCardContentProps) {
  const animationsEnabled = performanceConfig?.animation?.enabled ?? true;
  const animationState = selected ? 'selected' : isHovered ? 'hover' : 'idle';

  // Performance tier checks
  const showGlow = performanceConfig?.effects?.outerGlow ?? true;
  const showPremiumEffects = performanceConfig?.effects?.premiumEffects ?? true;
  const showTechCorners = performanceConfig?.effects?.techCorners ?? true;

  // Enhanced glow style for analytics/metrics feel
  const glowStyle = useMemo(
    () => ({
      textShadow: selected
        ? `0 0 16px ${colors.primary}80, 0 0 32px ${colors.primary}40`
        : isHovered
          ? `0 0 12px ${colors.primary}60`
          : 'none',
    }),
    [colors.primary, selected, isHovered]
  );

  const MetricsWrapper = animationsEnabled ? motion.div : 'div';

  // Grid pattern squares for data matrix effect (analytics/streaming data feel)
  const gridSquares = useMemo((): [number, number][] => {
    // Create a pattern that suggests streaming data points
    return [
      [1, 1], [3, 2], [5, 1], [7, 3],
      [2, 4], [4, 3], [6, 5], [8, 2],
      [1, 5], [5, 4], [3, 6], [7, 6],
    ];
  }, []);

  // Format snapshot date
  const formattedDate = useMemo(() => {
    if (!data.snapshot_date) return null;
    try {
      const date = new Date(data.snapshot_date);
      return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
      });
    } catch {
      return data.snapshot_date;
    }
  }, [data.snapshot_date]);

  return (
    <div className="relative px-4 py-4">
      {/* Layer 0: GridPattern - Streaming data analytics background (MEDIUM+ tier) */}
      {showPremiumEffects && (
        <GridPattern
          width={14}
          height={14}
          color={colors.primary}
          opacity={selected ? 0.28 : isHovered ? 0.2 : 0.14}
          squares={gridSquares}
          squareColor={colors.primary}
          flicker={animationsEnabled}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 1: BorderBeam - Animated border with dotted style feel (MEDIUM+ tier) */}
      {showPremiumEffects && (selected || isHovered) && (
        <BorderBeam
          color={colors.primary}
          secondaryColor={colors.secondary ?? colors.primary}
          borderRadius={12}
          thickness={2}
          duration={selected ? 3 : 5}
          beamLength={selected ? 0.15 : 0.1}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 2: Enhanced GlowEffect (MEDIUM+ tier) */}
      {showGlow && (selected || isHovered) && (
        <GlowEffect
          color={colors.primary}
          intensity={selected ? 'ultra' : isHovered ? 'high' : 'medium'}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Layer 3: Meteors - Data "streaming in" from external APIs (HIGH+ tier, selected only) */}
      {showPremiumEffects && selected && (
        <Meteors
          count={8}
          color={colors.primary}
          trailColor={colors.secondary ?? colors.primary}
          angle={200}
          minDuration={0.8}
          maxDuration={2}
          minDelay={0.1}
          maxDelay={2.5}
          meteorWidth={50}
          selected={selected}
          isHovered={isHovered}
          performanceConfig={performanceConfig}
          className="rounded-lg"
        />
      )}

      {/* Layer 4: MotionTechCorners - Metrics dashboard corners (MEDIUM+ tier) */}
      {showTechCorners && (selected || isHovered) && (
        <MotionTechCorners
          color={colors.primary}
          selected={selected}
          isHovered={isHovered}
          size={14}
          performanceConfig={performanceConfig}
        />
      )}

      {/* Content Layer */}
      <div className="relative z-10">
        {/* Taxonomy Badge: Layer (knowledge) + Realm (shared) + Trait (retrieved) + Class (SEOKeywordMetrics) */}
        <div className="mb-3">
          <TaxonomyBadge
            layer="knowledge"
            realm="shared"
            trait="retrieved"
            className="SEOKeywordMetrics"
            selected={selected}
            isHovered={isHovered}
            performanceConfig={performanceConfig}
            size="sm"
          />
        </div>

        {/* Snapshot date badge */}
        {formattedDate && (
          <div className="flex justify-end mb-2">
            <span
              className="inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-[8px] font-mono"
              style={{
                backgroundColor: `${colors.primary}20`,
                color: colors.primary,
                border: `1px dotted ${colors.primary}40`,
                boxShadow: selected ? `0 0 6px ${colors.primary}30` : 'none',
              }}
            >
              {'\u25C6'} {formattedDate}
            </span>
          </div>
        )}

        {/* Metrics key with subtle analytics styling */}
        <h3 className="text-xs font-mono text-white/60 mb-1 truncate">{data.key}</h3>

        {/* Display name with enhanced glow */}
        <h4
          className="text-base font-bold text-white mb-3"
          style={glowStyle}
        >
          {data.displayName}
        </h4>

        {/* Metrics section with dashboard styling */}
        <MetricsWrapper
          className="space-y-3"
          {...(animationsEnabled && {
            variants: metricsVariants,
            initial: 'idle',
            animate: animationState,
          })}
        >
          {/* Volume trend sparkline */}
          {data.volume_trend && data.volume_trend.length > 0 && (
            <div className="space-y-1">
              <span className="text-[8px] text-white/50">volume trend:</span>
              <Sparkline
                data={data.volume_trend}
                color={colors.primary}
                animated={animationsEnabled}
                selected={selected}
                isHovered={isHovered}
              />
            </div>
          )}

          {/* Position rank with delta */}
          {typeof data.position_rank === 'number' && (
            <PositionIndicator
              position={data.position_rank}
              history={data.position_history}
              color={colors.primary}
              selected={selected}
            />
          )}

          {/* Clicks / Impressions / CTR stats row */}
          <StatsRow
            clicks={data.clicks}
            impressions={data.impressions}
            ctr={data.ctr}
            color={colors.primary}
            selected={selected}
          />

          {/* Difficulty with delta + Trend row */}
          <div className="flex items-center justify-between">
            {typeof data.difficulty === 'number' && (
              <DifficultyDeltaBadge
                difficulty={data.difficulty}
                delta={data.difficulty_delta}
                color={colors.primary}
                selected={selected}
              />
            )}
            {data.trend && <TrendBadge trend={data.trend} />}
          </div>
        </MetricsWrapper>
      </div>
    </div>
  );
});
