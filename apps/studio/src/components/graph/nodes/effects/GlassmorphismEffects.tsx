'use client';

/**
 * GlassmorphismEffects - Selected state visual effects
 *
 * Provides premium glassmorphism and skeuomorphism effects for selected nodes:
 * - Top bevel highlight (skeuomorphism)
 * - Top gradient reflection (glassmorphism)
 * - Shimmer animation (Apple-style shine)
 *
 * Used by: StructuralNode, SharedLayerNode, SchemaNode, ProjectNode
 */

import { memo, useMemo } from 'react';
import { NODE_DESIGN } from '@/config/constants';

export interface GlassmorphismEffectsProps {
  /** Border radius to match parent container */
  borderRadius?: number;
  /** Whether to use circular style (for SharedLayerNode) */
  isCircular?: boolean;
}

/**
 * GlassmorphismEffects - Premium visual effects for selected nodes
 */
export const GlassmorphismEffects = memo(function GlassmorphismEffects({
  borderRadius = NODE_DESIGN.radius.innerSelected,
  isCircular = false,
}: GlassmorphismEffectsProps) {
  const radiusClass = isCircular ? 'rounded-full' : '';
  const effectRadius = isCircular ? undefined : borderRadius;

  // Memoize styles to prevent object recreation on every render
  const bevelStyle = useMemo(() => ({
    borderRadius: effectRadius,
    background: NODE_DESIGN.gradients.bevelHighlight,
  }), [effectRadius]);

  const reflectionStyle = useMemo(() => ({
    borderRadius: effectRadius,
    background: NODE_DESIGN.gradients.glassReflection,
  }), [effectRadius]);

  const shimmerStyle = useMemo(() => ({
    borderRadius: effectRadius,
    background: NODE_DESIGN.gradients.shimmer,
    backgroundSize: '200% 100%',
  }), [effectRadius]);

  return (
    <>
      {/* Skeuomorphism: Top bevel highlight */}
      <div
        className={`absolute inset-x-0 top-0 h-[2px] pointer-events-none ${radiusClass}`}
        style={bevelStyle}
      />

      {/* Glassmorphism: Subtle top gradient reflection */}
      <div
        className={`absolute inset-x-0 top-0 h-1/4 pointer-events-none ${radiusClass}`}
        style={reflectionStyle}
      />

      {/* Shimmer effect overlay - Very subtle Apple-style shine */}
      <div
        className={`absolute inset-0 pointer-events-none animate-shimmer overflow-hidden ${radiusClass}`}
        style={shimmerStyle}
      />
    </>
  );
});
