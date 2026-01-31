'use client';

/**
 * SchemaFilterPanel - Hierarchical filter UI for Schema Mode
 *
 * Design: Premium glassmorphism matching Data View style
 *
 * Features:
 * - Uses unified FilterSection + FilterCard design system
 * - Tri-state checkboxes for hierarchical selection
 * - Collapsible sections with smooth animations
 * - ARIA accessibility
 */

import { memo, useCallback, useMemo } from 'react';
import { Boxes } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';
import type { Subcategory } from '@novanet/core/graph';
import type { Scope } from '@/types';
import { useFilterStore } from '@/stores/filterStore';
import { cn } from '@/lib/utils';
import { scopeAccents, panelClasses, iconSizes } from '@/design/tokens';
import { FilterSection, type CheckboxState } from '@/components/ui/FilterSection';
import { FilterCard } from '@/components/ui/FilterCard';

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

  // Memoize scope data
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

  // Calculate checkbox state for a scope
  const getScopeCheckboxState = useCallback(
    (scope: Scope): CheckboxState => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const visibleCount = subcatNames.filter(
        (name) => !isSubcategoryCollapsed(scope, name)
      ).length;

      if (visibleCount === 0) return 'none';
      if (visibleCount === subcatNames.length) return 'all';
      return 'partial';
    },
    [isSubcategoryCollapsed]
  );

  // Handle scope checkbox click
  const handleScopeCheckboxClick = useCallback(
    (scope: Scope) => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const currentState = getScopeCheckboxState(scope);

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
      {/* Header - Premium Glassmorphism */}
      <div className={cn('relative', panelClasses.header)}>
        <div className="absolute inset-0 bg-gradient-to-br from-violet-500/5 via-transparent to-emerald-500/5 pointer-events-none" />

        <div className="relative flex items-center gap-3">
          <div className="relative">
            <div className="absolute inset-0 rounded-2xl bg-gradient-to-br from-violet-400 to-emerald-500 opacity-20 blur-lg" />
            <div className="relative w-10 h-10 rounded-xl bg-gradient-to-br from-violet-500/20 to-emerald-500/20 flex items-center justify-center border border-white/10 shadow-lg shadow-black/20">
              <Boxes className={cn(iconSizes.md, 'text-violet-400')} />
            </div>
          </div>

          <div className="flex-1 min-w-0">
            <h2 className="text-sm font-semibold text-white tracking-tight">
              Schema Browser
            </h2>
            <p className="text-[10px] text-white/40 mt-0.5">
              35 node types · 3 scopes
            </p>
          </div>
        </div>
      </div>

      {/* Content - Unified Filter Components */}
      <div className={cn(panelClasses.body, 'space-y-4')}>
        {scopeData.map(({ scope, scopeDef, accent, subcategories, nodeCount }) => (
          <FilterSection
            key={scope}
            id={scope.toLowerCase()}
            label={scopeDef.label}
            icon={<span className="text-base">{scopeDef.icon}</span>}
            accentColor={accent.color}
            checkboxState={getScopeCheckboxState(scope)}
            onCheckboxClick={() => handleScopeCheckboxClick(scope)}
            count={nodeCount}
            defaultExpanded={true}
          >
            {subcategories.map(([subcatName, subcatMeta]) => {
              const isHidden = isSubcategoryCollapsed(scope, subcatName);

              return (
                <FilterCard
                  key={subcatName}
                  id={`${scope}-${subcatName}`}
                  label={subcatMeta.label}
                  icon={<span className="text-sm">{subcatMeta.icon}</span>}
                  accentColor={accent.color}
                  isSelected={!isHidden}
                  onToggle={() => toggleSubcategoryCollapsed(scope, subcatName)}
                  count={subcatMeta.nodeTypes.length}
                  compact
                />
              );
            })}
          </FilterSection>
        ))}
      </div>

      {/* Footer */}
      <div className={cn(panelClasses.footer, 'bg-black/20')}>
        <div className="flex items-center justify-center gap-3">
          <span className="text-[10px] text-violet-400/60">📦 Project</span>
          <span className="text-white/20">·</span>
          <span className="text-[10px] text-emerald-400/60">🌍 Global</span>
          <span className="text-white/20">·</span>
          <span className="text-[10px] text-amber-400/60">🎯 Shared</span>
        </div>
      </div>
    </div>
  );
});
