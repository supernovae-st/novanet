'use client';

/**
 * MagneticEdge - Faint edge for IN_SUBCATEGORY relationships
 *
 * These edges represent the taxonomy grouping but should be
 * visually subtle to not overwhelm the actual business relationships.
 */

import { memo } from 'react';
import { BaseEdge, getStraightPath, type EdgeProps } from '@xyflow/react';

export const MagneticEdge = memo(function MagneticEdge({
  sourceX,
  sourceY,
  targetX,
  targetY,
  style,
}: EdgeProps) {
  const [edgePath] = getStraightPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  return (
    <BaseEdge
      path={edgePath}
      style={{
        ...style,
        stroke: '#ffffff15',
        strokeWidth: 1,
        strokeDasharray: '4 4',
      }}
    />
  );
});
