'use client';

/**
 * ResultsOverview - Compact node type badges with hover expansion
 *
 * Shows emoji + count by default, expands to show type name on hover.
 */

import { useMemo, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { hexToRgba, OPACITY } from '@/lib/colorUtils';
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
 * Compact node type breakdown - emoji + count, name on hover
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
    <div className={cn('flex items-center gap-2', className)}>
      {/* Node type badges - compact with hover expansion */}
      {typeCounts.map((item) => (
        <span
          key={item.type}
          className="group flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap transition-all duration-200 cursor-default"
          style={{
            backgroundColor: hexToRgba(item.color, OPACITY.MEDIUM),
            color: item.color,
            borderWidth: 1,
            borderColor: hexToRgba(item.color, OPACITY.BORDER),
          }}
          title={`${item.type}: ${item.count} nodes`}
        >
          <CategoryIcon
            category={item.category}
            size={14}
            strokeWidth={2}
            style={{ color: item.color }}
          />
          <span className="max-w-0 overflow-hidden group-hover:max-w-[80px] transition-all duration-200 ease-out">
            {item.type}
          </span>
          <span
            className="px-1.5 py-0.5 rounded text-[10px] font-bold min-w-[20px] text-center"
            style={{ backgroundColor: hexToRgba(item.color, OPACITY.BORDER) }}
          >
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
