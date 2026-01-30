// src/__tests__/generators/ViewParser.test.ts
import { describe, it, expect } from 'vitest';
import { ViewParser } from '../../generators/ViewParser.js';
import type { ExtendedViewDefinition } from '../../generators/types.js';
import * as path from 'path';

const VIEWS_DIR = path.join(process.cwd(), 'models/views');

describe('ViewParser', () => {
  describe('parseFile', () => {
    it('should parse a valid view YAML file', async () => {
      const view = await ViewParser.parseFile(
        path.join(VIEWS_DIR, 'page-generation-context.yaml')
      );

      expect(view.id).toBe('page-generation-context');
      expect(view.name).toBe('Page Generation Context');
      expect(view.root.type).toBe('Page');
      expect(view.include).toBeInstanceOf(Array);
      expect(view.include.length).toBeGreaterThan(0);
    });

    it('should throw for non-existent file', async () => {
      await expect(
        ViewParser.parseFile(path.join(VIEWS_DIR, 'non-existent.yaml'))
      ).rejects.toThrow();
    });
  });

  describe('parseString', () => {
    it('should parse valid YAML string', () => {
      const yaml = `
id: test-view
name: Test View
description: A test view
version: "1.0"

root:
  type: Page

include:
  - relation: HAS_BLOCK
    direction: outgoing
`;
      const view = ViewParser.parseString(yaml);

      expect(view.id).toBe('test-view');
      expect(view.name).toBe('Test View');
      expect(view.root.type).toBe('Page');
    });

    it('should parse view with docs section', () => {
      const yaml = `
id: test-with-docs
name: Test With Docs
description: A test view with docs
version: "1.0"

root:
  type: Page

include:
  - relation: HAS_BLOCK
    direction: outgoing

docs:
  title: Test Documentation
  category: generation
  description: This is a test view for documentation generation.
  layers:
    - name: Page Layer
      nodes: [Page, PagePrompt]
      color: blue
    - name: Block Layer
      nodes: [Block, BlockType]
      color: green
  examples:
    - name: Load page
      description: Load a page by key
      query: |
        MATCH (p:Page {key: $key})
        RETURN p
      params:
        key: "page-pricing"
`;
      const view = ViewParser.parseString(yaml);

      expect(view.docs).toBeDefined();
      expect(view.docs!.title).toBe('Test Documentation');
      expect(view.docs!.category).toBe('generation');
      expect(view.docs!.layers).toHaveLength(2);
      expect(view.docs!.layers[0].name).toBe('Page Layer');
      expect(view.docs!.layers[0].nodes).toContain('Page');
      expect(view.docs!.layers[0].color).toBe('blue');
      expect(view.docs!.examples).toHaveLength(1);
      expect(view.docs!.examples![0].name).toBe('Load page');
    });

    it('should reject invalid YAML', () => {
      const yaml = `
id: 123-invalid
name:
description:
version: not-semver
`;
      expect(() => ViewParser.parseString(yaml)).toThrow();
    });

    it('should reject invalid node type in docs layers', () => {
      const yaml = `
id: test-invalid-node
name: Test Invalid Node
description: A test with invalid node
version: "1.0"

root:
  type: Page

include:
  - relation: HAS_BLOCK
    direction: outgoing

docs:
  title: Test
  category: generation
  description: Test
  layers:
    - name: Invalid Layer
      nodes: [InvalidNodeType]
`;
      expect(() => ViewParser.parseString(yaml)).toThrow();
    });
  });

  describe('validate', () => {
    it('should validate a correct view definition', () => {
      const view: ExtendedViewDefinition = {
        id: 'valid-view',
        name: 'Valid View',
        description: 'A valid view',
        version: '1.0',
        root: { type: 'Page' },
        include: [{ relation: 'HAS_BLOCK', direction: 'outgoing' }],
      };

      expect(() => ViewParser.validate(view)).not.toThrow();
    });

    it('should reject view with invalid ID format', () => {
      const view = {
        id: 'Invalid ID With Spaces',
        name: 'Test',
        description: 'Test',
        version: '1.0',
        root: { type: 'Page' },
        include: [],
      };

      expect(() => ViewParser.validate(view)).toThrow(/kebab-case/);
    });
  });

  describe('loadAllViews', () => {
    it('should load all views from directory', async () => {
      const views = await ViewParser.loadAllViews(VIEWS_DIR);

      expect(views.length).toBeGreaterThan(0);
      expect(views.every(v => v.id && v.name)).toBe(true);
    });

    it('should skip _registry.yaml', async () => {
      const views = await ViewParser.loadAllViews(VIEWS_DIR);

      expect(views.some(v => v.id === '_registry')).toBe(false);
    });
  });

  describe('hasDocs', () => {
    it('should return true for view with docs', () => {
      const view: ExtendedViewDefinition = {
        id: 'test',
        name: 'Test',
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

      expect(ViewParser.hasDocs(view)).toBe(true);
    });

    it('should return false for view without docs', () => {
      const view: ExtendedViewDefinition = {
        id: 'test',
        name: 'Test',
        description: 'Test',
        version: '1.0',
        root: { type: 'Page' },
        include: [],
      };

      expect(ViewParser.hasDocs(view)).toBe(false);
    });
  });
});
