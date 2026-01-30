// src/__tests__/generators/RelationsParser.test.ts
// TDD: Tests FIRST for RelationsParser
import { describe, it, expect } from 'vitest';
import { RelationsParser } from '../../generators/RelationsParser.js';
import * as path from 'path';

const RELATIONS_PATH = path.join(process.cwd(), 'models/relations.yaml');

describe('RelationsParser', () => {
  describe('loadFromFile', () => {
    it('should load relations from YAML file', async () => {
      const edges = await RelationsParser.loadFromFile(RELATIONS_PATH);

      expect(edges).toBeInstanceOf(Array);
      expect(edges.length).toBeGreaterThan(0);
      expect(edges[0]).toHaveProperty('relation');
      expect(edges[0]).toHaveProperty('from');
      expect(edges[0]).toHaveProperty('to');
    });

    it('should throw for non-existent file with descriptive error', async () => {
      await expect(
        RelationsParser.loadFromFile('/non/existent/path.yaml')
      ).rejects.toThrow('RelationsParser: Failed to load /non/existent/path.yaml');
    });

    it('should throw for empty file path', async () => {
      await expect(
        RelationsParser.loadFromFile('')
      ).rejects.toThrow('RelationsParser: File path cannot be empty');
    });
  });

  describe('parseYaml', () => {
    it('should extract simple relations (from: A, to: B)', () => {
      const yaml = `
relations:
  HAS_CONCEPT:
    from: Project
    to: Concept
    description: "Project owns its semantic concepts"

  HAS_PAGE:
    from: Project
    to: Page
    description: "Project owns its page structures"
`;
      const edges = RelationsParser.parseYaml(yaml);

      expect(edges).toHaveLength(2);
      expect(edges[0]).toEqual({
        relation: 'HAS_CONCEPT',
        from: 'Project',
        to: 'Concept',
        description: 'Project owns its semantic concepts',
      });
      expect(edges[1]).toEqual({
        relation: 'HAS_PAGE',
        from: 'Project',
        to: 'Page',
        description: 'Project owns its page structures',
      });
    });

    it('should expand array sources ([A, B] -> creates multiple edges)', () => {
      const yaml = `
relations:
  HAS_L10N:
    from: "[Concept, Project]"
    to: "[ConceptL10n, ProjectL10n]"
    description: "Links invariant to its localized definitions"
`;
      const edges = RelationsParser.parseYaml(yaml);

      // [Concept, Project] x [ConceptL10n, ProjectL10n] = 4 combinations
      expect(edges).toHaveLength(4);
      expect(edges).toContainEqual({
        relation: 'HAS_L10N',
        from: 'Concept',
        to: 'ConceptL10n',
        description: 'Links invariant to its localized definitions',
      });
      expect(edges).toContainEqual({
        relation: 'HAS_L10N',
        from: 'Concept',
        to: 'ProjectL10n',
        description: 'Links invariant to its localized definitions',
      });
      expect(edges).toContainEqual({
        relation: 'HAS_L10N',
        from: 'Project',
        to: 'ConceptL10n',
        description: 'Links invariant to its localized definitions',
      });
      expect(edges).toContainEqual({
        relation: 'HAS_L10N',
        from: 'Project',
        to: 'ProjectL10n',
        description: 'Links invariant to its localized definitions',
      });
    });

    it('should expand array targets', () => {
      const yaml = `
relations:
  USES_CONCEPT:
    from: "[Page, Block]"
    to: Concept
    props: [purpose, temperature]
`;
      const edges = RelationsParser.parseYaml(yaml);

      expect(edges).toHaveLength(2);
      expect(edges).toContainEqual({
        relation: 'USES_CONCEPT',
        from: 'Page',
        to: 'Concept',
        props: ['purpose', 'temperature'],
      });
      expect(edges).toContainEqual({
        relation: 'USES_CONCEPT',
        from: 'Block',
        to: 'Concept',
        props: ['purpose', 'temperature'],
      });
    });

    it('should filter out inverse relations (inverse_of)', () => {
      const yaml = `
relations:
  HAS_L10N:
    from: "[Concept, Project]"
    to: "[ConceptL10n, ProjectL10n]"
    description: "Links invariant to its localized definitions"

  L10N_OF:
    from: "[ConceptL10n, ProjectL10n]"
    to: "[Concept, Project]"
    inverse_of: HAS_L10N
    description: "Inverse of HAS_L10N"

  HAS_PAGE:
    from: Project
    to: Page
`;
      const edges = RelationsParser.parseYaml(yaml);

      // HAS_L10N (4 expanded) + HAS_PAGE (1) = 5, L10N_OF filtered out
      expect(edges).toHaveLength(5);
      expect(edges.every(e => e.relation !== 'L10N_OF')).toBe(true);
    });

    it('should include relation properties (props)', () => {
      const yaml = `
relations:
  HAS_BLOCK:
    from: Page
    to: Block
    props: [position]

  SEMANTIC_LINK:
    from: Concept
    to: Concept
    props: [type, temperature]
`;
      const edges = RelationsParser.parseYaml(yaml);

      expect(edges).toHaveLength(2);
      expect(edges[0].props).toEqual(['position']);
      expect(edges[1].props).toEqual(['type', 'temperature']);
    });

    it('should handle shorthand relation syntax', () => {
      // Some relations use shorthand: { from: A, to: B, props: [...] }
      const yaml = `
relations:
  SUPPORTS_LOCALE: { from: Project, to: Locale, props: [status] }
  DEFAULT_LOCALE: { from: Project, to: Locale }
`;
      const edges = RelationsParser.parseYaml(yaml);

      expect(edges).toHaveLength(2);
      expect(edges[0]).toEqual({
        relation: 'SUPPORTS_LOCALE',
        from: 'Project',
        to: 'Locale',
        props: ['status'],
      });
      expect(edges[1]).toEqual({
        relation: 'DEFAULT_LOCALE',
        from: 'Project',
        to: 'Locale',
      });
    });

    it('should handle wildcard from: "*" as generic', () => {
      const yaml = `
relations:
  FOR_LOCALE:
    from: "*"
    to: Locale
`;
      const edges = RelationsParser.parseYaml(yaml);

      expect(edges).toHaveLength(1);
      expect(edges[0]).toEqual({
        relation: 'FOR_LOCALE',
        from: '*',
        to: 'Locale',
      });
    });

    it('should return empty array for invalid YAML', () => {
      const yaml = `not: valid: yaml: content`;

      const edges = RelationsParser.parseYaml(yaml);
      expect(edges).toEqual([]);
    });

    it('should return empty array for YAML without relations key', () => {
      const yaml = `
something_else:
  key: value
`;
      const edges = RelationsParser.parseYaml(yaml);
      expect(edges).toEqual([]);
    });
  });

  describe('real file parsing', () => {
    it('should parse all non-inverse relations from models/relations.yaml', async () => {
      const edges = await RelationsParser.loadFromFile(RELATIONS_PATH);

      // Verify we get the expected number of relations:
      // - 50 base relations in YAML
      // - 5 inverse relations filtered out (L10N_OF, OUTPUT_OF, BLOCK_OF, USED_BY, HAS_LOCALIZED_CONTENT)
      // - 45 non-inverse relations
      // - After expansion (array notation), 67 total edges
      expect(edges.length).toBe(67);

      // Verify no inverse relations are included
      const inverseRelations = ['L10N_OF', 'OUTPUT_OF', 'BLOCK_OF', 'USED_BY', 'HAS_LOCALIZED_CONTENT'];
      for (const inverse of inverseRelations) {
        expect(edges.some(e => e.relation === inverse)).toBe(false);
      }

      // Verify key relations are present
      const expectedRelations = [
        'HAS_CONCEPT',
        'HAS_PAGE',
        'HAS_L10N',
        'HAS_OUTPUT',
        'FOR_LOCALE',
        'USES_CONCEPT',
        'SEMANTIC_LINK',
      ];
      for (const rel of expectedRelations) {
        expect(edges.some(e => e.relation === rel)).toBe(true);
      }
    });

    it('should correctly expand HAS_L10N array sources', async () => {
      const edges = await RelationsParser.loadFromFile(RELATIONS_PATH);

      const hasL10nEdges = edges.filter(e => e.relation === 'HAS_L10N');
      // [Concept, Project] x [ConceptL10n, ProjectL10n] = 4 combinations
      expect(hasL10nEdges.length).toBe(4);
    });

    it('should correctly expand USES_CONCEPT array sources', async () => {
      const edges = await RelationsParser.loadFromFile(RELATIONS_PATH);

      const usesConceptEdges = edges.filter(e => e.relation === 'USES_CONCEPT');
      // [Page, Block] -> Concept = 2 edges
      expect(usesConceptEdges.length).toBe(2);
    });
  });

  describe('edge cases', () => {
    it('should handle empty relations object', () => {
      const yaml = `
relations: {}
`;
      const edges = RelationsParser.parseYaml(yaml);
      expect(edges).toEqual([]);
    });

    it('should handle relation with null values', () => {
      const yaml = `
relations:
  BROKEN:
    from: null
    to: Node
`;
      const edges = RelationsParser.parseYaml(yaml);
      // Should skip invalid relations
      expect(edges).toEqual([]);
    });

    it('should handle relation missing from or to', () => {
      const yaml = `
relations:
  INCOMPLETE:
    from: Project
    description: "Missing to field"
`;
      const edges = RelationsParser.parseYaml(yaml);
      expect(edges).toEqual([]);
    });
  });
});
