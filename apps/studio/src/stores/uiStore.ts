import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { ViewMode, UIState, SelectionState } from '@/types';
import { type SpacingPreset, DEFAULT_SPACING_PRESET } from '@/lib/forceSimulation';

// Layout types for graph arrangement
export type LayoutDirection = 'TB' | 'LR' | 'dagre' | 'radial' | 'force';

// Layout mode: containers (hardcoded groups) vs magnetic (Neo4j-driven attractors)
export type LayoutMode = 'containers' | 'magnetic';

// Bloom quality levels for 3D graph post-processing
// 'auto' = detect device capability and adapt
export type BloomQuality = 'off' | 'low' | 'medium' | 'high' | 'auto';

// Modal types - only one can be open at a time
export type ModalType = 'command-palette' | 'keyboard-shortcuts' | 'ai-chat' | 'cypher-editor' | 'locale-picker' | 'project-picker' | 'macropad-configurator' | null;

// Detail panel tab types
export type DetailPanelTab = 'overview' | 'data' | 'graph' | 'code';

// Selected edge data - unified format for both data and schema modes
export interface SelectedEdgeData {
  id: string;
  type: string; // Relation type (HAS_PAGE, etc.)
  source: string;
  target: string;
  data?: Record<string, unknown>;
}

interface UIStoreState extends UIState, SelectionState {
  // Minimap visibility
  minimapVisible: boolean;

  // Layout mode: containers (hardcoded) vs magnetic (Neo4j-driven)
  layoutMode: LayoutMode;

  // Modal state - exclusive (only one at a time)
  activeModal: ModalType;

  // Edge selection
  selectedEdgeId: string | null;
  /** Edge data for display - stored directly to support both data and schema modes */
  selectedEdgeData: SelectedEdgeData | null;

  // Edge hover state
  hoveredEdgeId: string | null;

  // Hover connections - node IDs connected to hovered node (for direct subscription in node components)
  hoveredConnectedNodeIds: Set<string>;

  // Detail panel tab selection
  detailPanelTab: DetailPanelTab;

  // Edge labels visibility
  showEdgeLabels: boolean;

  // Layout direction for graph arrangement
  layoutDirection: LayoutDirection;
  /** Counter to force re-layout even when direction unchanged */
  layoutVersion: number;

  // Spacing preset for force simulation
  spacingPreset: SpacingPreset;
  /** Custom spacing value (0-100, maps to interpolation between compact-spacious) */
  spacingValue: number;
  /** Counter to force re-layout when spacing changes */
  spacingVersion: number;

  // 3D graph bloom effect settings
  /** Bloom quality level (off/low/medium/high/auto) */
  bloomQuality: BloomQuality;
  /** Whether bloom is enabled (derived from bloomQuality !== 'off') */
  bloomEnabled: boolean;

  // View actions
  setViewMode: (mode: ViewMode) => void;
  toggleViewMode: () => void;
  toggleSidebar: () => void;
  togglePanel: () => void;
  toggleFocusMode: () => void;
  toggleMinimap: () => void;
  toggleEdgeLabels: () => void;
  setLayoutDirection: (direction: LayoutDirection) => void;
  /** Trigger layout recalculation (always runs, even if same direction) */
  triggerLayout: (direction?: LayoutDirection) => void;
  setLayoutMode: (mode: LayoutMode) => void;
  toggleLayoutMode: () => void;

  // Spacing actions
  setSpacingPreset: (preset: SpacingPreset) => void;
  setSpacingValue: (value: number) => void;

  // Bloom actions
  setBloomQuality: (quality: BloomQuality) => void;
  toggleBloom: () => void;

  // Selection actions
  setSelectedNode: (id: string | null) => void;
  setSelectedEdge: (id: string | null, edgeData?: SelectedEdgeData) => void;
  setHoveredNode: (id: string | null) => void;
  setHoveredEdge: (id: string | null) => void;
  setHoveredConnections: (ids: Set<string>) => void;
  clearSelection: () => void;

  // Detail panel tab action
  setDetailPanelTab: (tab: DetailPanelTab) => void;

  // Modal actions - exclusive (closes any open modal before opening new one)
  openModal: (modal: ModalType) => void;
  closeModal: () => void;
  isModalOpen: (modal: ModalType) => boolean;
}

