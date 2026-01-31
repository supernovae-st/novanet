'use client';

/**
 * EffectRenderer - Orchestrates rendering of effect primitives
 *
 * Responsible for:
 * - Resolving theme based on relation type
 * - Filtering effects based on LOD tier
 * - Rendering appropriate primitives
 *
 * NOTE: Animation budget is managed by FloatingEdge via useAnimationBudget hook.
 * EffectRenderer is only rendered when FloatingEdge determines canAnimate=true.
 */

import { memo, useMemo } from 'react';
import type { EffectPrimitiveProps, EdgeState, LODTier } from '../system/types';
import { resolveThemeCached } from '../system/registry';
import { filterEffectsForLOD, calculateLODTier } from '../system/performance/LODController';
import { animationBudget } from '../system/performance/AnimationBudget';
import { PRIMITIVE_REGISTRY } from './primitives';

export interface EffectRendererProps {
  /** Unique edge identifier */
  edgeId: string;
  /** Path element ID for effects to reference */
  pathId: string;
  /** Reversed path ID for bidirectional effects */
  reversedPathId?: string;
  /** Relation type string (e.g., 'HAS_CONCEPT', 'HAS_L10N') */
  relationType: string;
  /** Source node position */
  sourcePosition: { x: number; y: number };
  /** Target node position */
  targetPosition: { x: number; y: number };
  /** Current edge state */
  state: EdgeState;
  /** Current viewport zoom level */
  zoom: number;
  /** Distance from viewport center (for LOD) */
  distanceFromCenter?: number;
  /** Whether this edge is connected to selected node */
  isConnectedToSelected?: boolean;
  /** Optional intensity override (0-1) */
  intensityOverride?: number;
  /** Optional: force specific LOD tier */
  forceLOD?: LODTier;
}

export const EffectRenderer = memo(function EffectRenderer({
  edgeId,
  pathId,
  reversedPathId,
  relationType,
  sourcePosition,
  targetPosition,
  state,
  zoom,
  distanceFromCenter = 0,
  isConnectedToSelected = false,
  intensityOverride,
  forceLOD,
}: EffectRendererProps) {
  // Resolve theme for this relation type
  const theme = useMemo(() => {
    return resolveThemeCached(relationType, {
      isSelected: state === 'selected',
      isHovered: state === 'highlighted',
    });
  }, [relationType, state]);

  // Calculate LOD tier
  const lodTier = useMemo(() => {
    if (forceLOD) return forceLOD;

    return calculateLODTier(
      distanceFromCenter,
      zoom,
      state === 'selected',
      state === 'highlighted',
      isConnectedToSelected
    );
  }, [forceLOD, distanceFromCenter, zoom, state, isConnectedToSelected]);

  // Filter effects based on LOD
  const activeEffects = useMemo(() => {
    return filterEffectsForLOD(theme.effects, lodTier);
  }, [theme.effects, lodTier]);

  // Calculate intensity
  const intensity = useMemo(() => {
    if (intensityOverride !== undefined) return intensityOverride;

    // Base intensity from state
    let base = state === 'selected' ? 1.0
      : state === 'highlighted' ? 0.9
      : state === 'muted' ? 0.3
      : 0.7;

    // Apply LOD reduction
    if (lodTier === 'medium') base *= 0.8;
    if (lodTier === 'low') base *= 0.5;
    if (lodTier === 'minimal') base *= 0.3;

    return base;
  }, [intensityOverride, state, lodTier]);

  // Build primitive props
  const primitiveProps: Omit<EffectPrimitiveProps, 'pathId'> = useMemo(() => ({
    colors: theme.colors,
    timing: theme.timing,
    intensity,
    state,
    sourcePosition,
    targetPosition,
  }), [theme.colors, theme.timing, intensity, state, sourcePosition, targetPosition]);

  // Don't render if minimal LOD with no effects
  // Note: Budget check is done by FloatingEdge before rendering EffectRenderer
  if (lodTier === 'minimal' && activeEffects.length === 0) {
    return null;
  }

  return (
    <g className="effect-renderer" data-edge-id={edgeId} data-lod={lodTier}>
      {activeEffects.map((effectType) => {
        const PrimitiveComponent = PRIMITIVE_REGISTRY[effectType];

        if (!PrimitiveComponent) {
          return null;
        }

        // Some effects need the reversed path
        const effectPathId = effectType === 'impact' && reversedPathId
          ? reversedPathId
          : pathId;

        return (
          <PrimitiveComponent
            key={effectType}
            pathId={effectPathId}
            {...primitiveProps}
          />
        );
      })}
    </g>
  );
});

/**
 * Cleanup function to release animation budget when edge unmounts
 */
export function releaseEdgeAnimationSlot(edgeId: string): void {
  animationBudget.releaseSlot(edgeId);
}

/**
 * Batch render helper for multiple edges
 */
export interface BatchEffectConfig {
  edgeId: string;
  pathId: string;
  relationType: string;
  sourcePosition: { x: number; y: number };
  targetPosition: { x: number; y: number };
  state: EdgeState;
}

export function shouldRenderEffect(
  config: BatchEffectConfig,
  zoom: number,
  distanceFromCenter: number
): boolean {
  const lodTier = calculateLODTier(
    distanceFromCenter,
    zoom,
    config.state === 'selected',
    config.state === 'highlighted',
    false
  );

  // Skip minimal LOD edges entirely
  if (lodTier === 'minimal') return false;

  // Check animation budget
  const priority = config.state === 'selected' ? 'selected'
    : config.state === 'highlighted' ? 'highlighted'
    : 'default';

  return animationBudget.canAnimate(config.edgeId) ||
    animationBudget.requestSlot(config.edgeId, priority);
}
