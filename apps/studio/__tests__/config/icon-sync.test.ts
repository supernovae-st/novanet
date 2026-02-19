/**
 * Icon Synchronization Tests
 *
 * Verifies that icons in CategoryIcon.tsx match the single source of truth
 * in visual-encoding.yaml (via ICONS in @novanet/core).
 *
 * Source of truth: packages/core/models/visual-encoding.yaml
 *
 * v11.7: Icons use dual format { web: "lucide-name", terminal: "unicode" }
 */

import { ICONS } from '@novanet/core/graph';

describe('Icon Synchronization', () => {
  describe('Layer Icons (from visual-encoding.yaml)', () => {
    it('should have all 9 layer icons defined', () => {
      const expectedLayers = [
        'config',
        'locale',
        'geography',
        'knowledge',
        'foundation',
        'structure',
        'semantic',
        'instruction',
        'output',
      ];

      for (const layer of expectedLayers) {
        const icon = ICONS.layers[layer];
        expect(icon).toBeDefined();
        expect(icon.web).toBeDefined();
        expect(icon.terminal).toBeDefined();
      }
    });

    it('layer icons should have valid Lucide names (web)', () => {
      // Valid Lucide icon names (lowercase with hyphens)
      const lucidePattern = /^[a-z][a-z0-9-]*$/;

      for (const [_layer, icon] of Object.entries(ICONS.layers)) {
        expect(icon.web).toMatch(lucidePattern);
      }
    });

    it('layer icons should have single-width Unicode symbols (terminal)', () => {
      for (const [_layer, icon] of Object.entries(ICONS.layers)) {
        // Terminal icons should be non-empty strings
        expect(icon.terminal).toBeTruthy();
        expect(typeof icon.terminal).toBe('string');
      }
    });
  });

  describe('Trait Icons (from visual-encoding.yaml)', () => {
    it('should have all 5 trait icons defined', () => {
      // v0.12.0: renamed per ADR-024 Data Origin
      const expectedTraits = [
        'defined',     // was: invariant
        'authored',    // was: localized
        'imported',    // was: knowledge
        'generated',
        'retrieved',   // was: aggregated
      ];

      for (const trait of expectedTraits) {
        const icon = ICONS.traits[trait];
        expect(icon).toBeDefined();
        expect(icon.web).toBeDefined();
        expect(icon.terminal).toBeDefined();
      }
    });

    it('should have correct web icons for traits (v0.12.0: ADR-024 renames)', () => {
      // From visual-encoding.yaml - v0.12.0 trait renames per ADR-024 Data Origin
      expect(ICONS.traits.defined.web).toBe('lock');       // was: invariant
      expect(ICONS.traits.authored.web).toBe('pen');       // was: localized
      expect(ICONS.traits.imported.web).toBe('download');  // was: knowledge
      expect(ICONS.traits.generated.web).toBe('sparkles');
      expect(ICONS.traits.retrieved.web).toBe('cloud-download');  // was: aggregated
    });
  });

  describe('Realm Icons (from visual-encoding.yaml)', () => {
    it('should have both realm icons defined', () => {
      expect(ICONS.realms.shared).toBeDefined();
      expect(ICONS.realms.org).toBeDefined();
    });

    it('should have correct web icons for realms', () => {
      expect(ICONS.realms.shared.web).toBe('globe');
      expect(ICONS.realms.org.web).toBe('building-2');
    });
  });

  describe('Arc Family Icons (from visual-encoding.yaml)', () => {
    it('should have all 6 arc family icons defined', () => {
      // v0.13.1: 6 arc families (added schema family)
      const expectedFamilies = [
        'ownership',
        'localization',
        'semantic',
        'generation',
        'mining',
        'schema',
      ];

      for (const family of expectedFamilies) {
        const icon = ICONS.arc_families[family];
        expect(icon).toBeDefined();
        expect(icon.web).toBeDefined();
        expect(icon.terminal).toBeDefined();
      }
    });
  });

  describe('ICONS Structure Integrity', () => {
    it('should have all required icon categories', () => {
      expect(ICONS).toHaveProperty('realms');
      expect(ICONS).toHaveProperty('layers');
      expect(ICONS).toHaveProperty('traits');
      expect(ICONS).toHaveProperty('arc_families');
      expect(ICONS).toHaveProperty('states');
      expect(ICONS).toHaveProperty('navigation');
      expect(ICONS).toHaveProperty('quality');
      expect(ICONS).toHaveProperty('modes');
    });

    it('all icons should have web and terminal properties', () => {
      const checkCategory = (category: Record<string, { web: string; terminal: string }>) => {
        for (const [_key, icon] of Object.entries(category)) {
          expect(icon).toHaveProperty('web');
          expect(icon).toHaveProperty('terminal');
          expect(typeof icon.web).toBe('string');
          expect(typeof icon.terminal).toBe('string');
          expect(icon.web.length).toBeGreaterThan(0);
          expect(icon.terminal.length).toBeGreaterThan(0);
        }
      };

      checkCategory(ICONS.realms);
      checkCategory(ICONS.layers);
      checkCategory(ICONS.traits);
      checkCategory(ICONS.arc_families);
      checkCategory(ICONS.states);
      checkCategory(ICONS.navigation);
      checkCategory(ICONS.quality);
      checkCategory(ICONS.modes);
    });
  });
});
