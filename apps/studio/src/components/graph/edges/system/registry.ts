/**
 * Edge Animation System - Registry
 *
 * Theme resolver and relation-to-category mapping.
 * Central point for resolving complete edge themes.
 */

import type {
  RelationCategory,
  RelationType,
  ResolvedEdgeTheme,
  TimingConfig,
  SizeConfig,
  EdgeTheme,
  ThemeOverride,
} from './types';

import { CATEGORY_THEMES, RELATION_OVERRIDES, mergePalette } from './themes';
import {
  DEFAULT_STAGGER,
  DEFAULT_EASING,
  PARTICLE_SIZES,
  PARTICLE_COUNTS,
  GLOW_CONFIG,
  getDuration,
} from './constants';
// Arc family utilities (v11.7.0 - unified with @/design/colors/palette)
// Uses arcFamilyPalettes wrapper for backwards compatibility (takes relation type string)
import {
  getArcFamily,
  getArcFamilyPalette,
  getArcFamilyEffect,
  type ArcFamily,
} from './arcFamilyPalettes';

// =============================================================================
// Relation to Category Mapping
// =============================================================================

/**
 * Relation type prefixes/patterns to category mapping
 */
const CATEGORY_PATTERNS: Array<{ pattern: RegExp | string; category: RelationCategory }> = [
  // Structural
  { pattern: /^HAS_(?!CONTENT|GENERATED|PROMPT|RULES|VARIATION|SNAPSHOT|CITATION|REFORMULATION)/, category: 'structural' },
  { pattern: 'CONTAINS', category: 'structural' },

  // Localization
  { pattern: 'HAS_CONTENT', category: 'localization' },
  { pattern: /LOCALE/, category: 'localization' },
  { pattern: 'SUPPORTS', category: 'localization' },

  // Generation
  { pattern: 'HAS_GENERATED', category: 'generation' },
  { pattern: 'HAS_PROMPT', category: 'generation' },
  { pattern: 'HAS_RULES', category: 'generation' },
  { pattern: /GENERATED/, category: 'generation' },

  // Semantic
  { pattern: /CONCEPT/, category: 'semantic' },
  { pattern: 'OF_TYPE', category: 'semantic' },
  { pattern: 'SEMANTIC', category: 'semantic' },

  // SEO
  { pattern: /SEO/, category: 'seo' },
  { pattern: 'HAS_VARIATION', category: 'seo' },
  { pattern: 'HAS_SNAPSHOT', category: 'seo' },

  // GEO
  { pattern: /GEO/, category: 'geo' },
  { pattern: 'HAS_CITATION', category: 'geo' },
  { pattern: 'HAS_REFORMULATION', category: 'geo' },

  // Reference (fallback)
  { pattern: 'USES', category: 'reference' },
  { pattern: 'FALLBACK', category: 'reference' },
  { pattern: 'MODEL', category: 'reference' },
  { pattern: 'PROVIDER', category: 'reference' },
];

/**
 * Get the category for a relation type
 */
export function getCategory(relationType: string): RelationCategory {
  for (const { pattern, category } of CATEGORY_PATTERNS) {
    if (typeof pattern === 'string') {
      if (relationType.includes(pattern)) {
        return category;
      }
    } else {
      if (pattern.test(relationType)) {
        return category;
      }
    }
  }

  // Default to structural
  return 'structural';
}

// =============================================================================
// Theme Resolution
// =============================================================================

/**
 * Build timing config from theme and state
 */
function buildTimingConfig(
  theme: EdgeTheme,
  isSelected: boolean = false,
  isHovered: boolean = false,
): TimingConfig {
  return {
    duration: getDuration(theme.speed, isSelected, isHovered),
    speed: theme.speed,
    stagger: DEFAULT_STAGGER,
    easing: DEFAULT_EASING,
  };
}

/**
 * Build size config from theme and state
 */
function buildSizeConfig(
  theme: EdgeTheme,
  isHighlighted: boolean = false,
): SizeConfig {
  const baseSize = isHighlighted ? PARTICLE_SIZES.highlighted : PARTICLE_SIZES.base;
  const count = PARTICLE_COUNTS[theme.particlePreset] ?? 3;

  return {
    particleSize: baseSize,
    particleCount: count,
    glowMultiplier: isHighlighted ? GLOW_CONFIG.highlightedMultiplier : 1,
    trailLength: isHighlighted ? 1.3 : 1,
  };
}

/**
 * Merge base theme with override
 */
function mergeTheme(base: EdgeTheme, override: ThemeOverride): EdgeTheme {
  return {
    palette: mergePalette(base.palette, override.palette),
    effects: override.effects ?? base.effects,
    lineStyle: override.lineStyle ?? base.lineStyle,
    strokeWidth: override.strokeWidth ?? base.strokeWidth,
    particlePreset: override.particlePreset ?? base.particlePreset,
    speed: override.speed ?? base.speed,
    glowIntensity: override.glowIntensity ?? base.glowIntensity,
  };
}

