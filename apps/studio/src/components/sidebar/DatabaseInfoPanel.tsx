'use client';

/**
 * DatabaseInfoPanel - Data Explorer using unified Sidebar components
 *
 * Uses Sidebar.Content for consistent skeleton across all tabs:
 * - Same header structure as Schema Browser
 * - Same body padding (p-3)
 * - Same row heights and spacing
 *
 * Features:
 * - AI Search input in toolbar
 * - Segmented tabs: Views | Nodes | Rels
 * - Each tab uses Sidebar.Tree with Sidebar.Section/Row
 */

import { useState, useCallback, useEffect, memo, useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { RefreshCw } from 'lucide-react';
import { cn } from '@/lib/utils';
import { NODE_VISUAL_LAYERS, ALL_NODE_TYPES } from '@/config/nodeTypes';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { useQueryStore, QueryBuilder } from '@/stores/queryStore';
import { useFilterStore } from '@/stores/filterStore';
import { useViewStore } from '@/stores/viewStore';
import type { UseDatabaseSchemaReturn } from '@/hooks';
import { LoadingState } from '@/components/ui/EmptyState';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { formatTime } from '@/lib/formatters';
import { NodeLabelsSection } from './database/NodeLabelsSection';
import { RelationshipsSection } from './database/RelationshipsSection';
import { AiSearchInput } from './AiSearchInput';
import { ViewSelector } from './views';
import { Sidebar } from './SidebarContent';
import type { NodeType } from '@/types';

// Tab definitions
type TabId = 'views' | 'nodes' | 'rels';

// =============================================================================
// TOOLBAR COMPONENT
// =============================================================================

interface ToolbarProps {
  activeTab: TabId;
  onTabChange: (tab: TabId) => void;
  tabs: Array<{ id: TabId; label: string; count?: number }>;
  onRefresh: () => void;
  isLoading: boolean;
}

const Toolbar = memo(function Toolbar({
  activeTab,
  onTabChange,
  tabs,
  onRefresh,
  isLoading,
}: ToolbarProps) {
  return (
    <div className="flex flex-col gap-2 px-3 py-2.5">
      <div className="flex items-center gap-2">
        <div className="flex-1">
          <AiSearchInput placeholder="Ask AI to query the graph…" />
        </div>
        <button
          onClick={onRefresh}
          disabled={isLoading}
          className={cn(
            'p-2 rounded-lg flex-shrink-0',
            'bg-white/[0.03] border border-white/[0.06]',
            'text-white/30 hover:text-white/60 hover:bg-white/[0.06] hover:border-white/[0.10]',
            'transition-all duration-200',
            isLoading && 'text-white/20 cursor-not-allowed'
          )}
          title="Refresh schema"
          aria-label="Refresh database schema"
        >
          <RefreshCw className={cn('w-3.5 h-3.5', isLoading && 'animate-spin')} />
        </button>
      </div>
      <SegmentedTabs
        tabs={tabs}
        activeTab={activeTab}
        onTabChange={(id) => onTabChange(id as TabId)}
      />
    </div>
  );
});

// =============================================================================
// MAIN COMPONENT
// =============================================================================

interface DatabaseInfoPanelProps {
  /** Schema data lifted from SidebarTabs to avoid double-fetching */
  schemaData: UseDatabaseSchemaReturn;
}

export const DatabaseInfoPanel = memo(function DatabaseInfoPanel({
  schemaData,
}: DatabaseInfoPanelProps) {
  // Active tab state
  const [activeTab, setActiveTab] = useState<TabId>('views');

  // Database schema from props (lifted to SidebarTabs)
  const {
    schema,
    isLoading,
    error,
    lastUpdate,
    refresh: fetchSchema,
    labelCounts,
    maxNodeCount,
    maxRelCount,
  } = schemaData;

  // View store for count
  const viewCount = useViewStore(
    useShallow((state) => state.categories.flatMap((c) => c.views).length)
  );

  // Multi-select state
  const [selectedLabels, setSelectedLabels] = useState<Set<string>>(new Set());
  const [selectedRelTypes, setSelectedRelTypes] = useState<Set<string>>(new Set());

  // Reset selection when schema changes
  useEffect(() => {
    if (schema) {
      const validLabels = new Set(schema.nodeLabels.map((l) => l.label));
      const validRelTypes = new Set(schema.relationshipTypes.map((r) => r.type));

      setSelectedLabels((prev) => {
        const filtered = new Set([...prev].filter((l) => validLabels.has(l)));
        return filtered.size !== prev.size ? filtered : prev;
      });

      setSelectedRelTypes((prev) => {
        const filtered = new Set([...prev].filter((t) => validRelTypes.has(t)));
        return filtered.size !== prev.size ? filtered : prev;
      });
    }
  }, [schema]);

  const { executeQuery, isExecuting } = useQueryStore(
    useShallow((state) => ({
      executeQuery: state.executeQuery,
      isExecuting: state.isExecuting,
    }))
  );
  const setEnabledNodeTypes = useFilterStore((state) => state.setEnabledNodeTypes);

  // Toggle handlers
  const toggleLabel = useCallback((label: string) => {
    setSelectedLabels((prev) => {
      const next = new Set(prev);
      if (next.has(label)) {
        next.delete(label);
      } else {
        next.add(label);
      }
      return next;
    });
  }, []);

  const toggleCategoryLabels = useCallback((categoryId: string, types: string[]) => {
    setSelectedLabels((prev) => {
      const next = new Set(prev);
      NODE_VISUAL_LAYERS.find((c) => c.id === categoryId)?.nodeTypes.forEach((t) =>
        next.delete(t)
      );
      types.forEach((t) => next.add(t));
      return next;
    });
  }, []);

  const toggleAllNodes = useCallback(() => {
    if (!schema) return;
    setSelectedLabels((prev) => {
      const allLabels = schema.nodeLabels.map((l) => l.label);
      if (prev.size === allLabels.length) {
        return new Set();
      }
      return new Set(allLabels);
    });
  }, [schema]);

  const toggleRelType = useCallback((type: string) => {
    setSelectedRelTypes((prev) => {
      const next = new Set(prev);
      if (next.has(type)) {
        next.delete(type);
      } else {
        next.add(type);
      }
      return next;
    });
  }, []);

  const toggleAllRelTypes = useCallback(() => {
    if (!schema) return;
    setSelectedRelTypes((prev) => {
      if (prev.size === schema.relationshipTypes.length) {
        return new Set();
      }
      return new Set(schema.relationshipTypes.map((r) => r.type));
    });
  }, [schema]);

  // Execute queries
  const executeNodeQuery = useCallback(() => {
    if (selectedLabels.size === 0) return;

    const labels = Array.from(selectedLabels);
    let query: string;

    if (labels.length === 1) {
      query = QueryBuilder.matchNodesByLabel(labels[0]);
    } else {
      const conditions = labels.map((l) => `n:${l}`).join(' OR ');
      query = `MATCH (n) WHERE ${conditions} RETURN n LIMIT ${DEFAULT_FETCH_LIMIT}`;
    }

    setEnabledNodeTypes(labels as NodeType[]);
    executeQuery(query);
  }, [selectedLabels, executeQuery, setEnabledNodeTypes]);

  const executeRelQuery = useCallback(() => {
    if (selectedRelTypes.size === 0) return;

    const types = Array.from(selectedRelTypes);
    let query: string;

    if (types.length === 1) {
      query = QueryBuilder.matchRelationshipsByTypeWithNodes(types[0]);
    } else {
      const typeList = types.map((t) => `"${t}"`).join(', ');
      query = `MATCH (n)-[r]->(m) WHERE type(r) IN [${typeList}] RETURN n, r, m LIMIT ${DEFAULT_FETCH_LIMIT}`;
    }

    setEnabledNodeTypes([...ALL_NODE_TYPES]);
    executeQuery(query);
  }, [selectedRelTypes, executeQuery, setEnabledNodeTypes]);

  // Build tabs with counts
  const tabs = useMemo(
    () => [
      { id: 'views' as const, label: 'Views', count: viewCount || undefined },
      { id: 'nodes' as const, label: 'Nodes', count: schema?.totalNodes },
      { id: 'rels' as const, label: 'Rels', count: schema?.totalRelationships },
    ],
    [viewCount, schema?.totalNodes, schema?.totalRelationships]
  );

  // Render tab content
  const renderContent = () => {
    if (activeTab === 'views') {
      return <ViewSelector />;
    }

    if (activeTab === 'nodes') {
      if (isLoading && !schema) {
        return (
          <LoadingState
            title="Loading schema"
            description="Fetching database structure..."
            size="sm"
          />
        );
      }
      if (error) {
        return (
          <div className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 text-center">
            <p className="text-sm text-red-300">{error}</p>
            <button
              onClick={fetchSchema}
              className="mt-3 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 rounded-lg text-xs text-red-300 transition-colors"
            >
              Retry
            </button>
          </div>
        );
      }
      if (schema) {
        return (
          <NodeLabelsSection
            totalNodes={schema.totalNodes}
            labelCounts={labelCounts}
            maxCount={maxNodeCount}
            selectedLabels={selectedLabels}
            onToggleLabel={toggleLabel}
            onToggleCategoryLabels={toggleCategoryLabels}
            onToggleAllNodes={toggleAllNodes}
            onExecuteQuery={executeNodeQuery}
            isExecuting={isExecuting}
          />
        );
      }
      return null;
    }

    if (activeTab === 'rels') {
      if (isLoading && !schema) {
        return (
          <LoadingState
            title="Loading schema"
            description="Fetching database structure..."
            size="sm"
          />
        );
      }
      if (error) {
        return (
          <div className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 text-center">
            <p className="text-sm text-red-300">{error}</p>
            <button
              onClick={fetchSchema}
              className="mt-3 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 rounded-lg text-xs text-red-300 transition-colors"
            >
              Retry
            </button>
          </div>
        );
      }
      if (schema) {
        return (
          <RelationshipsSection
            totalRelationships={schema.totalRelationships}
            relationshipTypes={schema.relationshipTypes}
            maxCount={maxRelCount}
            selectedRelTypes={selectedRelTypes}
            onToggleRelType={toggleRelType}
            onToggleAllRelTypes={toggleAllRelTypes}
            onExecuteQuery={executeRelQuery}
            isExecuting={isExecuting}
          />
        );
      }
      return null;
    }

    return null;
  };

  return (
    <Sidebar.Content
      testId="database-info-panel"
      toolbar={
        <Toolbar
          activeTab={activeTab}
          onTabChange={setActiveTab}
          tabs={tabs}
          onRefresh={fetchSchema}
          isLoading={isLoading}
        />
      }
      footer={
        lastUpdate ? (
          <span className="text-[11px] text-white/40 text-center">
            Updated {formatTime(lastUpdate)}
          </span>
        ) : undefined
      }
    >
      {renderContent()}
    </Sidebar.Content>
  );
});
