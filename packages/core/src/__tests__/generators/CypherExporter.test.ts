// src/__tests__/generators/CypherExporter.test.ts
import { describe, it, expect } from 'vitest';
import { CypherExporter } from '../../generators/CypherExporter.js';
import type { ExtendedViewDefinition } from '../../generators/types.js';

const testView: ExtendedViewDefinition = {
  id: 'test-view',
  name: 'Test View',
  description: 'A test view',
  version: '1.0',
  root: { type: 'Page' },
  include: [
    { relation: 'HAS_BLOCK', direction: 'outgoing' },
  ],
  docs: {
    title: 'Test',
    category: 'generation',
    description: 'Test',
    layers: [{ name: 'Test', nodes: ['Page'] }],
    examples: [
      {
        name: 'Load page with blocks',
        description: 'Get a page and all its blocks',
        query: `MATCH (p:Page {key: $key})
MATCH (p)-[:HAS_BLOCK]->(b:Block)
RETURN p, collect(b) AS blocks`,
        params: { key: 'page-pricing' },
      },
      {
        name: 'Count blocks',
        query: 'MATCH (p:Page)-[:HAS_BLOCK]->(b:Block) RETURN count(b)',
      },
    ],
  },
};

describe('CypherExporter', () => {
  describe('extract', () => {
    it('should extract all queries from view', () => {
      const result = CypherExporter.extract(testView);

      expect(result.viewId).toBe('test-view');
      expect(result.queries).toHaveLength(2);
      expect(result.queries[0].name).toBe('Load page with blocks');
      expect(result.queries[1].name).toBe('Count blocks');
    });

    it('should preserve query content', () => {
      const result = CypherExporter.extract(testView);

      expect(result.queries[0].query).toContain('MATCH (p:Page {key: $key})');
      expect(result.queries[0].params).toEqual({ key: 'page-pricing' });
    });

    it('should return empty queries for view without examples', () => {
      const viewNoExamples: ExtendedViewDefinition = {
        id: 'no-examples',
        name: 'No Examples',
        description: 'Test',
        version: '1.0',
        root: { type: 'Page' },
        include: [],
        docs: {
          title: 'Test',
          category: 'overview',
          description: 'Test',
          layers: [{ name: 'Test', nodes: ['Page'] }],
        },
      };

      const result = CypherExporter.extract(viewNoExamples);
      expect(result.queries).toHaveLength(0);
    });
  });

  describe('toCypherFile', () => {
    it('should generate combined Cypher file', () => {
      const result = CypherExporter.toCypherFile(testView);

      expect(result).toContain('// View: test-view');
      expect(result).toContain('// Query: Load page with blocks');
      expect(result).toContain('// Query: Count blocks');
      expect(result).toContain('MATCH (p:Page {key: $key})');
    });

    it('should include parameter comments', () => {
      const result = CypherExporter.toCypherFile(testView, { includeParams: true });

      expect(result).toContain(':param key');
    });

    it('should separate queries with comments', () => {
      const result = CypherExporter.toCypherFile(testView);

      // Should have separators between queries
      expect(result.split('// Query:').length).toBe(3); // header + 2 queries
    });
  });

  describe('toNeoBrowserFormat', () => {
    it('should generate Neo4j Browser compatible format', () => {
      const result = CypherExporter.toNeoBrowserFormat(testView);

      // Neo4j Browser uses :play or specific formatting
      expect(result).toContain('test-view');
      expect(result).toContain('MATCH');
    });
  });

  describe('extractAll', () => {
    it('should extract from multiple views', () => {
      const views = [testView, { ...testView, id: 'test-view-2' }];
      const results = CypherExporter.extractAll(views);

      expect(results).toHaveLength(2);
      expect(results[0].viewId).toBe('test-view');
      expect(results[1].viewId).toBe('test-view-2');
    });
  });
});
