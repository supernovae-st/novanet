/**
 * Filter Store Tests
 *
 * Tests for the filter store with persistence and Set serialization.
 * Covers preset application, filter actions, and localStorage handling.
 */

import { useFilterStore } from '../filterStore';
import { CORE_TYPES } from '@/config/nodeTypes';
import { DEFAULT_PRESET } from '@/config/presets';
import type { NodeType } from '@/types';

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: jest.fn((key: string) => store[key] || null),
    setItem: jest.fn((key: string, value: string) => {
      store[key] = value;
    }),
    removeItem: jest.fn((key: string) => {
      delete store[key];
    }),
    clear: jest.fn(() => {
      store = {};
    }),
  };
})();

Object.defineProperty(window, 'localStorage', { value: localStorageMock });

describe('filterStore', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    localStorageMock.clear();

    // Reset store to initial state
    useFilterStore.setState({
      enabledNodeTypes: new Set(CORE_TYPES),
      selectedProject: null,
      selectedLocale: null,
      searchQuery: '',
      depthLimit: 2,
      activePresetId: DEFAULT_PRESET.id,
      customPresets: [],
      priorityFilter: [],
      freshnessFilter: [],
      categoryFilter: [],
      activeOnly: false,
      localeFamily: null,
    });
  });

  // ==========================================================================
  // Initial State
  // ==========================================================================

  describe('initial state', () => {
    it('should have CORE_TYPES enabled by default', () => {
      const state = useFilterStore.getState();
      expect(state.enabledNodeTypes).toEqual(new Set(CORE_TYPES));
    });

    it('should have default preset active', () => {
      const state = useFilterStore.getState();
      expect(state.activePresetId).toBe(DEFAULT_PRESET.id);
    });

    it('should have empty filters by default', () => {
      const state = useFilterStore.getState();
      expect(state.selectedProject).toBeNull();
      expect(state.selectedLocale).toBeNull();
      expect(state.searchQuery).toBe('');
      expect(state.priorityFilter).toEqual([]);
      expect(state.freshnessFilter).toEqual([]);
      expect(state.categoryFilter).toEqual([]);
    });
  });

  // ==========================================================================
  // Node Type Actions
  // ==========================================================================

  describe('setEnabledNodeTypes', () => {
    it('should set enabled node types', () => {
      const types: NodeType[] = ['Concept', 'Page', 'Block'];

      useFilterStore.getState().setEnabledNodeTypes(types);

      const state = useFilterStore.getState();
      expect(state.enabledNodeTypes).toEqual(new Set(types));
    });

    it('should clear active preset when manually changed', () => {
      useFilterStore.setState({ activePresetId: 'some-preset' });

      useFilterStore.getState().setEnabledNodeTypes(['Concept']);

      expect(useFilterStore.getState().activePresetId).toBeNull();
    });
  });

  describe('toggleNodeType', () => {
    it('should add node type if not present', () => {
      useFilterStore.setState({ enabledNodeTypes: new Set(['Concept']) });

      useFilterStore.getState().toggleNodeType('Page');

      const state = useFilterStore.getState();
      expect(state.enabledNodeTypes.has('Page')).toBe(true);
      expect(state.enabledNodeTypes.has('Concept')).toBe(true);
    });

    it('should remove node type if present', () => {
      useFilterStore.setState({ enabledNodeTypes: new Set(['Concept', 'Page']) });

      useFilterStore.getState().toggleNodeType('Concept');

      const state = useFilterStore.getState();
      expect(state.enabledNodeTypes.has('Concept')).toBe(false);
      expect(state.enabledNodeTypes.has('Page')).toBe(true);
    });

    it('should clear active preset when toggling', () => {
      useFilterStore.setState({ activePresetId: 'some-preset' });

      useFilterStore.getState().toggleNodeType('Page');

      expect(useFilterStore.getState().activePresetId).toBeNull();
    });
  });

  // ==========================================================================
  // Project/Locale Selection
  // ==========================================================================

  describe('setSelectedProject', () => {
    it('should set selected project', () => {
      useFilterStore.getState().setSelectedProject('project-1');

      expect(useFilterStore.getState().selectedProject).toBe('project-1');
    });

    it('should cascade: clear locale when project is deselected', () => {
      useFilterStore.setState({
        selectedProject: 'project-1',
        selectedLocale: 'en-US',
      });

      useFilterStore.getState().setSelectedProject(null);

      const state = useFilterStore.getState();
      expect(state.selectedProject).toBeNull();
      expect(state.selectedLocale).toBeNull();
    });
  });

  describe('setSelectedLocale', () => {
    it('should set selected locale', () => {
      useFilterStore.getState().setSelectedLocale('fr-FR');

      expect(useFilterStore.getState().selectedLocale).toBe('fr-FR');
    });
  });

  // ==========================================================================
  // Search and Depth
  // ==========================================================================

  describe('setSearchQuery', () => {
    it('should set search query', () => {
      useFilterStore.getState().setSearchQuery('test query');

      expect(useFilterStore.getState().searchQuery).toBe('test query');
    });
  });

  describe('setDepthLimit', () => {
    it('should set depth limit', () => {
      useFilterStore.getState().setDepthLimit(5);

      expect(useFilterStore.getState().depthLimit).toBe(5);
    });
  });

  // ==========================================================================
  // v7.2.1 Filter Actions
  // ==========================================================================

  describe('setPriorityFilter', () => {
    it('should set priority filter', () => {
      useFilterStore.getState().setPriorityFilter(['critical', 'high']);

      expect(useFilterStore.getState().priorityFilter).toEqual(['critical', 'high']);
    });

    it('should clear active preset', () => {
      useFilterStore.setState({ activePresetId: 'preset' });

      useFilterStore.getState().setPriorityFilter(['critical']);

      expect(useFilterStore.getState().activePresetId).toBeNull();
    });
  });

  describe('setFreshnessFilter', () => {
    it('should set freshness filter', () => {
      useFilterStore.getState().setFreshnessFilter(['realtime', 'hourly']);

      expect(useFilterStore.getState().freshnessFilter).toEqual(['realtime', 'hourly']);
    });
  });

  describe('setCategoryFilter', () => {
    it('should set category filter and update enabled node types', () => {
      useFilterStore.getState().setCategoryFilter(['project', 'content']);

      const state = useFilterStore.getState();
      expect(state.categoryFilter).toEqual(['project', 'content']);
      // Should have node types from both categories
      expect(state.enabledNodeTypes.has('Project')).toBe(true);
      expect(state.enabledNodeTypes.has('Concept')).toBe(true);
    });
  });

  describe('toggleCategory', () => {
    it('should add category if not present', () => {
      useFilterStore.setState({ categoryFilter: [] });

      useFilterStore.getState().toggleCategory('project');

      expect(useFilterStore.getState().categoryFilter).toContain('project');
    });

    it('should remove category if present', () => {
      useFilterStore.setState({ categoryFilter: ['project', 'content'] });

      useFilterStore.getState().toggleCategory('project');

      const state = useFilterStore.getState();
      expect(state.categoryFilter).not.toContain('project');
      expect(state.categoryFilter).toContain('content');
    });

    it('should reset to CORE_TYPES when all categories removed', () => {
      useFilterStore.setState({ categoryFilter: ['project'] });

      useFilterStore.getState().toggleCategory('project');

      expect(useFilterStore.getState().enabledNodeTypes).toEqual(new Set(CORE_TYPES));
    });
  });

  describe('setActiveOnly', () => {
    it('should set activeOnly flag', () => {
      useFilterStore.getState().setActiveOnly(true);

      expect(useFilterStore.getState().activeOnly).toBe(true);
    });
  });

  describe('setLocaleFamily', () => {
    it('should set locale family', () => {
      useFilterStore.getState().setLocaleFamily('en');

      expect(useFilterStore.getState().localeFamily).toBe('en');
    });
  });

  // ==========================================================================
  // Preset Actions
  // ==========================================================================

  describe('clearFilters', () => {
    it('should reset all filters to defaults', () => {
      // Set non-default values
      useFilterStore.setState({
        enabledNodeTypes: new Set(['Locale']),
        selectedProject: 'project-1',
        selectedLocale: 'en-US',
        searchQuery: 'test',
        depthLimit: 10,
        activePresetId: null,
        priorityFilter: ['critical'],
        freshnessFilter: ['realtime'],
        categoryFilter: ['project'],
        activeOnly: true,
        localeFamily: 'en',
      });

      useFilterStore.getState().clearFilters();

      const state = useFilterStore.getState();
      expect(state.enabledNodeTypes).toEqual(new Set(CORE_TYPES));
      expect(state.selectedProject).toBeNull();
      expect(state.selectedLocale).toBeNull();
      expect(state.searchQuery).toBe('');
      expect(state.depthLimit).toBe(2);
      expect(state.activePresetId).toBe(DEFAULT_PRESET.id);
      expect(state.priorityFilter).toEqual([]);
      expect(state.freshnessFilter).toEqual([]);
      expect(state.categoryFilter).toEqual([]);
      expect(state.activeOnly).toBe(false);
      expect(state.localeFamily).toBeNull();
    });
  });

  // ==========================================================================
  // Custom Presets
  // ==========================================================================

  describe('custom presets', () => {
    it('should add custom preset', () => {
      const preset = {
        id: 'custom-1',
        name: 'My Preset',
        description: 'Test preset',
        icon: '📁',
        shortcut: '',
        nodeTypes: ['Concept' as NodeType],
        locale: null,
      };

      useFilterStore.getState().addCustomPreset(preset);

      const state = useFilterStore.getState();
      expect(state.customPresets.length).toBe(1);
      expect(state.customPresets[0].id).toBe('custom-1');
      expect(state.customPresets[0].isCustom).toBe(true);
    });

    it('should remove custom preset', () => {
      useFilterStore.setState({
        customPresets: [
          { id: 'custom-1', name: 'Preset 1', description: '', icon: '', shortcut: '', nodeTypes: [], locale: null, isCustom: true },
          { id: 'custom-2', name: 'Preset 2', description: '', icon: '', shortcut: '', nodeTypes: [], locale: null, isCustom: true },
        ],
      });

      useFilterStore.getState().removeCustomPreset('custom-1');

      const state = useFilterStore.getState();
      expect(state.customPresets.length).toBe(1);
      expect(state.customPresets[0].id).toBe('custom-2');
    });
  });

  // ==========================================================================
  // NovaNetFilter Builder
  // ==========================================================================

  describe('toNovaNetFilter', () => {
    it('should create filter with current node types', () => {
      useFilterStore.setState({
        enabledNodeTypes: new Set(['Concept', 'Page']),
      });

      const filter = useFilterStore.getState().toNovaNetFilter();
      const types = filter.getResolvedNodeTypes();

      expect(types).toContain('Concept');
      expect(types).toContain('Page');
    });

    it('should include locale in filter', () => {
      useFilterStore.setState({
        selectedLocale: 'fr-FR',
      });

      const filter = useFilterStore.getState().toNovaNetFilter();
      const criteria = filter.getCriteria();

      // Locale is stored in filters.locale, not context.locale
      expect(criteria.filters.locale).toBe('fr-FR');
    });

    it('should include search query in filter', () => {
      useFilterStore.setState({
        searchQuery: 'test search',
      });

      const filter = useFilterStore.getState().toNovaNetFilter();
      const criteria = filter.getCriteria();

      // Search query is stored in filters.searchQuery, not criteria.search
      expect(criteria.filters.searchQuery).toBe('test search');
    });
  });

  // ==========================================================================
  // toCypher
  // ==========================================================================

  describe('toCypher', () => {
    it('should generate Cypher query from filter state', () => {
      useFilterStore.setState({
        enabledNodeTypes: new Set(['Concept']),
        depthLimit: 2,
      });

      const cypher = useFilterStore.getState().toCypher();

      expect(cypher).toContain('MATCH');
      expect(cypher).toContain('Concept');
    });
  });

  // ==========================================================================
  // Set Serialization (for localStorage)
  // ==========================================================================

  describe('Set serialization for persistence', () => {
    it('should serialize Set to array when storing', () => {
      // The persist middleware uses custom storage
      // This tests that the storage handler works correctly
      const types = new Set(['Concept', 'Page'] as NodeType[]);
      useFilterStore.setState({ enabledNodeTypes: types });

      // Trigger a state change that would cause persistence
      useFilterStore.getState().setSearchQuery('trigger');

      // Check that setItem was called with array format
      expect(localStorageMock.setItem).toHaveBeenCalled();
      const storedValue = localStorageMock.setItem.mock.calls[0]?.[1];
      if (storedValue) {
        const parsed = JSON.parse(storedValue);
        expect(Array.isArray(parsed.state.enabledNodeTypes)).toBe(true);
      }
    });
  });

  // ==========================================================================
  // Schema Mode - Collapsed Groups State (Task 3.1)
  // ==========================================================================

  describe('schema mode collapsed groups', () => {
    beforeEach(() => {
      // Reset collapsed state for each test
      useFilterStore.setState({
        collapsedRealms: [],
        collapsedLayers: [],
      });
    });

    describe('toggleRealmCollapsed', () => {
      it('should add realm to collapsedRealms when not present', () => {
        useFilterStore.getState().toggleRealmCollapsed('project');

        const state = useFilterStore.getState();
        expect(state.collapsedRealms).toContain('project');
      });

      it('should remove realm from collapsedRealms when already present', () => {
        useFilterStore.setState({ collapsedRealms: ['project'] });

        useFilterStore.getState().toggleRealmCollapsed('project');

        const state = useFilterStore.getState();
        expect(state.collapsedRealms).not.toContain('project');
      });

      it('should handle multiple realms independently', () => {
        useFilterStore.getState().toggleRealmCollapsed('project');
        useFilterStore.getState().toggleRealmCollapsed('global');

        const state = useFilterStore.getState();
        expect(state.collapsedRealms).toContain('project');
        expect(state.collapsedRealms).toContain('global');
        expect(state.collapsedRealms).not.toContain('shared');
      });
    });

    describe('toggleLayerCollapsed', () => {
      it('should add layer key to collapsedLayers when not present', () => {
        useFilterStore.getState().toggleLayerCollapsed('project', 'foundation');

        const state = useFilterStore.getState();
        expect(state.collapsedLayers).toContain('project-foundation');
      });

      it('should remove layer key from collapsedLayers when already present', () => {
        useFilterStore.setState({ collapsedLayers: ['project-foundation'] });

        useFilterStore.getState().toggleLayerCollapsed('project', 'foundation');

        const state = useFilterStore.getState();
        expect(state.collapsedLayers).not.toContain('project-foundation');
      });

      it('should handle multiple layers independently', () => {
        useFilterStore.getState().toggleLayerCollapsed('project', 'foundation');
        useFilterStore.getState().toggleLayerCollapsed('project', 'structure');
        useFilterStore.getState().toggleLayerCollapsed('global', 'knowledge');

        const state = useFilterStore.getState();
        expect(state.collapsedLayers).toContain('project-foundation');
        expect(state.collapsedLayers).toContain('project-structure');
        expect(state.collapsedLayers).toContain('global-knowledge');
      });
    });

    describe('isRealmCollapsed', () => {
      it('should return false when realm is not collapsed', () => {
        expect(useFilterStore.getState().isRealmCollapsed('project')).toBe(false);
      });

      it('should return true when realm is collapsed', () => {
        useFilterStore.getState().toggleRealmCollapsed('project');

        expect(useFilterStore.getState().isRealmCollapsed('project')).toBe(true);
      });
    });

    describe('isLayerCollapsed', () => {
      it('should return false when layer is not collapsed', () => {
        expect(useFilterStore.getState().isLayerCollapsed('project', 'foundation')).toBe(false);
      });

      it('should return true when layer is collapsed', () => {
        useFilterStore.getState().toggleLayerCollapsed('project', 'foundation');

        expect(useFilterStore.getState().isLayerCollapsed('project', 'foundation')).toBe(true);
      });
    });

    describe('resetSchemaFilters', () => {
      it('should reset collapsedRealms to empty array', () => {
        useFilterStore.setState({ collapsedRealms: ['project', 'global'] });

        useFilterStore.getState().resetSchemaFilters();

        expect(useFilterStore.getState().collapsedRealms).toEqual([]);
      });

      it('should reset collapsedLayers to empty array', () => {
        useFilterStore.setState({
          collapsedLayers: ['project-foundation', 'global-knowledge'],
        });

        useFilterStore.getState().resetSchemaFilters();

        expect(useFilterStore.getState().collapsedLayers).toEqual([]);
      });

      it('should reset both collapsedRealms and collapsedLayers', () => {
        useFilterStore.setState({
          collapsedRealms: ['project', 'global'],
          collapsedLayers: ['project-foundation', 'global-knowledge'],
        });

        useFilterStore.getState().resetSchemaFilters();

        const state = useFilterStore.getState();
        expect(state.collapsedRealms).toEqual([]);
        expect(state.collapsedLayers).toEqual([]);
      });
    });
  });
});
