// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v11.3.0 (61 nodes, 11 layers, 2 realms)
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 61 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(61);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBeDefined();
    }
  });

  it('should map org realm nodes correctly', () => {
    // v11.3: org realm has 8 layers
    // config (1): OrgConfig
    expect(NODE_LAYERS.OrgConfig).toBe('config');

    // foundation (3)
    expect(NODE_LAYERS.Project).toBe('foundation');
    expect(NODE_LAYERS.BrandIdentity).toBe('foundation');
    expect(NODE_LAYERS.ProjectContent).toBe('foundation');

    // structure (3)
    expect(NODE_LAYERS.Page).toBe('structure');
    expect(NODE_LAYERS.Block).toBe('structure');
    expect(NODE_LAYERS.ContentSlot).toBe('structure');

    // semantic (4)
    expect(NODE_LAYERS.AudiencePersona).toBe('semantic');
    expect(NODE_LAYERS.ChannelSurface).toBe('semantic');
    expect(NODE_LAYERS.Entity).toBe('semantic');
    expect(NODE_LAYERS.EntityContent).toBe('semantic');

    // output (3)
    expect(NODE_LAYERS.PageGenerated).toBe('output');
    expect(NODE_LAYERS.BlockGenerated).toBe('output');
    expect(NODE_LAYERS.OutputArtifact).toBe('output');

    // seo (5) - v11.3: SEO only, no GEO
    expect(NODE_LAYERS.SEOKeyword).toBe('seo');
    expect(NODE_LAYERS.SEOKeywordMetrics).toBe('seo');

    // geo (3) - v11.3: new layer split from SEO
    expect(NODE_LAYERS.GEOQuery).toBe('geo');
    expect(NODE_LAYERS.GEOAnswer).toBe('geo');
    expect(NODE_LAYERS.GEOMetrics).toBe('geo');
  });

  it('should map shared realm nodes correctly', () => {
    // v11.3: shared realm has 3 layers (locale, geography, knowledge)

    // locale (7) - locale definitions and settings
    expect(NODE_LAYERS.Locale).toBe('locale');
    expect(NODE_LAYERS.Style).toBe('locale');
    expect(NODE_LAYERS.Formatting).toBe('locale');
    expect(NODE_LAYERS.Adaptation).toBe('locale');
    expect(NODE_LAYERS.Slugification).toBe('locale');
    expect(NODE_LAYERS.Culture).toBe('locale');
    expect(NODE_LAYERS.Market).toBe('locale');

    // geography (6) - geographic classifications
    expect(NODE_LAYERS.Continent).toBe('geography');
    expect(NODE_LAYERS.GeoRegion).toBe('geography');
    expect(NODE_LAYERS.GeoSubRegion).toBe('geography');
    expect(NODE_LAYERS.EconomicRegion).toBe('geography');
    expect(NODE_LAYERS.IncomeGroup).toBe('geography');
    expect(NODE_LAYERS.LendingCategory).toBe('geography');

    // knowledge (19) - sets, atoms, linguistic taxonomy
    expect(NODE_LAYERS.TermSet).toBe('knowledge');
    expect(NODE_LAYERS.Term).toBe('knowledge');
    expect(NODE_LAYERS.ExpressionSet).toBe('knowledge');
    expect(NODE_LAYERS.Expression).toBe('knowledge');
    expect(NODE_LAYERS.PatternSet).toBe('knowledge');
    expect(NODE_LAYERS.Pattern).toBe('knowledge');
    expect(NODE_LAYERS.LanguageFamily).toBe('knowledge');
    expect(NODE_LAYERS.LanguageBranch).toBe('knowledge');
    expect(NODE_LAYERS.EntityCategory).toBe('knowledge');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('locale');
    expect(getLayer('Entity')).toBe('semantic');
    expect(getLayer('Term')).toBe('knowledge');
    expect(getLayer('Continent')).toBe('geography');
    expect(getLayer('GEOQuery')).toBe('geo');
  });

  it('getNodeTypesByLayer should return correct node types', () => {
    const foundation = getNodeTypesByLayer('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('BrandIdentity');
    expect(foundation).toContain('ProjectContent');
    expect(foundation).toHaveLength(3);

    // semantic layer has 4 nodes
    const semantic = getNodeTypesByLayer('semantic');
    expect(semantic).toContain('AudiencePersona');
    expect(semantic).toContain('ChannelSurface');
    expect(semantic).toContain('Entity');
    expect(semantic).toContain('EntityContent');
    expect(semantic).toHaveLength(4);

    // v11.3: knowledge layer has 19 nodes
    const knowledge = getNodeTypesByLayer('knowledge');
    expect(knowledge).toContain('TermSet');
    expect(knowledge).toContain('Term');
    expect(knowledge).toContain('ExpressionSet');
    expect(knowledge).toContain('Expression');
    expect(knowledge).toContain('LanguageFamily');
    expect(knowledge).toContain('EntityCategory');
    expect(knowledge).toHaveLength(19);

    // v11.3: locale layer has 7 nodes
    const locale = getNodeTypesByLayer('locale');
    expect(locale).toContain('Locale');
    expect(locale).toContain('Style');
    expect(locale).toContain('Formatting');
    expect(locale).toHaveLength(7);

    // v11.3: geography layer has 6 nodes
    const geography = getNodeTypesByLayer('geography');
    expect(geography).toContain('Continent');
    expect(geography).toContain('GeoRegion');
    expect(geography).toHaveLength(6);

    // v11.3: geo layer has 3 nodes (split from seo)
    const geo = getNodeTypesByLayer('geo');
    expect(geo).toContain('GEOQuery');
    expect(geo).toContain('GEOAnswer');
    expect(geo).toContain('GEOMetrics');
    expect(geo).toHaveLength(3);

    // v11.3: config layer has 1 node (OrgConfig only)
    const config = getNodeTypesByLayer('config');
    expect(config).toContain('OrgConfig');
    expect(config).toHaveLength(1);
  });
});
