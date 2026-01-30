// packages/core/src/graph/__tests__/subcategories.test.ts
import { describe, it, expect } from 'vitest';
import { NODE_SUBCATEGORIES, getSubcategory, getNodeTypesBySubcategory } from '../subcategories';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/subcategories', () => {
  it('should map all 35 node types to subcategories', () => {
    const mappedTypes = Object.keys(NODE_SUBCATEGORIES);
    expect(mappedTypes).toHaveLength(35);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_SUBCATEGORIES[nodeType]).toBeDefined();
    }
  });

  it('should map Project scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.Project).toBe('foundation');
    expect(NODE_SUBCATEGORIES.BrandIdentity).toBe('foundation');
    expect(NODE_SUBCATEGORIES.ProjectL10n).toBe('foundation');
    expect(NODE_SUBCATEGORIES.Page).toBe('structure');
    expect(NODE_SUBCATEGORIES.Block).toBe('structure');
    expect(NODE_SUBCATEGORIES.Concept).toBe('semantic');
    expect(NODE_SUBCATEGORIES.PageL10n).toBe('output');
    expect(NODE_SUBCATEGORIES.BlockL10n).toBe('output');
  });

  it('should map Global scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.Locale).toBe('config');
    expect(NODE_SUBCATEGORIES.LocaleVoice).toBe('knowledge');
    expect(NODE_SUBCATEGORIES.LocaleCulture).toBe('knowledge');
    expect(NODE_SUBCATEGORIES.Expression).toBe('knowledge');
  });

  it('should map Shared scope nodes correctly', () => {
    expect(NODE_SUBCATEGORIES.SEOKeywordL10n).toBe('seo');
    expect(NODE_SUBCATEGORIES.SEOMiningRun).toBe('seo');
    expect(NODE_SUBCATEGORIES.GEOSeedL10n).toBe('geo');
    expect(NODE_SUBCATEGORIES.GEOMiningRun).toBe('geo');
  });

  it('getSubcategory should return correct subcategory', () => {
    expect(getSubcategory('Project')).toBe('foundation');
    expect(getSubcategory('Locale')).toBe('config');
  });

  it('getNodeTypesBySubcategory should return correct node types', () => {
    const foundation = getNodeTypesBySubcategory('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectL10n');
    expect(foundation).toHaveLength(3);
  });
});
