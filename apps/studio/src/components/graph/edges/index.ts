/**
 * Edge Animation System
 *
 * Modular edge component for knowledge graph visualization.
 * Composable effect primitives with type-safe themes per relation type.
 *
 * Architecture:
 * - system/types.ts: Discriminated unions for type safety
 * - system/constants.ts: Centralized timing/sizing values
 * - system/themes.ts: Category-based color palettes
 * - system/registry.ts: Theme resolver with overrides
 * - system/performance/: LOD, Budget, Pool controllers
 * - effects/primitives/: 8 composable effect primitives
 * - effects/EffectRenderer.tsx: Orchestrates primitive rendering
 * - hooks/: useEdgeTheme, useEdgeLOD, useAnimationBudget, useEffectPool
 */

// =============================================================================
// Main Components
// =============================================================================

export { FloatingEdge, type FloatingEdgeData, type FloatingEdgeType } from './FloatingEdge';
export { MagneticEdge } from './MagneticEdge';
export { BundledEdge, type BundledEdgeProps } from './BundledEdge';

// =============================================================================
// Type System
// =============================================================================

export type {
  RelationCategory,
  RelationType,
  EffectPrimitive,
  AnimationSpeed,
  LineStyle,
  ParticlePreset,
  EdgeState,
  EdgePriority,
  LODTier,
  ColorPalette,
  TimingConfig,
  SizeConfig,
  EdgeTheme,
  ResolvedEdgeTheme,
  EffectPrimitiveProps,
  LODConfig,
  AnimationBudgetConfig,
} from './system/types';

// =============================================================================
// Theme System
// =============================================================================

export { resolveTheme, resolveThemeCached, getCategory } from './system/registry';
export { CATEGORY_THEMES, PALETTES, RELATION_OVERRIDES } from './system/themes';

// =============================================================================
// Constants
// =============================================================================

export {
  ANIMATION_DURATIONS,
  EASING_PRESETS,
  PARTICLE_SIZES,
  PARTICLE_COUNTS,
  GLOW_CONFIG,
  LOD_CONFIGS,
  EDGE_PRIORITIES,
  DEFAULT_ANIMATION_BUDGET,
  getDuration,
} from './system/constants';

// =============================================================================
// Performance Systems
// =============================================================================

export { calculateLODTier, filterEffectsForLOD, lodManager } from './system/performance/LODController';
export { animationBudget, getEdgePriority } from './system/performance/AnimationBudget';
export { effectPool } from './system/performance/LazyEffectPool';

// =============================================================================
// Effect Primitives
// =============================================================================

export {
  // Core primitives
  EmitPrimitive,
  ImpactPrimitive,
  GlowPrimitive,
  ParticlesPrimitive,
  TrailPrimitive,
  ZigzagPrimitive,
  InterferencePrimitive,
  ScanlinePrimitive,
  // Family-specific primitives (v11.6.1)
  EnergyPulsePrimitive,
  DNAHelixPrimitive,
  MatrixCodePrimitive,
  RadarSweepPrimitive,
  PRIMITIVE_REGISTRY,
} from './effects/primitives';

// =============================================================================
// Effect Renderer
// =============================================================================

export { EffectRenderer, releaseEdgeAnimationSlot, shouldRenderEffect } from './effects/EffectRenderer';
export type { EffectRendererProps, BatchEffectConfig } from './effects/EffectRenderer';

// =============================================================================
// Hooks
// =============================================================================

export { useEdgeTheme, useEdgeColors, useEdgeTiming, useEdgeEffects } from './hooks/useEdgeTheme';
export { useEdgeLOD, useLODStats } from './hooks/useEdgeLOD';
export { useAnimationBudget, useAnimationBudgetStats } from './hooks/useAnimationBudget';
export { useEffectPool, useEffectPoolStats, usePrewarmEffectPool } from './hooks/useEffectPool';
// Parallel edge hooks (v11.6.1)
export { useParallelEdges, getEdgeIndexInGroup, BUNDLE_THRESHOLD } from './hooks/useParallelEdges';
export type { ParallelEdgeGroup, UseParallelEdgesResult } from './hooks/useParallelEdges';

// =============================================================================
// Utilities
// =============================================================================

export {
  formatRelationType,
  getNodeIntersection,
  generateCurvedPath,
  generateParallelPath,
  getPathMidpoint,
  isValidPosition,
} from './EdgeUtils';

// =============================================================================
// Visibility Management
// =============================================================================

export {
  EdgeVisibilityProvider,
  useEdgeVisibility,
  useEdgeVisibilityStore,
} from './EdgeVisibilityManager';
