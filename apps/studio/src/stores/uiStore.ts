import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { ViewMode, UIState, SelectionState } from '@/types';
import {
  type SpacingPreset,
  DEFAULT_SPACING_PRESET,
  SPACING_PRESETS,
} from '@/lib/forceSimulation';

// Layout types for graph arrangement
export type LayoutDirection = 'TB' | 'LR' | 'dagre' | 'radial' | 'force';

// Data mode: real instances vs ontological schema
export type DataMode = 'data' | 'schema';

// Modal types - only one can be open at a time
export type ModalType = 'command-palette' | 'keyboard-shortcuts' | 'ai-chat' | 'cypher-editor' | 'locale-picker' | 'project-picker' | null;

interface UIStoreState extends UIState, SelectionState {
  // Dialog state (legacy)
  activeDialogs: string[];
  commandPaletteOpen: boolean;
  aiChatOpen: boolean;
  minimapVisible: boolean;

  // Modal state - exclusive (only one at a time)
  activeModal: ModalType;

  // Edge selection
  selectedEdgeId: string | null;

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

  // Data mode: real instances vs ontological schema
  dataMode: DataMode;

  // View actions
  setViewMode: (mode: ViewMode) => void;
  toggleViewMode: () => void;
  setDataMode: (mode: DataMode) => void;
  toggleDataMode: () => void;
  toggleSidebar: () => void;
  togglePanel: () => void;
  toggleFocusMode: () => void;
  toggleMinimap: () => void;
  toggleEdgeLabels: () => void;
  setLayoutDirection: (direction: LayoutDirection) => void;
  /** Trigger layout recalculation (always runs, even if same direction) */
  triggerLayout: (direction?: LayoutDirection) => void;

  // Spacing actions
  setSpacingPreset: (preset: SpacingPreset) => void;
  setSpacingValue: (value: number) => void;
  /** Cycle through spacing presets: compact → normal → spacious */
  cycleSpacingPreset: () => void;
  /** Get interpolated spacing options based on current value */
  getSpacingOptions: () => { chargeStrength: number; linkDistance: number; collisionRadius: number };

  // Selection actions
  setSelectedNode: (id: string | null) => void;
  setSelectedEdge: (id: string | null) => void;
  setHoveredNode: (id: string | null) => void;
  setHoveredEdge: (id: string | null) => void;
  setHoveredConnections: (ids: Set<string>) => void;
  setHighlightedNodes: (ids: string[]) => void;
  clearSelection: () => void;

  // Dialog actions (legacy)
  openDialog: (id: string) => void;
  closeDialog: (id: string) => void;
  closeTopDialog: () => void;
  isDialogOpen: (id: string) => boolean;
  setCommandPaletteOpen: (open: boolean) => void;
  setAiChatOpen: (open: boolean) => void;

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

/** Selector for hoveredConnectedNodeIds - use with useUIStore(selectHoveredConnectedNodeIds) */
export const selectHoveredConnectedNodeIds = (state: UIStoreState) => state.hoveredConnectedNodeIds;

/** Selector for dataMode - use with useUIStore(selectDataMode) */
export const selectDataMode = (state: UIStoreState) => state.dataMode;

/** Selector for layoutDirection - use with useUIStore(selectLayoutDirection) */
export const selectLayoutDirection = (state: UIStoreState) => state.layoutDirection;

/** Selector for layoutVersion - use with useUIStore(selectLayoutVersion) */
export const selectLayoutVersion = (state: UIStoreState) => state.layoutVersion;

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
      spacingPreset: DEFAULT_SPACING_PRESET,
      spacingValue: 100, // 0=compact, 50=normal, 100=spacious
      spacingVersion: 0,
      dataMode: 'data' as DataMode,

      // Selection state
      selectedNodeId: null,
      selectedEdgeId: null,
      hoveredNodeId: null,
      hoveredEdgeId: null,
      hoveredConnectedNodeIds: new Set(),
      highlightedNodeIds: new Set(),

      // Dialog state (legacy)
      activeDialogs: [],
      commandPaletteOpen: false,
      aiChatOpen: false,

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

      setDataMode: (mode) => {
        set((state) => {
          state.dataMode = mode;
        });
      },

      toggleDataMode: () => {
        set((state) => {
          state.dataMode = state.dataMode === 'data' ? 'schema' : 'data';
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

      cycleSpacingPreset: () => {
        set((state) => {
          const presets: SpacingPreset[] = ['compact', 'normal', 'spacious'];
          const currentIndex = presets.indexOf(state.spacingPreset);
          const nextIndex = (currentIndex + 1) % presets.length;
          state.spacingPreset = presets[nextIndex];
          const presetToValue: Record<SpacingPreset, number> = {
            compact: 0,
            normal: 50,
            spacious: 100,
          };
          state.spacingValue = presetToValue[state.spacingPreset];
          state.spacingVersion += 1;
        });
      },

      getSpacingOptions: () => {
        const { spacingValue } = get();
        // Interpolate between compact and spacious based on value (0-100)
        const t = spacingValue / 100; // 0 to 1
        const compact = SPACING_PRESETS.compact;
        const spacious = SPACING_PRESETS.spacious;

        return {
          chargeStrength: compact.chargeStrength + (spacious.chargeStrength - compact.chargeStrength) * t,
          linkDistance: compact.linkDistance + (spacious.linkDistance - compact.linkDistance) * t,
          collisionRadius: compact.collisionRadius + (spacious.collisionRadius - compact.collisionRadius) * t,
        };
      },

      // Selection actions
      setSelectedNode: (id) => {
        set((state) => {
          state.selectedNodeId = id;
          // Clear edge selection when selecting a node
          state.selectedEdgeId = null;
          // Auto-open panel when node selected
          if (id && !state.focusMode) {
            state.panelOpen = true;
          }
        });
      },

      setSelectedEdge: (id) => {
        set((state) => {
          state.selectedEdgeId = id;
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

      setHighlightedNodes: (ids) => {
        set((state) => {
          state.highlightedNodeIds = new Set(ids);
        });
      },

      clearSelection: () => {
        set((state) => {
          state.selectedNodeId = null;
          state.selectedEdgeId = null;
          state.hoveredNodeId = null;
          state.hoveredEdgeId = null;
          state.highlightedNodeIds = new Set();
        });
      },

      // Dialog actions
      openDialog: (id) => {
        set((state) => {
          if (!state.activeDialogs.includes(id)) {
            state.activeDialogs.push(id);
          }
        });
      },

      closeDialog: (id) => {
        set((state) => {
          state.activeDialogs = state.activeDialogs.filter((d) => d !== id);
        });
      },

      closeTopDialog: () => {
        set((state) => {
          state.activeDialogs.pop();
        });
      },

      isDialogOpen: (id) => {
        return get().activeDialogs.includes(id);
      },

      setCommandPaletteOpen: (open) => {
        set((state) => {
          state.commandPaletteOpen = open;
        });
      },

      setAiChatOpen: (open) => {
        set((state) => {
          state.aiChatOpen = open;
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
        spacingPreset: state.spacingPreset,
        spacingValue: state.spacingValue,
        dataMode: state.dataMode,
      }),
    }
  )
);

