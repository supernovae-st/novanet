/**
 * Navigation Route Logic Tests
 *
 * Tests input validation logic shared between the old TS route and the current
 * Rust-bridge route: parseCSV and limit clamping.
 *
 * The "facet intersection" section tests the DEPRECATED TypeScript resolution
 * path (resolveTypesForRealms, resolveTypesForTraits, NODE_LAYERS intersection).
 * Since v9.0.0 the navigation route calls `buildCypherViaRust()` instead.
 * These tests are kept as regression coverage for the deprecated helpers.
 *
 * Bridge-specific tests live in `src/lib/__tests__/novanetBridge.test.ts`.
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

const VALID_REALMS: Realm[] = ['global', 'tenant'];
const VALID_LAYERS: Layer[] = [
  'config', 'locale-knowledge', 'foundation', 'structure', 'semantic',
  'instruction', 'output', 'seo',
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
      expect(parseCSV('global,tenant', VALID_REALMS)).toEqual(['global', 'tenant']);
    });

    it('filters out invalid values', () => {
      expect(parseCSV('global,invalid,tenant', VALID_REALMS)).toEqual(['global', 'tenant']);
    });

    it('handles single value', () => {
      expect(parseCSV('tenant', VALID_REALMS)).toEqual(['tenant']);
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
  // Facet intersection (DEPRECATED — route now uses Rust bridge)
  // Tests the old TS resolution path kept in filterAdapter.ts for reference.
  // ==========================================================================

  describe('facet intersection (deprecated TS path)', () => {
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
      const types = resolveFacets(['global'], [], []);
      expect(types).toContain('Locale');
      // v10.9: SEOKeyword moved to tenant realm per YAML source of truth
      expect(types).not.toContain('SEOKeyword');
      expect(types).not.toContain('Project');
    });

    it('intersects realm + trait correctly', () => {
      const types = resolveFacets(['tenant'], [], ['localized']);
      // v10.9: Tenant realm + localized trait = ProjectContent
      expect(types).toContain('ProjectContent');
      expect(types).not.toContain('Page'); // invariant
      expect(types).not.toContain('Locale'); // global
    });

    it('intersects realm + layer correctly', () => {
      const types = resolveFacets(['tenant'], ['semantic'], []);
      // v10.6: tenant + semantic = AudiencePersona, ChannelSurface, Entity, EntityL10n
      expect(types).toContain('AudiencePersona');
      expect(types).toContain('ChannelSurface');
      expect(types).toContain('Entity'); // v10.9: in tenant realm
      expect(types).toContain('EntityContent'); // v10.9: in tenant realm
      expect(types).not.toContain('Page'); // structure layer
    });

    it('intersects all 3 dimensions', () => {
      const types = resolveFacets(['tenant'], ['output'], ['derived']);
      // v10.9: tenant + output + derived = PageGenerated, BlockGenerated, OutputArtifact, EvaluationSignal
      expect(types).toContain('PageGenerated');
      expect(types).toContain('BlockGenerated');
      expect(types.length).toBeGreaterThanOrEqual(4);
    });

    it('returns only localized types when filtering tenant + localized', () => {
      const types = resolveFacets(['tenant'], [], ['localized']);
      // v10.9: Tenant localized types = ProjectContent, EntityContent
      expect(types).toContain('ProjectContent');
      expect(types).toContain('EntityContent');
      expect(types).not.toContain('Locale'); // global realm
    });
  });
});
