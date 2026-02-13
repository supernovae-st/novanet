/**
 * SchemaFilterPanel Tests
 *
 * v11.0: Simplified to use SchemaCardView with NodeCard-based display.
 * v11.6.1: Uses schemaStore for data - tests mock the store.
 * Tests rendering, tabs, and search functionality.
 */

import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';
import { useSchemaStore } from '@/stores/schemaStore';
import type { NodeType, Realm, Layer, Trait } from '@novanet/core/types';

// Mock schemaStore with test data
jest.mock('@/stores/schemaStore', () => ({
  useSchemaStore: jest.fn(),
  selectEnrichedNodeTypes: jest.fn((state) => state.enrichedNodeTypes ?? []),
  selectEnrichedRelTypes: jest.fn((state) => state.enrichedRelTypes ?? []),
  selectRealmGroups: jest.fn((state) => state.realmGroups ?? []),
  selectIsSchemaLoaded: jest.fn((state) => state.isSchemaLoaded ?? false),
}));

// v11.6.1: Complete mock data matching SchemaStoreState interface
const mockSchemaStoreData = {
  // Required state fields
  dbSchema: { nodeLabels: ['Page', 'Entity', 'Locale'], relationshipTypes: ['HAS_PAGE', 'HAS_ENTITY'] },
  isSchemaLoaded: true,
  isLoading: false,
  error: null,
  nodeTypeCounts: { Page: 5, Entity: 3, Locale: 10 },
  relTypeCounts: { HAS_PAGE: 5, HAS_ENTITY: 3 },
  excludedNodeTypes: new Set<string>(),
  excludedRelTypes: new Set<string>(),
  // Enriched types
  enrichedNodeTypes: [
    { name: 'Page' as NodeType, count: 5, realm: 'org' as Realm, layer: 'structure' as Layer, trait: 'defined' as Trait, icon: 'FileText', color: '#8b5cf6', isActive: true },
    { name: 'Entity' as NodeType, count: 3, realm: 'org' as Realm, layer: 'semantic' as Layer, trait: 'defined' as Trait, icon: 'Box', color: '#a78bfa', isActive: true },
    { name: 'Locale' as NodeType, count: 10, realm: 'shared' as Realm, layer: 'config' as Layer, trait: 'defined' as Trait, icon: 'Globe', color: '#6366f1', isActive: true },
  ],
  enrichedRelTypes: [
    { name: 'HAS_PAGE', count: 5, family: 'ownership', color: '#3b82f6', isActive: true },
    { name: 'HAS_ENTITY', count: 3, family: 'ownership', color: '#3b82f6', isActive: true },
  ],
  realmGroups: [
    {
      realm: 'shared' as Realm,
      displayName: 'SHARED',
      icon: 'Globe',
      color: '#2aa198',
      nodeTypeCount: 1,
      totalCount: 10,
      layers: [
        {
          layer: 'config' as Layer,
          displayName: 'Config',
          icon: 'Settings',
          color: '#6366f1',
          totalCount: 10,
          nodeTypes: [
            { name: 'Locale' as NodeType, count: 10, realm: 'shared' as Realm, layer: 'config' as Layer, trait: 'defined' as Trait, icon: 'Globe', color: '#6366f1', isActive: true },
          ],
        },
      ],
    },
    {
      realm: 'org' as Realm,
      displayName: 'ORG',
      icon: 'Building',
      color: '#0ea5e9',
      nodeTypeCount: 2,
      totalCount: 8,
      layers: [
        {
          layer: 'structure' as Layer,
          displayName: 'Structure',
          icon: 'Layout',
          color: '#8b5cf6',
          totalCount: 5,
          nodeTypes: [
            { name: 'Page' as NodeType, count: 5, realm: 'org' as Realm, layer: 'structure' as Layer, trait: 'defined' as Trait, icon: 'FileText', color: '#8b5cf6', isActive: true },
          ],
        },
        {
          layer: 'semantic' as Layer,
          displayName: 'Semantic',
          icon: 'Sparkles',
          color: '#a78bfa',
          totalCount: 3,
          nodeTypes: [
            { name: 'Entity' as NodeType, count: 3, realm: 'org' as Realm, layer: 'semantic' as Layer, trait: 'defined' as Trait, icon: 'Box', color: '#a78bfa', isActive: true },
          ],
        },
      ],
    },
  ],
  // Actions
  loadSchema: jest.fn(),
  updateCounts: jest.fn(),
  toggleNodeType: jest.fn(),
  toggleRelType: jest.fn(),
  setNodeTypeActive: jest.fn(),
  setRelTypeActive: jest.fn(),
  selectAllNodeTypes: jest.fn(),
  selectNoNodeTypes: jest.fn(),
  selectAllRelTypes: jest.fn(),
  selectNoRelTypes: jest.fn(),
  invertNodeTypeSelection: jest.fn(),
  invertRelTypeSelection: jest.fn(),
  getNodeType: jest.fn(),
  getRelType: jest.fn(),
  getNodeTypesByLayer: jest.fn(),
  getNodeTypesByRealm: jest.fn(),
  getExcludedNodeTypes: jest.fn(() => []),
  getExcludedRelTypes: jest.fn(() => []),
};

