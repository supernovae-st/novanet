/**
 * NavigationModeToggle Tests
 *
 * Tests rendering of 4-mode segmented toggle, active state styling,
 * and click behavior triggering animationStore transitions.
 *
 * v9.5: Order is Meta | Data | Overlay | Query, labels are lowercase mono
 */

import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { NavigationModeToggle } from '../NavigationModeToggle';
import { useAnimationStore } from '@/stores/animationStore';

// Mock animationStore
jest.mock('@/stores/animationStore', () => ({
  useAnimationStore: jest.fn(),
}));

// Mock tooltip components to avoid Radix UI complexity in tests
jest.mock('@/components/ui/tooltip', () => ({
  Tooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
  TooltipTrigger: ({ children }: { children: React.ReactNode }) => <>{children}</>,
  TooltipContent: ({ children, sideOffset: _sideOffset }: { children: React.ReactNode; sideOffset?: number }) => (
    <span data-testid="tooltip">{children}</span>
  ),
  TooltipShortcut: ({ children }: { children: React.ReactNode }) => <kbd data-testid="shortcut">{children}</kbd>,
}));

const mockStartTransition = jest.fn();
const mockUseAnimationStore = useAnimationStore as jest.MockedFunction<typeof useAnimationStore>;

describe('NavigationModeToggle', () => {
  const mockOnModeChange = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
    mockUseAnimationStore.mockImplementation((selector) => {
      const state = { startTransition: mockStartTransition };
      return selector(state as never);
    });
  });

  // ==========================================================================
  // Rendering
  // ==========================================================================

  describe('rendering', () => {
    it('renders all 4 mode buttons in correct order', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      // v9.5: lowercase labels, Meta first
      expect(screen.getByText('meta')).toBeInTheDocument();
      expect(screen.getByText('data')).toBeInTheDocument();
      expect(screen.getByText('overlay')).toBeInTheDocument();
      expect(screen.getByText('query')).toBeInTheDocument();
    });

    it('renders icons as SVGs', () => {
      const { container } = render(
        <NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />
      );

      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBe(4);
    });

    it('uses monospace font for labels', () => {
      const { container } = render(
        <NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />
      );

      const buttons = container.querySelectorAll('button');
      buttons.forEach((btn) => {
        expect(btn.className).toContain('font-mono');
      });
    });

    it('shows shortcuts 1, 2, 3, 4 for each mode', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      const shortcuts = screen.getAllByTestId('shortcut');
      expect(shortcuts[0]).toHaveTextContent('1'); // meta
      expect(shortcuts[1]).toHaveTextContent('2'); // data
      expect(shortcuts[2]).toHaveTextContent('3'); // overlay
      expect(shortcuts[3]).toHaveTextContent('4'); // query
    });
  });

  // ==========================================================================
  // Active state
  // ==========================================================================

  describe('active state', () => {
    it('highlights data mode when active', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      const dataButton = screen.getByText('data').closest('button');
      expect(dataButton?.className).toContain('emerald');
    });

    it('highlights meta mode when active', () => {
      render(<NavigationModeToggle mode="meta" onModeChange={mockOnModeChange} />);

      const metaButton = screen.getByText('meta').closest('button');
      expect(metaButton?.className).toContain('blue');
    });

    it('highlights overlay mode when active', () => {
      render(<NavigationModeToggle mode="overlay" onModeChange={mockOnModeChange} />);

      const overlayButton = screen.getByText('overlay').closest('button');
      expect(overlayButton?.className).toContain('violet');
    });

    it('highlights query mode when active', () => {
      render(<NavigationModeToggle mode="query" onModeChange={mockOnModeChange} />);

      const queryButton = screen.getByText('query').closest('button');
      expect(queryButton?.className).toContain('amber');
    });

    it('inactive buttons have muted styling', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      const metaButton = screen.getByText('meta').closest('button');
      expect(metaButton?.className).toContain('text-white/30');
    });
  });

  // ==========================================================================
  // Click behavior
  // ==========================================================================

  describe('click behavior', () => {
    it('triggers startTransition on click of different mode', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('meta'));

      expect(mockStartTransition).toHaveBeenCalledWith('meta');
    });

    it('does not trigger transition when clicking active mode', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('data'));

      expect(mockStartTransition).not.toHaveBeenCalled();
    });

    it('triggers correct mode for each button', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('overlay'));
      expect(mockStartTransition).toHaveBeenCalledWith('overlay');

      mockStartTransition.mockClear();

      fireEvent.click(screen.getByText('query'));
      expect(mockStartTransition).toHaveBeenCalledWith('query');
    });
  });
});
