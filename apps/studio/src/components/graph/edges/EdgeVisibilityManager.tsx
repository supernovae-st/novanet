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

/**
 * Effect quality tiers based on edge count.
 *
 * ADAPTIVE DENSITY: More edges = simpler effects for performance.
 * Fewer edges = richer effects for "wow" factor.
 */
export type EffectTier = 'ultra' | 'high' | 'medium' | 'low' | 'minimal';

/**
 * Performance thresholds for progressive effect density.
 *
 * v11.6.4 PERFORMANCE OPTIMIZATION:
 * - ULTRA:   0-15 arcs   → Full signature effects (PowerConduit, DNAHelix, etc.)
 * - HIGH:    15-40 arcs  → Full signature effects
 * - MEDIUM:  40-80 arcs  → Simplified 2-element effect (no signature)
 * - LOW:     80-150 arcs → Simplified 2-element effect
 * - MINIMAL: 150+ arcs   → No animation at all
 *
 * Key changes from v11.6.3:
 * - MEDIUM tier now uses SimplifiedEdgeEffect (was signature effects)
 * - SimplifiedEdgeEffect reduced to 2 elements (was 7)
 * - Selection effect simplified for large graphs (no blur)
 * - Thresholds raised to keep animations for medium-sized graphs
 */
export const PERF_THRESHOLDS = {
  /** Below this: ULTRA tier (full signature effects) */
  ULTRA_MAX: 15,
  /** Below this: HIGH tier (full signature effects) */
  HIGH_MAX: 40,
  /** Below this: MEDIUM tier (simplified effects) */
  MEDIUM_MAX: 80,
  /** Below this: LOW tier (simplified effects) */
  LOW_MAX: 150,
  // Above LOW_MAX: MINIMAL tier (no animations)
  /** Hub node threshold: nodes with more visible connections force LOW tier */
  HUB_NODE_THRESHOLD: 6,
  /** Super hub threshold: nodes with many connections force MINIMAL tier (no animations) */
  SUPER_HUB_THRESHOLD: 12,
} as const;

/**
 * Calculate effect tier from edge count.
 * More edges = lower tier = simpler effects.
 */
export function calculateEffectTier(edgeCount: number): EffectTier {
  if (edgeCount <= PERF_THRESHOLDS.ULTRA_MAX) return 'ultra';
  if (edgeCount <= PERF_THRESHOLDS.HIGH_MAX) return 'high';
  if (edgeCount <= PERF_THRESHOLDS.MEDIUM_MAX) return 'medium';
  if (edgeCount <= PERF_THRESHOLDS.LOW_MAX) return 'low';
  return 'minimal';
}

/** Edge metadata for connection tracking */
interface EdgeMeta {
  source: string;
  target: string;
}

interface EdgeVisibilityState {
  /** Set of edge IDs currently visible in the viewport */
  visibleEdges: Set<string>;

  /** Edge metadata (source, target) for connection tracking */
  edgeMeta: Map<string, EdgeMeta>;

  /** Count of visible connections per node */
  nodeConnectionCount: Map<string, number>;

  /** Total edge count in the graph (for performance decisions) */
  totalEdgeCount: number;

  /** Current effect quality tier */
  effectTier: EffectTier;

  /** Whether to use simplified edge effects (low tier) */
  useSimplifiedEffects: boolean;

  /** Whether to disable edge animations entirely (minimal tier) */
  disableAnimations: boolean;

  /**
   * Register edge metadata (source, target) for connection tracking.
   * Call this once when the edge component mounts.
   */
  registerEdgeMeta: (id: string, source: string, target: string) => void;

  /**
   * Update visibility status for an edge.
   * Also updates node connection counts for hub detection.
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
   * Check if a node is a "hub" with too many visible connections.
   * Hub nodes force simplified effects for performance.
   * @param nodeId - Node identifier
   * @returns true if node has >= HUB_NODE_THRESHOLD visible connections
   */
  isHubNode: (nodeId: string) => boolean;

  /**
   * Get the effective tier for an edge, considering hub nodes.
   * If either source or target is a hub, returns 'low'.
   * @param edgeId - Edge identifier
   * @returns effective tier (may be downgraded for hub connections)
   */
  getEffectiveTier: (edgeId: string) => EffectTier;

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
  edgeMeta: new Map(),
  nodeConnectionCount: new Map(),
  totalEdgeCount: 0,
  effectTier: 'ultra' as EffectTier,
  useSimplifiedEffects: false,
  disableAnimations: false,

