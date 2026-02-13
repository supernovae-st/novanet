'use client';

/**
 * BlueprintOverlay - Visual indicator for schema mode nodes
 *
 * Adds blueprint-style visual elements to differentiate schema nodes from data nodes:
 * - Blue-tinted dark background
 * - Grid pattern (12-15% opacity, 12px spacing)
 * - Dashed border with custom spacing (SVG for precise control)
 * - Small diamond badge in corner
 *
 * Usage: Place inside node card when isSchemaMode is true
 */

import { memo } from 'react';

export interface BlueprintOverlayProps {
  /** Primary color for the grid and badge */
  color: string;
  /** Whether the node is selected (intensifies effect) */
  selected?: boolean;
  /** Border radius to match parent container */
  borderRadius?: number | string;
  /** Whether to show the badge */
  showBadge?: boolean;
}

// Blueprint blue tint color
const BLUEPRINT_BLUE = '#1e3a5f';
const BLUEPRINT_BLUE_LIGHT = '#2563eb';

/**
 * BlueprintOverlay - Schema mode visual indicator
 */
export const BlueprintOverlay = memo(function BlueprintOverlay({
  selected = false,
  borderRadius = 12,
  showBadge = true,
}: BlueprintOverlayProps) {
  const gridOpacity = selected ? 0.18 : 0.12;
  const bgOpacity = selected ? 0.25 : 0.15;
  const badgeOpacity = selected ? 1 : 0.8;
  const borderOpacity = selected ? 0.55 : 0.4;

  // Inset from edge (smaller = closer to edge)
  const inset = 6;
  const innerRadius = typeof borderRadius === 'number' ? Math.max(borderRadius - inset, 4) : borderRadius;

  return (
    <>
      {/* Blue tint background - blueprint paper effect */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius,
          background: `linear-gradient(135deg, ${BLUEPRINT_BLUE}${Math.round(bgOpacity * 255).toString(16).padStart(2, '0')}, ${BLUEPRINT_BLUE}${Math.round(bgOpacity * 0.5 * 255).toString(16).padStart(2, '0')})`,
        }}
      />

      {/* Grid pattern - blueprint style (more visible) */}
      <div
        className="absolute inset-0 pointer-events-none"
        style={{
          borderRadius,
          backgroundImage: `
            linear-gradient(${BLUEPRINT_BLUE_LIGHT}${Math.round(gridOpacity * 255).toString(16).padStart(2, '0')} 1px, transparent 1px),
            linear-gradient(90deg, ${BLUEPRINT_BLUE_LIGHT}${Math.round(gridOpacity * 255).toString(16).padStart(2, '0')} 1px, transparent 1px)
          `,
          backgroundSize: '12px 12px',
        }}
      />

      {/* Dashed border - closer to edge */}
      <div
        className="absolute pointer-events-none"
        style={{
          top: inset,
          left: inset,
          right: inset,
          bottom: inset,
          borderRadius: innerRadius,
          border: `2px dashed ${BLUEPRINT_BLUE_LIGHT}`,
          opacity: borderOpacity,
        }}
      />

      {/* Blueprint badge - small diamond in corner */}
      {showBadge && (
        <div
          className="absolute top-2 right-2 pointer-events-none z-10"
          style={{
            opacity: badgeOpacity,
          }}
        >
          <div
            className="w-5 h-5 flex items-center justify-center text-[11px] font-bold rounded"
            style={{
              color: BLUEPRINT_BLUE_LIGHT,
              background: `${BLUEPRINT_BLUE}60`,
              textShadow: `0 0 6px ${BLUEPRINT_BLUE_LIGHT}`,
              border: `1px solid ${BLUEPRINT_BLUE_LIGHT}40`,
            }}
          >
            ◇
          </div>
        </div>
      )}
    </>
  );
});
