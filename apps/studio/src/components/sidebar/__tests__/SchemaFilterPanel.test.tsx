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

    it('renders scope icons as Lucide SVGs', () => {
      render(<SchemaFilterPanel />);

      // Scope icons are Lucide SVGs (Package, Globe, Target), not emojis
      const svgIcons = document.querySelectorAll('svg');
      expect(svgIcons.length).toBeGreaterThan(0);
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

    it('renders subcategory labels with icons (Lucide SVG)', () => {
      render(<SchemaFilterPanel />);

      // Subcategory labels are rendered - icons are Lucide SVGs (not emojis)
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      expect(screen.getByText('Semantic')).toBeInTheDocument();
      expect(screen.getByText('Instruction')).toBeInTheDocument();
      expect(screen.getByText('Output')).toBeInTheDocument();
      expect(screen.getByText('Configuration')).toBeInTheDocument();
      expect(screen.getByText('Knowledge')).toBeInTheDocument();
      expect(screen.getByText('SEO')).toBeInTheDocument();
      expect(screen.getByText('GEO')).toBeInTheDocument();

      // Lucide icons render as SVG elements
      const svgIcons = document.querySelectorAll('svg');
      expect(svgIcons.length).toBeGreaterThan(0);
    });

    it('renders segmented tabs for Types and Rels', () => {
      render(<SchemaFilterPanel />);

      // Tab bar provides identity (no header needed)
      expect(screen.getByText('Types')).toBeInTheDocument();
      expect(screen.getByText('Rels')).toBeInTheDocument();
    });
  });

  describe('FilterSection Behavior', () => {
    it('has expand/collapse chevron buttons with aria-label', () => {
      render(<SchemaFilterPanel />);

      // FilterSection has chevron buttons with aria-label
      const collapseButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expect(collapseButtons.length).toBeGreaterThanOrEqual(3); // 3 scopes
    });

    it('sections default to expanded (aria-expanded=true)', () => {
      render(<SchemaFilterPanel />);

      // FilterSection buttons have aria-expanded
      const expandButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expandButtons.forEach((button) => {
        expect(button).toHaveAttribute('aria-expanded', 'true');
      });
    });

    it('has tri-state checkboxes for scopes', () => {
      render(<SchemaFilterPanel />);

      // FilterSection has checkboxes with role="checkbox"
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

    it('renders subcategory checkboxes with checked state', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Row renders checkboxes with aria-checked for toggle state
      // Find all checkboxes (3 scope checkboxes + 9 subcategory checkboxes = 12)
      const checkboxes = screen.getAllByRole('checkbox', { checked: true });
      // 3 scope checkboxes + 9 subcategory checkboxes = 12 checked by default
      expect(checkboxes.length).toBeGreaterThanOrEqual(12);
    });
  });

  describe('Accessibility', () => {
    it('has region role with aria-label', () => {
      render(<SchemaFilterPanel />);

      // No header passed → aria-label falls back to 'Sidebar panel'
      const panel = screen.getByRole('region', { name: 'Sidebar panel' });
      expect(panel).toBeInTheDocument();
    });

    it('has section buttons with aria-expanded', () => {
      render(<SchemaFilterPanel />);

      // FilterSection uses buttons with aria-expanded instead of tree/treeitem
      const sectionButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expect(sectionButtons.length).toBe(3); // 3 scopes
      sectionButtons.forEach((button) => {
        expect(button).toHaveAttribute('aria-expanded');
      });
    });

    it('sections have proper aria-controls', () => {
      render(<SchemaFilterPanel />);

      // FilterSection buttons have aria-controls pointing to content
      const sectionButtons = screen.getAllByRole('button', { name: /Collapse/ });
      sectionButtons.forEach((button) => {
        expect(button).toHaveAttribute('aria-controls');
      });
    });

    it('section content has group role', () => {
      render(<SchemaFilterPanel />);

      // FilterSection content has role="group"
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
