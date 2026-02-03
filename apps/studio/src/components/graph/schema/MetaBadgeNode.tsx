'use client';

/**
 * MetaBadgeNode - Premium nodes for Meta view (Realm & Layer)
 *
 * v9.6: Hierarchical visualization with WOW design:
 * - Realm: Large, glowing anchor nodes with neon borders (3 total)
 * - Layer: Medium, prominent category nodes (9 total)
 *
 * Features:
 * - Lucide icons (no emojis)
 * - Neon gradient borders with animated glow
 * - Glassmorphism + blueprint grid overlay
 * - Progress bar with count
 */

import { memo, useMemo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { useNodeInteractions } from '@/hooks';
import { Globe, Package, Target } from 'lucide-react';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { Realm, Layer } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

export interface MetaBadgeNodeData extends Record<string, unknown> {
  /** Display label (text only, no emoji) */
  label: string;
  /** Description */
  description: string;
  /** Meta type: 'realm' or 'layer' */
  metaType: 'realm' | 'layer';
  /** Accent color (hex) */
  color: string;
  /** Count of child types */
  typeCount?: number;
  /** Count of loaded instances */
  loadedCount?: number;
  /** Realm key for icon selection (for realms) */
  realmKey?: Realm;
  /** Layer key for icon selection (for layers) */
  layerKey?: Layer;
}

export type MetaBadgeNodeType = Node<MetaBadgeNodeData, 'metaBadge'>;

// =============================================================================
// Realm Icon Map
// =============================================================================

const REALM_ICONS = {
  project: Package,
  global: Globe,
  shared: Target,
} as const;

// =============================================================================
// Realm Node - Premium Neon Design
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
  const { label, color, typeCount = 0, realmKey = 'project' } = data;
  const RealmIconComponent = REALM_ICONS[realmKey as Realm] || Package;

  return (
    <div
      className={cn(
        'relative transition-all duration-300',
        selected && 'scale-[1.02]',
        isHovered && !selected && 'scale-[1.01]'
      )}
      style={{ width: 260 }}
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

      {/* Neon glow effect - outer blur */}
      <div
        className={cn(
          'absolute -inset-1 rounded-2xl transition-opacity duration-300',
          selected ? 'opacity-80' : isHovered ? 'opacity-50' : 'opacity-30'
        )}
        style={{
          background: `linear-gradient(135deg, ${color} 0%, ${color}80 100%)`,
          filter: 'blur(12px)',
        }}
      />

      {/* Main Container - Neon border */}
      <div
        className="relative rounded-xl overflow-hidden"
        style={{
          background: `linear-gradient(135deg, ${color} 0%, ${color}80 100%)`,
          padding: selected ? 3 : 2,
        }}
      >
        {/* Inner card - dark glassmorphism */}
        <div
          className={cn(
            'relative rounded-lg overflow-hidden',
            'backdrop-blur-xl'
          )}
          style={{
            background: 'linear-gradient(135deg, rgba(15,15,20,0.95) 0%, rgba(10,10,15,0.98) 100%)',
          }}
        >
          {/* Blueprint grid overlay */}
          <div
            className="absolute inset-0 opacity-[0.08]"
            style={{
              backgroundImage: `
                linear-gradient(${color} 1px, transparent 1px),
                linear-gradient(90deg, ${color} 1px, transparent 1px)
              `,
              backgroundSize: '16px 16px',
            }}
          />

          {/* Shimmer effect on select */}
          {selected && (
            <div
              className="absolute inset-0 animate-shimmer"
              style={{
                background: `linear-gradient(90deg, transparent 0%, ${color}15 50%, transparent 100%)`,
                backgroundSize: '200% 100%',
              }}
            />
          )}

          {/* Content */}
          <div className="relative px-5 py-4">
            {/* Row: Icon + Label + Count */}
            <div className="flex items-center gap-4">
              {/* Icon container with glow */}
              <div
                className={cn(
                  'flex items-center justify-center w-12 h-12 rounded-xl transition-all duration-300',
                  selected && 'scale-110'
                )}
                style={{
                  background: `linear-gradient(135deg, ${color}30 0%, ${color}10 100%)`,
                  border: `2px solid ${color}50`,
                  boxShadow: selected
                    ? `0 0 20px ${color}60, inset 0 0 10px ${color}20`
                    : `0 0 10px ${color}30`,
                }}
              >
                <RealmIconComponent
                  size={24}
                  strokeWidth={2.5}
                  style={{
                    color: color,
                    filter: `drop-shadow(0 0 6px ${color})`,
                  }}
                />
              </div>

              {/* Label + Progress */}
              <div className="flex-1 min-w-0">
                {/* Label */}
                <span
                  className="block text-lg font-black uppercase tracking-wide truncate"
                  style={{
                    color: selected ? 'white' : color,
                    textShadow: selected ? `0 0 15px ${color}` : 'none',
                  }}
                >
                  {label}
                </span>

                {/* Progress bar */}
                <div className="flex items-center gap-2 mt-2">
                  <div
                    className="flex-1 h-1.5 rounded-full overflow-hidden"
                    style={{ backgroundColor: `${color}20` }}
                  >
                    <div
                      className="h-full rounded-full transition-all duration-500"
                      style={{
                        width: `${Math.min((typeCount / 15) * 100, 100)}%`,
                        background: `linear-gradient(90deg, ${color} 0%, ${color}cc 100%)`,
                        boxShadow: `0 0 8px ${color}`,
                      }}
                    />
                  </div>
                  <span
                    className="text-sm font-bold tabular-nums"
                    style={{ color: `${color}cc` }}
                  >
                    {typeCount}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
});

