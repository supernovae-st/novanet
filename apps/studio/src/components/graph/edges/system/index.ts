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
