'use client';

/**
 * SchemaNode - Premium card for schema visualization with blueprint styling
 *
 * Synchronized design with data mode nodes (StructuralNode):
 * - Same gradient border with node type colors
 * - Same skeuomorphism and glassmorphism effects
 * - Same animations (ping, shimmer, float)
 *
 * Blueprint differentiation:
 * - Grid pattern overlay (6% opacity)
 * - Dashed border indicator
 * - Diamond badge in corner
 *
 * Uses shared design system components from effects/ directory.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { getNodeTypeColors } from '@/config/categoryColors';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { BlueprintOverlay } from '../nodes/BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from '../nodes/effects';
import { glassClasses, gapTokens } from '@/design/tokens';
import type { Realm } from '@novanet/core/types';

/**
 * Data interface for SchemaNode
 */
export interface SchemaNodeData extends Record<string, unknown> {
  nodeType: string;
  label: string;
  description: string;
  realm: Realm;
  layer: string;
}

/** Node type for SchemaNode */
export type SchemaNodeType = Node<SchemaNodeData, 'schemaNode'>;

/**
 * SchemaNode - Premium design with blueprint styling
 */
export const SchemaNode = memo(function SchemaNode({
  data,
  selected = false,
}: NodeProps<SchemaNodeType>) {
  // Use node type colors (same as data mode) instead of scope colors
  const colors = getNodeTypeColors(data.nodeType);
  const config = NODE_TYPE_CONFIG[data.nodeType as keyof typeof NODE_TYPE_CONFIG];

  // Shared interaction state management
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected });

  // Memoize gradient border style to prevent re-renders
  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(colors.primary, colors.secondary)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(colors.primary, colors.secondary)
        : NODE_DESIGN.gradients.borderDefault(colors.primary, colors.secondary),
    boxShadow: selected
      ? NODE_DESIGN.shadows.glowSelected(colors.primary)
      : isHovered
        ? NODE_DESIGN.shadows.glowHover(colors.primary)
        : NODE_DESIGN.shadows.glow(colors.primary),
  }), [colors.primary, colors.secondary, selected, isHovered]);

  // Memoize icon style to prevent re-renders
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
  }), [colors.primary, selected]);

  return (
    <div
      className={containerClassName}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {/* Selection pulse ring effect - WOW animation */}
      {selected && (
        <SelectionPulseRing color={colors.primary} borderRadius={NODE_DESIGN.radius.outer} />
      )}

      {/* Gradient border wrapper - 2px (3px when selected) */}
      <div
        className={cn(
          'relative transition-all duration-300',
          selected && 'animate-gradient-rotate',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          borderRadius: NODE_DESIGN.radius.outer,
          padding: selected ? NODE_DESIGN.border.selected : NODE_DESIGN.border.default,
          ...gradientBorderStyle,
        }}
      >
        {/* Inner card - Glassmorphism + Skeuomorphism when selected */}
        <div
          className={cn(
            'relative overflow-hidden transition-colors duration-300',
            selected && glassClasses.medium,
            selected && 'animate-float'
          )}
          style={{
            width: 180,
            borderRadius: selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${colors.primary}` : 'none',
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(colors.primary) : undefined,
          }}
        >
          {/* Glassmorphism effects (bevel, reflection, shimmer) */}
          {selected && (
            <GlassmorphismEffects borderRadius={NODE_DESIGN.radius.innerSelected} />
          )}

          {/* Blueprint overlay - schema mode indicator (always on) */}
          <BlueprintOverlay
            color={colors.primary}
            selected={selected}
            borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
          />

          {/* Handles - horizontal layout (left/right) for schema */}
          <NodeHandles color={colors.primary} selected={selected} layout="horizontal" />

          {/* Content */}
          <div className="relative px-4 py-3">
            {/* Header: Icon + Badge */}
            <div className="flex items-center justify-between mb-2">
              <div className={cn('flex items-center', gapTokens.default)}>
                {config && (
                  <CategoryIcon
                    category={config.category}
                    size={18}
                    strokeWidth={2}
                    className={cn(
                      'transition-transform duration-200',
                      (selected || isHovered) && 'scale-110'
                    )}
                    style={iconStyle}
                  />
                )}
                <span
                  className="text-[9px] font-bold uppercase tracking-wider"
                  style={{ color: colors.primary }}
                >
                  {data.layer}
                </span>
              </div>

              {/* Status dot */}
              <div
                className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 8px ${colors.primary}`,
                }}
              />
            </div>

            {/* Display Name */}
            <h3 className="text-sm font-bold text-white truncate">
              {data.label}
            </h3>

            {/* Node type */}
            <p
              className="text-[10px] font-mono truncate mt-0.5"
              style={{ color: `${colors.primary}70` }}
            >
              {data.nodeType}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
});
