'use client';

/**
 * SchemaFilterPanel - Hierarchical filter UI for Schema Mode
 *
 * Features:
 * - Shows all 3 scopes with their subcategories
 * - Uses unified FilterTree design system
 * - Tri-state checkboxes for visibility toggling
 * - Flat tree structure with border-left connectors
 * - ARIA accessibility attributes for screen readers
 */

import { memo, useCallback, useMemo } from 'react';
import { Boxes } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';
import type { Subcategory } from '@novanet/core/graph';
import type { Scope } from '@/types';
import { useFilterStore } from '@/stores/filterStore';
import { cn } from '@/lib/utils';
import { scopeAccents, panelClasses } from '@/design/tokens';
import { FilterTree } from '@/components/ui/FilterTree';
import { calculateCheckboxState } from '@/hooks';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';

// Ordered scopes for consistent rendering
const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

// Map scope names to accent keys
const SCOPE_ACCENT_MAP: Record<Scope, keyof typeof scopeAccents> = {
  Project: 'project',
  Global: 'global',
  Shared: 'shared',
};

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const {
    toggleSubcategoryCollapsed,
    isSubcategoryCollapsed,
    setSubcategoryCollapsed,
  } = useFilterStore(
    useShallow((state) => ({
      toggleSubcategoryCollapsed: state.toggleSubcategoryCollapsed,
      isSubcategoryCollapsed: state.isSubcategoryCollapsed,
      setSubcategoryCollapsed: state.setSubcategoryCollapsed,
    }))
  );

  // Memoize scope data to avoid recomputing on each render
  const scopeData = useMemo(() => {
    return SCOPE_ORDER.map((scope) => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const accentKey = SCOPE_ACCENT_MAP[scope];
      const accent = scopeAccents[accentKey];
      const subcategories = Object.entries(scopeDef.subcategories) as [
        Subcategory,
        (typeof scopeDef.subcategories)[Subcategory],
      ][];
      const nodeCount = subcategories.reduce(
        (sum, [, subcat]) => sum + subcat.nodeTypes.length,
        0
      );

      return {
        scope,
        scopeDef,
        accent,
        subcategories,
        nodeCount,
      };
    });
  }, []);

  // Calculate checkbox state for a scope (all subcategories visible vs some vs none)
  const getScopeCheckboxState = useCallback(
    (scope: Scope): CheckboxState => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const visibleSet = new Set(
        subcatNames.filter((name) => !isSubcategoryCollapsed(scope, name))
      );
      return calculateCheckboxState(subcatNames, visibleSet);
    },
    [isSubcategoryCollapsed]
  );

  // Handle scope checkbox click (toggle all subcategories in scope)
  const handleScopeCheckboxClick = useCallback(
    (scope: Scope) => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const currentState = getScopeCheckboxState(scope);

      // If all or partial visible, collapse all. If none visible, show all.
      const shouldCollapse = currentState !== 'none';
      subcatNames.forEach((name) => {
        setSubcategoryCollapsed(scope, name, shouldCollapse);
      });
    },
    [getScopeCheckboxState, setSubcategoryCollapsed]
  );

  return (
    <div
      className={cn('flex flex-col h-full', className)}
      data-testid="schema-filter-panel"
      role="region"
      aria-label="Schema filters"
    >
      {/* Header - Matching Data View style */}
      <div className={panelClasses.header}>
        <div className={panelClasses.headerContent}>
          <div className={panelClasses.headerIconBox}>
            <Boxes className={panelClasses.headerIcon} />
          </div>
          <div>
            <h2 className={panelClasses.headerTitle}>Schema Browser</h2>
            <p className={panelClasses.headerSubtitle}>35 node types</p>
          </div>
        </div>
      </div>

      {/* Content - FilterTree */}
      <div className={panelClasses.body}>
        <FilterTree.Root>
          {scopeData.map(({ scope, scopeDef, accent, subcategories, nodeCount }) => (
            <FilterTree.Section
              key={scope}
              id={scope.toLowerCase()}
              label={scopeDef.label}
              icon={<span className="text-base">{scopeDef.icon}</span>}
              color={accent.color}
              checkboxState={getScopeCheckboxState(scope)}
              onCheckboxClick={() => handleScopeCheckboxClick(scope)}
              count={nodeCount}
              defaultExpanded={true}
            >
              {subcategories.map(([subcatName, subcatMeta]) => {
                const isHidden = isSubcategoryCollapsed(scope, subcatName);

                return (
                  <FilterTree.Row
                    key={subcatName}
                    id={`${scope}-${subcatName}`}
                    label={subcatMeta.label}
                    icon={<span className="text-sm">{subcatMeta.icon}</span>}
                    color={accent.color}
                    isSelected={!isHidden}
                    onToggle={() => toggleSubcategoryCollapsed(scope, subcatName)}
                    count={subcatMeta.nodeTypes.length}
                  />
                );
              })}
            </FilterTree.Section>
          ))}
        </FilterTree.Root>
      </div>

      {/* Footer Stats - Minimal */}
      <div className={panelClasses.footer}>
        <p className={panelClasses.footerText}>
          3 scopes &middot; 9 categories &middot; 35 types
        </p>
      </div>
    </div>
  );
});
