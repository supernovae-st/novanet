'use client';

/**
 * LocaleKnowledgeNode - Circular nodes for locale knowledge types
 *
 * Category: locale (knowledge nodes within locale category)
 * Types: v10 tiered model - Formatting, Slugification, Adaptation, Style, TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 * Features:
 * - Circular gradient ring design (2px)
 * - Solid/hollow handles for direction indication
 * - Animated ring on selection
 * - Enhanced hover effects (scale, glow)
 * - Press feedback (scale down on mousedown)
 * - Hover info displayed in centralized bottom pill (via uiStore.hoveredNodeId)
 *
 * Uses shared design system components from effects/ directory.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { glassClasses } from '@/design/tokens';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLocaleKnowledgeColors } from '@/design/nodeColors';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { BaseNodeData } from './BaseNodeWrapper';
import { BlueprintOverlay } from './BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects, NodeHandles } from './effects';

export type LocaleKnowledgeNodeType = Node<BaseNodeData>;

/**
 * Get size based on connection count and type
 */
function getCircleSize(type: string, connectionCount?: number): number {
  const count = connectionCount || 0;
  // v10 knowledge tier sizes
  const baseSize: Record<string, number> = {
    // Technical tier
    Formatting: 55,
    Slugification: 55,
    Adaptation: 55,
    // Style tier
    Style: 65,
    // Semantic tier
    TermSet: 55,
    ExpressionSet: 55,
    PatternSet: 55,
    CultureSet: 55,
    TabooSet: 55,
    AudienceSet: 55,
  };
  const base = baseSize[type] || 55;
  if (count > 10) return base + 20;
  if (count > 5) return base + 12;
  if (count > 2) return base + 6;
  return base;
}

/**
 * LocaleKnowledgeNode - Circular Gradient Ring Design
 */
export const LocaleKnowledgeNode = memo(function LocaleKnowledgeNode(props: NodeProps<LocaleKnowledgeNodeType>) {
  const { data, selected = false } = props;
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.ExpressionSet;
  const colors = getLocaleKnowledgeColors(data.type);
  const size = getCircleSize(data.type, data.connectionCount);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;
  const isMetaMode = data.isMetaMode === true;

  // Shared interaction state management (circular variant)
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected, isDimmed, isHoverDimmed, isCircular: true });

  // Memoize gradient ring style to prevent re-renders
  const gradientRingStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(colors.primary, colors.secondary)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(colors.primary, colors.secondary)
        : NODE_DESIGN.gradients.borderDefault(colors.primary, colors.secondary),
    boxShadow: selected
      ? `0 0 35px 8px ${colors.primary}70, 0 0 70px 14px ${colors.primary}40, 0 0 100px 20px ${colors.primary}20`
      : isHovered
        ? `0 0 25px 5px ${colors.primary}50, 0 0 50px 10px ${colors.primary}25`
        : `0 0 18px 4px ${colors.primary}45, 0 0 35px 7px ${colors.primary}20`,
  }), [colors.primary, colors.secondary, selected, isHovered]);

  // Memoize icon style to prevent re-renders
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}90)`,
  }), [colors.primary, selected]);

  return (
    <div
      className={containerClassName}
      style={{
        ...containerStyle,
        ...(isMetaMode && !selected && { opacity: 0.6, filter: 'saturate(0.7)' }),
      }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {/* Selection pulse ring effect - WOW animation (circular) */}
      {selected && (
        <SelectionPulseRing color={colors.primary} borderRadius={NODE_DESIGN.radius.circular} />
      )}

      {/* Gradient ring wrapper - 2px (3px when selected) */}
      <div
        className={cn(
          'relative rounded-full transition-all duration-300',
          selected && 'animate-spin-slow',
          isHovered && !selected && 'animate-glow-pulse'
        )}
        style={{
          padding: selected ? NODE_DESIGN.border.selected : NODE_DESIGN.border.default,
          ...gradientRingStyle,
        }}
      >
        {/* Inner circle - Glassmorphism + Skeuomorphism when selected */}
        <div
          className={cn(
            'flex flex-col items-center justify-center rounded-full',
            selected && glassClasses.modal,
            selected && 'animate-float'
          )}
          style={{
            width: selected ? size - 2 : size,
            height: selected ? size - 2 : size,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `2px solid ${colors.primary}` : 'none',
            boxShadow: selected
              ? `
                inset 0 2px 0 0 rgba(255, 255, 255, 0.15),
                inset 0 -2px 0 0 rgba(0, 0, 0, 0.4),
                inset 0 0 20px ${colors.primary}25,
                0 8px 30px rgba(0, 0, 0, 0.5),
                0 4px 12px rgba(0, 0, 0, 0.4)
              `
              : undefined,
          }}
        >
          {/* Glassmorphism effects (circular variant) */}
          {selected && (
            <GlassmorphismEffects isCircular />
          )}

          {/* Blueprint overlay for meta mode (circular) */}
          {isMetaMode && (
            <BlueprintOverlay
              color={colors.primary}
              selected={selected}
              borderRadius={NODE_DESIGN.radius.circular}
              showBadge={false}
            />
          )}

          {/* Handles - vertical layout, small size for circular nodes */}
          <NodeHandles color={colors.primary} selected={selected} layout="vertical" size="small" />

          {/* Icon - SVG from Lucide */}
          <LayerIcon
            layer={config.layer}
            size={20}
            strokeWidth={2}
            className={cn(
              'transition-transform duration-200',
              (isHovered || selected) && 'scale-110'
            )}
            style={iconStyle}
          />

          {/* Label */}
          <span
            className="text-[9px] font-semibold text-center truncate max-w-[85%] mt-1"
            style={{
              color: selected ? 'white' : colors.primary,
              textShadow: selected ? `0 0 6px ${colors.primary}` : 'none',
            }}
          >
            {data.displayName.length > 10
              ? data.displayName.slice(0, 8) + '...'
              : data.displayName}
          </span>
        </div>
      </div>
    </div>
  );
});
