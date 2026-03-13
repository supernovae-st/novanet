// packages/core/src/graph/__tests__/layers.test.ts
// Tests for NODE_LAYERS — v0.19.1 (59 nodes, 10 layers, 2 realms)
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
    // v0.17.0: org realm has 6 layers (seo/geo removed, moved to shared/knowledge)
    // config (1): OrgConfig
    expect(NODE_LAYERS.OrgConfig).toBe('config');

    // foundation (8) — v0.17.0: ProjectGEOScope added
    expect(NODE_LAYERS.Project).toBe('foundation');
    expect(NODE_LAYERS.Brand).toBe('foundation');
    expect(NODE_LAYERS.BrandDesign).toBe('foundation');
    expect(NODE_LAYERS.BrandPrinciples).toBe('foundation');
    expect(NODE_LAYERS.PromptStyle).toBe('foundation');
    expect(NODE_LAYERS.ProjectNative).toBe('foundation');
    expect(NODE_LAYERS.ProjectSEOScope).toBe('foundation');
    expect(NODE_LAYERS.ProjectGEOScope).toBe('foundation');

    // structure (3)
    expect(NODE_LAYERS.Page).toBe('structure');
    expect(NODE_LAYERS.Block).toBe('structure');
    expect(NODE_LAYERS.ContentSlot).toBe('structure');

    // semantic (2) — v0.17.0: AudiencePersona, ChannelSurface removed
    expect(NODE_LAYERS.Entity).toBe('semantic');
    expect(NODE_LAYERS.EntityNative).toBe('semantic');

    // output (3)
    expect(NODE_LAYERS.PageNative).toBe('output');
    expect(NODE_LAYERS.BlockNative).toBe('output');
    expect(NODE_LAYERS.OutputArtifact).toBe('output');
  });

  it('should map shared realm nodes correctly', () => {
    // v0.17.0: shared realm has 4 layers (config, locale, geography, knowledge)

    // config (3) - Locale + EntityCategory + SEOKeywordFormat
    expect(NODE_LAYERS.EntityCategory).toBe('config');
    expect(NODE_LAYERS.Locale).toBe('config');
    expect(NODE_LAYERS.SEOKeywordFormat).toBe('config');

    // locale (5) - locale settings (v0.17.0: Market removed)
    expect(NODE_LAYERS.Style).toBe('locale');
    expect(NODE_LAYERS.Formatting).toBe('locale');
    expect(NODE_LAYERS.Adaptation).toBe('locale');
    expect(NODE_LAYERS.Slugification).toBe('locale');
    expect(NODE_LAYERS.Culture).toBe('locale');

    // geography (7) - geographic classifications — v0.12.4: Country added
    expect(NODE_LAYERS.Continent).toBe('geography');
    expect(NODE_LAYERS.Country).toBe('geography');
    expect(NODE_LAYERS.GeoRegion).toBe('geography');
    expect(NODE_LAYERS.GeoSubRegion).toBe('geography');
    expect(NODE_LAYERS.EconomicRegion).toBe('geography');
    expect(NODE_LAYERS.IncomeGroup).toBe('geography');
    expect(NODE_LAYERS.LendingCategory).toBe('geography');

    // knowledge (21) - sets, atoms, linguistic taxonomy, SEO/GEO (v0.17.0: TermSet, Term, SEOKeywordMetrics removed)
    expect(NODE_LAYERS.ExpressionSet).toBe('knowledge');
    expect(NODE_LAYERS.Expression).toBe('knowledge');
    expect(NODE_LAYERS.PatternSet).toBe('knowledge');
    expect(NODE_LAYERS.Pattern).toBe('knowledge');
    expect(NODE_LAYERS.LanguageFamily).toBe('knowledge');
    expect(NODE_LAYERS.LanguageBranch).toBe('knowledge');

    // v11.5: SEO/GEO in shared/knowledge (5 nodes, v0.17.0: SEOKeywordMetrics removed)
    expect(NODE_LAYERS.SEOKeyword).toBe('knowledge');
    expect(NODE_LAYERS.SEOKeywordSet).toBe('knowledge');
    expect(NODE_LAYERS.GEOQuery).toBe('knowledge');
    expect(NODE_LAYERS.GEOQuerySet).toBe('knowledge');
    expect(NODE_LAYERS.GEOAnswer).toBe('knowledge');
  });

  it('getLayer should return correct layer', () => {
    expect(getLayer('Project')).toBe('foundation');
    expect(getLayer('Locale')).toBe('config');
    expect(getLayer('Entity')).toBe('semantic');
    expect(getLayer('Expression')).toBe('knowledge'); // v0.17.0: Term removed
    expect(getLayer('Continent')).toBe('geography');
    expect(getLayer('GEOQuery')).toBe('knowledge');
    expect(getLayer('EntityCategory')).toBe('config');
    expect(getLayer('SEOKeywordFormat')).toBe('config');
  });

  it('getNodeTypesByLayer should return correct node types', () => {
    // v0.17.0: foundation layer has 8 nodes (ProjectGEOScope added)
    const foundation = getNodeTypesByLayer('foundation');
    expect(foundation).toContain('Project');
    expect(foundation).toContain('Brand');
    expect(foundation).toContain('BrandDesign');
    expect(foundation).toContain('BrandPrinciples');
    expect(foundation).toContain('PromptStyle');
    expect(foundation).toContain('ProjectNative');
    expect(foundation).toContain('ProjectSEOScope');
    expect(foundation).toContain('ProjectGEOScope');
    expect(foundation).toHaveLength(8);

    // v0.17.0: semantic layer has 2 nodes (AudiencePersona, ChannelSurface removed)
    const semantic = getNodeTypesByLayer('semantic');
    expect(semantic).toContain('Entity');
    expect(semantic).toContain('EntityNative');
    expect(semantic).toHaveLength(2);

    // v0.17.0: knowledge layer has 21 nodes (TermSet, Term, SEOKeywordMetrics removed)
    const knowledge = getNodeTypesByLayer('knowledge');
    expect(knowledge).toContain('ExpressionSet');
    expect(knowledge).toContain('Expression');
    expect(knowledge).toContain('LanguageFamily');
    expect(knowledge).toContain('SEOKeyword');
    expect(knowledge).toContain('SEOKeywordSet');
    expect(knowledge).toContain('GEOQuery');
    expect(knowledge).toContain('GEOQuerySet');
    expect(knowledge).toHaveLength(21);

    // v0.17.0: locale layer has 5 nodes (Market removed)
    const locale = getNodeTypesByLayer('locale');
    expect(locale).toContain('Style');
    expect(locale).toContain('Formatting');
    expect(locale).toContain('Adaptation');
    expect(locale).toHaveLength(5);

    // v0.12.4: geography layer has 7 nodes (Country added)
    const geography = getNodeTypesByLayer('geography');
    expect(geography).toContain('Continent');
    expect(geography).toContain('Country');
    expect(geography).toContain('GeoRegion');
    expect(geography).toHaveLength(7);

    // v0.19.1: instruction layer has 3 nodes (BlockRules removed, merged into BlockType.rules)
    const instruction = getNodeTypesByLayer('instruction');
    expect(instruction).toContain('BlockType');
    expect(instruction).toContain('BlockInstruction');
    expect(instruction).toContain('PromptArtifact');
    expect(instruction).toHaveLength(3);

    // v11.5: config layer has 4 nodes (3 shared + 1 org)
    const config = getNodeTypesByLayer('config');
    expect(config).toContain('OrgConfig');
    expect(config).toContain('EntityCategory');
    expect(config).toContain('Locale');
    expect(config).toContain('SEOKeywordFormat');
    expect(config).toHaveLength(4);
  });
});
