'use client';

/**
 * SidebarTabs - Tabbed sidebar with Schema and Data panels
 *
 * Uses NovaNet Icon Design System for consistent icons.
 * Tab switching syncs with dataMode in uiStore.
 *
 * - Schema tab: Ontological schema view (SchemaFilterPanel)
 * - Data tab: Instance data explorer (DatabaseInfoPanel)
 *
 * Both panels share the same skeleton: container → header → body → footer
 */

import { memo } from 'react';
import { Boxes, Database } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { ICON_SIZES } from '@/config/iconSystem';
import { useUIStore } from '@/stores/uiStore';
import { DatabaseInfoPanel } from './DatabaseInfoPanel';
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
      {/* Tab Navigation - Refined minimal style */}
      <div className="flex bg-[#0d0d12] px-2 pt-1.5 pb-0">
        {TABS.map((tab) => (
          <button
            key={tab.id}
            onClick={() => handleTabClick(tab.id)}
            className={cn(
              'flex-1 flex items-center justify-center px-3 py-2.5 rounded-lg',
              gapTokens.compact,
              'text-xs font-medium transition-colors duration-200',
              activeTab === tab.id
                ? 'text-white/90 bg-white/[0.08]'
                : 'text-white/40 hover:text-white/60 hover:bg-white/[0.04]'
            )}
          >
            <span
              className={cn(
                'transition-colors duration-200',
                activeTab === tab.id ? 'text-white/70' : 'text-white/30'
              )}
            >
              {tab.icon}
            </span>
            <span>{tab.label}</span>
          </button>
        ))}
      </div>

      {/* Tab Content - Unified skeleton for Schema and Data */}
      <div className="flex-1 overflow-hidden">
        {activeTab === 'schema' && <SchemaFilterPanel />}
        {activeTab === 'data' && <DatabaseInfoPanel />}
      </div>
    </div>
  );
});
