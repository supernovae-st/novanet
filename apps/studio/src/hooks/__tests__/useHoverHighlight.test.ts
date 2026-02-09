// src/hooks/__tests__/useHoverHighlight.test.ts
import { renderHook } from '@testing-library/react';
import { useHoverHighlight } from '../useHoverHighlight';
import { useUIStore } from '@/stores/uiStore';
import type { GraphEdge } from '@/types';

// Mock stores
jest.mock('@/stores/uiStore');

const mockUseUIStore = useUIStore as jest.MockedFunction<typeof useUIStore>;

describe('useHoverHighlight', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  const createMockEdges = (): GraphEdge[] => [
    { id: 'e1', source: 'node-a', target: 'node-b', type: 'HAS_BLOCK' },
    { id: 'e2', source: 'node-a', target: 'node-c', type: 'USES_ENTITY' },
    { id: 'e3', source: 'node-b', target: 'node-d', type: 'HAS_GENERATED' },
    { id: 'e4', source: 'node-c', target: 'node-e', type: 'SEMANTIC_LINK' },
  ];

  describe('when no node is hovered', () => {
    beforeEach(() => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: null, selectedNodeId: null };
        return selector(state as never);
      });
    });

    it('should not dim any nodes', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isNodeHoverDimmed('node-a')).toBe(false);
      expect(result.current.isNodeHoverDimmed('node-b')).toBe(false);
      expect(result.current.isNodeHoverDimmed('node-z')).toBe(false);
    });

    it('should not dim any edges', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHoverDimmed('node-a', 'node-b')).toBe(false);
      expect(result.current.isEdgeHoverDimmed('node-x', 'node-y')).toBe(false);
    });

    it('should not highlight any edges', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHighlighted('node-a', 'node-b')).toBe(false);
    });

    it('should have null hoveredId', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.hoveredId).toBeNull();
    });

    it('should have empty connectedIds', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.connectedIds.size).toBe(0);
    });
  });

  describe('when a node is hovered', () => {
    beforeEach(() => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: 'node-a', selectedNodeId: null };
        return selector(state as never);
      });
    });

    it('should not dim the hovered node', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isNodeHoverDimmed('node-a')).toBe(false);
    });

    it('should not dim directly connected nodes (1-hop)', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      // node-b and node-c are directly connected to node-a
      expect(result.current.isNodeHoverDimmed('node-b')).toBe(false);
      expect(result.current.isNodeHoverDimmed('node-c')).toBe(false);
    });

    it('should dim nodes not directly connected (2+ hops)', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      // node-d and node-e are 2 hops away from node-a
      expect(result.current.isNodeHoverDimmed('node-d')).toBe(true);
      expect(result.current.isNodeHoverDimmed('node-e')).toBe(true);
    });

    it('should dim nodes not in the graph', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isNodeHoverDimmed('node-z')).toBe(true);
    });

    it('should not dim edges connected to hovered node', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHoverDimmed('node-a', 'node-b')).toBe(false);
      expect(result.current.isEdgeHoverDimmed('node-a', 'node-c')).toBe(false);
      // Also works with reversed order (bidirectional)
      expect(result.current.isEdgeHoverDimmed('node-b', 'node-a')).toBe(false);
    });

    it('should dim edges not connected to hovered node', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHoverDimmed('node-b', 'node-d')).toBe(true);
      expect(result.current.isEdgeHoverDimmed('node-c', 'node-e')).toBe(true);
    });

    it('should highlight edges connected to hovered node', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHighlighted('node-a', 'node-b')).toBe(true);
      expect(result.current.isEdgeHighlighted('node-a', 'node-c')).toBe(true);
    });

    it('should have correct hoveredId', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.hoveredId).toBe('node-a');
    });

    it('should have correct connectedIds', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.connectedIds.has('node-b')).toBe(true);
      expect(result.current.connectedIds.has('node-c')).toBe(true);
      expect(result.current.connectedIds.has('node-d')).toBe(false);
    });
  });

  describe('when a node is selected (focus mode takes precedence)', () => {
    beforeEach(() => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: 'node-a', selectedNodeId: 'node-b' };
        return selector(state as never);
      });
    });

    it('should not dim any nodes (focus mode handles this)', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isNodeHoverDimmed('node-a')).toBe(false);
      expect(result.current.isNodeHoverDimmed('node-z')).toBe(false);
    });

    it('should not dim any edges (focus mode handles this)', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHoverDimmed('node-a', 'node-b')).toBe(false);
      expect(result.current.isEdgeHoverDimmed('node-x', 'node-y')).toBe(false);
    });

    it('should not highlight any edges (focus mode handles this)', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.isEdgeHighlighted('node-a', 'node-b')).toBe(false);
    });

    it('should suppress hoveredId when node is selected', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      // hoveredId should be null when a node is selected
      expect(result.current.hoveredId).toBeNull();
    });

    it('should have empty connectedIds when node is selected', () => {
      const edges = createMockEdges();
      const { result } = renderHook(() => useHoverHighlight(edges));

      expect(result.current.connectedIds.size).toBe(0);
    });
  });

  describe('edge cases', () => {
    it('should handle empty edges array', () => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: 'node-a', selectedNodeId: null };
        return selector(state as never);
      });

      const { result } = renderHook(() => useHoverHighlight([]));

      expect(result.current.connectedIds.size).toBe(0);
      expect(result.current.isNodeHoverDimmed('node-a')).toBe(false); // Hovered node itself
      expect(result.current.isNodeHoverDimmed('node-b')).toBe(true); // No connections
    });

    it('should handle self-loops correctly', () => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: 'node-a', selectedNodeId: null };
        return selector(state as never);
      });

      const edgesWithLoop: GraphEdge[] = [
        { id: 'e1', source: 'node-a', target: 'node-a', type: 'SEMANTIC_LINK' },
        { id: 'e2', source: 'node-a', target: 'node-b', type: 'HAS_BLOCK' },
      ];

      const { result } = renderHook(() => useHoverHighlight(edgesWithLoop));

      expect(result.current.connectedIds.has('node-a')).toBe(true);
      expect(result.current.connectedIds.has('node-b')).toBe(true);
    });

    it('should handle bidirectional edges', () => {
      mockUseUIStore.mockImplementation((selector) => {
        const state = { hoveredNodeId: 'node-a', selectedNodeId: null };
        return selector(state as never);
      });

      const bidirectionalEdges: GraphEdge[] = [
        { id: 'e1', source: 'node-a', target: 'node-b', type: 'SEMANTIC_LINK' },
        { id: 'e2', source: 'node-b', target: 'node-a', type: 'SEMANTIC_LINK' },
      ];

      const { result } = renderHook(() => useHoverHighlight(bidirectionalEdges));

      expect(result.current.connectedIds.has('node-b')).toBe(true);
    });
  });
});
