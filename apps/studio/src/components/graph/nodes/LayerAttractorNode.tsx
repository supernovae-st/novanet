'use client';

/**
 * LayerAttractorNode - Unified card design for Layer attractor nodes
 *
 * Used in magnetic layout mode as sub-grouping center for child nodes.
 * Uses CardShell + AttractorCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import type { Layer } from '@novanet/core/types';
import { LAYER_COLORS } from '@/design/colors/generated';
import { CardShell, AttractorCardContent } from './card';

// =============================================================================
// Types
// =============================================================================

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

// =============================================================================
// Main Component
// =============================================================================

export const LayerAttractorNode = memo(function LayerAttractorNode({
  data,
  selected = false,
}: NodeProps<LayerAttractorNodeType>) {
  const { key, label, realmKey, typeCount = 0, loadedCount = 0 } = data;

  // Get design system color from generated taxonomy (fallback to data.color for safety)
  const layerKey = key as Layer;
  const primaryColor = LAYER_COLORS[layerKey]?.color || data.color || '#64748b';

  const colors = useMemo(() => ({
    primary: primaryColor,
    secondary: primaryColor,
  }), [primaryColor]);

  // Prepare data for AttractorCardContent
  const contentData = useMemo(() => ({
    key,
    label,
    typeCount,
    loadedCount,
    realmKey,
  }), [key, label, typeCount, loadedCount, realmKey]);

  return (
    <>
      {/* Hidden handles for edges (attractor nodes use invisible connection points) */}
      <Handle type="target" position={Position.Top} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Bottom} className="!opacity-0 !w-1 !h-1" />

      <CardShell
        colors={colors}
        selected={selected}
        width={240}
        minHeight={120}
        showHandles={false}
        showBlueprintOverlay={true}
        ariaLabel={`${label} layer: ${typeCount} types, ${loadedCount} loaded`}
        renderContent={(ctx) => (
          <AttractorCardContent data={contentData} variant="layer" {...ctx} />
        )}
      />
    </>
  );
});
