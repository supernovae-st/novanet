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
    expect(types).toContain('Formatting');      // v10 technical tier
    expect(types).toContain('ExpressionSet');  // v10 semantic tier
    expect(types).toContain('SEOKeyword');     // v10.2: SEO moved to global
    expect(types).toContain('Entity');         // v10.3: Entity-Centric in global
    expect(types).not.toContain('Project');
  });

  it('returns project types for project realm', () => {
    const types = resolveTypesForRealms(['project']);
    expect(types).toContain('Project');
    expect(types).toContain('Page');
    expect(types).toContain('AudiencePersona');  // v10.3: semantic types remaining in project
    expect(types).not.toContain('Entity');       // v10.3: Entity moved to global
    expect(types).not.toContain('Locale');
  });

  it('returns union for multiple realms', () => {
    const types = resolveTypesForRealms(['global', 'project']);
    expect(types).toContain('Locale');
    expect(types).toContain('Project');
    expect(types).toContain('Entity');
  });

  it('returns all types for both realms', () => {
    const types = resolveTypesForRealms(['global', 'project']);
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
    expect(types).not.toContain('EntityL10n');
    expect(types).not.toContain('PageL10n');
  });

  it('returns localized types', () => {
    const types = resolveTypesForTraits(['localized']);
    expect(types).toContain('EntityL10n');
    expect(types).toContain('PageL10n');
    expect(types).toContain('BlockL10n');
    expect(types).toContain('ProjectL10n');
    expect(types).not.toContain('Project');
  });

  it('returns knowledge types', () => {
    const types = resolveTypesForTraits(['knowledge']);
    // v10 tiered model: technical/style/semantic
    expect(types).toContain('Formatting');     // technical tier
    expect(types).toContain('Style');          // style tier
    expect(types).toContain('ExpressionSet'); // semantic tier
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
      expect(result.query).toContain('n:EntityL10n');
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
      // project + localized = ProjectL10n, PageL10n, BlockL10n
      // (EntityL10n is global realm, not project)
      expect(result.query).toContain('n:ProjectL10n');
      expect(result.query).toContain('n:PageL10n');
      // EntityL10n is global, not project
      expect(result.query).not.toContain('n:EntityL10n');
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
      // project + semantic = AudiencePersona, ChannelSurface
      // (Entity/EntityL10n are in global realm, v10.3 Entity-Centric)
      expect(result.query).toContain('n:AudiencePersona');
      expect(result.query).toContain('n:ChannelSurface');
      expect(result.query).not.toContain('n:Entity');
      expect(result.query).not.toContain('n:Page');
    });

    it('returns global semantic types for global + semantic', () => {
      const result = buildFacetCypher({
        ...emptyFacets,
        realms: ['global'],
        layers: ['semantic'],
      });
      // global + semantic = Entity, EntityL10n (v10.3 Entity-Centric)
      expect(result.query).toContain('n:Entity');
      expect(result.query).toContain('n:EntityL10n');
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
