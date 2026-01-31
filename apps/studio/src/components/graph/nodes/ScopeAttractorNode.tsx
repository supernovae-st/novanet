'use client';

/**
 * ScopeAttractorNode - Visible scope node for magnetic grouping
 *
 * Unlike ScopeGroupNode (container), this is a regular node that
 * acts as a gravitational center for its child nodes.
 *
 * Features:
 * - Circular node (120px) with scope color
 * - Shows emoji, label, and node count
 * - Glowing effect, especially when selected
 * - Hidden handles for edges
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface ScopeAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  color: string;
  nodeCount: number;
}

export type ScopeAttractorNodeType = Node<ScopeAttractorData, 'scopeAttractor'>;

const SCOPE_SIZE = 120;

export const ScopeAttractorNode = memo(function ScopeAttractorNode({
  data,
  selected,
}: NodeProps<ScopeAttractorNodeType>) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center rounded-full',
        'border-4 transition-[transform,border-color,box-shadow] duration-300',
        selected ? 'scale-110' : 'scale-100'
      )}
      style={{
        width: SCOPE_SIZE,
        height: SCOPE_SIZE,
        backgroundColor: `${data.color}20`,
        borderColor: data.color,
        boxShadow: selected
          ? `0 0 40px ${data.color}60, 0 0 80px ${data.color}30`
          : `0 0 20px ${data.color}40`,
      }}
      aria-label={`${data.label} scope with ${data.nodeCount} nodes`}
    >
      {/* Emoji */}
      <span className="text-3xl" aria-hidden="true">{data.emoji}</span>

      {/* Label */}
      <span
        className="text-sm font-bold mt-1"
        style={{ color: data.color }}
      >
        {data.label}
      </span>

      {/* Count badge */}
      <span
        className="text-xs mt-1 px-2 py-0.5 rounded-full"
        style={{
          backgroundColor: `${data.color}30`,
          color: data.color
        }}
      >
        {data.nodeCount}
      </span>

      {/* Handles for edges (hidden) */}
      <Handle
        type="source"
        position={Position.Bottom}
        className="opacity-0"
        aria-hidden="true"
      />
      <Handle
        type="target"
        position={Position.Top}
        className="opacity-0"
        aria-hidden="true"
      />
    </div>
  );
});
