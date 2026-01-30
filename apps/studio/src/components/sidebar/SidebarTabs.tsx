'use client';

/**
 * SidebarTabs - Tabbed sidebar with Database and Filters panels
 *
 * Uses NovaNet Icon Design System for consistent icons.
 * Shows SchemaFilterPanel when in schema mode (v8.3.0).
 */

import { useState, useEffect, memo } from 'react';
import { SlidersHorizontal } from 'lucide-react';
import { cn } from '@/lib/utils';
import { GRAPH_ICONS, ICON_SIZES } from '@/config/iconSystem';
import { useUIStore, selectDataMode } from '@/stores/uiStore';
import { DatabaseInfoPanel } from './DatabaseInfoPanel';
import { FilterPanel } from './FilterPanel';
import { SchemaFilterPanel } from './SchemaFilterPanel';

type TabId = 'database' | 'filters';

interface Tab {
  id: TabId;
  label: string;
  icon: React.ReactNode;
}

// Use design system icons
const DatabaseIcon = GRAPH_ICONS.database;

const TABS: Tab[] = [
  { id: 'database', label: 'Database', icon: <DatabaseIcon className={ICON_SIZES.md} /> },
  { id: 'filters', label: 'Views', icon: <SlidersHorizontal className={ICON_SIZES.md} /> },
];

export const SidebarTabs = memo(function SidebarTabs() {
  const [activeTab, setActiveTab] = useState<TabId>('database');
  const dataMode = useUIStore(selectDataMode);

  // Auto-switch to filters tab when entering schema mode
  useEffect(() => {
    if (dataMode === 'schema') {
      setActiveTab('filters');
    }
  }, [dataMode]);

  return (
    <div className="h-full flex flex-col">
      {/* Tab Navigation - Linear style */}
      <div className="flex border-b border-white/8 bg-[#0d0d12]">
        {TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={cn(
              'flex-1 flex items-center justify-center gap-2 px-4 py-3.5',
              'text-xs font-medium transition-all duration-200',
              'border-b-2 -mb-px',
              activeTab === tab.id
                ? 'text-white border-accent-blue bg-accent-blue/8'
                : 'text-white/50 border-transparent hover:text-white/70 hover:bg-white/[0.04]'
            )}
          >
            <span
              className={cn(
                'transition-colors',
                activeTab === tab.id ? 'text-accent-blue' : 'text-white/40'
              )}
            >
              {tab.icon}
            </span>
            <span>{tab.label}</span>
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div className="flex-1 overflow-hidden">
        {activeTab === 'database' && <DatabaseInfoPanel />}
        {activeTab === 'filters' && (
          dataMode === 'schema' ? <SchemaFilterPanel /> : <FilterPanel />
        )}
      </div>
    </div>
  );
});
