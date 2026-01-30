'use client';

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { GRAPH_ICONS, ICON_COLORS } from '@/config/iconSystem';

interface StatsCounterProps {
  /** Number of nodes to display */
  nodeCount: number;
  /** Number of edges to display */
  edgeCount: number;
  /** Whether data is currently loading */
  isLoading?: boolean;
  /** Additional CSS classes */
  className?: string;
}

/**
 * Unified stats counter showing node/edge counts with icons
 * Uses the NovaNet Icon Design System:
 * - Nodes: emerald theme + Atom icon (interconnected entities)
 * - Relations: violet theme + GitBranch icon (connections)
 */
export const StatsCounter = memo(function StatsCounter({
  nodeCount,
  edgeCount,
  isLoading = false,
  className,
}: StatsCounterProps) {
  const NodeIcon = GRAPH_ICONS.node;
  const RelIcon = GRAPH_ICONS.relationship;

  return (
    <div
      className={cn(
        'flex items-center gap-3 text-sm transition-opacity duration-150',
        isLoading && 'opacity-50',
        className
      )}
    >
      {/* Nodes - Emerald theme */}
      <span className="flex items-center gap-1.5">
        <NodeIcon className={cn(
          'w-4 h-4',
          ICON_COLORS.node.muted,
          isLoading && 'animate-pulse'
        )} />
        <span className="text-white font-semibold tabular-nums">
          {nodeCount.toLocaleString()}
        </span>
        <span className="text-white/50">nodes</span>
      </span>

      <span className="text-white/20">|</span>

      {/* Relations - Violet theme */}
      <span className="flex items-center gap-1.5">
        <RelIcon className={cn('w-4 h-4', ICON_COLORS.relationship.muted)} />
        <span className="text-white font-semibold tabular-nums">
          {edgeCount.toLocaleString()}
        </span>
        <span className="text-white/50">relations</span>
      </span>
    </div>
  );
});