/**
 * Resolve complete theme for a relation type
 *
 * Resolution order:
 * 1. Get category from relation type (for base style)
 * 2. Get arc family from relation type (for colors + signature effect, v9.5+)
 * 3. Get arc family's signature effect primitive (v11.6.1)
 * 4. Merge category theme with arc family palette and effect
 * 5. Apply relation-specific overrides (if any)
 * 6. Compute timing and size configs
 */
export function resolveTheme(
  relationType: string,
  options: {
    isSelected?: boolean;
    isHovered?: boolean;
  } = {},
): ResolvedEdgeTheme {
  const { isSelected = false, isHovered = false } = options;
  const isHighlighted = isSelected || isHovered;

  // 1. Get category (for effects, line style, etc.)
  const category = getCategory(relationType);

  // 2. Get arc family (for colors, v9.5)
  const arcFamily = getArcFamily(relationType);
  const arcFamilyPalette = getArcFamilyPalette(relationType);

  // 3. Get arc family's signature effect
  const familyEffect = getArcFamilyEffect(arcFamily);

  // 4. Merge: category theme structure + arc family colors + arc family effect
  const categoryTheme = CATEGORY_THEMES[category];
  const baseTheme: EdgeTheme = {
    ...categoryTheme,
    palette: arcFamilyPalette,
    // Inject arc family's signature effect as primary, followed by category effects
    effects: [familyEffect, ...categoryTheme.effects.filter(e => e !== familyEffect)],
  };

  // 5. Apply overrides
  const override = RELATION_OVERRIDES[relationType as RelationType];
  const mergedTheme = override ? mergeTheme(baseTheme, override) : baseTheme;

  // 6. Build resolved theme
  return {
    ...mergedTheme,
    category,
    arcFamily,
    relationType,
    timing: buildTimingConfig(mergedTheme, isSelected, isHovered),
    sizes: buildSizeConfig(mergedTheme, isHighlighted),
    colors: mergedTheme.palette, // Alias for easier access
  };
}

// =============================================================================
// Theme Cache (Performance)
// =============================================================================

/**
 * Cache for resolved themes (without state-dependent values)
 */
const themeCache = new Map<string, EdgeTheme & { category: RelationCategory; arcFamily: ArcFamily }>();

/**
 * Get cached base theme (without timing/size)
 *
 * v9.5: Uses arc family palette for colors, category theme for effects/style
 */
export function getCachedBaseTheme(relationType: string): EdgeTheme & { category: RelationCategory; arcFamily: ArcFamily } {
  let cached = themeCache.get(relationType);

  if (!cached) {
    const category = getCategory(relationType);
    const arcFamily = getArcFamily(relationType);

    // Get base theme structure from category (effects, line style, etc.)
    const categoryTheme = CATEGORY_THEMES[category];

    // Get colors from arc family (v9.5)
    const arcFamilyPalette = getArcFamilyPalette(relationType);

    // Get arc family's signature effect (v11.6.1)
    const familyEffect = getArcFamilyEffect(arcFamily);

    // Merge: use category theme but override with arc family colors + effect
    const baseTheme: EdgeTheme = {
      ...categoryTheme,
      palette: arcFamilyPalette,
      // Inject arc family's signature effect as primary
      effects: [familyEffect, ...categoryTheme.effects.filter(e => e !== familyEffect)],
    };

    // Apply relation-specific overrides
    const override = RELATION_OVERRIDES[relationType as RelationType];
    const mergedTheme = override ? mergeTheme(baseTheme, override) : baseTheme;

    cached = { ...mergedTheme, category, arcFamily };
    themeCache.set(relationType, cached);
  }

  return cached;
}

/**
 * Resolve theme using cache + dynamic state
 *
 * v9.5: Now includes arcFamily in the resolved theme
 */
export function resolveThemeCached(
  relationType: string,
  options: {
    isSelected?: boolean;
    isHovered?: boolean;
  } = {},
): ResolvedEdgeTheme {
  const { isSelected = false, isHovered = false } = options;
  const isHighlighted = isSelected || isHovered;

  const cached = getCachedBaseTheme(relationType);

  return {
    ...cached,
    relationType,
    arcFamily: cached.arcFamily,
    timing: buildTimingConfig(cached, isSelected, isHovered),
    sizes: buildSizeConfig(cached, isHighlighted),
    colors: cached.palette, // Alias for easier access
  };
}

// =============================================================================
// Utilities
// =============================================================================

/**
 * Get all relation types for a category
 */
export function getRelationsForCategory(category: RelationCategory): RelationType[] {
  const relations: RelationType[] = [];

  for (const relationType of Object.keys(RELATION_OVERRIDES)) {
    if (getCategory(relationType) === category) {
      relations.push(relationType as RelationType);
    }
  }

  return relations;
}

/**
 * Check if a relation type has custom overrides
 */
export function hasCustomOverrides(relationType: string): boolean {
  return relationType in RELATION_OVERRIDES;
}

/**
 * Clear theme cache (useful for hot reload)
 */
export function clearThemeCache(): void {
  themeCache.clear();
}
