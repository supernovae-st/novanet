/**
 * EdgeVisibilityManager Tests - Task 2.1 Performance Optimization
 *
 * Tests for viewport culling system using IntersectionObserver.
 * Part of Phase 2: Viewport Culling for Graph Performance Ultra-Optimization Plan.
 *
 * This system tracks which edges are visible in the viewport to enable
 * selective rendering of expensive effects only for visible edges.
 */

import { renderHook, act } from '@testing-library/react';
import type { ReactNode } from 'react';

// Mock IntersectionObserver before importing the module
const mockObserve = jest.fn();
const mockUnobserve = jest.fn();
const mockDisconnect = jest.fn();

let intersectionCallback: IntersectionObserverCallback | null = null;

beforeAll(() => {
  global.IntersectionObserver = jest.fn().mockImplementation((callback) => {
    intersectionCallback = callback;
    return {
      observe: mockObserve,
      unobserve: mockUnobserve,
      disconnect: mockDisconnect,
    };
  });
});

// Import after mock is set up
import {
  useEdgeVisibility,
  useEdgeVisibilityStore,
  EdgeVisibilityProvider,
} from '../EdgeVisibilityManager';

describe('EdgeVisibilityManager', () => {
  beforeEach(() => {
    // Reset store state
    useEdgeVisibilityStore.getState().clear();
    // Reset mocks
    mockObserve.mockClear();
    mockUnobserve.mockClear();
    mockDisconnect.mockClear();
    intersectionCallback = null;
  });

  describe('useEdgeVisibilityStore', () => {
    it('should start with empty visibleEdges set', () => {
      const state = useEdgeVisibilityStore.getState();
      expect(state.visibleEdges.size).toBe(0);
    });

    it('should track visible edges via setVisible', () => {
      const store = useEdgeVisibilityStore.getState();

      act(() => {
        store.setVisible('edge-1', true);
        store.setVisible('edge-2', true);
      });

      // Re-get state after mutations
      const updatedState = useEdgeVisibilityStore.getState();
      expect(updatedState.isVisible('edge-1')).toBe(true);
      expect(updatedState.isVisible('edge-2')).toBe(true);
      expect(updatedState.isVisible('edge-3')).toBe(false);
    });

    it('should remove edge from visible set when setVisible(false)', () => {
      const store = useEdgeVisibilityStore.getState();

      act(() => {
        store.setVisible('edge-1', true);
      });

      expect(useEdgeVisibilityStore.getState().isVisible('edge-1')).toBe(true);

      act(() => {
        store.setVisible('edge-1', false);
      });

      expect(useEdgeVisibilityStore.getState().isVisible('edge-1')).toBe(false);
    });

    it('should clear all visible edges', () => {
      const store = useEdgeVisibilityStore.getState();

      act(() => {
        store.setVisible('edge-1', true);
        store.setVisible('edge-2', true);
        store.setVisible('edge-3', true);
      });

      expect(useEdgeVisibilityStore.getState().visibleEdges.size).toBe(3);

      act(() => {
        store.clear();
      });

      expect(useEdgeVisibilityStore.getState().visibleEdges.size).toBe(0);
    });

    it('should handle duplicate setVisible calls gracefully', () => {
      const store = useEdgeVisibilityStore.getState();

      act(() => {
        store.setVisible('edge-1', true);
        store.setVisible('edge-1', true);
        store.setVisible('edge-1', true);
      });

      expect(useEdgeVisibilityStore.getState().visibleEdges.size).toBe(1);
      expect(useEdgeVisibilityStore.getState().isVisible('edge-1')).toBe(true);
    });
  });

  describe('useEdgeVisibility hook', () => {
    const wrapper = ({ children }: { children: ReactNode }) => (
      <EdgeVisibilityProvider>{children}</EdgeVisibilityProvider>
    );

    it('should provide setVisible and isVisible from store', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });

      act(() => {
        result.current.setVisible('edge-1', true);
        result.current.setVisible('edge-2', true);
      });

      expect(result.current.isVisible('edge-1')).toBe(true);
      expect(result.current.isVisible('edge-2')).toBe(true);
      expect(result.current.isVisible('edge-3')).toBe(false);
    });

    it('should remove edge from visible set', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });

      act(() => {
        result.current.setVisible('edge-1', true);
        result.current.setVisible('edge-1', false);
      });

      expect(result.current.isVisible('edge-1')).toBe(false);
    });

    it('should provide registerEdge and unregisterEdge functions', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });

      expect(typeof result.current.registerEdge).toBe('function');
      expect(typeof result.current.unregisterEdge).toBe('function');
    });

    it('should provide clear function', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });

      act(() => {
        result.current.setVisible('edge-1', true);
        result.current.setVisible('edge-2', true);
      });

      expect(result.current.isVisible('edge-1')).toBe(true);

      act(() => {
        result.current.clear();
      });

      expect(result.current.isVisible('edge-1')).toBe(false);
      expect(result.current.isVisible('edge-2')).toBe(false);
    });
  });

  describe('EdgeVisibilityProvider', () => {
    const wrapper = ({ children }: { children: ReactNode }) => (
      <EdgeVisibilityProvider>{children}</EdgeVisibilityProvider>
    );

    it('should create IntersectionObserver with 100px rootMargin', () => {
      renderHook(() => useEdgeVisibility(), { wrapper });

      expect(global.IntersectionObserver).toHaveBeenCalledWith(
        expect.any(Function),
        expect.objectContaining({ rootMargin: '100px' })
      );
    });

    it('should observe element when registerEdge is called', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });
      const mockElement = document.createElement('div');

      act(() => {
        result.current.registerEdge('edge-1', mockElement);
      });

      expect(mockObserve).toHaveBeenCalledWith(mockElement);
    });

    it('should unobserve element when unregisterEdge is called', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });
      const mockElement = document.createElement('div');

      act(() => {
        result.current.registerEdge('edge-1', mockElement);
        result.current.unregisterEdge('edge-1', mockElement);
      });

      expect(mockUnobserve).toHaveBeenCalledWith(mockElement);
    });

    it('should update visibility when IntersectionObserver fires', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });
      const mockElement = document.createElement('div');

      act(() => {
        result.current.registerEdge('edge-1', mockElement);
      });

      // Simulate intersection callback
      act(() => {
        if (intersectionCallback) {
          intersectionCallback(
            [
              {
                target: mockElement,
                isIntersecting: true,
                intersectionRatio: 1,
                boundingClientRect: {} as DOMRectReadOnly,
                intersectionRect: {} as DOMRectReadOnly,
                rootBounds: null,
                time: 0,
              },
            ],
            {} as IntersectionObserver
          );
        }
      });

      expect(result.current.isVisible('edge-1')).toBe(true);
    });

    it('should set visibility to false when element leaves viewport', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });
      const mockElement = document.createElement('div');

      act(() => {
        result.current.registerEdge('edge-1', mockElement);
      });

      // First, make it visible
      act(() => {
        if (intersectionCallback) {
          intersectionCallback(
            [
              {
                target: mockElement,
                isIntersecting: true,
                intersectionRatio: 1,
                boundingClientRect: {} as DOMRectReadOnly,
                intersectionRect: {} as DOMRectReadOnly,
                rootBounds: null,
                time: 0,
              },
            ],
            {} as IntersectionObserver
          );
        }
      });

      expect(result.current.isVisible('edge-1')).toBe(true);

      // Then, make it not visible
      act(() => {
        if (intersectionCallback) {
          intersectionCallback(
            [
              {
                target: mockElement,
                isIntersecting: false,
                intersectionRatio: 0,
                boundingClientRect: {} as DOMRectReadOnly,
                intersectionRect: {} as DOMRectReadOnly,
                rootBounds: null,
                time: 0,
              },
            ],
            {} as IntersectionObserver
          );
        }
      });

      expect(result.current.isVisible('edge-1')).toBe(false);
    });

    it('should set visibility to false when unregisterEdge is called', () => {
      const { result } = renderHook(() => useEdgeVisibility(), { wrapper });
      const mockElement = document.createElement('div');

      act(() => {
        result.current.registerEdge('edge-1', mockElement);
      });

      // Make it visible
      act(() => {
        if (intersectionCallback) {
          intersectionCallback(
            [
              {
                target: mockElement,
                isIntersecting: true,
                intersectionRatio: 1,
                boundingClientRect: {} as DOMRectReadOnly,
                intersectionRect: {} as DOMRectReadOnly,
                rootBounds: null,
                time: 0,
              },
            ],
            {} as IntersectionObserver
          );
        }
      });

      expect(result.current.isVisible('edge-1')).toBe(true);

      // Unregister
      act(() => {
        result.current.unregisterEdge('edge-1', mockElement);
      });

      expect(result.current.isVisible('edge-1')).toBe(false);
    });
  });

  describe('hook without provider (graceful fallback)', () => {
    it('should provide noop functions for registerEdge/unregisterEdge without provider', () => {
      // Render without provider
      const { result } = renderHook(() => useEdgeVisibility());

      // Should not throw
      expect(() => {
        result.current.registerEdge('edge-1', document.createElement('div'));
        result.current.unregisterEdge('edge-1', document.createElement('div'));
      }).not.toThrow();
    });

    it('should still provide store methods without provider', () => {
      const { result } = renderHook(() => useEdgeVisibility());

      act(() => {
        result.current.setVisible('edge-1', true);
      });

      expect(result.current.isVisible('edge-1')).toBe(true);
    });
  });
});
