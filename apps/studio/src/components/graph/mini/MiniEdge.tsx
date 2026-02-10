'use client';

/**
 * MiniEdge - Simplified edge for sidebar ego graph
 *
 * Features:
 * - Thin line with arc family color
 * - Subtle arrow marker
 * - No labels (too small)
 */

import { memo } from 'react';
import { BaseEdge, getStraightPath, type EdgeProps } from '@xyflow/react';

export interface MiniEdgeData {
  family?: 'ownership' | 'localization' | 'semantic' | 'generation' | 'mining';
  [key: string]: unknown;
}

const ARC_FAMILY_COLORS: Record<string, string> = {
  ownership: '#3b82f6',    // blue
  localization: '#22c55e', // green
  semantic: '#f97316',     // orange
  generation: '#8b5cf6',   // purple
  mining: '#ec4899',       // pink
};

const DEFAULT_COLOR = '#6366f1';

export const MiniEdge = memo(function MiniEdge({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  data,
}: EdgeProps) {
  const [edgePath] = getStraightPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  const edgeData = data as MiniEdgeData | undefined;
  const color = edgeData?.family ? ARC_FAMILY_COLORS[edgeData.family] : DEFAULT_COLOR;

  return (
    <BaseEdge
      id={id}
      path={edgePath}
      style={{
        stroke: color,
        strokeWidth: 1.5,
        strokeOpacity: 0.6,
      }}
    />
  );
});

export default MiniEdge;
