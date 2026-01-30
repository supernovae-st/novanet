/**
 * Lazy Effect Pool
 *
 * Object pooling for effect components.
 * Reduces GC pressure and improves instantiation performance.
 */

import type { EffectPrimitive } from '../types';
import { POOL_PREWARM_COUNTS, POOL_MAX_SIZE } from '../constants';

// =============================================================================
// Types
// =============================================================================

export interface PooledEffect {
  primitive: EffectPrimitive;
  element: SVGGElement | null;
  inUse: boolean;
  edgeId: string | null;
  lastUsed: number;
}

// =============================================================================
// Effect Pool
// =============================================================================

/**
 * Pool manager for SVG effect elements
 */
export class EffectPool {
  private pools: Map<EffectPrimitive, PooledEffect[]> = new Map();
  private activeByEdge: Map<string, Set<PooledEffect>> = new Map();
  private isPrewarmed: boolean = false;

  constructor() {
    this.initializePools();
  }

  /**
   * Initialize empty pools for all primitive types
   */
  private initializePools(): void {
    const primitives: EffectPrimitive[] = [
      'emit', 'particles', 'trail', 'impact',
      'glow', 'zigzag', 'interference', 'scanline',
    ];

    for (const primitive of primitives) {
      this.pools.set(primitive, []);
    }
  }

  /**
   * Pre-warm pools with specified counts or primitives
   */
  prewarm(
    arg1?: Partial<Record<EffectPrimitive, number>> | EffectPrimitive[],
    arg2?: number
  ): void {
    const defaultCounts = POOL_PREWARM_COUNTS;
    if (this.isPrewarmed) return;

    // Handle array form: prewarm(['particles', 'glow'], 10)
    if (Array.isArray(arg1) && typeof arg2 === 'number') {
      for (const primitive of arg1) {
        const pool = this.pools.get(primitive);
        if (!pool) continue;

        for (let i = 0; i < arg2; i++) {
          pool.push(this.createPooledEffect(primitive));
        }
      }
    } else {
      // Handle object form: prewarm({ particles: 20, glow: 30 }) or default
      const counts = (arg1 as Partial<Record<EffectPrimitive, number>>) ?? defaultCounts;
      for (const [primitive, count] of Object.entries(counts)) {
        const pool = this.pools.get(primitive as EffectPrimitive);
        if (!pool) continue;

        for (let i = 0; i < count; i++) {
          pool.push(this.createPooledEffect(primitive as EffectPrimitive));
        }
      }
    }

    this.isPrewarmed = true;
  }

  /**
   * Acquire an effect from the pool
   */
  acquire(primitive: EffectPrimitive, edgeId: string): PooledEffect | undefined;
  acquire(edgeId: string, primitive: EffectPrimitive): PooledEffect;
  acquire(arg1: string | EffectPrimitive, arg2: string | EffectPrimitive): PooledEffect | undefined {
    // Normalize arguments (support both orderings)
    let edgeId: string;
    let primitive: EffectPrimitive;

    if (typeof arg1 === 'string' && !['emit', 'particles', 'trail', 'impact', 'glow', 'zigzag', 'interference', 'scanline'].includes(arg1)) {
      edgeId = arg1;
      primitive = arg2 as EffectPrimitive;
    } else {
      primitive = arg1 as EffectPrimitive;
      edgeId = arg2 as string;
    }

    return this._acquire(edgeId, primitive);
  }

  /**
   * Internal acquire implementation
   */
  private _acquire(edgeId: string, primitive: EffectPrimitive): PooledEffect {
    const pool = this.pools.get(primitive);
    if (!pool) {
      throw new Error(`Unknown primitive: ${primitive}`);
    }

    // Find available pooled effect
    let effect = pool.find(e => !e.inUse);

    // Create new if none available (up to max size)
    if (!effect) {
      if (pool.length < POOL_MAX_SIZE) {
        effect = this.createPooledEffect(primitive);
        pool.push(effect);
      } else {
        // Pool exhausted - find oldest unused and recycle
        const oldest = this.findOldestUnused(pool);
        if (oldest) {
          effect = oldest;
        } else {
          // All in use - create temporary (will be discarded)
          effect = this.createPooledEffect(primitive);
        }
      }
    }

    // Mark as in use
    effect.inUse = true;
    effect.edgeId = edgeId;
    effect.lastUsed = Date.now();

    // Track by edge
    if (!this.activeByEdge.has(edgeId)) {
      this.activeByEdge.set(edgeId, new Set());
    }
    this.activeByEdge.get(edgeId)!.add(effect);

    return effect;
  }

