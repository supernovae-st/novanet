'use client';

/**
 * GlassmorphismEffects - Selected state visual effects
 *
 * Provides premium glassmorphism and skeuomorphism effects for selected nodes:
 * - Top bevel highlight (skeuomorphism)
 * - Top gradient reflection (glassmorphism)
 * - Shimmer animation (Apple-style shine)
 *
 * Used by: StructuralNode, LocaleKnowledgeNode, SchemaNode, ProjectNode
 */

import { memo } from 'react';
import { NODE_DESIGN } from '@/config/constants';

export interface GlassmorphismEffectsProps {
  /** Border radius to match parent container */
  borderRadius?: number;
  /** Whether to use circular style (for LocaleKnowledgeNode) */
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

  return (
    <>
      {/* Skeuomorphism: Top bevel highlight */}
      <div
        className={`absolute inset-x-0 top-0 h-[2px] pointer-events-none ${radiusClass}`}
        style={{
          borderRadius: isCircular ? undefined : borderRadius,
          background: NODE_DESIGN.gradients.bevelHighlight,
        }}
      />

      {/* Glassmorphism: Subtle top gradient reflection */}
      <div
        className={`absolute inset-x-0 top-0 h-1/4 pointer-events-none ${radiusClass}`}
        style={{
          borderRadius: isCircular ? undefined : borderRadius,
          background: NODE_DESIGN.gradients.glassReflection,
        }}
      />

      {/* Shimmer effect overlay - Very subtle Apple-style shine */}
      <div
        className={`absolute inset-0 pointer-events-none animate-shimmer overflow-hidden ${radiusClass}`}
        style={{
          borderRadius: isCircular ? undefined : borderRadius,
          background: NODE_DESIGN.gradients.shimmer,
          backgroundSize: '200% 100%',
        }}
      />
    </>
  );
});
