'use client';

/**
 * BundledEdge - Collapsed representation for 4+ parallel edges
 *
 * When multiple edges connect the same two nodes:
 * - 2-3 edges: Render individually with curved fan-out
 * - 4+ edges: Collapse into single bundle with count badge
 *
 * Features:
 * - Collapsed state: Single thick dashed line with count badge
 * - Expanded state (on hover): Fan-out showing all individual edges
 * - Smooth transition between states
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */

import { memo, useCallback } from 'react';
import type { Edge } from '@xyflow/react';
import { generateCurvedPath, generateParallelPath, getPathMidpoint } from './EdgeUtils';

// =============================================================================
// Types
// =============================================================================

interface Point {
  x: number;
  y: number;
}

export interface BundledEdgeProps {
  /** Edges in this bundle */
  edges: Edge[];
  /** Source node position (center) */
  source: Point;
  /** Target node position (center) */
  target: Point;
  /** Whether the bundle is currently expanded (showing all edges) */
  isExpanded: boolean;
  /** Callback when hover state changes */
  onHover: (expanded: boolean) => void;
  /** Optional: Render function for individual edges when expanded */
  renderEdge?: (edge: Edge, path: string, index: number) => React.ReactNode;
}

// =============================================================================
// Constants
// =============================================================================

const COLORS = {
  bundleLine: '#64748b',      // Slate 500
  badgeBackground: '#1e293b', // Slate 800
  badgeBorder: '#475569',     // Slate 600
  badgeText: '#e2e8f0',       // Slate 200
  hoverGlow: '#3b82f6',       // Blue 500
};

const SIZES = {
  baseStrokeWidth: 3,
  strokeWidthPerEdge: 0.5,
  maxStrokeWidth: 8,
  badgeWidth: 28,
  badgeHeight: 22,
  badgeRadius: 6,
  fontSize: 12,
};

// =============================================================================
// Component
// =============================================================================

export const BundledEdge = memo(function BundledEdge({
  edges,
  source,
  target,
  isExpanded,
  onHover,
  renderEdge,
}: BundledEdgeProps) {
  const count = edges.length;

  // Memoized handlers
  const handleMouseEnter = useCallback(() => onHover(true), [onHover]);
  const handleMouseLeave = useCallback(() => onHover(false), [onHover]);

  // Generate main bundle path
  const bundlePath = generateCurvedPath(source, target);
  const midpoint = getPathMidpoint(bundlePath);

  // If expanded or small group, render individual edges with fan-out
  if (isExpanded || count <= 3) {
    return (
      <g
        className="bundled-edge bundled-edge--expanded"
        onMouseLeave={handleMouseLeave}
      >
        {edges.map((edge, index) => {
          const path = generateParallelPath(source, target, index, count);

          if (renderEdge) {
            return renderEdge(edge, path, index);
          }

          // Default edge rendering
          return (
            <g key={edge.id} className="bundled-edge__individual">
              {/* Edge glow */}
              <path
                d={path}
                fill="none"
                stroke={COLORS.hoverGlow}
                strokeWidth={4}
                strokeOpacity={0.2}
                style={{ filter: 'blur(4px)' }}
              />
              {/* Edge line */}
              <path
                d={path}
                fill="none"
                stroke={COLORS.bundleLine}
                strokeWidth={2}
                strokeLinecap="round"
              />
              {/* Arrowhead */}
              <circle
                r={4}
                fill={COLORS.bundleLine}
              >
                <animateMotion
                  dur="0.01s"
                  fill="freeze"
                  keyPoints="1;1"
                  keyTimes="0;1"
                >
                  <mpath href={`#parallel-path-${edge.id}`} />
                </animateMotion>
              </circle>
              {/* Hidden path for animation reference */}
              <path
                id={`parallel-path-${edge.id}`}
                d={path}
                fill="none"
                stroke="none"
              />
            </g>
          );
        })}
      </g>
    );
  }

  // Render collapsed bundle
  const strokeWidth = Math.min(
    SIZES.baseStrokeWidth + count * SIZES.strokeWidthPerEdge,
    SIZES.maxStrokeWidth
  );

  return (
    <g
      className="bundled-edge bundled-edge--collapsed"
      onMouseEnter={handleMouseEnter}
      style={{ cursor: 'pointer' }}
    >
      {/* Background glow (visible on hover via CSS) */}
      <path
        className="bundled-edge__glow"
        d={bundlePath}
        fill="none"
        stroke={COLORS.hoverGlow}
        strokeWidth={strokeWidth + 6}
        strokeOpacity={0}
        style={{ filter: 'blur(8px)', transition: 'stroke-opacity 0.2s' }}
      />

      {/* Bundle line */}
      <path
        d={bundlePath}
        fill="none"
        stroke={COLORS.bundleLine}
        strokeWidth={strokeWidth}
        strokeDasharray="10 5"
        strokeLinecap="round"
        opacity={0.7}
        style={{ transition: 'opacity 0.2s, stroke-width 0.2s' }}
      />

      {/* Count badge */}
      {midpoint && (
        <g
          transform={`translate(${midpoint.x}, ${midpoint.y})`}
          className="bundled-edge__badge"
        >
          {/* Badge shadow */}
          <rect
            x={-SIZES.badgeWidth / 2 + 1}
            y={-SIZES.badgeHeight / 2 + 1}
            width={SIZES.badgeWidth}
            height={SIZES.badgeHeight}
            rx={SIZES.badgeRadius}
            fill="rgba(0,0,0,0.3)"
          />
          {/* Badge background */}
          <rect
            x={-SIZES.badgeWidth / 2}
            y={-SIZES.badgeHeight / 2}
            width={SIZES.badgeWidth}
            height={SIZES.badgeHeight}
            rx={SIZES.badgeRadius}
            fill={COLORS.badgeBackground}
            stroke={COLORS.badgeBorder}
            strokeWidth={1.5}
          />
          {/* Count text */}
          <text
            textAnchor="middle"
            dominantBaseline="central"
            fill={COLORS.badgeText}
            fontSize={SIZES.fontSize}
            fontWeight="bold"
            fontFamily="system-ui, -apple-system, sans-serif"
          >
            {count}
          </text>
        </g>
      )}

      {/* Hover trigger area (invisible, larger hit area) */}
      <path
        d={bundlePath}
        fill="none"
        stroke="transparent"
        strokeWidth={strokeWidth + 20}
      />
    </g>
  );
});

// =============================================================================
// CSS for hover effects (add to your global styles or CSS module)
// =============================================================================

/*
.bundled-edge--collapsed:hover .bundled-edge__glow {
  stroke-opacity: 0.4 !important;
}

.bundled-edge--collapsed:hover path:not(.bundled-edge__glow) {
  opacity: 1 !important;
}

.bundled-edge--collapsed:hover .bundled-edge__badge rect {
  stroke: #3b82f6;
}
*/
