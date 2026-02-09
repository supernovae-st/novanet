import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { ViewMode, UIState, SelectionState } from '@/types';
import { type SpacingPreset, DEFAULT_SPACING_PRESET } from '@/lib/forceSimulation';

// Layout types for graph arrangement
export type LayoutDirection = 'TB' | 'LR' | 'dagre' | 'radial' | 'force';

// Layout mode: containers (hardcoded groups) vs magnetic (Neo4j-driven attractors)
export type LayoutMode = 'containers' | 'magnetic';

// Navigation mode: how the user explores the graph
// v11.0: Simplified to Meta (schema) and Data only
export type NavigationMode = 'data' | 'meta';

// Modal types - only one can be open at a time
export type ModalType = 'command-palette' | 'keyboard-shortcuts' | 'ai-chat' | 'cypher-editor' | 'locale-picker' | 'project-picker' | null;

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

  // Navigation mode: how the user explores the graph
  navigationMode: NavigationMode;

  // View actions
  setViewMode: (mode: ViewMode) => void;
  toggleViewMode: () => void;
  setNavigationMode: (mode: NavigationMode) => void;
  cycleNavigationMode: () => void;
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

  // Selection actions
  setSelectedNode: (id: string | null) => void;
  setSelectedEdge: (id: string | null, edgeData?: SelectedEdgeData) => void;
  setHoveredNode: (id: string | null) => void;
  setHoveredEdge: (id: string | null) => void;
  setHoveredConnections: (ids: Set<string>) => void;
  clearSelection: () => void;

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

/** Selector for navigationMode - use with useUIStore(selectNavigationMode) */
export const selectNavigationMode = (state: UIStoreState) => state.navigationMode;

/** Selector for layoutDirection - use with useUIStore(selectLayoutDirection) */
export const selectLayoutDirection = (state: UIStoreState) => state.layoutDirection;

/** Selector for layoutVersion - use with useUIStore(selectLayoutVersion) */
export const selectLayoutVersion = (state: UIStoreState) => state.layoutVersion;

/** Selector for layoutMode - use with useUIStore(selectLayoutMode) */
export const selectLayoutMode = (state: UIStoreState) => state.layoutMode;

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
      searchOpen: false,
      focusMode: false,
      minimapVisible: true,
      showEdgeLabels: true,
      layoutDirection: 'TB' as LayoutDirection,
      layoutVersion: 0,
      layoutMode: 'containers' as LayoutMode,
      spacingPreset: DEFAULT_SPACING_PRESET,
      spacingValue: 100, // 0=compact, 50=normal, 100=spacious
      spacingVersion: 0,
      navigationMode: 'data' as NavigationMode,

      // Selection state
      selectedNodeId: null,
      selectedEdgeId: null,
      selectedEdgeData: null,
      hoveredNodeId: null,
      hoveredEdgeId: null,
      hoveredConnectedNodeIds: new Set(),
      highlightedNodeIds: new Set(),

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

      setNavigationMode: (mode) => {
        set((state) => {
          state.navigationMode = mode;
        });
      },

      cycleNavigationMode: () => {
        set((state) => {
          // v11.0: Simplified to Meta and Data only
          const modes: NavigationMode[] = ['meta', 'data'];
          const idx = modes.indexOf(state.navigationMode);
          state.navigationMode = modes[(idx + 1) % modes.length];
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
        navigationMode: state.navigationMode,
      }),
      version: 10,
      migrate: (persistedState: unknown, version: number) => {
        if (version < 9) {
          // v9: clear stale v8 state, reset to defaults
          return {
            viewMode: '2d',
            sidebarOpen: true,
            minimapVisible: true,
            showEdgeLabels: true,
            layoutDirection: 'TB',
            layoutMode: 'containers',
            spacingPreset: DEFAULT_SPACING_PRESET,
            spacingValue: 100,
            navigationMode: 'data',
          };
        }
        if (version < 10) {
          // v10: DataMode -> NavigationMode, 'schema' -> 'meta'
          const prev = persistedState as Record<string, unknown>;
          const oldMode = prev.dataMode ?? prev.navigationMode ?? 'data';
          return {
            ...prev,
            dataMode: undefined,
            navigationMode: oldMode === 'schema' ? 'meta' : oldMode,
          };
        }
        return persistedState as UIStoreState;
      },
    }
  )
);

