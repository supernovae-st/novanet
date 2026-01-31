'use client';

/**
 * NodeLabelsSection - Hierarchical node labels explorer
 *
 * Features:
 * - Category-grouped node types using FilterTree
 * - Tri-state checkboxes for selection
 * - Progress bars showing counts
 * - Execute query button
 *
 * Uses FilterTree design system for consistent UI with SchemaFilterPanel
 */

import { memo, useCallback, useMemo } from 'react';
import { NODE_TYPE_CONFIG, NODE_VISUAL_CATEGORIES } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { FilterTree } from '@/components/ui/FilterTree';
import { SelectionHeader } from '@/components/ui/SelectionHeader';
import { calculateCheckboxState } from '@/hooks';
import { iconSizes } from '@/design/tokens';
import { ACTION_ICONS, STATUS_ICONS } from '@/config/iconSystem';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';
import type { NodeType } from '@/types';

const PlayIcon = ACTION_ICONS.execute;
const LoaderIcon = STATUS_ICONS.loading;

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

  // Memoize category data
  const categoryData = useMemo(() => {
    return NODE_VISUAL_CATEGORIES.map((category) => {
      const totalCount = category.nodeTypes.reduce(
        (sum, type) => sum + (labelCounts.get(type) || 0),
        0
      );
      const checkboxState = calculateCheckboxState(category.nodeTypes, selectedLabels);

      return {
        category,
        totalCount,
        checkboxState,
      };
    });
  }, [labelCounts, selectedLabels]);

  // Handle category checkbox click
  const handleCategoryClick = useCallback(
    (categoryId: string, nodeTypes: readonly NodeType[], currentState: CheckboxState) => {
      if (currentState === 'all') {
        onToggleCategoryLabels(categoryId, []);
      } else {
        onToggleCategoryLabels(categoryId, [...nodeTypes]);
      }
    },
    [onToggleCategoryLabels]
  );

  return (
    <section data-testid="node-labels-container">
      <SelectionHeader
        label="All Nodes"
        totalCount={totalNodes}
        selectedCount={selectedLabels.size}
        checkboxState={nodesCheckboxState}
        onToggleAll={onToggleAllNodes}
        onExecute={onExecuteQuery}
        isExecuting={isExecuting}
      />

      <FilterTree.Root showProgressBars={false} maxCount={maxCount} disabled={isExecuting}>
        {/* Category Tree */}
        {categoryData.map(({ category, totalCount, checkboxState }) => (
          <FilterTree.Section
            key={category.id}
            id={category.id}
            label={category.label}
            icon={
              <CategoryIcon
                category={category.id}
                size={16}
                strokeWidth={2}
                style={{ color: category.color }}
              />
            }
            color={category.color}
            checkboxState={checkboxState}
            onCheckboxClick={() =>
              handleCategoryClick(category.id, category.nodeTypes, checkboxState)
            }
            count={totalCount}
            defaultExpanded
          >
            {category.nodeTypes.map((nodeType) => {
              const config = NODE_TYPE_CONFIG[nodeType];
              const color = config?.color || '#6b7280';
              const label = config?.label || nodeType;
              const nodeCategory = config?.category || 'project';
              const count = labelCounts.get(nodeType) || 0;

              return (
                <FilterTree.Row
                  key={nodeType}
                  id={nodeType}
                  label={label}
                  icon={
                    <CategoryIcon
                      category={nodeCategory}
                      size={14}
                      strokeWidth={2}
                      className="flex-shrink-0"
                      style={{ color }}
                    />
                  }
                  color={color}
                  isSelected={selectedLabels.has(nodeType)}
                  onToggle={() => onToggleLabel(nodeType)}
                  count={count}
                />
              );
            })}
          </FilterTree.Section>
        ))}
      </FilterTree.Root>
    </section>
  );
});
