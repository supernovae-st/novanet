/**
 * Edge Animation Hooks
 *
 * Custom hooks for managing edge animations
 */

// Theme hooks
export {
  useEdgeTheme,
  useEdgeColors,
  useEdgeTiming,
  useEdgeEffects,
} from './useEdgeTheme';
export type { UseEdgeThemeOptions, UseEdgeThemeResult } from './useEdgeTheme';

// LOD hooks
export {
  useEdgeLOD,
  useLODStats,
  useRegisterEdgeLOD,
} from './useEdgeLOD';
export type { UseEdgeLODOptions, UseEdgeLODResult } from './useEdgeLOD';

// Animation budget hooks
export {
  useAnimationBudget,
  useAnimationBudgetStats,
  useHasBudgetCapacity,
  useBatchAnimationBudget,
} from './useAnimationBudget';
export type { UseAnimationBudgetOptions, UseAnimationBudgetResult } from './useAnimationBudget';

// Effect pool hooks
export {
  useEffectPool,
  useEffectPoolStats,
  usePrewarmEffectPool,
  useEffectPoolCleanup,
} from './useEffectPool';
export type { UseEffectPoolOptions, UseEffectPoolResult } from './useEffectPool';
