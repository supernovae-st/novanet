// packages/core/src/graph/__tests__/generator.test.ts
// Tests for schema graph generator — v9.0.0
// TDD: Write tests first, then implementation

import { describe, it, expect } from 'vitest';
import { generateSchemaGraph, getSchemaHierarchy } from '../generator.js';
import { NODE_TYPES } from '../../types/nodes.js';

describe('graph/generator', () => {
  describe('generateSchemaGraph', () => {
    it('should generate 44 schema nodes', () => {
      const result = generateSchemaGraph();
      expect(result.nodes).toHaveLength(44);
    });

    it('should generate schema edges from RelationRegistry', () => {
      const result = generateSchemaGraph();
      // RelationRegistry has 50 relation types, expanded to ~89 edges
      // due to multi-type relations creating multiple edges
      expect(result.edges.length).toBeGreaterThan(50);
    });

    it('should include all required node properties', () => {
      const result = generateSchemaGraph();
      const projectNode = result.nodes.find(n => n.nodeType === 'Project');

      expect(projectNode).toBeDefined();
      expect(projectNode?.id).toBe('schema-Project');
      expect(projectNode?.realm).toBe('project');
      expect(projectNode?.layer).toBe('foundation');
      expect(projectNode?.label).toBe('Project');
      expect(projectNode?.description).toBeDefined();
      expect(projectNode?.trait).toBe('invariant');
    });

    it('should include all required edge properties', () => {
      const result = generateSchemaGraph();
      const hasPageEdge = result.edges.find(e => e.relationType === 'HAS_PAGE');

      expect(hasPageEdge).toBeDefined();
      expect(hasPageEdge?.sourceType).toBe('Project');
      expect(hasPageEdge?.targetType).toBe('Page');
      expect(hasPageEdge?.label).toBeDefined();
      expect(hasPageEdge?.description).toBeDefined();
      expect(hasPageEdge?.cardinality).toBeDefined();
    });

    it('should map all 44 node types', () => {
      const result = generateSchemaGraph();
      const nodeTypes = result.nodes.map(n => n.nodeType);

      // Every NODE_TYPE should be represented
      for (const nodeType of NODE_TYPES) {
        expect(nodeTypes).toContain(nodeType);
      }
    });

    it('should validate edge node types exist before creating edges (P0 fix)', () => {
      const result = generateSchemaGraph();

      // All edges should reference valid node types
      const validNodeTypes = new Set(NODE_TYPES);

      for (const edge of result.edges) {
        const sourceTypes = Array.isArray(edge.sourceType) ? edge.sourceType : [edge.sourceType];
        const targetTypes = Array.isArray(edge.targetType) ? edge.targetType : [edge.targetType];

        for (const source of sourceTypes) {
          expect(validNodeTypes.has(source)).toBe(true);
        }
        for (const target of targetTypes) {
          expect(validNodeTypes.has(target)).toBe(true);
        }
      }
    });

    it('should create Cartesian product for multi-type relations', () => {
      const result = generateSchemaGraph();

      // FOR_LOCALE has multiple source types: ConceptL10n, ProjectL10n, PageL10n, BlockL10n, SEOKeywordL10n, GEOSeedL10n
      // All going to Locale (1 target)
      // Should create 6 edges for this relation
      const forLocaleEdges = result.edges.filter(e => e.relationType === 'FOR_LOCALE');
      expect(forLocaleEdges.length).toBe(6);
    });
  });

  describe('getSchemaHierarchy', () => {
    it('should return hierarchical data with all 3 realms', () => {
      const result = getSchemaHierarchy();
      expect(Object.keys(result.realms)).toHaveLength(3);
      expect(result.realms.project).toBeDefined();
      expect(result.realms.global).toBeDefined();
      expect(result.realms.shared).toBeDefined();
    });

    it('should include stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalNodes).toBe(44);
      expect(result.stats.nodesByRealm.project).toBe(21);
      expect(result.stats.nodesByRealm.global).toBe(15);
      expect(result.stats.nodesByRealm.shared).toBe(8);
    });

    it('should include all nodes', () => {
      const result = getSchemaHierarchy();
      expect(result.nodes).toHaveLength(44);
    });

    it('should include edges', () => {
      const result = getSchemaHierarchy();
      expect(result.edges.length).toBeGreaterThan(50);
    });

    it('should have correct realm definitions', () => {
      const result = getSchemaHierarchy();

      // Project realm
      expect(result.realms.project.label).toBe('PROJECT');
      expect(result.realms.project.icon).toBe('📦');
      expect(Object.keys(result.realms.project.layers)).toHaveLength(5);

      // Global realm
      expect(result.realms.global.label).toBe('GLOBAL');
      expect(result.realms.global.icon).toBe('🌍');
      expect(Object.keys(result.realms.global.layers)).toHaveLength(2);

      // Shared realm
      expect(result.realms.shared.label).toBe('SHARED');
      expect(result.realms.shared.icon).toBe('🎯');
      expect(Object.keys(result.realms.shared.layers)).toHaveLength(2);
    });

    it('should have totalEdges in stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalEdges).toBe(result.edges.length);
    });
  });
});
