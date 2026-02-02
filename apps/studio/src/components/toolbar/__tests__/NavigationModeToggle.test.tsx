/**
 * NavigationModeToggle Tests
 *
 * Tests rendering of 4-mode segmented toggle, active state styling,
 * and click behavior triggering animationStore transitions.
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
  TooltipContent: ({ children }: { children: React.ReactNode }) => <span data-testid="tooltip">{children}</span>,
  TooltipShortcut: ({ children }: { children: React.ReactNode }) => <kbd>{children}</kbd>,
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
    it('renders all 4 mode buttons', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      expect(screen.getByText('Data')).toBeInTheDocument();
      expect(screen.getByText('Meta')).toBeInTheDocument();
      expect(screen.getByText('Overlay')).toBeInTheDocument();
      expect(screen.getByText('Query')).toBeInTheDocument();
    });

    it('renders icons as SVGs', () => {
      const { container } = render(
        <NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />
      );

      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBe(4); // One icon per mode
    });
  });

  // ==========================================================================
  // Active state
  // ==========================================================================

  describe('active state', () => {
    it('highlights data mode when active', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      const dataButton = screen.getByText('Data').closest('button');
      expect(dataButton?.className).toContain('emerald');
    });

    it('highlights meta mode when active', () => {
      render(<NavigationModeToggle mode="meta" onModeChange={mockOnModeChange} />);

      const metaButton = screen.getByText('Meta').closest('button');
      expect(metaButton?.className).toContain('blue');
    });

    it('highlights overlay mode when active', () => {
      render(<NavigationModeToggle mode="overlay" onModeChange={mockOnModeChange} />);

      const overlayButton = screen.getByText('Overlay').closest('button');
      expect(overlayButton?.className).toContain('violet');
    });

    it('highlights query mode when active', () => {
      render(<NavigationModeToggle mode="query" onModeChange={mockOnModeChange} />);

      const queryButton = screen.getByText('Query').closest('button');
      expect(queryButton?.className).toContain('amber');
    });

    it('inactive buttons have muted styling', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      const metaButton = screen.getByText('Meta').closest('button');
      expect(metaButton?.className).toContain('text-white/40');
    });
  });

  // ==========================================================================
  // Click behavior
  // ==========================================================================

  describe('click behavior', () => {
    it('triggers startTransition on click of different mode', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('Meta'));

      expect(mockStartTransition).toHaveBeenCalledWith('meta');
    });

    it('does not trigger transition when clicking active mode', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('Data'));

      expect(mockStartTransition).not.toHaveBeenCalled();
    });

    it('triggers correct mode for each button', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.click(screen.getByText('Overlay'));
      expect(mockStartTransition).toHaveBeenCalledWith('overlay');

      mockStartTransition.mockClear();

      fireEvent.click(screen.getByText('Query'));
      expect(mockStartTransition).toHaveBeenCalledWith('query');
    });
  });
});
