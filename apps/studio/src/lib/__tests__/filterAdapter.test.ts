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
    expect(types).toContain('Formatting');      // v10 technical tier
    expect(types).toContain('ExpressionSet');  // v10 semantic tier
    // v10.9: SEO moved to tenant realm per YAML source of truth
    expect(types).not.toContain('SEOKeyword');
    // v10.6: Entity/EntityContent in tenant realm
    expect(types).not.toContain('Entity');
    expect(types).not.toContain('Project');
  });

  it('returns tenant types for tenant realm', () => {
    const types = resolveTypesForRealms(['tenant']);
    expect(types).toContain('Organization');     // v10.6: Organization in tenant/config
    expect(types).toContain('Project');
    expect(types).toContain('Page');
    expect(types).toContain('AudiencePersona');  // v10.3: semantic types in tenant
    expect(types).toContain('Entity');           // v10.6: Entity in tenant/semantic
    expect(types).toContain('EntityContent');    // v10.9: EntityL10n → EntityContent
    expect(types).toContain('SEOKeyword');       // v10.9: SEO in tenant realm
    expect(types).not.toContain('Locale');
  });

  it('returns union for multiple realms', () => {
    const types = resolveTypesForRealms(['global', 'tenant']);
    expect(types).toContain('Locale');
    expect(types).toContain('Project');
    expect(types).toContain('Entity');
  });

  it('returns 65 types for global + tenant (all node types)', () => {
    const types = resolveTypesForRealms(['global', 'tenant']);
    // v11.1: 2 realms total. global (32) + tenant (33) = 65 (+EntityCategory)
    expect(types.length).toBe(65);
    expect(types).toContain('Organization'); // v10.6: Organization in tenant realm
    expect(types).toContain('EntityCategory'); // v11.1: EntityCategory in global realm
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
    expect(types).not.toContain('EntityContent');  // v10.9: EntityL10n → EntityContent (localized)
    expect(types).not.toContain('ProjectContent'); // v10.9: localized, not invariant
  });

  it('returns localized types', () => {
    const types = resolveTypesForTraits(['localized']);
    expect(types).toContain('ProjectContent');
    expect(types).toContain('EntityContent');   // v10.9: EntityL10n → EntityContent
    // v10.9: SEOKeyword has knowledge trait, not localized
    expect(types).not.toContain('SEOKeyword');
    expect(types).not.toContain('Project');
  });

  it('returns knowledge types', () => {
    const types = resolveTypesForTraits(['knowledge']);
    // v10 tiered model: technical/style/semantic
    expect(types).toContain('Formatting');     // technical tier
    expect(types).toContain('Style');          // style tier
    expect(types).toContain('ExpressionSet'); // semantic tier
    expect(types).toContain('SEOKeyword');    // v10.9: SEO has knowledge trait
    expect(types).not.toContain('Project');
  });

  it('returns derived types', () => {
    const types = resolveTypesForTraits(['derived']);
    expect(types).toContain('SEOKeywordMetrics');
  });

  it('returns job types', () => {
    const types = resolveTypesForTraits(['job']);
    expect(types).toContain('SEOMiningRun');
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
      expect(result.query).toContain('n:ProjectContent');
      expect(result.query).toContain('n:EntityContent');  // v10.9: localized
      // v10.9: SEOKeyword has knowledge trait, not localized
      expect(result.query).not.toContain('n:SEOKeyword');
      // Project (invariant) should not appear — ProjectContent (localized) should
      expect(result.query).not.toMatch(/\bn:Project\b(?!Content)/);
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
        realms: ['tenant'],
        traits: ['localized'],
      });
      // v10.9: tenant + localized = ProjectContent
      expect(result.query).toContain('n:ProjectContent');
      // Locale is global, not tenant
      expect(result.query).not.toContain('n:Locale');
      // Project is invariant, not localized
      expect(result.query).not.toMatch(/\bn:Project\b(?!Content)/);
    });

    it('intersects realm + layer', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['tenant'],
        layers: ['semantic'],
      });
      // v10.9: tenant + semantic = AudiencePersona, ChannelSurface, Entity, EntityContent
      expect(result.query).toContain('n:AudiencePersona');
      expect(result.query).toContain('n:ChannelSurface');
      expect(result.query).toContain('n:Entity');        // v10.6: Entity in tenant
      expect(result.query).toContain('n:EntityContent'); // v10.9: EntityL10n → EntityContent
      expect(result.query).not.toContain('n:Page');
    });

    it('returns empty for global + semantic (no semantic types in global)', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['global'],
        layers: ['semantic'],
      });
      // v10.6: Semantic layer is only in tenant realm. Global has no semantic layer.
      // Empty intersection falls back to all non-Meta nodes
      expect(result.query).toContain('NOT n:Meta');
      expect(result.query).not.toContain('n:Entity');
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
