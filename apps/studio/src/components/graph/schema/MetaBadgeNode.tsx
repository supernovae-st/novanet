'use client';

/**
 * MetaBadgeNode - Unified card design for Realm & Layer badges
 *
 * v11.0: Redesigned to match NodeCard style from sidebar:
 * - Same glow effect with colored border
 * - Same glassmorphism background
 * - Same badge + title + subtitle layout
 * - Consistent with the rest of the UI
 *
 * Two variants:
 * - Realm (larger): GLOBAL, TENANT
 * - Layer (smaller): CONFIG, LOCALE KNOWLEDGE, etc.
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { Globe, Building2 } from 'lucide-react';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { Realm, Layer } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

export interface MetaBadgeNodeData extends Record<string, unknown> {
  /** Display label */
  label: string;
  /** Description */
  description: string;
  /** Meta type: 'realm' or 'layer' */
  metaType: 'realm' | 'layer';
  /** Accent color (hex) */
  color: string;
  /** Count of child types */
  typeCount?: number;
  /** Realm key for icon selection (for realms) */
  realmKey?: Realm;
  /** Layer key for icon selection (for layers) */
  layerKey?: Layer;
}

export type MetaBadgeNodeType = Node<MetaBadgeNodeData, 'metaBadge'>;

// =============================================================================
// Realm Icons
// =============================================================================

const REALM_ICONS: Record<Realm, typeof Globe> = {
  shared: Globe,
  org: Building2,
};

// =============================================================================
// Shared Styles (matching NodeCard)
// =============================================================================

const getGlowStyle = (color: string, selected: boolean, isHovered: boolean) => ({
  boxShadow: selected
    ? `0 0 30px ${color}50, 0 0 60px ${color}25, inset 0 0 20px ${color}10`
    : isHovered
      ? `0 0 25px ${color}40, 0 0 50px ${color}20`
      : `0 0 20px ${color}30, 0 0 40px ${color}15`,
  borderColor: selected ? color : `${color}60`,
});

const getBadgeStyle = (color: string) => ({
  background: `linear-gradient(135deg, ${color}35, ${color}25)`,
  borderColor: `${color}50`,
  color: color,
  boxShadow: `0 0 12px ${color}30`,
});

// =============================================================================
// Realm Node - Larger card for GLOBAL, TENANT
// =============================================================================

const RealmNode = memo(function RealmNode({
  data,
  selected,
  isHovered,
  handlers,
}: {
  data: MetaBadgeNodeData;
  selected: boolean;
  isHovered: boolean;
  handlers: {
    onMouseEnter: () => void;
    onMouseLeave: () => void;
    onMouseDown: () => void;
    onMouseUp: () => void;
  };
}) {
  const { label, color, typeCount = 0, realmKey = 'shared', description } = data;
  const RealmIconComponent = REALM_ICONS[realmKey as Realm] || Globe;

  const glowStyle = useMemo(
    () => getGlowStyle(color, selected, isHovered),
    [color, selected, isHovered]
  );

  const badgeStyle = useMemo(() => getBadgeStyle(color), [color]);

  return (
    <div
      className={cn(
        'relative transition-all duration-300',
        selected && 'scale-[1.02]',
        isHovered && !selected && 'scale-[1.01]'
      )}
      style={{ width: 220 }}
      onMouseEnter={handlers.onMouseEnter}
      onMouseLeave={handlers.onMouseLeave}
      onMouseDown={handlers.onMouseDown}
      onMouseUp={handlers.onMouseUp}
    >
      {/* Handles */}
      <Handle
        type="target"
        position={Position.Left}
        className="!w-3 !h-3 !border-2 !rounded-full"
        style={{
          left: -6,
          backgroundColor: `${color}40`,
          borderColor: color,
        }}
      />
      <Handle
        type="source"
        position={Position.Right}
        className="!w-3 !h-3 !border-2 !rounded-full"
        style={{
          right: -6,
          backgroundColor: `${color}40`,
          borderColor: color,
        }}
      />

      {/* Main Card - NodeCard style */}
      <div
        className={cn(
          'relative flex flex-col rounded-xl border-2 transition-all duration-300',
          'bg-[#0d0d12]/90 backdrop-blur-sm',
          'p-4',
          selected && 'ring-2 ring-offset-2 ring-offset-[#0d0d12]'
        )}
        style={{
          ...glowStyle,
          '--tw-ring-color': selected ? color : undefined,
        } as React.CSSProperties}
      >
        {/* Badge - Realm type */}
        <div
          className="inline-flex items-center self-start px-2.5 py-1 rounded-full text-[10px] font-bold border mb-3 gap-1.5"
          style={badgeStyle}
        >
          <RealmIconComponent
            size={12}
            strokeWidth={2.5}
            style={{ color }}
          />
          REALM
        </div>

        {/* Title with icon */}
        <div className="flex items-center gap-3 mb-1">
          <div
            className="flex items-center justify-center w-10 h-10 rounded-lg"
            style={{
              background: `${color}20`,
              border: `1.5px solid ${color}40`,
            }}
          >
            <RealmIconComponent
              size={20}
              strokeWidth={2}
              style={{ color }}
            />
          </div>
          <h3 className="text-lg font-bold text-white uppercase tracking-wide">
            {label}
          </h3>
        </div>

        {/* Subtitle - count */}
        <p className="text-white/50 text-xs">
          {typeCount} node types
        </p>
      </div>
    </div>
  );
});

