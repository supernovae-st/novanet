'use client';

/**
 * MiniNode - Simplified node for sidebar ego graph
 *
 * Features:
 * - Small circular shape (32x32px)
 * - Color by layer gradient
 * - Realm indicator ring
 * - Hover highlight
 * - Click to navigate
 */

import { memo } from 'react';
import { Handle, Position, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { getLayerGradientColors } from '@/design/nodeColors';
import type { Layer, Realm } from '@novanet/core/types';

export interface MiniNodeData {
  label: string;
  layer?: Layer;
  realm?: Realm;
  isCenter?: boolean;
  [key: string]: unknown;
}

const REALM_COLORS: Record<Realm, string> = {
  shared: '#2aa198', // solarized cyan
  org: '#6c71c4',    // solarized purple
};

export const MiniNode = memo(function MiniNode({
  data,
  selected,
}: NodeProps) {
  const { label, layer, realm, isCenter } = data as MiniNodeData;
  const colors = getLayerGradientColors(layer);
  const realmColor = realm ? REALM_COLORS[realm] : '#6366f1';

  return (
    <>
      {/* Hidden handles for edges */}
      <Handle type="target" position={Position.Left} className="!opacity-0 !w-0 !h-0" />
      <Handle type="source" position={Position.Right} className="!opacity-0 !w-0 !h-0" />

      {/* Node circle */}
      <div
        className={cn(
          'relative flex items-center justify-center',
          'w-8 h-8 rounded-full',
          'transition-all duration-150',
          'cursor-pointer',
          selected && 'ring-2 ring-white ring-offset-1 ring-offset-transparent',
          isCenter && 'w-10 h-10'
        )}
        style={{
          background: `linear-gradient(135deg, ${colors.primary}, ${colors.secondary})`,
          boxShadow: isCenter
            ? `0 0 12px ${colors.primary}60, inset 0 0 8px rgba(255,255,255,0.2)`
            : `0 0 8px ${colors.primary}40`,
        }}
        title={label}
      >
        {/* Realm indicator ring */}
        <div
          className="absolute inset-0 rounded-full"
          style={{
            border: `2px solid ${realmColor}`,
            opacity: 0.6,
          }}
        />

        {/* Center dot for center node */}
        {isCenter && (
          <div className="w-2 h-2 rounded-full bg-white/80" />
        )}
      </div>

      {/* Label (only for center node) */}
      {isCenter && (
        <div
          className="absolute -bottom-5 left-1/2 -translate-x-1/2 whitespace-nowrap"
          style={{ fontSize: '9px', color: 'rgba(255,255,255,0.6)' }}
        >
          {label.length > 12 ? `${label.slice(0, 12)}...` : label}
        </div>
      )}
    </>
  );
});

export default MiniNode;
