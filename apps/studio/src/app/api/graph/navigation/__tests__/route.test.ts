/**
 * Navigation Route Logic Tests
 *
 * Tests the facet resolution and intersection logic used by the navigation route.
 * The route itself is thin glue around these pure functions + fetchGraphData.
 *
 * NextRequest requires Web API polyfills not available in jsdom,
 * so we test the exported resolution helpers directly.
 */

import {
  resolveTypesForRealms,
  resolveTypesForTraits,
} from '@/lib/filterAdapter';
import { NODE_LAYERS } from '@/config/nodeTypes';
import type { NodeType, Realm, Layer, Trait } from '@novanet/core/types';

// =============================================================================
// parseCSV (replicated — private in route, but logic is trivial)
// =============================================================================

function parseCSV<T extends string>(param: string | null, valid: T[]): T[] {
  if (!param) return [];
  return param.split(',').filter((v): v is T => valid.includes(v as T));
}

const VALID_REALMS: Realm[] = ['global', 'project', 'shared'];
const VALID_LAYERS: Layer[] = [
  'config', 'knowledge', 'foundation', 'structure', 'semantic',
  'instruction', 'output', 'seo', 'geo',
];
const VALID_TRAITS: Trait[] = ['invariant', 'localized', 'knowledge', 'derived', 'job'];

describe('navigation route logic', () => {
  // ==========================================================================
  // CSV parsing (same logic as route's parseCSV)
  // ==========================================================================

  describe('parseCSV', () => {
    it('returns empty for null', () => {
      expect(parseCSV(null, VALID_REALMS)).toEqual([]);
    });

    it('returns empty for empty string', () => {
      expect(parseCSV('', VALID_REALMS)).toEqual([]);
    });

    it('parses valid comma-separated values', () => {
      expect(parseCSV('global,project', VALID_REALMS)).toEqual(['global', 'project']);
    });

    it('filters out invalid values', () => {
      expect(parseCSV('global,invalid,project', VALID_REALMS)).toEqual(['global', 'project']);
    });

    it('handles single value', () => {
      expect(parseCSV('shared', VALID_REALMS)).toEqual(['shared']);
    });

    it('works for layers', () => {
      expect(parseCSV('foundation,semantic,fake', VALID_LAYERS)).toEqual(['foundation', 'semantic']);
    });

    it('works for traits', () => {
      expect(parseCSV('invariant,localized', VALID_TRAITS)).toEqual(['invariant', 'localized']);
    });
  });

  // ==========================================================================
  // Limit clamping (same logic as route)
  // ==========================================================================

  describe('limit clamping', () => {
    function clampLimit(raw: string | null): number {
      const rawLimit = parseInt(raw || '500', 10);
      return Math.min(Math.max(1, isNaN(rawLimit) ? 500 : rawLimit), 5000);
    }

    it('defaults to 500 for null', () => {
      expect(clampLimit(null)).toBe(500);
    });

    it('accepts valid limit', () => {
      expect(clampLimit('100')).toBe(100);
    });

    it('caps at 5000', () => {
      expect(clampLimit('10000')).toBe(5000);
    });

    it('enforces minimum of 1', () => {
      expect(clampLimit('-5')).toBe(1);
    });

    it('falls back to 500 for NaN', () => {
      expect(clampLimit('abc')).toBe(500);
    });
  });

  // ==========================================================================
  // Facet intersection (same logic as route)
  // ==========================================================================

  describe('facet intersection', () => {
    function resolveFacets(
      realms: Realm[],
      layers: Layer[],
      traits: Trait[]
    ): NodeType[] {
      const realmTypes = resolveTypesForRealms(realms);
      const traitTypes = resolveTypesForTraits(traits);
      const layerTypes: NodeType[] = [];
      for (const layer of layers) {
        const types = NODE_LAYERS[layer];
        if (types) types.forEach((t) => layerTypes.push(t));
      }

      const sets = [realmTypes, traitTypes, layerTypes].filter((s) => s.length > 0);

      if (sets.length === 0) return [];
      if (sets.length === 1) return sets[0];

      const first = new Set(sets[0]);
      for (let i = 1; i < sets.length; i++) {
        const current = new Set(sets[i]);
        for (const t of first) {
          if (!current.has(t)) first.delete(t);
        }
      }
      return [...first];
    }

    it('returns empty for no facets', () => {
      expect(resolveFacets([], [], [])).toEqual([]);
    });

    it('returns realm types for realm-only filter', () => {
      const types = resolveFacets(['shared'], [], []);
      expect(types).toContain('SEOKeywordL10n');
      expect(types).toContain('GEOMiningRun');
      expect(types).not.toContain('Project');
    });

    it('intersects realm + trait correctly', () => {
      const types = resolveFacets(['project'], [], ['localized']);
      // Project realm + localized trait = ProjectL10n, ConceptL10n, PageL10n, BlockL10n
      expect(types).toContain('ConceptL10n');
      expect(types).toContain('ProjectL10n');
      expect(types).not.toContain('Page'); // invariant
      expect(types).not.toContain('Locale'); // global
    });

    it('intersects realm + layer correctly', () => {
      const types = resolveFacets(['project'], ['semantic'], []);
      expect(types).toContain('Concept');
      expect(types).toContain('ConceptL10n');
      expect(types).not.toContain('Page'); // structure layer
    });

    it('intersects all 3 dimensions', () => {
      const types = resolveFacets(['project'], ['output'], ['localized']);
      // project + output + localized = PageL10n, BlockL10n
      expect(types).toContain('PageL10n');
      expect(types).toContain('BlockL10n');
      expect(types.length).toBe(2);
    });

    it('returns empty when intersection has no overlap', () => {
      const types = resolveFacets(['shared'], [], ['invariant']);
      // No shared types are invariant
      expect(types).toEqual([]);
    });
  });
});
