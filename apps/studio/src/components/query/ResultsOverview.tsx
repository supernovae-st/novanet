'use client';

/**
 * ResultsOverview - Compact node type badges with hover expansion
 *
 * Unified glass design: all badges share the same dark glass background.
 * Color identity comes from the icon only - no per-type colored backgrounds.
 * Shows icon + count by default, expands to show type name on hover.
 */

import { useMemo, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { useGraphStore } from '@/stores/graphStore';
import { NODE_TYPE_CONFIG, type NodeCategory } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { NodeType } from '@/types';

interface ResultsOverviewProps {
  /** Additional CSS classes */
  className?: string;
  /** Maximum number of type badges to show */
  maxTypes?: number;
}

/**
 * Compact node type breakdown - icon + count, name on hover
 */
export const ResultsOverview = memo(function ResultsOverview({
  className,
  maxTypes = 6,
}: ResultsOverviewProps) {
  const { nodes, totalNodes } = useGraphStore(
    useShallow((state) => ({
      nodes: state.nodes,
      totalNodes: state.totalNodes,
    }))
  );

  // Calculate node type breakdown - single pass for both counts and total types
  const { typeCounts, totalTypes } = useMemo(() => {
    const counts = new Map<NodeType, number>();

    // Single pass: count all types
    for (const node of nodes) {
      const current = counts.get(node.type) || 0;
      counts.set(node.type, current + 1);
    }

    // Sort by count descending, take top N
    const topCounts = Array.from(counts.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, maxTypes)
      .map(([type, count]) => {
        const config = NODE_TYPE_CONFIG[type] || {
          color: '#6b7280',
          category: 'project' as NodeCategory,
        };
        return {
          type,
          count,
          color: config.color,
          category: config.category,
        };
      });

    return {
      typeCounts: topCounts,
      totalTypes: counts.size,
    };
  }, [nodes, maxTypes]);

  const remainingCount = totalTypes - typeCounts.length;

  // Show empty state if no nodes
  if (totalNodes === 0) {
    return (
      <div className={cn('flex items-center text-white/40 text-xs', gapTokens.default, className)}>
        <span>No results</span>
      </div>
    );
  }

  return (
    <div className={cn('flex items-center', gapTokens.default, className)}>
      {/* Node type badges - dark bg, colored icon with hover glow */}
      {typeCounts.map((item) => (
        <span
          key={item.type}
          className={cn(
            'group flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap cursor-default',
            'bg-white/[0.06] border border-white/[0.08]',
            'hover:bg-white/[0.10] hover:border-white/[0.15]',
            'transition-colors duration-200'
          )}
          style={{
            // CSS custom prop for dynamic hover glow via Tailwind
            ['--badge-glow' as string]: `${item.color}25`,
            ['--badge-glow-strong' as string]: `${item.color}40`,
          }}
          title={`${item.type}: ${item.count} nodes`}
        >
          <CategoryIcon
            category={item.category}
            size={14}
            strokeWidth={2}
            className="transition duration-200 group-hover:scale-110"
            style={{
              color: item.color,
              filter: `drop-shadow(0 0 4px ${item.color}50)`,
            }}
          />
          <span className="max-w-0 overflow-hidden group-hover:max-w-[80px] transition-all duration-200 ease-out text-white/60 group-hover:text-white/80">
            {item.type}
          </span>
          <span className="text-white/70 text-[10px] font-bold tabular-nums min-w-[20px] text-center">
            {item.count}
          </span>
        </span>
      ))}

      {/* Show remaining count if there are more types */}
      {remainingCount > 0 && (
        <span
          className="text-[10px] text-white/40 whitespace-nowrap px-1"
          title={`${remainingCount} more node types`}
        >
          +{remainingCount}
        </span>
      )}
    </div>
  );
});
