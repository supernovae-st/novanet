'use client';

/**
 * LocaleKnowledgeNode - Unified card design for locale/knowledge nodes
 *
 * v11.6 Design - matches StructuralNode:
 * - Card design (150-180px width) instead of circular
 * - 2px gradient border (layer colors)
 * - Icon + type label header
 * - Display name + key
 * - Layer badge
 *
 * Types: v11.5 knowledge atoms - Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
 * Plus containers: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 * And locale/geography nodes: Locale, Culture, Style, Region, Country, etc.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { glassClasses, gapTokens } from '@/design/tokens';
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
 * Get card width based on node type
 * Slightly smaller than StructuralNode since these are knowledge atoms
 */
function getCardWidth(type: string): number {
  switch (type) {
    // Containers (larger)
    case 'TermSet':
    case 'ExpressionSet':
    case 'PatternSet':
    case 'CultureSet':
    case 'TabooSet':
    case 'AudienceSet':
    case 'CategorySet':
      return 175;
    // Locale/geography nodes
    case 'Locale':
    case 'Culture':
    case 'Style':
    case 'Region':
    case 'Country':
    case 'Continent':
      return 170;
    // Knowledge atoms (smaller)
    case 'Term':
    case 'Expression':
    case 'Pattern':
    case 'CultureRef':
    case 'Taboo':
    case 'AudienceTrait':
      return 155;
    default:
      return 160;
  }
}

/**
 * LocaleKnowledgeNode - Gradient Edge Card Design
 */
export const LocaleKnowledgeNode = memo(function LocaleKnowledgeNode(props: NodeProps<LocaleKnowledgeNodeType>) {
  const { data, selected = false } = props;
  const config = NODE_TYPE_CONFIG[data.type] || NODE_TYPE_CONFIG.Term;
  const colors = getLocaleKnowledgeColors(data.type);
  const width = getCardWidth(data.type);
  const isDimmed = data.dimmed === true;
  const isHoverDimmed = data.hoverDimmed === true;
  const isMetaMode = data.isMetaMode === true;

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
      style={{
        ...containerStyle,
        ...(isMetaMode && !selected && { opacity: 0.6, filter: 'saturate(0.7)' }),
      }}
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
            selected && glassClasses.medium,
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

          {/* Blueprint overlay for meta mode */}
          {isMetaMode && (
            <BlueprintOverlay
              color={colors.primary}
              selected={selected}
              borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            />
          )}

          {/* Handles - vertical layout (top/bottom) */}
          <NodeHandles color={colors.primary} selected={selected} layout="vertical" />

          {/* Content */}
          <div className="relative px-3 py-2.5">
            {/* Header: Icon + Badge */}
            <div className="flex items-center justify-between mb-1.5">
              <div className={cn('flex items-center', gapTokens.default)}>
                <LayerIcon
                  layer={config.layer}
                  size={18}
                  strokeWidth={2}
                  className={cn(
                    'transition-transform duration-200',
                    (selected || isHovered) && 'scale-110'
                  )}
                  style={iconStyle}
                />
                <span
                  className="text-[9px] font-bold uppercase tracking-wider"
                  style={{ color: colors.primary }}
                >
                  {config.label}
                </span>
              </div>

              {/* Status dot */}
              <div
                className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 6px ${colors.primary}`,
                }}
              />
            </div>

            {/* Display Name */}
            <h3 className="text-sm font-bold text-white truncate">
              {data.displayName}
            </h3>

            {/* Key */}
            {data.key !== data.displayName && (
              <p
                className="text-[10px] font-mono truncate mt-0.5"
                style={{ color: `${colors.primary}70` }}
              >
                {data.key}
              </p>
            )}

            {/* Layer badge */}
            <div
              className={cn('mt-2 inline-flex items-center px-1.5 py-0.5 rounded-full text-[8px] font-semibold uppercase tracking-wider border', gapTokens.compact)}
              style={{
                background: `${colors.primary}15`,
                borderColor: `${colors.primary}35`,
                color: colors.primary,
              }}
            >
              <span
                className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
                style={{
                  background: colors.primary,
                  boxShadow: `0 0 4px ${colors.primary}`,
                }}
              />
              {config.layer}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
});
