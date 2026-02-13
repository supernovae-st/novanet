/**
 * Edge Animation System - Themes
 *
 * Color palettes, category base themes, and relation overrides.
 * Hybrid approach: category defines base, relation can override.
 */

import type {
  RelationCategory,
  ColorPalette,
  EdgeTheme,
  ThemeOverride,
  RelationType,
  EffectPrimitive,
} from './types';

// =============================================================================
// Color Palettes (per category)
// =============================================================================

/**
 * Color palettes mapped to relation categories
 * Colors harmonized with Linear design system
 */
export const PALETTES: Record<RelationCategory, ColorPalette> = {
  structural: {
    primary: '#5E6AD2',   // Linear blue
    secondary: '#3b82f6', // Blue 500
    tertiary: '#06b6d4',  // Cyan 500
    glow: '#5E6AD2',
  },
  localization: {
    primary: '#10b981',   // Emerald 500
    secondary: '#22c55e', // Green 500
    tertiary: '#6ee7b7',  // Emerald 300
    glow: '#10b981',
  },
  generation: {
    primary: '#F2994A',   // Linear orange
    secondary: '#f97316', // Orange 500
    tertiary: '#fbbf24',  // Amber 400
    glow: '#F2994A',
  },
  semantic: {
    primary: '#9B51E0',   // Linear purple
    secondary: '#8b5cf6', // Violet 500
    tertiary: '#a78bfa',  // Violet 400
    glow: '#9B51E0',
  },
  seo: {
    primary: '#ec4899',   // Pink 500
    secondary: '#f472b6', // Pink 400
    tertiary: '#f9a8d4',  // Pink 300
    glow: '#ec4899',
  },
  geo: {
    primary: '#06b6d4',   // Cyan 500
    secondary: '#22d3ee', // Cyan 400
    tertiary: '#67e8f9',  // Cyan 300
    glow: '#06b6d4',
  },
  reference: {
    primary: '#14b8a6',   // Teal 500
    secondary: '#2dd4bf', // Teal 400
    tertiary: '#5eead4',  // Teal 300
    glow: '#14b8a6',
  },
} as const;

// =============================================================================
// Category Base Themes
// =============================================================================

/**
 * Default effect stack for most categories
 */
const DEFAULT_EFFECTS: EffectPrimitive[] = ['emit', 'particles', 'trail', 'impact', 'glow'];

/**
 * Base themes per relation category
 */
export const CATEGORY_THEMES: Record<RelationCategory, EdgeTheme> = {
  structural: {
    palette: PALETTES.structural,
    effects: DEFAULT_EFFECTS,
    lineStyle: 'solid',
    strokeWidth: 3,
    particlePreset: 'plasma',
    speed: 'normal',
    glowIntensity: 0.8,
  },

  localization: {
    palette: PALETTES.localization,
    effects: [...DEFAULT_EFFECTS, 'interference'],
    lineStyle: 'double',
    strokeWidth: 2.5,
    particlePreset: 'helix',
    speed: 'slow',
    glowIntensity: 0.75,
  },

  generation: {
    palette: PALETTES.generation,
    effects: [...DEFAULT_EFFECTS, 'scanline'],
    lineStyle: 'solid',
    strokeWidth: 3.5,
    particlePreset: 'spark',
    speed: 'fast',
    glowIntensity: 0.85,
  },

  semantic: {
    palette: PALETTES.semantic,
    effects: [...DEFAULT_EFFECTS, 'interference'],
    lineStyle: 'dotted',
    strokeWidth: 3,
    particlePreset: 'aurora',
    speed: 'slow',
    glowIntensity: 0.75,
  },

  seo: {
    palette: PALETTES.seo,
    effects: DEFAULT_EFFECTS,
    lineStyle: 'solid',
    strokeWidth: 3,
    particlePreset: 'wave',
    speed: 'normal',
    glowIntensity: 0.75,
  },

  geo: {
    palette: PALETTES.geo,
    effects: [...DEFAULT_EFFECTS, 'scanline'],
    lineStyle: 'dashed',
    strokeWidth: 2.5,
    particlePreset: 'orbit',
    speed: 'normal',
    glowIntensity: 0.7,
  },

  reference: {
    palette: PALETTES.reference,
    effects: DEFAULT_EFFECTS,
    lineStyle: 'solid',
    strokeWidth: 2.5,
    particlePreset: 'flow',
    speed: 'normal',
    glowIntensity: 0.8,
  },
} as const;

