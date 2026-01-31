'use client';

/**
 * SchemaFilterPanel - Hierarchical filter UI for Schema Mode
 *
 * Features:
 * - Shows all 3 scopes with their subcategories
 * - Each scope is collapsible with scope-colored accents
 * - Each subcategory is toggleable (shows node count)
 * - Premium glassmorphism design with subtle glow effects
 * - ARIA accessibility attributes for screen readers
 */

import { memo, useCallback } from 'react';
import { ChevronDown, Boxes } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';
import type { Subcategory } from '@novanet/core/graph';
import type { Scope } from '@/types';
import { useFilterStore } from '@/stores/filterStore';
import { cn } from '@/lib/utils';

// Ordered scopes for consistent rendering
const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

// Scope-specific colors for visual distinction
const SCOPE_COLORS: Record<
  Scope,
  {
    border: string;
    borderHover: string;
    glow: string;
    accent: string;
    badge: string;
    headerBg: string;
  }
> = {
  Project: {
    border: 'border-violet-500/20',
    borderHover: 'hover:border-violet-500/40',
    glow: 'shadow-violet-500/10',
    accent: 'text-violet-400',
    badge: 'bg-violet-500/20 text-violet-300',
    headerBg: 'from-violet-500/10 to-transparent',
  },
  Global: {
    border: 'border-emerald-500/20',
    borderHover: 'hover:border-emerald-500/40',
    glow: 'shadow-emerald-500/10',
    accent: 'text-emerald-400',
    badge: 'bg-emerald-500/20 text-emerald-300',
    headerBg: 'from-emerald-500/10 to-transparent',
  },
  Shared: {
    border: 'border-amber-500/20',
    borderHover: 'hover:border-amber-500/40',
    glow: 'shadow-amber-500/10',
    accent: 'text-amber-400',
    badge: 'bg-amber-500/20 text-amber-300',
    headerBg: 'from-amber-500/10 to-transparent',
  },
};

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const {
    toggleScopeCollapsed,
    toggleSubcategoryCollapsed,
    isScopeCollapsed,
    isSubcategoryCollapsed,
  } = useFilterStore(
    useShallow((state) => ({
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
      {/* Header - Minimal & Clean */}
      <div className="px-4 py-4 border-b border-white/[0.06]">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-white/10 to-white/5 border border-white/10 flex items-center justify-center">
            <Boxes className="w-4 h-4 text-white/60" />
          </div>
          <div>
            <h2 className="text-sm font-medium text-white/90">Schema Browser</h2>
            <p className="text-[11px] text-white/40">35 node types</p>
          </div>
        </div>
      </div>

      {/* Content - Scrollable */}
      <div className="flex-1 overflow-y-auto scrollbar-thin p-3 space-y-2">
        {SCOPE_ORDER.map((scope) => {
          const scopeDef = SCOPE_HIERARCHY[scope];
          const isCollapsed = isScopeCollapsed(scope);
          const nodeCount = getScopeNodeCount(scope);
          const colors = SCOPE_COLORS[scope];
          const subcategories = Object.entries(scopeDef.subcategories) as [
            Subcategory,
            (typeof scopeDef.subcategories)[Subcategory],
          ][];

          return (
            <div
              key={scope}
              className={cn(
                'rounded-xl border bg-white/[0.02] transition-all duration-200',
                colors.border,
                colors.borderHover,
                !isCollapsed && `shadow-lg ${colors.glow}`
              )}
            >
              {/* Scope Header */}
              <button
                onClick={() => toggleScopeCollapsed(scope)}
                className={cn(
                  'w-full flex items-center gap-3 px-3 py-2.5 rounded-xl',
                  'transition-colors duration-150',
                  'text-left focus:outline-none focus-visible:ring-2 focus-visible:ring-white/20',
                  !isCollapsed && `bg-gradient-to-r ${colors.headerBg}`
                )}
                aria-expanded={!isCollapsed}
                aria-controls={`scope-${scope}-content`}
              >
                {/* Scope Icon */}
                <span className="text-lg flex-shrink-0">{scopeDef.icon}</span>

                {/* Scope Name */}
                <span className="flex-1 text-[13px] font-medium text-white/90 uppercase tracking-wide">
                  {scopeDef.label}
                </span>

                {/* Node Count Badge */}
                <span
                  className={cn(
                    'px-2 py-0.5 rounded-md text-[11px] font-medium',
                    colors.badge
                  )}
                >
                  {nodeCount}
                </span>

                {/* Chevron */}
                <ChevronDown
                  className={cn(
                    'w-4 h-4 text-white/30 transition-transform duration-200',
                    isCollapsed && '-rotate-90'
                  )}
                  aria-hidden="true"
                />
              </button>

              {/* Subcategories - Collapsible with animation */}
              <div
                id={`scope-${scope}-content`}
                className={cn(
                  'overflow-hidden transition-all duration-200',
                  isCollapsed ? 'max-h-0 opacity-0' : 'max-h-96 opacity-100'
                )}
                role="group"
                aria-label={`${scopeDef.label} subcategories`}
              >
                <div className="px-2 pb-2 space-y-0.5">
                  {subcategories.map(([subcatName, subcatMeta]) => {
                    const isSubCollapsed = isSubcategoryCollapsed(scope, subcatName);
                    const count = subcatMeta.nodeTypes.length;

                    return (
                      <button
                        key={subcatName}
                        onClick={() => toggleSubcategoryCollapsed(scope, subcatName)}
                        className={cn(
                          'w-full flex items-center gap-2.5 px-3 py-2 rounded-lg',
                          'transition-all duration-150',
                          'text-left focus:outline-none focus-visible:ring-1 focus-visible:ring-white/20',
                          isSubCollapsed
                            ? 'opacity-40 hover:opacity-60'
                            : 'bg-white/[0.04] hover:bg-white/[0.08]'
                        )}
                        aria-pressed={!isSubCollapsed}
                        aria-label={`${subcatMeta.label}: ${count} node types. ${
                          isSubCollapsed ? 'Hidden' : 'Visible'
                        }`}
                      >
                        {/* Subcategory Icon */}
                        <span
                          className={cn(
                            'text-sm flex-shrink-0',
                            isSubCollapsed && 'grayscale'
                          )}
                          aria-hidden="true"
                        >
                          {subcatMeta.icon}
                        </span>

                        {/* Subcategory Name */}
                        <span
                          className={cn(
                            'flex-1 text-[13px]',
                            isSubCollapsed ? 'text-white/40' : 'text-white/80'
                          )}
                        >
                          {subcatMeta.label}
                        </span>

                        {/* Count */}
                        <span
                          className={cn(
                            'text-[11px] tabular-nums',
                            isSubCollapsed ? 'text-white/30' : 'text-white/50'
                          )}
                        >
                          ({count})
                        </span>
                      </button>
                    );
                  })}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Footer Stats - Minimal */}
      <div className="px-4 py-3 border-t border-white/[0.06]">
        <p className="text-[11px] text-white/30 text-center">
          3 scopes &middot; 9 categories &middot; 35 types
        </p>
      </div>
    </div>
  );
});
