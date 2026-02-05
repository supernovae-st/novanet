// src/components/sidebar/__tests__/SchemaFilterPanel.test.tsx
import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';
import { useFilterStore } from '@/stores/filterStore';

// Mock the filterStore
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));

const mockToggleLayerCollapsed = jest.fn();
const mockIsLayerCollapsed = jest.fn();
const mockSetLayerCollapsed = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

describe('SchemaFilterPanel', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockIsLayerCollapsed.mockReturnValue(false);

    mockUseFilterStore.mockImplementation((selector) => {
      const state = {
        toggleLayerCollapsed: mockToggleLayerCollapsed,
        isLayerCollapsed: mockIsLayerCollapsed,
        setLayerCollapsed: mockSetLayerCollapsed,
      };
      return selector ? selector(state as never) : state;
    });
  });

  describe('Rendering', () => {
    it('renders both realms with icons and labels', () => {
      render(<SchemaFilterPanel />);

      // Check realm headers with icons (v10.6: 2 realms)
      expect(screen.getByText('TENANT')).toBeInTheDocument();
      expect(screen.getByText('GLOBAL')).toBeInTheDocument();
    });

    it('renders realm icons as Lucide SVGs', () => {
      render(<SchemaFilterPanel />);

      // Realm icons are Lucide SVGs (Package, Globe), not emojis
      const svgIcons = document.querySelectorAll('svg');
      expect(svgIcons.length).toBeGreaterThan(0);
    });

    it('renders node count for each realm in parentheses', () => {
      render(<SchemaFilterPanel />);

      // Realm counts are in parentheses — v10.6: tenant + global
      // Note: exact counts depend on REALM_HIERARCHY which comes from @novanet/core
      const countElements = screen.getAllByText(/^\(\d+\)$/);
      expect(countElements.length).toBeGreaterThanOrEqual(2);
    });

    it('renders layers for Tenant realm', () => {
      render(<SchemaFilterPanel />);

      // Tenant layers (v10.6: foundation, structure, semantic, instruction, output)
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      // Semantic layer exists in both realms, so use getAllByText
      expect(screen.getAllByText('Semantic Layer').length).toBeGreaterThanOrEqual(1);
      expect(screen.getByText('Instructions')).toBeInTheDocument();
      expect(screen.getByText('Generated Output')).toBeInTheDocument();
    });

    it('renders layers for Global realm', () => {
      render(<SchemaFilterPanel />);

      // Global layers (v10.6: config, knowledge, seo)
      expect(screen.getByText('Configuration')).toBeInTheDocument();
      expect(screen.getByText('Locale Knowledge')).toBeInTheDocument();
      expect(screen.getByText('SEO Intelligence')).toBeInTheDocument();
    });

    it('renders layer labels with icons (Lucide SVG)', () => {
      render(<SchemaFilterPanel />);

      // Layer labels are rendered - icons are Lucide SVGs (not emojis)
      // v10.6: 8 layers total (5 tenant + 3 global)
      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByText('Structure')).toBeInTheDocument();
      expect(screen.getByText('Instructions')).toBeInTheDocument();
      expect(screen.getByText('Generated Output')).toBeInTheDocument();
      expect(screen.getByText('Configuration')).toBeInTheDocument();
      expect(screen.getByText('Locale Knowledge')).toBeInTheDocument();
      expect(screen.getByText('SEO Intelligence')).toBeInTheDocument();

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

      // FilterSection has chevron buttons with aria-label (v10.6: 2 realms)
      const collapseButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expect(collapseButtons.length).toBeGreaterThanOrEqual(2);
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

      // FilterSection has checkboxes with role="checkbox" (v10.6: 2 realms)
      const checkboxes = screen.getAllByRole('checkbox');
      expect(checkboxes.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe('Layer Toggle Behavior', () => {
    it('calls toggleLayerCollapsed when layer row is clicked', () => {
      render(<SchemaFilterPanel />);

      // Click on Foundation layer row
      const foundationText = screen.getByText('Foundation');
      fireEvent.click(foundationText);

      expect(mockToggleLayerCollapsed).toHaveBeenCalledWith('tenant', 'foundation');
    });

    it('renders layer checkboxes with checked state', () => {
      render(<SchemaFilterPanel />);

      // FilterTree.Row renders checkboxes with aria-checked for toggle state
      // Find all checkboxes (v10.6: 2 realm checkboxes + layer checkboxes)
      const checkboxes = screen.getAllByRole('checkbox', { checked: true });
      expect(checkboxes.length).toBeGreaterThanOrEqual(2);
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

      // FilterSection uses buttons with aria-expanded instead of tree/treeitem (v10.6: 2 realms)
      const sectionButtons = screen.getAllByRole('button', { name: /Collapse/ });
      expect(sectionButtons.length).toBe(2);
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

      // FilterSection content has role="group" (v10.6: 2 realms)
      const groups = screen.getAllByRole('group');
      expect(groups.length).toBeGreaterThanOrEqual(2);
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

      // FilterTree.Section renders scope labels (v10.6: 2 realms)
      expect(screen.getByText('TENANT')).toBeInTheDocument();
      expect(screen.getByText('GLOBAL')).toBeInTheDocument();
    });
  });
});
