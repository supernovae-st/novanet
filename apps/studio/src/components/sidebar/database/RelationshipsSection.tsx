'use client';

/**
 * RelationshipsSection - Hierarchical relationship types explorer
 *
 * Features:
 * - Category-grouped relationship types using FilterTree
 * - Tri-state checkboxes for selection
 * - Progress bars showing counts
 *
 * Uses FilterTree design system for consistent UI with NodeLabelsSection
 */

import { memo, useCallback, useMemo } from 'react';
import { ArrowRight } from 'lucide-react';
import {
  RELATIONSHIP_VISUAL_CATEGORIES,
  relationshipTypeConfigs,
  type RelationshipCategory,
} from '@/config/relationshipTypes';
import { FilterTree } from '@/components/ui/FilterTree';
import { calculateCheckboxState } from '@/hooks';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';
import type { RelationType } from '@/hooks';
import { iconSizes } from '@/design/tokens';

// =============================================================================
// MAIN SECTION COMPONENT
// =============================================================================

export interface RelationshipsSectionProps {
  /** Total relationships in database */
  totalRelationships: number;
  /** Relationship types with counts */
  relationshipTypes: RelationType[];
  /** Maximum count for progress bars */
  maxCount: number;
  /** Currently selected relationship types */
  selectedRelTypes: Set<string>;
  /** Callback when relationship type is toggled */
  onToggleRelType: (type: string) => void;
  /** Callback when category relationship types are toggled */
  onToggleCategoryRelTypes?: (categoryId: string, types: string[]) => void;
  /** Callback when all relationships toggled */
  onToggleAllRelTypes: () => void;
  /** Callback to execute query */
  onExecuteQuery: () => void;
  /** Whether query is executing */
  isExecuting?: boolean;
}

export const RelationshipsSection = memo(function RelationshipsSection({
  relationshipTypes,
  maxCount,
  selectedRelTypes,
  onToggleRelType,
  onToggleCategoryRelTypes,
  isExecuting = false,
}: RelationshipsSectionProps) {
  // Create a map of type -> count for quick lookup
  const typeCountMap = useMemo(() => {
    const map = new Map<string, number>();
    relationshipTypes.forEach((r) => map.set(r.type, r.count));
    return map;
  }, [relationshipTypes]);

  // Memoize category data
  const categoryData = useMemo(() => {
    return RELATIONSHIP_VISUAL_CATEGORIES.map((category) => {
      // Filter to only include relation types that exist in the database
      const existingTypes = category.relationTypes.filter((type) => typeCountMap.has(type));
      const totalCount = existingTypes.reduce(
        (sum, type) => sum + (typeCountMap.get(type) || 0),
        0
      );
      const checkboxState = calculateCheckboxState(existingTypes, selectedRelTypes);

      return {
        category,
        existingTypes,
        totalCount,
        checkboxState,
      };
    }).filter((data) => data.existingTypes.length > 0); // Only show categories with data
  }, [typeCountMap, selectedRelTypes]);

  // Handle category checkbox click
  const handleCategoryClick = useCallback(
    (categoryId: RelationshipCategory, relationTypes: string[], currentState: CheckboxState) => {
      if (onToggleCategoryRelTypes) {
        if (currentState === 'all') {
          onToggleCategoryRelTypes(categoryId, []);
        } else {
          onToggleCategoryRelTypes(categoryId, relationTypes);
        }
      } else {
        // Fallback: toggle each type individually
        relationTypes.forEach((type) => {
          const isSelected = selectedRelTypes.has(type);
          if (currentState === 'all' && isSelected) {
            onToggleRelType(type);
          } else if (currentState !== 'all' && !isSelected) {
            onToggleRelType(type);
          }
        });
      }
    },
    [onToggleCategoryRelTypes, onToggleRelType, selectedRelTypes]
  );

  return (
    <section data-testid="relationships-container">
      <FilterTree.Root showProgressBars={true} maxCount={maxCount} disabled={isExecuting}>
        {/* Category Tree */}
        {categoryData.map(({ category, existingTypes, totalCount, checkboxState }) => (
          <FilterTree.Section
            key={category.id}
            id={category.id}
            label={category.label}
            icon={<span className="text-sm">{category.icon}</span>}
            color={category.color}
            checkboxState={checkboxState}
            onCheckboxClick={() =>
              handleCategoryClick(category.id, existingTypes, checkboxState)
            }
            count={totalCount}
            defaultExpanded
          >
            {existingTypes.map((relType) => {
              const config = relationshipTypeConfigs[relType];
              const color = config?.color || '#6b7280';
              const label = config?.label || relType;
              const count = typeCountMap.get(relType) || 0;

              return (
                <FilterTree.Row
                  key={relType}
                  id={relType}
                  label={label}
                  icon={<ArrowRight className={iconSizes.sm} />}
                  color={color}
                  isSelected={selectedRelTypes.has(relType)}
                  onToggle={() => onToggleRelType(relType)}
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
