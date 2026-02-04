// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v9.0.0
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 46 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(46);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBeDefined();
    }
  });

  it('should map project realm nodes correctly', () => {
    expect(NODE_LAYERS.Project).toBe('foundation');
    expect(NODE_LAYERS.BrandIdentity).toBe('foundation');
    expect(NODE_LAYERS.ProjectL10n).toBe('foundation');
    expect(NODE_LAYERS.Page).toBe('structure');
    expect(NODE_LAYERS.Block).toBe('structure');
    expect(NODE_LAYERS.Concept).toBe('semantic');
    expect(NODE_LAYERS.PageL10n).toBe('output');
    expect(NODE_LAYERS.BlockL10n).toBe('output');
  });

  it('should map global realm nodes correctly', () => {
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(NODE_LAYERS.LocaleVoice).toBe('knowledge');
    expect(NODE_LAYERS.LocaleCulture).toBe('knowledge');
    expect(NODE_LAYERS.Expression).toBe('knowledge');
  });

  it('should map shared realm nodes correctly', () => {
    expect(NODE_LAYERS.SEOKeywordL10n).toBe('seo');
    expect(NODE_LAYERS.SEOMiningRun).toBe('seo');
    expect(NODE_LAYERS.GEOSeedL10n).toBe('geo');
    expect(NODE_LAYERS.GEOMiningRun).toBe('geo');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('config');
  });

  it('getNodeTypesByLayer should return correct node types', () => {
    const foundation = getNodeTypesByLayer('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectL10n');
    expect(foundation).toHaveLength(3);
  });
});
