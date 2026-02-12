// src/__tests__/filters.test.ts
import { describe, it, expect } from 'vitest';
import { NovaNetFilter } from '../filters/NovaNetFilter.js';
import { CypherGenerator } from '../filters/CypherGenerator.js';
import { ViewLoader } from '../filters/ViewLoader.js';

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

    it('fromEntity() sets root to Entity type (v10.3)', () => {
      const filter = NovaNetFilter.create().fromEntity('tier-pro');
      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Entity', key: 'tier-pro' });
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

    it('includeEntities() adds USES_ENTITY include rule (v10.3)', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeEntities();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'USES_ENTITY', direction: 'outgoing' })
      );
    });

    it('includeEntities({ spreading: true }) sets depth to 2 (v10.3)', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeEntities({ spreading: true });
      const criteria = filter.getCriteria();
      const entityRule = criteria.includes.find(i => i.relation === 'USES_ENTITY');
      expect(entityRule?.depth).toBe(2);
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

    it('includeSemanticLinks() adds SEMANTIC_LINK include rule (v10.3: from Entity)', () => {
      const filter = NovaNetFilter.create().fromEntity('tier-pro').includeSemanticLinks();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'SEMANTIC_LINK', direction: 'outgoing' })
      );
    });

    it('includeSemanticLinks({ depth: 2 }) sets traversal depth', () => {
      const filter = NovaNetFilter.create().fromEntity('tier-pro').includeSemanticLinks({ depth: 2 });
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

    // REMOVED v10.3: includeProjectConcepts tests (HAS_CONCEPT arc removed)
    // Entity is now in org realm, accessed via USES_ENTITY from Page/Block

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

    it('includeOutputs() adds HAS_GENERATED include rule', () => {
      const filter = NovaNetFilter.create().fromPage('page-pricing').includeOutputs();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_GENERATED', direction: 'outgoing' })
      );
    });

    it('includeContent() adds HAS_CONTENT include rule (v11.6: renamed from includeL10n)', () => {
      const filter = NovaNetFilter.create().fromEntity('tier-pro').includeContent();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'HAS_CONTENT', direction: 'outgoing' })
      );
    });

    it('includeSEO() adds EXPRESSES include rule (v10.3: TARGETS_SEO → EXPRESSES)', () => {
      const filter = NovaNetFilter.create().fromEntity('tier-pro').includeSEO();
      const criteria = filter.getCriteria();
      expect(criteria.includes).toContainEqual(
        expect.objectContaining({ relation: 'EXPRESSES', direction: 'outgoing' })
      );
    });

    // REMOVED v10.3: includeGEO test (GEO layer removed)

    it('includeKnowledge() adds all knowledge relations', () => {
      const filter = NovaNetFilter.create().fromLocale('fr-FR').includeKnowledge();
      const criteria = filter.getCriteria();
      // v11.5: Culture, Market, Formatting, Slugification, ExpressionSet
      const knowledgeRelations = ['HAS_CULTURE', 'HAS_MARKET', 'HAS_FORMATTING', 'HAS_SLUGIFICATION', 'HAS_EXPRESSIONS'];
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
      // v11.5: Check HAS_CULTURE instead of HAS_IDENTITY
      const cultureRules = criteria.includes.filter(i => i.relation === 'HAS_CULTURE');
      expect(cultureRules).toHaveLength(1);
    });
  });

  describe('filtering', () => {
    it('forLocale() sets locale filter', () => {
      const filter = NovaNetFilter.create().forLocale('fr-FR');
      const criteria = filter.getCriteria();
      expect(criteria.filters.locale).toBe('fr-FR');
    });

    // REMOVED v8.2.0: withPriority and withFreshness tests (YAML v7.11.0 alignment)
    // it('withPriority() sets priority filter', () => { ... });
    // it('withFreshness() sets freshness filter', () => { ... });

    // REMOVED v9.0.0: byCategory (NodeCategory killed)

    it('byTypes() sets nodeTypes filter', () => {
      const filter = NovaNetFilter.create().byTypes('Page', 'Block');
      const criteria = filter.getCriteria();
      expect(criteria.filters.nodeTypes).toEqual(['Page', 'Block']);
    });

    it('excludeTypes() sets excludeTypes filter', () => {
      const filter = NovaNetFilter.create().excludeTypes('SEOKeyword', 'SEOKeywordMetrics');
      const criteria = filter.getCriteria();
      expect(criteria.filters.excludeTypes).toEqual(['SEOKeyword', 'SEOKeywordMetrics']);
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
    it('supports fluent chaining of multiple methods (v10.3: Entity)', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeEntities({ spreading: true })
        .includePrompts({ activeOnly: true })
        .forLocale('fr-FR')
        .maxDepth(2);

      const criteria = filter.getCriteria();

      expect(criteria.root).toEqual({ type: 'Page', key: 'page-pricing' });
      expect(criteria.includes).toHaveLength(3);
      expect(criteria.filters.locale).toBe('fr-FR');
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

    it('generates OPTIONAL MATCH for includes with relationship variable', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified, relationship variable captured
      expect(result.query).toMatch(/OPTIONAL MATCH \(root\)-\[r\d+:HAS_BLOCK\]->\(block\)/);
    });

    it('generates WHERE for locale filter with $locale param', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .forLocale('fr-FR');
      const result = CypherGenerator.generate(filter);

      expect(result.params.locale).toBe('fr-FR');
    });

    // REMOVED v8.2.0: priority and freshness WHERE clause tests (YAML v7.11.0 alignment)
    // it('generates WHERE for priority filter with $priorities param', () => { ... });
    // it('generates WHERE for freshness filter with $freshness param', () => { ... });

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
        .excludeTypes('SEOKeyword', 'SEOKeywordMetrics');
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('WHERE');
      expect(result.query).toContain('(NOT root:SEOKeyword AND NOT root:SEOKeywordMetrics)');
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

    // REMOVED v9.0.0: byCategory CypherGenerator test (NodeCategory killed)

    it('generates RETURN with collect(DISTINCT)', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('RETURN');
      expect(result.query).toContain('collect(DISTINCT block) AS blocks');
    });
  });

  describe('spreading activation (v10.3: Entity)', () => {
    it('generates SEMANTIC_LINK traversal for entities with depth > 1', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeEntities({ spreading: true });
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified, alias is now 'usedEntity'
      expect(result.query).toMatch(/OPTIONAL MATCH \(root\)-\[r\d+:USES_ENTITY\]->\(usedEntity\)/);
      expect(result.query).toMatch(/OPTIONAL MATCH \(usedEntity\)-\[r\d+:SEMANTIC_LINK\*1\.\.1\]->\(relatedUsedEntity:Entity\)/);
    });

    it('does not generate SEMANTIC_LINK for entities with depth = 1', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeEntities();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified, alias is now 'usedEntity'
      expect(result.query).toMatch(/OPTIONAL MATCH \(root\)-\[r\d+:USES_ENTITY\]->\(usedEntity\)/);
      expect(result.query).not.toContain('SEMANTIC_LINK');
    });
  });

  describe('active filter', () => {
    it('generates {active: true} filter for prompts', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includePrompts({ activeOnly: true });
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified, only property filter
      expect(result.query).toMatch(/OPTIONAL MATCH \(root\)-\[r\d+:HAS_PROMPT\]->\(prompt \{active: true\}\)/);
    });

    it('does not add {active: true} when activeOnly is false', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includePrompts();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified
      expect(result.query).toMatch(/OPTIONAL MATCH \(root\)-\[r\d+:HAS_PROMPT\]->\(prompt\)/);
      expect(result.query).not.toContain('{active: true}');
    });
  });

  describe('direction handling', () => {
    it('generates outgoing arrow (->) for outgoing direction', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified
      expect(result.query).toMatch(/\(root\)-\[r\d+:HAS_BLOCK\]->\(block\)/);
    });

    it('generates OF_TYPE relation for blocks', () => {
      const filter = NovaNetFilter.create()
        .fromBlock('block-hero')
        .includeBlockType();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified
      expect(result.query).toMatch(/\(root\)-\[r\d+:OF_TYPE\]->\(blockType\)/);
    });

    it('generates Project include relations (v10.3: no HAS_CONCEPT)', () => {
      const filter = NovaNetFilter.create()
        .fromProject('qrcode-ai')
        .includePages()
        .includeBrandIdentity();
      const result = CypherGenerator.generate(filter);

      // v11.6: Target type not specified, relationship variables still captured
      expect(result.query).toMatch(/\[r\d+:HAS_PAGE\]->\(page\)/);
      expect(result.query).toMatch(/\[r\d+:HAS_BRAND_IDENTITY\]->\(brandIdentity\)/);
      // REMOVED v10.3: HAS_CONCEPT (Entity is in org realm)
    });
  });

  describe('complex queries', () => {
    it('generates full page-generation-context query (v10.3: Entity)', () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeEntities({ spreading: true })
        .includePrompts({ activeOnly: true })
        .forLocale('fr-FR');

      const result = CypherGenerator.generate(filter);

      // Should have root match
      expect(result.query).toContain('MATCH (root:Page {key: $rootKey})');

      // Should have includes (v10.3: USES_ENTITY replaces USES_CONCEPT)
      expect(result.query).toContain('OPTIONAL MATCH');
      expect(result.query).toContain('HAS_BLOCK');
      expect(result.query).toContain('USES_ENTITY');
      expect(result.query).toContain('HAS_PROMPT');

      // Should have spreading activation
      expect(result.query).toContain('SEMANTIC_LINK');

      // Should have RETURN
      expect(result.query).toContain('RETURN');

      // Should have all params
      expect(result.params.rootKey).toBe('page-pricing');
      expect(result.params.locale).toBe('fr-FR');
    });

    it('generates locale knowledge query', () => {
      const filter = NovaNetFilter.create()
        .fromLocale('fr-FR')
        .includeKnowledge();

      const result = CypherGenerator.generate(filter);

      expect(result.query).toContain('MATCH (root:Locale {key: $rootKey})');
      // v11.5: Culture, Market, Formatting, Slugification, ExpressionSet
      expect(result.query).toContain('HAS_CULTURE');
      expect(result.query).toContain('HAS_MARKET');
      expect(result.query).toContain('HAS_FORMATTING');
      expect(result.query).toContain('HAS_SLUGIFICATION');
      expect(result.query).toContain('HAS_EXPRESSIONS');
      expect(result.params.rootKey).toBe('fr-FR');
    });
  });
});

