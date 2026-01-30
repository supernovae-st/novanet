// src/__tests__/filters.test.ts
import { describe, it, expect } from 'vitest';
import { NovaNetFilter } from '../filters/NovaNetFilter.js';
import { CypherGenerator } from '../filters/CypherGenerator.js';
import { ViewLoader } from '../filters/ViewLoader.js';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

describe('NovaNetFilter', () => {
  describe('create()', () => {
    it('returns a new NovaNetFilter instance', () => {
      const filter = NovaNetFilter.create();
      expect(filter).toBeInstanceOf(NovaNetFilter);
    });
  });

  describe('root selection', () => {
    it('fromPage() sets root to Page type', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing');
      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Page', key: 'page-pricing' });
    });

    it('fromBlock() sets root to Block type', () => {
      const filter = NovaNetFilter.create().fromBlock('block-hero');
      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Block', key: 'block-hero' });
    });

    it('fromConcept() sets root to Concept type', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro');
      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Concept', key: 'tier-pro' });
    });

    it('fromLocale() sets root to Locale type', () => {
      const filter = NovaNetFilter.create().fromLocale('fr-FR');
      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Locale', key: 'fr-FR' });
    });
  });

  describe('include methods', () => {
    it('includeBlocks() adds HAS_BLOCK include rule', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeBlocks();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_BLOCK', direction: 'outgoing' })
      );
    });

    it('includeConcepts() adds USES_CONCEPT include rule', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeConcepts();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'USES_CONCEPT', direction: 'outgoing' })
      );
    });

    it('includeConcepts({ spreading: true }) sets depth to 2', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeConcepts({ spreading: true });
      const criteria = filter.getCriteria();
      const conceptRule = criteria.includes.find(i => i.relation === 'USES_CONCEPT');
      expect(conceptRule?.depth).toBe(2);
    });

    it('includePrompts() adds HAS_PROMPT include rule', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includePrompts();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_PROMPT', direction: 'outgoing' })
      );
    });

    it('includePrompts({ activeOnly: true }) filters active prompts', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includePrompts({ activeOnly: true });
      const criteria = filter.getCriteria();
      const promptRule = criteria.includes.find(i => i.relation === 'HAS_PROMPT');
      expect(promptRule?.filters?.active).toBe(true);
    });

    it('includeBlockType() adds OF_TYPE include rule', () => {
      const filter = NovaNetFilter.create().fromBlock('block-hero').includeBlockType();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'OF_TYPE', direction: 'outgoing' })
      );
    });

    it('includeSemanticLinks() adds SEMANTIC_LINK include rule', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro').includeSemanticLinks();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'SEMANTIC_LINK', direction: 'outgoing' })
      );
    });

    it('includeSemanticLinks({ depth: 2 }) sets traversal depth', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro').includeSemanticLinks({ depth: 2 });
      const criteria = filter.getCriteria();
      const linkRule = criteria.includes.find(i => i.relation === 'SEMANTIC_LINK');
      expect(linkRule?.depth).toBe(2);
    });

    it('includePages() adds HAS_PAGE include rule', () => {
      const filter = NovaNetFilter.create().fromProject('qrcode-ai').includePages();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_PAGE', direction: 'outgoing' })
      );
    });

    it('includeBrandIdentity() adds HAS_BRAND_IDENTITY include rule', () => {
      const filter = NovaNetFilter.create().fromProject('qrcode-ai').includeBrandIdentity();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_BRAND_IDENTITY', direction: 'outgoing' })
      );
    });

    it('includeProjectConcepts() adds HAS_CONCEPT include rule', () => {
      const filter = NovaNetFilter.create().fromProject('qrcode-ai').includeProjectConcepts();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_CONCEPT', direction: 'outgoing' })
      );
    });

    it('includeProjectConcepts({ depth: 2 }) sets traversal depth', () => {
      const filter = NovaNetFilter.create().fromProject('qrcode-ai').includeProjectConcepts({ depth: 2 });
      const criteria = filter.getCriteria();
      const conceptRule = criteria.includes.find(i => i.relation === 'HAS_CONCEPT');
      expect(conceptRule?.depth).toBe(2);
    });

    it('includeRules() adds HAS_RULES include rule', () => {
      const filter = NovaNetFilter.create().fromBlock('block-hero').includeRules();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_RULES', direction: 'outgoing' })
      );
    });

    it('includeRules({ activeOnly: true }) filters active rules', () => {
      const filter = NovaNetFilter.create().fromBlock('block-hero').includeRules({ activeOnly: true });
      const criteria = filter.getCriteria();
      const rulesRule = criteria.includes.find(i => i.relation === 'HAS_RULES');
      expect(rulesRule?.filters?.active).toBe(true);
    });

    it('includeOutputs() adds HAS_OUTPUT include rule', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeOutputs();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_OUTPUT', direction: 'outgoing' })
      );
    });

    it('includeL10n() adds HAS_L10N include rule', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro').includeL10n();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_L10N', direction: 'outgoing' })
      );
    });

    it('includeSEO() adds TARGETS_SEO include rule', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro').includeSEO();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'TARGETS_SEO', direction: 'outgoing' })
      );
    });

    it('includeGEO() adds TARGETS_GEO include rule', () => {
      const filter = NovaNetFilter.create().fromConcept('tier-pro').includeGEO();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'TARGETS_GEO', direction: 'outgoing' })
      );
    });

    it('includeKnowledge() adds all knowledge relations', () => {
      const filter = NovaNetFilter.create().fromLocale('fr-FR').includeKnowledge();
      const criteria = filter.getCriteria();
      const knowledgeRelations = ['HAS_IDENTITY', 'HAS_VOICE', 'HAS_CULTURE', 'HAS_MARKET', 'HAS_LEXICON'];
      for (const relation of knowledgeRelations) {
        expect(criteria.includes).toContainEqual(
          expect.objectContaining({ relation, direction: 'outgoing' })
        );
      }
    });

    it('includeKnowledge() prevents duplicates when called multiple times', () => {
      const filter = NovaNetFilter.create()
        .fromLocale('fr-FR')
        .includeKnowledge()
        .includeKnowledge();
      const criteria = filter.getCriteria();
      const identityRules = criteria.includes.filter(i => i.relation === 'HAS_IDENTITY');
      expect(identityRules).toHaveLength(1);
    });
  });

  describe('filtering', () => {
    it('forLocale() sets locale filter', () => {
      const filter = NovaNetFilter.create().forLocale('fr-FR');
      const criteria = filter.getCriteria();
      expect(criteria.filters.locale).toBe('fr-FR');
    });

    it('withPriority() sets priority filter', () => {
      const filter = NovaNetFilter.create().withPriority('critical', 'high');
      const criteria = filter.getCriteria();
      expect(criteria.filters.priority).toEqual(['critical', 'high']);
    });

    it('withFreshness() sets freshness filter', () => {
      const filter = NovaNetFilter.create().withFreshness('static', 'daily');
      const criteria = filter.getCriteria();
      expect(criteria.filters.freshness).toEqual(['static', 'daily']);
    });

    it('byCategory() sets category filter', () => {
      const filter = NovaNetFilter.create().byCategory('project', 'content');
      const criteria = filter.getCriteria();
      expect(criteria.filters.categories).toEqual(['project', 'content']);
    });

    it('byTypes() sets nodeTypes filter', () => {
      const filter = NovaNetFilter.create().byTypes('Page', 'Block');
      const criteria = filter.getCriteria();
      expect(criteria.filters.nodeTypes).toEqual(['Page', 'Block']);
    });

    it('excludeTypes() sets excludeTypes filter', () => {
      const filter = NovaNetFilter.create().excludeTypes('SEOMiningRun', 'SEOKeywordMetrics');
      const criteria = filter.getCriteria();
      expect(criteria.filters.excludeTypes).toEqual(['SEOMiningRun', 'SEOKeywordMetrics']);
    });

    it('search() sets searchQuery filter', () => {
      const filter = NovaNetFilter.create().search('pricing');
      const criteria = filter.getCriteria();
      expect(criteria.filters.searchQuery).toBe('pricing');
    });

    it('search() with fields sets searchFields filter', () => {
      const filter = NovaNetFilter.create().search('pricing', ['key', 'display_name']);
      const criteria = filter.getCriteria();
      expect(criteria.filters.searchQuery).toBe('pricing');
      expect(criteria.filters.searchFields).toEqual(['key', 'display_name']);
    });

    it('maxDepth() sets depth limit', () => {
      const filter = NovaNetFilter.create().maxDepth(3);
      const criteria = filter.getCriteria();
      expect(criteria.filters.maxDepth).toBe(3);
    });
  });

  describe('chaining', () => {
    it('supports fluent chaining of multiple methods', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeConcepts({ spreading: true })
        .includePrompts({ activeOnly: true })
        .forLocale('fr-FR')
        .withPriority('critical', 'high')
        .maxDepth(2);

      const criteria = filter.getCriteria();

      expect(criteria.root).toEqual({ type: 'Page', key: 'page-pricing' });
      expect(criteria.includes).toHaveLength(3);
      expect(criteria.filters.locale).toBe('fr-FR');
      expect(criteria.filters.priority).toEqual(['critical', 'high']);
      expect(criteria.filters.maxDepth).toBe(2);
    });
  });
});

