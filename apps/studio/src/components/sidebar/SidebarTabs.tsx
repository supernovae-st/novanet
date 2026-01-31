'use client';

/**
 * SidebarTabs - Tabbed sidebar with Schema and Data panels
 *
 * Uses NovaNet Icon Design System for consistent icons.
 * Tab switching syncs with dataMode in uiStore.
 *
 * - Schema tab: Ontological schema view (SchemaFilterPanel)
 * - Data tab: Instance data explorer (DatabaseInfoPanel + FilterPanel)
 */

import { memo } from 'react';
import { Boxes, Database } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { opacity, glassClasses, gapTokens } from '@/design/tokens';
import { ICON_SIZES } from '@/config/iconSystem';
import { useUIStore } from '@/stores/uiStore';
import { DatabaseInfoPanel } from './DatabaseInfoPanel';
import { FilterPanel } from './FilterPanel';
import { SchemaFilterPanel } from './SchemaFilterPanel';

type TabId = 'schema' | 'data';

interface Tab {
  id: TabId;
  label: string;
  icon: React.ReactNode;
}

const TABS: Tab[] = [
  { id: 'schema', label: 'Schema', icon: <Boxes className={ICON_SIZES.md} /> },
  { id: 'data', label: 'Data', icon: <Database className={ICON_SIZES.md} /> },
];

export const SidebarTabs = memo(function SidebarTabs() {
  const { dataMode, setDataMode } = useUIStore(
    useShallow((s) => ({
      dataMode: s.dataMode,
      setDataMode: s.setDataMode,
    }))
  );

  // Derive active tab from dataMode
  const activeTab: TabId = dataMode === 'schema' ? 'schema' : 'data';

  const handleTabClick = (tabId: TabId) => {
    setDataMode(tabId === 'schema' ? 'schema' : 'data');
  };

  return (
    <div className="h-full flex flex-col">
      {/* Tab Navigation - Linear style */}
      <div className="flex border-b border-white/[0.06] bg-[#0d0d12]">
        {TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => handleTabClick(tab.id)}
            className={cn(
              cn('flex-1 flex items-center justify-center px-4 py-3.5', gapTokens.default),
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
        {activeTab === 'schema' && <SchemaFilterPanel />}
        {activeTab === 'data' && (
          <div className="h-full flex flex-col overflow-hidden">
            <DatabaseInfoPanel />
            <div className="border-t border-white/[0.06]">
              <FilterPanel />
            </div>
          </div>
        )}
      </div>
    </div>
  );
});