// =============================================================================
// ViewLoader Tests (Task 4)
// =============================================================================

describe('ViewLoader', () => {
  describe('loadView()', () => {
    it('loads a view definition from generated constants', async () => {
      const view = await ViewLoader.loadView('page-generation-context');

      expect(view.id).toBe('page-generation-context');
      expect(view.name).toBe('Page Generation Context');
      expect(view.root.type).toBe('Page');
    });

    it('parses include rules correctly', async () => {
      const view = await ViewLoader.loadView('page-generation-context');

      expect(view.include).toBeInstanceOf(Array);
      expect(view.include.length).toBeGreaterThan(0);
      expect(view.include[0]).toHaveProperty('relation');
      expect(view.include[0]).toHaveProperty('direction');
    });

    it('parses filters correctly', async () => {
      const view = await ViewLoader.loadView('page-generation-context');

      expect(view.filters).toBeDefined();
      expect(view.filters?.locale).toBe('$locale');
    });

    it('throws error for non-existent view', async () => {
      await expect(ViewLoader.loadView('non-existent')).rejects.toThrow();
    });
  });

  describe('toFilter()', () => {
    it('converts ViewDefinition to NovaNetFilter', async () => {
      const view = await ViewLoader.loadView('page-generation-context');
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });

      const criteria = filter.getCriteria();
      expect(criteria.root).toEqual({ type: 'Page', key: 'page-pricing' });
    });

    it('applies locale from params when view uses $locale placeholder', async () => {
      const view = await ViewLoader.loadView('page-generation-context');
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });

      const criteria = filter.getCriteria();
      expect(criteria.filters.locale).toBe('fr-FR');
    });

    it('applies include rules from view definition', async () => {
      const view = await ViewLoader.loadView('page-generation-context');
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing' });

      const criteria = filter.getCriteria();
      expect(criteria.includes.length).toBeGreaterThan(0);
    });
  });

  describe('loadRegistry()', () => {
    it('loads the view registry', async () => {
      const registry = await ViewLoader.loadRegistry();

      expect(registry.version).toBeDefined();
      expect(registry.views).toBeInstanceOf(Array);
      expect(registry.views.length).toBeGreaterThan(0);
    });

    it('registry entries have required fields', async () => {
      const registry = await ViewLoader.loadRegistry();

      for (const entry of registry.views) {
        expect(entry.id).toBeDefined();
        expect(entry.description).toBeDefined();
        expect(entry.category).toBeDefined();
        // v11.6.1: All views have embedded Cypher queries
        expect(entry.cypher).toBeDefined();
      }
    });

    it('all views have valid Cypher queries', async () => {
      const registry = await ViewLoader.loadRegistry();

      for (const entry of registry.views) {
        // Each view should have a non-empty Cypher query
        expect(entry.cypher).toBeDefined();
        expect(entry.cypher!.trim().length).toBeGreaterThan(0);
        // Cypher should contain MATCH or RETURN
        expect(entry.cypher).toMatch(/MATCH|RETURN/i);
      }
    });

    it('getCypher returns query with params', async () => {
      const cypherResult = await ViewLoader.getCypher('data-complete');
      expect(cypherResult.query).toBeDefined();
      expect(cypherResult.query.length).toBeGreaterThan(0);
      expect(cypherResult.params).toBeDefined();
    });
  });
});
