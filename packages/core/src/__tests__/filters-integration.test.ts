// src/__tests__/filters-integration.test.ts
// Integration tests for the filter system with Neo4j
// These tests execute actual Cypher queries against a running Neo4j instance

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { NovaNetFilter } from '../filters/NovaNetFilter.js';
import { CypherGenerator } from '../filters/CypherGenerator.js';
import { ViewLoader } from '../filters/ViewLoader.js';
import { getDriver, closeDriver, isNeo4jAvailable } from '../db/client.js';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// =============================================================================
// INTEGRATION TEST SUITE
// =============================================================================

describe('Filter System Integration Tests', () => {
  const viewsDir = path.resolve(__dirname, '../../models/views');
  let neo4jAvailable = false;

  beforeAll(async () => {
    // Check if Neo4j is available
    neo4jAvailable = await isNeo4jAvailable();
    if (!neo4jAvailable) {
      console.warn('\n[WARN] Neo4j not available - integration tests will be skipped');
      console.warn('       Start Neo4j with: npm run infra:up (from monorepo root)\n');
    }
  });

  afterAll(async () => {
    // Close driver connection if it was opened
    if (neo4jAvailable) {
      await closeDriver();
    }
  });

  // ===========================================================================
  // NovaNetFilter -> CypherGenerator -> Neo4j Execution
  // ===========================================================================

  describe('NovaNetFilter -> CypherGenerator -> Neo4j', () => {
    it('executes a simple page query', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const filter = NovaNetFilter.create()
        .fromPage('page-pricing');

      const { query, params } = CypherGenerator.generate(filter);

      // Verify query structure
      expect(query).toContain('MATCH (root:Page {key: $rootKey})');
      expect(params.rootKey).toBe('page-pricing');

      // Execute against Neo4j
      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        // Query should execute without errors (node may or may not exist)
        expect(result.records).toBeDefined();
        expect(Array.isArray(result.records)).toBe(true);
      } finally {
        await session.close();
      }
    });

    it('executes page-generation-context query pattern', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeConcepts({ spreading: true })
        .includePrompts({ activeOnly: true })
        .forLocale('fr-FR');

      const { query, params } = CypherGenerator.generate(filter);

      // Verify all expected clauses are present
      expect(query).toContain('MATCH (root:Page {key: $rootKey})');
      expect(query).toContain('OPTIONAL MATCH');
      expect(query).toContain('HAS_BLOCK');
      expect(query).toContain('USES_CONCEPT');
      expect(query).toContain('HAS_PROMPT');
      expect(query).toContain('RETURN');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
        // Verify the result structure
        if (result.records.length > 0) {
          const record = result.records[0];
          expect(record.has('root')).toBe(true);
        }
      } finally {
        await session.close();
      }
    });

    it('executes locale-full-knowledge query pattern', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const filter = NovaNetFilter.create()
        .fromLocale('fr-FR')
        .includeKnowledge();

      const { query, params } = CypherGenerator.generate(filter);

      // Verify query includes all knowledge relations
      expect(query).toContain('MATCH (root:Locale {key: $rootKey})');
      expect(query).toContain('HAS_IDENTITY');
      expect(query).toContain('HAS_VOICE');
      expect(query).toContain('HAS_CULTURE');
      expect(query).toContain('HAS_MARKET');
      expect(query).toContain('HAS_LEXICON');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('executes concept with SEO/GEO query pattern', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const filter = NovaNetFilter.create()
        .fromConcept('tier-pro')
        .includeL10n()
        .includeSEO()
        .includeGEO()
        .forLocale('en-US');

      const { query, params } = CypherGenerator.generate(filter);

      expect(query).toContain('MATCH (root:Concept {key: $rootKey})');
      expect(query).toContain('HAS_L10N');
      expect(query).toContain('TARGETS_SEO');
      expect(query).toContain('TARGETS_GEO');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('executes project overview query pattern', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      // v8.2.0: Removed withPriority (YAML v7.11.0 alignment)
      const filter = NovaNetFilter.create()
        .fromProject('qrcode-ai')
        .includePages();

      const { query, params } = CypherGenerator.generate(filter);

      expect(query).toContain('MATCH (root:Project {key: $rootKey})');
      expect(query).toContain('HAS_PAGE');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('executes block semantic network query pattern', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const filter = NovaNetFilter.create()
        .fromBlock('hero-pricing')
        .includeConcepts({ spreading: true })
        .includePrompts({ activeOnly: true });

      const { query, params } = CypherGenerator.generate(filter);

      expect(query).toContain('MATCH (root:Block {key: $rootKey})');
      expect(query).toContain('USES_CONCEPT');
      expect(query).toContain('SEMANTIC_LINK');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });
  });

  // ===========================================================================
  // ViewLoader -> NovaNetFilter -> CypherGenerator -> Neo4j
  // ===========================================================================

  describe('ViewLoader -> NovaNetFilter -> CypherGenerator -> Neo4j', () => {
    it('loads and executes page-generation-context view', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const view = await ViewLoader.loadView('page-generation-context', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });
      const { query, params } = CypherGenerator.generate(filter);

      // Verify the view was converted correctly
      expect(view.id).toBe('page-generation-context');
      expect(params.rootKey).toBe('page-pricing');
      expect(params.locale).toBe('fr-FR');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('loads and executes block-semantic-network view', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const view = await ViewLoader.loadView('block-semantic-network', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'hero-pricing', locale: 'fr-FR' });
      const { query, params } = CypherGenerator.generate(filter);

      expect(view.id).toBe('block-semantic-network');
      expect(params.rootKey).toBe('hero-pricing');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('loads and executes locale-full-knowledge view', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const view = await ViewLoader.loadView('locale-full-knowledge', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'fr-FR' });
      const { query, params } = CypherGenerator.generate(filter);

      expect(view.id).toBe('locale-full-knowledge');
      expect(params.rootKey).toBe('fr-FR');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('loads and executes concept-ecosystem view', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const view = await ViewLoader.loadView('concept-ecosystem', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'tier-pro', locale: 'en-US' });
      const { query, params } = CypherGenerator.generate(filter);

      expect(view.id).toBe('concept-ecosystem');
      expect(params.rootKey).toBe('tier-pro');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('loads and executes project-overview view', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const view = await ViewLoader.loadView('project-overview', viewsDir);
      const filter = ViewLoader.toFilter(view, { key: 'qrcode-ai' });
      const { query, params } = CypherGenerator.generate(filter);

      expect(view.id).toBe('project-overview');
      expect(params.rootKey).toBe('qrcode-ai');

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });
  });

  // ===========================================================================
  // Registry Verification
  // ===========================================================================

  describe('View Registry Verification', () => {
    it('all 13 registered views exist and are valid', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      expect(registry.views.length).toBe(13);
      expect(registry.views.map(v => v.id).sort()).toEqual([
        'block-generation',
        'block-semantic-network',
        'complete-graph',
        'concept-ecosystem',
        'geo-pipeline',
        'global-layer',
        'locale-full-knowledge',
        'page-generation-context',
        'project-context',
        'project-layer',
        'project-overview',
        'seo-pipeline',
        'shared-layer',
      ]);
    });

    it('all registered views can be loaded and converted to filters', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      for (const viewInfo of registry.views) {
        const view = await ViewLoader.loadView(viewInfo.id, viewsDir);

        // Verify view structure
        expect(view.id).toBe(viewInfo.id);
        expect(view.name).toBeDefined();
        expect(view.description).toBeDefined();
        expect(view.version).toBeDefined();
        expect(view.root).toBeDefined();
        expect(view.root.type).toBeDefined();
        expect(view.include).toBeInstanceOf(Array);

        // Convert to filter
        const filter = ViewLoader.toFilter(view, { key: 'test-key', locale: 'en-US' });
        const criteria = filter.getCriteria();

        // Verify filter was created
        expect(criteria.root).toBeDefined();
        expect(criteria.root?.key).toBe('test-key');
      }
    });

    it('all registered views generate valid Cypher queries', async () => {
      const registry = await ViewLoader.loadRegistry(viewsDir);

      for (const viewInfo of registry.views) {
        const view = await ViewLoader.loadView(viewInfo.id, viewsDir);
        const filter = ViewLoader.toFilter(view, { key: 'test-key', locale: 'en-US' });
        const { query, params } = CypherGenerator.generate(filter);

        // Verify query structure
        expect(query).toContain('MATCH (root:');
        expect(query).toContain('RETURN');
        expect(params.rootKey).toBe('test-key');
      }
    });

    it('all registered views execute against Neo4j without syntax errors', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      const registry = await ViewLoader.loadRegistry(viewsDir);
      const driver = getDriver();

      for (const viewInfo of registry.views) {
        const view = await ViewLoader.loadView(viewInfo.id, viewsDir);
        const filter = ViewLoader.toFilter(view, { key: 'test-key', locale: 'en-US' });
        const { query, params } = CypherGenerator.generate(filter);

        const session = driver.session();
        try {
          // This should not throw - we're verifying syntax, not data
          const result = await session.run(query, params);
          expect(result.records).toBeDefined();
        } catch (error) {
          // If there's an error, include which view failed for debugging
          throw new Error(`View '${viewInfo.id}' failed: ${error}`);
        } finally {
          await session.close();
        }
      }
    });
  });

  // ===========================================================================
  // Edge Cases and Error Handling
  // ===========================================================================

  describe('Edge Cases and Error Handling', () => {
    it('handles empty result sets gracefully', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      // Query for a non-existent key
      const filter = NovaNetFilter.create()
        .fromPage('non-existent-page-12345')
        .includeBlocks();

      const { query, params } = CypherGenerator.generate(filter);

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        // Should return empty results, not throw
        expect(result.records).toBeDefined();
        expect(result.records.length).toBe(0);
      } finally {
        await session.close();
      }
    });

    it('handles multiple OPTIONAL MATCH clauses correctly', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      // Build a complex filter with many includes
      const filter = NovaNetFilter.create()
        .fromLocale('en-US')
        .includeKnowledge(); // Adds 5 OPTIONAL MATCH clauses

      const { query, params } = CypherGenerator.generate(filter);

      // Verify multiple OPTIONAL MATCH clauses
      const optionalMatches = query.match(/OPTIONAL MATCH/g) ?? [];
      expect(optionalMatches.length).toBeGreaterThanOrEqual(5);

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('handles special characters in keys', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      // Test with special characters that could cause issues
      const filter = NovaNetFilter.create()
        .fromConcept('test-key-with-dashes');

      const { query, params } = CypherGenerator.generate(filter);

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        // Should execute without errors (parameterized queries protect against injection)
        expect(result.records).toBeDefined();
      } finally {
        await session.close();
      }
    });

    it('verifies parameterized queries prevent injection', async () => {
      if (!neo4jAvailable) {
        console.log('    [SKIPPED] Neo4j not available');
        return;
      }

      // Attempt a potential injection string
      const maliciousKey = "'; DROP DATABASE; --";
      const filter = NovaNetFilter.create()
        .fromPage(maliciousKey);

      const { query, params } = CypherGenerator.generate(filter);

      // The key should be passed as a parameter, not interpolated
      expect(query).toContain('$rootKey');
      expect(query).not.toContain(maliciousKey);
      expect(params.rootKey).toBe(maliciousKey);

      const driver = getDriver();
      const session = driver.session();

      try {
        const result = await session.run(query, params);
        // Should execute safely (just return no results)
        expect(result.records).toBeDefined();
        expect(result.records.length).toBe(0);
      } finally {
        await session.close();
      }
    });
  });

  // ===========================================================================
  // Performance Characteristics
  // ===========================================================================

  describe('Query Performance Characteristics', () => {
    it('generated queries use efficient patterns', async () => {
      // Verify queries use OPTIONAL MATCH (not MATCH) for includes
      // This ensures queries don't fail when related nodes don't exist
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeBlocks()
        .includeConcepts();

      const { query } = CypherGenerator.generate(filter);

      // Root should use MATCH (required)
      expect(query).toMatch(/^MATCH \(root:/);

      // Includes should use OPTIONAL MATCH
      expect(query).toContain('OPTIONAL MATCH');

      // Should use collect(DISTINCT) for aggregation
      expect(query).toContain('collect(DISTINCT');
    });

    it('spreading activation limits depth appropriately', async () => {
      const filter = NovaNetFilter.create()
        .fromPage('page-pricing')
        .includeConcepts({ spreading: true });

      const { query } = CypherGenerator.generate(filter);

      // Spreading should be limited to prevent expensive traversals
      // Default spreading depth is 2, so SEMANTIC_LINK should be *1..1
      expect(query).toContain('SEMANTIC_LINK*1..1');
    });
  });
});
