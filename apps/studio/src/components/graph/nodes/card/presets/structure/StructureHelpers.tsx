'use client';

/**
 * Structure Layer Helper Components
 *
 * Shared components for Structure Layer cards (Page, Block, ContentSlot)
 * - URLPathDisplay: Localized URL paths
 * - BlockOrderIndicator: Visual order position
 * - SEOScoreDisplay: 5-bar score indicator
 */

import { memo } from 'react';
import { motion } from 'motion/react';

// =============================================================================
// URLPathDisplay - Localized URL paths with hierarchy
// =============================================================================

export interface LocalePath {
  locale: string;
  path: string;
}

export interface URLPathDisplayProps {
  basePath: string;
  localePaths?: LocalePath[];
  maxPaths?: number;
}

export const URLPathDisplay = memo(function URLPathDisplay({
  basePath,
  localePaths = [],
  maxPaths = 2,
}: URLPathDisplayProps) {
  const visiblePaths = localePaths.slice(0, maxPaths);
  const remainingCount = localePaths.length - maxPaths;

  return (
    <div className="flex flex-col gap-1 p-2 rounded bg-white/5 font-mono text-[10px]">
      <div className="flex items-center gap-1">
        <span className="text-white/40">🌐</span>
        <span className="text-white/80">{basePath}</span>
      </div>
      {visiblePaths.map(({ locale, path }) => (
        <div key={locale} className="flex items-center gap-1 pl-3">
          <span className="text-white/30">↳</span>
          <span className="text-blue-400/60">/{locale}</span>
          <span className="text-white/50">{path}</span>
        </div>
      ))}
      {remainingCount > 0 && (
        <span className="text-white/30 pl-3">+{remainingCount} more</span>
      )}
    </div>
  );
});

// =============================================================================
// BlockOrderIndicator - Visual block position in page
// =============================================================================

export interface BlockOrderIndicatorProps {
  order: number;
  total: number;
  color: string;
  maxDisplay?: number;
  animate?: boolean;
}

export const BlockOrderIndicator = memo(function BlockOrderIndicator({
  order,
  total,
  color,
  maxDisplay = 8,
  animate = true,
}: BlockOrderIndicatorProps) {
  const displayCount = Math.min(total, maxDisplay);
  const BlockSquare = animate ? motion.div : 'div';

  return (
    <div className="flex gap-0.5 items-center">
      {Array.from({ length: displayCount }).map((_, i) => (
        <BlockSquare
          key={i}
          className="w-3 h-3 rounded-sm border"
          style={{
            backgroundColor: i + 1 === order ? color : 'transparent',
            borderColor: i + 1 === order ? color : `${color}30`,
          }}
          {...(animate && {
            whileHover: { scale: 1.2 },
          })}
        />
      ))}
      {total > maxDisplay && (
        <span className="text-[9px] text-white/40 ml-0.5">+{total - maxDisplay}</span>
      )}
    </div>
  );
});

// =============================================================================
// SEOScoreDisplay - 5-bar score visualization
// =============================================================================

export interface SEOScoreDisplayProps {
  score: number;
}

const getScoreColor = (score: number): string => {
  if (score >= 80) return '#10b981'; // green
  if (score >= 60) return '#eab308'; // yellow
  return '#ef4444'; // red
};

export const SEOScoreDisplay = memo(function SEOScoreDisplay({
  score,
}: SEOScoreDisplayProps) {
  const filledBars = Math.ceil(score / 20);
  const barColor = getScoreColor(score);

  return (
    <div className="flex items-center gap-1">
      <span className="text-[9px] text-white/40">SEO</span>
      <div className="flex gap-0.5">
        {[1, 2, 3, 4, 5].map((i) => (
          <div
            key={i}
            className="w-1.5 h-3 rounded-sm"
            style={{
              backgroundColor: i <= filledBars ? barColor : 'rgba(255,255,255,0.1)',
            }}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// PillarBadge - Star indicator for pillar pages
// =============================================================================

export interface PillarBadgeProps {
  isPillar: boolean;
  color: string;
}

export const PillarBadge = memo(function PillarBadge({
  isPillar,
  color,
}: PillarBadgeProps) {
  if (!isPillar) return null;

  return (
    <span
      className="text-sm"
      style={{
        color,
        filter: `drop-shadow(0 0 4px ${color})`,
      }}
      title="Pillar page"
    >
      ★
    </span>
  );
});

// =============================================================================
// StatCounter - Small stat with label
// =============================================================================

export interface StatCounterProps {
  label: string;
  value: number | string;
  icon?: string;
  color?: string;
}

export const StatCounter = memo(function StatCounter({
  label,
  value,
  icon,
  color = '#fff',
}: StatCounterProps) {
  return (
    <div className="flex items-center gap-1 text-[9px]">
      {icon && <span>{icon}</span>}
      <span className="text-white/40">{label}</span>
      <span style={{ color }}>{value}</span>
    </div>
  );
});
