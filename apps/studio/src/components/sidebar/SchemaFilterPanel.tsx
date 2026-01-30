'use client';

/**
 * SchemaFilterPanel - Hierarchical filter UI for Schema Mode
 *
 * Features:
 * - Shows all 3 scopes with their subcategories
 * - Each scope is collapsible (shows chevron)
 * - Each subcategory is toggleable (shows node count)
 * - Uses SCOPE_HIERARCHY from @novanet/core/graph
 * - Uses filterStore actions for toggling
 * - ARIA accessibility attributes for screen readers
 *
 * @see Tasks 4.1 and 4.2 from docs/plans/2026-01-30-schema-mode-v2.md
 */

import { memo, useCallback } from 'react';
import { ChevronDown, ChevronRight, Layers } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';
import type { Subcategory } from '@novanet/core/graph';
import type { Scope } from '@/types';
import { useFilterStore } from '@/stores/filterStore';
import { cn } from '@/lib/utils';

// Ordered scopes for consistent rendering
const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

// Scope-specific colors for visual distinction
const SCOPE_COLORS: Record<Scope, { border: string; bg: string; accent: string }> = {
  Project: {
    border: 'border-violet-500/30',
    bg: 'bg-violet-500/5',
    accent: 'text-violet-400',
  },
  Global: {
    border: 'border-emerald-500/30',
    bg: 'bg-emerald-500/5',
    accent: 'text-emerald-400',
  },
  Shared: {
    border: 'border-amber-500/30',
    bg: 'bg-amber-500/5',
    accent: 'text-amber-400',
  },
};

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const {
    collapsedScopes,
    collapsedSubcategories,
    toggleScopeCollapsed,
    toggleSubcategoryCollapsed,
    isScopeCollapsed,
    isSubcategoryCollapsed,
  } = useFilterStore(
    useShallow((state) => ({
      collapsedScopes: state.collapsedScopes,
      collapsedSubcategories: state.collapsedSubcategories,
      toggleScopeCollapsed: state.toggleScopeCollapsed,
      toggleSubcategoryCollapsed: state.toggleSubcategoryCollapsed,
      isScopeCollapsed: state.isScopeCollapsed,
      isSubcategoryCollapsed: state.isSubcategoryCollapsed,
    }))
  );

  // Compute total node count for a scope
  const getScopeNodeCount = useCallback((scope: Scope): number => {
    const scopeDef = SCOPE_HIERARCHY[scope];
    return Object.values(scopeDef.subcategories).reduce(
      (sum, subcat) => sum + subcat.nodeTypes.length,
      0
    );
  }, []);

  return (
    <div
      className={cn('flex flex-col h-full', className)}
      data-testid="schema-filter-panel"
      role="region"
      aria-label="Schema filters"
    >
      {/* Header - Premium Glassmorphism */}
      <div className="relative px-4 py-5 border-b border-white/[0.08]">
        {/* Background glow effect */}
        <div
          className={cn(
            'absolute inset-0 bg-gradient-to-br pointer-events-none',
            'from-cyan-500/5 via-transparent to-violet-500/5'
          )}
        />

        <div className="relative flex items-center gap-3">
          {/* Icon with animated gradient */}
          <div className="relative">
            <div
              className={cn(
                'absolute inset-0 rounded-2xl bg-gradient-to-br',
                'from-cyan-400 to-violet-500 opacity-20 blur-lg'
              )}
            />
            <div
              className={cn(
                'relative w-11 h-11 rounded-2xl bg-gradient-to-br flex items-center justify-center',
                'from-cyan-500/20 to-violet-500/20 border border-white/10',
                'shadow-lg shadow-black/20'
              )}
            >
              <Layers className="w-5 h-5 text-cyan-400" />
            </div>
          </div>

          <div className="flex-1">
            <h2 className="text-[15px] font-semibold text-white tracking-tight">
              Schema Filters
            </h2>
            <p className="text-[11px] text-white/40 mt-0.5">
              35 node types across 3 scopes
            </p>
          </div>
        </div>
      </div>

      {/* Content - Scrollable */}
      <div className="flex-1 overflow-y-auto scrollbar-thin p-3 space-y-3">
        {SCOPE_ORDER.map((scope) => {
          const scopeDef = SCOPE_HIERARCHY[scope];
          const isCollapsed = isScopeCollapsed(scope);
          const nodeCount = getScopeNodeCount(scope);
          const colors = SCOPE_COLORS[scope];
          const subcategories = Object.entries(scopeDef.subcategories) as [
            Subcategory,
            typeof scopeDef.subcategories[Subcategory]
          ][];

          return (
            <div
              key={scope}
              className={cn(
                'rounded-xl border',
                colors.border,
                colors.bg
              )}
            >
              {/* Scope Header - Clickable to collapse/expand */}
              <button
                onClick={() => toggleScopeCollapsed(scope)}
                className={cn(
                  'w-full flex items-center justify-between px-3 py-2.5 rounded-xl',
                  'bg-white/5 hover:bg-white/10 transition-colors',
                  'text-left focus:outline-none focus:ring-2 focus:ring-white/20'
                )}
                aria-expanded={!isCollapsed}
                aria-controls={`scope-${scope}-content`}
              >
                <span className="flex items-center gap-2 text-sm font-medium text-white/90">
                  <span className="text-base">{scopeDef.icon}</span>
                  <span>{scopeDef.label}</span>
                  <span className={cn('text-xs font-normal', colors.accent)}>
                    {nodeCount}
                  </span>
                </span>
                <span className="text-white/40">
                  {isCollapsed ? (
                    <ChevronRight className="w-4 h-4" aria-hidden="true" />
                  ) : (
                    <ChevronDown className="w-4 h-4" aria-hidden="true" />
                  )}
                </span>
              </button>

              {/* Subcategories - Collapsible */}
              {!isCollapsed && (
                <div
                  id={`scope-${scope}-content`}
                  className="px-2 pb-2 space-y-0.5"
                  role="group"
                  aria-label={`${scopeDef.label} subcategories`}
                >
                  {subcategories.map(([subcatName, subcatMeta]) => {
                    const isSubCollapsed = isSubcategoryCollapsed(scope, subcatName);
                    const count = subcatMeta.nodeTypes.length;

                    return (
                      <button
                        key={subcatName}
                        onClick={() => toggleSubcategoryCollapsed(scope, subcatName)}
                        className={cn(
                          'w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg',
                          'bg-white/5 hover:bg-white/10 transition-colors',
                          'text-left text-sm focus:outline-none focus:ring-1 focus:ring-white/20',
                          isSubCollapsed ? 'opacity-50' : 'opacity-100'
                        )}
                        aria-pressed={!isSubCollapsed}
                        aria-label={`${subcatMeta.label}: ${count} node types. ${
                          isSubCollapsed ? 'Hidden' : 'Visible'
                        }`}
                      >
                        <span className="flex items-center gap-1.5 text-white/80">
                          <span className="text-sm" aria-hidden="true">
                            {subcatMeta.icon}
                          </span>
                          <span>{subcatMeta.label}</span>
                        </span>
                        <span className="text-xs text-white/50">({count})</span>
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          );
        })}

        {/* Divider */}
        <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent my-2" />

        {/* Stats Footer */}
        <div className="px-2 py-2 text-center">
          <p className="text-xs text-white/40">
            35 node types &bull; 9 subcategories &bull; 3 scopes
          </p>
        </div>
      </div>
    </div>
  );
});
