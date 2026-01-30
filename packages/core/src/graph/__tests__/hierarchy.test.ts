// packages/core/src/graph/__tests__/hierarchy.test.ts
import { describe, it, expect } from 'vitest';
import { SCOPE_HIERARCHY, getScopeDefinition, getSubcategoryMeta, getSubcategoriesForScope } from '../hierarchy';
import type { Scope } from '../../types/nodes';

describe('graph/hierarchy', () => {
  it('should define all 3 scopes', () => {
    const scopes = Object.keys(SCOPE_HIERARCHY) as Scope[];
    expect(scopes).toHaveLength(3);
    expect(scopes).toContain('Project');
    expect(scopes).toContain('Global');
    expect(scopes).toContain('Shared');
  });

  it('should have correct Project scope structure', () => {
    const project = SCOPE_HIERARCHY.Project;
    expect(project.label).toBe('PROJECT');
    expect(project.icon).toBe('📦');
    expect(Object.keys(project.subcategories)).toHaveLength(5);
    expect(project.subcategories.foundation.nodeTypes).toContain('Project');
  });

  it('should have correct Global scope structure', () => {
    const global = SCOPE_HIERARCHY.Global;
    expect(global.label).toBe('GLOBAL');
    expect(global.icon).toBe('🌍');
    expect(Object.keys(global.subcategories)).toHaveLength(2);
    expect(global.subcategories.config.nodeTypes).toContain('Locale');
    expect(global.subcategories.knowledge.nodeTypes).toHaveLength(14);
  });

  it('should have correct Shared scope structure', () => {
    const shared = SCOPE_HIERARCHY.Shared;
    expect(shared.label).toBe('SHARED');
    expect(shared.icon).toBe('🎯');
    expect(Object.keys(shared.subcategories)).toHaveLength(2);
  });

  it('getScopeDefinition should return scope metadata', () => {
    const def = getScopeDefinition('Project');
    expect(def.label).toBe('PROJECT');
  });

  it('getSubcategoryMeta should return subcategory metadata', () => {
    const meta = getSubcategoryMeta('Project', 'foundation');
    expect(meta?.label).toBe('Foundation');
    expect(meta?.nodeTypes).toHaveLength(3);
  });

  it('getSubcategoriesForScope should return all subcategories for a scope', () => {
    const projectSubcats = getSubcategoriesForScope('Project');
    expect(projectSubcats).toHaveLength(5);
    expect(projectSubcats).toContain('foundation');
    expect(projectSubcats).toContain('structure');
    expect(projectSubcats).toContain('semantic');
    expect(projectSubcats).toContain('instruction');
    expect(projectSubcats).toContain('output');

    const globalSubcats = getSubcategoriesForScope('Global');
    expect(globalSubcats).toHaveLength(2);
    expect(globalSubcats).toContain('config');
    expect(globalSubcats).toContain('knowledge');

    const sharedSubcats = getSubcategoriesForScope('Shared');
    expect(sharedSubcats).toHaveLength(2);
    expect(sharedSubcats).toContain('seo');
    expect(sharedSubcats).toContain('geo');
  });

  it('should have correct node counts per subcategory', () => {
    // Project scope: foundation (3), structure (2), semantic (2), instruction (5), output (2) = 14
    // Source of truth: models/nodes/project/ folder structure
    expect(SCOPE_HIERARCHY.Project.subcategories.foundation.nodeTypes).toHaveLength(3);
    expect(SCOPE_HIERARCHY.Project.subcategories.structure.nodeTypes).toHaveLength(2);  // Page, Block
    expect(SCOPE_HIERARCHY.Project.subcategories.semantic.nodeTypes).toHaveLength(2);
    expect(SCOPE_HIERARCHY.Project.subcategories.instruction.nodeTypes).toHaveLength(5);  // PageType, PagePrompt, BlockType, BlockPrompt, BlockRules
    expect(SCOPE_HIERARCHY.Project.subcategories.output.nodeTypes).toHaveLength(2);

    // Global scope: config (1), knowledge (14) = 15
    expect(SCOPE_HIERARCHY.Global.subcategories.config.nodeTypes).toHaveLength(1);
    expect(SCOPE_HIERARCHY.Global.subcategories.knowledge.nodeTypes).toHaveLength(14);

    // Shared scope: seo (3), geo (3) = 6
    expect(SCOPE_HIERARCHY.Shared.subcategories.seo.nodeTypes).toHaveLength(3);
    expect(SCOPE_HIERARCHY.Shared.subcategories.geo.nodeTypes).toHaveLength(3);
  });

  it('should have valid scope definitions with required fields', () => {
    for (const scope of ['Project', 'Global', 'Shared'] as Scope[]) {
      const def = SCOPE_HIERARCHY[scope];
      expect(def.scope).toBe(scope);
      expect(typeof def.label).toBe('string');
      expect(typeof def.icon).toBe('string');
      expect(typeof def.description).toBe('string');
      expect(typeof def.subcategories).toBe('object');
    }
  });

  it('should have valid subcategory metadata with required fields', () => {
    for (const scope of ['Project', 'Global', 'Shared'] as Scope[]) {
      const def = SCOPE_HIERARCHY[scope];
      for (const [, subcatMeta] of Object.entries(def.subcategories)) {
        expect(typeof subcatMeta.label).toBe('string');
        expect(typeof subcatMeta.description).toBe('string');
        expect(typeof subcatMeta.icon).toBe('string');
        expect(Array.isArray(subcatMeta.nodeTypes)).toBe(true);
        expect(subcatMeta.nodeTypes.length).toBeGreaterThan(0);
      }
    }
  });
});
