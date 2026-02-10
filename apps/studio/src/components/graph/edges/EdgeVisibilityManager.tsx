/**
 * EdgeVisibilityManager - Task 2.1 Performance Optimization
 *
 * Viewport culling system using IntersectionObserver to track which edges
 * are visible in the viewport. This enables selective rendering of expensive
 * effects (particles, glow, energy) only for visible edges.
 *
 * Part of Phase 2: Viewport Culling for Graph Performance Ultra-Optimization Plan.
 *
 * Architecture:
 * - Zustand store: Tracks Set<string> of visible edge IDs
 * - React Context: Provides IntersectionObserver and registration callbacks
 * - Combined hook: Merges store and context for edge components
 *
 * Performance Impact:
 * - Reduces active animations from 19,000 to 50-200 (viewport only)
 * - 100px rootMargin provides smooth transitions when scrolling
 * - O(1) visibility lookup via Set data structure
 */

'use client';

import {
  createContext,
  useContext,
  useCallback,
  useMemo,
  useRef,
  useEffect,
  type ReactNode,
} from 'react';
import { create } from 'zustand';

// =============================================================================
// ZUSTAND STORE
// =============================================================================

/** Performance mode thresholds */
export const PERF_THRESHOLDS = {
  /** Above this edge count, use simplified 2-element effects */
  SIMPLIFY_EFFECTS: 200,
  /** Above this edge count, disable animations entirely */
  DISABLE_ANIMATIONS: 500,
} as const;

interface EdgeVisibilityState {
  /** Set of edge IDs currently visible in the viewport */
  visibleEdges: Set<string>;

  /** Total edge count in the graph (for performance decisions) */
  totalEdgeCount: number;

  /** Whether to use simplified edge effects */
  useSimplifiedEffects: boolean;

  /** Whether to disable edge animations entirely */
  disableAnimations: boolean;

  /**
   * Update visibility status for an edge.
   * @param id - Edge identifier
   * @param visible - Whether the edge is visible in viewport
   */
  setVisible: (id: string, visible: boolean) => void;

  /**
   * Check if an edge is currently visible.
   * O(1) lookup using Set.has()
   * @param id - Edge identifier
   * @returns true if edge is in visible set
   */
  isVisible: (id: string) => boolean;

  /**
   * Update total edge count and recalculate performance flags.
   * @param count - Total number of edges in the graph
   */
  setTotalEdgeCount: (count: number) => void;

  /**
   * Clear all visibility tracking.
   * Used when unmounting or resetting the view.
   */
  clear: () => void;
}

export const useEdgeVisibilityStore = create<EdgeVisibilityState>((set, get) => ({
  visibleEdges: new Set(),
  totalEdgeCount: 0,
  useSimplifiedEffects: false,
  disableAnimations: false,

  setVisible: (id: string, visible: boolean) => {
    set(state => {
      const newSet = new Set(state.visibleEdges);
      if (visible) {
        newSet.add(id);
      } else {
        newSet.delete(id);
      }
      return { visibleEdges: newSet };
    });
  },

  isVisible: (id: string) => get().visibleEdges.has(id),

  setTotalEdgeCount: (count: number) => {
    set({
      totalEdgeCount: count,
      useSimplifiedEffects: count > PERF_THRESHOLDS.SIMPLIFY_EFFECTS,
      disableAnimations: count > PERF_THRESHOLDS.DISABLE_ANIMATIONS,
    });
  },

  clear: () => set({
    visibleEdges: new Set(),
    totalEdgeCount: 0,
    useSimplifiedEffects: false,
    disableAnimations: false,
  }),
}));

// =============================================================================
// REACT CONTEXT FOR INTERSECTION OBSERVER
// =============================================================================

interface EdgeVisibilityContextValue {
  /**
   * Register an edge element for visibility tracking.
   * @param id - Edge identifier
   * @param element - DOM element to observe
   */
  registerEdge: (id: string, element: Element) => void;

  /**
   * Unregister an edge element from visibility tracking.
   * @param id - Edge identifier
   * @param element - DOM element to stop observing
   */
  unregisterEdge: (id: string, element: Element) => void;
}

const EdgeVisibilityContext = createContext<EdgeVisibilityContextValue | null>(null);

// =============================================================================
// PROVIDER COMPONENT
// =============================================================================

interface EdgeVisibilityProviderProps {
  children: ReactNode;
}

