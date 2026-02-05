// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v10.3.0 (Entity-Centric, GEO removed)
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
    expect(Object.keys(global.layers)).toHaveLength(4);  // v10.3: config, knowledge, seo, semantic
    expect(global.layers.config.nodeTypes).toContain('Locale');
    expect(global.layers.knowledge.nodeTypes).toHaveLength(16);  // v10.3: containers (10) + atoms (6)
    expect(global.layers.seo.nodeTypes).toContain('SEOKeyword');  // v10.3: SEO in global
    expect(global.layers.semantic.nodeTypes).toContain('Entity');  // v10.3: Entity in global
    expect(global.layers.semantic.nodeTypes).toHaveLength(2);  // v10.3: Entity, EntityL10n only
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
    expect(globalLayers).toHaveLength(4);  // v10.3: config, knowledge, seo, semantic
    expect(globalLayers).toContain('config');
    expect(globalLayers).toContain('knowledge');
    expect(globalLayers).toContain('seo');
    expect(globalLayers).toContain('semantic');
  });

  it('should have correct node counts per layer', () => {
    // Project realm: foundation (3), structure (5), semantic (2), instruction (5), output (5) = 20
    expect(REALM_HIERARCHY.project.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.project.layers.structure.nodeTypes).toHaveLength(5);
    expect(REALM_HIERARCHY.project.layers.semantic.nodeTypes).toHaveLength(2);  // v10.3: AudiencePersona, ChannelSurface
    expect(REALM_HIERARCHY.project.layers.instruction.nodeTypes).toHaveLength(5);
    expect(REALM_HIERARCHY.project.layers.output.nodeTypes).toHaveLength(5);

    // Global realm: config (1), knowledge (16), seo (3), semantic (2) = 22
    expect(REALM_HIERARCHY.global.layers.config.nodeTypes).toHaveLength(1);
    expect(REALM_HIERARCHY.global.layers.knowledge.nodeTypes).toHaveLength(16);  // v10.3: containers (10) + atoms (6)
    expect(REALM_HIERARCHY.global.layers.seo.nodeTypes).toHaveLength(3);  // v10.3: SEO in global
    expect(REALM_HIERARCHY.global.layers.semantic.nodeTypes).toHaveLength(2);  // v10.3: Entity, EntityL10n
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
