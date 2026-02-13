'use client';

/**
 * SchemaNode - Premium card for schema visualization
 *
 * v11.5 Design:
 * - Wider card (240px), taller
 * - Large icon top-left with glow
 * - 3 stacked badges: Realm, Layer, Trait (with design system colors)
 * - Glow pulsé + gradient badges
 * - Node color = Layer color
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { LayerIcon, RealmIcon, TraitIcon } from '@/components/ui/CategoryIcon';
import { BlueprintOverlay } from '../nodes/BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects } from '../nodes/effects';
import { glassClasses } from '@/design/tokens';
import type { Realm, Layer } from '@novanet/core/types';
import {
  REALM_COLORS,
  LAYER_COLORS,
  TRAIT_COLORS,
} from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

// v0.12.0: renamed per ADR-024 Data Origin
export type Trait = 'defined' | 'authored' | 'imported' | 'generated' | 'retrieved';

// v0.12.0: 5 traits renamed per ADR-024 Data Origin
const TRAIT_BORDER_STYLES: Record<string, { style: string; width: number; className?: string }> = {
  defined: { style: 'solid', width: 2 },     // was: invariant
  authored: { style: 'dashed', width: 2 },   // was: localized
  imported: { style: 'dotted', width: 2 },   // was: knowledge
  generated: { style: 'double', width: 2 },
  retrieved: { style: 'dotted', width: 3 },  // was: aggregated
};

export interface SchemaNodeData extends Record<string, unknown> {
  nodeType: string;
  label: string;
  description: string;
  realm: Realm;
  layer: string;
  trait?: Trait;
}

export type SchemaNodeType = Node<SchemaNodeData, 'schemaNode'>;

// =============================================================================
// GlowBadge - Badge with glow + gradient + icon
// =============================================================================

const GlowBadge = memo(function GlowBadge({
  label,
  icon,
  color,
}: {
  label: string;
  icon: React.ReactNode;
  color: string;
}) {
  return (
    <span
      className="flex items-center gap-1.5 text-[11px] font-bold uppercase tracking-wide px-2.5 py-1.5 rounded-md"
      style={{
        background: `linear-gradient(135deg, ${color}40, ${color}20)`,
        color: color,
        border: `1px solid ${color}60`,
        boxShadow: `0 0 12px ${color}35, inset 0 1px 0 ${color}25`,
      }}
    >
      {icon}
      {label}
    </span>
  );
});

// =============================================================================
// SchemaNode Component
// =============================================================================

export const SchemaNode = memo(function SchemaNode({
  data,
  selected = false,
}: NodeProps<SchemaNodeType>) {
  const config = NODE_TYPE_CONFIG[data.nodeType as keyof typeof NODE_TYPE_CONFIG];

  // Get classification values
  const realm = (data.realm || 'shared') as Realm;
  const layer = (config?.layer || data.layer || 'foundation') as Layer;
  const trait = (data.trait || 'defined') as Trait;

  // Use LAYER color as the primary node color (from generated taxonomy)
  const layerColor = LAYER_COLORS[layer]?.color || '#64748b';
  const realmColor = REALM_COLORS[realm]?.color || '#2aa198';
  const traitColor = TRAIT_COLORS[trait]?.color || '#3b82f6';

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
      ? NODE_DESIGN.gradients.borderSelected(layerColor, layerColor)
      : isHovered
        ? NODE_DESIGN.gradients.borderHover(layerColor, layerColor)
        : NODE_DESIGN.gradients.borderDefault(layerColor, layerColor),
    boxShadow: selected
      ? NODE_DESIGN.shadows.glowSelected(layerColor)
      : isHovered
        ? NODE_DESIGN.shadows.glowHover(layerColor)
        : NODE_DESIGN.shadows.glow(layerColor),
  }), [layerColor, selected, isHovered]);

  const traitBorder = TRAIT_BORDER_STYLES[trait] || TRAIT_BORDER_STYLES.defined; // v11.8: ADR-024
  const traitBorderStyle = useMemo(() => ({
    borderStyle: traitBorder.style,
    borderWidth: `${traitBorder.width}px`,
    borderColor: `${layerColor}60`,
  }), [traitBorder.style, traitBorder.width, layerColor]);

  return (
    <div
      className={containerClassName}
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
    >
      {selected && (
        <SelectionPulseRing color={layerColor} borderRadius={NODE_DESIGN.radius.outer} />
      )}

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
        {/* Inner card - premium sizing with breathing room */}
        <div
          className={cn(
            'relative overflow-hidden transition-all duration-300',
            selected && glassClasses.medium,
            selected && 'animate-float',
            isHovered && !selected && 'animate-shimmer-sweep',
            traitBorder.className
          )}
          style={{
            width: 280,
            minHeight: 160,
            borderRadius: selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            border: selected ? `${NODE_DESIGN.border.innerSelected}px solid ${layerColor}` : undefined,
            borderStyle: selected ? undefined : (traitBorderStyle.borderStyle as React.CSSProperties['borderStyle']),
            borderWidth: selected ? undefined : traitBorderStyle.borderWidth,
            borderColor: selected ? undefined : traitBorderStyle.borderColor,
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(layerColor) : undefined,
            // CSS variable for animation color
            '--pulse-color': `${layerColor}60`,
            '--glow-color': layerColor,
            '--scan-color': `${layerColor}80`,
          } as React.CSSProperties}
        >
          {selected && (
            <GlassmorphismEffects borderRadius={NODE_DESIGN.radius.innerSelected} />
          )}

          <BlueprintOverlay
            color={layerColor}
            selected={selected}
            borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            showBadge={false}
          />

          {/* Content - more breathing room */}
          <div className="relative px-6 py-5">
            {/* Top row: Large icon left, Badges right */}
            <div className="flex justify-between items-start mb-5">
              {/* Large Layer icon with premium gradient glow + animation */}
              <div
                className={cn(
                  'flex items-center justify-center w-14 h-14 rounded-xl transition-all duration-300',
                  isHovered && 'animate-icon-glow'
                )}
                style={{
                  background: `
                    radial-gradient(ellipse at 30% 20%, ${layerColor}50 0%, transparent 50%),
                    radial-gradient(ellipse at 70% 80%, ${layerColor}30 0%, transparent 50%),
                    linear-gradient(135deg, ${layerColor}35, ${layerColor}15, ${layerColor}25)
                  `,
                  border: `1.5px solid ${layerColor}50`,
                  boxShadow: isHovered
                    ? `0 0 30px ${layerColor}50, 0 0 50px ${layerColor}25, inset 0 0 20px ${layerColor}20`
                    : `0 0 25px ${layerColor}35, inset 0 0 15px ${layerColor}15`,
                  '--glow-color': layerColor,
                } as React.CSSProperties}
              >
                <LayerIcon
                  layer={layer}
                  size={28}
                  strokeWidth={1.5}
                  style={{ color: layerColor }}
                />
              </div>

              {/* 3 Stacked badges with design system colors */}
              <div className="flex flex-col gap-2 items-end">
                <GlowBadge
                  label={realm.toUpperCase()}
                  icon={<RealmIcon realm={realm} size={12} strokeWidth={2} style={{ color: realmColor }} />}
                  color={realmColor}
                />
                <GlowBadge
                  label={layer.toUpperCase().replace(/-/g, ' ')}
                  icon={<LayerIcon layer={layer} size={12} strokeWidth={2} style={{ color: layerColor }} />}
                  color={layerColor}
                />
                <GlowBadge
                  label={trait.toUpperCase()}
                  icon={<TraitIcon trait={trait} size={12} strokeWidth={2} style={{ color: traitColor }} />}
                  color={traitColor}
                />
              </div>
            </div>

            {/* Title - larger */}
            <h3 className="text-lg font-bold text-white truncate mb-1">
              {data.label}
            </h3>

            {/* Subtitle - bolder, layer color */}
            <p
              className="text-sm font-semibold truncate"
              style={{ color: layerColor }}
            >
              {data.nodeType}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
});
