/**
 * Arc Family Effects Integration Tests
 *
 * Verifies that arc family signature effects are properly resolved
 * in the theme system (v11.6.1).
 *
 * @see docs/plans/2026-02-10-arc-animation-system-v2-design.md
 */

import { resolveTheme, resolveThemeCached, clearThemeCache } from '../system/registry';
import { getArcFamilyEffect, ARC_FAMILY_EFFECTS } from '../system/arcFamilyPalettes';

describe('Arc Family Effects Integration', () => {
  // Clear cache before each test to ensure fresh theme resolution
  beforeEach(() => {
    clearThemeCache();
  });

  describe('getArcFamilyEffect', () => {
    it('returns correct effect for each arc family', () => {
      expect(getArcFamilyEffect('ownership')).toBe('energyPulse');
      expect(getArcFamilyEffect('localization')).toBe('dnaHelix');
      expect(getArcFamilyEffect('semantic')).toBe('zigzag');
      expect(getArcFamilyEffect('generation')).toBe('matrixCode');
      expect(getArcFamilyEffect('mining')).toBe('radarSweep');
      expect(getArcFamilyEffect('schema')).toBe('schemaFlow');
    });
  });

  describe('resolveTheme', () => {
    it('includes arc family signature effect as primary effect for ownership relations', () => {
      const theme = resolveTheme('HAS_PAGE');
      expect(theme.arcFamily).toBe('ownership');
      expect(theme.effects[0]).toBe('energyPulse');
    });

    it('includes arc family signature effect as primary effect for localization relations', () => {
      const theme = resolveTheme('HAS_NATIVE');
      expect(theme.arcFamily).toBe('localization');
      expect(theme.effects[0]).toBe('dnaHelix');
    });

    it('includes arc family signature effect as primary effect for semantic relations', () => {
      const theme = resolveTheme('USES_ENTITY');
      expect(theme.arcFamily).toBe('semantic');
      expect(theme.effects[0]).toBe('zigzag');
    });

    it('includes arc family signature effect as primary effect for generation relations', () => {
      const theme = resolveTheme('GENERATED_BY');
      expect(theme.arcFamily).toBe('generation');
      expect(theme.effects[0]).toBe('matrixCode');
    });

    it('includes arc family signature effect as primary effect for mining relations', () => {
      const theme = resolveTheme('HAS_SEO_TARGET');
      expect(theme.arcFamily).toBe('mining');
      expect(theme.effects[0]).toBe('radarSweep');
    });

    it('does not duplicate signature effect in effects array', () => {
      const theme = resolveTheme('USES_ENTITY'); // semantic -> zigzag
      // zigzag should only appear once even if category theme had it
      const zigzagCount = theme.effects.filter(e => e === 'zigzag').length;
      expect(zigzagCount).toBe(1);
    });
  });

  describe('resolveThemeCached', () => {
    it('also includes arc family signature effect as primary', () => {
      const theme = resolveThemeCached('HAS_PAGE');
      expect(theme.arcFamily).toBe('ownership');
      expect(theme.effects[0]).toBe('energyPulse');
    });

    it('cached theme matches resolved theme effects', () => {
      const resolved = resolveTheme('GENERATED_BY');
      const cached = resolveThemeCached('GENERATED_BY');
      expect(cached.effects).toEqual(resolved.effects);
    });
  });

  describe('ARC_FAMILY_EFFECTS mapping', () => {
    it('has an effect for all 6 arc families', () => {
      expect(Object.keys(ARC_FAMILY_EFFECTS)).toHaveLength(6);
      expect(ARC_FAMILY_EFFECTS.ownership).toBeDefined();
      expect(ARC_FAMILY_EFFECTS.localization).toBeDefined();
      expect(ARC_FAMILY_EFFECTS.semantic).toBeDefined();
      expect(ARC_FAMILY_EFFECTS.generation).toBeDefined();
      expect(ARC_FAMILY_EFFECTS.mining).toBeDefined();
      expect(ARC_FAMILY_EFFECTS.schema).toBeDefined();
    });
  });
});
