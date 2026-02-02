import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { Realm, Layer, Trait, NodeType } from '@novanet/core/types';
import type { FilterPreset } from '@/types';
import { CORE_TYPES, ALL_NODE_TYPES, NODE_LAYERS } from '@/config/nodeTypes';
import { DEFAULT_PRESET } from '@/config/presets';
import { NovaNetFilter, VIEW_PRESETS, getViewPresetByShortcut } from '@/lib/filterAdapter';
import { logger } from '@/lib/logger';

// Valid node types (v7.2.1) - used to filter out legacy types from localStorage
const validNodeTypes = new Set<string>(ALL_NODE_TYPES);

// Extended filter state (aligned with v9.0.0)
interface ExtendedFilterState {
  // Core filters
  enabledNodeTypes: Set<NodeType>;
  selectedProject: string | null;  // Project filter (cascades to locale)
  selectedLocale: string | null;
  searchQuery: string;
  depthLimit: number;
  activePresetId: string | null;

  // Faceted filters (v9)
  layerFilter: Layer[];
  activeOnly: boolean;
  localeFamily: string | null;

  // Navigation query facets (v9 Phase 6)
  realmFilter: Realm[];
  traitFilter: Trait[];
  edgeFamilyFilter: string[];

  // Schema mode collapsed groups (Task 3.1)
  collapsedRealms: Realm[];
  collapsedLayers: string[]; // Format: "Realm-layer"
}

interface FilterStoreState extends ExtendedFilterState {
  // Actions
  setEnabledNodeTypes: (types: NodeType[]) => void;
  toggleNodeType: (type: NodeType) => void;
  setSelectedProject: (project: string | null) => void;
  setSelectedLocale: (locale: string | null) => void;
  setSearchQuery: (query: string) => void;
  setDepthLimit: (depth: number) => void;

  // Faceted filter actions (v9)
  setLayerFilter: (layers: Layer[]) => void;
  toggleLayer: (layer: Layer) => void;
  setActiveOnly: (active: boolean) => void;
  setLocaleFamily: (family: string | null) => void;

  // Navigation query facet actions (v9 Phase 6)
  setRealmFilter: (realms: Realm[]) => void;
  toggleRealm: (realm: Realm) => void;
  setTraitFilter: (traits: Trait[]) => void;
  toggleTrait: (trait: Trait) => void;
  setEdgeFamilyFilter: (families: string[]) => void;
  toggleEdgeFamily: (family: string) => void;

  // Preset actions
  applyViewPreset: (presetId: string) => void;
  applyViewPresetByShortcut: (shortcut: string) => void;
  clearFilters: () => void;

  // NovaNetFilter builder
  toNovaNetFilter: () => NovaNetFilter;

  // Generate Cypher query from current filter state
  toCypher: () => string;

  // Custom presets
  customPresets: FilterPreset[];
  addCustomPreset: (preset: FilterPreset) => void;
  removeCustomPreset: (id: string) => void;

  // Schema mode collapsed groups actions (Task 3.1)
  toggleRealmCollapsed: (realm: Realm) => void;
  toggleLayerCollapsed: (realm: Realm, layer: string) => void;
  setLayerCollapsed: (realm: Realm, layer: string, collapsed: boolean) => void;
  isRealmCollapsed: (realm: Realm) => boolean;
  isLayerCollapsed: (realm: Realm, layer: string) => boolean;
  resetSchemaFilters: () => void;
}

