/**
 * Card Variants Index
 *
 * Type-safe styling variants for NovaNet card components.
 * Based on taxonomy visual encoding (ADR-005).
 *
 * Two variant systems:
 * 1. Layer Variants - Visual encoding based on Realm/Layer
 * 2. Level Variants - Visual encoding based on abstraction level (Taxonomy/Schema/Data)
 */

// =============================================================================
// Layer Variants (ADR-005: Realm/Layer encoding)
// =============================================================================

export {
  // Utility
  cn,
  // Gradient variants
  LAYER_GRADIENTS,
  getLayerGradientClass,
  // Border color variants
  REALM_BORDERS,
  getRealmBorderClass,
  // Combined card function
  getLayerCardClasses,
  type LayerCardVariantProps,
  // Badge variants
  LAYER_BADGE_CLASSES,
  REALM_BADGE_CLASSES,
  getLayerBadgeClass,
  getRealmBadgeClass,
  // Taxonomy badge
  getTaxonomyBadgeClasses,
  type TaxonomyBadgeVariants,
  // Icon glows
  LAYER_ICON_GLOWS,
  getLayerIconGlowClass,
} from './layerCardVariants';

// =============================================================================
// Level Variants (3-Level Architecture: Taxonomy/Schema/Data)
// =============================================================================

export {
  // Visual configuration
  LEVEL_VISUALS,
  getLevelVisuals,
  getLevelShadow,
  getLevelClasses,
  type LevelVisualConfig,
  // Badge configuration
  LEVEL_BADGES,
  getLevelBadgeConfig,
  type LevelBadgeConfig,
  // Animation configuration
  LEVEL_ANIMATIONS,
  getLevelAnimation,
  type LevelAnimationConfig,
  // Typography configuration
  LEVEL_TYPOGRAPHY,
  getLevelTypography,
  type LevelTypographyConfig,
  // Combined style generator
  getLevelStyles,
  type LevelStyles,
  // Type detection utilities
  TAXONOMY_TYPES,
  SCHEMA_TYPES,
  getNodeLevel,
  isTaxonomyType,
  isSchemaType,
  isDataType,
} from './levelVariants';