// =============================================================================
// Memoized Selectors (stable references for direct subscription)
// =============================================================================

/** Selector for hoveredNodeId - use with useUIStore(selectHoveredNodeId) */
export const selectHoveredNodeId = (state: UIStoreState) => state.hoveredNodeId;

/** Selector for hoveredEdgeId - use with useUIStore(selectHoveredEdgeId) */
export const selectHoveredEdgeId = (state: UIStoreState) => state.hoveredEdgeId;

/** Selector for selectedNodeId - use with useUIStore(selectSelectedNodeId) */
export const selectSelectedNodeId = (state: UIStoreState) => state.selectedNodeId;

/** Selector for selectedEdgeId - use with useUIStore(selectSelectedEdgeId) */
export const selectSelectedEdgeId = (state: UIStoreState) => state.selectedEdgeId;

/** Selector for selectedEdgeData - use with useUIStore(selectSelectedEdgeData) */
export const selectSelectedEdgeData = (state: UIStoreState) => state.selectedEdgeData;

/** Selector for hoveredConnectedNodeIds - use with useUIStore(selectHoveredConnectedNodeIds) */
export const selectHoveredConnectedNodeIds = (state: UIStoreState) => state.hoveredConnectedNodeIds;

/** Selector for layoutDirection - use with useUIStore(selectLayoutDirection) */
export const selectLayoutDirection = (state: UIStoreState) => state.layoutDirection;

/** Selector for layoutVersion - use with useUIStore(selectLayoutVersion) */
export const selectLayoutVersion = (state: UIStoreState) => state.layoutVersion;

/** Selector for layoutMode - use with useUIStore(selectLayoutMode) */
export const selectLayoutMode = (state: UIStoreState) => state.layoutMode;

/** Selector for detailPanelTab - use with useUIStore(selectDetailPanelTab) */
export const selectDetailPanelTab = (state: UIStoreState) => state.detailPanelTab;

/** Selector for bloomQuality - use with useUIStore(selectBloomQuality) */
export const selectBloomQuality = (state: UIStoreState) => state.bloomQuality;

/** Selector for bloomEnabled - use with useUIStore(selectBloomEnabled) */
export const selectBloomEnabled = (state: UIStoreState) => state.bloomEnabled;

// =============================================================================
// Composite Selectors (grouped state for fewer subscriptions)
// =============================================================================

/**
 * Graph3D selection state - combine into single subscription with shallow equality
 * Use: const { selectedNodeId, hoveredNodeId } = useUIStore(selectGraph3DState, shallow)
 */
export const selectGraph3DState = (state: UIStoreState) => ({
  selectedNodeId: state.selectedNodeId,
  hoveredNodeId: state.hoveredNodeId,
});

/**
 * Graph3D actions - stable reference (actions never change)
 * Use: const actions = useUIStore(selectGraph3DActions)
 */
export const selectGraph3DActions = (state: UIStoreState) => ({
  setSelectedNode: state.setSelectedNode,
  setHoveredNode: state.setHoveredNode,
  setSelectedEdge: state.setSelectedEdge,
  setHoveredEdge: state.setHoveredEdge,
});

/**
 * Layout state - combine direction, version, and mode
 * Use: const layout = useUIStore(selectLayoutState, shallow)
 */
export const selectLayoutState = (state: UIStoreState) => ({
  layoutDirection: state.layoutDirection,
  layoutVersion: state.layoutVersion,
  layoutMode: state.layoutMode,
});

/**
 * Selection state (nodes + edges) - for detail panels
 * Use: const selection = useUIStore(selectSelectionState, shallow)
 */
export const selectSelectionState = (state: UIStoreState) => ({
  selectedNodeId: state.selectedNodeId,
  selectedEdgeId: state.selectedEdgeId,
  selectedEdgeData: state.selectedEdgeData,
});

/**
 * Hover highlight state - for useHoverHighlight hook
 * Use: const hover = useUIStore(selectHoverState, shallow)
 */
export const selectHoverState = (state: UIStoreState) => ({
  hoveredNodeId: state.hoveredNodeId,
  hoveredEdgeId: state.hoveredEdgeId,
  selectedNodeId: state.selectedNodeId,
});