// =============================================================================
// CypherGenerator Tests (Task 3)
// =============================================================================

describe('CypherGenerator', () => {
  describe('basic queries', () => {
    it('generates MATCH for root node with $rootKey param', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('MATCH (root:Page {key: $rootKey})');
      expect(result.params.rootKey).toBe('page-pricing');
    });

    it('generates OPTIONAL MATCH for includes', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('OPTIONAL MATCH (root)-[:HAS_BLOCK]->(block:Block)');
    });

    it('generates WHERE for locale filter with $locale param', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .forLocale('fr-FR');
      const result = CypherGenerator.generate(filter);

      expect(result.params.locale).toBe('fr-FR');
    });

    it('generates WHERE for priority filter with $priorities param', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .withPriority('critical', 'high');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('root.priority IN $priorities');
      expect(result.params.priorities).toEqual(['critical', 'high']);
    });

    it('generates WHERE for freshness filter with $freshness param', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .withFreshness('static', 'daily');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('root.freshness IN $freshness');
      expect(result.params.freshness).toEqual(['static', 'daily']);
    });

    it('generates WHERE for byTypes filter', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .byTypes('Page', 'Block');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('(root:Page OR root:Block)');
    });

    it('generates WHERE for excludeTypes filter', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .excludeTypes('SEOMiningRun', 'SEOKeywordMetrics');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('(NOT root:SEOMiningRun AND NOT root:SEOKeywordMetrics)');
    });

    it('generates WHERE for search filter with default fields', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .search('pricing');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('toLower(root.key) CONTAINS toLower($searchQuery)');
      expect(result.query).toContain('toLower(root.display_name) CONTAINS toLower($searchQuery)');
      expect(result.query).toContain('toLower(root.description) CONTAINS toLower($searchQuery)');
      expect(result.params.searchQuery).toBe('pricing');
    });

    it('generates WHERE for search filter with custom fields', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .search('test', ['key', 'llm_context']);
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('toLower(root.key) CONTAINS toLower($searchQuery)');
      expect(result.query).toContain('toLower(root.llm_context) CONTAINS toLower($searchQuery)');
      expect(result.query).not.toContain('root.display_name');
    });

    it('generates WHERE for byCategory filter (expands categories)', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .byCategory('generation');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      // generation category includes prompts + output
      expect(result.query).toContain('root:PagePrompt');
      expect(result.query).toContain('root:PageL10n');
    });

    it('generates RETURN with collect(DISTINCT)', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('RETURN');
      expect(result.query).toContain('collect(DISTINCT block) AS blocks');
    });
  });

  describe('spreading activation', () => {
    it('generates SEMANTIC_LINK traversal for concepts with depth > 1', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeConcepts({ spreading: true });
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('OPTIONAL MATCH (root)-[:USES_CONCEPT]->(concept:Concept)');
      expect(result.query).toContain('OPTIONAL MATCH (concept)-[:SEMANTIC_LINK*1..1]->(relatedConcept:Concept)');
    });

    it('does not generate SEMANTIC_LINK for concepts with depth = 1', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeConcepts();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('OPTIONAL MATCH (root)-[:USES_CONCEPT]->(concept:Concept)');
      expect(result.query).not.toContain('SEMANTIC_LINK');
    });
  });

  describe('active filter', () => {
    it('generates {active: true} filter for prompts', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includePrompts({ activeOnly: true });
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('OPTIONAL MATCH (root)-[:HAS_PROMPT]->(prompt:PagePrompt {active: true})');
    });

    it('does not add {active: true} when activeOnly is false', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includePrompts();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('OPTIONAL MATCH (root)-[:HAS_PROMPT]->(prompt:PagePrompt)');
      expect(result.query).not.toContain('{active: true}');
    });
  });

  describe('direction handling', () => {
    it('generates outgoing arrow (->) for outgoing direction', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('(root)-[:HAS_BLOCK]->(block:Block)');
    });

    it('generates BlockType for OF_TYPE relation', () => {
      const filter = NovaNetFilter.create()
        .fromBlock('block-hero')
        .includeBlockType();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('(root)-[:OF_TYPE]->(blockType:BlockType)');
    });

    it('generates Project include relations', () => {
      const filter = NovaNetFilter.create()
        .fromProject('qrcode-ai')
        .includePages()
        .includeBrandIdentity()
        .includeProjectConcepts();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('[:HAS_PAGE]->(page:Page)');
      expect(result.query).toContain('[:HAS_BRAND_IDENTITY]->(brandIdentity:BrandIdentity)');
      expect(result.query).toContain('[:HAS_CONCEPT]->(projectConcept:Concept)');
    });
  });

  describe('complex queries', () => {
    it('generates full page-generation-context query', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeConcepts({ spreading: true })
        .includePrompts({ activeOnly: true })
        .forLocale('fr-FR')
        .withPriority('critical', 'high');

      const result = CypherGenerator.generate(filter);

      // Should have root match
      expect(result.query).toContain('MATCH (root:Page {key: $rootKey})');

      // Should have includes
      expect(result.query).toContain('OPTIONAL MATCH');
      expect(result.query).toContain('HAS_BLOCK');
      expect(result.query).toContain('USES_CONCEPT');
      expect(result.query).toContain('HAS_PROMPT');

      // Should have spreading activation
      expect(result.query).toContain('SEMANTIC_LINK');

      // Should have WHERE
      expect(result.query).toContain('WHERE');

      // Should have RETURN
      expect(result.query).toContain('RETURN');

      // Should have all params
      expect(result.params.rootKey).toBe('page-pricing');
      expect(result.params.locale).toBe('fr-FR');
      expect(result.params.priorities).toEqual(['critical', 'high']);
    });

    it('generates locale knowledge query', () => {
      const filter = NovaNetFilter.create()
        .fromLocale('fr-FR')
        .includeKnowledge();

      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('MATCH (root:Locale {key: $rootKey})');
      expect(result.query).toContain('HAS_IDENTITY');
      expect(result.query).toContain('HAS_VOICE');
      expect(result.query).toContain('HAS_CULTURE');
      expect(result.query).toContain('HAS_MARKET');
      expect(result.query).toContain('HAS_LEXICON');
      expect(result.params.rootKey).toBe('fr-FR');
    });
  });
});

