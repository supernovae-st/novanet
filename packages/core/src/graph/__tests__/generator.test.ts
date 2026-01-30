// packages/core/src/graph/__tests__/generator.test.ts
// Tests for schema graph generator
// TDD: Write tests first, then implementation

import { describe, it, expect } from 'vitest';
import { generateSchemaGraph, getSchemaHierarchy } from '../generator.js';
import { NODE_TYPES } from '../../types/nodes.js';

describe('graph/generator', () => {
  describe('generateSchemaGraph', () => {
    it('should generate 35 schema nodes', () => {
      const result = generateSchemaGraph();
      expect(result.nodes).toHaveLength(35);
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
      expect(projectNode?.scope).toBe('Project');
      expect(projectNode?.subcategory).toBe('foundation');
      expect(projectNode?.label).toBe('Project');
      expect(projectNode?.description).toBeDefined();
      expect(projectNode?.behavior).toBe('invariant');
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

    it('should map all 35 node types', () => {
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
    it('should return hierarchical data with all 3 scopes', () => {
      const result = getSchemaHierarchy();
      expect(Object.keys(result.scopes)).toHaveLength(3);
      expect(result.scopes.Project).toBeDefined();
      expect(result.scopes.Global).toBeDefined();
      expect(result.scopes.Shared).toBeDefined();
    });

    it('should include stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalNodes).toBe(35);
      expect(result.stats.nodesByScope.Project).toBe(14);
      expect(result.stats.nodesByScope.Global).toBe(15);
      expect(result.stats.nodesByScope.Shared).toBe(6);
    });

    it('should include all nodes', () => {
      const result = getSchemaHierarchy();
      expect(result.nodes).toHaveLength(35);
    });

    it('should include edges', () => {
      const result = getSchemaHierarchy();
      expect(result.edges.length).toBeGreaterThan(50);
    });

    it('should have correct scope definitions', () => {
      const result = getSchemaHierarchy();

      // Project scope
      expect(result.scopes.Project.label).toBe('PROJECT');
      expect(result.scopes.Project.icon).toBe('📦');
      expect(Object.keys(result.scopes.Project.subcategories)).toHaveLength(5);

      // Global scope
      expect(result.scopes.Global.label).toBe('GLOBAL');
      expect(result.scopes.Global.icon).toBe('🌍');
      expect(Object.keys(result.scopes.Global.subcategories)).toHaveLength(2);

      // Shared scope
      expect(result.scopes.Shared.label).toBe('SHARED');
      expect(result.scopes.Shared.icon).toBe('🎯');
      expect(Object.keys(result.scopes.Shared.subcategories)).toHaveLength(2);
    });

    it('should have totalEdges in stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalEdges).toBe(result.edges.length);
    });
  });
});