/**
 * Node selection actions - for useNodeSelection hook
 * Use: const actions = useUIStore(selectNodeSelectionActions)
 */
export const selectNodeSelectionActions = (state: UIStoreState) => ({
  setSelectedNode: state.setSelectedNode,
  setDetailPanelTab: state.setDetailPanelTab,
  clearSelection: state.clearSelection,
});

// =============================================================================
// Store Implementation
// =============================================================================

export const useUIStore = create<UIStoreState>()(
  persist(
    immer((set, get) => ({
      // Initial state
      viewMode: '2d',
      sidebarOpen: true,
      panelOpen: false,
      focusMode: false,
      minimapVisible: true,
      showEdgeLabels: true,
      layoutDirection: 'TB' as LayoutDirection,
      layoutVersion: 0,
      layoutMode: 'containers' as LayoutMode,
      spacingPreset: DEFAULT_SPACING_PRESET,
      spacingValue: 100, // 0=compact, 50=normal, 100=spacious
      spacingVersion: 0,

      // Bloom settings (default to auto-detect)
      bloomQuality: 'auto' as BloomQuality,
      bloomEnabled: true, // Will be updated based on auto-detection

      // Selection state
      selectedNodeId: null,
      selectedEdgeId: null,
      selectedEdgeData: null,
      hoveredNodeId: null,
      hoveredEdgeId: null,
      hoveredConnectedNodeIds: new Set(),
      highlightedNodeIds: new Set(),

      // Detail panel tab
      detailPanelTab: 'overview' as DetailPanelTab,

      // Modal state - exclusive
      activeModal: null as ModalType,

      // View actions
      setViewMode: (mode) => {
        set((state) => {
          state.viewMode = mode;
        });
      },

      toggleViewMode: () => {
        set((state) => {
          state.viewMode = state.viewMode === '2d' ? '3d' : '2d';
        });
      },

      toggleSidebar: () => {
        set((state) => {
          state.sidebarOpen = !state.sidebarOpen;
        });
      },

      togglePanel: () => {
        set((state) => {
          state.panelOpen = !state.panelOpen;
        });
      },

      toggleFocusMode: () => {
        set((state) => {
          state.focusMode = !state.focusMode;
          if (state.focusMode) {
            // Hide both sidebars in focus mode
            state.sidebarOpen = false;
            state.panelOpen = false;
          }
        });
      },

      toggleMinimap: () => {
        set((state) => {
          state.minimapVisible = !state.minimapVisible;
        });
      },

      toggleEdgeLabels: () => {
        set((state) => {
          state.showEdgeLabels = !state.showEdgeLabels;
        });
      },

      setLayoutDirection: (direction) => {
        set((state) => {
          state.layoutDirection = direction;
        });
      },

      triggerLayout: (direction) => {
        set((state) => {
          if (direction) {
            state.layoutDirection = direction;
          }
          // Always increment to force re-layout even if same direction
          state.layoutVersion += 1;
        });
      },

      setLayoutMode: (mode) => {
        set((state) => {
          state.layoutMode = mode;
        });
      },

      toggleLayoutMode: () => {
        set((state) => {
          state.layoutMode = state.layoutMode === 'containers' ? 'magnetic' : 'containers';
        });
      },

      // Spacing actions
      setSpacingPreset: (preset) => {
        set((state) => {
          state.spacingPreset = preset;
          // Map preset to value: compact=0, normal=50, spacious=100
          const presetToValue: Record<SpacingPreset, number> = {
            compact: 0,
            normal: 50,
            spacious: 100,
          };
          state.spacingValue = presetToValue[preset];
          state.spacingVersion += 1;
        });
      },

      setSpacingValue: (value) => {
        set((state) => {
          state.spacingValue = Math.max(0, Math.min(100, value));
          // Auto-update preset based on value ranges
          if (value <= 25) {
            state.spacingPreset = 'compact';
          } else if (value <= 75) {
            state.spacingPreset = 'normal';
          } else {
            state.spacingPreset = 'spacious';
          }
          state.spacingVersion += 1;
        });
      },

      // Bloom actions
      setBloomQuality: (quality) => {
        set((state) => {
          state.bloomQuality = quality;
          state.bloomEnabled = quality !== 'off';
        });
      },

      toggleBloom: () => {
        set((state) => {
          if (state.bloomEnabled) {
            state.bloomQuality = 'off';
            state.bloomEnabled = false;
          } else {
            state.bloomQuality = 'auto';
            state.bloomEnabled = true;
          }
        });
      },

      // Selection actions
      setSelectedNode: (id) => {
        set((state) => {
          state.selectedNodeId = id;
          // Clear edge selection when selecting a node
          state.selectedEdgeId = null;
          state.selectedEdgeData = null;
          // Auto-open panel when node selected
          if (id && !state.focusMode) {
            state.panelOpen = true;
          }
        });
      },

      setSelectedEdge: (id, edgeData) => {
        set((state) => {
          state.selectedEdgeId = id;
          // Store edge data for display (supports both data and schema modes)
          state.selectedEdgeData = edgeData ?? null;
          // Clear node selection when selecting an edge
          state.selectedNodeId = null;
          // Auto-open panel when edge selected
          if (id && !state.focusMode) {
            state.panelOpen = true;
          }
        });
      },

      setHoveredNode: (id) => {
        set((state) => {
          state.hoveredNodeId = id;
        });
      },

      setHoveredEdge: (id) => {
        set((state) => {
          state.hoveredEdgeId = id;
        });
      },

      setHoveredConnections: (ids) => {
        set((state) => {
          state.hoveredConnectedNodeIds = ids;
        });
      },

      clearSelection: () => {
        set((state) => {
          state.selectedNodeId = null;
          state.selectedEdgeId = null;
          state.selectedEdgeData = null;
          state.hoveredNodeId = null;
          state.hoveredEdgeId = null;
          state.highlightedNodeIds = new Set();
        });
      },

      // Detail panel tab action
      setDetailPanelTab: (tab) => {
        set((state) => {
          state.detailPanelTab = tab;
        });
      },

      // Modal actions - exclusive (pure state updates, no DOM side effects)
      // Body scroll lock is handled by useModal/useBodyScrollLock hooks in components
      openModal: (modal) => {
        set((state) => {
          state.activeModal = modal;
        });
      },

      closeModal: () => {
        set((state) => {
          state.activeModal = null;
        });
      },

      isModalOpen: (modal) => {
        return get().activeModal === modal;
      },
    })),
    {
      name: 'novanet-ui',
      partialize: (state) => ({
        viewMode: state.viewMode,
        sidebarOpen: state.sidebarOpen,
        minimapVisible: state.minimapVisible,
        showEdgeLabels: state.showEdgeLabels,
        layoutDirection: state.layoutDirection,
        layoutMode: state.layoutMode,
        spacingPreset: state.spacingPreset,
        spacingValue: state.spacingValue,
        detailPanelTab: state.detailPanelTab,
        bloomQuality: state.bloomQuality,
      }),
      version: 13, // v13: Add bloom quality settings
      migrate: (persistedState: unknown, version: number) => {
        const prev = persistedState as Record<string, unknown>;

        if (version < 12) {
          // v12: Remove navigationMode, reset to clean state
          // Strip navigationMode and dataMode (legacy) from persisted state
          const { navigationMode, dataMode, ...rest } = prev;
          return {
            viewMode: (rest.viewMode as string) ?? '2d',
            sidebarOpen: (rest.sidebarOpen as boolean) ?? true,
            minimapVisible: (rest.minimapVisible as boolean) ?? true,
            showEdgeLabels: (rest.showEdgeLabels as boolean) ?? true,
            layoutDirection: (rest.layoutDirection as string) ?? 'TB',
            layoutMode: (rest.layoutMode as string) ?? 'containers',
            spacingPreset: (rest.spacingPreset as string) ?? DEFAULT_SPACING_PRESET,
            spacingValue: (rest.spacingValue as number) ?? 100,
            detailPanelTab: (rest.detailPanelTab as string) ?? 'overview',
            bloomQuality: 'auto',
          };
        }

        if (version < 13) {
          // v13: Add bloom quality (default to auto)
          return {
            ...prev,
            bloomQuality: 'auto',
          };
        }

        return persistedState as UIStoreState;
      },
    }
  )
);

