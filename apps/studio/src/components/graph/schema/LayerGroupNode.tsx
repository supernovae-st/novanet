'use client';

/**
 * LayerGroupNode - Nested glass container for layer groups
 *
 * Features:
 * - Subtle glass effect inheriting parent realm color
 * - Compact label with hover interaction
 * - NodeResizer for interactive resizing (like RealmGroupNode)
 * - Draggable within parent realm container
 * - Smooth transitions
 *
 * Nested inside RealmGroupNode - inherits realm coloring
 */

import { memo, useState, useCallback } from 'react';
import { type NodeProps, type Node, NodeResizer } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { Realm } from '@novanet/core/types';

/**
 * Data interface for LayerGroupNode
 */
export interface LayerGroupData extends Record<string, unknown> {
  realm: Realm;
  layer: string;
  label: string;
  icon: string;
  nodeCount: number;
}

/** Node type for LayerGroupNode */
export type LayerGroupNodeType = Node<LayerGroupData, 'layerGroup'>;

/**
 * Realm color configuration (subset of parent colors)
 */
const REALM_COLORS: Record<Realm, {
  primary: string;
  secondary: string;
  glow: string;
}> = {
  project: {
    primary: '#8b5cf6',
    secondary: '#a78bfa',
    glow: 'rgba(139, 92, 246, 0.2)',
  },
  global: {
    primary: '#10b981',
    secondary: '#34d399',
    glow: 'rgba(16, 185, 129, 0.2)',
  },
  shared: {
    primary: '#f59e0b',
    secondary: '#fbbf24',
    glow: 'rgba(245, 158, 11, 0.2)',
  },
};

/**
 * LayerGroupNode - Nested container within a realm
 */
export const LayerGroupNode = memo(function LayerGroupNode({
  data,
  selected,
}: NodeProps<LayerGroupNodeType>) {
  const [isHovered, setIsHovered] = useState(false);

  const colors = REALM_COLORS[data.realm] || REALM_COLORS.project;

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
      ? `0 0 20px ${colors.glow}, inset 0 0 20px ${colors.glow}`
      : isHovered
        ? `0 0 12px ${colors.glow}, inset 0 0 15px ${colors.glow}`
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
        'w-full h-full rounded-xl border',
        'transition-[border-color,box-shadow,background-color] duration-200'
      )}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Resizer - visible only when selected */}
      <NodeResizer
        isVisible={selected}
        minWidth={200}
        minHeight={100}
        lineClassName="!border-[1.5px]"
        handleClassName={cn(
          'w-2.5 h-2.5 rounded-full',
          'border-2',
          'transition duration-200'
        )}
        handleStyle={{
          backgroundColor: colors.primary,
          borderColor: colors.secondary,
          boxShadow: `0 0 6px ${colors.glow}`,
        }}
      />

      {/* Label badge positioned above the container */}
      <div
        className={cn(
          'absolute -top-5 left-3 flex items-center',
          gapTokens.default,
          'px-2.5 py-1 rounded-lg',
          'border bg-black/70',
          'transition duration-200',
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
