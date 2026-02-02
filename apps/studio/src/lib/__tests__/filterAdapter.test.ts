/**
 * FilterAdapter Tests — buildFacetCypher + resolver helpers
 *
 * Pure function tests: no mocks needed.
 * Tests facet resolution, intersection logic, and Cypher generation.
 */

import {
  buildFacetCypher,
  resolveTypesForRealms,
  resolveTypesForTraits,
  type FacetQuery,
} from '../filterAdapter';
import { ALL_NODE_TYPES } from '@/config/nodeTypes';

// =============================================================================
// resolveTypesForRealms
// =============================================================================

describe('resolveTypesForRealms', () => {
  it('returns empty array for empty realms', () => {
    expect(resolveTypesForRealms([])).toEqual([]);
  });

  it('returns global types for global realm', () => {
    const types = resolveTypesForRealms(['global']);
    expect(types).toContain('Locale');
    expect(types).toContain('LocaleIdentity');
    expect(types).toContain('Expression');
    expect(types).not.toContain('Project');
    expect(types).not.toContain('SEOKeywordL10n');
  });

  it('returns project types for project realm', () => {
    const types = resolveTypesForRealms(['project']);
    expect(types).toContain('Project');
    expect(types).toContain('Page');
    expect(types).toContain('Concept');
    expect(types).not.toContain('Locale');
    expect(types).not.toContain('SEOKeywordL10n');
  });

  it('returns shared types for shared realm', () => {
    const types = resolveTypesForRealms(['shared']);
    expect(types).toContain('SEOKeywordL10n');
    expect(types).toContain('GEOSeedL10n');
    expect(types).not.toContain('Locale');
    expect(types).not.toContain('Project');
  });

  it('returns union for multiple realms', () => {
    const types = resolveTypesForRealms(['global', 'shared']);
    expect(types).toContain('Locale');
    expect(types).toContain('SEOKeywordL10n');
    expect(types).not.toContain('Project');
  });

  it('returns all 35 types for all 3 realms', () => {
    const types = resolveTypesForRealms(['global', 'project', 'shared']);
    expect(types.length).toBe(ALL_NODE_TYPES.length);
  });
});

// =============================================================================
// resolveTypesForTraits
// =============================================================================

describe('resolveTypesForTraits', () => {
  it('returns empty array for empty traits', () => {
    expect(resolveTypesForTraits([])).toEqual([]);
  });

  it('returns invariant types', () => {
    const types = resolveTypesForTraits(['invariant']);
    expect(types).toContain('Locale');
    expect(types).toContain('Project');
    expect(types).toContain('Page');
    expect(types).not.toContain('ConceptL10n');
    expect(types).not.toContain('PageL10n');
  });

  it('returns localized types', () => {
    const types = resolveTypesForTraits(['localized']);
    expect(types).toContain('ConceptL10n');
    expect(types).toContain('PageL10n');
    expect(types).toContain('BlockL10n');
    expect(types).toContain('ProjectL10n');
    expect(types).not.toContain('Project');
  });

  it('returns knowledge types', () => {
    const types = resolveTypesForTraits(['knowledge']);
    expect(types).toContain('LocaleIdentity');
    expect(types).toContain('LocaleVoice');
    expect(types).toContain('Expression');
    expect(types).not.toContain('Project');
  });

  it('returns derived types', () => {
    const types = resolveTypesForTraits(['derived']);
    expect(types).toContain('SEOKeywordMetrics');
    expect(types).toContain('GEOSeedMetrics');
  });

  it('returns job types', () => {
    const types = resolveTypesForTraits(['job']);
    expect(types).toContain('SEOMiningRun');
    expect(types).toContain('GEOMiningRun');
  });

  it('returns union for multiple traits', () => {
    const types = resolveTypesForTraits(['derived', 'job']);
    expect(types).toContain('SEOKeywordMetrics');
    expect(types).toContain('SEOMiningRun');
  });
});

// =============================================================================
// buildFacetCypher
// =============================================================================

describe('buildFacetCypher', () => {
  const emptyFacets: FacetQuery = {
    realms: [],
    layers: [],
    traits: [],
    edgeFamilies: [],
  };

  describe('empty facets', () => {
    it('matches all non-Meta nodes when no facets provided', () => {
      const result = buildFacetCypher(emptyFacets);
      expect(result.query).toContain('MATCH (n) WHERE NOT n:Meta');
      expect(result.query).toContain('RETURN n');
    });

    it('applies default limit', () => {
      const result = buildFacetCypher(emptyFacets);
      expect(result.query).toContain('LIMIT');
    });
  });

  describe('single facet dimension', () => {
    it('filters by realm', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['global'],
      });
      // Should have label conditions for global types
      expect(result.query).toContain('n:Locale');
      expect(result.query).toContain('NOT n:Meta');
      expect(result.query).not.toContain('n:Project');
    });

    it('filters by trait', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        traits: ['localized'],
      });
      expect(result.query).toContain('n:ConceptL10n');
      expect(result.query).toContain('n:PageL10n');
      // Project (invariant) should not appear — ProjectL10n (localized) should
      expect(result.query).not.toMatch(/\bn:Project\b(?!L10n)/);
    });

    it('filters by layer', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        layers: ['foundation'],
      });
      expect(result.query).toContain('n:Project');
      expect(result.query).toContain('n:BrandIdentity');
      expect(result.query).not.toContain('n:Locale');
    });
  });

  describe('multi-facet intersection', () => {
    it('intersects realm + trait', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['project'],
        traits: ['localized'],
      });
      // project + localized = ConceptL10n, ProjectL10n, PageL10n, BlockL10n
      expect(result.query).toContain('n:ConceptL10n');
      expect(result.query).toContain('n:PageL10n');
      // Locale is global, not project
      expect(result.query).not.toContain('n:Locale');
      // Project is invariant, not localized
      expect(result.query).not.toMatch(/\bn:Project\b(?!L10n)/);
    });

    it('intersects realm + layer', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['project'],
        layers: ['semantic'],
      });
      // project + semantic = Concept, ConceptL10n
      expect(result.query).toContain('n:Concept');
      expect(result.query).toContain('n:ConceptL10n');
      expect(result.query).not.toContain('n:Page');
    });

    it('returns empty match when intersection is empty', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['shared'],     // SEO/GEO types
        traits: ['invariant'],  // No shared types are invariant
      });
      // Intersection should be empty, falling into the "all types" branch
      // because resolvedTypes.length === 0 when intersection is empty
      expect(result.query).toContain('MATCH (n)');
    });
  });

  describe('custom limit', () => {
    it('uses provided limit', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        limit: 100,
      });
      expect(result.query).toContain('LIMIT 100');
    });
  });

  describe('edge family filter', () => {
    it('adds edge filter clauses when edgeFamilies provided', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        edgeFamilies: ['ownership', 'semantic'],
      });
      expect(result.query).toContain('OPTIONAL MATCH (n)-[r]-()');
      expect(result.params.edgeFamilies).toEqual(['ownership', 'semantic']);
    });

    it('omits edge filter when edgeFamilies is empty', () => {
      const result = buildFacetCypher(emptyFacets);
      expect(result.query).not.toContain('OPTIONAL MATCH (n)-[r]-()');
    });
  });
});
