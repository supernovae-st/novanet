// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v11.2.0 (62 nodes, 2 realms, 3 job nodes removed)
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 62 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(62);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBeDefined();
    }
  });

  it('should map org realm nodes correctly', () => {
    expect(NODE_LAYERS.Project).toBe('foundation');
    expect(NODE_LAYERS.BrandIdentity).toBe('foundation');
    expect(NODE_LAYERS.ProjectContent).toBe('foundation');
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
    // v11.1: config layer (global) has 14 nodes including EntityCategory
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(NODE_LAYERS.EntityCategory).toBe('config');
    expect(NODE_LAYERS.Style).toBe('config');
    expect(NODE_LAYERS.Formatting).toBe('config');
    expect(NODE_LAYERS.Adaptation).toBe('config');
    expect(NODE_LAYERS.Slugification).toBe('config');
    expect(NODE_LAYERS.Culture).toBe('config');
    expect(NODE_LAYERS.Market).toBe('config');
    expect(NODE_LAYERS.Continent).toBe('config');
    // locale-knowledge layer has 18 nodes (Sets + Atoms + Linguistic taxonomy)
    expect(NODE_LAYERS.ExpressionSet).toBe('locale-knowledge');
    expect(NODE_LAYERS.Term).toBe('locale-knowledge');
    expect(NODE_LAYERS.Expression).toBe('locale-knowledge');
    expect(NODE_LAYERS.LanguageFamily).toBe('locale-knowledge');
    // SEO layer has 9 nodes
    expect(NODE_LAYERS.SEOKeyword).toBe('seo');
    expect(NODE_LAYERS.SEOKeyword).toBe('seo');
    expect(NODE_LAYERS.GEOQuery).toBe('seo');
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
    expect(foundation).toContain('ProjectContent');
    expect(foundation).toHaveLength(3);

    // semantic layer has 4 nodes (Entity-Centric)
    const semantic = getNodeTypesByLayer('semantic');
    expect(semantic).toContain('AudiencePersona');
    expect(semantic).toContain('ChannelSurface');
    expect(semantic).toContain('Entity');
    expect(semantic).toContain('EntityContent');
    expect(semantic).toHaveLength(4);

    // v10.9: locale-knowledge layer has 18 nodes (Sets + Atoms + Linguistic taxonomy)
    const knowledge = getNodeTypesByLayer('locale-knowledge');
    expect(knowledge).toContain('TermSet');
    expect(knowledge).toContain('Term');
    expect(knowledge).toContain('ExpressionSet');
    expect(knowledge).toContain('Expression');
    expect(knowledge).toContain('LanguageFamily');
    expect(knowledge).toHaveLength(18);

    // v11.1: config layer has 16 nodes (14 global + 2 tenant)
    const config = getNodeTypesByLayer('config');
    expect(config).toContain('Locale');
    expect(config).toContain('Style');
    expect(config).toContain('Formatting');
    expect(config).toContain('Organization');
    expect(config).toContain('Tenant');
    expect(config).toContain('Continent');
    expect(config).toContain('EntityCategory');
    expect(config).toHaveLength(16);
  });
});
