'use client';

/**
 * LayerAttractorNode - Unified card design for Layer attractor nodes
 *
 * v11.6 Design - matches MetaBadgeNode/SchemaNode:
 * - Card (240px width, 120px min height) - slightly smaller than Realm
 * - Large icon top-left with glow
 * - Stacked badges on right (LAYER + layer name)
 * - Glow pulse + gradient borders
 * - Shows layer info + typeCount / loadedCount
 *
 * Used in magnetic layout mode as sub-grouping center for child nodes.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { BlueprintOverlay } from './BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import type { Layer } from '@novanet/core/types';
import { LAYER_COLORS } from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface LayerAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  realmKey: string;
  color: string;
  typeCount: number;
  loadedCount: number;
}

export type LayerAttractorNodeType = Node<LayerAttractorData, 'layerAttractor'>;

// =============================================================================
// GlowBadge - Badge with glow + gradient effect (matches SchemaNode)
// =============================================================================

const GlowBadge = memo(function GlowBadge({
  label,
  icon,
  color,
}: {
  label: string;
  icon?: React.ReactNode;
  color: string;
}) {
  return (
    <span
      className="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wide px-2 py-1 rounded-md"
      style={{
        background: `linear-gradient(135deg, ${color}40, ${color}20)`,
        color: color,
        border: `1px solid ${color}60`,
        boxShadow: `0 0 10px ${color}30, inset 0 1px 0 ${color}20`,
      }}
    >
      {icon}
      {label}
    </span>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const LayerAttractorNode = memo(function LayerAttractorNode({
  data,
  selected = false,
}: NodeProps<LayerAttractorNodeType>) {
  const { key, label, typeCount = 0, loadedCount = 0 } = data;

  // Get design system color from generated taxonomy (fallback to data.color for safety)
  const layerKey = key as Layer;
  const primaryColor = LAYER_COLORS[layerKey]?.color || data.color || '#64748b';

  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
    containerClassName,
    containerStyle,
  } = useNodeInteractions({ selected });

  const gradientBorderStyle = useMemo(() => ({
    background: selected
      ? NODE_DESIGN.gradients.borderSelected(primaryColor, primaryColor)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(primaryColor, primaryColor)
        : NODE_DESIGN.gradients.borderDefault(primaryColor, primaryColor),
    boxShadow: selected
      ? NODE_DESIGN.shadows.glowSelected(primaryColor)
      : isHovered
        ? NODE_DESIGN.shadows.glowHover(primaryColor)
        : NODE_DESIGN.shadows.glow(primaryColor),
  }), [primaryColor, selected, isHovered]);

  const traitBorderStyle = useMemo(() => ({
    borderStyle: 'solid' as const,
    borderWidth: '2px',
    borderColor: `${primaryColor}60`,
  }), [primaryColor]);

  return (
    <div
      className={containerClassName}
      style={{ ...containerStyle, width: 240 }}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`${label} layer: ${typeCount} types, ${loadedCount} loaded`}
    >
      {/* Hidden handles for edges */}
      <Handle type="target" position={Position.Top} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Bottom} className="!opacity-0 !w-1 !h-1" />

      {/* Gradient border wrapper */}
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
        {/* Inner card - compact sizing */}
        <div
          className={cn(
            'relative overflow-hidden transition-all duration-300',
            isHovered && !selected && 'animate-shimmer-sweep'
          )}
          style={{
            minHeight: 120,
            borderRadius: selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${primaryColor}` : undefined,
            borderStyle: selected ? undefined : traitBorderStyle.borderStyle,
            borderWidth: selected ? undefined : traitBorderStyle.borderWidth,
            borderColor: selected ? undefined : traitBorderStyle.borderColor,
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(primaryColor) : undefined,
            // CSS variable for animation color
            '--pulse-color': `${primaryColor}60`,
            '--glow-color': primaryColor,
            '--scan-color': `${primaryColor}80`,
          } as React.CSSProperties}
        >
          <BlueprintOverlay
            color={primaryColor}
            selected={selected}
            borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            showBadge={false}
          />

          {/* Content - compact padding */}
          <div className="relative px-4 py-4">
            {/* Top row: Icon left, Badges right */}
            <div className="flex justify-between items-start mb-3">
              {/* Icon with gradient glow */}
              <div
                className={cn(
                  'flex items-center justify-center w-11 h-11 rounded-lg transition-all duration-300',
                  isHovered && 'animate-icon-glow'
                )}
                style={{
                  background: `
                    radial-gradient(ellipse at 30% 20%, ${primaryColor}50 0%, transparent 50%),
                    radial-gradient(ellipse at 70% 80%, ${primaryColor}30 0%, transparent 50%),
                    linear-gradient(135deg, ${primaryColor}35, ${primaryColor}15, ${primaryColor}25)
                  `,
                  border: `1.5px solid ${primaryColor}50`,
                  boxShadow: isHovered
                    ? `0 0 25px ${primaryColor}50, 0 0 40px ${primaryColor}25, inset 0 0 15px ${primaryColor}20`
                    : `0 0 20px ${primaryColor}35, inset 0 0 12px ${primaryColor}15`,
                  '--glow-color': primaryColor,
                } as React.CSSProperties}
              >
                <LayerIcon
                  layer={layerKey}
                  size={22}
                  strokeWidth={1.5}
                  style={{ color: primaryColor }}
                />
              </div>

              {/* Stacked badges on right */}
              <div className="flex flex-col gap-1.5 items-end">
                <GlowBadge
                  label="LAYER"
                  icon={<LayerIcon layer={layerKey} size={10} strokeWidth={2} style={{ color: primaryColor }} />}
                  color={primaryColor}
                />
                <GlowBadge
                  label={layerKey.toUpperCase().replace(/-/g, ' ')}
                  color={primaryColor}
                />
              </div>
            </div>

            {/* Title */}
            <h3 className="text-base font-bold text-white truncate mb-0.5">
              {label}
            </h3>

            {/* Subtitle - dual count */}
            <p
              className="text-xs font-semibold truncate"
              style={{ color: primaryColor }}
            >
              {typeCount} types &middot; {loadedCount} loaded
            </p>
          </div>
        </div>
      </div>
    </div>
  );
});