// =============================================================================
// Relation Overrides (Sparse)
// =============================================================================

/**
 * Relation-specific overrides
 * Only specify what differs from the category base theme
 */
export const RELATION_OVERRIDES: Partial<Record<RelationType, ThemeOverride>> = {
  // ─── Structural Overrides (v10.3: HAS_CONCEPT removed) ───
  HAS_PAGE: {
    strokeWidth: 4,
    particlePreset: 'plasma',
  },
  HAS_BLOCK: {
    strokeWidth: 2.5,
    speed: 'fast',
  },
  HAS_AUDIENCE: {
    particlePreset: 'pulse',
    lineStyle: 'double',
  },

  // ─── Localization Overrides ───
  // v11.6.1: Effects now come from arc family (dnaHelix), not overrides
  SUPPORTS_LOCALE: {
    particlePreset: 'helix',
    glowIntensity: 0.9,
    strokeWidth: 3,
  },
  HAS_CONTENT: {
    speed: 'normal',
    // effects removed: now uses arcFamily 'localization' → 'dnaHelix'
  },
  FOR_LOCALE: {
    lineStyle: 'dashed',
    particlePreset: 'aurora',
  },

  // ─── Generation Overrides ───
  // v11.6.1: Effects now come from arc family, additional effects can be added
  HAS_GENERATED: {
    speed: 'ultra',
    glowIntensity: 0.95,
    // effects removed: now uses arcFamily 'generation' → 'matrixCode' + category defaults
  },
  HAS_INSTRUCTION: {
    lineStyle: 'zigzag',
    particlePreset: 'orbit',
    palette: {
      primary: '#06b6d4',
      secondary: '#22d3ee',
      tertiary: '#67e8f9',
      glow: '#06b6d4',
    },
  },
  HAS_RULES: {
    lineStyle: 'dotted',
    particlePreset: 'pulse',
  },

  // ─── Semantic Overrides ───
  // v11.6.1: Effects now come from arc family (zigzag), not overrides
  USES_ENTITY: {
    speed: 'normal',
    strokeWidth: 3.5,
    // effects removed: now uses arcFamily 'semantic' → 'zigzag'
  },
  OF_TYPE: {
    lineStyle: 'dashed',
    speed: 'slow',
  },

  // ─── SEO Overrides ───
  // v11.6.1: Effects now come from arc family (radarSweep for mining)
  TARGETS_SEO: {
    speed: 'fast',
    // effects removed: now uses arcFamily 'mining' → 'radarSweep'
  },
  HAS_VARIATION: {
    lineStyle: 'dotted',
    particlePreset: 'pulse',
  },

  // ─── GEO Overrides ───
  // v11.6.1: Effects now come from arc family (radarSweep for mining)
  TARGETS_GEO: {
    speed: 'fast',
    // effects removed: now uses arcFamily 'mining' → 'radarSweep'
  },
  HAS_CITATION: {
    lineStyle: 'solid',
    particlePreset: 'spark',
  },

  // ─── Reference Overrides ───
  USES: {
    speed: 'normal',
  },
  FALLBACK_TO: {
    lineStyle: 'dashed',
    particlePreset: 'pulse',
    palette: {
      primary: '#F2C94C',
      secondary: '#fbbf24',
      tertiary: '#fde047',
      glow: '#F2C94C',
    },
  },
} as const;

// =============================================================================
// Theme Helpers
// =============================================================================

/**
 * Get color scheme name from palette (for gradient/filter IDs)
 */
export function getColorSchemeName(palette: ColorPalette): string {
  // Find matching palette by primary color
  for (const [name, p] of Object.entries(PALETTES)) {
    if (p.primary === palette.primary) {
      return name;
    }
  }
  return 'structural'; // Default
}

/**
 * Merge base palette with override (if any)
 */
export function mergePalette(
  base: ColorPalette,
  override?: Partial<ColorPalette>,
): ColorPalette {
  if (!override) return base;
  return {
    primary: override.primary ?? base.primary,
    secondary: override.secondary ?? base.secondary,
    tertiary: override.tertiary ?? base.tertiary,
    glow: override.glow ?? base.glow,
  };
}
