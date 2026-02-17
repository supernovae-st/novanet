'use client';

/**
 * RealmNode - "Dimensional Gateway" design for Realm nodes
 *
 * Visual Encoding:
 * - Primary color → REALM color (shared=cyan #2aa198, org=violet #6c71c4)
 * - Aurora mesh background with animated blobs
 * - 3D tilt effect on mouse movement
 * - Animated SVG border with gradient stroke
 * - Holographic shimmer sweep on hover
 * - Spotlight effect following cursor
 * - Portal icon with concentric rings
 *
 * Used for: Realm nodes (shared, org) in schema/blueprint views
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps, Handle, Position } from '@xyflow/react';
import { useNodeInteractions } from '@/hooks';
import { RealmOrbitalCardContent, type RealmNodeData } from './card';
import type { BaseNodeData } from './BaseNodeWrapper';
import {
  REALM_COLORS,
  type RealmKey,
} from '@/design/colors/generated';

// =============================================================================
// Types
// =============================================================================

export interface RealmNodeExtendedData extends BaseNodeData {
  /** Realm: shared | org */
  realm: 'shared' | 'org';
  /** Description */
  description: string;
  /** Total node count */
  nodeCount: number;
  /** Layer distribution for stats */
  layerDistribution: Array<{
    layer: string;
    count: number;
    percentage: number;
  }>;
}

export type RealmNodeType = Node<RealmNodeExtendedData>;

// =============================================================================
// Constants
// =============================================================================

const DEFAULT_REALM_COLOR = '#2aa198'; // cyan-500 (shared)

// =============================================================================
// Component
// =============================================================================

export const RealmNode = memo(function RealmNode(props: NodeProps<RealmNodeType>) {
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

  // Get realm color
  const realmColor = useMemo(() => {
    const realmKey = data.realm as RealmKey;
    return REALM_COLORS[realmKey]?.color ?? DEFAULT_REALM_COLOR;
  }, [data.realm]);

  // Prepare data for RealmOrbitalCardContent (with safe defaults)
  const contentData: RealmNodeData = useMemo(() => ({
    id: data.id,
    type: data.type,
    key: data.realm ?? 'shared',
    displayName: data.displayName ?? data.realm ?? 'Realm',
    description: data.description ?? '',
    nodeCount: data.nodeCount ?? 0,
    layerDistribution: data.layerDistribution ?? [],
  }), [data]);

  // Container opacity for dimming
  const containerStyle = useMemo(() => ({
    opacity: data.dimmed ? 0.06 : data.hoverDimmed ? 0.25 : 1,
    transition: 'opacity 0.3s ease-out',
  }), [data.dimmed, data.hoverDimmed]);

  return (
    <div
      className="relative"
      style={containerStyle}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      aria-label={`Realm node: ${data.displayName}`}
    >
      {/* React Flow Handles */}
      <Handle
        type="target"
        position={Position.Top}
        className="!w-3 !h-3 !border-2 !rounded-full !-top-1.5"
        style={{
          background: realmColor,
          borderColor: `${realmColor}60`,
          boxShadow: selected ? `0 0 10px ${realmColor}` : 'none',
        }}
      />

      {/* Content - handles all styling */}
      <RealmOrbitalCardContent
        data={contentData}
        colors={{ primary: realmColor, secondary: realmColor }}
        selected={selected}
        isHovered={isHovered}
        width={420}
      />

      <Handle
        type="source"
        position={Position.Bottom}
        className="!w-3 !h-3 !border-2 !rounded-full !-bottom-1.5"
        style={{
          background: realmColor,
          borderColor: `${realmColor}60`,
          boxShadow: selected ? `0 0 10px ${realmColor}` : 'none',
        }}
      />
    </div>
  );
});
