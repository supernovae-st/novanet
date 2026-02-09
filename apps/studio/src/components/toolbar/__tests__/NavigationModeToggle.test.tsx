/**
 * NavigationModeToggle Tests
 *
 * v11.0: Simplified to Meta and Data modes only.
 * Tests rendering, active state styling, keyboard shortcuts (1-2),
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
    it('renders only meta and data mode buttons in correct order', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      expect(screen.getByText('meta')).toBeInTheDocument();
      expect(screen.getByText('data')).toBeInTheDocument();
      expect(screen.queryByText('overlay')).not.toBeInTheDocument();
      expect(screen.queryByText('query')).not.toBeInTheDocument();
    });

    it('renders icons as SVGs', () => {
      const { container } = render(
        <NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />
      );

      const svgs = container.querySelectorAll('svg');
      expect(svgs.length).toBe(2);
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

    it('shows kbd shortcuts 1, 2 inline', () => {
      const { container } = render(
        <NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />
      );

      const kbds = container.querySelectorAll('kbd');
      expect(kbds).toHaveLength(2);
      expect(kbds[0]).toHaveTextContent('1');
      expect(kbds[1]).toHaveTextContent('2');
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
  });

  // ==========================================================================
  // Keyboard shortcuts
  // ==========================================================================

  describe('keyboard shortcuts', () => {
    it('switches to meta on pressing 1', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.keyDown(window, { key: '1' });

      expect(mockStartTransition).toHaveBeenCalledWith('meta');
    });

    it('switches to data on pressing 2', () => {
      render(<NavigationModeToggle mode="meta" onModeChange={mockOnModeChange} />);

      fireEvent.keyDown(window, { key: '2' });

      expect(mockStartTransition).toHaveBeenCalledWith('data');
    });

    it('does not switch when already on target mode', () => {
      render(<NavigationModeToggle mode="meta" onModeChange={mockOnModeChange} />);

      fireEvent.keyDown(window, { key: '1' });

      expect(mockStartTransition).not.toHaveBeenCalled();
    });

    it('ignores keys 3 and 4 (removed modes)', () => {
      render(<NavigationModeToggle mode="data" onModeChange={mockOnModeChange} />);

      fireEvent.keyDown(window, { key: '3' });
      fireEvent.keyDown(window, { key: '4' });

      expect(mockStartTransition).not.toHaveBeenCalled();
    });
  });
});