// =============================================================================
// ViewLoader Tests (Task 4)
// =============================================================================

describe('ViewLoader', () => {
  const viewsDir = path.resolve(__dirname, '../../models/views');

  describe('loadView()', () => {
    it('loads a YAML view definition', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);

      expect(view.id).toBe('page-generation-context');
      expect(view.name).toBe('Page Generation Context');
      expect(view.root.type).toBe('Page');
    });

    it('parses include rules correctly', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);

      expect(view.include).toBeInstanceOf(Array);
      expect(view.include.length).toBeGreaterThan(0);
      expect(view.include[0]).toHaveProperty('relation');
      expect(view.include[0]).toHaveProperty('direction');
    });

    it('parses filters correctly', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);

      expect(view.filters).toBeDefined();
      expect(view.filters?.priority).toEqual(['critical', 'high', 'medium']);
      expect(view.filters?.locale).toBe('$locale');
    });

    it('throws error for non-existent view', async () => {
      await expect(ViewLoader.loadView('non-existent', viewsDir)).rejects.toThrow();
    });
  });

  describe('toFilter()', () => {
    it('converts ViewDefinition to NovaNetFilter', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });

      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Page', key: 'page-pricing' });
    });

    it('applies locale from params when view uses $locale placeholder', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });

      const criteria = filter.getCriteria();
      expect(criteria.filters.locale).toBe('fr-FR');
    });

    it('applies priority filter from view definition', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing' });

      const criteria = filter.getCriteria();
      expect(criteria.filters.priority).toEqual(['critical', 'high', 'medium']);
    });

    it('applies include rules from view definition', async () => {
      const view = await ViewLoader.loadView('page-generation-context', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing' });

      const criteria = filter.getCriteria();
      expect(criteria.includes.length).toBeGreaterThan(0);
    });
  });

  describe('loadRegistry()', () => {
    it('loads the view registry', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      expect(registry.version).toBeDefined();
      expect(registry.views).toBeInstanceOf(Array);
      expect(registry.views.length).toBeGreaterThan(0);
    });

    it('registry entries have required fields', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      for (const entry of registry.views) {
        expect(entry.id).toBeDefined();
        expect(entry.file).toBeDefined();
        expect(entry.description).toBeDefined();
      }
    });

    it('all registered views can be loaded', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      for (const entry of registry.views) {
        const view = await ViewLoader.loadView(entry.id, viewsDir);
        expect(view.id).toBe(entry.id);
      }
    });
  });
});