export const useFilterStore = create<FilterStoreState>()(
  persist(
    immer((set, get) => ({
      // Initial state
      enabledNodeTypes: new Set(CORE_TYPES),
      selectedProject: null,
      selectedLocale: null,
      searchQuery: '',
      depthLimit: 2,
      activePresetId: DEFAULT_PRESET.id,
      customPresets: [],

      // Faceted filters (v9)
      layerFilter: [],
      activeOnly: false,
      localeFamily: null,

      // Navigation query facets (v9 Phase 6)
      realmFilter: [],
      traitFilter: [],
      edgeFamilyFilter: [],

      // Schema mode collapsed groups initial state (Task 3.1)
      collapsedRealms: [],
      collapsedLayers: [],

      // Actions
      setEnabledNodeTypes: (types) => {
        set((state) => {
          state.enabledNodeTypes = new Set(types);
          state.activePresetId = null; // Clear preset when manually changed
        });
      },

      toggleNodeType: (type) => {
        set((state) => {
          // Direct mutation with immer (cleaner pattern)
          if (state.enabledNodeTypes.has(type)) {
            state.enabledNodeTypes.delete(type);
          } else {
            state.enabledNodeTypes.add(type);
          }
          state.activePresetId = null;
        });
      },

      setSelectedProject: (project) => {
        set((state) => {
          state.selectedProject = project;
          // Cascade: clear locale when project is deselected
          if (!project) {
            state.selectedLocale = null;
          }
        });
      },

      setSelectedLocale: (locale) => {
        set((state) => {
          state.selectedLocale = locale;
        });
      },

      setSearchQuery: (query) => {
        set((state) => {
          state.searchQuery = query;
        });
      },

      setDepthLimit: (depth) => {
        set((state) => {
          state.depthLimit = depth;
        });
      },

      setLayerFilter: (layers) => {
        set((state) => {
          state.layerFilter = layers;
          // Update enabled node types based on layers
          const types = new Set<NodeType>();
          for (const layer of layers) {
            const layerTypes = NODE_LAYERS[layer];
            if (layerTypes) layerTypes.forEach(t => types.add(t));
          }
          if (types.size > 0) {
            state.enabledNodeTypes = types;
          }
          state.activePresetId = null;
        });
      },

      toggleLayer: (layer) => {
        set((state) => {
          const idx = state.layerFilter.indexOf(layer);
          if (idx >= 0) {
            state.layerFilter.splice(idx, 1);
          } else {
            state.layerFilter.push(layer);
          }
          // Update enabled node types
          const types = new Set<NodeType>();
          for (const l of state.layerFilter) {
            const layerTypes = NODE_LAYERS[l];
            if (layerTypes) layerTypes.forEach(t => types.add(t));
          }
          if (types.size > 0) {
            state.enabledNodeTypes = types;
          } else {
            state.enabledNodeTypes = new Set(CORE_TYPES);
          }
          state.activePresetId = null;
        });
      },

      setActiveOnly: (active) => {
        set((state) => {
          state.activeOnly = active;
        });
      },

      setLocaleFamily: (family) => {
        set((state) => {
          state.localeFamily = family;
        });
      },

      // Navigation query facet actions (v9 Phase 6)
      setRealmFilter: (realms) => {
        set((state) => {
          state.realmFilter = realms;
          state.activePresetId = null;
        });
      },

      toggleRealm: (realm) => {
        set((state) => {
          const idx = state.realmFilter.indexOf(realm);
          if (idx >= 0) {
            state.realmFilter.splice(idx, 1);
          } else {
            state.realmFilter.push(realm);
          }
          state.activePresetId = null;
        });
      },

      setTraitFilter: (traits) => {
        set((state) => {
          state.traitFilter = traits;
          state.activePresetId = null;
        });
      },

      toggleTrait: (trait) => {
        set((state) => {
          const idx = state.traitFilter.indexOf(trait);
          if (idx >= 0) {
            state.traitFilter.splice(idx, 1);
          } else {
            state.traitFilter.push(trait);
          }
          state.activePresetId = null;
        });
      },

      setEdgeFamilyFilter: (families) => {
        set((state) => {
          state.edgeFamilyFilter = families;
          state.activePresetId = null;
        });
      },

      toggleEdgeFamily: (family) => {
        set((state) => {
          const idx = state.edgeFamilyFilter.indexOf(family);
          if (idx >= 0) {
            state.edgeFamilyFilter.splice(idx, 1);
          } else {
            state.edgeFamilyFilter.push(family);
          }
          state.activePresetId = null;
        });
      },

      // Preset actions
      applyViewPreset: (presetId) => {
        const viewPreset = VIEW_PRESETS.find(p => p.id === presetId);
        if (viewPreset) {
          const filter = viewPreset.filter();
          const criteria = filter.getCriteria();
          set((state) => {
            // Apply node types from filter
            const types = filter.getResolvedNodeTypes();
            if (types.length > 0) {
              state.enabledNodeTypes = new Set(types);
            }
            // Apply other criteria
            if (criteria.filters.layers) {
              state.layerFilter = criteria.filters.layers;
            }
            state.activePresetId = presetId;
          });
        }
      },

      applyViewPresetByShortcut: (shortcut) => {
        const viewPreset = getViewPresetByShortcut(shortcut);
        if (viewPreset) {
          get().applyViewPreset(viewPreset.id);
        }
      },

      clearFilters: () => {
        set((state) => {
          state.enabledNodeTypes = new Set(CORE_TYPES);
          state.selectedProject = null;
          state.selectedLocale = null;
          state.searchQuery = '';
          state.depthLimit = 2;
          state.activePresetId = DEFAULT_PRESET.id;
          // Reset faceted filters (v9)
          state.layerFilter = [];
          state.activeOnly = false;
          state.localeFamily = null;
          // Reset navigation query facets (v9 Phase 6)
          state.realmFilter = [];
          state.traitFilter = [];
          state.edgeFamilyFilter = [];
        });
      },

      // NovaNetFilter builder - converts store state to NovaNetFilter
      toNovaNetFilter: () => {
        const state = get();
        let filter = NovaNetFilter.create();

        // Apply node types
        if (state.enabledNodeTypes.size > 0) {
          filter = filter.byTypes(...Array.from(state.enabledNodeTypes));
        }

        // Apply layers
        if (state.layerFilter.length > 0) {
          filter = filter.byLayer(...state.layerFilter);
        }

        // Apply locale
        if (state.selectedLocale) {
          filter = filter.forLocale(state.selectedLocale);
        }
        if (state.localeFamily) {
          filter = filter.forLocaleFamily(state.localeFamily);
        }

        // Apply search
        if (state.searchQuery) {
          filter = filter.search(state.searchQuery);
        }

        // Apply depth
        if (state.depthLimit) {
          filter = filter.maxDepth(state.depthLimit);
        }

        // Apply active-only
        if (state.activeOnly) {
          filter = filter.activeOnly();
        }

        return filter;
      },

      // Generate Cypher query from current filter state
      toCypher: () => {
        const filter = get().toNovaNetFilter();
        const cypherQuery = filter.toCypher();
        return cypherQuery.query;
      },

      // Custom presets
      addCustomPreset: (preset) => {
        set((state) => {
          state.customPresets.push({ ...preset, isCustom: true });
        });
      },

      removeCustomPreset: (id) => {
        set((state) => {
          state.customPresets = state.customPresets.filter((p: { id: string }) => p.id !== id);
        });
      },

      // Schema mode collapsed groups actions (Task 3.1)
      toggleRealmCollapsed: (realm) => {
        set((state) => {
          const idx = state.collapsedRealms.indexOf(realm);
          if (idx >= 0) {
            state.collapsedRealms.splice(idx, 1);
          } else {
            state.collapsedRealms.push(realm);
          }
        });
      },

      toggleLayerCollapsed: (realm, layer) => {
        set((state) => {
          const key = `${realm}-${layer}`;
          const idx = state.collapsedLayers.indexOf(key);
          if (idx >= 0) {
            state.collapsedLayers.splice(idx, 1);
          } else {
            state.collapsedLayers.push(key);
          }
        });
      },

      setLayerCollapsed: (realm, layer, collapsed) => {
        set((state) => {
          const key = `${realm}-${layer}`;
          const idx = state.collapsedLayers.indexOf(key);
          if (collapsed && idx < 0) {
            state.collapsedLayers.push(key);
          } else if (!collapsed && idx >= 0) {
            state.collapsedLayers.splice(idx, 1);
          }
        });
      },

      isRealmCollapsed: (realm) => {
        return get().collapsedRealms.includes(realm);
      },

      isLayerCollapsed: (realm, layer) => {
        const key = `${realm}-${layer}`;
        return get().collapsedLayers.includes(key);
      },

      resetSchemaFilters: () => {
        set((state) => {
          state.collapsedRealms = [];
          state.collapsedLayers = [];
        });
      },
    })),
    {
      name: 'novanet-filters',
      // Custom storage to properly handle Set serialization
      storage: {
        getItem: (name) => {
          const str = localStorage.getItem(name);
          if (!str) return null;
          try {
            const parsed = JSON.parse(str);
            // Filter out any invalid/legacy node types from stored data
            const storedTypes = parsed.state?.enabledNodeTypes || [];
            const validTypes = storedTypes.filter((t: string) => validNodeTypes.has(t));
            // If all stored types were invalid, use CORE_TYPES as default
            const typesToUse = validTypes.length > 0 ? validTypes : CORE_TYPES;
            return {
              ...parsed,
              state: {
                ...parsed.state,
                // Convert array back to Set with validated types
                enabledNodeTypes: new Set(typesToUse),
              },
            };
          } catch (error) {
            // Log error for debugging and clear corrupted localStorage
            logger.warn('FilterStore', 'Failed to parse localStorage, resetting to defaults', error);
            localStorage.removeItem(name);
            return null;
          }
        },
        setItem: (name, value) => {
          const toStore = {
            ...value,
            state: {
              ...value.state,
              // Convert Set to array for JSON serialization
              enabledNodeTypes: Array.from(value.state.enabledNodeTypes),
            },
          };
          localStorage.setItem(name, JSON.stringify(toStore));
        },
        removeItem: (name) => localStorage.removeItem(name),
      },
      partialize: (state) => ({
        enabledNodeTypes: state.enabledNodeTypes,
        selectedProject: state.selectedProject,
        selectedLocale: state.selectedLocale,
        depthLimit: state.depthLimit,
        activePresetId: state.activePresetId,
        customPresets: state.customPresets,
        // Faceted filters (v9)
        layerFilter: state.layerFilter,
        activeOnly: state.activeOnly,
        localeFamily: state.localeFamily,
        // Navigation query facets (v9 Phase 6)
        realmFilter: state.realmFilter,
        traitFilter: state.traitFilter,
        edgeFamilyFilter: state.edgeFamilyFilter,
        // Schema mode collapsed groups (Task 3.1)
        collapsedRealms: state.collapsedRealms,
        collapsedLayers: state.collapsedLayers,
      }) as FilterStoreState,
      version: 9,
      migrate: (persistedState: unknown, version: number) => {
        if (version < 9) {
          // v9: clear stale v8 category-based filters, reset to defaults
          return {
            enabledNodeTypes: new Set(CORE_TYPES),
            selectedProject: null,
            selectedLocale: null,
            depthLimit: 2,
            activePresetId: DEFAULT_PRESET.id,
            customPresets: [],
            layerFilter: [],
            activeOnly: false,
            localeFamily: null,
            realmFilter: [],
            traitFilter: [],
            edgeFamilyFilter: [],
            collapsedRealms: [],
            collapsedLayers: [],
          };
        }
        return persistedState as FilterStoreState;
      },
    }
  )
);

