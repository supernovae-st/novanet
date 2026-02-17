/**
 * Card Variants Index
 *
 * Type-safe styling variants for NovaNet card components.
 * Based on taxonomy visual encoding (ADR-005).
 */

export {
  // Utility
  cn,
  // Gradient variants
  LAYER_GRADIENTS,
  getLayerGradientClass,
  // Border color variants
  REALM_BORDERS,
  getRealmBorderClass,
  // Border style variants
  TRAIT_BORDER_CLASSES,
  getTraitBorderClass,
  // Combined card function
  getLayerCardClasses,
  type LayerCardVariantProps,
  // Badge variants
  LAYER_BADGE_CLASSES,
  REALM_BADGE_CLASSES,
  TRAIT_BADGE_CLASSES,
  getLayerBadgeClass,
  getRealmBadgeClass,
  getTraitBadgeClass,
  // Taxonomy badge
  getTaxonomyBadgeClasses,
  type TaxonomyBadgeVariants,
  // Icon glows
  LAYER_ICON_GLOWS,
  getLayerIconGlowClass,
} from './layerCardVariants';
