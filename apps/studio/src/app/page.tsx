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

import { useCallback, useEffect, useMemo, useRef, useState, lazy, Suspense } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { Box, PanelLeft, Keyboard, X, Network, Table2, Code, Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { useUIStore, useFilterStore, useGraphStore, useAnimationStore } from '@/stores';
import { useGraphData, useFilteredGraph, UrlSyncComponent } from '@/hooks';
import { ContextPicker, ViewPicker } from '@/components/sidebar';
import { isInputFocused, matchesKeyCombo } from '@/lib/keyboard';
import { NODE_TYPE_CONFIG, type NodeTypeConfig } from '@/config/nodeTypes';
import type { GraphNode, HoverNodeInfo, SchemaGroupNode, NodeType, SchemaGroupType } from '@/types';
import { resolveThemeCached } from '@/components/graph/edges';

// Helper to safely get node config for hover tooltips (handles both real nodes and schema groups)
function getHoverNodeConfig(node: HoverNodeInfo): NodeTypeConfig | null {
  // Schema groups don't have a config entry
  if (node.type === 'RealmGroup' || node.type === 'LayerGroup') {
    return null;
  }
  return NODE_TYPE_CONFIG[node.type as NodeType] || null;
}

// Lazy load Graph2D - saves ~400KB from initial bundle
const Graph2D = lazy(() =>
  import('@/components/graph').then((mod) => ({ default: mod.Graph2D }))
);
import { GraphErrorBoundary } from '@/components/ui/ErrorBoundary';
import { StatsCounter, Pill, Divider, RefreshButton, LayerIcon, MatrixRainOverlay } from '@/components/ui';
import { SidebarTabs } from '@/components/sidebar/SidebarTabs';
import { NavigationModeToggle } from '@/components/toolbar/NavigationModeToggle';
import { NodeDetailsPanel } from '@/components/sidebar/NodeDetailsPanel';
import { EdgeDetailsPanel } from '@/components/sidebar/EdgeDetailsPanel';
import { KeyboardHelpPanel } from '@/components/dx/KeyboardHelpPanel';
import { CommandPalette, useCommandPalette, useCommandPaletteState } from '@/components/ui/CommandPalette';
import { AiSearchOverlay } from '@/components/chat/AiSearchOverlay';
import { QueryPill, ResultsOverview, ExpandedBreakdown, TableView, RawView } from '@/components/query';
import type { ExpandedViewType } from '@/components/query/ResultsOverview';
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
      selectedEdgeData: state.selectedEdgeData,
      hoveredNodeId: state.hoveredNodeId,
      hoveredEdgeId: state.hoveredEdgeId,
      activeModal: state.activeModal,
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
      triggerLayout: state.triggerLayout,
      toggleLayoutMode: state.toggleLayoutMode,
      setSelectedNode: state.setSelectedNode,
      setSelectedEdge: state.setSelectedEdge,
      clearSelection: state.clearSelection,
      openModal: state.openModal,
      closeModal: state.closeModal,
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
      setTraitFilter: state.setTraitFilter,
      setEdgeFamilyFilter: state.setEdgeFamilyFilter,
    }))
  );
  const traitFilter = useFilterStore((s) => s.traitFilter);
  const edgeFamilyFilter = useFilterStore((s) => s.edgeFamilyFilter);

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
      executeQuery: state.executeQuery,
    }))
  );

  // Animation Store - Matrix transition state
  const transitionState = useAnimationStore(
    useShallow((state) => ({
      isTransitioning: state.isTransitioning,
      transitionPhase: state.transitionPhase,
      targetMode: state.targetMode,
    }))
  );
  const transitionActions = useAnimationStore(
    useShallow((state) => ({
      startTransition: state.startTransition,
      setTransitionPhase: state.setTransitionPhase,
      endTransition: state.endTransition,
    }))
  );

  // UI action for data mode (used by transition orchestration)
  const setNavigationMode = useUIStore((state) => state.setNavigationMode);

  // ═══════════════════════════════════════════════════════════════════════════
  // DERIVED STATE & HOOKS
  // ═══════════════════════════════════════════════════════════════════════════

  // Filtered graph stats (schema mode shows distinct relation types, not edges)
  const { visibleNodeCount, visibleEdgeCount, isMetaMode, distinctRelationTypes } = useFilteredGraph();

  // Graph data fetching
  const { fetchData, fetchSchemaData, executeQuery, navigationMode, isLoading: isFetching } = useGraphData();

  // Stats pill expansion (hover nodes/relations to see type breakdown)
  const [expandedView, setExpandedView] = useState<ExpandedViewType>(null);
  const hoverTimeoutRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  const openExpanded = useCallback((view: ExpandedViewType) => {
    if (hoverTimeoutRef.current) clearTimeout(hoverTimeoutRef.current);
    setExpandedView(view);
  }, []);

  const scheduleCloseExpanded = useCallback(() => {
    hoverTimeoutRef.current = setTimeout(() => setExpandedView(null), 150);
  }, []);

  const cancelCloseExpanded = useCallback(() => {
    if (hoverTimeoutRef.current) clearTimeout(hoverTimeoutRef.current);
  }, []);

  // Keyboard shortcuts modal
  const [shortcutsOpen, setShortcutsOpen] = useState(false);
  const openShortcuts = useCallback(() => setShortcutsOpen(true), []);
  const closeShortcuts = useCallback(() => setShortcutsOpen(false), []);

  // Command palette (⌘K)
  const { isOpen: paletteOpen, open: openPalette, close: closePalette } = useCommandPaletteState();

  // AI search overlay (⌘J) - uses uiStore exclusive modal system
  const openAiSearch = useCallback(() => uiActions.openModal('ai-chat'), [uiActions]);
  const closeAiSearch = useCallback(() => uiActions.closeModal(), [uiActions]);
  const aiSearchOpen = uiState.activeModal === 'ai-chat';

  // Get selected node/edge data (memoized)
  const selectedNode = useMemo(
    () => (uiState.selectedNodeId ? getNodeById(uiState.selectedNodeId) : null),
    [uiState.selectedNodeId, getNodeById]
  );
  // Use selectedEdgeData from store (supports both data and schema modes)
  // Fallback to getEdgeById for backwards compatibility
  const selectedEdge = useMemo(
    () => uiState.selectedEdgeData ?? (uiState.selectedEdgeId ? getEdgeById(uiState.selectedEdgeId) : null),
    [uiState.selectedEdgeData, uiState.selectedEdgeId, getEdgeById]
  );

  // Get hovered node/edge data (memoized) for centralized hover info
  // Supports both data mode (full node data) and schema mode (synthetic from node id)
  const hoveredNode = useMemo((): HoverNodeInfo | null => {
    if (!uiState.hoveredNodeId) return null;

    // Data mode: look up in graphStore
    const node = getNodeById(uiState.hoveredNodeId);
    if (node) return node;

    // Schema mode: handle different node id formats
    if (navigationMode === 'meta') {
      const nodeId = uiState.hoveredNodeId;

      // Realm containers: realm-{Realm} (e.g., realm-global, realm-project)
      if (nodeId.startsWith('realm-')) {
        const realm = nodeId.replace('realm-', '');
        const realmEmoji = realm === 'global' ? '🌍' : realm === 'project' ? '📦' : '🎯';
        const realmLabel = realm.charAt(0).toUpperCase() + realm.slice(1);
        return {
          id: nodeId,
          type: 'RealmGroup',
          key: realm,
          displayName: `${realmEmoji} ${realmLabel} Realm`,
        } as SchemaGroupNode;
      }

      // Layer containers: layer-{Realm}-{LayerName}
      if (nodeId.startsWith('layer-')) {
        const parts = nodeId.replace('layer-', '').split('-');
        const layerName = parts.slice(1).join('-');
        return {
          id: nodeId,
          type: 'LayerGroup',
          key: layerName,
          displayName: layerName.charAt(0).toUpperCase() + layerName.slice(1),
        } as SchemaGroupNode;
      }

      // Schema nodes: schema-{NodeType}
      if (nodeId.startsWith('schema-')) {
        const nodeType = nodeId.replace('schema-', '');
        const config = NODE_TYPE_CONFIG[nodeType as keyof typeof NODE_TYPE_CONFIG];
        if (config) {
          return {
            id: nodeId,
            type: nodeType,
            key: nodeType,
            displayName: config.label,
          } as GraphNode;
        }
      }
    }

    return null;
  }, [uiState.hoveredNodeId, getNodeById, navigationMode]);
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

  // Track previous navigationMode to detect changes
  const prevNavigationModeRef = useRef<typeof navigationMode | null>(null);

  // Fetch initial data OR re-fetch when navigationMode changes (e.g., from URL sync)
  useEffect(() => {
    if (isFetching) return;

    // Check if this is a mode change (vs initial load)
    const isInitialLoad = totalNodes === 0;
    const modeChanged = prevNavigationModeRef.current !== null && prevNavigationModeRef.current !== navigationMode;

    // Fetch on initial load OR mode change
    if (isInitialLoad || modeChanged) {
      if (navigationMode === 'meta') {
        fetchSchemaData();
      } else {
        fetchData({ limit: DEFAULT_FETCH_LIMIT });
      }
    }

    prevNavigationModeRef.current = navigationMode;
  }, [totalNodes, isFetching, fetchData, fetchSchemaData, navigationMode]);

  // ═══════════════════════════════════════════════════════════════════════════
  // MATRIX TRANSITION ORCHESTRATION
  // ═══════════════════════════════════════════════════════════════════════════
  //
  // Phase flow: dissolve (400ms) → fetch (variable) → reform (400ms)
  // - dissolve: fade out graph, show matrix rain
  // - fetch: load new data in background
  // - reform: fade in new graph, hide matrix rain
  //
  useEffect(() => {
    if (!transitionState.isTransitioning) return;

    const { transitionPhase, targetMode } = transitionState;

    // Phase 1: DISSOLVE - wait 400ms, then switch mode and fetch data
    if (transitionPhase === 'dissolve' && targetMode) {
      const timer = setTimeout(() => {
        // Switch to fetch phase and update data mode
        transitionActions.setTransitionPhase('fetch');
        setNavigationMode(targetMode);
        // Data fetch is triggered automatically by the navigationMode change effect above
      }, 400);
      return () => clearTimeout(timer);
    }

    // Phase 2: FETCH - wait for data to load, then move to reform
    // We detect when fetch completes by watching isFetching go from true to false
    if (transitionPhase === 'fetch' && !isFetching) {
      transitionActions.setTransitionPhase('reform');
    }

    // Phase 3: REFORM - wait 400ms, then end transition
    if (transitionPhase === 'reform') {
      const timer = setTimeout(() => {
        transitionActions.endTransition();
      }, 400);
      return () => clearTimeout(timer);
    }
  }, [
    transitionState.isTransitioning,
    transitionState.transitionPhase,
    transitionState.targetMode,
    isFetching,
    setNavigationMode,
    transitionActions,
  ]);

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

      // AI search overlay
      if (matchesKeyCombo(e, 'mod+j')) {
        e.preventDefault();
        openAiSearch();
        return;
      }

      // Keyboard shortcuts (/)
      if (e.key === '/' && !e.metaKey && !e.ctrlKey) {
        e.preventDefault();
        openShortcuts();
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

      // Cycle navigation mode (N) - triggers Matrix transition
      if (e.key === 'n' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        if (!transitionState.isTransitioning) {
          const modes: typeof navigationMode[] = ['data', 'meta', 'overlay', 'query'];
          const idx = modes.indexOf(navigationMode);
          const nextMode = modes[(idx + 1) % modes.length];
          transitionActions.startTransition(nextMode);
        }
        return;
      }

      // Cycle trait filter (T) - none → invariant → localized → knowledge → derived → job → none
      if (e.key === 't' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        const traits = ['invariant', 'localized', 'knowledge', 'derived', 'job'] as const;
        if (traitFilter.length === 0) {
          filterActions.setTraitFilter([traits[0]]);
        } else {
          const idx = traits.indexOf(traitFilter[0] as typeof traits[number]);
          if (idx >= 0 && idx < traits.length - 1) {
            filterActions.setTraitFilter([traits[idx + 1]]);
          } else {
            filterActions.setTraitFilter([]);
          }
        }
        return;
      }

      // Cycle edge family filter (E) - none → ownership → localization → semantic → generation → mining → none
      if (e.key === 'e' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        const families = ['ownership', 'localization', 'semantic', 'generation', 'mining'];
        if (edgeFamilyFilter.length === 0) {
          filterActions.setEdgeFamilyFilter([families[0]]);
        } else {
          const idx = families.indexOf(edgeFamilyFilter[0]);
          if (idx >= 0 && idx < families.length - 1) {
            filterActions.setEdgeFamilyFilter([families[idx + 1]]);
          } else {
            filterActions.setEdgeFamilyFilter([]);
          }
        }
        return;
      }

      // Layout shortcuts (Shift + H/V/D/R/F/M) - must use triggerLayout to increment layoutVersion
      if (e.shiftKey && !e.metaKey && !e.ctrlKey) {
        switch (e.key) {
          case 'H':
            e.preventDefault();
            uiActions.triggerLayout('LR');
            return;
          case 'V':
            e.preventDefault();
            uiActions.triggerLayout('TB');
            return;
          case 'D':
            e.preventDefault();
            uiActions.triggerLayout('dagre');
            return;
          case 'R':
            e.preventDefault();
            uiActions.triggerLayout('radial');
            return;
          case 'F':
            e.preventDefault();
            uiActions.triggerLayout('force');
            return;
          case 'M':
            e.preventDefault();
            uiActions.toggleLayoutMode();
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
  }, [uiActions, filterActions, shortcutsOpen, closeShortcuts, openPalette, openAiSearch, navigationMode, transitionState.isTransitioning, transitionActions, traitFilter, edgeFamilyFilter]);

  // ═══════════════════════════════════════════════════════════════════════════
  // MEMOIZED HANDLERS
  // ═══════════════════════════════════════════════════════════════════════════

  // Refresh data
  const handleRefresh = useCallback(() => {
    if (navigationMode === 'meta') {
      fetchSchemaData();
    } else {
      fetchData({ limit: DEFAULT_FETCH_LIMIT });
    }
  }, [fetchData, fetchSchemaData, navigationMode]);

  // Note: Data mode changes are handled by the effect above
  // The toggle only updates the store, the effect fetches the data

  // Run current query (uses queryStore.executeQuery for matrix animation)
  const handleRunQuery = useCallback(() => {
    if (queryState.currentQuery && !queryState.isExecuting) {
      queryActions.executeQuery(queryState.currentQuery);
    }
  }, [queryState.currentQuery, queryState.isExecuting, queryActions]);

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
          <aside className="w-80 border-r border-white/8 shrink-0 overflow-hidden z-30 bg-[#0d0d12]">
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
              'hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-colors duration-200',
              'shadow-lg shadow-black/50 text-white/50 hover:text-accent-blue',
              uiState.sidebarOpen ? 'left-[304px] rounded-r-lg' : 'left-0 rounded-r-lg'
            )}
            title="Toggle sidebar ([)"
          >
            <PanelLeft className={cn(iconSizes.md, !uiState.sidebarOpen && 'rotate-180')} />
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
                      <div className={cn('flex flex-col items-center', gapTokens.large)}>
                        <Loader2 className={cn('text-novanet-400 animate-spin', 'w-8 h-8')} />
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
                    <Box className="w-10 h-10 text-white/40" />
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

            {/* Matrix Rain Overlay - shows during Data ↔ Schema transitions */}
            <MatrixRainOverlay
              visible={transitionState.isTransitioning}
              phase={transitionState.transitionPhase}
            />

            {/* Top Bar: Unified layout with 2 rows */}
            {!uiState.focusMode && (
              <div className={cn('absolute top-4 left-4 right-4 z-30 flex flex-col', gapTokens.spacious)}>
                {/* Row 1: QueryPill (full-width) - only in Data mode */}
                {navigationMode === 'data' && (
                  <QueryPill className="w-full" onRun={handleRunQuery} />
                )}
                {/* Row 2: Stats (left) + Navigation Mode (center) + Context Picker (right) */}
                <div className={cn('flex items-start justify-between', gapTokens.large)}>
                  <Pill size="md" className="items-stretch py-3" glow={queryState.isExecuting || transitionState.isTransitioning} glowColor={transitionState.isTransitioning ? 'novanet' : 'emerald'}>
                    <div className="relative z-10 flex flex-col w-full">
                      {/* Main row */}
                      <div className={cn('flex items-center', gapTokens.default)}>
                        <StatsCounter
                          nodeCount={visibleNodeCount}
                          edgeCount={isMetaMode ? distinctRelationTypes : visibleEdgeCount}
                          isLoading={queryState.isExecuting}
                          expandedView={expandedView}
                          onHoverNodes={() => openExpanded('nodes')}
                          onHoverRelations={() => openExpanded('relations')}
                          onHoverLeave={scheduleCloseExpanded}
                          isMetaMode={isMetaMode}
                        />
                        {visibleNodeCount > 0 && (
                          <>
                            <Divider />
                            <ResultsOverview
                              expandedView={expandedView}
                              onHoverOverflow={() => openExpanded('all')}
                              onHoverLeave={scheduleCloseExpanded}
                            />
                          </>
                        )}
                        <Divider />
                        <RefreshButton onClick={handleRefresh} isLoading={isFetching} />
                      </div>
                      {/* Expanded breakdown - animates in/out */}
                      <ExpandedBreakdown
                        view={expandedView}
                        onMouseEnter={cancelCloseExpanded}
                        onMouseLeave={scheduleCloseExpanded}
                      />
                    </div>
                  </Pill>
                  <NavigationModeToggle
                    mode={navigationMode}
                    onModeChange={setNavigationMode}
                  />
                  <Pill size="md">
                    {navigationMode === 'meta' ? <ViewPicker /> : <ContextPicker />}
                  </Pill>
                </div>
              </div>
            )}

            {/* Bottom left - Keyboard shortcuts */}
            <button
              onClick={openShortcuts}
              className={cn('absolute bottom-4 left-4 px-3 py-2 rounded-xl bg-[#0d0d12] border border-white/10 hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-colors text-white/40 hover:text-accent-blue shadow-lg shadow-black/40 flex items-center', gapTokens.default)}
              title="Keyboard shortcuts (/)"
            >
              <Keyboard className={iconSizes.md} />
              <kbd className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/10 border border-white/10">/</kbd>
            </button>

            {/* Bottom center - View controls + hover info + shortcut hint */}
            <div className={cn('absolute bottom-4 left-1/2 -translate-x-1/2 z-30 flex flex-col items-center', gapTokens.tight)}>
              {/* Hover info - shows node/edge details on hover */}
              {(hoveredNode || hoveredEdge) && (
                <div
                  className={cn('glass px-4 py-2 rounded-xl animate-fade-in flex items-center text-xs', gapTokens.spacious)}
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
                    (() => {
                      const config = getHoverNodeConfig(hoveredNode);
                      const color = config?.color || '#888';
                      return (
                        <>
                          <div className={cn('flex items-center', gapTokens.default)}>
                            <LayerIcon
                              layer={config?.layer || 'foundation'}
                              size={16}
                              strokeWidth={2}
                              style={{
                                color,
                                filter: `drop-shadow(0 0 4px ${color}80)`,
                              }}
                            />
                            <span
                              className="font-bold uppercase text-[10px] tracking-wider"
                              style={{ color }}
                            >
                              {config?.label || hoveredNode.type}
                            </span>
                          </div>
                          <span className="text-white/70 font-mono truncate max-w-[200px]">
                            {hoveredNode.displayName || hoveredNode.key}
                          </span>
                        </>
                      );
                    })()
                  ) : hoveredEdge && hoveredEdgeNodes ? (
                    <>
                      {/* Source Node */}
                      <div className={cn('flex items-center', gapTokens.compact)}>
                        <LayerIcon
                          layer={NODE_TYPE_CONFIG[hoveredEdgeNodes.sourceNode.type]?.layer || 'foundation'}
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
                      <div className={cn('flex items-center', gapTokens.compact)}>
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
                      <div className={cn('flex items-center', gapTokens.compact)}>
                        <LayerIcon
                          layer={NODE_TYPE_CONFIG[hoveredEdgeNodes.targetNode.type]?.layer || 'foundation'}
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
              <span className="text-[10px] text-white/40 select-none pointer-events-none">
                click inspect · dbl-click expand
              </span>
            <Pill size="sm">
              {/* 2D Graph / 3D Graph / Table / Raw - Unified glass segmented control */}
              <div className="flex items-center rounded-lg p-0.5">
                <button
                  onClick={() => {
                    handleSetQueryViewMode('graph');
                    if (uiState.viewMode !== '2d') uiActions.toggleViewMode();
                  }}
                  className={cn(
                    'flex items-center px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors duration-150',
                    gapTokens.compact,
                    queryState.viewMode === 'graph' && uiState.viewMode === '2d'
                      ? 'bg-white/[0.10] text-white/90 border border-white/[0.15]'
                      : 'text-white/40 border border-transparent hover:text-white/70 hover:bg-white/[0.06]'
                  )}
                  title="2D Graph view"
                >
                  <Network className={iconSizes.sm} />
                  <span className="hidden sm:inline">2D Graph</span>
                </button>
                <button
                  onClick={() => {
                    handleSetQueryViewMode('graph');
                    if (uiState.viewMode !== '3d') uiActions.toggleViewMode();
                  }}
                  className={cn(
                    'flex items-center px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors duration-150',
                    gapTokens.compact,
                    queryState.viewMode === 'graph' && uiState.viewMode === '3d'
                      ? 'bg-white/[0.10] text-white/90 border border-white/[0.15]'
                      : 'text-white/40 border border-transparent hover:text-white/70 hover:bg-white/[0.06]'
                  )}
                  title="3D Graph view"
                >
                  <Box className={iconSizes.sm} />
                  <span className="hidden sm:inline">3D Graph</span>
                </button>
                <button
                  onClick={() => handleSetQueryViewMode('table')}
                  className={cn(
                    'flex items-center px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors duration-150',
                    gapTokens.compact,
                    queryState.viewMode === 'table'
                      ? 'bg-white/[0.10] text-white/90 border border-white/[0.15]'
                      : 'text-white/40 border border-transparent hover:text-white/70 hover:bg-white/[0.06]'
                  )}
                  title="Table view"
                >
                  <Table2 className={iconSizes.sm} />
                  <span className="hidden sm:inline">Table</span>
                </button>
                <button
                  onClick={() => handleSetQueryViewMode('raw')}
                  className={cn(
                    'flex items-center px-2.5 py-1.5 rounded-md text-xs font-medium transition-colors duration-150',
                    gapTokens.compact,
                    queryState.viewMode === 'raw'
                      ? 'bg-white/[0.10] text-white/90 border border-white/[0.15]'
                      : 'text-white/40 border border-transparent hover:text-white/70 hover:bg-white/[0.06]'
                  )}
                  title="Raw JSON view"
                >
                  <Code className={iconSizes.sm} />
                  <span className="hidden sm:inline">Raw</span>
                </button>
              </div>
            </Pill>
            </div>

            {/* Focus mode indicator */}
            {uiState.focusMode && (
              <div className={cn('absolute top-4 left-1/2 -translate-x-1/2 glass px-4 py-2.5 flex items-center text-sm', gapTokens.spacious)}>
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
                <X className={iconSizes.md} />
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
      <KeyboardHelpPanel isOpen={shortcutsOpen} onClose={closeShortcuts} />

      {/* Command Palette (⌘K) */}
      <CommandPalette isOpen={paletteOpen} onClose={closePalette} commands={commands} />

      {/* AI Search Overlay (⌘J) */}
      <AiSearchOverlay
        isOpen={aiSearchOpen}
        onClose={closeAiSearch}
        onExecuteQuery={executeQuery}
      />
    </div>
  );
}
