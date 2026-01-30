// src/__tests__/integration/docs-generation.test.ts
// Integration tests for the full YAML → MD documentation pipeline
import { describe, it, expect, beforeAll } from 'vitest';
import { ViewParser } from '../../generators/ViewParser.js';
import { MarkdownGenerator } from '../../generators/MarkdownGenerator.js';
import { CypherExporter } from '../../generators/CypherExporter.js';
import type { ExtendedViewDefinition } from '../../generators/types.js';
import * as path from 'path';

const VIEWS_DIR = path.join(process.cwd(), 'models/views');
const TEST_VIEW_YAML = `
id: integration-test-view
name: Integration Test View
description: A view for testing the full pipeline
version: "1.0"

root:
  type: Page

include:
  - relation: HAS_BLOCK
    direction: outgoing
  - relation: HAS_PROMPT
    direction: outgoing

docs:
  title: Integration Test Documentation
  category: generation
  description: |
    This view tests the complete documentation generation pipeline
    from YAML definition to Markdown output.
  layers:
    - name: Page Layer
      nodes: [Page, PagePrompt]
      color: blue
    - name: Block Layer
      nodes: [Block, BlockType]
      color: green
  examples:
    - name: Load page context
      description: Load a page with all its related content
      query: |
        MATCH (p:Page {key: $pageKey})
        MATCH (p)-[:HAS_BLOCK]->(b:Block)
        MATCH (p)-[:HAS_PROMPT]->(pp:PagePrompt)
        RETURN p, collect(b) AS blocks, pp
      params:
        pageKey: "page-pricing"
  notes:
    - This is an integration test view
    - It should not be committed to the repository
`;

describe('Documentation Generation Pipeline', () => {
  let testView: ExtendedViewDefinition;

  beforeAll(() => {
    testView = ViewParser.parseString(TEST_VIEW_YAML);
  });

  describe('Full Pipeline: YAML → View → MD', () => {
    it('should parse YAML and generate valid Markdown', () => {
      // Step 1: Parse YAML
      const view = ViewParser.parseString(TEST_VIEW_YAML);
      expect(view.id).toBe('integration-test-view');
      expect(ViewParser.hasDocs(view)).toBe(true);

      // Step 2: Generate Markdown
      const result = MarkdownGenerator.generate(view);

      // Step 3: Verify structure
      expect(result.content).toContain('# Integration Test Documentation');
      expect(result.content).toContain('## Overview');
      expect(result.content).toContain('## Graph Diagram');
      expect(result.content).toContain('```mermaid');
      expect(result.content).toContain('## Nodes');
      expect(result.content).toContain('## Relations');
      expect(result.content).toContain('## Cypher Queries');
      expect(result.content).toContain('## Notes');
    });

    it('should generate valid Mermaid syntax', () => {
      const view = ViewParser.parseString(TEST_VIEW_YAML);
      const mermaid = MarkdownGenerator.generateMermaid(view);

      // Valid Mermaid should have these elements
      expect(mermaid).toContain('flowchart TB');
      expect(mermaid).toContain('classDef');
      expect(mermaid).toContain('subgraph');
      expect(mermaid).toContain('Page_Layer');
      expect(mermaid).toContain('Block_Layer');
    });

    it('should extract Cypher queries correctly', () => {
      const view = ViewParser.parseString(TEST_VIEW_YAML);
      const cypher = CypherExporter.extract(view);

      expect(cypher.queries).toHaveLength(1);
      expect(cypher.queries[0].name).toBe('Load page context');
      expect(cypher.queries[0].query).toContain('MATCH (p:Page {key: $pageKey})');
    });
  });

  describe('Real Views Integration', () => {
    it('should load existing views from models/views', async () => {
      const views = await ViewParser.loadAllViews(VIEWS_DIR);

      expect(views.length).toBeGreaterThan(0);
      views.forEach(view => {
        expect(view.id).toBeDefined();
        expect(view.name).toBeDefined();
        expect(view.root.type).toBeDefined();
      });
    });

    it('should generate MD for views with docs section', async () => {
      const views = await ViewParser.loadAllViews(VIEWS_DIR);
      const viewsWithDocs = views.filter(ViewParser.hasDocs);

      // If any views have docs, they should generate valid MD
      for (const view of viewsWithDocs) {
        const result = MarkdownGenerator.generate(view);
        expect(result.content).toContain(`# ${view.docs!.title}`);
        expect(result.viewId).toBe(view.id);
      }
    });
  });

  describe('Error Handling', () => {
    it('should throw for invalid YAML', () => {
      const invalidYaml = `
id: 123-starts-with-number
name: Invalid
`;
      expect(() => ViewParser.parseString(invalidYaml)).toThrow();
    });

    it('should throw for view without docs when generating MD', () => {
      const viewWithoutDocs: ExtendedViewDefinition = {
        id: 'no-docs',
        name: 'No Docs',
        description: 'Test',
        version: '1.0',
        root: { type: 'Page' },
        include: [],
      };

      expect(() => MarkdownGenerator.generate(viewWithoutDocs)).toThrow(/docs section/);
    });
  });

  describe('Content Verification', () => {
    it('should include all nodes from all layers in the MD', () => {
      const result = MarkdownGenerator.generate(testView);

      // All nodes should appear in the nodes table
      expect(result.content).toContain('| Page |');
      expect(result.content).toContain('| PagePrompt |');
      expect(result.content).toContain('| Block |');
      expect(result.content).toContain('| BlockType |');
    });

    it('should include all relations in the MD', () => {
      const result = MarkdownGenerator.generate(testView);

      expect(result.content).toContain('| HAS_BLOCK |');
      expect(result.content).toContain('| HAS_PROMPT |');
    });

    it('should include Cypher parameters in the MD', () => {
      const result = MarkdownGenerator.generate(testView);

      expect(result.content).toContain('**Parameters:**');
      expect(result.content).toContain('`pageKey`');
      expect(result.content).toContain('"page-pricing"');
    });
  });
});
