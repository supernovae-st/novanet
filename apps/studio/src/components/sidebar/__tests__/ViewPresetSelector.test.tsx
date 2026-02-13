// src/components/sidebar/__tests__/ViewPresetSelector.test.tsx
import '@testing-library/jest-dom';
import { render, screen, fireEvent } from '@testing-library/react';
import { ViewPresetSelector } from '../ViewPresetSelector';
import { useFilterStore } from '@/stores/filterStore';

// Mock the filterStore
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));

const mockApplyViewPreset = jest.fn();
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;

// JSDOM doesn't implement scrollIntoView - mock for useGridNavigation
beforeAll(() => {
  Element.prototype.scrollIntoView = jest.fn();
});

describe('ViewPresetSelector', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockUseFilterStore.mockImplementation((selector) => {
      const state = {
        applyViewPreset: mockApplyViewPreset,
        activePresetId: null,
      };
      return selector ? selector(state as never) : state;
    });
  });

  it('renders all 9 VIEW_PRESETS', () => {
    render(<ViewPresetSelector />);
    // Component displays first word of each preset name for compact display
    expect(screen.getByText('Project')).toBeInTheDocument();    // Project Structure
    expect(screen.getByText('Generation')).toBeInTheDocument(); // Generation Chain
    expect(screen.getByText('Locale')).toBeInTheDocument();     // Locale Knowledge
    expect(screen.getByText('Entity')).toBeInTheDocument();     // Entity Network (v10.4)
    expect(screen.getByText('Prompts')).toBeInTheDocument();    // Prompts & Rules
    expect(screen.getByText('SEO')).toBeInTheDocument();        // SEO Keywords (v10.4)
    expect(screen.getByText('Defined')).toBeInTheDocument();     // Defined Types (v11.8: was Invariant)
    expect(screen.getByText('Authored')).toBeInTheDocument();   // Authored Content (v11.8: was Localized)
    expect(screen.getByText('All')).toBeInTheDocument();        // All Nodes
  });

  it('renders keyboard shortcut badges', () => {
    render(<ViewPresetSelector />);
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
    expect(screen.getByText('0')).toBeInTheDocument();
  });

  it('calls onSelect when preset is clicked', () => {
    const onSelect = jest.fn();
    render(<ViewPresetSelector onSelect={onSelect} />);
    fireEvent.click(screen.getByText('Project'));
    expect(onSelect).toHaveBeenCalledWith('project-structure');
  });

  it('calls applyViewPreset from store when preset is clicked', () => {
    render(<ViewPresetSelector />);
    fireEvent.click(screen.getByText('Project'));
    expect(mockApplyViewPreset).toHaveBeenCalledWith('project-structure');
  });

  it('shows active preset with visual indicator', () => {
    render(<ViewPresetSelector activePresetId="locale-knowledge" />);
    const activeItem = screen.getByText('Locale').closest('button');
    expect(activeItem).toHaveClass('bg-white/[0.1]');
  });

  it('uses activePresetId from store when not provided as prop', () => {
    mockUseFilterStore.mockImplementation((selector) => {
      const state = {
        applyViewPreset: mockApplyViewPreset,
        activePresetId: 'entity-network',
      };
      return selector ? selector(state as never) : state;
    });

    render(<ViewPresetSelector />);
    const activeItem = screen.getByText('Entity').closest('button');
    expect(activeItem).toHaveClass('bg-white/[0.1]');
  });

  it('renders with Quick Views header', () => {
    render(<ViewPresetSelector />);
    expect(screen.getByText('Quick Views')).toBeInTheDocument();
  });

  it('renders in 3-column grid layout', () => {
    const { container } = render(<ViewPresetSelector />);
    const grid = container.querySelector('.grid-cols-3');
    expect(grid).toBeInTheDocument();
  });
});
