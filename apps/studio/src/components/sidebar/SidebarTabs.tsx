'use client';

/**
 * SidebarTabs - Rich tabbed sidebar with Schema and Data panels
 *
 * Tab bar handles identity (no header card needed in panels):
 * - Schema tab: static stats (35 types · 3 scopes)
 * - Data tab: dynamic stats from DB + live dot
 *
 * Design: underline indicator on active tab, no border-radius,
 * page-switching feel with accent-colored bar.
 *
 * Lifts useDatabaseSchema() here to share data with DatabaseInfoPanel
 * and show stats in the tab bar without double-fetching.
 */

import { memo, useCallback } from 'react';
import { Boxes, Database } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { useUIStore } from '@/stores/uiStore';
import { useAnimationStore } from '@/stores/animationStore';
import { useDatabaseSchema } from '@/hooks';
import { DatabaseInfoPanel } from './DatabaseInfoPanel';
import { SchemaFilterPanel } from './SchemaFilterPanel';

type TabId = 'schema' | 'data';

/** Accent colors per tab for the underline bar + icon */
const TAB_ACCENTS: Record<TabId, string> = {
  schema: '#a78bfa', // violet-400
  data: '#22d3ee',   // cyan-400
};

export const SidebarTabs = memo(function SidebarTabs() {
  const dataMode = useUIStore((s) => s.dataMode);
  const { isTransitioning, startTransition } = useAnimationStore(
    useShallow((s) => ({
      isTransitioning: s.isTransitioning,
      startTransition: s.startTransition,
    }))
  );

  // Lift schema hook to share between tab bar stats and DatabaseInfoPanel
  const schemaData = useDatabaseSchema();
  const { schema, isLoading: schemaLoading } = schemaData;

  const activeTab: TabId = dataMode === 'schema' ? 'schema' : 'data';

  const handleTabClick = useCallback(
    (tabId: TabId) => {
      const targetMode = tabId === 'schema' ? 'schema' : 'data';
      // Skip if already in target mode or transition in progress
      if (dataMode === targetMode || isTransitioning) return;
      // Start the Matrix transition - page.tsx orchestrates the mode change
      startTransition(targetMode);
    },
    [dataMode, isTransitioning, startTransition]
  );

  return (
    <div className="h-full flex flex-col">
      {/* Tab Navigation - underline style, page-switching feel */}
      <div className="relative flex bg-[#0d0d12]" role="tablist" aria-label="Sidebar mode">
        {/* Schema Tab */}
        <button
          role="tab"
          aria-selected={activeTab === 'schema'}
          aria-controls="sidebar-panel-schema"
          onClick={() => handleTabClick('schema')}
          className={cn(
            'relative flex-1 flex items-center gap-2.5 px-4 py-3',
            'transition-all duration-200',
            activeTab === 'schema'
              ? 'text-white/90'
              : 'text-white/35 hover:text-white/55'
          )}
        >
          <Boxes
            className={cn(
              iconSizes.md,
              'transition-colors duration-200 flex-shrink-0',
              activeTab === 'schema' ? 'text-violet-400' : 'text-white/25'
            )}
          />
          <div className="min-w-0 text-left">
            <div className="text-xs font-medium leading-tight">Schema</div>
            <div
              className={cn(
                'text-[10px] leading-tight mt-0.5 tabular-nums',
                activeTab === 'schema' ? 'text-white/40' : 'text-white/20'
              )}
            >
              35 types · 3 scopes
            </div>
          </div>

          {/* Active indicator bar */}
          <div
            className={cn(
              'absolute bottom-0 left-3 right-3 h-[2px] rounded-full',
              'transition-all duration-300 ease-out',
              activeTab === 'schema' ? 'opacity-100' : 'opacity-0 scale-x-0'
            )}
            style={{ backgroundColor: TAB_ACCENTS.schema }}
          />
        </button>

        {/* Data Tab */}
        <button
          role="tab"
          aria-selected={activeTab === 'data'}
          aria-controls="sidebar-panel-data"
          onClick={() => handleTabClick('data')}
          className={cn(
            'relative flex-1 flex items-center gap-2.5 px-4 py-3',
            'transition-all duration-200',
            activeTab === 'data'
              ? 'text-white/90'
              : 'text-white/35 hover:text-white/55'
          )}
        >
          <Database
            className={cn(
              iconSizes.md,
              'transition-colors duration-200 flex-shrink-0',
              activeTab === 'data' ? 'text-cyan-400' : 'text-white/25'
            )}
          />
          <div className="flex-1 min-w-0 text-left">
            <div className="flex items-center gap-1.5 leading-tight">
              <span className="text-xs font-medium">Data</span>
              {/* Live status dot */}
              {!schemaLoading && schema && (
                <span className="relative flex h-1.5 w-1.5">
                  <span className="animate-ping motion-reduce:animate-none absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75" />
                  <span className="relative inline-flex rounded-full h-1.5 w-1.5 bg-emerald-500" />
                </span>
              )}
              {schemaLoading && (
                <span className="relative flex h-1.5 w-1.5">
                  <span className="animate-ping motion-reduce:animate-none absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75" />
                  <span className="relative inline-flex rounded-full h-1.5 w-1.5 bg-amber-500" />
                </span>
              )}
            </div>
            <div
              className={cn(
                'text-[10px] leading-tight mt-0.5 tabular-nums',
                activeTab === 'data' ? 'text-white/40' : 'text-white/20'
              )}
            >
              {schema
                ? `${schema.totalNodes.toLocaleString()} nodes · ${(schema.totalRelationships ?? 0).toLocaleString()} rels`
                : 'Loading…'}
            </div>
          </div>

          {/* Active indicator bar */}
          <div
            className={cn(
              'absolute bottom-0 left-3 right-3 h-[2px] rounded-full',
              'transition-all duration-300 ease-out',
              activeTab === 'data' ? 'opacity-100' : 'opacity-0 scale-x-0'
            )}
            style={{ backgroundColor: TAB_ACCENTS.data }}
          />
        </button>

        {/* Bottom border line (behind the active bars) */}
        <div className="absolute bottom-0 left-0 right-0 h-px bg-white/[0.06]" />
      </div>

      {/* Tab Content */}
      <div className="flex-1 overflow-hidden" id={`sidebar-panel-${activeTab}`} role="tabpanel">
        {activeTab === 'schema' && <SchemaFilterPanel />}
        {activeTab === 'data' && <DatabaseInfoPanel schemaData={schemaData} />}
      </div>
    </div>
  );
});
