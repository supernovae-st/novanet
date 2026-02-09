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
      // Setup: Full adjacencyMap has concept connected to 10 L10N nodes
      const fullAdjacencyMap = new Map<string, Set<string>>([
        ['concept-1', new Set(['l10n-1', 'l10n-2', 'l10n-3', 'l10n-4', 'l10n-5', 'l10n-6', 'l10n-7', 'l10n-8', 'l10n-9', 'l10n-10'])],
        ['l10n-1', new Set(['concept-1'])],
        ['l10n-2', new Set(['concept-1'])],
        // ... others would connect back
      ]);

      // But filtered edges only show 2 connections (e.g., only fr-FR and en-US visible)
      const filteredEdges: GraphEdge[] = [
        { id: 'e1', source: 'concept-1', target: 'l10n-1', type: 'HAS_CONTENT' },
        { id: 'e2', source: 'concept-1', target: 'l10n-2', type: 'HAS_CONTENT' },
      ];

      mockUseGraphStore.mockImplementation((selector) => {
        const state = { adjacencyMap: fullAdjacencyMap };
        return selector(state as never);
      });

      mockUseUIStore.mockImplementation((selector) => {
        const state = { selectedNodeId: 'concept-1' };
        return selector(state as never);
      });

      const { result } = renderHook(() => useFocusMode(filteredEdges));

      // Only l10n-1 and l10n-2 should be connected (from filtered edges)
      expect(result.current.connectedIds.has('l10n-1')).toBe(true);
      expect(result.current.connectedIds.has('l10n-2')).toBe(true);

      // l10n-3 through l10n-10 should NOT be connected (not in filtered edges)
      expect(result.current.connectedIds.has('l10n-3')).toBe(false);
      expect(result.current.connectedIds.has('l10n-10')).toBe(false);

      // Dimming should work correctly
      expect(result.current.isNodeDimmed('l10n-1')).toBe(false); // Connected
      expect(result.current.isNodeDimmed('l10n-3')).toBe(true);  // Not connected
    });
  });
});
