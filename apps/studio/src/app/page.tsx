'use client';

/**
 * NovaNet Visualizer - Main Page (Optimized)
 *
 * Beautiful knowledge graph visualization with:
 * - Turbo-style glowing nodes and gradient edges
 * - Intelligent dagre layout
 * - Filter presets with keyboard shortcuts
 * - Node details panel
 * - AI chat integration
 *
 * Performance optimizations (v7.2.6):
 * - useShallow selectors prevent unnecessary re-renders
 * - Memoized handlers prevent cascading updates
 * - Stable action references via useShallow
 */

import { useCallback, useEffect, useMemo, useRef, lazy, Suspense } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { Box, PanelLeft, Keyboard, X, Network, Table2, Code, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { useUIStore, useFilterStore, useGraphStore } from '@/stores';
import { useGraphData, useFilteredGraph, UrlSyncComponent } from '@/hooks';
import { ContextPicker } from '@/components/sidebar/ContextPicker';
import { isInputFocused, matchesKeyCombo } from '@/lib/keyboard';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { resolveThemeCached } from '@/components/graph/edges';

// Lazy load Graph2D - saves ~400KB from initial bundle
const Graph2D = lazy(() =>
  import('@/components/graph').then((mod) => ({ default: mod.Graph2D }))
);
import { GraphErrorBoundary } from '@/components/ui/ErrorBoundary';
import { StatsCounter, Pill, Divider, RefreshButton, CategoryIcon, DataModeToggle } from '@/components/ui';
import { SidebarTabs } from '@/components/sidebar/SidebarTabs';
import { NodeDetailsPanel } from '@/components/sidebar/NodeDetailsPanel';
import { EdgeDetailsPanel } from '@/components/sidebar/EdgeDetailsPanel';
import { KeyboardShortcuts, useKeyboardShortcuts } from '@/components/ui/KeyboardShortcuts';
import { CommandPalette, useCommandPalette, useCommandPaletteState } from '@/components/ui/CommandPalette';
import { QueryPill, ResultsOverview, TableView, RawView } from '@/components/query';
import { useQueryStore, QueryBuilder } from '@/stores/queryStore';

export default function HomePage() {
  // ═══════════════════════════════════════════════════════════════════════════
  // STORE SELECTORS (Optimized with useShallow)
  // ═══════════════════════════════════════════════════════════════════════════

  // UI Store - state values
  const uiState = useUIStore(
    useShallow((state) => ({
      viewMode: state.viewMode,
      sidebarOpen: state.sidebarOpen,
      focusMode: state.focusMode,
      minimapVisible: state.minimapVisible,
      selectedNodeId: state.selectedNodeId,
      selectedEdgeId: state.selectedEdgeId,
      hoveredNodeId: state.hoveredNodeId,
      hoveredEdgeId: state.hoveredEdgeId,
    }))
  );

  // UI Store - actions (stable references)
  const uiActions = useUIStore(
    useShallow((state) => ({
      toggleViewMode: state.toggleViewMode,
      toggleSidebar: state.toggleSidebar,
      toggleFocusMode: state.toggleFocusMode,
      toggleMinimap: state.toggleMinimap,
      toggleEdgeLabels: state.toggleEdgeLabels,
      setLayoutDirection: state.setLayoutDirection,
      setSelectedNode: state.setSelectedNode,
      setSelectedEdge: state.setSelectedEdge,
      clearSelection: state.clearSelection,
    }))
  );

  // Filter Store - state + actions
  const filterState = useFilterStore(
    useShallow((state) => ({
      enabledNodeTypes: state.enabledNodeTypes,
      activePresetId: state.activePresetId,
    }))
  );
  const filterActions = useFilterStore(
    useShallow((state) => ({
      applyViewPresetByShortcut: state.applyViewPresetByShortcut,
      toCypher: state.toCypher,
    }))
  );

  // Graph Store - state + selectors
  const totalNodes = useGraphStore((state) => state.totalNodes);
  const getNodeById = useGraphStore((state) => state.getNodeById);
  const getEdgeById = useGraphStore((state) => state.getEdgeById);

  // Query Store - state + actions
  const queryState = useQueryStore(
    useShallow((state) => ({
      viewMode: state.viewMode,
      isExecuting: state.isExecuting,
      currentQuery: state.currentQuery,
    }))
  );
  const queryActions = useQueryStore(
    useShallow((state) => ({
      setViewMode: state.setViewMode,
      setQuery: state.setQuery,
    }))
  );

  // ═══════════════════════════════════════════════════════════════════════════
  // DERIVED STATE & HOOKS
  // ═══════════════════════════════════════════════════════════════════════════

  // Filtered graph stats
  const { visibleNodeCount, visibleEdgeCount } = useFilteredGraph();

  // Graph data fetching
  const { fetchData, fetchSchemaData, executeQuery, dataMode, isLoading: isFetching } = useGraphData();

  // Keyboard shortcuts modal
  const { isOpen: shortcutsOpen, open: openShortcuts, close: closeShortcuts } = useKeyboardShortcuts();

  // Command palette (⌘K)
  const { isOpen: paletteOpen, open: openPalette, close: closePalette } = useCommandPaletteState();

  // Get selected node/edge data (memoized)
  const selectedNode = useMemo(
    () => (uiState.selectedNodeId ? getNodeById(uiState.selectedNodeId) : null),
    [uiState.selectedNodeId, getNodeById]
  );
  const selectedEdge = useMemo(
    () => (uiState.selectedEdgeId ? getEdgeById(uiState.selectedEdgeId) : null),
    [uiState.selectedEdgeId, getEdgeById]
  );

  // Get hovered node/edge data (memoized) for centralized hover info
  const hoveredNode = useMemo(
    () => (uiState.hoveredNodeId ? getNodeById(uiState.hoveredNodeId) : null),
    [uiState.hoveredNodeId, getNodeById]
  );
  const hoveredEdge = useMemo(
    () => (uiState.hoveredEdgeId ? getEdgeById(uiState.hoveredEdgeId) : null),
    [uiState.hoveredEdgeId, getEdgeById]
  );

  // Get source/target nodes for hovered edge (for rich tooltip display)
  const hoveredEdgeNodes = useMemo(() => {
    if (!hoveredEdge) return null;
    const sourceNode = getNodeById(hoveredEdge.source);
    const targetNode = getNodeById(hoveredEdge.target);
    if (!sourceNode || !targetNode) return null;
    const theme = resolveThemeCached(hoveredEdge.type);
    return { sourceNode, targetNode, colors: theme.colors };
  }, [hoveredEdge, getNodeById]);

  // Note: URL sync is rendered as a component (see render section) to handle
  // Next.js Suspense requirements for useSearchParams

  // ═══════════════════════════════════════════════════════════════════════════
  // EFFECTS
  // ═══════════════════════════════════════════════════════════════════════════

  // Sync filter state to query bar - shows the Cypher equivalent of current filters
  useEffect(() => {
    const cypher = filterActions.toCypher();
    if (cypher) {
      queryActions.setQuery(cypher);
    }
  }, [filterState.enabledNodeTypes, filterState.activePresetId, filterActions, queryActions]);

  // Track previous dataMode to detect changes
  const prevDataModeRef = useRef<typeof dataMode | null>(null);

  // Fetch initial data OR re-fetch when dataMode changes (e.g., from URL sync)
  useEffect(() => {
    if (isFetching) return;

    // Check if this is a mode change (vs initial load)
    const isInitialLoad = totalNodes === 0;
    const modeChanged = prevDataModeRef.current !== null && prevDataModeRef.current !== dataMode;

    // Fetch on initial load OR mode change
    if (isInitialLoad || modeChanged) {
      if (dataMode === 'schema') {
        fetchSchemaData();
      } else {
        fetchData({ limit: DEFAULT_FETCH_LIMIT });
      }
    }

    prevDataModeRef.current = dataMode;
  }, [totalNodes, isFetching, fetchData, fetchSchemaData, dataMode]);

  // Global keyboard handler
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (isInputFocused()) return;

      // Command palette
      if (matchesKeyCombo(e, 'mod+k')) {
        e.preventDefault();
        openPalette();
        return;
      }

      // View toggle
      if (e.key === 'v' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        uiActions.toggleViewMode();
        return;
      }

      // Focus mode
      if (e.key === 'g' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        uiActions.toggleFocusMode();
        return;
      }

      // Minimap toggle
      if (e.key === 'm' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        uiActions.toggleMinimap();
        return;
      }

      // Edge labels toggle
      if (e.key === 'l' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        uiActions.toggleEdgeLabels();
        return;
      }

      // Layout shortcuts (Shift + H/V/D/R/F)
      if (e.shiftKey && !e.metaKey && !e.ctrlKey) {
        switch (e.key) {
          case 'H':
            e.preventDefault();
            uiActions.setLayoutDirection('LR');
            return;
          case 'V':
            e.preventDefault();
            uiActions.setLayoutDirection('TB');
            return;
          case 'D':
            e.preventDefault();
            uiActions.setLayoutDirection('dagre');
            return;
          case 'R':
            e.preventDefault();
            uiActions.setLayoutDirection('radial');
            return;
          case 'F':
            e.preventDefault();
            uiActions.setLayoutDirection('force');
            return;
        }
      }

      // Sidebar toggle
      if (e.key === '[') {
        e.preventDefault();
        uiActions.toggleSidebar();
        return;
      }

      // Close panel (deselect) with ]
      if (e.key === ']') {
        e.preventDefault();
        uiActions.setSelectedNode(null);
        uiActions.setSelectedEdge(null);
        return;
      }

      // Escape - close dialogs, clear selection
      if (e.key === 'Escape') {
        if (shortcutsOpen) {
          closeShortcuts();
          return;
        }
        uiActions.setSelectedNode(null);
        return;
      }

      // View preset shortcuts (1-8, 0) - maps to VIEW_PRESETS v7.2.1
      if (/^[0-8]$/.test(e.key) && !e.metaKey && !e.ctrlKey && !e.altKey) {
        e.preventDefault();
        filterActions.applyViewPresetByShortcut(e.key);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [uiActions, filterActions, shortcutsOpen, closeShortcuts, openPalette]);

  // ═══════════════════════════════════════════════════════════════════════════
  // MEMOIZED HANDLERS
  // ═══════════════════════════════════════════════════════════════════════════

  // Refresh data
  const handleRefresh = useCallback(() => {
    if (dataMode === 'schema') {
      fetchSchemaData();
    } else {
      fetchData({ limit: DEFAULT_FETCH_LIMIT });
    }
  }, [fetchData, fetchSchemaData, dataMode]);

  // Note: Data mode changes are handled by the effect above
  // The toggle only updates the store, the effect fetches the data

  // Run current query
  const handleRunQuery = useCallback(() => {
    if (queryState.currentQuery && !queryState.isExecuting) {
      executeQuery(queryState.currentQuery);
    }
  }, [queryState.currentQuery, queryState.isExecuting, executeQuery]);

  // Expand node (double-click in graph) - Neo4j Browser-style
  const handleExpandNode = useCallback(
    async (nodeId: string) => {
      const query = QueryBuilder.expandNode(nodeId);
      await executeQuery(query);
    },
    [executeQuery]
  );

  // Graph click handlers (stable references prevent Graph2D re-renders)
  // Click behavior:
  // - Click: No action (drag & hover still work)
  // - ⌘+Click: Select node + open panel + zoom/center
  // - Double-click: Zoom/center + expand neighbors (NO panel)

  const handlePaneClick = useCallback(() => {
    uiActions.setSelectedNode(null);
  }, [uiActions]);

  const handleClosePanel = useCallback(() => {
    uiActions.setSelectedNode(null);
    uiActions.setSelectedEdge(null);
  }, [uiActions]);

  // View mode setters
  const handleSetQueryViewMode = useCallback(
    (mode: 'graph' | 'table' | 'raw') => {
      queryActions.setViewMode(mode);
    },
    [queryActions]
  );

  // Command palette commands
  const commands = useCommandPalette({
    toggleViewMode: uiActions.toggleViewMode,
    toggleFocusMode: uiActions.toggleFocusMode,
    toggleMinimap: uiActions.toggleMinimap,
    toggleEdgeLabels: uiActions.toggleEdgeLabels,
    toggleSidebar: uiActions.toggleSidebar,
    openShortcuts,
    applyViewPresetByShortcut: filterActions.applyViewPresetByShortcut,
    onRefresh: handleRefresh,
    clearSelection: uiActions.clearSelection,
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // RENDER
  // ═══════════════════════════════════════════════════════════════════════════

  return (
    <div className="h-screen w-screen flex flex-col bg-black overflow-hidden">
      {/* URL Sync - Suspense-wrapped for Next.js compatibility (v8.2.0) */}
      <UrlSyncComponent />

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left Sidebar - Filters & Database Info (Linear-dark) */}
        {!uiState.focusMode && uiState.sidebarOpen && (
          <aside className="w-72 border-r border-white/8 shrink-0 overflow-hidden z-30 bg-[#0d0d12]">
            <SidebarTabs />
          </aside>
        )}

        {/* Toggle sidebar button */}
        {!uiState.focusMode && (
          <button
            onClick={uiActions.toggleSidebar}
            className={cn(
              'absolute top-1/2 -translate-y-1/2 z-20',
              'p-2 bg-[#0d0d12] border border-white/10',
              'hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-all duration-200',
              'shadow-lg shadow-black/50 text-white/50 hover:text-accent-blue',
              uiState.sidebarOpen ? 'left-[284px] rounded-r-lg' : 'left-0 rounded-r-lg'
            )}
            title="Toggle sidebar ([)"
          >
            <PanelLeft className={cn('w-4 h-4', !uiState.sidebarOpen && 'rotate-180')} />
          </button>
        )}

        {/* Canvas */}
        <main className="flex-1 relative">
          {/* Graph Canvas */}
          <div className="absolute inset-0 overflow-hidden">
            {queryState.viewMode === 'table' ? (
              <div className="absolute inset-0 overflow-auto">
                <TableView />
              </div>
            ) : queryState.viewMode === 'raw' ? (
              <div className="absolute inset-0 overflow-auto">
                <RawView />
              </div>
            ) : uiState.viewMode === '2d' ? (
              <GraphErrorBoundary>
                <Suspense
                  fallback={
                    <div className="absolute inset-0 flex items-center justify-center bg-black">
                      <div className="flex flex-col items-center gap-4">
                        <Loader2 className="w-8 h-8 text-novanet-400 animate-spin" />
                        <span className="text-sm text-white/50">Loading graph...</span>
                      </div>
                    </div>
                  }
                >
                  <Graph2D
                    className="absolute inset-0"
                    showMinimap={uiState.minimapVisible}
                    showControls={true}
                    onNodeDoubleClick={handleExpandNode}
                    onPaneClick={handlePaneClick}
                  />
                </Suspense>
              </GraphErrorBoundary>
            ) : (
              // 3D view placeholder
              <div className="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-black via-zinc-950 to-black">
                <div className="text-center">
                  <div className="w-20 h-20 rounded-2xl bg-white/5 flex items-center justify-center mx-auto mb-6">
                    <Box className="w-10 h-10 text-white/20" />
                  </div>
                  <h1 className="text-2xl font-semibold text-white/70 mb-2">3D Graph View</h1>
                  <p className="text-sm text-white/40 mb-6 max-w-md">
                    Immersive 3D visualization coming soon. Navigate through your knowledge graph in three dimensions.
                  </p>
                  <button
                    onClick={uiActions.toggleViewMode}
                    className="px-6 py-2.5 bg-novanet-500 text-white rounded-xl hover:bg-novanet-600 transition-colors font-medium"
                  >
                    Switch to 2D
                  </button>
                </div>
              </div>
            )}

            {/* Top Bar: Unified layout with 2 rows */}
            {!uiState.focusMode && (
              <div className="absolute top-4 left-4 right-4 z-30 flex flex-col gap-3">
                {/* Row 1: QueryPill (full-width) - PRIMARY ACTION */}
                <QueryPill className="w-full" onRun={handleRunQuery} />
                {/* Row 2: Stats (left) + Context Picker (right) */}
                <div className="flex items-center justify-between gap-4">
                  <Pill size="md">
                    <StatsCounter
                      nodeCount={visibleNodeCount}
                      edgeCount={visibleEdgeCount}
                      isLoading={queryState.isExecuting}
                    />
                    {totalNodes > 0 && (
                      <>
                        <Divider />
                        <ResultsOverview />
                      </>
                    )}
                    <Divider />
                    <DataModeToggle />
                    <Divider />
                    <RefreshButton onClick={handleRefresh} isLoading={isFetching} />
                  </Pill>
                  <Pill size="md">
                    <ContextPicker />
                  </Pill>
                </div>
              </div>
            )}

            {/* Bottom left - Keyboard shortcuts */}
            <button
              onClick={openShortcuts}
              className="absolute bottom-4 left-4 p-2.5 rounded-xl bg-[#0d0d12] border border-white/10 hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-all text-white/40 hover:text-accent-blue shadow-lg shadow-black/40"
              title="Keyboard shortcuts (?)"
            >
              <Keyboard className="w-4 h-4" />
            </button>

            {/* Bottom center - View controls + hover info + shortcut hint */}
            <div className="absolute bottom-4 left-1/2 -translate-x-1/2 z-30 flex flex-col items-center gap-1">
              {/* Hover info - shows node/edge details on hover */}
              {(hoveredNode || hoveredEdge) && (
                <div
                  className="glass px-4 py-2 rounded-xl animate-fade-in flex items-center gap-3 text-xs backdrop-blur-xl"
                  style={{
                    borderColor: hoveredEdge && hoveredEdgeNodes
                      ? `${hoveredEdgeNodes.colors.primary}50`
                      : 'rgba(255,255,255,0.1)',
                    boxShadow: hoveredEdge && hoveredEdgeNodes
                      ? `0 0 20px ${hoveredEdgeNodes.colors.primary}30, 0 0 40px ${hoveredEdgeNodes.colors.primary}15, inset 0 0 20px ${hoveredEdgeNodes.colors.primary}08`
                      : undefined,
                  }}
                >
                  {hoveredNode ? (
                    <>
                      <div className="flex items-center gap-2">
                        <CategoryIcon
                          category={NODE_TYPE_CONFIG[hoveredNode.type]?.category || 'project'}
                          size={16}
                          strokeWidth={2}
                          style={{
                            color: NODE_TYPE_CONFIG[hoveredNode.type]?.color || '#888',
                            filter: `drop-shadow(0 0 4px ${NODE_TYPE_CONFIG[hoveredNode.type]?.color || '#888'}80)`,
                          }}
                        />
                        <span
                          className="font-bold uppercase text-[10px] tracking-wider"
                          style={{ color: NODE_TYPE_CONFIG[hoveredNode.type]?.color || '#888' }}
                        >
                          {NODE_TYPE_CONFIG[hoveredNode.type]?.label || hoveredNode.type}
                        </span>
                      </div>
                      <span className="text-white/70 font-mono truncate max-w-[200px]">
                        {hoveredNode.displayName || hoveredNode.key}
                      </span>
                    </>
                  ) : hoveredEdge && hoveredEdgeNodes ? (
                    <>
                      {/* Source Node */}
                      <div className="flex items-center gap-1.5">
                        <CategoryIcon
                          category={NODE_TYPE_CONFIG[hoveredEdgeNodes.sourceNode.type]?.category || 'project'}
                          size={18}
                          strokeWidth={2}
                          style={{
                            color: NODE_TYPE_CONFIG[hoveredEdgeNodes.sourceNode.type]?.color || '#888',
                            filter: `drop-shadow(0 0 6px ${NODE_TYPE_CONFIG[hoveredEdgeNodes.sourceNode.type]?.color || '#888'}80)`,
                          }}
                        />
                        <span className="text-white/90 font-medium truncate max-w-[120px]">
                          {hoveredEdgeNodes.sourceNode.displayName || hoveredEdgeNodes.sourceNode.key}
                        </span>
                      </div>

                      {/* Arrow + Relation Type */}
                      <div className="flex items-center gap-1.5">
                        <span
                          className="text-sm"
                          style={{ color: hoveredEdgeNodes.colors.primary }}
                        >
                          →
                        </span>
                        <span
                          className="font-bold uppercase text-[10px] tracking-wider px-2 py-0.5 rounded-md"
                          style={{
                            color: hoveredEdgeNodes.colors.primary,
                            backgroundColor: `${hoveredEdgeNodes.colors.primary}20`,
                            textShadow: `0 0 8px ${hoveredEdgeNodes.colors.glow}`,
                          }}
                        >
                          {hoveredEdge.type.replace(/_/g, ' ')}
                        </span>
                        <span
                          className="text-sm"
                          style={{ color: hoveredEdgeNodes.colors.primary }}
                        >
                          →
                        </span>
                      </div>

                      {/* Target Node */}
                      <div className="flex items-center gap-1.5">
                        <CategoryIcon
                          category={NODE_TYPE_CONFIG[hoveredEdgeNodes.targetNode.type]?.category || 'project'}
                          size={18}
                          strokeWidth={2}
                          style={{
                            color: NODE_TYPE_CONFIG[hoveredEdgeNodes.targetNode.type]?.color || '#888',
                            filter: `drop-shadow(0 0 6px ${NODE_TYPE_CONFIG[hoveredEdgeNodes.targetNode.type]?.color || '#888'}80)`,
                          }}
                        />
                        <span className="text-white/90 font-medium truncate max-w-[120px]">
                          {hoveredEdgeNodes.targetNode.displayName || hoveredEdgeNodes.targetNode.key}
                        </span>
                      </div>
                    </>
                  ) : null}
                </div>
              )}
              {/* Shortcut hint - concise */}
              <span className="text-[10px] text-white/25 select-none pointer-events-none">
                ⌘+click inspect · dbl-click expand
              </span>
            <Pill size="sm">
              {/* 2D Graph / 3D Graph / Table / Raw - Linear-style segmented control */}
              <div className="flex items-center bg-white/[0.06] rounded-lg p-0.5 border border-white/10">
                <button
                  onClick={() => {
                    handleSetQueryViewMode('graph');
                    if (uiState.viewMode !== '2d') uiActions.toggleViewMode();
                  }}
                  className={cn(
                    'flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs font-medium transition-all duration-150',
                    queryState.viewMode === 'graph' && uiState.viewMode === '2d'
                      ? 'bg-accent-blue/30 text-white border border-accent-blue/50 shadow-sm shadow-accent-blue/20'
                      : 'text-white/50 hover:text-white/80 hover:bg-white/8'
                  )}
                  title="2D Graph view"
                >
                  <Network className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline">2D Graph</span>
                </button>
                <button
                  onClick={() => {
                    handleSetQueryViewMode('graph');
                    if (uiState.viewMode !== '3d') uiActions.toggleViewMode();
                  }}
                  className={cn(
                    'flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs font-medium transition-all duration-150',
                    queryState.viewMode === 'graph' && uiState.viewMode === '3d'
                      ? 'bg-accent-blue/30 text-white border border-accent-blue/50 shadow-sm shadow-accent-blue/20'
                      : 'text-white/50 hover:text-white/80 hover:bg-white/8'
                  )}
                  title="3D Graph view"
                >
                  <Box className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline">3D Graph</span>
                </button>
                <button
                  onClick={() => handleSetQueryViewMode('table')}
                  className={cn(
                    'flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs font-medium transition-all duration-150',
                    queryState.viewMode === 'table'
                      ? 'bg-accent-blue/30 text-white border border-accent-blue/50 shadow-sm shadow-accent-blue/20'
                      : 'text-white/50 hover:text-white/80 hover:bg-white/8'
                  )}
                  title="Table view"
                >
                  <Table2 className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline">Table</span>
                </button>
                <button
                  onClick={() => handleSetQueryViewMode('raw')}
                  className={cn(
                    'flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs font-medium transition-all duration-150',
                    queryState.viewMode === 'raw'
                      ? 'bg-accent-blue/30 text-white border border-accent-blue/50 shadow-sm shadow-accent-blue/20'
                      : 'text-white/50 hover:text-white/80 hover:bg-white/8'
                  )}
                  title="Raw JSON view"
                >
                  <Code className="w-3.5 h-3.5" />
                  <span className="hidden sm:inline">Raw</span>
                </button>
              </div>
            </Pill>
            </div>

            {/* Focus mode indicator */}
            {uiState.focusMode && (
              <div className="absolute top-4 left-1/2 -translate-x-1/2 glass px-4 py-2.5 flex items-center gap-3 text-sm">
                <div className="w-2 h-2 rounded-full bg-novanet-400 animate-pulse" />
                <span className="text-white/70">Focus Mode</span>
                <span className="text-white/40">Press</span>
                <kbd className="bg-white/10 px-1.5 py-0.5 rounded text-xs">G</kbd>
                <span className="text-white/40">to exit</span>
              </div>
            )}
          </div>
        </main>

        {/* Right Panel - Node/Edge Details (slides in on selection) */}
        {!uiState.focusMode && (selectedNode || selectedEdge) && (
          <aside className="w-[420px] border-l border-white/8 shrink-0 overflow-hidden flex flex-col bg-[#0d0d12] animate-slide-in-right">
            {/* Panel Header */}
            <div className="flex items-center justify-between px-5 py-4 border-b border-white/[0.08]">
              <span className="text-sm font-semibold text-white/80">
                {selectedEdge ? 'Relationship Details' : 'Node Details'}
              </span>
              <button
                onClick={handleClosePanel}
                className="p-1.5 rounded-lg hover:bg-white/10 text-white/40 hover:text-white/60 transition-colors"
                title="Close (] or Esc)"
              >
                <X className="w-4 h-4" />
              </button>
            </div>

            {/* Node/Edge Details */}
            <div className="flex-1 overflow-hidden">
              {selectedEdge ? (
                <EdgeDetailsPanel edge={selectedEdge} />
              ) : selectedNode ? (
                <NodeDetailsPanel node={selectedNode} />
              ) : null}
            </div>
          </aside>
        )}
      </div>

      {/* Keyboard Shortcuts Modal */}
      <KeyboardShortcuts isOpen={shortcutsOpen} onClose={closeShortcuts} />

      {/* Command Palette (⌘K) */}
      <CommandPalette isOpen={paletteOpen} onClose={closePalette} commands={commands} />
    </div>
  );
}
