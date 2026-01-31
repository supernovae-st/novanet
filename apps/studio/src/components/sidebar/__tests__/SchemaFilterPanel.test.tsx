// src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx
import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';
import { useFilterStore } from '@/stores/filterStore';

// Mock the filterStore
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));

const mockToggleScopeCollapsed = jest.fn();
const mockToggleSubcategoryCollapsed = jest.fn();
const mockIsScopeCollapsed = jest.fn();
const mockIsSubcategoryCollapsed = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

describe('SchemaFilterPanel', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockIsScopeCollapsed.mockReturnValue(false);
    mockIsSubcategoryCollapsed.mockReturnValue(false);

    mockUseFilterStore.mockImplementation((selector) => {
      const state = {
        collapsedScopes: [],
        collapsedSubcategories: [],
        toggleScopeCollapsed: mockToggleScopeCollapsed,
        toggleSubcategoryCollapsed: mockToggleSubcategoryCollapsed,
        isScopeCollapsed: mockIsScopeCollapsed,
        isSubcategoryCollapsed: mockIsSubcategoryCollapsed,
      };
      return selector ? selector(state as never) : state;
    });
  });

  describe('Rendering', () => {
    it('renders all 3 scopes with icons and labels', () => {
      render(<SchemaFilterPanel />);

      // Check scope headers with icons
      expect(screen.getByText('PROJECT')).toBeInTheDocument();
      expect(screen.getByText('GLOBAL')).toBeInTheDocument();
      expect(screen.getByText('SHARED')).toBeInTheDocument();
    });

    it('renders scope emojis', () => {
      render(<SchemaFilterPanel />);

      // Emojis are rendered as text
      expect(screen.getByText(/📦/)).toBeInTheDocument(); // Project
      expect(screen.getByText(/🌍/)).toBeInTheDocument(); // Global
      expect(screen.getByText(/🎯/)).toBeInTheDocument(); // Shared
    });

    it('renders node count for each scope', () => {
      render(<SchemaFilterPanel />);

      // Project scope has 14 node types
      expect(screen.getByText('14')).toBeInTheDocument();
      // Global scope has 15 node types
      expect(screen.getByText('15')).toBeInTheDocument();
      // Shared scope has 6 node types
      expect(screen.getByText('6')).toBeInTheDocument();
    });

    it('renders subcategories for Project scope', () => {
      render(<SchemaFilterPanel />);

      // Project subcategories
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      expect(screen.getByText('Semantic')).toBeInTheDocument();
      expect(screen.getByText('Instruction')).toBeInTheDocument();
      expect(screen.getByText('Output')).toBeInTheDocument();
    });

    it('renders subcategories for Global scope', () => {
      render(<SchemaFilterPanel />);

      // Global subcategories
      expect(screen.getByText('Configuration')).toBeInTheDocument();
      expect(screen.getByText('Knowledge')).toBeInTheDocument();
    });

    it('renders subcategories for Shared scope', () => {
      render(<SchemaFilterPanel />);

      // Shared subcategories
      expect(screen.getByText('SEO')).toBeInTheDocument();
      expect(screen.getByText('GEO')).toBeInTheDocument();
    });

    it('renders subcategory icons', () => {
      render(<SchemaFilterPanel />);

      // Subcategory icons
      expect(screen.getByText(/🏛️/)).toBeInTheDocument(); // Foundation
      expect(screen.getByText(/🧱/)).toBeInTheDocument(); // Structure
      expect(screen.getByText(/💡/)).toBeInTheDocument(); // Semantic
      expect(screen.getByText(/📝/)).toBeInTheDocument(); // Instruction
      expect(screen.getByText(/📄/)).toBeInTheDocument(); // Output
      expect(screen.getByText(/⚙️/)).toBeInTheDocument(); // Configuration
      expect(screen.getByText(/🧠/)).toBeInTheDocument(); // Knowledge
      expect(screen.getByText(/🔍/)).toBeInTheDocument(); // SEO
      expect(screen.getByText(/🤖/)).toBeInTheDocument(); // GEO
    });

    it('renders node count in parentheses for each subcategory', () => {
      render(<SchemaFilterPanel />);

      // Correct counts from models/nodes/ folder structure:
      // - foundation: 3 nodes (BrandIdentity, Project, ProjectL10n)
      // - instruction: 5 nodes (BlockPrompt, BlockRules, BlockType, PagePrompt, PageType)
      // - output: 2 nodes (BlockL10n, PageL10n)
      // - semantic: 2 nodes (Concept, ConceptL10n)
      // - structure: 2 nodes (Block, Page)
      // - config: 1 node (Locale)
      // - knowledge: 14 nodes
      // - geo: 3 nodes
      // - seo: 3 nodes

      // Foundation, SEO, GEO have 3 nodes each
      const threeNodes = screen.getAllByText('(3)');
      expect(threeNodes.length).toBeGreaterThanOrEqual(3);

      // Structure, Semantic, Output have 2 nodes each
      const twoNodes = screen.getAllByText('(2)');
      expect(twoNodes.length).toBeGreaterThanOrEqual(3);

      // Instruction has 5 nodes (unique count)
      expect(screen.getByText('(5)')).toBeInTheDocument();

      // Knowledge has 14 nodes (unique count)
      expect(screen.getByText('(14)')).toBeInTheDocument();

      // Config has 1 node (unique count)
      expect(screen.getByText('(1)')).toBeInTheDocument();
    });

    it('renders the header with Schema Browser title', () => {
      render(<SchemaFilterPanel />);

      expect(screen.getByText('Schema Browser')).toBeInTheDocument();
      expect(screen.getByText('35 node types')).toBeInTheDocument();
    });

    it('renders the stats footer', () => {
      render(<SchemaFilterPanel />);

      // The footer text contains all stats in a single element
      const statsFooter = screen.getByText(/3 scopes .* 9 categories .* 35 types/);
      expect(statsFooter).toBeInTheDocument();
    });
  });

  describe('Collapse/Expand Behavior', () => {
    it('calls toggleScopeCollapsed when scope header is clicked', () => {
      render(<SchemaFilterPanel />);

      // Click on Project scope header
      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toBeInTheDocument();
      fireEvent.click(projectButton!);

      expect(mockToggleScopeCollapsed).toHaveBeenCalledWith('Project');
    });

    it('shows ChevronDown when scope is expanded', () => {
      mockIsScopeCollapsed.mockReturnValue(false);
      render(<SchemaFilterPanel />);

      // When expanded, aria-expanded should be true
      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toHaveAttribute('aria-expanded', 'true');
    });

    it('shows ChevronRight when scope is collapsed', () => {
      mockIsScopeCollapsed.mockImplementation((scope) => scope === 'Project');
      render(<SchemaFilterPanel />);

      // When collapsed, aria-expanded should be false
      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toHaveAttribute('aria-expanded', 'false');
    });

    it('hides subcategories when scope is collapsed', () => {
      mockIsScopeCollapsed.mockImplementation((scope) => scope === 'Project');
      render(<SchemaFilterPanel />);

      // Project subcategories container should be visually hidden (max-h-0 opacity-0)
      const projectContent = document.getElementById('scope-Project-content');
      expect(projectContent).toHaveClass('max-h-0', 'opacity-0');

      // Other scopes should still show their subcategories (max-h-96 opacity-100)
      const globalContent = document.getElementById('scope-Global-content');
      expect(globalContent).toHaveClass('max-h-96', 'opacity-100');
    });
  });

  describe('Subcategory Toggle Behavior', () => {
    it('calls toggleSubcategoryCollapsed when subcategory is clicked', () => {
      render(<SchemaFilterPanel />);

      // Click on Foundation subcategory
      const foundationButton = screen.getByText('Foundation').closest('button');
      expect(foundationButton).toBeInTheDocument();
      fireEvent.click(foundationButton!);

      expect(mockToggleSubcategoryCollapsed).toHaveBeenCalledWith('Project', 'foundation');
    });

    it('applies opacity when subcategory is collapsed', () => {
      mockIsSubcategoryCollapsed.mockImplementation(
        (scope, subcat) => scope === 'Project' && subcat === 'foundation'
      );
      render(<SchemaFilterPanel />);

      const foundationButton = screen.getByText('Foundation').closest('button');
      // Collapsed subcategories have opacity-40 class
      expect(foundationButton).toHaveClass('opacity-40');
    });

    it('has normal styling when subcategory is not collapsed', () => {
      mockIsSubcategoryCollapsed.mockReturnValue(false);
      render(<SchemaFilterPanel />);

      const foundationButton = screen.getByText('Foundation').closest('button');
      // Non-collapsed subcategories have background styling
      expect(foundationButton).toHaveClass('bg-white/[0.04]');
    });
  });

  describe('Accessibility', () => {
    it('has region role with aria-label', () => {
      render(<SchemaFilterPanel />);

      const panel = screen.getByRole('region', { name: 'Schema filters' });
      expect(panel).toBeInTheDocument();
    });

    it('scope buttons have aria-expanded attribute', () => {
      render(<SchemaFilterPanel />);

      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toHaveAttribute('aria-expanded');
    });

    it('scope buttons have aria-controls attribute', () => {
      render(<SchemaFilterPanel />);

      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toHaveAttribute('aria-controls', 'scope-Project-content');
    });

    it('subcategory buttons have aria-pressed attribute', () => {
      render(<SchemaFilterPanel />);

      const foundationButton = screen.getByText('Foundation').closest('button');
      expect(foundationButton).toHaveAttribute('aria-pressed');
    });

    it('subcategory buttons have descriptive aria-label', () => {
      render(<SchemaFilterPanel />);

      const foundationButton = screen.getByText('Foundation').closest('button');
      expect(foundationButton).toHaveAttribute(
        'aria-label',
        expect.stringContaining('Foundation')
      );
      expect(foundationButton).toHaveAttribute(
        'aria-label',
        expect.stringContaining('node types')
      );
    });

    it('subcategory content regions have role group', () => {
      render(<SchemaFilterPanel />);

      const projectSubcats = screen.getByRole('group', { name: 'PROJECT subcategories' });
      expect(projectSubcats).toBeInTheDocument();
    });
  });

  describe('Styling', () => {
    it('applies custom className', () => {
      render(<SchemaFilterPanel className="custom-class" />);

      const panel = screen.getByTestId('schema-filter-panel');
      expect(panel).toHaveClass('custom-class');
    });

    it('has glassmorphism styling on scope buttons', () => {
      render(<SchemaFilterPanel />);

      // Scope headers should be buttons with proper styling
      const projectButton = screen.getByText('PROJECT').closest('button');
      expect(projectButton).toBeInTheDocument();
      expect(projectButton).toHaveClass('transition-colors');
    });
  });
});
