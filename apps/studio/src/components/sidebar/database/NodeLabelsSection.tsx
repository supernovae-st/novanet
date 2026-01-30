'use client';

/**
 * NodeLabelsSection - Hierarchical node labels explorer
 *
 * Features:
 * - Category-grouped node types
 * - Tri-state checkboxes for selection
 * - Progress bars showing counts
 * - Execute query button
 */

import { memo, useCallback, useMemo, useState } from 'react';
import { cn } from '@/lib/utils';
import { GRAPH_ICONS, ICON_COLORS, ACTION_ICONS, STATUS_ICONS, NAV_ICONS } from '@/config/iconSystem';
import { TriStateCheckbox, type CheckboxState } from '@/components/ui/TriStateCheckbox';
import { ProgressBar } from '@/components/ui/ProgressBar';
import { NODE_TYPE_CONFIG, NODE_VISUAL_CATEGORIES, type CategoryConfig } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { calculateCheckboxState } from '@/hooks';
import type { NodeType } from '@/types';

// Design system icons
const LoaderIcon = STATUS_ICONS.loading;
const PlayIcon = ACTION_ICONS.execute;
const ChevronDownIcon = NAV_ICONS.chevronDown;
const CheckIcon = STATUS_ICONS.success;

// =============================================================================
// NODE TYPE ROW
// =============================================================================

interface NodeTypeRowProps {
  nodeType: NodeType;
  count: number;
  maxCount: number;
  isSelected: boolean;
  onToggle: () => void;
  disabled?: boolean;
}

const NodeTypeRow = memo(function NodeTypeRow({
  nodeType,
  count,
  maxCount,
  isSelected,
  onToggle,
  disabled,
}: NodeTypeRowProps) {
  const config = NODE_TYPE_CONFIG[nodeType];
  const color = config?.color || '#6b7280';
  const label = config?.label || nodeType;
  const category = config?.category || 'project';

  return (
    <button
      onClick={onToggle}
      disabled={disabled}
      role="checkbox"
      aria-checked={isSelected}
      aria-label={`${label} (${count} nodes)`}
      data-selected={isSelected}
      className={cn(
        'group w-full flex items-center gap-2.5 py-2 px-2.5 -mx-2 rounded-xl transition-all duration-200',
        'hover:bg-white/[0.04] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
        isSelected && 'bg-white/[0.06]',
        disabled && 'opacity-50 cursor-not-allowed'
      )}
    >
      {/* Checkbox */}
      <div
        className={cn(
          'w-4 h-4 rounded-md border-[1.5px] flex items-center justify-center transition-all duration-200 flex-shrink-0',
          isSelected ? 'border-transparent' : 'border-white/20'
        )}
        style={{
          backgroundColor: isSelected ? `${color}30` : 'transparent',
          borderColor: isSelected ? color : undefined,
        }}
      >
        {isSelected && <CheckIcon className="w-2.5 h-2.5" style={{ color }} />}
      </div>

      {/* Icon */}
      <CategoryIcon
        category={category}
        size={14}
        strokeWidth={2}
        className="flex-shrink-0"
        style={{ color }}
      />

      {/* Label */}
      <span
        className={cn(
          'text-[12px] font-medium transition-colors duration-200 flex-1 text-left truncate',
          isSelected ? 'text-white' : 'text-white/60 group-hover:text-white/80'
        )}
      >
        {label}
      </span>

      {/* Progress bar */}
      <div className="w-16 flex-shrink-0">
        <ProgressBar value={count} max={maxCount} color={color} />
      </div>

      {/* Count */}
      <span
        className={cn(
          'text-[11px] font-mono w-7 text-right transition-colors duration-200 flex-shrink-0',
          isSelected ? 'text-white/80' : 'text-white/40'
        )}
      >
        {count > 999 ? `${(count / 1000).toFixed(1)}k` : count}
      </span>
    </button>
  );
});

// =============================================================================
// CATEGORY SECTION
// =============================================================================

interface CategorySectionProps {
  category: CategoryConfig;
  counts: Map<string, number>;
  maxCount: number;
  selectedTypes: Set<string>;
  onToggleType: (type: string) => void;
  onToggleAll: (types: string[]) => void;
  disabled?: boolean;
  defaultExpanded?: boolean;
}

