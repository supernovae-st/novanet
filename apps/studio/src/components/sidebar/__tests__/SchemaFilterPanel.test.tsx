// src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx
import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';
import { useFilterStore } from '@/stores/filterStore';

// Mock the filterStore
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));

const mockToggleSubcategoryCollapsed = jest.fn();
const mockIsSubcategoryCollapsed = jest.fn();
const mockSetSubcategoryCollapsed = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

describe('SchemaFilterPanel', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockIsSubcategoryCollapsed.mockReturnValue(false);

    mockUseFilterStore.mockImplementation((selector) => {
      const state = {
        toggleSubcategoryCollapsed: mockToggleSubcategoryCollapsed,
        isSubcategoryCollapsed: mockIsSubcategoryCollapsed,
        setSubcategoryCollapsed: mockSetSubcategoryCollapsed,
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

    it('renders node count for each scope in parentheses', () => {
      render(<SchemaFilterPanel />);

      // Scope counts are in parentheses: (14), (15), (6)
      expect(screen.getByText('(14)')).toBeInTheDocument(); // Project
      expect(screen.getByText('(15)')).toBeInTheDocument(); // Global
      expect(screen.getByText('(6)')).toBeInTheDocument(); // Shared
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

    it('renders the header with Schema Browser title', () => {
      render(<SchemaFilterPanel />);

      expect(screen.getByText('Schema Browser')).toBeInTheDocument();
      expect(screen.getByText('35 node types')).toBeInTheDocument();
    });

    it('renders the stats footer', () => {
      render(<SchemaFilterPanel />);

      // The footer text contains all stats
      const statsFooter = screen.getByText(/3 scopes .* 9 categories .* 35 types/);
      expect(statsFooter).toBeInTheDocument();
    });
  });

  describe('FilterTree Section Behavior', () => {
    it('has expand/collapse chevron buttons with aria-label', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Section has chevron buttons with aria-label
      const collapseButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expect(collapseButtons.length).toBeGreaterThanOrEqual(3); // 3 scopes
    });

    it('sections default to expanded (aria-expanded=true)', () => {
      render(<SchemaFilterPanel />);

      // FilterTree sections have aria-expanded on the treeitem
      const treeItems = screen.getAllByRole('treeitem');
      treeItems.forEach((item) => {
        expect(item).toHaveAttribute('aria-expanded', 'true');
      });
    });

    it('has tri-state checkboxes for scopes', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Section has TriStateCheckbox with role="checkbox"
      const checkboxes = screen.getAllByRole('checkbox');
      expect(checkboxes.length).toBeGreaterThanOrEqual(3); // At least 3 scopes
    });
  });

  describe('Subcategory Toggle Behavior', () => {
    it('calls toggleSubcategoryCollapsed when subcategory row is clicked', () => {
      render(<SchemaFilterPanel />);

      // Click on Foundation subcategory row
      const foundationText = screen.getByText('Foundation');
      fireEvent.click(foundationText);

      expect(mockToggleSubcategoryCollapsed).toHaveBeenCalledWith('Project', 'foundation');
    });

    it('renders subcategory checkboxes', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Row renders checkbox-like elements
      // The subcategory rows have check icons when selected
      const checkboxes = screen.getAllByRole('checkbox');
      expect(checkboxes.length).toBeGreaterThanOrEqual(9); // 9 subcategories + 3 scopes
    });
  });

  describe('Accessibility', () => {
    it('has region role with aria-label', () => {
      render(<SchemaFilterPanel />);

      const panel = screen.getByRole('region', { name: 'Schema filters' });
      expect(panel).toBeInTheDocument();
    });

    it('has tree role on FilterTree root', () => {
      render(<SchemaFilterPanel />);

      const tree = screen.getByRole('tree');
      expect(tree).toBeInTheDocument();
    });

    it('sections have treeitem role', () => {
      render(<SchemaFilterPanel />);

      const treeItems = screen.getAllByRole('treeitem');
      expect(treeItems.length).toBe(3); // 3 scopes
    });

    it('section content has group role', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Section content has role="group"
      const groups = screen.getAllByRole('group');
      expect(groups.length).toBeGreaterThanOrEqual(3); // 3 scope groups
    });
  });

  describe('Styling', () => {
    it('applies custom className', () => {
      render(<SchemaFilterPanel className="custom-class" />);

      const panel = screen.getByTestId('schema-filter-panel');
      expect(panel).toHaveClass('custom-class');
    });

    it('has proper FilterTree structure for scopes', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Section renders scope labels
      expect(screen.getByText('PROJECT')).toBeInTheDocument();
      expect(screen.getByText('GLOBAL')).toBeInTheDocument();
      expect(screen.getByText('SHARED')).toBeInTheDocument();
    });
  });
});
