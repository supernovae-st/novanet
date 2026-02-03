'use client';

import { memo } from 'react';
import { Boxes } from 'lucide-react';
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
  /** Callback when META badge hover state changes (for Matrix effect) */
  onMetaHoverChange?: (isHovering: boolean) => void;
  /** Callback when META badge is clicked (Easter egg trigger) */
  onMetaClick?: () => void;
}

/**
 * Unified stats counter showing node/edge counts with icons
 * Hovering nodes/relations expands the pill to show type breakdowns
 * In meta mode, shows "Meta · X types · Y relations" with adapted wording
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
  onMetaHoverChange,
  onMetaClick,
}: StatsCounterProps) {
  const NodeIcon = GRAPH_ICONS.node;
  const RelIcon = GRAPH_ICONS.relationship;

  // Wording changes in schema mode
  const nodeLabel = isMetaMode ? 'types' : 'nodes';
  const relLabel = 'relations';

  return (
    <div
      className={cn(
        'group/stats flex items-center text-sm font-mono transition-all duration-300',
        gapTokens.spacious,
        isLoading && 'opacity-50',
        className
      )}
    >
      {/* Meta badge - Glowing border style with subtle pulse animation - CLICKABLE for Easter egg */}
      {isMetaMode && (
        <>
          <button
            type="button"
            onClick={onMetaClick}
            className={cn(
              'flex items-center rounded-xl px-3 py-1.5 cursor-pointer',
              gapTokens.default,
              // Colors and typography - monospace code style
              'bg-blue-500/15 text-blue-300 font-mono font-medium text-xs uppercase tracking-wider',
              // Border with glow
              'border-2 border-blue-400/40',
              // Subtle animated glow pulse
              'animate-[glow-pulse_4s_ease-in-out_infinite]',
              // Hover: intensify glow
              'transition-all duration-300',
              'hover:border-blue-400/60',
              'hover:bg-blue-500/20',
              'hover:text-blue-200',
              'hover:shadow-[0_0_16px_rgba(59,130,246,0.4),0_0_32px_rgba(59,130,246,0.15)]',
              // Active: scale down
              'active:scale-95'
            )}
            onMouseEnter={() => onMetaHoverChange?.(true)}
            onMouseLeave={() => onMetaHoverChange?.(false)}
            title="Click for a surprise..."
          >
            <Boxes className="w-4 h-4" />
            <span>Meta</span>
          </button>
          <span className="text-white/20 group-hover/stats:text-white/40 transition-colors">·</span>
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
