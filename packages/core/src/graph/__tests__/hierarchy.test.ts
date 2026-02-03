// packages/core/src/graph/__tests__/hierarchy.test.ts
// Tests for REALM_HIERARCHY — v9.0.0
import { describe, it, expect } from 'vitest';
import { REALM_HIERARCHY, getRealmDefinition, getLayerMeta, getLayersForRealm } from '../hierarchy';
import type { Realm } from '../../types/nodes';

describe('graph/hierarchy', () => {
  it('should define all 3 realms', () => {
    const realms = Object.keys(REALM_HIERARCHY) as Realm[];
    expect(realms).toHaveLength(3);
    expect(realms).toContain('project');
    expect(realms).toContain('global');
    expect(realms).toContain('shared');
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
    expect(Object.keys(global.layers)).toHaveLength(2);
    expect(global.layers.config.nodeTypes).toContain('Locale');
    expect(global.layers.knowledge.nodeTypes).toHaveLength(14);
  });

  it('should have correct shared realm structure', () => {
    const shared = REALM_HIERARCHY.shared;
    expect(shared.label).toBe('SHARED');
    expect(shared.icon).toBe('🎯');
    expect(Object.keys(shared.layers)).toHaveLength(2);
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
    expect(globalLayers).toHaveLength(2);
    expect(globalLayers).toContain('config');
    expect(globalLayers).toContain('knowledge');

    const sharedLayers = getLayersForRealm('shared');
    expect(sharedLayers).toHaveLength(2);
    expect(sharedLayers).toContain('seo');
    expect(sharedLayers).toContain('geo');
  });

  it('should have correct node counts per layer', () => {
    // Project realm: foundation (3), structure (3), semantic (4), instruction (6), output (5) = 21
    expect(REALM_HIERARCHY.project.layers.foundation.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.project.layers.structure.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.project.layers.semantic.nodeTypes).toHaveLength(4);
    expect(REALM_HIERARCHY.project.layers.instruction.nodeTypes).toHaveLength(6);
    expect(REALM_HIERARCHY.project.layers.output.nodeTypes).toHaveLength(5);

    // Global realm: config (1), knowledge (14) = 15
    expect(REALM_HIERARCHY.global.layers.config.nodeTypes).toHaveLength(1);
    expect(REALM_HIERARCHY.global.layers.knowledge.nodeTypes).toHaveLength(14);

    // Shared realm: seo (3), geo (5) = 8
    expect(REALM_HIERARCHY.shared.layers.seo.nodeTypes).toHaveLength(3);
    expect(REALM_HIERARCHY.shared.layers.geo.nodeTypes).toHaveLength(5);
  });

  it('should have valid realm definitions with required fields', () => {
    for (const realm of ['project', 'global', 'shared'] as Realm[]) {
      const def = REALM_HIERARCHY[realm];
      expect(def.realm).toBe(realm);
      expect(typeof def.label).toBe('string');
      expect(typeof def.icon).toBe('string');
      expect(typeof def.description).toBe('string');
      expect(typeof def.layers).toBe('object');
    }
  });

  it('should have valid layer metadata with required fields', () => {
    for (const realm of ['project', 'global', 'shared'] as Realm[]) {
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
