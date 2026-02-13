// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v0.12.0 (59 nodes, 10 layers, 2 realms)
import { describe, it, expect } from 'vitest';
import { NODE_LAYERS, getLayer, getNodeTypesByLayer } from '../layers';
import { NODE_TYPES } from '../../types/nodes';

describe('graph/layers', () => {
  it('should map all 59 node types to layers', () => {
    const mappedTypes = Object.keys(NODE_LAYERS);
    expect(mappedTypes).toHaveLength(59);

    // Every NODE_TYPE should be mapped
    for (const nodeType of NODE_TYPES) {
      expect(NODE_LAYERS[nodeType]).toBeDefined();
    }
  });

  it('should map org realm nodes correctly', () => {
    // v11.5: org realm has 6 layers (seo/geo removed, moved to shared/knowledge)
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
  });

  it('should map shared realm nodes correctly', () => {
    // v11.5: shared realm has 4 layers (config, locale, geography, knowledge)

    // config (3) - Locale + EntityCategory + SEOKeywordFormat
    expect(NODE_LAYERS.EntityCategory).toBe('config');
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(NODE_LAYERS.SEOKeywordFormat).toBe('config');

    // locale (6) - locale settings (not including Locale node itself)
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

    // knowledge (24) - sets, atoms, linguistic taxonomy, SEO/GEO (v11.5: moved from org)
    expect(NODE_LAYERS.TermSet).toBe('knowledge');
    expect(NODE_LAYERS.Term).toBe('knowledge');
    expect(NODE_LAYERS.ExpressionSet).toBe('knowledge');
    expect(NODE_LAYERS.Expression).toBe('knowledge');
    expect(NODE_LAYERS.PatternSet).toBe('knowledge');
    expect(NODE_LAYERS.Pattern).toBe('knowledge');
    expect(NODE_LAYERS.LanguageFamily).toBe('knowledge');
    expect(NODE_LAYERS.LanguageBranch).toBe('knowledge');

    // v11.5: SEO/GEO in shared/knowledge (6 nodes)
    expect(NODE_LAYERS.SEOKeyword).toBe('knowledge');
    expect(NODE_LAYERS.SEOKeywordMetrics).toBe('knowledge');
    expect(NODE_LAYERS.SEOKeywordSet).toBe('knowledge');
    expect(NODE_LAYERS.GEOQuery).toBe('knowledge');
    expect(NODE_LAYERS.GEOQuerySet).toBe('knowledge');
    expect(NODE_LAYERS.GEOAnswer).toBe('knowledge');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('config');
    expect(getLayer('Entity')).toBe('semantic');
    expect(getLayer('Term')).toBe('knowledge');
    expect(getLayer('Continent')).toBe('geography');
    expect(getLayer('GEOQuery')).toBe('knowledge');
    expect(getLayer('EntityCategory')).toBe('config');
    expect(getLayer('SEOKeywordFormat')).toBe('config');
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

    // v11.5: knowledge layer has 24 nodes (includes SEO/GEO)
    const knowledge = getNodeTypesByLayer('knowledge');
    expect(knowledge).toContain('TermSet');
    expect(knowledge).toContain('Term');
    expect(knowledge).toContain('ExpressionSet');
    expect(knowledge).toContain('Expression');
    expect(knowledge).toContain('LanguageFamily');
    expect(knowledge).toContain('SEOKeyword');
    expect(knowledge).toContain('SEOKeywordSet');
    expect(knowledge).toContain('GEOQuery');
    expect(knowledge).toContain('GEOQuerySet');
    expect(knowledge).toHaveLength(24);

    // v11.5: locale layer has 6 nodes (Locale is in config)
    const locale = getNodeTypesByLayer('locale');
    expect(locale).toContain('Style');
    expect(locale).toContain('Formatting');
    expect(locale).toContain('Adaptation');
    expect(locale).toHaveLength(6);

    // v11.5: geography layer has 6 nodes
    const geography = getNodeTypesByLayer('geography');
    expect(geography).toContain('Continent');
    expect(geography).toContain('GeoRegion');
    expect(geography).toHaveLength(6);

    // v11.5: config layer has 4 nodes (3 shared + 1 org)
    const config = getNodeTypesByLayer('config');
    expect(config).toContain('OrgConfig');
    expect(config).toContain('EntityCategory');
    expect(config).toContain('Locale');
    expect(config).toContain('SEOKeywordFormat');
    expect(config).toHaveLength(4);
  });
});
