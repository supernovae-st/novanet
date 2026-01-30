import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { NodeType, FilterPreset } from '@/types';
import { CORE_TYPES, ALL_NODE_TYPES } from '@/config/nodeTypes';
import { DEFAULT_PRESET } from '@/config/presets';
import type { Priority, Freshness, NodeCategory } from '@/lib/filterAdapter';
import { NovaNetFilter, NODE_CATEGORIES, VIEW_PRESETS, getViewPresetByShortcut } from '@/lib/filterAdapter';
import { logger } from '@/lib/logger';

// Valid node types (v7.2.1) - used to filter out legacy types from localStorage
const validNodeTypes = new Set<string>(ALL_NODE_TYPES);

// Extended filter state (aligned with novanet-core v7.2.1)
interface ExtendedFilterState {
  // Core filters
  enabledNodeTypes: Set<NodeType>;
  selectedProject: string | null;  // Project filter (cascades to locale)
  selectedLocale: string | null;
  searchQuery: string;
  depthLimit: number;
  activePresetId: string | null;

  // v7.2.1 additions
  priorityFilter: Priority[];
  freshnessFilter: Freshness[];
  categoryFilter: NodeCategory[];
  activeOnly: boolean;
  localeFamily: string | null;
}

interface FilterStoreState extends ExtendedFilterState {
  // Actions
  setEnabledNodeTypes: (types: NodeType[]) => void;
  toggleNodeType: (type: NodeType) => void;
  setSelectedProject: (project: string | null) => void;
  setSelectedLocale: (locale: string | null) => void;
  setSearchQuery: (query: string) => void;
  setDepthLimit: (depth: number) => void;

  // v7.2.1 filter actions
  setPriorityFilter: (priorities: Priority[]) => void;
  setFreshnessFilter: (freshness: Freshness[]) => void;
  setCategoryFilter: (categories: NodeCategory[]) => void;
  toggleCategory: (category: NodeCategory) => void;
  setActiveOnly: (active: boolean) => void;
  setLocaleFamily: (family: string | null) => void;

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

      // v7.2.1 initial state
      priorityFilter: [],
      freshnessFilter: [],
      categoryFilter: [],
      activeOnly: false,
      localeFamily: null,

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

      // v7.2.1 filter actions
      setPriorityFilter: (priorities) => {
        set((state) => {
          state.priorityFilter = priorities;
          state.activePresetId = null;
        });
      },

      setFreshnessFilter: (freshness) => {
        set((state) => {
          state.freshnessFilter = freshness;
          state.activePresetId = null;
        });
      },

      setCategoryFilter: (categories) => {
        set((state) => {
          state.categoryFilter = categories;
          // Update enabled node types based on categories
          const types = new Set<NodeType>();
          for (const category of categories) {
            NODE_CATEGORIES[category].forEach(t => types.add(t));
          }
          if (types.size > 0) {
            state.enabledNodeTypes = types;
          }
          state.activePresetId = null;
        });
      },

      toggleCategory: (category) => {
        set((state) => {
          const idx = state.categoryFilter.indexOf(category);
          if (idx >= 0) {
            state.categoryFilter.splice(idx, 1);
          } else {
            state.categoryFilter.push(category);
          }
          // Update enabled node types
          const types = new Set<NodeType>();
          for (const cat of state.categoryFilter) {
            NODE_CATEGORIES[cat].forEach(t => types.add(t));
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
            if (criteria.filters.priority) {
              state.priorityFilter = criteria.filters.priority;
            }
            if (criteria.filters.freshness) {
              state.freshnessFilter = criteria.filters.freshness;
            }
            if (criteria.filters.categories) {
              state.categoryFilter = criteria.filters.categories;
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
          // Reset v7.2.1 filters
          state.priorityFilter = [];
          state.freshnessFilter = [];
          state.categoryFilter = [];
          state.activeOnly = false;
          state.localeFamily = null;
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

        // Apply categories
        if (state.categoryFilter.length > 0) {
          filter = filter.byCategory(...state.categoryFilter);
        }

        // Apply locale
        if (state.selectedLocale) {
          filter = filter.forLocale(state.selectedLocale);
        }
        if (state.localeFamily) {
          filter = filter.forLocaleFamily(state.localeFamily);
        }

        // Apply priority/freshness
        if (state.priorityFilter.length > 0) {
          filter = filter.withPriority(...state.priorityFilter);
        }
        if (state.freshnessFilter.length > 0) {
          filter = filter.withFreshness(...state.freshnessFilter);
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
          state.customPresets = state.customPresets.filter((p) => p.id !== id);
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
        // v7.2.1 additions
        priorityFilter: state.priorityFilter,
        freshnessFilter: state.freshnessFilter,
        categoryFilter: state.categoryFilter,
        activeOnly: state.activeOnly,
        localeFamily: state.localeFamily,
      }) as FilterStoreState,
    }
  )
);

// =============================================================================
// SELECTORS - Use these for optimal re-render performance
// =============================================================================

/** Select enabled node types */
export const selectEnabledNodeTypes = (state: FilterStoreState) => state.enabledNodeTypes;

/** Select selected project */
export const selectSelectedProject = (state: FilterStoreState) => state.selectedProject;

/** Select selected locale */
export const selectSelectedLocale = (state: FilterStoreState) => state.selectedLocale;

/** Select search query */
export const selectSearchQuery = (state: FilterStoreState) => state.searchQuery;

/** Select active preset ID */
export const selectActivePresetId = (state: FilterStoreState) => state.activePresetId;

/** Select depth limit */
export const selectDepthLimit = (state: FilterStoreState) => state.depthLimit;

/** Select priority filter */
export const selectPriorityFilter = (state: FilterStoreState) => state.priorityFilter;

/** Select freshness filter */
export const selectFreshnessFilter = (state: FilterStoreState) => state.freshnessFilter;

/** Select category filter */
export const selectCategoryFilter = (state: FilterStoreState) => state.categoryFilter;
