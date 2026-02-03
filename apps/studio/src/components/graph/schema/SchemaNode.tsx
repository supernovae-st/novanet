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
import { getNodeTypeColors } from '@/design/nodeColors';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { BlueprintOverlay } from '../nodes/BlueprintOverlay';
import { NODE_BG, NODE_DESIGN } from '@/config/constants';
import { useNodeInteractions } from '@/hooks';
import { SelectionPulseRing, GlassmorphismEffects } from '../nodes/effects';
import { glassClasses } from '@/design/tokens';
import type { Realm } from '@novanet/core/types';

/**
 * Trait type - locale behavior encoding
 * v9: Border style encodes trait
 */
export type Trait = 'invariant' | 'localized' | 'knowledge' | 'derived' | 'job';

/**
 * Trait border styles (v9 visual encoding)
 * - invariant: Solid 2px - stable, structural nodes
 * - localized: Dashed 2px - generated per locale
 * - knowledge: Double border - cultural/linguistic expertise
 * - derived: Dotted 1px - computed aggregates
 * - job: Thin 1px - background tasks
 */
const TRAIT_BORDER_STYLES: Record<Trait, { style: string; width: number; className?: string }> = {
  invariant: { style: 'solid', width: 2 },
  localized: { style: 'dashed', width: 2 },
  knowledge: { style: 'double', width: 4, className: 'ring-2 ring-inset ring-white/20' },
  derived: { style: 'dotted', width: 1 },
  job: { style: 'solid', width: 1 },
};

/**
 * Data interface for SchemaNode
 */
export interface SchemaNodeData extends Record<string, unknown> {
  nodeType: string;
  label: string;
  description: string;
  realm: Realm;
  layer: string;
  /** Trait for border style encoding */
  trait?: Trait;
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

  // v9: Trait border style encoding
  const trait = (data.trait || 'invariant') as Trait;
  const traitBorder = TRAIT_BORDER_STYLES[trait];
  const traitBorderStyle = useMemo(() => ({
    borderStyle: traitBorder.style,
    borderWidth: `${traitBorder.width}px`,
    borderColor: `${colors.primary}60`,
  }), [traitBorder.style, traitBorder.width, colors.primary]);

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
        {/* v9: Border style encodes Trait (invariant=solid, localized=dashed, knowledge=double, derived=dotted, job=thin) */}
        <div
          className={cn(
            'relative overflow-hidden transition-colors duration-300',
            selected && glassClasses.medium,
            selected && 'animate-float',
            traitBorder.className // Additional class for knowledge trait (double ring)
          )}
          style={{
            width: 180,
            borderRadius: selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner,
            backgroundColor: selected ? NODE_DESIGN.selectedBg : NODE_BG.default,
            // v9: Apply trait border style (overridden when selected)
            ...(selected
              ? { border: `${NODE_DESIGN.border.innerSelected}px solid ${colors.primary}` }
              : traitBorderStyle),
            boxShadow: selected ? NODE_DESIGN.shadows.skeuomorphic(colors.primary) : undefined,
          }}
        >
          {/* Glassmorphism effects (bevel, reflection, shimmer) */}
          {selected && (
            <GlassmorphismEffects borderRadius={NODE_DESIGN.radius.innerSelected} />
          )}

          {/* Blueprint overlay - schema mode indicator (no diamond badge) */}
          <BlueprintOverlay
            color={colors.primary}
            selected={selected}
            borderRadius={selected ? NODE_DESIGN.radius.innerSelected : NODE_DESIGN.radius.inner}
            showBadge={false}
          />

          {/* Trait badge - top right corner */}
          <div className="absolute top-2 right-2 z-10">
            <span
              className={cn(
                'flex items-center gap-1 text-[8px] font-bold uppercase tracking-wide px-1.5 py-0.5 rounded',
                trait === 'job' && 'animate-pulse'
              )}
              style={{
                backgroundColor: `${colors.primary}25`,
                color: colors.primary,
                border: `1px ${traitBorder.style} ${colors.primary}50`,
                boxShadow: `0 0 8px ${colors.primary}30`,
              }}
            >
              {config && (
                <LayerIcon
                  layer={config.layer}
                  size={10}
                  strokeWidth={2.5}
                  style={{ color: colors.primary }}
                />
              )}
              {trait}
            </span>
          </div>

          {/* Content */}
          <div className="relative px-4 py-3 pt-8">
            {/* Display Name - full width */}
            <h3 className="text-sm font-bold text-white truncate mb-1">
              {data.label}
            </h3>

            {/* Node type */}
            <p
              className="text-[10px] font-mono truncate"
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
