'use client';

/**
 * useAnimationBudget - Hook for animation budget management
 *
 * Ensures only a limited number of edges animate simultaneously
 */

import { useEffect, useMemo, useCallback, useState } from 'react';
import type { EdgePriority } from '../system/types';
import { animationBudget } from '../system/performance/AnimationBudget';

export interface UseAnimationBudgetOptions {
  /** Unique edge identifier */
  edgeId: string;
  /** Edge priority level */
  priority?: EdgePriority;
  /** Whether animation is requested */
  enabled?: boolean;
}

export interface UseAnimationBudgetResult {
  /** Whether edge has permission to animate */
  canAnimate: boolean;
  /** Force request a slot (may evict lower priority) */
  requestSlot: () => boolean;
  /** Release the current slot */
  releaseSlot: () => void;
}

/**
 * Hook to manage animation budget for a single edge
 */
export function useAnimationBudget(
  options: UseAnimationBudgetOptions
): UseAnimationBudgetResult {
  const { edgeId, priority = 'default', enabled = true } = options;

  const [canAnimate, setCanAnimate] = useState(false);

  // Request slot on mount/enable, release on unmount/disable
  useEffect(() => {
    if (!enabled) {
      animationBudget.releaseSlot(edgeId);
      setCanAnimate(false);
      return;
    }

    const granted = animationBudget.requestSlot(edgeId, priority);
    setCanAnimate(granted);

    return () => {
      animationBudget.releaseSlot(edgeId);
    };
  }, [edgeId, priority, enabled]);

  const requestSlot = useCallback(() => {
    const granted = animationBudget.requestSlot(edgeId, priority);
    setCanAnimate(granted);
    return granted;
  }, [edgeId, priority]);

  const releaseSlot = useCallback(() => {
    animationBudget.releaseSlot(edgeId);
    setCanAnimate(false);
  }, [edgeId]);

  return {
    canAnimate,
    requestSlot,
    releaseSlot,
  };
}

/**
 * Hook to get budget utilization statistics
 * Returns fresh stats on every call (no memoization - stats change frequently)
 */
export function useAnimationBudgetStats() {
  return animationBudget.getStats();
}

/**
 * Hook to check if there's budget capacity
 * Returns fresh value on every call
 */
export function useHasBudgetCapacity() {
  const stats = animationBudget.getStats();
  return stats.current < stats.max;
}

/**
 * Hook to batch request slots for multiple edges
 * Uses stable serialized keys to prevent thrashing on array reference changes
 */
export function useBatchAnimationBudget(
  edges: Array<{ edgeId: string; priority: EdgePriority }>
): Map<string, boolean> {
  // Create stable dependency keys
  const edgeKeys = useMemo(
    () => edges.map(e => `${e.edgeId}:${e.priority}`).join(','),
    [edges]
  );

  const results = useMemo(() => {
    const map = new Map<string, boolean>();

    // Sort by priority (highest first)
    const sorted = [...edges].sort((a, b) => {
      const priorityOrder: Record<EdgePriority, number> = {
        selected: 0,
        highlighted: 1,
        connected: 2,
        default: 3,
      };
      return priorityOrder[a.priority] - priorityOrder[b.priority];
    });

    for (const { edgeId, priority } of sorted) {
      const granted = animationBudget.requestSlot(edgeId, priority);
      map.set(edgeId, granted);
    }

    return map;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [edgeKeys]); // Use stable key instead of array reference

  // Cleanup on unmount - also use stable key
  useEffect(() => {
    return () => {
      for (const { edgeId } of edges) {
        animationBudget.releaseSlot(edgeId);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [edgeKeys]);

  return results;
}
