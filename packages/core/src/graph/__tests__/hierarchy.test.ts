// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v10.4.0 (Entity-Centric in knowledge layer, 2 realms)
import { describe, it, expect } from 'vitest';
import { REALM_HIERARCHY, getRealmDefinition, getLayerMeta, getLayersForRealm } from '../hierarchy';
import type { Realm } from '../../types/nodes';

describe('graph/hierarchy', () => {
  it('should define all 2 realms', () => {
    const realms = Object.keys(REALM_HIERARCHY) as Realm[];
    expect(realms).toHaveLength(2);
    expect(realms).toContain('project');
    expect(realms).toContain('global');
  });

  it('should have correct project realm structure', () => {
    const project = REALM_HIERARCHY.project;
    expect(project.label).toBe('PROJECT');
    expect(project.icon).toBe('📦');
    expect(Object.keys(project.layers)).toHaveLength(5);
    expect(project.layers.foundation.nodeTypes).toContain('Project');
  });

  it('should have correct global realm structure', () => {
    const global = REALM_HIERARCHY.global;
    expect(global.label).toBe('GLOBAL');
    expect(global.icon).toBe('🌍');
    expect(Object.keys(global.layers)).toHaveLength(3);  // v10.4: config, knowledge, seo
    expect(global.layers.config.nodeTypes).toContain('Locale');
    // v10.4: knowledge layer includes Entity-Centric nodes (18 total)
    expect(global.layers.knowledge.nodeTypes).toHaveLength(18);
    expect(global.layers.knowledge.nodeTypes).toContain('Entity');
    expect(global.layers.knowledge.nodeTypes).toContain('EntityL10n');
    expect(global.layers.seo.nodeTypes).toContain('SEOKeyword');
  });

  it('getRealmDefinition should return realm metadata', () => {
    const def = getRealmDefinition('project');
    expect(def.label).toBe('PROJECT');
  });

  it('getLayerMeta should return layer metadata', () => {
    const meta = getLayerMeta('project', 'foundation');
    expect(meta?.label).toBe('Foundation');
    expect(meta?.nodeTypes).toHaveLength(3);
  });

  it('getLayersForRealm should return all layers for a realm', () => {
    const projectLayers = getLayersForRealm('project');
    expect(projectLayers).toHaveLength(5);
    expect(projectLayers).toContain('foundation');
    expect(projectLayers).toContain('structure');
    expect(projectLayers).toContain('semantic');
    expect(projectLayers).toContain('instruction');
    expect(projectLayers).toContain('output');

    const globalLayers = getLayersForRealm('global');
    expect(globalLayers).toHaveLength(3);  // v10.4: config, knowledge, seo
    expect(globalLayers).toContain('config');
    expect(globalLayers).toContain('knowledge');
    expect(globalLayers).toContain('seo');
  });

  it('should have correct node counts per layer', () => {
    // Project realm: foundation (3), structure (3), semantic (2), instruction (7), output (5) = 20
    expect(REALM_HIERARCHY.project.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.project.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.project.layers.semantic.nodeTypes).toHaveLength(2);  // AudiencePersona, ChannelSurface
    expect(REALM_HIERARCHY.project.layers.instruction.nodeTypes).toHaveLength(7);
    expect(REALM_HIERARCHY.project.layers.output.nodeTypes).toHaveLength(5);

    // Global realm: config (1), knowledge (18), seo (3) = 22
    expect(REALM_HIERARCHY.global.layers.config.nodeTypes).toHaveLength(1);
    expect(REALM_HIERARCHY.global.layers.knowledge.nodeTypes).toHaveLength(18);  // includes Entity, EntityL10n
    expect(REALM_HIERARCHY.global.layers.seo.nodeTypes).toHaveLength(3);
  });

  it('should have valid realm definitions with required fields', () => {
    for (const realm of ['project', 'global'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      expect(def.realm).toBe(realm);
      expect(typeof def.label).toBe('string');
      expect(typeof def.icon).toBe('string');
      expect(typeof def.description).toBe('string');
      expect(typeof def.layers).toBe('object');
    }
  });

  it('should have valid layer metadata with required fields', () => {
    for (const realm of ['project', 'global'] as Realm[]) {
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
});
