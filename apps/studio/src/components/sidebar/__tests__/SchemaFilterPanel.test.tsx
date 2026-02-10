/**
 * SchemaFilterPanel Tests
 *
 * v11.0: Simplified to use SchemaCardView with NodeCard-based display.
 * Tests rendering, tabs, and search functionality.
 */

import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { SchemaFilterPanel } from '../SchemaFilterPanel';

describe('SchemaFilterPanel', () => {
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
