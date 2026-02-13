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
  isSchemaMode?: boolean;
  /** Callback when Schema badge hover state changes (for Matrix effect) */
  onSchemaHoverChange?: (isHovering: boolean) => void;
  /** Callback when Schema badge is clicked (Easter egg trigger) */
  onSchemaClick?: () => void;
}

/**
 * Unified stats counter showing node/edge counts with icons
 * Hovering nodes/relations expands the pill to show type breakdowns
 * In schema mode, shows "Schema · X types · Y relations" with adapted wording
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
  isSchemaMode = false,
  onSchemaHoverChange,
  onSchemaClick,
}: StatsCounterProps) {
  const NodeIcon = GRAPH_ICONS.node;
  const RelIcon = GRAPH_ICONS.relationship;

  // Wording changes in schema mode
  const nodeLabel = isSchemaMode ? 'types' : 'nodes';
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
      {/* Mode badge - Always visible: Schema (blue) or Data (emerald) */}
      <button
        type="button"
        onClick={onSchemaClick}
        className={cn(
          'flex items-center rounded-xl px-3 py-1.5 cursor-pointer',
          gapTokens.default,
          // Colors and typography - monospace code style
          'font-mono font-medium text-xs uppercase tracking-wider',
          // Mode-specific colors
          isSchemaMode
            ? 'bg-blue-500/15 text-blue-300 border-2 border-blue-400/40 hover:border-blue-400/60 hover:bg-blue-500/20 hover:text-blue-200 hover:shadow-[0_0_16px_rgba(59,130,246,0.4),0_0_32px_rgba(59,130,246,0.15)]'
            : 'bg-emerald-500/15 text-emerald-300 border-2 border-emerald-400/40 hover:border-emerald-400/60 hover:bg-emerald-500/20 hover:text-emerald-200 hover:shadow-[0_0_16px_rgba(16,185,129,0.4),0_0_32px_rgba(16,185,129,0.15)]',
          // Subtle animated glow pulse (only in schema mode)
          isSchemaMode && 'animate-[glow-pulse_4s_ease-in-out_infinite]',
          // Transition
          'transition-all duration-300',
          // Active: scale down
          'active:scale-95'
        )}
        onMouseEnter={() => onSchemaHoverChange?.(true)}
        onMouseLeave={() => onSchemaHoverChange?.(false)}
      >
        <Boxes className="w-4 h-4" />
        <span>{isSchemaMode ? 'Schema' : 'Data'}</span>
      </button>
      <span className="text-white/20 group-hover/stats:text-white/40 transition-colors">·</span>

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
