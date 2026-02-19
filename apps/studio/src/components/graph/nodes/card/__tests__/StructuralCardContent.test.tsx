/**
 * StructuralCardContent Component Tests (TDD)
 *
 * Testing the content preset for structural nodes (Page, Entity, Block, etc.)
 * Layout: Icon + Type label | Status dot
 *         Display name
 *         Key (optional)
 *         Layer badge
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import { StructuralCardContent, type StructuralCardContentProps } from '../presets/StructuralCardContent';

// Mock LayerIcon
jest.mock('@/components/ui/CategoryIcon', () => ({
  LayerIcon: ({ layer, size }: { layer: string; size: number }) => (
    <div data-testid="layer-icon" data-layer={layer} data-size={size} />
  ),
}));

// Mock NODE_TYPE_CONFIG
jest.mock('@/config/nodeTypes', () => ({
  NODE_TYPE_CONFIG: {
    Page: { label: 'Page', layer: 'structure' },
    Entity: { label: 'Entity', layer: 'semantic' },
    Block: { label: 'Block', layer: 'structure' },
  },
}));

const defaultProps: StructuralCardContentProps = {
  data: {
    id: 'node-1',
    type: 'Page',
    key: 'homepage',
    displayName: 'Homepage',
  },
  colors: { primary: '#0ea5e9', secondary: '#0284c7' },
  selected: false,
  isHovered: false,
  width: 200,
};

describe('StructuralCardContent', () => {
  describe('Rendering', () => {
    it('renders display name', () => {
      render(<StructuralCardContent {...defaultProps} />);

      expect(screen.getByText('Homepage')).toBeInTheDocument();
    });

    it('renders type label from NODE_TYPE_CONFIG', () => {
      render(<StructuralCardContent {...defaultProps} />);

      expect(screen.getByText('Page')).toBeInTheDocument();
    });

    it('renders key when different from displayName', () => {
      render(<StructuralCardContent {...defaultProps} />);

      expect(screen.getByText('homepage')).toBeInTheDocument();
    });

    it('renders key in footer even when same as displayName (v0.13.1 Passport design)', () => {
      // v0.13.1 Passport design: key is always shown in footer for DX
      const props = {
        ...defaultProps,
        data: { ...defaultProps.data, key: 'Homepage', displayName: 'Homepage' },
      };
      render(<StructuralCardContent {...props} />);

      // Should have two "Homepage" - display name + key in footer
      const elements = screen.getAllByText('Homepage');
      expect(elements).toHaveLength(2);
    });

    it('renders layer badge', () => {
      render(<StructuralCardContent {...defaultProps} />);

      expect(screen.getByText('structure')).toBeInTheDocument();
    });

    it('renders LayerIcon with correct props', () => {
      render(<StructuralCardContent {...defaultProps} />);

      const icon = screen.getByTestId('layer-icon');
      expect(icon).toHaveAttribute('data-layer', 'structure');
    });

    it('renders status dot', () => {
      const { container } = render(<StructuralCardContent {...defaultProps} />);

      // Status dot should have rounded-full class
      const statusDot = container.querySelector('.rounded-full');
      expect(statusDot).toBeInTheDocument();
    });
  });

  describe('Selection State', () => {
    it('applies pulse animation to status dot when selected', () => {
      const { container } = render(<StructuralCardContent {...defaultProps} selected={true} />);

      const statusDot = container.querySelector('.animate-pulse');
      expect(statusDot).toBeInTheDocument();
    });

    it('renders icon in both selected and hovered states', () => {
      const { rerender } = render(
        <StructuralCardContent {...defaultProps} selected={true} />
      );

      // Icon should render in selected state
      expect(screen.getByTestId('layer-icon')).toBeInTheDocument();

      // Icon should render in hovered state
      rerender(<StructuralCardContent {...defaultProps} isHovered={true} />);
      expect(screen.getByTestId('layer-icon')).toBeInTheDocument();
    });
  });

  describe('Color Application', () => {
    it('applies primary color to type label', () => {
      render(<StructuralCardContent {...defaultProps} />);

      const typeLabel = screen.getByText('Page');
      expect(typeLabel).toHaveStyle({ color: '#0ea5e9' });
    });

    it('applies primary color to layer badge', () => {
      render(<StructuralCardContent {...defaultProps} />);

      const badge = screen.getByText('structure');
      // Badge should have color styling
      expect(badge).toHaveStyle({ color: '#0ea5e9' });
    });
  });

  describe('Layout', () => {
    it('has consistent padding on content zone', () => {
      const { container } = render(<StructuralCardContent {...defaultProps} />);

      // v0.13.1 Passport design: padding is on Content Zone inner div, not root CardWrapper
      // Root is flex container: "relative flex h-full rounded-xl overflow-hidden"
      // Content Zone has: "relative flex-1 px-3 py-2.5 min-w-0 z-10"
      const contentZone = container.querySelector('.px-3.py-2\\.5');
      expect(contentZone).toBeInTheDocument();
    });
  });
});
