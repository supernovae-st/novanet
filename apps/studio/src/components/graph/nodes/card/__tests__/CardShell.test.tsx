/**
 * CardShell Component Tests (TDD)
 *
 * Testing the unified card wrapper component that handles:
 * - Gradient borders
 * - Selection effects (pulse ring, glassmorphism)
 * - Node handles
 * - Interaction states (hover, dimmed)
 * - Render props pattern for content
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import { CardShell, type CardShellProps } from '../CardShell';

// Mock React Flow hooks
jest.mock('@xyflow/react', () => ({
  Handle: ({ type, position }: { type: string; position: string }) => (
    <div data-testid={`handle-${type}-${position}`} />
  ),
  Position: {
    Top: 'top',
    Bottom: 'bottom',
    Left: 'left',
    Right: 'right',
  },
}));

// Mock effects components
jest.mock('../../effects', () => ({
  SelectionPulseRing: ({ color }: { color: string }) => (
    <div data-testid="pulse-ring" data-color={color} />
  ),
  GlassmorphismEffects: () => <div data-testid="glassmorphism" />,
  NodeHandles: ({ layout }: { layout: string }) => (
    <div data-testid="node-handles" data-layout={layout} />
  ),
}));

// Mock BlueprintOverlay
jest.mock('../../BlueprintOverlay', () => ({
  BlueprintOverlay: ({ color }: { color: string }) => (
    <div data-testid="blueprint-overlay" data-color={color} />
  ),
}));

// Mock useNodeInteractions hook
jest.mock('@/hooks', () => ({
  useNodeInteractions: ({ selected }: { selected: boolean }) => ({
    isHovered: false,
    handleMouseEnter: jest.fn(),
    handleMouseLeave: jest.fn(),
    handleMouseDown: jest.fn(),
    handleMouseUp: jest.fn(),
    containerClassName: selected ? 'selected' : '',
    containerStyle: {},
  }),
}));

const defaultProps: CardShellProps = {
  colors: { primary: '#8b5cf6', secondary: '#6366f1' },
  selected: false,
  renderContent: ({ colors, selected, isHovered, width }) => (
    <div data-testid="content">
      <span data-testid="content-color">{colors.primary}</span>
      <span data-testid="content-selected">{String(selected)}</span>
      <span data-testid="content-hovered">{String(isHovered)}</span>
      <span data-testid="content-width">{width}</span>
    </div>
  ),
};

describe('CardShell', () => {
  describe('Rendering', () => {
    it('renders content via render props', () => {
      render(<CardShell {...defaultProps} />);

      expect(screen.getByTestId('content')).toBeInTheDocument();
      expect(screen.getByTestId('content-color')).toHaveTextContent('#8b5cf6');
    });

    it('passes correct context to renderContent', () => {
      render(<CardShell {...defaultProps} width={280} />);

      expect(screen.getByTestId('content-width')).toHaveTextContent('280');
      expect(screen.getByTestId('content-selected')).toHaveTextContent('false');
    });

    it('applies custom width', () => {
      const { container } = render(<CardShell {...defaultProps} width={320} />);

      // Inner card should have the width
      const innerCard = container.querySelector('[style*="width"]');
      expect(innerCard).toBeInTheDocument();
    });

    it('applies minHeight when provided', () => {
      const { container } = render(<CardShell {...defaultProps} minHeight={140} />);

      // Find the inner card div (has overflow-hidden class)
      const innerCard = container.querySelector('.overflow-hidden');
      expect(innerCard).toBeInTheDocument();
      expect(innerCard).toHaveStyle({ minHeight: '140px' });
    });
  });

  describe('Selection Effects', () => {
    it('shows SelectionPulseRing when selected and showPulseRing is true', () => {
      render(<CardShell {...defaultProps} selected={true} showPulseRing={true} />);

      expect(screen.getByTestId('pulse-ring')).toBeInTheDocument();
    });

    it('hides SelectionPulseRing when showPulseRing is false', () => {
      render(<CardShell {...defaultProps} selected={true} showPulseRing={false} />);

      expect(screen.queryByTestId('pulse-ring')).not.toBeInTheDocument();
    });

    it('shows GlassmorphismEffects when selected', () => {
      render(<CardShell {...defaultProps} selected={true} />);

      expect(screen.getByTestId('glassmorphism')).toBeInTheDocument();
    });

    it('hides GlassmorphismEffects when showGlassmorphism is false', () => {
      render(<CardShell {...defaultProps} selected={true} showGlassmorphism={false} />);

      expect(screen.queryByTestId('glassmorphism')).not.toBeInTheDocument();
    });
  });

  describe('Node Handles', () => {
    it('shows NodeHandles by default', () => {
      render(<CardShell {...defaultProps} />);

      expect(screen.getByTestId('node-handles')).toBeInTheDocument();
    });

    it('hides NodeHandles when showHandles is false', () => {
      render(<CardShell {...defaultProps} showHandles={false} />);

      expect(screen.queryByTestId('node-handles')).not.toBeInTheDocument();
    });

    it('passes vertical layout to NodeHandles', () => {
      render(<CardShell {...defaultProps} />);

      expect(screen.getByTestId('node-handles')).toHaveAttribute('data-layout', 'vertical');
    });
  });

  describe('Blueprint Overlay', () => {
    it('shows BlueprintOverlay when isMetaMode is true', () => {
      render(<CardShell {...defaultProps} isMetaMode={true} />);

      expect(screen.getByTestId('blueprint-overlay')).toBeInTheDocument();
    });

    it('hides BlueprintOverlay when isMetaMode is false', () => {
      render(<CardShell {...defaultProps} isMetaMode={false} />);

      expect(screen.queryByTestId('blueprint-overlay')).not.toBeInTheDocument();
    });

    it('can override BlueprintOverlay visibility', () => {
      render(<CardShell {...defaultProps} isMetaMode={true} showBlueprintOverlay={false} />);

      expect(screen.queryByTestId('blueprint-overlay')).not.toBeInTheDocument();
    });
  });

  describe('Default Props', () => {
    it('has sensible defaults for optional props', () => {
      render(<CardShell {...defaultProps} selected={true} isMetaMode={true} />);

      // Default showPulseRing = true
      expect(screen.getByTestId('pulse-ring')).toBeInTheDocument();

      // Default showHandles = true
      expect(screen.getByTestId('node-handles')).toBeInTheDocument();

      // Default showGlassmorphism = true
      expect(screen.getByTestId('glassmorphism')).toBeInTheDocument();

      // Default showBlueprintOverlay = true when isMetaMode
      expect(screen.getByTestId('blueprint-overlay')).toBeInTheDocument();
    });

    it('uses default width of 200 when not specified', () => {
      render(<CardShell {...defaultProps} />);

      expect(screen.getByTestId('content-width')).toHaveTextContent('200');
    });
  });

  describe('Gradient Border Styling', () => {
    it('applies gradient border based on selection state', () => {
      const { container, rerender } = render(<CardShell {...defaultProps} selected={false} />);

      // Check border wrapper exists
      const borderWrapper = container.querySelector('[class*="transition-all"]');
      expect(borderWrapper).toBeInTheDocument();

      // Rerender with selected
      rerender(<CardShell {...defaultProps} selected={true} />);

      // Should have animation class when selected
      const selectedBorder = container.querySelector('[class*="animate-gradient-rotate"]');
      expect(selectedBorder).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('applies aria-label when provided', () => {
      render(<CardShell {...defaultProps} ariaLabel="Project node: My Project" />);

      const container = screen.getByLabelText('Project node: My Project');
      expect(container).toBeInTheDocument();
    });
  });
});
