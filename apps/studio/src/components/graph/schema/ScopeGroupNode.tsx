'use client';

/**
 * ScopeGroupNode - Glass container for scope groups in Schema Mode
 *
 * Features (Task 5: TurboNode styling):
 * - Glass effect with scope-colored border glow
 * - Animated border on selection
 * - Premium label badge with glow
 * - NodeResizer for interactive resizing
 *
 * Scope Colors (hex values):
 * - Project: violet (#8b5cf6) - 📦
 * - Global: emerald (#10b981) - 🌍
 * - Shared: amber (#f59e0b) - 🎯
 */

import { memo, useState, useCallback } from 'react';
import { type NodeProps, type Node, NodeResizer } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import type { Scope } from '@novanet/core/types';

/**
 * Data interface for ScopeGroupNode
 */
export interface ScopeGroupData extends Record<string, unknown> {
  scope: Scope;
  label: string;
  icon: string;
  nodeCount: number;
}

/** Node type for ScopeGroupNode */
export type ScopeGroupNodeType = Node<ScopeGroupData, 'scopeGroup'>;

/**
 * Scope color configuration for glass effect
 */
const SCOPE_COLORS: Record<Scope, {
  primary: string;
  secondary: string;
  glow: string;
  bgGlow: string;
}> = {
  Project: {
    primary: '#8b5cf6',
    secondary: '#a78bfa',
    glow: 'rgba(139, 92, 246, 0.3)',
    bgGlow: 'rgba(139, 92, 246, 0.05)',
  },
  Global: {
    primary: '#10b981',
    secondary: '#34d399',
    glow: 'rgba(16, 185, 129, 0.3)',
    bgGlow: 'rgba(16, 185, 129, 0.05)',
  },
  Shared: {
    primary: '#f59e0b',
    secondary: '#fbbf24',
    glow: 'rgba(245, 158, 11, 0.3)',
    bgGlow: 'rgba(245, 158, 11, 0.05)',
  },
};

/**
 * ScopeGroupNode - Premium glass container for scope hierarchy
 */
export const ScopeGroupNode = memo(function ScopeGroupNode({
  data,
  selected,
}: NodeProps<ScopeGroupNodeType>) {
  const [isHovered, setIsHovered] = useState(false);

  const colors = SCOPE_COLORS[data.scope] || SCOPE_COLORS.Project;

  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => setIsHovered(false), []);

  // Container style with glass effect
  const containerStyle = {
    backgroundColor: colors.bgGlow,
    borderColor: selected
      ? colors.primary
      : isHovered
        ? `${colors.primary}80`
        : `${colors.primary}40`,
    boxShadow: selected
      ? `0 0 30px ${colors.glow}, inset 0 0 60px ${colors.bgGlow}`
      : isHovered
        ? `0 0 20px ${colors.glow}, inset 0 0 40px ${colors.bgGlow}`
        : `inset 0 0 30px ${colors.bgGlow}`,
  };

  // Label badge style
  const labelStyle = {
    background: `linear-gradient(135deg, ${colors.primary}20, ${colors.secondary}20)`,
    borderColor: `${colors.primary}60`,
    boxShadow: selected ? `0 0 15px ${colors.glow}` : 'none',
  };

  return (
    <div
      className={cn(
        'w-full h-full rounded-2xl border-2',
        'transition-[border-color,box-shadow,background-color] duration-300',
        selected && 'border-solid',
        !selected && 'border-dashed'
      )}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {/* Resizer - visible only when selected */}
      <NodeResizer
        isVisible={selected}
        minWidth={250}
        minHeight={150}
        lineClassName={cn('!border-2', `!border-[${colors.primary}]`)}
        handleClassName={cn(
          'w-3 h-3 rounded-full',
          'border-2',
          'transition-all duration-200'
        )}
        handleStyle={{
          backgroundColor: colors.primary,
          borderColor: colors.secondary,
          boxShadow: `0 0 8px ${colors.glow}`,
        }}
      />

      {/* Label badge positioned above the container */}
      <div
        className={cn(
          'absolute -top-8 left-4 flex items-center',
          gapTokens.spacious,
          'px-4 py-2 rounded-xl',
          'border bg-black/80',
          'transition-[border-color,box-shadow,background-color,transform] duration-300',
          selected && 'scale-105'
        )}
        style={labelStyle}
      >
        {/* Scope icon */}
        <span className="text-lg">{data.icon}</span>

        {/* Scope label */}
        <span
          className="text-sm font-bold tracking-wide"
          style={{ color: colors.primary }}
        >
          {data.label}
        </span>

        {/* Node count badge */}
        <span
          className="px-2 py-0.5 rounded-full text-xs font-semibold"
          style={{
            backgroundColor: `${colors.primary}30`,
            color: colors.secondary,
          }}
        >
          {data.nodeCount} types
        </span>

        {/* Status dot */}
        <div
          className={cn(
            'w-2 h-2 rounded-full',
            selected && 'animate-pulse'
          )}
          style={{
            backgroundColor: colors.primary,
            boxShadow: `0 0 6px ${colors.glow}`,
          }}
        />
      </div>
    </div>
  );
});
