/**
 * useNavigationMode Tests
 *
 * Tests boolean helpers, mode-aware fetch dispatch, and facet param building.
 * Mocks all stores + useGraphData to isolate hook logic.
 */

import { renderHook, act } from '@testing-library/react';
import { useNavigationMode } from '../useNavigationMode';
import { useUIStore } from '@/stores/uiStore';
import { useFilterStore } from '@/stores/filterStore';
import { useGraphStore } from '@/stores/graphStore';
import { useGraphData } from '../useGraphData';
import { generateSchemaGraph } from '@/lib/schemaGenerator';
import { fetchJSON } from '@/lib/fetchClient';

// Mock all dependencies — provide exported selectors for uiStore
jest.mock('@/stores/uiStore', () => ({
  useUIStore: jest.fn(),
  selectNavigationMode: (s: { navigationMode: string }) => s.navigationMode,
}));
jest.mock('@/stores/filterStore', () => ({
  useFilterStore: jest.fn(),
}));
jest.mock('@/stores/graphStore', () => ({
  useGraphStore: jest.fn(),
}));
jest.mock('../useGraphData');
jest.mock('@/lib/schemaGenerator');
jest.mock('@/lib/fetchClient', () => ({
  fetchJSON: jest.fn(),
  getErrorMessage: (err: unknown) =>
    err instanceof Error ? err.message : String(err),
}));
jest.mock('@/lib/logger', () => ({
  logger: { warn: jest.fn(), info: jest.fn(), error: jest.fn() },
}));
jest.mock('@/config/constants', () => ({
  DEFAULT_FETCH_LIMIT: 500,
}));

const mockUseUIStore = useUIStore as jest.MockedFunction<typeof useUIStore>;
const mockUseFilterStore = useFilterStore as jest.MockedFunction<typeof useFilterStore>;
const mockUseGraphStore = useGraphStore as jest.MockedFunction<typeof useGraphStore>;
const mockUseGraphData = useGraphData as jest.MockedFunction<typeof useGraphData>;
const mockGenerateSchemaGraph = generateSchemaGraph as jest.MockedFunction<typeof generateSchemaGraph>;
const mockFetchJSON = fetchJSON as jest.MockedFunction<typeof fetchJSON>;

const mockSetNavigationMode = jest.fn();
const mockCycleNavigationMode = jest.fn();
const mockFetchData = jest.fn();
const mockFetchSchemaData = jest.fn();
const mockSetGraphData = jest.fn();

type NavigationMode = 'data' | 'meta' | 'overlay' | 'query';

