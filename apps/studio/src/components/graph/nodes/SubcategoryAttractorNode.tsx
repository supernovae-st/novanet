'use client';

/**
 * SubcategoryAttractorNode - Visible subcategory node for magnetic grouping
 *
 * Features:
 * - Circular node (90px)
 * - Shows emoji, label, and dual count (typeCount / loadedCount)
 * - Uses parent scope's color (slightly dimmer)
 * - Hidden handles for edges
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface SubcategoryAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  scopeKey: string;
  color: string;
  typeCount: number;
  loadedCount: number;
}

export type SubcategoryAttractorNodeType = Node<SubcategoryAttractorData, 'subcategoryAttractor'>;

const SUBCAT_SIZE = 90;

export const SubcategoryAttractorNode = memo(function SubcategoryAttractorNode({
  data,
  selected,
}: NodeProps<SubcategoryAttractorNodeType>) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center rounded-full',
        'border-2 transition-[transform,border-color,box-shadow] duration-300',
        selected ? 'scale-110' : 'scale-100'
      )}
      style={{
        width: SUBCAT_SIZE,
        height: SUBCAT_SIZE,
        backgroundColor: `${data.color}15`,
        borderColor: `${data.color}80`,
        boxShadow: selected
          ? `0 0 30px ${data.color}50`
          : `0 0 15px ${data.color}30`,
      }}
      aria-label={`${data.label} subcategory: ${data.typeCount} types, ${data.loadedCount} loaded`}
    >
      {/* Emoji */}
      <span className="text-xl" aria-hidden="true">{data.emoji}</span>

      {/* Label */}
      <span
        className="text-xs font-semibold"
        style={{ color: data.color }}
      >
        {data.label}
      </span>

      {/* Count */}
      <span
        className="text-[10px]"
        style={{ color: `${data.color}90` }}
      >
        {data.typeCount} &middot; {data.loadedCount}
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
