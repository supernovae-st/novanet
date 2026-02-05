// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v10.6.0 (2-Realm Architecture)
import { describe, it, expect } from 'vitest';
import { REALM_HIERARCHY, getRealmDefinition, getLayerMeta, getLayersForRealm } from '../hierarchy';
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
    expect(Object.keys(global.layers)).toHaveLength(3);  // config, locale-knowledge, seo
    expect(global.layers.config.nodeTypes).toContain('Locale');
    // locale-knowledge layer (12 nodes: 6 Sets + 6 Atoms)
    expect(global.layers['locale-knowledge'].nodeTypes).toHaveLength(12);
    expect(global.layers['locale-knowledge'].nodeTypes).toContain('TermSet');
    expect(global.layers['locale-knowledge'].nodeTypes).toContain('Term');
    expect(global.layers.seo.nodeTypes).toContain('SEOKeyword');
  });

  it('should have correct tenant realm structure', () => {
    const tenant = REALM_HIERARCHY.tenant;
    expect(tenant.label).toBe('TENANT');
    expect(tenant.icon).toBe('🏢');
    expect(Object.keys(tenant.layers)).toHaveLength(6);  // config, foundation, structure, semantic, instruction, output
    // config layer has Organization
    expect(tenant.layers.config.nodeTypes).toContain('Organization');
    expect(tenant.layers.foundation.nodeTypes).toContain('Project');
    // Entity/EntityL10n are in tenant/semantic
    expect(tenant.layers.semantic.nodeTypes).toContain('Entity');
    expect(tenant.layers.semantic.nodeTypes).toContain('EntityL10n');
  });

  it('getRealmDefinition should return realm metadata', () => {
    const def = getRealmDefinition('tenant');
    expect(def.label).toBe('TENANT');
  });

  it('getLayerMeta should return layer metadata', () => {
    const meta = getLayerMeta('tenant', 'foundation');
    expect(meta?.label).toBe('Foundation');
    expect(meta?.nodeTypes).toHaveLength(3);
  });

  it('getLayersForRealm should return all layers for a realm', () => {
    const globalLayers = getLayersForRealm('global');
    expect(globalLayers).toHaveLength(3);
    expect(globalLayers).toContain('config');
    expect(globalLayers).toContain('locale-knowledge');
    expect(globalLayers).toContain('seo');

    const tenantLayers = getLayersForRealm('tenant');
    expect(tenantLayers).toHaveLength(6);
    expect(tenantLayers).toContain('config');
    expect(tenantLayers).toContain('foundation');
    expect(tenantLayers).toContain('structure');
    expect(tenantLayers).toContain('semantic');
    expect(tenantLayers).toContain('instruction');
    expect(tenantLayers).toContain('output');
  });

  it('should have correct node counts per layer', () => {
    // Global realm: config (5), locale-knowledge (12), seo (3) = 20
    expect(REALM_HIERARCHY.global.layers.config.nodeTypes).toHaveLength(5);
    expect(REALM_HIERARCHY.global.layers['locale-knowledge'].nodeTypes).toHaveLength(12);
    expect(REALM_HIERARCHY.global.layers.seo.nodeTypes).toHaveLength(3);

    // Tenant realm: config (1), foundation (3), structure (3), semantic (4), instruction (7), output (5) = 23
    expect(REALM_HIERARCHY.tenant.layers.config.nodeTypes).toHaveLength(1);  // Organization
    expect(REALM_HIERARCHY.tenant.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.tenant.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.tenant.layers.semantic.nodeTypes).toHaveLength(4);  // Entity, EntityL10n, AudiencePersona, ChannelSurface
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

  it('should have total of 43 nodes across all realms', () => {
    let totalNodes = 0;

    for (const realm of ['global', 'tenant'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      for (const [, layerMeta] of Object.entries(def.layers)) {
        totalNodes += layerMeta.nodeTypes.length;
      }
    }

    expect(totalNodes).toBe(43);
  });
});