  /**
   * Release a single effect or all effects for an edge
   */
  release(effectOrEdgeId: PooledEffect | string): void {
    if (typeof effectOrEdgeId === 'string') {
      // Release by edge ID
      this.releaseByEdge(effectOrEdgeId);
    } else {
      // Release single effect
      this._releaseEffect(effectOrEdgeId);
    }
  }

  /**
   * Release all effects for an edge
   */
  releaseByEdge(edgeId: string): void {
    const effects = this.activeByEdge.get(edgeId);
    if (!effects) return;

    for (const effect of effects) {
      effect.inUse = false;
      effect.edgeId = null;
      // Element stays in pool for reuse
    }

    this.activeByEdge.delete(edgeId);
  }

  /**
   * Release a specific effect
   */
  private _releaseEffect(effect: PooledEffect): void {
    effect.inUse = false;

    if (effect.edgeId) {
      const edgeEffects = this.activeByEdge.get(effect.edgeId);
      if (edgeEffects) {
        edgeEffects.delete(effect);
        if (edgeEffects.size === 0) {
          this.activeByEdge.delete(effect.edgeId);
        }
      }
      effect.edgeId = null;
    }
  }

  /**
   * Get active effect count for an edge
   */
  getActiveCount(edgeId: string): number {
    return this.activeByEdge.get(edgeId)?.size ?? 0;
  }

  /**
   * Get total pool statistics
   */
  getStats(): {
    total: number;
    inUse: number;
    available: number;
    byPrimitive: Record<string, { total: number; inUse: number }>;
  } {
    let total = 0;
    let inUse = 0;
    const byPrimitive: Record<string, { total: number; inUse: number }> = {};

    for (const [primitive, pool] of this.pools) {
      const poolInUse = pool.filter(e => e.inUse).length;
      total += pool.length;
      inUse += poolInUse;
      byPrimitive[primitive] = {
        total: pool.length,
        inUse: poolInUse,
      };
    }

    return {
      total,
      inUse,
      available: total - inUse,
      byPrimitive,
    };
  }

  /**
   * Clear all pools
   */
  clear(): void {
    for (const pool of this.pools.values()) {
      pool.length = 0;
    }
    this.activeByEdge.clear();
    this.isPrewarmed = false;
  }

  /**
   * Cleanup unused effects (alias for gc)
   */
  cleanup(maxAge: number = 60000): number {
    return this.gc(maxAge);
  }

  /**
   * Reset the entire pool system
   */
  reset(): void {
    this.clear();
    this.initializePools();
  }

  /**
   * Garbage collect unused effects
   */
  gc(maxAge: number = 60000): number {
    const now = Date.now();
    let collected = 0;

    for (const pool of this.pools.values()) {
      // Keep at least prewarm count
      const minKeep = POOL_PREWARM_COUNTS[pool[0]?.primitive as keyof typeof POOL_PREWARM_COUNTS] ?? 5;

      // Remove old unused effects beyond minimum
      const toRemove: number[] = [];
      let kept = 0;

      for (let i = 0; i < pool.length; i++) {
        const effect = pool[i];
        if (!effect.inUse) {
          if (kept >= minKeep && now - effect.lastUsed > maxAge) {
            toRemove.push(i);
          } else {
            kept++;
          }
        }
      }

      // Remove in reverse order to preserve indices
      for (let i = toRemove.length - 1; i >= 0; i--) {
        pool.splice(toRemove[i], 1);
        collected++;
      }
    }

    return collected;
  }

  // ─── Private Methods ───

  private createPooledEffect(primitive: EffectPrimitive): PooledEffect {
    return {
      primitive,
      element: null, // Lazily created when needed
      inUse: false,
      edgeId: null,
      lastUsed: Date.now(),
    };
  }

  private findOldestUnused(pool: PooledEffect[]): PooledEffect | null {
    let oldest: PooledEffect | null = null;
    let oldestTime = Infinity;

    for (const effect of pool) {
      if (!effect.inUse && effect.lastUsed < oldestTime) {
        oldest = effect;
        oldestTime = effect.lastUsed;
      }
    }

    return oldest;
  }
}

/**
 * Singleton effect pool instance
 */
export const effectPool = new EffectPool();

// =============================================================================
// React Integration Hook
// =============================================================================

/**
 * Hook for using effect pool in components
 * (To be used in useEffectPool.ts hook)
 */
export function acquireEffects(
  edgeId: string,
  primitives: EffectPrimitive[],
): PooledEffect[] {
  return primitives.map(primitive => effectPool.acquire(edgeId, primitive));
}

export function releaseEffects(edgeId: string): void {
  effectPool.release(edgeId);
}
