'use client';

/**
 * ClassNode - "Holographic Matrix" premium design for schema Class nodes
 *
 * Visual Encoding (ADR-005, visual-encoding.yaml):
 * - Border COLOR → REALM (shared=cyan #2aa198, org=violet #6c71c4)
 * - Accent/glow → LAYER (9 colors from taxonomy)
 * - Premium effects: L-corners, scanlines, grid, shimmer, matrix rain
 *
 * Used for: NodeClass, ArcClass nodes in schema/blueprint views
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps, Handle, Position } from '@xyflow/react';
import { useNodeInteractions } from '@/hooks';
import { ClassCardContent, type ClassNodeData } from './card';
import type { BaseNodeData } from './BaseNodeWrapper';
import {
  REALM_COLORS,
  LAYER_COLORS,
  type RealmKey,
  type LayerKey,
} from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface ClassNodeExtendedData extends BaseNodeData {
  /** Realm: shared | org */
  realm?: string;
  /** Layer: config, locale, semantic, etc. */
  layer?: string;
  /** Trait: defined, authored, imported, generated, retrieved */
  trait?: string;
  /** Property count */
  propCount?: number;
  /** For ArcClass: source node type */
  source?: string;
  /** For ArcClass: target node type */
  target?: string;
  /** Arc family for ArcClass */
  family?: string;
}

export type ClassNodeType = Node<ClassNodeExtendedData>;

// =============================================================================
// Constants
// =============================================================================

const CLASS_CARD_WIDTH = 240;

// Default colors (fallback)
const DEFAULT_COLORS = {
  primary: '#06b6d4',   // cyan-500
  secondary: '#8b5cf6', // violet-500
};

// =============================================================================
// Component
// =============================================================================

export const ClassNode = memo(function ClassNode(props: NodeProps<ClassNodeType>) {
  const { data, selected = false } = props;

  // Interaction state
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
  } = useNodeInteractions({
    selected,
    isDimmed: data.dimmed === true,
    isHoverDimmed: data.hoverDimmed === true,
  });

  // Get dynamic colors based on realm/layer
  const dynamicColors = useMemo(() => {
    const realmKey = (data.realm || 'shared') as RealmKey;
    const layerKey = (data.layer || 'semantic') as LayerKey;

    // Accent color from LAYER (for glow, effects)
    const layerColor = LAYER_COLORS[layerKey]?.color ?? DEFAULT_COLORS.secondary;
    // Border color from REALM
    const realmColor = REALM_COLORS[realmKey]?.color ?? DEFAULT_COLORS.primary;

    return {
      primary: layerColor,      // Layer color for accents/glow
      secondary: realmColor,    // Realm color for borders
    };
  }, [data.realm, data.layer]);

  // Prepare data for ClassCardContent
  const contentData: ClassNodeData = useMemo(() => ({
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
    realm: data.realm,
    layer: data.layer,
    trait: data.trait,
    propCount: data.propCount,
    source: data.source,
    target: data.target,
    family: data.family,
  }), [data]);

  // Container opacity for dimming
  const containerStyle = useMemo(() => ({
    opacity: data.dimmed ? 0.06 : data.hoverDimmed ? 0.25 : 1,
    transition: 'opacity 0.3s ease-out, transform 0.3s ease-out',
    transform: selected ? 'scale(1.02)' : 'scale(1)',
  }), [data.dimmed, data.hoverDimmed, selected]);

  return (
    <div
      className="relative"
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`Class node: ${data.displayName}`}
    >
      {/* React Flow Handles */}
      <Handle
        type="target"
        position={Position.Top}
        className="!w-3 !h-3 !border-2 !rounded-full !-top-1.5"
        style={{
          background: dynamicColors.primary,
          borderColor: `${dynamicColors.primary}60`,
          boxShadow: selected ? `0 0 10px ${dynamicColors.primary}` : 'none',
        }}
      />

      {/* Content - ClassCardContent now handles all premium styling */}
      <div style={{ width: CLASS_CARD_WIDTH }}>
        <ClassCardContent
          data={contentData}
          colors={{ primary: dynamicColors.primary, secondary: dynamicColors.secondary }}
          selected={selected}
          isHovered={isHovered}
          width={CLASS_CARD_WIDTH}
        />
      </div>

      <Handle
        type="source"
        position={Position.Bottom}
        className="!w-3 !h-3 !border-2 !rounded-full !-bottom-1.5"
        style={{
          background: dynamicColors.primary,
          borderColor: `${dynamicColors.primary}60`,
          boxShadow: selected ? `0 0 10px ${dynamicColors.primary}` : 'none',
        }}
      />
    </div>
  );
});
