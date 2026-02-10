// src/hooks/__tests__/useFocusMode.test.ts
import { renderHook } from '@testing-library/react';
import { useFocusMode } from '../useFocusMode';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

// Mock stores
jest.mock('@/stores/graphStore');
jest.mock('@/stores/uiStore');

const mockUseGraphStore = useGraphStore as jest.MockedFunction<typeof useGraphStore>;
const mockUseUIStore = useUIStore as jest.MockedFunction<typeof useUIStore>;

describe('useFocusMode', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('with filtered edges', () => {
    it('should only consider connections from filtered edges, not full adjacencyMap', () => {
      // Setup: Full adjacencyMap has entity connected to 10 content nodes
      const fullAdjacencyMap = new Map<string, Set<string>>([
        ['entity-1', new Set(['content-1', 'content-2', 'content-3', 'content-4', 'content-5', 'content-6', 'content-7', 'content-8', 'content-9', 'content-10'])],
        ['content-1', new Set(['entity-1'])],
        ['content-2', new Set(['entity-1'])],
        // ... others would connect back
      ]);

      // But filtered edges only show 2 connections (e.g., only fr-FR and en-US visible)
      const filteredEdges: GraphEdge[] = [
        { id: 'e1', source: 'entity-1', target: 'content-1', type: 'HAS_CONTENT' },
        { id: 'e2', source: 'entity-1', target: 'content-2', type: 'HAS_CONTENT' },
      ];

      mockUseGraphStore.mockImplementation((selector) => {
        const state = { adjacencyMap: fullAdjacencyMap };
        return selector(state as never);
      });

      mockUseUIStore.mockImplementation((selector) => {
        const state = { selectedNodeId: 'entity-1' };
        return selector(state as never);
      });

      const { result } = renderHook(() => useFocusMode(filteredEdges));

      // Only content-1 and content-2 should be connected (from filtered edges)
      expect(result.current.connectedIds.has('content-1')).toBe(true);
      expect(result.current.connectedIds.has('content-2')).toBe(true);

      // content-3 through content-10 should NOT be connected (not in filtered edges)
      expect(result.current.connectedIds.has('content-3')).toBe(false);
      expect(result.current.connectedIds.has('content-10')).toBe(false);

      // Dimming should work correctly
      expect(result.current.isNodeDimmed('content-1')).toBe(false); // Connected
      expect(result.current.isNodeDimmed('content-3')).toBe(true);  // Not connected
    });
  });
});
