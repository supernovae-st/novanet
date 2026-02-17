/**
 * Design Tokens Index
 *
 * Semantic tokens for NovaNet visual encoding.
 * Source of truth: visual-encoding.yaml
 */

export {
  LAYER_TOKENS,
  type LayerToken,
  getLayerGradient,
  getLayerBadgeClasses,
  getLayerIconGlow,
  getAllLayers,
  getLayersByRealm,
} from './layerTokens';

export {
  TRAIT_TOKENS,
  type TraitToken,
  getTraitAnimation,
  getTraitBorderClass,
  getTraitCreator,
  getAllTraits,
  getTraitBorderStyle,
  TRAIT_ANIMATION_TIMING,
} from './traitTokens';
