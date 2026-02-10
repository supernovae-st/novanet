// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v11.3.0 (61 nodes, 11 layers, 2 realms)
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
    expect(shared.icon).toBe('🌍');
    // v11.3: shared realm has 3 layers (locale, geography, knowledge)
    expect(Object.keys(shared.layers)).toHaveLength(3);
    expect(shared.layers.locale.nodeTypes).toContain('Locale');
    expect(shared.layers.geography.nodeTypes).toContain('Continent');
    expect(shared.layers.knowledge.nodeTypes).toContain('TermSet');
    expect(shared.layers.knowledge.nodeTypes).toContain('Term');
    expect(shared.layers.knowledge.nodeTypes).toContain('LanguageFamily');
  });

  it('should have correct org realm structure', () => {
    const org = REALM_HIERARCHY.org;
    expect(org.label).toBe('ORG');
    expect(org.icon).toBe('🏢');
    // v11.3: org realm has 8 layers
    expect(Object.keys(org.layers)).toHaveLength(8);
    // config layer has OrgConfig only
    expect(org.layers.config.nodeTypes).toContain('OrgConfig');
    expect(org.layers.foundation.nodeTypes).toContain('Project');
    // Entity/EntityContent are in org/semantic
    expect(org.layers.semantic.nodeTypes).toContain('Entity');
    expect(org.layers.semantic.nodeTypes).toContain('EntityContent');
    // v11.3: SEO and GEO are separate layers
    expect(org.layers.seo.nodeTypes).toContain('SEOKeyword');
    expect(org.layers.geo.nodeTypes).toContain('GEOQuery');
  });

  it('should have correct node counts per layer', () => {
    // v11.3: Shared realm (32 nodes total)
    expect(REALM_HIERARCHY.shared.layers.locale.nodeTypes).toHaveLength(7);
    expect(REALM_HIERARCHY.shared.layers.geography.nodeTypes).toHaveLength(6);
    expect(REALM_HIERARCHY.shared.layers.knowledge.nodeTypes).toHaveLength(19);

    // v11.3: Org realm (29 nodes total)
    expect(REALM_HIERARCHY.org.layers.config.nodeTypes).toHaveLength(1);  // OrgConfig
    expect(REALM_HIERARCHY.org.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.semantic.nodeTypes).toHaveLength(4);
    expect(REALM_HIERARCHY.org.layers.instruction.nodeTypes).toHaveLength(7);
    expect(REALM_HIERARCHY.org.layers.seo.nodeTypes).toHaveLength(5);
    expect(REALM_HIERARCHY.org.layers.geo.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.output.nodeTypes).toHaveLength(3);
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

  it('should have total of 61 nodes across all realms', () => {
    let totalNodes = 0;

    for (const realm of ['shared', 'org'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        totalNodes += layerMeta.nodeTypes.length;
      }
    }

    expect(totalNodes).toBe(61);  // v11.3: 61 nodes
  });
});