// =============================================================================
// Layer Node - Compact card for CONFIG, LOCALE KNOWLEDGE, etc.
// =============================================================================

const LayerNode = memo(function LayerNode({
  data,
  selected,
  isHovered,
  handlers,
}: {
  data: MetaBadgeNodeData;
  selected: boolean;
  isHovered: boolean;
  handlers: {
    onMouseEnter: () => void;
    onMouseLeave: () => void;
    onMouseDown: () => void;
    onMouseUp: () => void;
  };
}) {
  const { label, color, typeCount = 0, layerKey = 'foundation' } = data;

  const glowStyle = useMemo(
    () => getGlowStyle(color, selected, isHovered),
    [color, selected, isHovered]
  );

  const badgeStyle = useMemo(() => getBadgeStyle(color), [color]);

  return (
    <div
      className={cn(
        'relative transition-all duration-300',
        selected && 'scale-[1.02]',
        isHovered && !selected && 'scale-[1.01]'
      )}
      style={{ width: 180 }}
      onMouseEnter={handlers.onMouseEnter}
      onMouseLeave={handlers.onMouseLeave}
      onMouseDown={handlers.onMouseDown}
      onMouseUp={handlers.onMouseUp}
    >
      {/* Handles */}
      <Handle
        type="target"
        position={Position.Left}
        className="!w-2.5 !h-2.5 !border-2 !rounded-full"
        style={{
          left: -5,
          backgroundColor: `${color}30`,
          borderColor: color,
        }}
      />
      <Handle
        type="source"
        position={Position.Right}
        className="!w-2.5 !h-2.5 !border-2 !rounded-full"
        style={{
          right: -5,
          backgroundColor: `${color}30`,
          borderColor: color,
        }}
      />

      {/* Main Card - NodeCard style */}
      <div
        className={cn(
          'relative flex flex-col rounded-xl border-2 transition-all duration-300',
          'bg-[#0d0d12]/90 backdrop-blur-sm',
          'p-3',
          selected && 'ring-2 ring-offset-2 ring-offset-[#0d0d12]'
        )}
        style={{
          ...glowStyle,
          '--tw-ring-color': selected ? color : undefined,
        } as React.CSSProperties}
      >
        {/* Badge - Layer type */}
        <div
          className="inline-flex items-center self-start px-2 py-0.5 rounded-full text-[9px] font-bold border mb-2 gap-1"
          style={badgeStyle}
        >
          <LayerIcon
            layer={layerKey as Layer}
            size={10}
            strokeWidth={2.5}
            style={{ color }}
          />
          LAYER
        </div>

        {/* Title with icon */}
        <div className="flex items-center gap-2 mb-0.5">
          <div
            className="flex items-center justify-center w-7 h-7 rounded-md"
            style={{
              background: `${color}20`,
              border: `1px solid ${color}40`,
            }}
          >
            <LayerIcon
              layer={layerKey as Layer}
              size={14}
              strokeWidth={2}
              style={{ color }}
            />
          </div>
          <h3
            className="text-sm font-semibold text-white uppercase tracking-wide truncate"
            style={{ maxWidth: 110 }}
          >
            {label}
          </h3>
        </div>

        {/* Subtitle - count */}
        <p className="text-white/50 text-[11px]">
          {typeCount} types
        </p>
      </div>
    </div>
  );
});

// =============================================================================
// Main Component - Routes to Realm or Layer
// =============================================================================

export const MetaBadgeNode = memo(function MetaBadgeNode({
  data,
  selected = false,
}: NodeProps<MetaBadgeNodeType>) {
  const { metaType } = data;

  // Shared interaction state
  const {
    isHovered,
    handleMouseEnter,
    handleMouseLeave,
    handleMouseDown,
    handleMouseUp,
  } = useNodeInteractions({ selected });

  const handlers = useMemo(() => ({
    onMouseEnter: handleMouseEnter,
    onMouseLeave: handleMouseLeave,
    onMouseDown: handleMouseDown,
    onMouseUp: handleMouseUp,
  }), [handleMouseEnter, handleMouseLeave, handleMouseDown, handleMouseUp]);

  // Route to appropriate component
  if (metaType === 'realm') {
    return (
      <RealmNode
        data={data}
        selected={selected}
        isHovered={isHovered}
        handlers={handlers}
      />
    );
  }

  return (
    <LayerNode
      data={data}
      selected={selected}
      isHovered={isHovered}
      handlers={handlers}
    />
  );
});