  registerEdgeMeta: (id: string, source: string, target: string) => {
    set(state => {
      const newMeta = new Map(state.edgeMeta);
      newMeta.set(id, { source, target });
      return { edgeMeta: newMeta };
    });
  },

  setVisible: (id: string, visible: boolean) => {
    const state = get();
    const wasVisible = state.visibleEdges.has(id);

    // No change? Skip update
    if (wasVisible === visible) return;

    const meta = state.edgeMeta.get(id);

    set(prevState => {
      const newVisibleEdges = new Set(prevState.visibleEdges);
      const newNodeCount = new Map(prevState.nodeConnectionCount);

      if (visible) {
        newVisibleEdges.add(id);
        // Increment connection counts for source and target
        if (meta) {
          newNodeCount.set(meta.source, (newNodeCount.get(meta.source) ?? 0) + 1);
          newNodeCount.set(meta.target, (newNodeCount.get(meta.target) ?? 0) + 1);
        }
      } else {
        newVisibleEdges.delete(id);
        // Decrement connection counts for source and target
        if (meta) {
          const srcCount = (newNodeCount.get(meta.source) ?? 1) - 1;
          const tgtCount = (newNodeCount.get(meta.target) ?? 1) - 1;
          if (srcCount <= 0) {
            newNodeCount.delete(meta.source);
          } else {
            newNodeCount.set(meta.source, srcCount);
          }
          if (tgtCount <= 0) {
            newNodeCount.delete(meta.target);
          } else {
            newNodeCount.set(meta.target, tgtCount);
          }
        }
      }

      return { visibleEdges: newVisibleEdges, nodeConnectionCount: newNodeCount };
    });
  },

  isVisible: (id: string) => get().visibleEdges.has(id),

  isHubNode: (nodeId: string) => {
    const count = get().nodeConnectionCount.get(nodeId) ?? 0;
    return count >= PERF_THRESHOLDS.HUB_NODE_THRESHOLD;
  },

  getEffectiveTier: (edgeId: string) => {
    const state = get();
    const meta = state.edgeMeta.get(edgeId);

    // If source or target is a hub node, force LOW tier
    if (meta) {
      const srcCount = state.nodeConnectionCount.get(meta.source) ?? 0;
      const tgtCount = state.nodeConnectionCount.get(meta.target) ?? 0;
      if (srcCount >= PERF_THRESHOLDS.HUB_NODE_THRESHOLD ||
          tgtCount >= PERF_THRESHOLDS.HUB_NODE_THRESHOLD) {
        return 'low';
      }
    }

    return state.effectTier;
  },

  setTotalEdgeCount: (count: number) => {
    const tier = calculateEffectTier(count);
    set({
      totalEdgeCount: count,
      effectTier: tier,
      // LOW tier and below use simplified effects
      useSimplifiedEffects: tier === 'low' || tier === 'minimal',
      // MINIMAL tier disables animations entirely
      disableAnimations: tier === 'minimal',
    });
  },

  clear: () => set({
    visibleEdges: new Set(),
    edgeMeta: new Map(),
    nodeConnectionCount: new Map(),
    totalEdgeCount: 0,
    effectTier: 'ultra' as EffectTier,
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
  /** Register edge metadata for connection tracking (from store) */
  registerEdgeMeta: (id: string, source: string, target: string) => void;
  /** Update visibility for an edge (from store) */
  setVisible: (id: string, visible: boolean) => void;
  /** Check if edge is visible (from store) */
  isVisible: (id: string) => boolean;
  /** Check if a node is a hub (from store) */
  isHubNode: (nodeId: string) => boolean;
  /** Get effective tier for an edge considering hub nodes (from store) */
  getEffectiveTier: (edgeId: string) => EffectTier;
  /** Clear all visibility tracking (from store) */
  clear: () => void;
  /** Register edge for IntersectionObserver (from context) */
  registerEdge: (id: string, element: Element) => void;
  /** Unregister edge from IntersectionObserver (from context) */
  unregisterEdge: (id: string, element: Element) => void;
  /** Update total edge count (from store) */
  setTotalEdgeCount: (count: number) => void;
  /** Current effect quality tier (from store) */
  effectTier: EffectTier;
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
    registerEdgeMeta: store.registerEdgeMeta,
    setVisible: store.setVisible,
    isVisible: store.isVisible,
    isHubNode: store.isHubNode,
    getEffectiveTier: store.getEffectiveTier,
    clear: store.clear,
    setTotalEdgeCount: store.setTotalEdgeCount,
    effectTier: store.effectTier,
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
