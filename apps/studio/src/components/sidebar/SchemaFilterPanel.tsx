'use client';

/**
 * SchemaFilterPanel - Schema Browser with NodeCard display
 *
 * v11.6.1: Unified card-based schema browser using schemaStore.
 * Displays actual counts from Neo4j and supports filtering.
 */

import { memo, useMemo, useState, useEffect } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { Sidebar } from './SidebarContent';
import { SchemaCardView } from './SchemaCardView';
import { useSchemaStore, selectEnrichedNodeTypes, selectEnrichedRelTypes, selectIsSchemaLoaded } from '@/stores/schemaStore';

// Tab definitions
type SchemaTabId = 'types' | 'rels';

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const [activeTab, setActiveTab] = useState<SchemaTabId>('types');

  // Get schema data from store
  const { enrichedNodeTypes, enrichedRelTypes, isSchemaLoaded, loadSchema } = useSchemaStore(
    useShallow((state) => ({
      enrichedNodeTypes: selectEnrichedNodeTypes(state),
      enrichedRelTypes: selectEnrichedRelTypes(state),
      isSchemaLoaded: selectIsSchemaLoaded(state),
      loadSchema: state.loadSchema,
    }))
  );

  // Load schema on mount
  useEffect(() => {
    if (!isSchemaLoaded) {
      loadSchema();
    }
  }, [isSchemaLoaded, loadSchema]);

  // Tab definitions with actual counts
  const tabs = useMemo(() => [
    { id: 'types' as const, label: 'Types', count: enrichedNodeTypes.length },
    { id: 'rels' as const, label: 'Rels', count: enrichedRelTypes.length },
  ], [enrichedNodeTypes.length, enrichedRelTypes.length]);

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
