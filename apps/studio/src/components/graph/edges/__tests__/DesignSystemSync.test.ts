/**
 * Design System Synchronization Tests
 *
 * Validates that all design system components are properly synchronized:
 * - Arc family definitions
 * - Color palettes
 * - Effect primitives
 * - Theme resolution
 *
 * These tests ensure the arc animation system v2 remains consistent
 * across all modules.
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */

// v11.7.0: Use unified palette system (colors now from @/design/colors/palette)
import {
  ARC_FAMILY_EFFECTS,
  getArcFamily,
  getArcFamilyEffect,
  getArcFamilyPalette,
  ARC_PALETTES,
  ARC_STROKES,
  type ArcFamily,
} from '../system/arcFamilyPalettes';
import { ARC_FAMILY_COLORS } from '@/design/colors/generated';

import { CATEGORY_THEMES, PALETTES } from '../system/themes';
import { resolveTheme, clearThemeCache } from '../system/registry';
import type { RelationCategory } from '../system/types';

// =============================================================================
// Arc Family Completeness
// =============================================================================

describe('Arc Family Completeness', () => {
  const ALL_ARC_FAMILIES: ArcFamily[] = ['ownership', 'localization', 'semantic', 'generation', 'mining', 'schema'];

  it('ARC_FAMILY_COLORS has all 6 families', () => {
    for (const family of ALL_ARC_FAMILIES) {
      expect(ARC_FAMILY_COLORS[family]).toBeDefined();
      // v11.7.0: ARC_FAMILY_COLORS now has ColorTokens structure with .color property
      expect(ARC_FAMILY_COLORS[family].color).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('ARC_PALETTES has all 6 families with complete palettes', () => {
    for (const family of ALL_ARC_FAMILIES) {
      const palette = ARC_PALETTES[family];
      expect(palette).toBeDefined();
      expect(palette.primary).toMatch(/^#[0-9a-f]{6}$/i);
      expect(palette.secondary).toMatch(/^#[0-9a-f]{6}$/i);
      expect(palette.tertiary).toMatch(/^#[0-9a-f]{6}$/i);
      // v11.7.0: glow is now hex with alpha suffix
      expect(palette.glow).toMatch(/^#[0-9a-f]{6,8}$/i);
    }
  });

  it('ARC_FAMILY_EFFECTS has all 6 families with unique effects', () => {
    const effects = new Set<string>();
    for (const family of ALL_ARC_FAMILIES) {
      const effect = ARC_FAMILY_EFFECTS[family];
      expect(effect).toBeDefined();
      expect(effects.has(effect)).toBe(false); // Must be unique
      effects.add(effect);
    }
    expect(effects.size).toBe(6);
  });

  it('ARC_STROKES has all 6 families', () => {
    for (const family of ALL_ARC_FAMILIES) {
      const stroke = ARC_STROKES[family];
      expect(stroke).toBeDefined();
      expect(['solid', 'dashed', 'dotted']).toContain(stroke.style);
      expect(stroke.width).toBeGreaterThan(0);
    }
  });
});

// =============================================================================
// Color Consistency
// =============================================================================

describe('Color Consistency', () => {
  it('ARC_FAMILY_COLORS matches palette primary colors', () => {
    const families = Object.keys(ARC_FAMILY_COLORS) as ArcFamily[];
    for (const family of families) {
      // v11.7.0: ARC_FAMILY_COLORS has .color property, palette has .primary
      expect(ARC_FAMILY_COLORS[family].color).toBe(ARC_PALETTES[family].primary);
    }
  });

  it('Palette glow is derived from primary color', () => {
    const families = Object.keys(ARC_PALETTES) as ArcFamily[];
    for (const family of families) {
      const palette = ARC_PALETTES[family];
      // v11.7.0: glow is now primary + alpha, so it starts with primary hex
      expect(palette.glow.startsWith(palette.primary)).toBe(true);
    }
  });
});

// =============================================================================
// Effect Mapping
// =============================================================================

describe('Effect Mapping', () => {
  const EXPECTED_EFFECTS: Record<ArcFamily, string> = {
    ownership: 'energyPulse',
    localization: 'dnaHelix',
    semantic: 'zigzag',
    generation: 'matrixCode',
    mining: 'radarSweep',
    schema: 'schemaFlow',
  };

  it('each arc family has the correct signature effect', () => {
    for (const [family, expectedEffect] of Object.entries(EXPECTED_EFFECTS)) {
      expect(getArcFamilyEffect(family as ArcFamily)).toBe(expectedEffect);
    }
  });

  it('getArcFamilyEffect matches ARC_FAMILY_EFFECTS', () => {
    const families = Object.keys(ARC_FAMILY_EFFECTS) as ArcFamily[];
    for (const family of families) {
      expect(getArcFamilyEffect(family)).toBe(ARC_FAMILY_EFFECTS[family]);
    }
  });
});

// =============================================================================
// Theme Resolution
// =============================================================================

describe('Theme Resolution Integration', () => {
  beforeEach(() => {
    clearThemeCache();
  });

  it('resolveTheme returns arcFamily in resolved theme', () => {
    const theme = resolveTheme('HAS_PAGE');
    expect(theme.arcFamily).toBeDefined();
    expect(['ownership', 'localization', 'semantic', 'generation', 'mining', 'schema']).toContain(theme.arcFamily);
  });

  it('resolved theme effects start with arc family signature effect', () => {
    const testCases = [
      { relation: 'HAS_PAGE', family: 'ownership', effect: 'energyPulse' },
      { relation: 'HAS_NATIVE', family: 'localization', effect: 'dnaHelix' },
      { relation: 'USES_ENTITY', family: 'semantic', effect: 'zigzag' },
      { relation: 'GENERATED_BY', family: 'generation', effect: 'matrixCode' },
      { relation: 'HAS_SEO_TARGET', family: 'mining', effect: 'radarSweep' },
    ];

    for (const { relation, family, effect } of testCases) {
      const theme = resolveTheme(relation);
      expect(theme.arcFamily).toBe(family);
      expect(theme.effects[0]).toBe(effect);
    }
  });

  it('resolved theme uses arc family palette colors', () => {
    const theme = resolveTheme('HAS_PAGE');
    const expectedPalette = getArcFamilyPalette('HAS_PAGE');
    expect(theme.colors.primary).toBe(expectedPalette.primary);
  });
});

// =============================================================================
// Category Themes
// =============================================================================

describe('Category Themes', () => {
  const ALL_CATEGORIES: RelationCategory[] = [
    'structural', 'localization', 'generation', 'semantic', 'seo', 'geo', 'reference'
  ];

  it('all categories have defined themes', () => {
    for (const category of ALL_CATEGORIES) {
      expect(CATEGORY_THEMES[category]).toBeDefined();
      expect(CATEGORY_THEMES[category].effects).toBeDefined();
      expect(Array.isArray(CATEGORY_THEMES[category].effects)).toBe(true);
    }
  });

  it('all category palettes are defined', () => {
    for (const category of ALL_CATEGORIES) {
      expect(PALETTES[category]).toBeDefined();
      expect(PALETTES[category].primary).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });
});

// =============================================================================
// Relation Type Resolution
// =============================================================================

describe('Relation Type Resolution', () => {
  it('ownership relations resolve to ownership family', () => {
    const ownershipRelations = ['HAS_PAGE', 'HAS_BLOCK', 'CONTAINS'];
    for (const relation of ownershipRelations) {
      expect(getArcFamily(relation)).toBe('ownership');
    }
  });

  it('localization relations resolve to localization family', () => {
    const localizationRelations = ['HAS_NATIVE', 'FOR_LOCALE'];
    for (const relation of localizationRelations) {
      expect(getArcFamily(relation)).toBe('localization');
    }
  });

  it('semantic relations resolve to semantic family', () => {
    expect(getArcFamily('USES_ENTITY')).toBe('semantic');
    expect(getArcFamily('SEMANTIC_LINK')).toBe('semantic');
  });

  it('generation relations resolve to generation family', () => {
    expect(getArcFamily('GENERATED_BY')).toBe('generation');
    expect(getArcFamily('USES_MODEL')).toBe('generation');
  });

  it('mining relations resolve to mining family', () => {
    const miningRelations = ['HAS_SEO_TARGET', 'HAS_GEO_TARGET', 'HAS_METRICS'];
    for (const relation of miningRelations) {
      expect(getArcFamily(relation)).toBe('mining');
    }
  });
});