// =============================================================================
// Layer Node - Compact Neon Design
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

  return (
    <div
      className={cn(
        'relative transition-all duration-200',
        selected && 'scale-[1.02]'
      )}
      style={{ width: 200 }}
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

      {/* Subtle glow */}
      {(selected || isHovered) && (
        <div
          className={cn(
            'absolute -inset-0.5 rounded-lg transition-opacity duration-200',
            selected ? 'opacity-60' : 'opacity-30'
          )}
          style={{
            background: color,
            filter: 'blur(8px)',
          }}
        />
      )}

      {/* Main Container - Neon border */}
      <div
        className="relative rounded-lg overflow-hidden"
        style={{
          background: `linear-gradient(135deg, ${color}90 0%, ${color}60 100%)`,
          padding: selected ? 2 : 1.5,
        }}
      >
        {/* Inner card */}
        <div
          className="relative rounded-md overflow-hidden"
          style={{
            background: 'linear-gradient(135deg, rgba(15,15,20,0.95) 0%, rgba(10,10,15,0.98) 100%)',
          }}
        >
          {/* Blueprint grid overlay */}
          <div
            className="absolute inset-0 opacity-[0.05]"
            style={{
              backgroundImage: `
                linear-gradient(${color} 1px, transparent 1px),
                linear-gradient(90deg, ${color} 1px, transparent 1px)
              `,
              backgroundSize: '12px 12px',
            }}
          />

          {/* Content */}
          <div className="relative px-3 py-2.5">
            {/* Row: Icon + Label + Progress + Count */}
            <div className="flex items-center gap-2.5">
              {/* Icon with glow */}
              <div
                className={cn(
                  'flex items-center justify-center w-8 h-8 rounded-lg transition-all duration-200',
                  selected && 'scale-105'
                )}
                style={{
                  background: `${color}20`,
                  border: `1.5px solid ${color}40`,
                  boxShadow: `0 0 8px ${color}30`,
                }}
              >
                <LayerIcon
                  layer={layerKey as Layer}
                  size={16}
                  strokeWidth={2}
                  style={{
                    color: color,
                    filter: `drop-shadow(0 0 4px ${color})`,
                  }}
                />
              </div>

              {/* Label */}
              <span
                className="text-xs font-bold uppercase tracking-wide flex-shrink-0"
                style={{
                  color: selected ? 'white' : color,
                  textShadow: selected ? `0 0 10px ${color}` : 'none',
                }}
              >
                {label}
              </span>

              {/* Spacer + Progress bar */}
              <div
                className="flex-1 h-1 rounded-full overflow-hidden"
                style={{ backgroundColor: `${color}15` }}
              >
                <div
                  className="h-full rounded-full transition-all duration-300"
                  style={{
                    width: `${Math.min((typeCount / 14) * 100, 100)}%`,
                    backgroundColor: color,
                    boxShadow: `0 0 4px ${color}`,
                  }}
                />
              </div>

              {/* Count */}
              <span
                className="text-xs font-bold tabular-nums"
                style={{ color: `${color}99` }}
              >
                {typeCount}
              </span>
            </div>
          </div>
        </div>
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
