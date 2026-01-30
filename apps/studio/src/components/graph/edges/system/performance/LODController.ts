/**
 * LOD (Level of Detail) Controller
 *
 * Distance-based detail levels for edge effects.
 * Reduces visual complexity for far/offscreen edges.
 */

import type { LODTier, LODConfig, EffectPrimitive } from '../types';
import { LOD_CONFIGS, LOD_DISTANCES, CORE_EFFECTS } from '../constants';

// =============================================================================
// LOD Tier Calculation
// =============================================================================

/**
 * Calculate LOD tier based on distance and state
 */
export function calculateLODTier(
  distance: number,
  zoom: number,
  isSelected: boolean,
  isHovered: boolean,
  isConnectedToSelected: boolean = false,
): LODTier {
  // Selected/hovered always get high detail
  if (isSelected) return 'high';
  if (isHovered) return 'high';

  // Connected to selected node gets medium
  if (isConnectedToSelected) return 'medium';

  // Adjust distance for zoom level
  const effectiveDistance = distance / Math.max(zoom, 0.1);

  // Distance-based tiers (using mapped values from LOD_DISTANCES)
  if (effectiveDistance < LOD_DISTANCES.full) return 'high';
  if (effectiveDistance < LOD_DISTANCES.reduced) return 'medium';
  if (effectiveDistance < LOD_DISTANCES.minimal) return 'low';

  return 'minimal';
}

/**
 * Get LOD configuration for a tier
 */
export function getLODConfig(tier: LODTier): LODConfig {
  return LOD_CONFIGS[tier];
}

// =============================================================================
// Effect Filtering
// =============================================================================

/**
 * Filter effects based on LOD tier
 */
export function filterEffectsForLOD(
  effects: EffectPrimitive[],
  tier: LODTier,
): EffectPrimitive[] {
  const config = LOD_CONFIGS[tier];

  switch (config.effects) {
    case 'ALL':
      return effects;

    case 'CORE':
      return effects.filter(e => CORE_EFFECTS.includes(e));

    case 'GLOW':
      return effects.filter(e => e === 'glow');

    case 'NONE':
      return [];

    default:
      // Array of specific effects
      return effects.filter(e => (config.effects as EffectPrimitive[]).includes(e));
  }
}

/**
 * Get intensity multiplier for LOD tier
 */
export function getLODIntensity(tier: LODTier): number {
  switch (tier) {
    case 'high':
      return 1;
    case 'medium':
      return 0.7;
    case 'low':
      return 0.4;
    case 'minimal':
      return 0.2;
  }
}

/**
 * Check if animations should run for this tier
 */
export function shouldAnimate(tier: LODTier): boolean {
  return tier !== 'minimal';
}

/**
 * Check if edge should be rendered at all
 */
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function shouldRender(tier: LODTier): boolean {
  return true; // All tiers render something (tier kept for API consistency)
}

// =============================================================================
// Distance Calculation Helpers
// =============================================================================

/**
 * Calculate distance from viewport center to edge midpoint
 */
export function calculateEdgeDistance(
  edgeMidpoint: { x: number; y: number },
  viewportCenter: { x: number; y: number },
): number {
  const dx = edgeMidpoint.x - viewportCenter.x;
  const dy = edgeMidpoint.y - viewportCenter.y;
  return Math.sqrt(dx * dx + dy * dy);
}

/**
 * Calculate edge midpoint from source and target
 */
export function calculateEdgeMidpoint(
  source: { x: number; y: number },
  target: { x: number; y: number },
): { x: number; y: number } {
  return {
    x: (source.x + target.x) / 2,
    y: (source.y + target.y) / 2,
  };
}

// =============================================================================
// LOD Manager Class
// =============================================================================

/**
 * LOD Manager - tracks and updates LOD tiers for all edges
 */
export class LODManager {
  private tiers: Map<string, LODTier> = new Map();
  private viewportCenter: { x: number; y: number } = { x: 0, y: 0 };
  private zoom: number = 1;

  /**
   * Update viewport state
   */
  updateViewport(center: { x: number; y: number }, zoom: number): void {
    this.viewportCenter = center;
    this.zoom = zoom;
  }

  /**
   * Calculate and cache LOD tier for an edge
   */
  updateEdgeLOD(
    edgeId: string,
    midpoint: { x: number; y: number },
    isSelected: boolean,
    isHovered: boolean,
    isConnectedToSelected: boolean,
  ): LODTier {
    const distance = calculateEdgeDistance(midpoint, this.viewportCenter);
    const tier = calculateLODTier(
      distance,
      this.zoom,
      isSelected,
      isHovered,
      isConnectedToSelected,
    );

    this.tiers.set(edgeId, tier);
    return tier;
  }

  /**
   * Get cached LOD tier for an edge
   */
  getTier(edgeId: string): LODTier {
    return this.tiers.get(edgeId) ?? 'high';
  }

  /**
   * Update edge state (for hooks)
   */
  updateEdge(
    edgeId: string,
    state: {
      distance?: number;
      zoom?: number;
      isSelected?: boolean;
      isHovered?: boolean;
      isConnected?: boolean;
    }
  ): LODTier {
    const distance = state.distance ?? 0;
    const zoom = state.zoom ?? this.zoom;
    const tier = calculateLODTier(
      distance,
      zoom,
      state.isSelected ?? false,
      state.isHovered ?? false,
      state.isConnected ?? false
    );
    this.tiers.set(edgeId, tier);
    return tier;
  }

  /**
   * Remove edge from tracking
   */
  removeEdge(edgeId: string): void {
    this.tiers.delete(edgeId);
  }

  /**
   * Clear all cached tiers
   */
  clear(): void {
    this.tiers.clear();
  }

  /**
   * Get statistics about current LOD distribution
   */
  getStats(): Record<LODTier, number> {
    const stats: Record<LODTier, number> = {
      high: 0,
      medium: 0,
      low: 0,
      minimal: 0,
    };

    for (const tier of this.tiers.values()) {
      stats[tier]++;
    }

    return stats;
  }
}

/**
 * Singleton LOD manager instance
 */
export const lodManager = new LODManager();
