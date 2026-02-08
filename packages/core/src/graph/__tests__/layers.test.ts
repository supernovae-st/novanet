// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v10.6.0 (43 nodes, 2 realms)
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 43 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(43);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBeDefined();
    }
  });

  it('should map tenant realm nodes correctly', () => {
    expect(NODE_LAYERS.Project).toBe('foundation');
    expect(NODE_LAYERS.BrandIdentity).toBe('foundation');
    expect(NODE_LAYERS.ProjectL10n).toBe('foundation');
    expect(NODE_LAYERS.Page).toBe('structure');
    expect(NODE_LAYERS.Block).toBe('structure');
    expect(NODE_LAYERS.AudiencePersona).toBe('semantic');
    expect(NODE_LAYERS.ChannelSurface).toBe('semantic');
    // Entity-Centric in tenant.semantic
    expect(NODE_LAYERS.Entity).toBe('semantic');
    expect(NODE_LAYERS.EntityContent).toBe('semantic');
    expect(NODE_LAYERS.PageGenerated).toBe('output');
    expect(NODE_LAYERS.BlockGenerated).toBe('output');
  });

  it('should map global realm nodes correctly', () => {
    // config layer has 5 nodes
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(NODE_LAYERS.Style).toBe('config');
    expect(NODE_LAYERS.Formatting).toBe('config');
    expect(NODE_LAYERS.Adaptation).toBe('config');
    expect(NODE_LAYERS.Slugification).toBe('config');
    // locale-knowledge layer has 12 nodes (Sets + Atoms only)
    expect(NODE_LAYERS.ExpressionSet).toBe('locale-knowledge');
    expect(NODE_LAYERS.Term).toBe('locale-knowledge');
    expect(NODE_LAYERS.Expression).toBe('locale-knowledge');
    // SEO in global realm
    expect(NODE_LAYERS.SEOKeyword).toBe('seo');
    expect(NODE_LAYERS.SEOMiningRun).toBe('seo');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('config');
    expect(getLayer('Entity')).toBe('semantic');  // in tenant.semantic
  });

  it('getNodeTypesByLayer should return correct node types', () => {
    const foundation = getNodeTypesByLayer('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectL10n');
    expect(foundation).toHaveLength(3);

    // semantic layer has 4 nodes (Entity-Centric moved here)
    const semantic = getNodeTypesByLayer('semantic');
    expect(semantic).toContain('AudiencePersona');
    expect(semantic).toContain('ChannelSurface');
    expect(semantic).toContain('Entity');
    expect(semantic).toContain('EntityContent');
    expect(semantic).toHaveLength(4);

    // locale-knowledge layer has 12 nodes (Sets + Atoms only)
    const knowledge = getNodeTypesByLayer('locale-knowledge');
    expect(knowledge).toContain('TermSet');
    expect(knowledge).toContain('Term');
    expect(knowledge).toContain('ExpressionSet');
    expect(knowledge).toContain('Expression');
    expect(knowledge).toHaveLength(12);

    // v10.6: config layer has 6 nodes (5 global + 1 tenant Organization)
    const config = getNodeTypesByLayer('config');
    expect(config).toContain('Locale');
    expect(config).toContain('Style');
    expect(config).toContain('Formatting');
    expect(config).toContain('Organization');
    expect(config).toHaveLength(6);
  });
});
