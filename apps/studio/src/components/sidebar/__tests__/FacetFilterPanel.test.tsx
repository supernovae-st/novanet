/**
 * FacetFilterPanel Tests
 *
 * Tests rendering of 4 facet sections (Realms, Layers, Traits, Arc Families),
 * tri-state checkboxes, and toggle behavior via filterStore.
 */

import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import { FacetFilterPanel } from '../FacetFilterPanel';
import { useFilterStore } from '@/stores/filterStore';

// Mock filterStore
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));

const mockToggleRealm = jest.fn();
const mockToggleTrait = jest.fn();
const mockToggleLayer = jest.fn();
const mockToggleArcFamily = jest.fn();
const mockSetRealmFilter = jest.fn();
const mockSetLayerFilter = jest.fn();
const mockSetTraitFilter = jest.fn();
const mockSetArcFamilyFilter = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

function setupStore(overrides: Record<string, unknown> = {}) {
  const defaultState = {
    realmFilter: [] as string[],
    traitFilter: [] as string[],
    layerFilter: [] as string[],
    arcFamilyFilter: [] as string[],
    toggleRealm: mockToggleRealm,
    toggleTrait: mockToggleTrait,
    toggleLayer: mockToggleLayer,
    toggleArcFamily: mockToggleArcFamily,
    setRealmFilter: mockSetRealmFilter,
    setLayerFilter: mockSetLayerFilter,
    setTraitFilter: mockSetTraitFilter,
    setArcFamilyFilter: mockSetArcFamilyFilter,
    ...overrides,
  };

  // Mock getState for section toggle handlers
  (useFilterStore as unknown as { getState: () => typeof defaultState }).getState = () => defaultState;

  mockUseFilterStore.mockImplementation((selector) => {
    return selector ? selector(defaultState as never) : defaultState;
  });

  return defaultState;
}

describe('FacetFilterPanel', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    setupStore();
  });

  // ==========================================================================
  // Rendering
  // ==========================================================================

  describe('rendering', () => {
    it('renders all 4 facet sections', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Realms')).toBeInTheDocument();
      expect(screen.getByText('Layers')).toBeInTheDocument();
      expect(screen.getByText('Traits')).toBeInTheDocument();
      expect(screen.getByText('Arc Families')).toBeInTheDocument();
    });

    it('renders 2 realm items (v10.6: global + tenant)', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Global')).toBeInTheDocument();
      expect(screen.getByText('Tenant')).toBeInTheDocument();
      // v10.6: organization + project merged into tenant
    });

    it('renders 10 layer items (v11.5: 4 shared + 6 org)', () => {
      render(<FacetFilterPanel />);

      // Shared realm (4 layers: config, locale, geography, knowledge)
      expect(screen.getByText('Locale')).toBeInTheDocument();
      expect(screen.getByText('Geography')).toBeInTheDocument();
      // v11.8: "Knowledge" is layer only, trait renamed to "Imported" per ADR-024
      expect(screen.getByText('Knowledge')).toBeInTheDocument();
      expect(screen.getByText('Imported')).toBeInTheDocument();
      expect(screen.getByText('Configuration')).toBeInTheDocument();  // v11.5: shared config layer visible
      // Org realm (6 layers: config, foundation, structure, semantic, instruction, output)
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      // "Semantic" appears in both Layers and Arc Families sections
      expect(screen.getAllByText('Semantic')).toHaveLength(2);
      expect(screen.getByText('Instructions')).toBeInTheDocument();
      // v11.5: SEO/GEO layers removed - nodes in shared/knowledge now
      expect(screen.getByText('Generated Output')).toBeInTheDocument();
    });

    // v11.8: Renamed per ADR-024 Data Origin semantics
    it('renders 5 trait items (v11.8: defined/authored/imported/generated/retrieved)', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Defined')).toBeInTheDocument();    // was: Invariant
      expect(screen.getByText('Authored')).toBeInTheDocument();   // was: Localized
      expect(screen.getByText('Imported')).toBeInTheDocument();   // was: Knowledge trait
      expect(screen.getByText('Generated')).toBeInTheDocument();
      expect(screen.getByText('Retrieved')).toBeInTheDocument();  // was: Aggregated
    });

    it('renders 5 arc family items', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Ownership')).toBeInTheDocument();
      expect(screen.getByText('Localization')).toBeInTheDocument();
      // "Semantic" shared with Layers — already tested above
      expect(screen.getByText('Generation')).toBeInTheDocument();
      expect(screen.getByText('Mining')).toBeInTheDocument();
    });

    it('renders Faceted Query header', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Faceted Query')).toBeInTheDocument();
    });

    it('shows help text when no facets active', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Select facets to filter the graph')).toBeInTheDocument();
    });
  });

  // ==========================================================================
  // Active facet count
  // ==========================================================================

  describe('active facet count', () => {
    it('shows active count when facets are selected', () => {
      setupStore({
        realmFilter: ['shared'],
        traitFilter: ['authored'], // v11.8: was 'localized'
        layerFilter: [],
      });

      render(<FacetFilterPanel />);

      expect(screen.getByText('2 active')).toBeInTheDocument();
    });

    it('hides active count when no facets selected', () => {
      render(<FacetFilterPanel />);

      expect(screen.queryByText(/active/)).not.toBeInTheDocument();
    });

    it('hides help text when facets are active', () => {
      setupStore({
        realmFilter: ['shared'],
        traitFilter: [],
        layerFilter: [],
      });

      render(<FacetFilterPanel />);

      expect(screen.queryByText('Select facets to filter the graph')).not.toBeInTheDocument();
    });
  });

  // ==========================================================================
  // Test ID
  // ==========================================================================

  describe('test id', () => {
    it('has correct test id', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByTestId('facet-filter-panel')).toBeInTheDocument();
    });
  });
});