/**
 * Provider component that creates and manages the IntersectionObserver.
 *
 * Features:
 * - Creates observer with 100px rootMargin for smooth transitions
 * - Maintains element -> edgeId mapping for callback resolution
 * - Cleans up observer on unmount
 *
 * @example
 * ```tsx
 * <EdgeVisibilityProvider>
 *   <ReactFlow>
 *     {edges.map(edge => <FloatingEdge key={edge.id} {...edge} />)}
 *   </ReactFlow>
 * </EdgeVisibilityProvider>
 * ```
 */
export function EdgeVisibilityProvider({ children }: EdgeVisibilityProviderProps) {
  const observerRef = useRef<IntersectionObserver | null>(null);
  const elementMapRef = useRef<Map<Element, string>>(new Map());
  const setVisible = useEdgeVisibilityStore(state => state.setVisible);

  // Create IntersectionObserver on mount
  useEffect(() => {
    observerRef.current = new IntersectionObserver(
      (entries) => {
        entries.forEach(entry => {
          const edgeId = elementMapRef.current.get(entry.target);
          if (edgeId) {
            setVisible(edgeId, entry.isIntersecting);
          }
        });
      },
      {
        // 100px buffer for smooth transitions when scrolling/panning
        rootMargin: '100px',
      }
    );

    return () => {
      observerRef.current?.disconnect();
    };
  }, [setVisible]);

  const registerEdge = useCallback((id: string, element: Element) => {
    elementMapRef.current.set(element, id);
    observerRef.current?.observe(element);
  }, []);

  const unregisterEdge = useCallback((id: string, element: Element) => {
    elementMapRef.current.delete(element);
    observerRef.current?.unobserve(element);
    // Mark as not visible when unregistered
    setVisible(id, false);
  }, [setVisible]);

  const value = useMemo(
    () => ({
      registerEdge,
      unregisterEdge,
    }),
    [registerEdge, unregisterEdge]
  );

  return (
    <EdgeVisibilityContext.Provider value={value}>
      {children}
    </EdgeVisibilityContext.Provider>
  );
}

// =============================================================================
// COMBINED HOOK
// =============================================================================

interface UseEdgeVisibilityReturn {
  /** Set of visible edge IDs (from store) */
  visibleEdges: Set<string>;
  /** Update visibility for an edge (from store) */
  setVisible: (id: string, visible: boolean) => void;
  /** Check if edge is visible (from store) */
  isVisible: (id: string) => boolean;
  /** Clear all visibility tracking (from store) */
  clear: () => void;
  /** Register edge for IntersectionObserver (from context) */
  registerEdge: (id: string, element: Element) => void;
  /** Unregister edge from IntersectionObserver (from context) */
  unregisterEdge: (id: string, element: Element) => void;
  /** Update total edge count (from store) */
  setTotalEdgeCount: (count: number) => void;
  /** Whether to use simplified 2-element effects (from store) */
  useSimplifiedEffects: boolean;
  /** Whether to disable animations entirely (from store) */
  disableAnimations: boolean;
}

/**
 * Combined hook providing both store methods and context callbacks.
 *
 * When used outside EdgeVisibilityProvider, registerEdge/unregisterEdge
 * are no-op functions to prevent errors.
 *
 * @example
 * ```tsx
 * function FloatingEdge({ id }: { id: string }) {
 *   const pathRef = useRef<SVGPathElement>(null);
 *   const { isVisible, registerEdge, unregisterEdge } = useEdgeVisibility();
 *
 *   useEffect(() => {
 *     const el = pathRef.current;
 *     if (el) {
 *       registerEdge(id, el);
 *       return () => unregisterEdge(id, el);
 *     }
 *   }, [id, registerEdge, unregisterEdge]);
 *
 *   const visible = isVisible(id);
 *   return visible ? <ExpensiveEffects /> : <SimplePath />;
 * }
 * ```
 */
export function useEdgeVisibility(): UseEdgeVisibilityReturn {
  const context = useContext(EdgeVisibilityContext);
  const store = useEdgeVisibilityStore();

  return {
    // From store
    visibleEdges: store.visibleEdges,
    setVisible: store.setVisible,
    isVisible: store.isVisible,
    clear: store.clear,
    setTotalEdgeCount: store.setTotalEdgeCount,
    useSimplifiedEffects: store.useSimplifiedEffects,
    disableAnimations: store.disableAnimations,
    // From context (with noop fallbacks)
    registerEdge: context?.registerEdge ?? noop,
    unregisterEdge: context?.unregisterEdge ?? noop,
  };
}

// No-op function for graceful fallback when used outside provider
 
function noop(_id: string, _element: Element): void {
  // Intentionally empty - allows hook to work without provider
}
