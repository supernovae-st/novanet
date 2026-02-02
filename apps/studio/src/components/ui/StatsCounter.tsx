'use client';

import { memo } from 'react';
import { Layers } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
import { GRAPH_ICONS, ICON_COLORS } from '@/config/iconSystem';
import type { ExpandedViewType } from '@/components/query/ResultsOverview';

interface StatsCounterProps {
  /** Number of nodes to display */
  nodeCount: number;
  /** Number of edges to display */
  edgeCount: number;
  /** Whether data is currently loading */
  isLoading?: boolean;
  /** Additional CSS classes */
  className?: string;
  /** Currently expanded breakdown view */
  expandedView?: ExpandedViewType;
  /** Hover callbacks for expanding breakdowns */
  onHoverNodes?: () => void;
  onHoverRelations?: () => void;
  onHoverLeave?: () => void;
  /** Whether in schema mode (changes wording and adds badge) */
  isMetaMode?: boolean;
}

/**
 * Unified stats counter showing node/edge counts with icons
 * Hovering nodes/relations expands the pill to show type breakdowns
 * In meta mode, shows "🔷 Schema · X types · Y relations" with adapted wording
 */
export const StatsCounter = memo(function StatsCounter({
  nodeCount,
  edgeCount,
  isLoading = false,
  className,
  expandedView,
  onHoverNodes,
  onHoverRelations,
  onHoverLeave,
  isMetaMode = false,
}: StatsCounterProps) {
  const NodeIcon = GRAPH_ICONS.node;
  const RelIcon = GRAPH_ICONS.relationship;

  // Wording changes in schema mode
  const nodeLabel = isMetaMode ? 'types' : 'nodes';
  const relLabel = 'relations';

  return (
    <div
      className={cn(
        'flex items-center text-sm transition-opacity duration-150',
        gapTokens.spacious,
        isLoading && 'opacity-50',
        className
      )}
    >
      {/* Schema badge (schema mode only) */}
      {isMetaMode && (
        <>
          <span className={cn(
            'flex items-center rounded-md px-2 py-0.5',
            gapTokens.compact,
            'bg-blue-500/20 text-blue-400 font-medium'
          )}>
            <Layers className={iconSizes.sm} />
            <span>Schema</span>
          </span>
          <span className="text-white/40">·</span>
        </>
      )}

      {/* Nodes/Types - Emerald theme */}
      <span
        className={cn(
          'flex items-center rounded-md px-1.5 py-0.5 -mx-1.5 cursor-default',
          gapTokens.compact,
          'transition-all duration-200',
          expandedView === 'nodes' || expandedView === 'all'
            ? 'bg-white/[0.08] text-white'
            : 'hover:bg-white/[0.05]'
        )}
        onMouseEnter={onHoverNodes}
        onMouseLeave={onHoverLeave}
      >
        <NodeIcon className={cn(
          iconSizes.md,
          ICON_COLORS.node.muted,
          isLoading && 'animate-pulse'
        )} />
        <span className="text-white font-semibold tabular-nums">
          {nodeCount.toLocaleString()}
        </span>
        <span className="text-white/50">{nodeLabel}</span>
      </span>

      <span className="text-white/40">|</span>

      {/* Relations - Violet theme */}
      <span
        className={cn(
          'flex items-center rounded-md px-1.5 py-0.5 -mx-1.5 cursor-default',
          gapTokens.compact,
          'transition-all duration-150',
          expandedView === 'relations' || expandedView === 'all'
            ? 'bg-white/[0.08] text-white'
            : 'hover:bg-white/[0.05]'
        )}
        onMouseEnter={onHoverRelations}
        onMouseLeave={onHoverLeave}
      >
        <RelIcon className={cn(iconSizes.md, ICON_COLORS.relationship.muted)} />
        <span className="text-white font-semibold tabular-nums">
          {edgeCount.toLocaleString()}
        </span>
        <span className="text-white/50">{relLabel}</span>
      </span>
    </div>
  );
});
