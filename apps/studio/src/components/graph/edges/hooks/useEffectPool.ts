'use client';

/**
 * useEffectPool - Hook for object pooling of SVG effects
 *
 * Manages reusable effect instances to reduce GC pressure
 */

import { useEffect, useMemo, useCallback } from 'react';
import type { EffectPrimitive } from '../system/types';
import { effectPool, type PooledEffect } from '../system/performance/LazyEffectPool';

export interface UseEffectPoolOptions {
  /** Unique edge identifier */
  edgeId: string;
  /** Effect types needed */
  effectTypes: EffectPrimitive[];
  /** Whether pooling is enabled */
  enabled?: boolean;
}

export interface UseEffectPoolResult {
  /** Acquire an effect from the pool */
  acquire: (type: EffectPrimitive) => PooledEffect | undefined;
  /** Release an effect back to the pool */
  release: (effect: PooledEffect) => void;
  /** Release all effects for this edge */
  releaseAll: () => void;
}

/**
 * Hook to manage pooled effects for a single edge
 */
export function useEffectPool(options: UseEffectPoolOptions): UseEffectPoolResult {
  const { edgeId, enabled = true } = options;

  // Track acquired effects for this edge
  const acquiredEffects = useMemo(() => new Set<PooledEffect>(), []);

  // Acquire effect from pool
  const acquire = useCallback(
    (type: EffectPrimitive): PooledEffect | undefined => {
      if (!enabled) return undefined;

      const effect = effectPool.acquire(type, edgeId);
      if (effect) {
        acquiredEffects.add(effect);
      }
      return effect;
    },
    [edgeId, enabled, acquiredEffects]
  );

  // Release single effect
  const release = useCallback(
    (effect: PooledEffect) => {
      effectPool.release(effect);
      acquiredEffects.delete(effect);
    },
    [acquiredEffects]
  );

  // Release all effects for this edge
  const releaseAll = useCallback(() => {
    effectPool.releaseByEdge(edgeId);
    acquiredEffects.clear();
  }, [edgeId, acquiredEffects]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      effectPool.releaseByEdge(edgeId);
    };
  }, [edgeId]);

  return {
    acquire,
    release,
    releaseAll,
  };
}

/**
 * Hook to get pool statistics
 */
export function useEffectPoolStats() {
  return useMemo(() => {
    return effectPool.getStats();
  }, []);
}

/**
 * Hook to prewarm the pool for expected effects
 */
export function usePrewarmEffectPool(
  effectTypes: EffectPrimitive[],
  countPerType: number = 10
) {
  useEffect(() => {
    effectPool.prewarm(effectTypes, countPerType);
  }, [effectTypes, countPerType]);
}

/**
 * Hook to cleanup pool (useful for view changes)
 */
export function useEffectPoolCleanup() {
  const cleanup = useCallback(() => {
    effectPool.cleanup();
  }, []);

  const reset = useCallback(() => {
    effectPool.reset();
  }, []);

  return { cleanup, reset };
}
