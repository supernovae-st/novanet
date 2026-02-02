/**
 * FacetFilterPanel Tests
 *
 * Tests rendering of 4 facet sections (Realms, Layers, Traits, Edge Families),
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
const mockToggleEdgeFamily = jest.fn();
const mockSetRealmFilter = jest.fn();
const mockSetLayerFilter = jest.fn();
const mockSetTraitFilter = jest.fn();
const mockSetEdgeFamilyFilter = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

function setupStore(overrides: Record<string, unknown> = {}) {
  const defaultState = {
    realmFilter: [] as string[],
    traitFilter: [] as string[],
    layerFilter: [] as string[],
    edgeFamilyFilter: [] as string[],
    toggleRealm: mockToggleRealm,
    toggleTrait: mockToggleTrait,
    toggleLayer: mockToggleLayer,
    toggleEdgeFamily: mockToggleEdgeFamily,
    setRealmFilter: mockSetRealmFilter,
    setLayerFilter: mockSetLayerFilter,
    setTraitFilter: mockSetTraitFilter,
    setEdgeFamilyFilter: mockSetEdgeFamilyFilter,
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
      expect(screen.getByText('Edge Families')).toBeInTheDocument();
    });

    it('renders 3 realm items', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Global')).toBeInTheDocument();
      expect(screen.getByText('Project')).toBeInTheDocument();
      expect(screen.getByText('Shared')).toBeInTheDocument();
    });

    it('renders 9 layer items', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Configuration')).toBeInTheDocument();
      expect(screen.getByText('Locale Knowledge')).toBeInTheDocument();
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      // "Semantic" appears in both Layers and Edge Families sections
      expect(screen.getAllByText('Semantic')).toHaveLength(2);
      expect(screen.getByText('Instructions')).toBeInTheDocument();
      expect(screen.getByText('Generated Output')).toBeInTheDocument();
      expect(screen.getByText('SEO Intelligence')).toBeInTheDocument();
      expect(screen.getByText('GEO Intelligence')).toBeInTheDocument();
    });

    it('renders 5 trait items', () => {
      render(<FacetFilterPanel />);

      expect(screen.getByText('Invariant')).toBeInTheDocument();
      expect(screen.getByText('Localized')).toBeInTheDocument();
      expect(screen.getByText('Knowledge')).toBeInTheDocument();
      expect(screen.getByText('Derived')).toBeInTheDocument();
      expect(screen.getByText('Job')).toBeInTheDocument();
    });

    it('renders 5 edge family items', () => {
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
        realmFilter: ['global'],
        traitFilter: ['localized'],
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
        realmFilter: ['global'],
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
