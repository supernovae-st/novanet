'use client';

/**
 * LayerAttractorNode - Visible layer node for magnetic grouping
 *
 * Features:
 * - Circular node (90px)
 * - Shows emoji, label, and dual count (typeCount / loadedCount)
 * - Uses parent realm's color (slightly dimmer)
 * - Hidden handles for edges
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';

export interface LayerAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  realmKey: string;
  color: string;
  typeCount: number;
  loadedCount: number;
}

export type LayerAttractorNodeType = Node<LayerAttractorData, 'layerAttractor'>;

const LAYER_SIZE = 90;

export const LayerAttractorNode = memo(function LayerAttractorNode({
  data,
  selected,
}: NodeProps<LayerAttractorNodeType>) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center rounded-full',
        'border-2 transition-[transform,border-color,box-shadow] duration-300',
        selected ? 'scale-110' : 'scale-100'
      )}
      style={{
        width: LAYER_SIZE,
        height: LAYER_SIZE,
        backgroundColor: `${data.color}15`,
        borderColor: `${data.color}80`,
        boxShadow: selected
          ? `0 0 30px ${data.color}50`
          : `0 0 15px ${data.color}30`,
      }}
      aria-label={`${data.label} layer: ${data.typeCount} types, ${data.loadedCount} loaded`}
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
