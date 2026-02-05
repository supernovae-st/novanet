// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v10.3.0 (Entity-Centric, GEO removed)
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 42 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(42);

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
    expect(NODE_LAYERS.AudiencePersona).toBe('semantic');  // v10.3: project.semantic
    expect(NODE_LAYERS.ChannelSurface).toBe('semantic');   // v10.3: project.semantic
    expect(NODE_LAYERS.PageL10n).toBe('output');
    expect(NODE_LAYERS.BlockL10n).toBe('output');
  });

  it('should map global realm nodes correctly', () => {
    expect(NODE_LAYERS.Locale).toBe('config');
    // v10.3: knowledge tier model (containers + atoms)
    expect(NODE_LAYERS.Style).toBe('knowledge');
    expect(NODE_LAYERS.ExpressionSet).toBe('knowledge');
    expect(NODE_LAYERS.Formatting).toBe('knowledge');
    expect(NODE_LAYERS.Term).toBe('knowledge');  // v10.3: knowledge atoms
    expect(NODE_LAYERS.Expression).toBe('knowledge');
    // v10.3: SEO in global realm
    expect(NODE_LAYERS.SEOKeyword).toBe('seo');
    expect(NODE_LAYERS.SEOMiningRun).toBe('seo');
    // v10.3: Entity in global.semantic
    expect(NODE_LAYERS.Entity).toBe('semantic');
    expect(NODE_LAYERS.EntityL10n).toBe('semantic');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('config');
    expect(getLayer('Entity')).toBe('semantic');  // v10.3
  });

  it('getNodeTypesByLayer should return correct node types', () => {
    const foundation = getNodeTypesByLayer('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectL10n');
    expect(foundation).toHaveLength(3);

    // v10.3: semantic layer has Entity in global and AudiencePersona/ChannelSurface in project
    const semantic = getNodeTypesByLayer('semantic');
    expect(semantic).toContain('Entity');
    expect(semantic).toContain('EntityL10n');
    expect(semantic).toContain('AudiencePersona');
    expect(semantic).toContain('ChannelSurface');
    expect(semantic).toHaveLength(4);
  });
});
