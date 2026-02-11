/**
 * Edge Animation System - Performance Module
 *
 * LOD, animation budget, and object pooling for 60fps at scale.
 */

// LOD Controller
export {
  calculateLODTier,
  getLODConfig,
  filterEffectsForLOD,
  getLODIntensity,
  shouldAnimate,
  shouldRender,
  LODManager,
  lodManager,
} from './LODController';

// Animation Budget
export {
  AnimationBudgetManager,
  animationBudget,
  getEdgePriority,
} from './AnimationBudget';

// Effect Pool
export {
  EffectPool,
  effectPool,
  acquireEffects,
  releaseEffects,
} from './LazyEffectPool';
