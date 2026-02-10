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

import {
  ARC_FAMILY_COLORS,
  ARC_FAMILY_PALETTES,
  ARC_FAMILY_EFFECTS,
  ARC_FAMILY_STROKES,
  getArcFamily,
  getArcFamilyEffect,
  getArcFamilyPalette,
  type ArcFamily,
} from '../system/arcFamilyPalettes';

import { CATEGORY_THEMES, PALETTES } from '../system/themes';
import { resolveTheme, getCategory, clearThemeCache } from '../system/registry';
import type { EffectPrimitive, RelationCategory } from '../system/types';

// =============================================================================
// Arc Family Completeness
// =============================================================================

describe('Arc Family Completeness', () => {
  const ALL_ARC_FAMILIES: ArcFamily[] = ['ownership', 'localization', 'semantic', 'generation', 'mining'];

  it('ARC_FAMILY_COLORS has all 5 families', () => {
    for (const family of ALL_ARC_FAMILIES) {
      expect(ARC_FAMILY_COLORS[family]).toBeDefined();
      expect(ARC_FAMILY_COLORS[family]).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('ARC_FAMILY_PALETTES has all 5 families with complete palettes', () => {
    for (const family of ALL_ARC_FAMILIES) {
      const palette = ARC_FAMILY_PALETTES[family];
      expect(palette).toBeDefined();
      expect(palette.primary).toMatch(/^#[0-9a-f]{6}$/i);
      expect(palette.secondary).toMatch(/^#[0-9a-f]{6}$/i);
      expect(palette.tertiary).toMatch(/^#[0-9a-f]{6}$/i);
      expect(palette.glow).toMatch(/^#[0-9a-f]{6}$/i);
    }
  });

  it('ARC_FAMILY_EFFECTS has all 5 families with unique effects', () => {
    const effects = new Set<string>();
    for (const family of ALL_ARC_FAMILIES) {
      const effect = ARC_FAMILY_EFFECTS[family];
      expect(effect).toBeDefined();
      expect(effects.has(effect)).toBe(false); // Must be unique
      effects.add(effect);
    }
    expect(effects.size).toBe(5);
  });

  it('ARC_FAMILY_STROKES has all 5 families', () => {
    for (const family of ALL_ARC_FAMILIES) {
      const stroke = ARC_FAMILY_STROKES[family];
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
      expect(ARC_FAMILY_COLORS[family]).toBe(ARC_FAMILY_PALETTES[family].primary);
    }
  });

  it('Palette glow matches primary color', () => {
    const families = Object.keys(ARC_FAMILY_PALETTES) as ArcFamily[];
    for (const family of families) {
      const palette = ARC_FAMILY_PALETTES[family];
      expect(palette.glow).toBe(palette.primary);
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
    expect(['ownership', 'localization', 'semantic', 'generation', 'mining']).toContain(theme.arcFamily);
  });

  it('resolved theme effects start with arc family signature effect', () => {
    const testCases = [
      { relation: 'HAS_PAGE', family: 'ownership', effect: 'energyPulse' },
      { relation: 'HAS_CONTENT', family: 'localization', effect: 'dnaHelix' },
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
    const localizationRelations = ['HAS_CONTENT', 'FOR_LOCALE'];
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
