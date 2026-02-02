'use client';

/**
 * LabelFilter - Neo4j Browser style node label filter
 *
 * Features:
 * - Counts nodes by type from graphStore.nodes
 * - Groups by category (from NODE_TYPE_CONFIG)
 * - Toggle buttons with icon, label, and count badge
 * - Shift+click = select ONLY this type
 * - Regular click = toggle this type
 * - Glassmorphism design matching existing sidebar
 */

import { useMemo, memo, useCallback } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { useGraphStore } from '@/stores/graphStore';
import { useFilterStore } from '@/stores/filterStore';
import { NODE_TYPE_CONFIG, NODE_VISUAL_LAYERS, type LayerConfig } from '@/config/nodeTypes';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { NodeType } from '@/types';

// =============================================================================
// LABEL BUTTON
// =============================================================================

interface LabelButtonProps {
  nodeType: NodeType;
  count: number;
  isSelected: boolean;
  onToggle: (type: NodeType, exclusive: boolean) => void;
}

const LabelButton = memo(function LabelButton({
  nodeType,
  count,
  isSelected,
  onToggle,
}: LabelButtonProps) {
  const config = NODE_TYPE_CONFIG[nodeType];
  const color = config?.color || '#6b7280';
  const label = config?.label || nodeType;
  const layer = config?.layer || 'foundation';

  const handleClick = useCallback(
    (e: React.MouseEvent) => {
      onToggle(nodeType, e.shiftKey);
    },
    [nodeType, onToggle]
  );

  // Skip if no nodes of this type
  if (count === 0) return null;

  return (
    <button
      onClick={handleClick}
      title={`${label} (${count} nodes) - Shift+click to select only this type`}
      aria-label={`${label}: ${count} nodes. ${isSelected ? 'Selected' : 'Not selected'}. Shift+click to select only this type.`}
      aria-pressed={isSelected}
      className={cn(
        'flex items-center justify-between px-2.5 py-1.5 rounded-lg',
        gapTokens.default,
        'text-sm transition-colors duration-200',
        isSelected
          ? 'bg-white/10 text-white'
          : 'text-white/60 hover:bg-white/5 hover:text-white/80'
      )}
    >
      <span className={cn('flex items-center min-w-0', gapTokens.default)}>
        <LayerIcon
          layer={layer}
          size={14}
          strokeWidth={2}
          className="flex-shrink-0"
          style={{ color }}
        />
        <span className="truncate">{label}</span>
      </span>
      <span
        className="px-1.5 py-0.5 rounded text-[10px] font-mono flex-shrink-0"
        style={{
          backgroundColor: `${color}30`,
          color: color,
        }}
      >
        {count > 999 ? `${(count / 1000).toFixed(1)}k` : count}
      </span>
    </button>
  );
});

// =============================================================================
// CATEGORY GROUP
// =============================================================================

interface LayerGroupProps {
  layerConfig: LayerConfig;
  typeCounts: Map<NodeType, number>;
  enabledTypes: Set<NodeType>;
  onToggle: (type: NodeType, exclusive: boolean) => void;
}

