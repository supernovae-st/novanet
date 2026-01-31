'use client';

/**
 * SelectionPulseRing - Animated selection indicator
 *
 * Creates a double-ring pulse animation when a node is selected.
 * Uses the node's primary color for consistent theming.
 *
 * Used by: StructuralNode, LocaleKnowledgeNode, SchemaNode, ProjectNode
 */

import { memo } from 'react';
import { NODE_DESIGN } from '@/config/constants';

export interface SelectionPulseRingProps {
  /** Primary color for the ring */
  color: string;
  /** Border radius (default: 16 for cards, 9999 for circular) */
  borderRadius?: number;
}

/**
 * SelectionPulseRing - Animated double-ring selection effect
 */
export const SelectionPulseRing = memo(function SelectionPulseRing({
  color,
  borderRadius = NODE_DESIGN.radius.outer + 2,
}: SelectionPulseRingProps) {
  return (
    <>
      {/* Primary pulse ring */}
      <div
        className="absolute inset-0 animate-selection-ping pointer-events-none"
        style={{
          borderRadius,
          border: `2px solid ${color}`,
          boxShadow: NODE_DESIGN.shadows.selectionPulse(color),
        }}
      />
      {/* Secondary delayed pulse ring */}
      <div
        className="absolute inset-0 animate-selection-ping-delayed pointer-events-none"
        style={{
          borderRadius,
          border: `2px solid ${color}`,
          boxShadow: NODE_DESIGN.shadows.selectionPulseDelayed(color),
        }}
      />
    </>
  );
});