function setupMocks(mode: NavigationMode, filters: Record<string, string[]> = {}) {
  const uiState = {
    navigationMode: mode,
    setNavigationMode: mockSetNavigationMode,
    cycleNavigationMode: mockCycleNavigationMode,
  };

  const filterState = {
    realmFilter: filters.realmFilter ?? [],
    traitFilter: filters.traitFilter ?? [],
    layerFilter: filters.layerFilter ?? [],
    edgeFamilyFilter: filters.edgeFamilyFilter ?? [],
  };

  mockUseUIStore.mockImplementation((selector) => {
    return selector(uiState as never);
  });

  mockUseFilterStore.mockImplementation((selector) => {
    return selector(filterState as never);
  });

  mockUseGraphStore.mockImplementation((selector) => {
    return selector({ setGraphData: mockSetGraphData } as never);
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
    it.each<[NavigationMode, boolean, boolean, boolean]>([
      //    mode       includesMeta  includesData  usesFacets
      ['data',         false,        true,         false],
      ['meta',         true,         false,        false],
      ['overlay',      true,         true,         false],
      ['query',        false,        true,         true],
    ])('mode=%s → includesMeta=%s, includesData=%s, usesFacets=%s',
      (mode, expectedMeta, expectedData, expectedFacets) => {
        setupMocks(mode);
        const { result } = renderHook(() => useNavigationMode());

        expect(result.current.includesMeta).toBe(expectedMeta);
        expect(result.current.includesData).toBe(expectedData);
        expect(result.current.usesFacets).toBe(expectedFacets);
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
      result.current.setMode('query');
      expect(mockSetNavigationMode).toHaveBeenCalledWith('query');
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
    it('data mode calls fetchData', async () => {
      setupMocks('data');
      mockFetchData.mockResolvedValue({ success: true, data: { nodes: [], edges: [] } });

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchData).toHaveBeenCalledWith({ limit: 500 });
      expect(mockFetchSchemaData).not.toHaveBeenCalled();
    });

    it('meta mode calls fetchSchemaData', async () => {
      setupMocks('meta');
      mockFetchSchemaData.mockResolvedValue({ success: true, data: { nodes: [], edges: [] } });

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchSchemaData).toHaveBeenCalled();
      expect(mockFetchData).not.toHaveBeenCalled();
    });

    it('overlay mode fetches data then merges schema', async () => {
      setupMocks('overlay');
      mockFetchData.mockResolvedValue({
        success: true,
        data: {
          nodes: [{ id: 'n1', label: 'Project' }],
          edges: [{ id: 'e1', source: 'n1', target: 'n2' }],
        },
      });
      mockGenerateSchemaGraph.mockReturnValue({
        nodes: [{ id: 'schema-1', label: 'Kind' }],
        edges: [{ id: 'se-1', source: 'schema-1', target: 'schema-2' }],
      } as never);

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchData).toHaveBeenCalled();
      expect(mockGenerateSchemaGraph).toHaveBeenCalled();
      expect(mockSetGraphData).toHaveBeenCalledWith({
        nodes: expect.arrayContaining([
          expect.objectContaining({ id: 'n1' }),
          expect.objectContaining({ id: 'schema-1', isMetaMode: true }),
        ]),
        edges: expect.arrayContaining([
          expect.objectContaining({ id: 'e1' }),
          expect.objectContaining({ id: 'se-1' }),
        ]),
      });
    });

    it('overlay mode still returns data result even if schema merge fails', async () => {
      setupMocks('overlay');
      mockFetchData.mockResolvedValue({
        success: true,
        data: { nodes: [], edges: [] },
      });
      mockGenerateSchemaGraph.mockImplementation(() => {
        throw new Error('schema gen failed');
      });

      const { result } = renderHook(() => useNavigationMode());
      let fetchResult: unknown;
      await act(async () => {
        fetchResult = await result.current.fetchForMode();
      });

      expect((fetchResult as { success: boolean }).success).toBe(true);
    });

    it('query mode calls fetchJSON with facet params', async () => {
      setupMocks('query', {
        realmFilter: ['global', 'project'],
        layerFilter: ['semantic'],
        traitFilter: ['localized'],
        edgeFamilyFilter: [],
      });
      mockFetchJSON.mockResolvedValue({
        success: true,
        data: { nodes: [], edges: [] },
      });

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchJSON).toHaveBeenCalledTimes(1);
      const url = mockFetchJSON.mock.calls[0][0] as string;
      expect(url).toContain('/api/graph/navigation');
      expect(url).toContain('realms=global');
      expect(url).toContain('layers=semantic');
      expect(url).toContain('traits=localized');
    });

    it('query mode omits empty facet params', async () => {
      setupMocks('query', {
        realmFilter: ['shared'],
        layerFilter: [],
        traitFilter: [],
        edgeFamilyFilter: [],
      });
      mockFetchJSON.mockResolvedValue({ success: true, data: { nodes: [], edges: [] } });

      const { result } = renderHook(() => useNavigationMode());
      await act(async () => {
        await result.current.fetchForMode();
      });

      expect(mockFetchJSON).toHaveBeenCalledTimes(1);
      const url = mockFetchJSON.mock.calls[0][0] as string;
      expect(url).toContain('realms=shared');
      expect(url).not.toContain('layers=');
      expect(url).not.toContain('traits=');
    });

    it('query mode returns error on fetch failure', async () => {
      setupMocks('query');
      mockFetchJSON.mockRejectedValue(new Error('Network error'));

      const { result } = renderHook(() => useNavigationMode());
      let fetchResult: unknown;
      await act(async () => {
        fetchResult = await result.current.fetchForMode();
      });

      expect((fetchResult as { success: boolean; error: string }).success).toBe(false);
      expect((fetchResult as { error: string }).error).toBe('Network error');
    });
  });
});
