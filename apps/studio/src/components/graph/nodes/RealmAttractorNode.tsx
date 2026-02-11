'use client';

/**
 * RealmAttractorNode - Unified card design for Realm attractor nodes
 *
 * Used in magnetic layout mode as gravitational center for child nodes.
 * Uses CardShell + AttractorCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import type { Realm } from '@novanet/core/types';
import { REALM_COLORS } from '@/design/colors/generated';
import { CardShell, AttractorCardContent } from './card';

// =============================================================================
// Types
// =============================================================================

export interface RealmAttractorData extends Record<string, unknown> {
  key: string;
  label: string;
  emoji: string;
  color: string;
  typeCount: number;
  loadedCount: number;
}

export type RealmAttractorNodeType = Node<RealmAttractorData, 'realmAttractor'>;

// =============================================================================
// Main Component
// =============================================================================

export const RealmAttractorNode = memo(function RealmAttractorNode({
  data,
  selected = false,
}: NodeProps<RealmAttractorNodeType>) {
  const { key, label, typeCount = 0, loadedCount = 0 } = data;

  // Get design system color from generated taxonomy (fallback to data.color for safety)
  const realmKey = key as Realm;
  const primaryColor = REALM_COLORS[realmKey]?.color || data.color || '#2aa198';

  const colors = useMemo(() => ({
    primary: primaryColor,
    secondary: primaryColor,
  }), [primaryColor]);

  // Prepare data for AttractorCardContent
  const contentData = useMemo(() => ({
    key,
    label,
    typeCount,
    loadedCount,
  }), [key, label, typeCount, loadedCount]);

  return (
    <>
      {/* Hidden handles for edges (attractor nodes use invisible connection points) */}
      <Handle type="target" position={Position.Top} className="!opacity-0 !w-1 !h-1" />
      <Handle type="source" position={Position.Bottom} className="!opacity-0 !w-1 !h-1" />

      <CardShell
        colors={colors}
        selected={selected}
        width={280}
        minHeight={140}
        showHandles={false}
        showBlueprintOverlay={true}
        ariaLabel={`${label} realm: ${typeCount} types, ${loadedCount} loaded`}
        renderContent={(ctx) => (
          <AttractorCardContent data={contentData} variant="realm" {...ctx} />
        )}
      />
    </>
  );
});
