/**
 * useNavigationMode Tests
 *
 * v11.0: Tests simplified to Meta and Data modes only.
 * Tests boolean helpers, mode-aware fetch dispatch.
 * Mocks all stores + useGraphData to isolate hook logic.
 */

import { renderHook, act } from '@testing-library/react';
import { useNavigationMode } from '../useNavigationMode';
import { useUIStore } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
import { useGraphData } from '../useGraphData';

// Mock all dependencies — provide exported selectors for uiStore
jest.mock('@/stores/uiStore', () => ({
  useUIStore: jest.fn(),
  selectNavigationMode: (s: { navigationMode: string }) => s.navigationMode,
}));
jest.mock('@/stores/graphStore', () => ({
  useGraphStore: jest.fn(),
}));
jest.mock('../useGraphData');
jest.mock('@/config/constants', () => ({
  DEFAULT_FETCH_LIMIT: 500,
}));

const mockUseUIStore = useUIStore as jest.MockedFunction<typeof useUIStore>;
const mockUseGraphStore = useGraphStore as jest.MockedFunction<typeof useGraphStore>;
const mockUseGraphData = useGraphData as jest.MockedFunction<typeof useGraphData>;

const mockSetNavigationMode = jest.fn();
const mockCycleNavigationMode = jest.fn();
const mockFetchData = jest.fn();
const mockFetchSchemaData = jest.fn();
const mockClearGraph = jest.fn();

type NavigationMode = 'data' | 'meta';

function setupMocks(mode: NavigationMode) {
  const uiState = {
    navigationMode: mode,
    setNavigationMode: mockSetNavigationMode,
    cycleNavigationMode: mockCycleNavigationMode,
  };

  mockUseUIStore.mockImplementation((selector) => {
    return selector(uiState as never);
  });

  mockUseGraphStore.mockImplementation((selector) => {
    return selector({ clearGraph: mockClearGraph } as never);
  });

  mockUseGraphData.mockReturnValue({
    fetchData: mockFetchData,
    fetchSchemaData: mockFetchSchemaData,
  } as never);
}

describe('useNavigationMode', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  // ==========================================================================
  // Boolean helpers
  // ==========================================================================

  describe('boolean helpers', () => {
    it.each<[NavigationMode, boolean, boolean]>([
      //    mode       includesMeta  includesData
      ['data',         false,        true],
      ['meta',         true,         false],
    ])('mode=%s → includesMeta=%s, includesData=%s',
      (mode, expectedMeta, expectedData) => {
        setupMocks(mode);
        const { result } = renderHook(() => useNavigationMode());

        expect(result.current.includesMeta).toBe(expectedMeta);
        expect(result.current.includesData).toBe(expectedData);
      }
    );
  });

  // ==========================================================================
  // Mode passthrough
  // ==========================================================================

  describe('mode passthrough', () => {
    it('returns current mode from uiStore', () => {
      setupMocks('meta');
      const { result } = renderHook(() => useNavigationMode());
      expect(result.current.mode).toBe('meta');
    });

    it('exposes setMode from uiStore', () => {
      setupMocks('data');
      const { result } = renderHook(() => useNavigationMode());
      result.current.setMode('meta');
      expect(mockSetNavigationMode).toHaveBeenCalledWith('meta');
    });

    it('exposes cycleMode from uiStore', () => {
      setupMocks('data');
      const { result } = renderHook(() => useNavigationMode());
      result.current.cycleMode();
      expect(mockCycleNavigationMode).toHaveBeenCalled();
    });
  });

  // ==========================================================================
  // fetchForMode dispatch
  // ==========================================================================

  describe('fetchForMode', () => {
    it('meta mode calls fetchSchemaData', async () => {
      setupMocks('meta');
      mockFetchSchemaData.mockResolvedValue({ success: true, data: { nodes: [], edges: [] } });

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchSchemaData).toHaveBeenCalled();
      expect(mockClearGraph).not.toHaveBeenCalled();
    });

    it('data mode clears graph and returns empty data', async () => {
      setupMocks('data');

      const { result } = renderHook(() => useNavigationMode());
      let fetchResult: unknown;
      await act(async () => {
        fetchResult = await result.current.fetchForMode();
      });

      expect(mockClearGraph).toHaveBeenCalled();
      expect(mockFetchSchemaData).not.toHaveBeenCalled();
      expect((fetchResult as { success: boolean }).success).toBe(true);
      expect((fetchResult as { data: { nodes: unknown[] } }).data.nodes).toHaveLength(0);
    });
  });
});
