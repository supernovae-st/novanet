'use client';

/**
 * BaseNodeWrapper - Foundation component for all custom node types
 *
 * Premium design foundation with:
 * - GlowingBorder integration for Nika-style effects
 * - Layered shadow system with depth perception
 * - Consistent border radius via design tokens
 * - Semantic transition timing from design system
 * - WCAG 2.1 AA compliant focus indicators
 * - Size variants (xs through xl)
 * - Focus mode support (dimmed state)
 */

import { memo, useState, useCallback } from 'react';
import { Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { glassClasses, durations, easing } from '@/design/tokens';
import { GlowingBorder } from '@/components/ui/GlowingBorder';
import { SelectionHalo } from '../SelectionHalo';
import { EdgeConnectionHalo, type EdgeConnectionRole } from '../EdgeConnectionHalo';
import {
  useUIStore,
  selectHoveredNodeId,
  selectHoveredEdgeId,
  selectSelectedNodeId,
  selectHoveredConnectedNodeIds,
  selectSelectedEdgeData,
} from '@/stores/uiStore';
import type { NodeType } from '@/types';

export interface BaseNodeData extends Record<string, unknown> {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  icon?: string;
  description?: string;
  category?: string;
  /** Connection count for size scaling */
  connectionCount?: number;
  /** Whether this node is fully dimmed (focus mode - 15% opacity) */
  dimmed?: boolean;
  /** Whether this node is lightly dimmed (hover mode - 25% opacity) */
  hoverDimmed?: boolean;
  /** Whether this node is in schema mode (blueprint styling) */
  isSchemaMode?: boolean;
  /** BCP-47 locale code for locale-specific nodes (*Native, Knowledge atoms, Locale layer) */
  locale?: string;
}

export interface BaseNodeWrapperProps {
  data: BaseNodeData;
  selected?: boolean;
  /** Primary color for glow */
  color: string;
  /** Secondary color for gradient */
  colorSecondary?: string;
  /** Node size variant */
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  /** Custom shape class */
  shapeClass?: string;
  children: React.ReactNode;
}

// ============================================================================
// SIZE VARIANTS - Consistent card dimensions
// ============================================================================
const SIZE_CLASSES = {
  xs: 'min-w-[100px] max-w-[140px]',
  sm: 'min-w-[120px] max-w-[180px]',
  md: 'min-w-[160px] max-w-[220px]',
  lg: 'min-w-[200px] max-w-[280px]',
  xl: 'min-w-[240px] max-w-[320px]',
} as const;

// ============================================================================
// PADDING VARIANTS - Semantic spacing by size
// ============================================================================
const PADDING_CLASSES = {
  xs: 'px-3 py-2',        // Compact: 12px/8px
  sm: 'px-3.5 py-2.5',    // Tight: 14px/10px
  md: 'px-4 py-3',        // Standard: 16px/12px (default)
  lg: 'px-5 py-3.5',      // Comfortable: 20px/14px
  xl: 'px-6 py-4',        // Spacious: 24px/16px
} as const;

// ============================================================================
// BORDER RADIUS - Derived from design tokens for consistency
// ============================================================================
const BORDER_RADIUS = {
  card: 14,               // Standard card radius (matches rounded-xl)
  cardLg: 18,             // Large cards
  pill: 9999,             // Full pill shape
} as const;

// ============================================================================
// SHADOW SYSTEM - Layered depth perception
// ============================================================================
const NODE_SHADOWS = {
  // Ambient shadow - always visible for depth
  ambient: '0 2px 8px -2px rgba(0, 0, 0, 0.4)',
  // Default resting state - subtle elevation
  default: `
    0 1px 2px -1px rgba(0, 0, 0, 0.3),
    0 4px 8px -2px rgba(0, 0, 0, 0.25),
    0 8px 16px -4px rgba(0, 0, 0, 0.15)
  `.trim().replace(/\s+/g, ' '),
  // Hovered - lifted elevation
  hover: `
    0 2px 4px -1px rgba(0, 0, 0, 0.35),
    0 8px 16px -4px rgba(0, 0, 0, 0.3),
    0 16px 32px -8px rgba(0, 0, 0, 0.2)
  `.trim().replace(/\s+/g, ' '),
  // Selected - prominent with color accent (applied via GlowingBorder)
  selected: `
    0 4px 8px -2px rgba(0, 0, 0, 0.4),
    0 12px 24px -6px rgba(0, 0, 0, 0.35),
    0 24px 48px -12px rgba(0, 0, 0, 0.25)
  `.trim().replace(/\s+/g, ' '),
} as const;

// ============================================================================
// TRANSITION TIMING - Semantic transitions from design system
// ============================================================================
const TRANSITION_STYLES = {
  // Transform transitions (scale, translate)
  transform: `transform ${durations.normal}ms ${easing.out}`,
  // Opacity transitions (fade in/out)
  opacity: `opacity ${durations.fast}ms ${easing.out}`,
  // Shadow transitions (elevation changes)
  shadow: `box-shadow ${durations.slow}ms ${easing.out}`,
  // All common properties combined
  all: `
    opacity ${durations.fast}ms ${easing.out},
    transform ${durations.normal}ms ${easing.out},
    box-shadow ${durations.slow}ms ${easing.out},
    filter ${durations.normal}ms ${easing.out}
  `.trim().replace(/\s+/g, ' '),
} as const;

export const BaseNodeWrapper = memo(function BaseNodeWrapper({
  data,
  selected = false,
  color,
  colorSecondary,
  size = 'md',
  shapeClass = 'rounded-xl',
  children,
}: BaseNodeWrapperProps) {
  const [isHovered, setIsHovered] = useState(false);
  const [isFocused, setIsFocused] = useState(false);

  // ==========================================================================
  // DIRECT STORE SUBSCRIPTION - Bypass React Flow's broken data updates
  // ==========================================================================
  // React Flow's internal memoization blocks data prop updates for nodes.
  // We subscribe directly to Zustand store to get fresh hover state.
  const hoveredNodeId = useUIStore(selectHoveredNodeId);
  const hoveredEdgeId = useUIStore(selectHoveredEdgeId);
  const selectedNodeId = useUIStore(selectSelectedNodeId);
  const hoveredConnectedNodeIds = useUIStore(selectHoveredConnectedNodeIds);
  const selectedEdgeData = useUIStore(selectSelectedEdgeData);

  // ==========================================================================
  // EVENT HANDLERS - Memoized for performance
  // ==========================================================================
  const handleMouseEnter = useCallback(() => setIsHovered(true), []);
  const handleMouseLeave = useCallback(() => setIsHovered(false), []);
  const handleFocus = useCallback(() => setIsFocused(true), []);
  const handleBlur = useCallback(() => setIsFocused(false), []);

  // ==========================================================================
  // COMPUTED STATE
  // ==========================================================================
  // Compute edge connection role for EdgeConnectionHalo
  const edgeConnectionRole: EdgeConnectionRole =
    selectedEdgeData?.source === data.id
      ? 'source'
      : selectedEdgeData?.target === data.id
        ? 'target'
        : null;

  // Focus mode dimming (when a node is selected, non-connected nodes dim to 15%)
  // Still comes from data prop as focus mode works correctly
  const isDimmed = data.dimmed === true;

  // Compute hover dimming locally (bypass React Flow's stale data)
  // Edge hover: only edge endpoints visible (stored in hoveredConnectedNodeIds)
  // Node hover: only hovered node + 1-hop connections visible
  // Priority: selectedNode (focus mode) > edgeHover > nodeHover
  const hasHover = hoveredEdgeId !== null || hoveredNodeId !== null;
  const isThisNodeHighlighted =
    data.id === hoveredNodeId || hoveredConnectedNodeIds.has(data.id);
  const isHoverDimmed = !selectedNodeId && hasHover && !isThisNodeHighlighted;

  // Combined dimming state - focus mode takes priority
  const effectiveDimmed = isDimmed || isHoverDimmed;
  // Opacity: focus = 15%, hover = 25%, normal = 100%
  const opacity = isDimmed ? 0.15 : isHoverDimmed ? 0.25 : 1;

  // ==========================================================================
  // SHADOW COMPUTATION - Layered depth system
  // ==========================================================================
  const currentShadow = selected
    ? NODE_SHADOWS.selected
    : isHovered
      ? NODE_SHADOWS.hover
      : NODE_SHADOWS.default;

  // ==========================================================================
  // BORDER RADIUS - Consistent across all layers
  // ==========================================================================
  const effectiveBorderRadius = shapeClass.includes('full')
    ? BORDER_RADIUS.pill
    : BORDER_RADIUS.card;

  return (
    <div
      className={cn(
        'relative overflow-visible',
        // Focus indicator for keyboard navigation (WCAG 2.1 AA)
        'focus-visible:outline-none',
        effectiveDimmed && 'pointer-events-none',
        isDimmed && 'grayscale'
      )}
      style={{
        opacity,
        transform: effectiveDimmed ? 'scale(0.95)' : 'scale(1)',
        transition: TRANSITION_STYLES.all,
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onFocus={handleFocus}
      onBlur={handleBlur}
      // Accessibility: Make node focusable for keyboard navigation
      tabIndex={effectiveDimmed ? -1 : 0}
      role="button"
      aria-label={`Node: ${data.displayName || data.key}`}
      aria-selected={selected}
      aria-disabled={effectiveDimmed}
    >
      {/* Edge Connection Halo - OUTSIDE GlowingBorder to avoid overflow clipping */}
      <EdgeConnectionHalo
        role={edgeConnectionRole}
        color={color}
        className={shapeClass}
      />

      <GlowingBorder
        color={color}
        colorSecondary={colorSecondary || color}
        isSelected={selected}
        isHovered={(isHovered || isFocused) && !isDimmed}
        animated={selected || isHovered || isFocused}
        borderRadius={effectiveBorderRadius}
      >
        <div
          className={cn(
            'relative',
            glassClasses.modal,
            SIZE_CLASSES[size],
            shapeClass
          )}
          style={{
            boxShadow: currentShadow,
            borderRadius: `${effectiveBorderRadius}px`,
            transition: TRANSITION_STYLES.shadow,
          }}
        >
          {/* Selection Halo - Pulsing ring for selected nodes */}
          <SelectionHalo
            isSelected={selected}
            color={color}
            className={shapeClass}
          />

          {/* Focus Ring - WCAG 2.1 AA compliant (3:1 contrast, 2px minimum) */}
          {isFocused && !selected && (
            <div
              className="absolute inset-0 pointer-events-none rounded-[inherit]"
              style={{
                outline: `2px solid ${color}`,
                outlineOffset: '2px',
                borderRadius: `${effectiveBorderRadius}px`,
              }}
              aria-hidden="true"
            />
          )}

          {/* Target Handle (invisible - required for React Flow) */}
          <Handle
            type="target"
            position={Position.Top}
            className="!w-0 !h-0 !min-w-0 !min-h-0 !border-0 !bg-transparent !opacity-0"
          />

          {/* Content - Size-aware padding */}
          <div className={PADDING_CLASSES[size]}>
            {children}
          </div>

          {/* Source Handle (invisible - required for React Flow) */}
          <Handle
            type="source"
            position={Position.Bottom}
            className="!w-0 !h-0 !min-w-0 !min-h-0 !border-0 !bg-transparent !opacity-0"
          />
        </div>
      </GlowingBorder>
    </div>
  );
});

// ============================================================================
// EXPORTS - Design system constants for child components
// ============================================================================
export { NODE_SHADOWS, BORDER_RADIUS, PADDING_CLASSES, TRANSITION_STYLES };