const CategorySection = memo(function CategorySection({
  category,
  counts,
  maxCount,
  selectedTypes,
  onToggleType,
  onToggleAll,
  disabled,
  defaultExpanded = true,
}: CategorySectionProps) {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);

  // Calculate checkbox state using shared utility
  const checkboxState = useMemo((): CheckboxState => {
    return calculateCheckboxState(category.nodeTypes, selectedTypes);
  }, [category.nodeTypes, selectedTypes]);

  // Calculate total count for category
  const totalCount = useMemo(() => {
    return category.nodeTypes.reduce((sum, type) => sum + (counts.get(type) || 0), 0);
  }, [category.nodeTypes, counts]);

  // Handle category checkbox click
  const handleCategoryClick = useCallback(() => {
    if (checkboxState === 'all') {
      onToggleAll([]);
    } else {
      onToggleAll(category.nodeTypes);
    }
  }, [checkboxState, category.nodeTypes, onToggleAll]);

  return (
    <div className="mb-1">
      {/* Category Header */}
      <div
        className={cn(
          'flex items-center gap-2 py-2 px-1 rounded-lg transition-all duration-200',
          'hover:bg-white/[0.03] cursor-pointer select-none'
        )}
      >
        {/* Expand/Collapse */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          aria-expanded={isExpanded}
          aria-label={`${isExpanded ? 'Collapse' : 'Expand'} ${category.label} category`}
          className="p-0.5 -m-0.5 rounded transition-colors hover:bg-white/[0.06] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50"
        >
          <ChevronDownIcon
            className={cn(
              'w-3.5 h-3.5 text-white/40 transition-transform duration-200',
              !isExpanded && '-rotate-90'
            )}
          />
        </button>

        {/* Tri-state Checkbox */}
        <TriStateCheckbox
          state={checkboxState}
          onClick={handleCategoryClick}
          color={category.color}
          disabled={disabled}
          label={`Select all ${category.label} nodes`}
        />

        {/* Icon + Label */}
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex items-center gap-2 flex-1"
        >
          <CategoryIcon
            category={category.id}
            size={16}
            strokeWidth={2}
            style={{ color: category.color }}
          />
          <span
            className="text-[13px] font-semibold"
            style={{ color: category.color }}
          >
            {category.label}
          </span>
          <span className="text-[11px] text-white/40">
            ({totalCount.toLocaleString()})
          </span>
        </button>
      </div>

      {/* Node Types List */}
      <div
        className={cn(
          'ml-6 pl-3 border-l border-white/[0.06] overflow-hidden transition-all duration-300',
          isExpanded ? 'max-h-[500px] opacity-100' : 'max-h-0 opacity-0'
        )}
      >
        {category.nodeTypes.map((nodeType) => (
          <NodeTypeRow
            key={nodeType}
            nodeType={nodeType}
            count={counts.get(nodeType) || 0}
            maxCount={maxCount}
            isSelected={selectedTypes.has(nodeType)}
            onToggle={() => onToggleType(nodeType)}
            disabled={disabled}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// MAIN SECTION COMPONENT
// =============================================================================

export interface NodeLabelsSectionProps {
  /** Total nodes in database */
  totalNodes: number;
  /** Map of label -> count */
  labelCounts: Map<string, number>;
  /** Maximum count for progress bars */
  maxCount: number;
  /** Currently selected labels */
  selectedLabels: Set<string>;
  /** Callback when label is toggled */
  onToggleLabel: (label: string) => void;
  /** Callback when category labels are toggled */
  onToggleCategoryLabels: (categoryId: string, types: string[]) => void;
  /** Callback when all nodes toggled */
  onToggleAllNodes: () => void;
  /** Callback to execute query */
  onExecuteQuery: () => void;
  /** Whether query is executing */
  isExecuting?: boolean;
}

export const NodeLabelsSection = memo(function NodeLabelsSection({
  totalNodes,
  labelCounts,
  maxCount,
  selectedLabels,
  onToggleLabel,
  onToggleCategoryLabels,
  onToggleAllNodes,
  onExecuteQuery,
  isExecuting = false,
}: NodeLabelsSectionProps) {
  // Calculate nodes checkbox state
  const nodesCheckboxState = useMemo((): CheckboxState => {
    const allLabels = Array.from(labelCounts.keys());
    if (allLabels.length === 0) return 'none';
    return calculateCheckboxState(allLabels, selectedLabels);
  }, [labelCounts, selectedLabels]);

  return (
    <section>
      {/* Section Header */}
      <div className="flex items-center justify-between px-1 mb-2">
        <div className="flex items-center gap-2">
          <TriStateCheckbox
            state={nodesCheckboxState}
            onClick={onToggleAllNodes}
            color={ICON_COLORS.node.primary}
            disabled={isExecuting}
            label="Select all nodes"
          />
          <GRAPH_ICONS.node className={cn('w-3.5 h-3.5', ICON_COLORS.node.muted)} />
          <span className="text-xs font-semibold text-white/70">Nodes</span>
          <span className="text-[10px] text-white/40">
            ({totalNodes.toLocaleString()})
          </span>
          {selectedLabels.size > 0 && (
            <span className="px-1.5 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 text-[10px] font-semibold animate-in fade-in duration-200">
              {selectedLabels.size} selected
            </span>
          )}
        </div>
        <button
          onClick={onExecuteQuery}
          disabled={selectedLabels.size === 0 || isExecuting}
          aria-label={`Execute query for ${selectedLabels.size} selected node types`}
          data-testid="execute-node-query"
          className={cn(
            'p-1.5 rounded-lg transition-all duration-200',
            selectedLabels.size > 0
              ? 'text-emerald-400 hover:text-emerald-300 hover:bg-emerald-500/10 hover:scale-110'
              : 'text-white/20 cursor-not-allowed'
          )}
          title="Execute query for selected labels"
        >
          {isExecuting ? (
            <LoaderIcon className="w-4 h-4 animate-spin" />
          ) : (
            <PlayIcon className="w-4 h-4" />
          )}
        </button>
      </div>

      {/* Category Tree */}
      <div className="space-y-0.5" data-testid="node-labels-container">
        {NODE_VISUAL_CATEGORIES.map((category) => (
          <CategorySection
            key={category.id}
            category={category}
            counts={labelCounts}
            maxCount={maxCount}
            selectedTypes={selectedLabels}
            onToggleType={onToggleLabel}
            onToggleAll={(types) => onToggleCategoryLabels(category.id, types)}
            disabled={isExecuting}
          />
        ))}
      </div>
    </section>
  );
});
