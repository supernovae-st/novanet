'use client';

/**
 * SubcategoryGroupNode - Nested glass container for subcategory groups
 *
 * Features (Task 5: TurboNode styling):
 * - Subtle glass effect inheriting parent scope color
 * - Compact label with hover interaction
 * - Smooth transitions
 *
 * Nested inside ScopeGroupNode - inherits scope coloring
 */

import { memo, useState, useCallback } from 'react';
import { type NodeProps, type Node } from '@xyflow/react';
import { cn } from '@/lib/utils';
import type { Scope } from '@novanet/core/types';

/**
 * Data interface for SubcategoryGroupNode
 */
export interface SubcategoryGroupData extends Record<string, unknown> {
  scope: Scope;
  subcategory: string;
  label: string;
  icon: string;
  nodeCount: number;
}

/** Node type for SubcategoryGroupNode */
export type SubcategoryGroupNodeType = Node<SubcategoryGroupData, 'subcategoryGroup'>;

/**
 * Scope color configuration (subset of parent colors)
 */
const SCOPE_COLORS: Record<Scope, {
  primary: string;
  glow: string;
}> = {
  Project: {
    primary: '#8b5cf6',
    glow: 'rgba(139, 92, 246, 0.2)',
  },
  Global: {
    primary: '#10b981',
    glow: 'rgba(16, 185, 129, 0.2)',
  },
  Shared: {
    primary: '#f59e0b',
    glow: 'rgba(245, 158, 11, 0.2)',
  },
};

/**
 * SubcategoryGroupNode - Nested container within a scope
 */
export const SubcategoryGroupNode = memo(function SubcategoryGroupNode({
  data,
  selected,
}: NodeProps<SubcategoryGroupNodeType>) {
  const [isHovered, setIsHovered] = useState(false);

  const colors = SCOPE_COLORS[data.scope] || SCOPE_COLORS.Project;

  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => setIsHovered(false), []);

  // Container style
  const containerStyle = {
    backgroundColor: isHovered || selected
      ? `${colors.primary}08`
      : 'rgba(255, 255, 255, 0.02)',
    borderColor: selected
      ? `${colors.primary}60`
      : isHovered
        ? `${colors.primary}40`
        : 'rgba(255, 255, 255, 0.08)',
    boxShadow: selected
      ? `inset 0 0 20px ${colors.glow}`
      : isHovered
        ? `inset 0 0 15px ${colors.glow}`
        : 'none',
  };

  // Label style
  const labelStyle = {
    backgroundColor: selected
      ? `${colors.primary}20`
      : isHovered
        ? `${colors.primary}15`
        : 'rgba(0, 0, 0, 0.6)',
    borderColor: selected ? `${colors.primary}40` : 'transparent',
    boxShadow: selected ? `0 0 10px ${colors.glow}` : 'none',
  };

  return (
    <div
      className={cn(
        'w-full h-full rounded-xl border transition-all duration-200',
        'backdrop-blur-[2px]'
      )}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Label badge positioned above the container */}
      <div
        className={cn(
          'absolute -top-5 left-3 flex items-center gap-2',
          'px-2.5 py-1 rounded-lg',
          'border backdrop-blur-sm',
          'transition-all duration-200',
          selected && 'scale-105'
        )}
        style={labelStyle}
      >
        {/* Icon */}
        <span className="text-sm">{data.icon}</span>

        {/* Label */}
        <span
          className="text-xs font-medium"
          style={{ color: selected || isHovered ? colors.primary : 'rgba(255, 255, 255, 0.7)' }}
        >
          {data.label}
        </span>

        {/* Count */}
        <span
          className="text-[10px] px-1.5 py-0.5 rounded"
          style={{
            backgroundColor: `${colors.primary}20`,
            color: selected || isHovered ? colors.primary : 'rgba(255, 255, 255, 0.5)',
          }}
        >
          {data.nodeCount}
        </span>
      </div>
    </div>
  );
});
