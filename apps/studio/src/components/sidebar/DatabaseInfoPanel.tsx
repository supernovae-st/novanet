'use client';

/**
 * DatabaseInfoPanel - Data Explorer with tabbed interface
 *
 * Design: A+B Hybrid (Segmented Tabs + Minimal Content)
 *
 * Features:
 * - AI Search input always visible at top
 * - Segmented tabs: Views | Nodes | Rels (with counts)
 * - Views tab: YAML view presets (primary entry point)
 * - Nodes tab: Hierarchical node type browser
 * - Rels tab: Relationship type browser
 * - Premium glassmorphism design
 */

import { useState, useCallback, useEffect, memo, useMemo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { GRAPH_ICONS, ACTION_ICONS } from '@/config/iconSystem';
import { cn } from '@/lib/utils';
import { NODE_VISUAL_CATEGORIES, ALL_NODE_TYPES } from '@/config/nodeTypes';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { useQueryStore, QueryBuilder } from '@/stores/queryStore';
import { useFilterStore } from '@/stores/filterStore';
import { useAiQueryStore } from '@/stores/aiQueryStore';
import { useViewStore } from '@/stores/viewStore';
import { useDatabaseSchema } from '@/hooks';
import { LoadingState } from '@/components/ui/EmptyState';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { formatTime } from '@/lib/formatters';
import { panelClasses, glassClasses, iconSizes, iconButtonClasses, gapTokens } from '@/design/tokens';
import { NodeLabelsSection } from './database/NodeLabelsSection';
import { RelationshipsSection } from './database/RelationshipsSection';
import { AiSearchInput } from './AiSearchInput';
import { ViewSelector } from './views';
import type { NodeType } from '@/types';

// Design system icons
const RefreshIcon = ACTION_ICONS.refresh;

// Tab definitions
type TabId = 'views' | 'nodes' | 'rels';

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export const DatabaseInfoPanel = memo(function DatabaseInfoPanel() {
  // Active tab state
  const [activeTab, setActiveTab] = useState<TabId>('views');

  // Database schema from hook
  const {
    schema,
    isLoading,
    error,
    lastUpdate,
    refresh: fetchSchema,
    labelCounts,
    maxNodeCount,
    maxRelCount,
  } = useDatabaseSchema();

  // View store for count
  const viewCount = useViewStore(
    useShallow((state) => state.categories.flatMap((c) => c.views).length)
  );

  // AI query handling
  const { submitAiQuery, isProcessing: isAiProcessing } = useAiQueryStore(
    useShallow((state) => ({
      submitAiQuery: state.submitAiQuery,
      isProcessing: state.isProcessing,
    }))
  );

  const handleAiSubmit = useCallback(
    async (question: string) => {
      await submitAiQuery(question);
    },
    [submitAiQuery]
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
      NODE_VISUAL_CATEGORIES.find((c) => c.id === categoryId)?.nodeTypes.forEach((t) =>
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

  // Build tabs with counts (no icons - cleaner design)
  const tabs = useMemo(
    () => [
      {
        id: 'views' as const,
        label: 'Views',
        count: viewCount || undefined,
      },
      {
        id: 'nodes' as const,
        label: 'Nodes',
        count: schema?.totalNodes,
      },
      {
        id: 'rels' as const,
        label: 'Rels',
        count: schema?.totalRelationships,
      },
    ],
    [viewCount, schema?.totalNodes, schema?.totalRelationships]
  );

  return (
    <div
      className={cn(
        'h-full',
        panelClasses.container
      )}
      data-testid="database-info-panel"
    >
      {/* Header - Premium Glassmorphism */}
      <div className={cn('relative', panelClasses.header)}>
        <div className="absolute inset-0 bg-gradient-to-br from-novanet-500/5 via-transparent to-emerald-500/5 pointer-events-none" />

        <div className={cn('relative flex items-center', gapTokens.spacious)}>
          <div className="relative">
            <div className="absolute inset-0 rounded-2xl bg-gradient-to-br from-novanet-400 to-emerald-500 opacity-20 blur-lg" />
            <div className="relative w-10 h-10 rounded-xl bg-gradient-to-br from-novanet-500/20 to-emerald-500/20 flex items-center justify-center border border-white/10 shadow-lg shadow-black/20">
              <GRAPH_ICONS.database className={cn(iconSizes.md, 'text-novanet-400')} />
            </div>
          </div>

          <div className="flex-1 min-w-0">
            <h2 className="text-sm font-semibold text-white tracking-tight">
              Data Explorer
            </h2>
            <p className="text-[10px] text-white/40 mt-0.5 truncate">
              {schema?.totalNodes !== undefined
                ? `${schema.totalNodes.toLocaleString()} nodes · ${schema.totalRelationships?.toLocaleString() ?? 0} rels`
                : 'Loading...'}
            </p>
          </div>

          <button
            onClick={fetchSchema}
            disabled={isLoading}
            className={cn(
              iconButtonClasses.ghost,
              isLoading && 'text-white/40 cursor-not-allowed'
            )}
            title="Refresh schema"
            aria-label="Refresh database schema"
          >
            <RefreshIcon className={cn(iconSizes.sm, isLoading && 'animate-spin')} />
          </button>
        </div>
      </div>

      {/* AI Search - Always visible */}
      <div className="px-4 pt-4">
        <AiSearchInput
          onSubmit={handleAiSubmit}
          isLoading={isAiProcessing}
          placeholder="Ask AI to query the graph…"
        />
      </div>

      {/* Segmented Tabs */}
      <div className="px-4 py-4">
        <SegmentedTabs
          tabs={tabs}
          activeTab={activeTab}
          onTabChange={(id) => setActiveTab(id as TabId)}
        />
      </div>

      {/* Tab Content */}
      <div className={panelClasses.body}>
        {activeTab === 'views' && (
          <div className="px-4 pb-6">
            <ViewSelector />
          </div>
        )}

        {activeTab === 'nodes' && (
          <>
            {isLoading && !schema ? (
              <LoadingState
                title="Loading schema"
                description="Fetching database structure…"
                size="sm"
              />
            ) : error ? (
              <div className="px-4 pb-4">
                <div className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 text-center">
                  <p className="text-sm text-red-300">{error}</p>
                  <button
                    onClick={fetchSchema}
                    className="mt-3 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 rounded-lg text-xs text-red-300 transition-colors"
                  >
                    Retry
                  </button>
                </div>
              </div>
            ) : schema ? (
              <div className="px-4 pb-6">
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
              </div>
            ) : null}
          </>
        )}

        {activeTab === 'rels' && (
          <>
            {isLoading && !schema ? (
              <LoadingState
                title="Loading schema"
                description="Fetching database structure…"
                size="sm"
              />
            ) : error ? (
              <div className="px-4 pb-4">
                <div className="bg-red-500/10 border border-red-500/20 rounded-xl p-4 text-center">
                  <p className="text-sm text-red-300">{error}</p>
                  <button
                    onClick={fetchSchema}
                    className="mt-3 px-4 py-2 bg-red-500/20 hover:bg-red-500/30 rounded-lg text-xs text-red-300 transition-colors"
                  >
                    Retry
                  </button>
                </div>
              </div>
            ) : schema ? (
              <div className="px-4 pb-6">
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
              </div>
            ) : null}
          </>
        )}
      </div>

      {/* Footer - Subtle status bar */}
      {lastUpdate && (
        <div className={cn(panelClasses.footer, 'bg-black/20')}>
          <span className={panelClasses.footerText}>
            Updated {formatTime(lastUpdate)}
          </span>
        </div>
      )}
    </div>
  );
});
