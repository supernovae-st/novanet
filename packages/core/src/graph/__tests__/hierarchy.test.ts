// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v11.2.0 (2 realms, 9 layers, 62 nodes)
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
    expect(Object.keys(shared.layers)).toHaveLength(2);  // config, locale-knowledge
    expect(shared.layers.config.nodeTypes).toContain('Locale');
    // v10.9: locale-knowledge layer has 18 nodes (6 Sets + 6 Atoms + 6 Linguistic)
    expect(shared.layers['locale-knowledge'].nodeTypes).toHaveLength(18);
    expect(shared.layers['locale-knowledge'].nodeTypes).toContain('TermSet');
    expect(shared.layers['locale-knowledge'].nodeTypes).toContain('Term');
    expect(shared.layers['locale-knowledge'].nodeTypes).toContain('LanguageFamily');
  });

  it('should have correct org realm structure', () => {
    const org = REALM_HIERARCHY.org;
    expect(org.label).toBe('ORG');
    expect(org.icon).toBe('🏢');
    expect(Object.keys(org.layers)).toHaveLength(7);  // config, foundation, structure, semantic, seo, instruction, output
    // config layer has Organization + Tenant
    expect(org.layers.config.nodeTypes).toContain('Organization');
    expect(org.layers.config.nodeTypes).toContain('Tenant');
    expect(org.layers.foundation.nodeTypes).toContain('Project');
    // Entity/EntityContent are in org/semantic
    expect(org.layers.semantic.nodeTypes).toContain('Entity');
    expect(org.layers.semantic.nodeTypes).toContain('EntityContent');
    // v11.2: SEO is in org realm
    expect(org.layers.seo.nodeTypes).toContain('SEOKeyword');
  });

  it('should have correct node counts per layer', () => {
    // v11.1: Global realm: config (14 = 13 + EntityCategory), locale-knowledge (18) = 32
    expect(REALM_HIERARCHY.shared.layers.config.nodeTypes).toHaveLength(14);
    expect(REALM_HIERARCHY.shared.layers['locale-knowledge'].nodeTypes).toHaveLength(18);

    // v11.2: Tenant realm: config (2), foundation (3), structure (3), semantic (4), seo (8), instruction (7), output (3) = 30
    expect(REALM_HIERARCHY.org.layers.config.nodeTypes).toHaveLength(2);  // Organization, Tenant
    expect(REALM_HIERARCHY.org.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.org.layers.semantic.nodeTypes).toHaveLength(4);  // Entity, EntityContent, AudiencePersona, ChannelSurface
    expect(REALM_HIERARCHY.org.layers.seo.nodeTypes).toHaveLength(8);  // SEO + GEO (SEOMiningRun removed)
    expect(REALM_HIERARCHY.org.layers.instruction.nodeTypes).toHaveLength(7);
    expect(REALM_HIERARCHY.org.layers.output.nodeTypes).toHaveLength(3);  // v11.2: job nodes removed
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

  it('should have total of 62 nodes across all realms', () => {
    let totalNodes = 0;

    for (const realm of ['shared', 'org'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        totalNodes += layerMeta.nodeTypes.length;
      }
    }

    expect(totalNodes).toBe(62);  // v11.2: 3 job nodes removed
  });
});
