// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v11.1.0 (2 realms, 9 layers, +EntityCategory)
import { describe, it, expect } from 'vitest';
import { REALM_HIERARCHY } from '../hierarchy';
import type { Realm } from '../../types/nodes';

describe('graph/hierarchy', () => {
  it('should define all 2 realms', () => {
    const realms = Object.keys(REALM_HIERARCHY) as Realm[];
    expect(realms).toHaveLength(2);
    expect(realms).toContain('global');
    expect(realms).toContain('tenant');
  });

  it('should have correct global realm structure', () => {
    const global = REALM_HIERARCHY.global;
    expect(global.label).toBe('GLOBAL');
    expect(global.icon).toBe('🌍');
    expect(Object.keys(global.layers)).toHaveLength(2);  // config, locale-knowledge
    expect(global.layers.config.nodeTypes).toContain('Locale');
    // v10.9: locale-knowledge layer has 18 nodes (6 Sets + 6 Atoms + 6 Linguistic)
    expect(global.layers['locale-knowledge'].nodeTypes).toHaveLength(18);
    expect(global.layers['locale-knowledge'].nodeTypes).toContain('TermSet');
    expect(global.layers['locale-knowledge'].nodeTypes).toContain('Term');
    expect(global.layers['locale-knowledge'].nodeTypes).toContain('LanguageFamily');
  });

  it('should have correct tenant realm structure', () => {
    const tenant = REALM_HIERARCHY.tenant;
    expect(tenant.label).toBe('TENANT');
    expect(tenant.icon).toBe('🏢');
    expect(Object.keys(tenant.layers)).toHaveLength(7);  // config, foundation, structure, semantic, seo, instruction, output
    // config layer has Organization + Tenant
    expect(tenant.layers.config.nodeTypes).toContain('Organization');
    expect(tenant.layers.config.nodeTypes).toContain('Tenant');
    expect(tenant.layers.foundation.nodeTypes).toContain('Project');
    // Entity/EntityContent are in tenant/semantic
    expect(tenant.layers.semantic.nodeTypes).toContain('Entity');
    expect(tenant.layers.semantic.nodeTypes).toContain('EntityContent');
    // v10.9: SEO is in tenant realm
    expect(tenant.layers.seo.nodeTypes).toContain('SEOKeyword');
  });

  it('should have correct node counts per layer', () => {
    // v11.1: Global realm: config (14 = 13 + EntityCategory), locale-knowledge (18) = 32
    expect(REALM_HIERARCHY.global.layers.config.nodeTypes).toHaveLength(14);
    expect(REALM_HIERARCHY.global.layers['locale-knowledge'].nodeTypes).toHaveLength(18);

    // v11.1: Tenant realm: config (2), foundation (3), structure (3), semantic (4), seo (9), instruction (7), output (5) = 33
    expect(REALM_HIERARCHY.tenant.layers.config.nodeTypes).toHaveLength(2);  // Organization, Tenant
    expect(REALM_HIERARCHY.tenant.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.tenant.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.tenant.layers.semantic.nodeTypes).toHaveLength(4);  // Entity, EntityContent, AudiencePersona, ChannelSurface
    expect(REALM_HIERARCHY.tenant.layers.seo.nodeTypes).toHaveLength(9);  // SEO + GEO
    expect(REALM_HIERARCHY.tenant.layers.instruction.nodeTypes).toHaveLength(7);
    expect(REALM_HIERARCHY.tenant.layers.output.nodeTypes).toHaveLength(5);
  });

  it('should have valid realm definitions with required fields', () => {
    for (const realm of ['global', 'tenant'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      expect(def.realm).toBe(realm);
      expect(typeof def.label).toBe('string');
      expect(typeof def.icon).toBe('string');
      expect(typeof def.description).toBe('string');
      expect(typeof def.layers).toBe('object');
    }
  });

  it('should have valid layer metadata with required fields', () => {
    for (const realm of ['global', 'tenant'] as Realm[]) {
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

  it('should have total of 65 nodes across all realms', () => {
    let totalNodes = 0;

    for (const realm of ['global', 'tenant'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        totalNodes += layerMeta.nodeTypes.length;
      }
    }

    expect(totalNodes).toBe(65);  // v11.1: +EntityCategory
  });
});
