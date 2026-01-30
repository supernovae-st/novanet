'use client';

/**
 * SelectionHalo - Visual selection indicator for graph nodes
 *
 * Features:
 * - Pulsing glow/ring effect around selected nodes
 * - Uses boxShadow for the halo effect
 * - Matches glassmorphism design system (white/10 borders, indigo accents)
 * - Only renders when isSelected is true (performance)
 *
 * @see ADR-007: Glassmorphism UI Theme
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';

export interface SelectionHaloProps {
  /** Whether the node is selected */
  isSelected: boolean;
  /** Accent color for the halo (hex format) - defaults to indigo */
  color?: string;
  /** Additional CSS classes */
  className?: string;
}

/**
 * Selection halo ring that renders around selected nodes
 *
 * Designed to overlay the node content with absolute positioning.
 * Parent must have `position: relative`.
 */
export const SelectionHalo = memo(function SelectionHalo({
  isSelected,
  color = '#6366f1', // Indigo-500 (design system accent)
  className,
}: SelectionHaloProps) {
  // Performance: Early return if not selected
  if (!isSelected) return null;

  return (
    <div
      className={cn(
        'absolute inset-0 rounded-xl pointer-events-none z-10',
        'animate-breathing',
        className
      )}
      style={{
        // Multi-layer boxShadow for rich glow effect:
        // 1. Inner ring (3px solid border effect)
        // 2. Medium glow (20px spread)
        // 3. Outer subtle glow (40px spread with transparency)
        boxShadow: `
          0 0 0 3px ${color},
          0 0 20px ${color}40,
          0 0 40px ${color}20
        `,
      }}
      aria-hidden="true"
    />
  );
});
