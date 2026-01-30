'use client';

/**
 * ScopeGroupNode - Container node for scope groups in Schema Mode
 *
 * Features:
 * - Dashed border with scope-specific color (violet/emerald/amber)
 * - Scope label with emoji and node count badge
 * - NodeResizer for resizable groups
 * - Glassmorphism styling matching NovaNet design
 *
 * Scope Colors:
 * - Project: violet (📦)
 * - Global: emerald (🌍)
 * - Shared: amber (🎯)
 */

import { memo } from 'react';
import { type NodeProps, type Node, NodeResizer } from '@xyflow/react';
import { cn } from '@/lib/utils';
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
 * Scope color mapping - matches plan requirements
 * Project = violet, Global = emerald, Shared = amber
 */
const SCOPE_COLORS: Record<Scope, { border: string; bg: string }> = {
  Project: {
    border: 'border-violet-500/50',
    bg: 'bg-violet-500/5',
  },
  Global: {
    border: 'border-emerald-500/50',
    bg: 'bg-emerald-500/5',
  },
  Shared: {
    border: 'border-amber-500/50',
    bg: 'bg-amber-500/5',
  },
};

/**
 * ScopeGroupNode - Top-level container for scope hierarchy
 *
 * Used in Schema Mode to group subcategories and nodes by scope.
 * Features NodeResizer for manual size adjustment when selected.
 */
export const ScopeGroupNode = memo(function ScopeGroupNode({
  data,
  selected,
}: NodeProps<ScopeGroupNodeType>) {
  const colorConfig = SCOPE_COLORS[data.scope] || {
    border: 'border-gray-500/50',
    bg: 'bg-gray-500/5',
  };

  return (
    <div
      className={cn(
        'w-full h-full rounded-xl border-2 border-dashed',
        colorConfig.border,
        colorConfig.bg,
        selected && 'ring-2 ring-white/20'
      )}
    >
      {/* Resizer - visible only when selected */}
      <NodeResizer
        isVisible={selected}
        minWidth={200}
        minHeight={100}
        lineClassName="border-white/20"
        handleClassName="w-2 h-2 bg-white/50 border border-white/20"
      />

      {/* Label badge positioned above the container */}
      <div className="absolute -top-7 left-3 flex items-center gap-2 px-2 py-1 rounded-md bg-black/80 backdrop-blur-sm">
        <span className="text-sm font-semibold text-white/90">
          {data.icon} {data.label}
        </span>
        <span className="text-xs text-white/50">{data.nodeCount} types</span>
      </div>
    </div>
  );
});
