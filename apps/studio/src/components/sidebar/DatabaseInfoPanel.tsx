'use client';

/**
 * DatabaseInfoPanel - Hierarchical database explorer with categories
 *
 * Features:
 * - Hierarchical tree view grouped by category
 * - Tri-state checkboxes (none/partial/all selected)
 * - Progress bars proportional to max count
 * - Collapsible categories (expanded by default)
 * - Multi-select with category-level toggle
 * - Premium visual design with smooth animations
 */

import { useState, useCallback, useEffect, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { GRAPH_ICONS, ACTION_ICONS } from '@/config/iconSystem';
import { cn } from '@/lib/utils';
import { NODE_CATEGORIES, ALL_NODE_TYPES } from '@/config/nodeTypes';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { useQueryStore, QueryBuilder } from '@/stores/queryStore';
import { useFilterStore } from '@/stores/filterStore';
import { useDatabaseSchema } from '@/hooks';
import { LoadingState } from '@/components/ui/EmptyState';
import { formatTime } from '@/lib/formatters';
import { NodeLabelsSection } from './database/NodeLabelsSection';
import { RelationshipsSection } from './database/RelationshipsSection';
import type { NodeType } from '@/types';

// Design system icons
const RefreshIcon = ACTION_ICONS.refresh;

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export const DatabaseInfoPanel = memo(function DatabaseInfoPanel() {
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

  // Multi-select state
  const [selectedLabels, setSelectedLabels] = useState<Set<string>>(new Set());
  const [selectedRelTypes, setSelectedRelTypes] = useState<Set<string>>(new Set());

  // Reset selection when schema changes (prevents stale references to removed labels)
  useEffect(() => {
    if (schema) {
      // Filter out any selected labels that no longer exist in schema
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
    // Note: setters are stable but included to satisfy exhaustive-deps
  }, [schema, setSelectedLabels, setSelectedRelTypes]);

  const { executeQuery, isExecuting } = useQueryStore(
    useShallow((state) => ({
      executeQuery: state.executeQuery,
      isExecuting: state.isExecuting,
    }))
  );
  const setEnabledNodeTypes = useFilterStore((state) => state.setEnabledNodeTypes);

  // Toggle single label
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

  // Toggle category labels
  const toggleCategoryLabels = useCallback((categoryId: string, types: string[]) => {
    setSelectedLabels((prev) => {
      const next = new Set(prev);
      // First remove all types from this category
      NODE_CATEGORIES.find((c) => c.id === categoryId)?.nodeTypes.forEach((t) =>
        next.delete(t)
      );
      // Then add the specified types
      types.forEach((t) => next.add(t));
      return next;
    });
  }, []);

  // Toggle all nodes
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

  // Toggle relationship type
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

  // Select/clear all relationships
  const toggleAllRelTypes = useCallback(() => {
    if (!schema) return;
    setSelectedRelTypes((prev) => {
      if (prev.size === schema.relationshipTypes.length) {
        return new Set();
      }
      return new Set(schema.relationshipTypes.map((r) => r.type));
    });
  }, [schema]);

  // Execute node query
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

  // Execute relationship query
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

    setEnabledNodeTypes(ALL_NODE_TYPES);
    executeQuery(query);
  }, [selectedRelTypes, executeQuery, setEnabledNodeTypes]);

  // formatTime imported from lib/formatters

  return (
    <div
      className="h-full flex flex-col bg-gradient-to-b from-black/60 to-black/40 backdrop-blur-xl"
      data-testid="database-info-panel"
    >
      {/* Header - Premium Glassmorphism */}
      <div className="relative px-4 py-5 border-b border-white/[0.08]">
        {/* Background glow effect */}
        <div className="absolute inset-0 bg-gradient-to-br from-novanet-500/5 via-transparent to-emerald-500/5 pointer-events-none" />

        <div className="relative flex items-center gap-3">
          {/* Icon with animated gradient */}
          <div className="relative">
            <div className="absolute inset-0 rounded-2xl bg-gradient-to-br from-novanet-400 to-emerald-500 opacity-20 blur-lg" />
            <div className="relative w-11 h-11 rounded-2xl bg-gradient-to-br from-novanet-500/20 to-emerald-500/20 flex items-center justify-center border border-white/10 shadow-lg shadow-black/20">
              <GRAPH_ICONS.database className="w-5 h-5 text-novanet-400" />
            </div>
          </div>

          <div className="flex-1">
            <h2 className="text-[15px] font-semibold text-white tracking-tight">
              Database Explorer
            </h2>
            <p className="text-[11px] text-white/40 mt-0.5">
              {schema ? `${schema.totalNodes.toLocaleString()} nodes · ${schema.totalRelationships.toLocaleString()} relationships` : 'Loading...'}
            </p>
          </div>

          {/* Refresh button */}
          <button
            onClick={fetchSchema}
            disabled={isLoading}
            className={cn(
              'p-2 rounded-xl transition-all duration-200',
              isLoading
                ? 'text-white/20 cursor-not-allowed'
                : 'text-white/40 hover:text-white/70 hover:bg-white/[0.06]'
            )}
            title="Refresh schema"
            aria-label="Refresh database schema"
          >
            <RefreshIcon className={cn('w-4 h-4', isLoading && 'animate-spin')} />
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        {isLoading && !schema ? (
          <LoadingState
            title="Loading schema"
            description="Fetching database structure"
            size="sm"
          />
        ) : error ? (
          <div className="p-4">
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
          <div className="p-3 space-y-4">
            {/* Node Labels Section */}
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

            {/* Divider */}
            <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent" />

            {/* Relationships Section */}
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
      </div>

      {/* Footer - Subtle status bar */}
      {lastUpdate && (
        <div className="px-4 py-2 border-t border-white/[0.04] bg-black/20">
          <span className="text-[10px] text-white/30">
            Updated {formatTime(lastUpdate)}
          </span>
        </div>
      )}
    </div>
  );
});
