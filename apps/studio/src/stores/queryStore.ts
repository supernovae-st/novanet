/**
 * QueryStore - Manages current Cypher query and results
 *
 * Stores the active query, results, and view mode for the query panel.
 */

import { create } from 'zustand';
import type { GraphNode, GraphEdge } from '@/types';
import { useGraphStore } from './graphStore';
import { useSchemaStore } from './schemaStore';
import { useFilterStore } from './filterStore';
import { DEFAULT_QUERY_LIMIT, EXPAND_QUERY_LIMIT, MIN_EXECUTION_ANIMATION_MS } from '@/config/constants';
import { postJSON, getErrorMessage } from '@/lib/fetchClient';
import { extractLimit } from '@/lib/cypher/injectFilters';

export type ResultViewMode = 'graph' | 'table' | 'raw';

export interface QueryResult {
  nodes: GraphNode[];
  edges: GraphEdge[];
  totalNodes: number;
  totalArcs: number;
  duration: number;
  timestamp: Date;
}

export interface QueryState {
  // Current query
  currentQuery: string | null;
  isExecuting: boolean;
  error: string | null;

  // Results
  result: QueryResult | null;
  viewMode: ResultViewMode;

  // Actions
  setQuery: (query: string) => void;
  setExecuting: (executing: boolean) => void;
  setResult: (result: QueryResult | null) => void;
  setError: (error: string | null) => void;
  setViewMode: (mode: ResultViewMode) => void;
  clear: () => void;

  // Execute query - returns result for caller to capture (avoids race condition)
  executeQuery: (query: string, params?: Record<string, unknown>) => Promise<QueryResult | null>;
}

// AbortController for cancelling in-flight queries
let currentAbortController: AbortController | null = null;

export const useQueryStore = create<QueryState>((set) => ({
  currentQuery: null,
  isExecuting: false,
  error: null,
  result: null,
  viewMode: 'graph',

  setQuery: (query) => set({ currentQuery: query }),
  setExecuting: (executing) => set({ isExecuting: executing }),
  setResult: (result) => set({ result }),
  setError: (error) => set({ error }),
  setViewMode: (mode) => set({ viewMode: mode }),

  clear: () =>
    set({
      currentQuery: null,
      result: null,
      error: null,
      isExecuting: false,
    }),

  executeQuery: async (query: string, params?: Record<string, unknown>): Promise<QueryResult | null> => {
    // Cancel any in-flight query to prevent race conditions
    if (currentAbortController) {
      currentAbortController.abort();
    }

    // Capture in local variable to avoid race condition in finally block
    // (new request could start before finally runs, nulling the wrong controller)
    const abortController = new AbortController();
    currentAbortController = abortController;

    // QUERY-FIRST: Substitute params into query so what's displayed is exactly what runs
    // This makes the query copy-pasteable to Neo4j Browser
    let finalQuery = query;
    if (params) {
      for (const [key, value] of Object.entries(params)) {
        const placeholder = `$${key}`;
        // Handle string values with quotes, numbers/booleans without
        const replacement = typeof value === 'string' ? `"${value}"` : String(value);
        finalQuery = finalQuery.replace(new RegExp(`\\$${key}\\b`, 'g'), replacement);
      }
    }

    set({ currentQuery: finalQuery, isExecuting: true, error: null });

    // v12.1: Query-First - sync LIMIT from query to filterStore.displayLimit
    // This keeps the UI filter selector in sync with the actual query
    const queryLimit = extractLimit(finalQuery);
    if (queryLimit !== null) {
      useFilterStore.getState().setDisplayLimit(queryLimit);
    }

    try {
      interface QueryResponse {
        success: boolean;
        data?: { nodes: GraphNode[]; edges: GraphEdge[] };
        meta?: { totalNodes: number; totalArcs: number; queryDuration: number };
        error?: string;
      }

      // Run query + minimum animation delay in parallel
      // so the matrix effect is always visible
      // QUERY-FIRST: Send finalQuery (params already substituted) for consistency
      const [data] = await Promise.all([
        postJSON<QueryResponse>(
          '/api/graph/query',
          { cypher: finalQuery },
          { signal: abortController.signal }
        ),
        new Promise((r) => setTimeout(r, MIN_EXECUTION_ANIMATION_MS)),
      ]);

      if (!data.success) {
        throw new Error(data.error || 'Query failed');
      }

      const nodes = data.data?.nodes || [];
      const edges = data.data?.edges || [];

      // Create result object
      const result: QueryResult = {
        nodes,
        edges,
        totalNodes: data.meta?.totalNodes || nodes.length,
        totalArcs: data.meta?.totalArcs || edges.length,
        duration: data.meta?.queryDuration || 0,
        timestamp: new Date(),
      };

      // Update query store with results
      set({
        result,
        isExecuting: false,
      });

      // Also update graphStore so the graph visualization updates
      useGraphStore.getState().setGraphData({ nodes, edges });

      // Update schemaStore with result counts for filter chip states
      const nodeTypeCounts: Record<string, number> = {};
      const relTypeCounts: Record<string, number> = {};
      for (const node of nodes) {
        nodeTypeCounts[node.type] = (nodeTypeCounts[node.type] || 0) + 1;
      }
      for (const edge of edges) {
        relTypeCounts[edge.type] = (relTypeCounts[edge.type] || 0) + 1;
      }
      useSchemaStore.getState().updateCounts(nodeTypeCounts, relTypeCounts);

      // Return result for caller to capture (avoids race condition)
      return result;
    } catch (err) {
      // Ignore abort errors (expected when cancelling previous queries)
      if (err instanceof Error && err.name === 'AbortError') {
        return null;
      }

      set({
        error: getErrorMessage(err),
        isExecuting: false,
      });

      return null;
    } finally {
      // Only null if this is still the active controller
      // (avoids nulling a newer request's controller)
      if (currentAbortController === abortController) {
        currentAbortController = null;
      }
    }
  },
}));

/**
 * Helper to generate common queries
 * All queries include LIMIT by default to avoid performance issues
 * Queries are Neo4j Browser-style: returns only what was clicked
 */
export const QueryBuilder = {
  // Returns just nodes of a type (like Neo4j Browser)
  matchNodesByLabel: (label: string) =>
    `MATCH (n:${label}) RETURN n LIMIT ${DEFAULT_QUERY_LIMIT}`,

  // Returns relationships with their connected nodes
  matchRelationshipsByTypeWithNodes: (type: string) =>
    `MATCH (n)-[r:${type}]->(m) RETURN n, r, m LIMIT ${DEFAULT_QUERY_LIMIT}`,

  // Expand node: get this node with all its direct relationships (Neo4j Browser double-click)
  expandNode: (nodeId: string) =>
    `MATCH (n)-[r]-(m) WHERE elementId(n) = '${nodeId}' RETURN n, r, m LIMIT ${EXPAND_QUERY_LIMIT}`,
};