const LayerGroup = memo(function LayerGroup({
  layerConfig,
  typeCounts,
  enabledTypes,
  onToggle,
}: LayerGroupProps) {
  // Filter to types that have nodes in the graph
  const visibleTypes = useMemo(() => {
    return layerConfig.nodeTypes.filter((t) => (typeCounts.get(t) || 0) > 0);
  }, [layerConfig.nodeTypes, typeCounts]);

  // Calculate total count for this category
  const totalCount = useMemo(() => {
    return visibleTypes.reduce((sum, t) => sum + (typeCounts.get(t) || 0), 0);
  }, [visibleTypes, typeCounts]);

  // Skip empty categories
  if (visibleTypes.length === 0) return null;

  return (
    <div className="space-y-1">
      {/* Category Header */}
      <div className={cn('flex items-center px-1 py-1', gapTokens.default)}>
        <LayerIcon
          layer={layerConfig.id}
          size={14}
          strokeWidth={2}
          style={{ color: layerConfig.color }}
        />
        <span
          className="text-[11px] font-semibold uppercase tracking-wider"
          style={{ color: layerConfig.color }}
        >
          {layerConfig.label}
        </span>
        <span className="text-[10px] text-white/40">
          ({totalCount.toLocaleString()})
        </span>
      </div>

      {/* Node Type Buttons */}
      <div className="flex flex-col gap-0.5">
        {visibleTypes.map((nodeType) => (
          <LabelButton
            key={nodeType}
            nodeType={nodeType}
            count={typeCounts.get(nodeType) || 0}
            isSelected={enabledTypes.has(nodeType)}
            onToggle={onToggle}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export interface LabelFilterProps {
  className?: string;
}

export const LabelFilter = memo(function LabelFilter({ className }: LabelFilterProps) {
  // Get nodes from graph store
  const nodes = useGraphStore((state) => state.nodes);

  // Get filter state and actions - single subscription with useShallow
  const { enabledNodeTypes, toggleNodeType, setEnabledNodeTypes } = useFilterStore(
    useShallow((state) => ({
      enabledNodeTypes: state.enabledNodeTypes,
      toggleNodeType: state.toggleNodeType,
      setEnabledNodeTypes: state.setEnabledNodeTypes,
    }))
  );

  // Count nodes by type
  const typeCounts = useMemo(() => {
    const counts = new Map<NodeType, number>();
    for (const node of nodes) {
      counts.set(node.type, (counts.get(node.type) || 0) + 1);
    }
    return counts;
  }, [nodes]);

  // Calculate total visible nodes
  const totalNodes = useMemo(() => {
    let total = 0;
    for (const count of typeCounts.values()) {
      total += count;
    }
    return total;
  }, [typeCounts]);

  // Handle toggle with exclusive mode (Shift+click)
  const handleToggle = useCallback(
    (type: NodeType, exclusive: boolean) => {
      if (exclusive) {
        // Shift+click: select ONLY this type
        setEnabledNodeTypes([type]);
      } else {
        // Regular click: toggle this type
        toggleNodeType(type);
      }
    },
    [setEnabledNodeTypes, toggleNodeType]
  );

  // Count selected types that have nodes
  const selectedCount = useMemo(() => {
    let count = 0;
    for (const type of enabledNodeTypes) {
      if (typeCounts.has(type)) {
        count++;
      }
    }
    return count;
  }, [enabledNodeTypes, typeCounts]);

  // Calculate visible node count (nodes matching enabled types)
  const visibleNodeCount = useMemo(() => {
    let count = 0;
    for (const type of enabledNodeTypes) {
      count += typeCounts.get(type) || 0;
    }
    return count;
  }, [enabledNodeTypes, typeCounts]);

  if (nodes.length === 0) {
    return (
      <div className={cn('px-3 py-4', className)}>
        <p className="text-sm text-white/40 text-center">
          No nodes loaded
        </p>
      </div>
    );
  }

  return (
    <div className={cn('space-y-4', className)}>
      {/* Header */}
      <div className="px-1">
        <h3 className="text-sm font-medium text-white/70 uppercase tracking-wider flex items-center justify-between">
          <span>Node Labels</span>
          <span className="text-[10px] text-white/40 normal-case">
            {visibleNodeCount.toLocaleString()} / {totalNodes.toLocaleString()}
          </span>
        </h3>
        {selectedCount > 0 && (
          <p className="text-[10px] text-white/40 mt-1">
            {selectedCount} type{selectedCount !== 1 ? 's' : ''} selected
            <span className="ml-2 text-white/40">Shift+click for exclusive</span>
          </p>
        )}
      </div>

      {/* Category Groups */}
      <div className="space-y-3">
        {NODE_VISUAL_LAYERS.map((layerConfig) => (
          <LayerGroup
            key={layerConfig.id}
            layerConfig={layerConfig}
            typeCounts={typeCounts}
            enabledTypes={enabledNodeTypes}
            onToggle={handleToggle}
          />
        ))}
      </div>
    </div>
  );
});
