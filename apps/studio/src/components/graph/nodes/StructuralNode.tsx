'use client';

/**
 * StructuralNode - Gradient Edge design for structural nodes
 *
 * Categories: project, content, locale, generation
 * Features:
 * - 2px gradient border (category colors)
 * - Solid/hollow handles for direction indication
 * - Animated border on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 *
 * Uses shared design system components from effects/ directory.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getStructuralColors } from '@/config/categoryColors';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { BaseNodeData } from './BaseNodeWrapper';
import { BlueprintOverlay } from './BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from './effects';

export type StructuralNodeType = Node<BaseNodeData>;

/**
 * Get card width based on node type
 */
function getCardWidth(type: string): number {
  switch (type) {
    case 'Page': return 210;
    case 'Concept': return 195;
    case 'Block': return 175;
    case 'BlockType': return 165;
    case 'Locale': return 200;
    default: return 180;
  }
}

/**
 * StructuralNode - Gradient Edge Design
 */
export const StructuralNode = memo(function StructuralNode(props: NodeProps<StructuralNodeType>) {
  const { data, selected = false } = props;
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Project;
  const colors = getStructuralColors(data.type);
  const width = getCardWidth(data.type);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;
  const isSchemaMode = data.isSchemaMode === true;

  // Shared interaction state management
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected, isDimmed, isHoverDimmed });

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
        <SelectionPulseRing color={colors.primary} borderRadius={16} />
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
            'relative overflow-hidden transition-all duration-500 ease-out',
            selected ? 'backdrop-blur-xl' : '',
            selected && 'animate-float'
          )}
          style={{
            width,
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

          {/* Blueprint overlay for schema mode */}
          {isSchemaMode && (
            <BlueprintOverlay
              color={colors.primary}
              selected={selected}
              borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            />
          )}

          {/* Handles - vertical layout (top/bottom) */}
          <NodeHandles color={colors.primary} selected={selected} layout="vertical" />

          {/* Content */}
          <div className="relative px-4 py-3">
            {/* Header: Icon + Badge */}
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <CategoryIcon
                  category={config.category}
                  size={20}
                  strokeWidth={2}
                  className={cn(
                    'transition-transform duration-200',
                    (selected || isHovered) && 'scale-110'
                  )}
                  style={iconStyle}
                />
                <span
                  className="text-[10px] font-bold uppercase tracking-wider"
                  style={{ color: colors.primary }}
                >
                  {config.label}
                </span>
              </div>

              {/* Status dot */}
              <div
                className={cn('w-2.5 h-2.5 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 8px ${colors.primary}`,
                }}
              />
            </div>

            {/* Display Name */}
            <h3 className="text-base font-bold text-white truncate">
              {data.displayName}
            </h3>

            {/* Key */}
            {data.key !== data.displayName && (
              <p
                className="text-[11px] font-mono truncate mt-0.5"
                style={{ color: `${colors.primary}70` }}
              >
                {data.key}
              </p>
            )}

            {/* Category badge */}
            <div
              className="mt-2.5 inline-flex items-center gap-1.5 px-2 py-1 rounded-full text-[9px] font-semibold uppercase tracking-wider border"
              style={{
                background: `${colors.primary}15`,
                borderColor: `${colors.primary}35`,
                color: colors.primary,
              }}
            >
              <span
                className={cn('w-1.5 h-1.5 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 6px ${colors.primary}`,
                }}
              />
              {config.category}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
});
