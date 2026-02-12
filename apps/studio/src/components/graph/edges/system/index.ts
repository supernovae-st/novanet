/**
 * Edge Animation System - Core Module
 *
 * Type-safe edge theming and configuration system.
 */

// Types
export * from './types';

// Constants
export * from './constants';

// Themes
export {
  PALETTES,
  CATEGORY_THEMES,
  RELATION_OVERRIDES,
  getColorSchemeName,
  mergePalette,
} from './themes';

// Registry
export {
  getCategory,
  resolveTheme,
  resolveThemeCached,
  getCachedBaseTheme,
  getRelationsForCategory,
  hasCustomOverrides,
  clearThemeCache,
} from './registry';

// Performance
export * from './performance';

// Arc Family (v11.7.0 - unified with @/design/colors/palette)
export {
  type ArcFamily,
  type ArcFamilyKey,
  type ArcFamilyEffect,
  ARC_PALETTES,
  ARC_STROKES,
  ARC_FAMILY_HEX,
  ARC_FAMILY_EFFECTS,
  getArcFamily,
  getArcFamilyPalette,
  getArcPalette,
  getArcFamilyPaletteByKey,
  getArcFamilyEffect,
  getRelationEffect,
} from './arcFamilyPalettes';

// ColorPalette type (re-export from unified system)
export type { ColorPalette } from '@/design/colors/palette';
