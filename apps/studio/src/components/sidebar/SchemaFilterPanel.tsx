'use client';

/**
 * SchemaFilterPanel - Schema Browser with NodeCard display
 *
 * v11.0: Unified card-based schema browser similar to TUI meta view.
 * Uses NodeCard components for consistent styling across the app.
 */

import { memo, useMemo, useState } from 'react';
import { RelationType } from '@novanet/core/schemas';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { Sidebar } from './SidebarContent';
import { SchemaCardView } from './SchemaCardView';

// Tab definitions
type SchemaTabId = 'types' | 'rels';

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const [activeTab, setActiveTab] = useState<SchemaTabId>('types');

  // Relationship count
  const relCount = useMemo(() => {
    return Object.keys(RelationType).length;
  }, []);

  // Tab definitions
  const tabs = useMemo(() => [
    { id: 'types' as const, label: 'Types', count: 42 },
    { id: 'rels' as const, label: 'Rels', count: relCount },
  ], [relCount]);

  // Render Rels tab content (placeholder for now)
  const renderRelsContent = () => (
    <div className="flex items-center justify-center h-32 text-white/40 text-sm">
      Relationships browser coming soon
    </div>
  );

  return (
    <Sidebar.Content
      testId="schema-filter-panel"
      className={className}
      toolbar={
        <div className="flex flex-col gap-2 px-3 py-2.5">
          <SegmentedTabs
            tabs={tabs}
            activeTab={activeTab}
            onTabChange={(id) => setActiveTab(id as SchemaTabId)}
          />
        </div>
      }
    >
      {activeTab === 'types' ? (
        <SchemaCardView />
      ) : (
        renderRelsContent()
      )}
    </Sidebar.Content>
  );
});
