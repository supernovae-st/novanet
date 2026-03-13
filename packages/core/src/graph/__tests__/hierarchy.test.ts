// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v0.17.0 (57 nodes, 10 layers, 2 realms)
import { describe, it, expect } from 'vitest';
import { REALM_HIERARCHY } from '../hierarchy';
import type { Realm } from '../../types/nodes';

describe('graph/hierarchy', () => {
  it('should define all 2 realms', () => {
    const realms = Object.keys(REALM_HIERARCHY) as Realm[];
    expect(realms).toHaveLength(2);
    expect(realms).toContain('shared');
    expect(realms).toContain('org');
  });

  it('should have correct shared realm structure', () => {
    const shared = REALM_HIERARCHY.shared;
    expect(shared.label).toBe('SHARED');
    expect(shared.icon).toBe('◉');  // v11.7: Unicode icons (no emoji)
    // v11.5: shared realm has 4 layers (config, locale, geography, knowledge)
    expect(Object.keys(shared.layers)).toHaveLength(4);
    expect(shared.layers.config.nodeTypes).toContain('EntityCategory');
    expect(shared.layers.config.nodeTypes).toContain('Locale');  // v11.5: Locale in config
    expect(shared.layers.config.nodeTypes).toContain('SEOKeywordFormat');  // v11.5: SEOKeywordFormat in config
    expect(shared.layers.locale.nodeTypes).toContain('Style');   // locale layer has Style, not Locale
    expect(shared.layers.geography.nodeTypes).toContain('Continent');
    expect(shared.layers.knowledge.nodeTypes).toContain('ExpressionSet'); // v0.17.0: TermSet removed
    expect(shared.layers.knowledge.nodeTypes).toContain('Expression');    // v0.17.0: Term removed
    expect(shared.layers.knowledge.nodeTypes).toContain('LanguageFamily');
    // v11.5: SEO/GEO moved to shared/knowledge
    expect(shared.layers.knowledge.nodeTypes).toContain('SEOKeyword');
    expect(shared.layers.knowledge.nodeTypes).toContain('GEOQuery');
  });

  it('should have correct org realm structure', () => {
    const org = REALM_HIERARCHY.org;
    expect(org.label).toBe('ORG');
    expect(org.icon).toBe('◎');  // v11.7: Unicode icons (no emoji)
    // v11.5: org realm has 6 layers (seo/geo removed)
    expect(Object.keys(org.layers)).toHaveLength(6);
    // config layer has OrgConfig only
    expect(org.layers.config.nodeTypes).toContain('OrgConfig');
    expect(org.layers.foundation.nodeTypes).toContain('Project');
    // Entity/EntityNative are in org/semantic
    expect(org.layers.semantic.nodeTypes).toContain('Entity');
    expect(org.layers.semantic.nodeTypes).toContain('EntityNative');
  });

  it('should have correct node counts per layer', () => {
    // v0.17.0: Shared realm (36 nodes total)
    expect(REALM_HIERARCHY.shared.layers.config.nodeTypes).toHaveLength(3);  // EntityCategory, Locale, SEOKeywordFormat
    expect(REALM_HIERARCHY.shared.layers.locale.nodeTypes).toHaveLength(5);  // v0.17.0: Market removed
    expect(REALM_HIERARCHY.shared.layers.geography.nodeTypes).toHaveLength(7);  // v0.12.4: Country added
    expect(REALM_HIERARCHY.shared.layers.knowledge.nodeTypes).toHaveLength(21); // v0.17.0: TermSet, Term, SEOKeywordMetrics removed

    // v0.19.0: Org realm (24 nodes total, +3 enrichment nodes in output)
    expect(REALM_HIERARCHY.org.layers.config.nodeTypes).toHaveLength(1);  // OrgConfig
    expect(REALM_HIERARCHY.org.layers.foundation.nodeTypes).toHaveLength(8);  // v0.17.0: ProjectGEOScope added
    expect(REALM_HIERARCHY.org.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.semantic.nodeTypes).toHaveLength(2);  // v0.17.0: AudiencePersona, ChannelSurface removed
    expect(REALM_HIERARCHY.org.layers.instruction.nodeTypes).toHaveLength(4);  // v0.12.4: PageStructure, PageInstruction deleted
    expect(REALM_HIERARCHY.org.layers.output.nodeTypes).toHaveLength(6);  // v0.19.0: +CultureRefEnrichment, ExpressionEnrichment, TabooEnrichment
  });

  it('should have valid realm definitions with required fields', () => {
    for (const realm of ['shared', 'org'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      expect(def.realm).toBe(realm);
      expect(typeof def.label).toBe('string');
      expect(typeof def.icon).toBe('string');
      expect(typeof def.description).toBe('string');
      expect(typeof def.layers).toBe('object');
    }
  });

  it('should have valid layer metadata with required fields', () => {
    for (const realm of ['shared', 'org'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        expect(typeof layerMeta.label).toBe('string');
        expect(typeof layerMeta.description).toBe('string');
        expect(typeof layerMeta.icon).toBe('string');
        expect(Array.isArray(layerMeta.nodeTypes)).toBe(true);
        expect(layerMeta.nodeTypes.length).toBeGreaterThan(0);
      }
    }
  });

  it('should have total of 60 nodes across all realms', () => {
    let totalNodes = 0;

    for (const realm of ['shared', 'org'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        totalNodes += layerMeta.nodeTypes.length;
      }
    }

    expect(totalNodes).toBe(60);  // v0.19.0: 60 nodes (36 shared + 24 org)
  });
});
