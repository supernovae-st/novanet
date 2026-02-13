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

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { X, Loader2, HelpCircle, Layers, Gamepad2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
// Note: DEFAULT_FETCH_LIMIT not currently used but may be needed for pagination
import { useUIStore, useFilterStore, useGraphStore, useAnimationStore, useViewStore } from '@/stores';
import { useGraphData, useFilteredGraph, UrlSyncComponent } from '@/hooks';
import { ViewPicker } from '@/components/sidebar';
import { isInputFocused, matchesKeyCombo } from '@/lib/keyboard';
import { NODE_TYPE_CONFIG, type NodeTypeConfig } from '@/config/nodeTypes';
import type { GraphNode, HoverNodeInfo, SchemaGroupNode, NodeType } from '@/types';
import { resolveThemeCached } from '@/components/graph/edges';

// Helper to safely get node config for hover tooltips (handles both real nodes and schema groups)
function getHoverNodeConfig(node: HoverNodeInfo): NodeTypeConfig | null {
  // Schema groups don't have a config entry
  if (node.type === 'RealmGroup' || node.type === 'LayerGroup') {
    return null;
  }
  return NODE_TYPE_CONFIG[node.type as NodeType] || null;
}

// Type guard: check if a node is a schema group (Realm or Layer container)
function isSchemaGroupNode(node: HoverNodeInfo): node is SchemaGroupNode {
  return node.type === 'RealmGroup' || node.type === 'LayerGroup';
}

// Import GraphCanvas for 2D/3D view switching
import { GraphCanvas, Graph3DLegend } from '@/components/graph';
import { GraphErrorBoundary } from '@/components/ui/ErrorBoundary';
import { StatsCounter, Pill, Divider, RefreshButton, LayerIcon, MatrixRainOverlay, MatrixExplosionOverlay, DisplayLimitSelector, LocaleFilterSelector } from '@/components/ui';
import { ViewModeToggle } from '@/components/toolbar/ViewModeToggle';
import { TabbedDetailPanel } from '@/components/sidebar/TabbedDetailPanel';
import { TabbedArcPanel } from '@/components/sidebar/TabbedArcPanel';
import { KeyboardHelpPanel } from '@/components/dx/KeyboardHelpPanel';
import { CommandPalette, useCommandPalette, useCommandPaletteState } from '@/components/ui/CommandPalette';
import { AiSearchOverlay } from '@/components/chat/AiSearchOverlay';
import { MacropadVisualizer } from '@/components/macropad';
import { QueryPill, ExpandedBreakdown, TableView, RawView } from '@/components/query';
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
      setViewMode: state.setViewMode,
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
      setArcFamilyFilter: state.setArcFamilyFilter,
    }))
  );
  const traitFilter = useFilterStore((s) => s.traitFilter);
  const arcFamilyFilter = useFilterStore((s) => s.arcFamilyFilter);

  // Graph Store - state + selectors
  const _totalNodes = useGraphStore((state) => state.totalNodes);  // May be needed for stats
  const getNodeById = useGraphStore((state) => state.getNodeById);
  const getEdgeById = useGraphStore((state) => state.getEdgeById);
  const _clearGraph = useGraphStore((state) => state.clearGraph);  // May be needed for reset

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

  // Animation Store - Matrix transition state (for visual effects)
  const transitionState = useAnimationStore(
    useShallow((state) => ({
      isTransitioning: state.isTransitioning,
      transitionPhase: state.transitionPhase,
      targetMode: state.targetMode,
    }))
  );

  // View Store - v12: Views are the single source of truth for navigation
  const viewState = useViewStore(
    useShallow((state) => ({
      activeViewId: state.activeViewId,
      isRegistryLoaded: state.isRegistryLoaded,
      isExecuting: state.isExecuting,
    }))
  );
  const viewActions = useViewStore(
    useShallow((state) => ({
      loadDefaultView: state.loadDefaultView,
      executeView: state.executeView,
    }))
  );


  // ═══════════════════════════════════════════════════════════════════════════
  // DERIVED STATE & HOOKS
  // ═══════════════════════════════════════════════════════════════════════════

  // Filtered graph stats (schema mode shows distinct relation types, not edges)
  const { visibleNodeCount, visibleEdgeCount, isMetaMode, distinctRelationTypes } = useFilteredGraph();

  // Graph data fetching - v12: Most fetching now goes through viewStore
  const { executeQuery, isLoading: isFetching } = useGraphData();

  // Stats pill expansion (hover nodes/relations to see type breakdown)
  const [expandedView, setExpandedView] = useState<ExpandedViewType>(null);
  const hoverTimeoutRef = useRef<ReturnType<typeof setTimeout>>(undefined);
  // META badge hover triggers Matrix effect on pill
  const [isMetaHovered, setIsMetaHovered] = useState(false);
  // Easter egg - Matrix explosion effect
  const [isExplosionActive, setIsExplosionActive] = useState(false);

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

  // Visual encoding legend
  const [legendOpen, setLegendOpen] = useState(false);
  const toggleLegend = useCallback(() => setLegendOpen(prev => !prev), []);

  // Command palette (⌘K)
  const { isOpen: paletteOpen, open: openPalette, close: closePalette } = useCommandPaletteState();

  // AI search overlay (⌘J) - uses uiStore exclusive modal system
  const openAiSearch = useCallback(() => uiActions.openModal('ai-chat'), [uiActions]);
  const closeAiSearch = useCallback(() => uiActions.closeModal(), [uiActions]);
  const aiSearchOpen = uiState.activeModal === 'ai-chat';

  // Macropad configurator (P) - uses uiStore exclusive modal system
  const openMacropad = useCallback(() => uiActions.openModal('macropad-configurator'), [uiActions]);
  const _closeMacropad = useCallback(() => uiActions.closeModal(), [uiActions]);  // Close via modal callback
  const macropadOpen = uiState.activeModal === 'macropad-configurator';

  // Get selected node/edge data (memoized)
  // Supports both data nodes (full node data) and schema nodes (synthetic from node id)
  const selectedNode = useMemo((): HoverNodeInfo | null => {
    if (!uiState.selectedNodeId) return null;

    // Data nodes: look up in graphStore
    const node = getNodeById(uiState.selectedNodeId);
    if (node) return node;

    // Schema nodes: handle different node id formats (based on prefix)
    const nodeId = uiState.selectedNodeId;

    // Realm containers: realm-{Realm} (e.g., realm-shared, realm-org)
    if (nodeId.startsWith('realm-')) {
      const realm = nodeId.replace('realm-', '');
      const realmEmoji = realm === 'shared' ? '🌍' : realm === 'org' ? '🏢' : '🎯';
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

    return null;
  }, [uiState.selectedNodeId, getNodeById]);
  // Use selectedEdgeData from store (supports both data and schema modes)
  // Fallback to getEdgeById for backwards compatibility
  const selectedEdge = useMemo(
    () => uiState.selectedEdgeData ?? (uiState.selectedEdgeId ? getEdgeById(uiState.selectedEdgeId) : null),
    [uiState.selectedEdgeData, uiState.selectedEdgeId, getEdgeById]
  );

  // Get hovered node/edge data (memoized) for centralized hover info
  // Supports both data nodes (full node data) and schema nodes (synthetic from node id)
  const hoveredNode = useMemo((): HoverNodeInfo | null => {
    if (!uiState.hoveredNodeId) return null;

    // Data nodes: look up in graphStore
    const node = getNodeById(uiState.hoveredNodeId);
    if (node) return node;

    // Schema nodes: handle different node id formats (based on prefix)
    const nodeId = uiState.hoveredNodeId;

    // Realm containers: realm-{Realm} (e.g., realm-shared, realm-org)
    if (nodeId.startsWith('realm-')) {
      const realm = nodeId.replace('realm-', '');
      const realmEmoji = realm === 'shared' ? '🌍' : realm === 'org' ? '🏢' : '🎯';
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

    return null;
  }, [uiState.hoveredNodeId, getNodeById]);
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

  // v12.0: Initial data loading via viewStore.loadDefaultView()
  // Loads the registry and executes the persisted/default view
  useEffect(() => {
    viewActions.loadDefaultView();
  }, [viewActions.loadDefaultView]);

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

      // Macropad configurator
      if (e.key === 'p' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        openMacropad();
        return;
      }

      // Cycle trait filter (T) - none → defined → authored → imported → generated → retrieved → none
      // v11.8: Renamed per ADR-024 Data Origin semantics
      if (e.key === 't' && !e.metaKey && !e.ctrlKey && !e.shiftKey) {
        e.preventDefault();
        const traits = ['defined', 'authored', 'imported', 'generated', 'retrieved'] as const;
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
        if (arcFamilyFilter.length === 0) {
          filterActions.setArcFamilyFilter([families[0]]);
        } else {
          const idx = families.indexOf(arcFamilyFilter[0]);
          if (idx >= 0 && idx < families.length - 1) {
            filterActions.setArcFamilyFilter([families[idx + 1]]);
          } else {
            filterActions.setArcFamilyFilter([]);
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
  }, [uiActions, filterActions, shortcutsOpen, closeShortcuts, openPalette, openAiSearch, openMacropad, traitFilter, arcFamilyFilter]);

  // ═══════════════════════════════════════════════════════════════════════════
  // MEMOIZED HANDLERS
  // ═══════════════════════════════════════════════════════════════════════════

  // Refresh data - v12.0: Re-executes the current view
  const handleRefresh = useCallback(() => {
    viewActions.executeView(viewState.activeViewId);
  }, [viewActions, viewState.activeViewId]);

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

  const _handleClosePanel = useCallback(() => {
    uiActions.setSelectedNode(null);
    uiActions.setSelectedEdge(null);
  }, [uiActions]);  // May be needed for panel close button

  // Command palette commands
  const commands = useCommandPalette({
    toggleViewMode: uiActions.toggleViewMode,
    toggleFocusMode: uiActions.toggleFocusMode,
    toggleMinimap: uiActions.toggleMinimap,
    toggleEdgeLabels: uiActions.toggleEdgeLabels,
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
            ) : (
              /* GraphCanvas handles 2D/3D view switching internally */
              <GraphErrorBoundary>
                <GraphCanvas
                  className="absolute inset-0"
                  showMinimap={uiState.minimapVisible}
                  showControls={true}
                  onNodeDoubleClick={handleExpandNode}
                  onPaneClick={handlePaneClick}
                  showViewToggle={false}
                />
              </GraphErrorBoundary>
            )}

            {/* Matrix Rain Overlay - shows during Data ↔ Schema transitions */}
            <MatrixRainOverlay
              visible={transitionState.isTransitioning}
              phase={transitionState.transitionPhase}
            />

            {/* Loading indicator during fetch phase */}
            {transitionState.transitionPhase === 'fetch' && (
              <div className="absolute inset-0 z-20 flex items-center justify-center pointer-events-none">
                <div className="flex flex-col items-center gap-3">
                  <Loader2 className="w-8 h-8 text-novanet-400 animate-spin" />
                  <span className="text-sm text-white/50 font-mono">
                    Loading {transitionState.targetMode} mode...
                  </span>
                </div>
              </div>
            )}

            {/* Top Bar: Unified layout with 2 rows */}
            {!uiState.focusMode && (
              <div className={cn('absolute top-4 left-4 right-4 z-30 flex flex-col', gapTokens.spacious)}>
                {/* Row 1: QueryPill (full-width) - always visible, query drives data */}
                <QueryPill className="w-full" onRun={handleRunQuery} />
                {/* Row 2: Stats (left) + Navigation Mode (center) + Context Picker (right) */}
                <div className={cn('flex items-start justify-between', gapTokens.large)}>
                  <Pill size="md" className="items-stretch py-3" glow={queryState.isExecuting || transitionState.isTransitioning || isMetaHovered} glowColor={isMetaHovered ? 'novanet' : transitionState.isTransitioning ? 'novanet' : 'emerald'}>
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
                          onMetaHoverChange={setIsMetaHovered}
                          onMetaClick={() => !isExplosionActive && setIsExplosionActive(true)}
                        />
                        <Divider />
                        <RefreshButton onClick={handleRefresh} isLoading={isFetching} />
                        <Divider />
                        <LocaleFilterSelector />
                        <Divider />
                        <DisplayLimitSelector />
                      </div>
                      {/* Expanded breakdown - animates in/out */}
                      <ExpandedBreakdown
                        view={expandedView}
                        onMouseEnter={cancelCloseExpanded}
                        onMouseLeave={scheduleCloseExpanded}
                      />
                    </div>
                  </Pill>
                  <Pill size="md">
                    <ViewPicker />
                  </Pill>
                </div>
              </div>
            )}

            {/* Bottom left - Visual Encoding & Keyboard shortcuts & Macropad */}
            <div className={cn('absolute bottom-4 left-4 flex items-center', gapTokens.tight)}>
              {/* Visual Encoding Legend toggle */}
              <button
                onClick={toggleLegend}
                className={cn(
                  'px-3 py-2 rounded-xl bg-[#0d0d12] border border-white/10 hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-colors shadow-lg shadow-black/40 flex items-center',
                  legendOpen ? 'text-accent-blue border-accent-blue/30' : 'text-white/40 hover:text-accent-blue',
                  gapTokens.default
                )}
                title="Visual Encoding (L)"
              >
                <Layers className={iconSizes.md} />
                <kbd className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/10 border border-white/10">L</kbd>
              </button>
              {/* Keyboard shortcuts */}
              <button
                onClick={openShortcuts}
                className={cn('px-3 py-2 rounded-xl bg-[#0d0d12] border border-white/10 hover:bg-accent-blue/15 hover:border-accent-blue/30 transition-colors text-white/40 hover:text-accent-blue shadow-lg shadow-black/40 flex items-center', gapTokens.default)}
                title="Keyboard shortcuts (?)"
              >
                <HelpCircle className={iconSizes.md} />
                <kbd className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/10 border border-white/10">?</kbd>
              </button>
              {/* Macropad Configurator */}
              <button
                onClick={openMacropad}
                className={cn(
                  'px-3 py-2 rounded-xl bg-[#0d0d12] border border-white/10 hover:bg-novanet-500/15 hover:border-novanet-500/30 transition-colors shadow-lg shadow-black/40 flex items-center',
                  macropadOpen ? 'text-novanet-400 border-novanet-500/30' : 'text-white/40 hover:text-novanet-400',
                  gapTokens.default
                )}
                title="Macropad Configurator (P)"
              >
                <Gamepad2 className={iconSizes.md} />
                <kbd className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/10 border border-white/10">P</kbd>
              </button>
            </div>

            {/* Visual Encoding Legend Panel */}
            {legendOpen && (
              <Graph3DLegend
                className="!absolute !bottom-16 !left-4 !right-auto z-40"
                collapsed={false}
                onToggle={toggleLegend}
              />
            )}

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
              <ViewModeToggle
                mode={uiState.viewMode}
                onModeChange={uiActions.setViewMode}
              />
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
            {selectedEdge ? (
              /* Tabbed Arc Details Panel (has its own header) */
              <TabbedArcPanel arc={selectedEdge} className="h-full" />
            ) : selectedNode && isSchemaGroupNode(selectedNode) ? (
              /* Schema Group Panel (Realm or Layer container) */
              <div className="h-full flex flex-col">
                <div className="px-4 py-3 border-b border-white/8 flex items-center justify-between">
                  <div>
                    <h2 className="text-lg font-semibold text-white">{selectedNode.displayName}</h2>
                    <p className="text-sm text-white/50">{selectedNode.type === 'RealmGroup' ? 'Realm' : 'Layer'}</p>
                  </div>
                  <button
                    onClick={() => uiActions.setSelectedNode(null)}
                    className="p-1.5 rounded-md hover:bg-white/10 text-white/50 hover:text-white transition-colors"
                  >
                    <X size={18} />
                  </button>
                </div>
                <div className="flex-1 p-4 overflow-auto">
                  <div className="space-y-4">
                    <div className="p-3 rounded-lg bg-white/5 border border-white/10">
                      <p className="text-sm text-white/70">
                        {selectedNode.type === 'RealmGroup'
                          ? `The ${selectedNode.key} realm contains node types for ${selectedNode.key === 'shared' ? 'universal, read-only knowledge' : 'organization-specific content'}.`
                          : `The ${selectedNode.key} layer groups related node types.`}
                      </p>
                    </div>
                    <p className="text-xs text-white/40">
                      Click on a node type within this {selectedNode.type === 'RealmGroup' ? 'realm' : 'layer'} to see detailed information.
                    </p>
                  </div>
                </div>
              </div>
            ) : selectedNode ? (
              /* Tabbed Node Details Panel (has its own header) */
              <TabbedDetailPanel node={selectedNode} className="h-full" />
            ) : null}
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

      {/* Macropad Configurator (P) */}
      <MacropadVisualizer />

      {/* Easter Egg - Matrix Explosion Effect */}
      <MatrixExplosionOverlay
        isActive={isExplosionActive}
        onComplete={() => setIsExplosionActive(false)}
      />
    </div>
  );
}