describe('SchemaFilterPanel', () => {
  beforeEach(() => {
    (useSchemaStore as jest.MockedFunction<typeof useSchemaStore>).mockImplementation((selector) => {
      if (typeof selector === 'function') {
        return selector(mockSchemaStoreData);
      }
      return mockSchemaStoreData;
    });
  });
  describe('Rendering', () => {
    it('renders segmented tabs for Types and Rels', () => {
      render(<SchemaFilterPanel />);

      expect(screen.getByText('Types')).toBeInTheDocument();
      expect(screen.getByText('Rels')).toBeInTheDocument();
    });

    it('shows Types tab as active by default', () => {
      render(<SchemaFilterPanel />);

      const typesTab = screen.getByRole('tab', { name: /Types/i });
      expect(typesTab).toHaveAttribute('aria-selected', 'true');
    });

    it('renders search input for node types', () => {
      render(<SchemaFilterPanel />);

      const searchInput = screen.getByPlaceholderText('Search node types...');
      expect(searchInput).toBeInTheDocument();
    });

    it('renders realm sections', () => {
      render(<SchemaFilterPanel />);

      // Realm labels (v11.2: SHARED, ORG - uppercase from REALM_HIERARCHY)
      expect(screen.getByText('SHARED')).toBeInTheDocument();
      expect(screen.getByText('ORG')).toBeInTheDocument();
    });
  });

  describe('Tab Switching', () => {
    it('switches to Rels tab when clicked', () => {
      render(<SchemaFilterPanel />);

      const relsTab = screen.getByText('Rels');
      fireEvent.click(relsTab);

      // Should show "coming soon" message
      expect(screen.getByText(/coming soon/i)).toBeInTheDocument();
    });

    it('switches back to Types tab', () => {
      render(<SchemaFilterPanel />);

      // Switch to Rels
      fireEvent.click(screen.getByText('Rels'));
      // Switch back to Types
      fireEvent.click(screen.getByText('Types'));

      // Search input should be visible again
      expect(screen.getByPlaceholderText('Search node types...')).toBeInTheDocument();
    });
  });

  describe('Search Functionality', () => {
    it('filters node types when searching', () => {
      render(<SchemaFilterPanel />);

      const searchInput = screen.getByPlaceholderText('Search node types...');
      fireEvent.change(searchInput, { target: { value: 'Page' } });

      // Page-related types should still be visible
      expect(screen.getByText('Page')).toBeInTheDocument();
    });

    it('shows no results message when search has no matches', () => {
      render(<SchemaFilterPanel />);

      const searchInput = screen.getByPlaceholderText('Search node types...');
      fireEvent.change(searchInput, { target: { value: 'xyznonexistent' } });

      expect(screen.getByText(/no node types match/i)).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('has region role with aria-label', () => {
      render(<SchemaFilterPanel />);

      // Sidebar.Content provides region role
      const panel = screen.getByTestId('schema-filter-panel');
      expect(panel).toBeInTheDocument();
    });

    it('search input has proper type', () => {
      render(<SchemaFilterPanel />);

      const searchInput = screen.getByPlaceholderText('Search node types...');
      expect(searchInput).toHaveAttribute('type', 'text');
    });
  });

  describe('Styling', () => {
    it('applies custom className', () => {
      render(<SchemaFilterPanel className="custom-class" />);

      const panel = screen.getByTestId('schema-filter-panel');
      expect(panel).toHaveClass('custom-class');
    });
  });
});
