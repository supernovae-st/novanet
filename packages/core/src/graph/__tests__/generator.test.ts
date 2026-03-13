// packages/core/src/graph/__tests__/generator.test.ts
// Tests for schema graph generator — v0.19.0 (60 nodes, 2 realms, 10 layers)
// TDD: Write tests first, then implementation

import { describe, it, expect } from 'vitest';
import { generateSchemaGraph, getSchemaHierarchy } from '../generator.js';
import { NODE_TYPES } from '../../types/nodes.js';

describe('graph/generator', () => {
  describe('generateSchemaGraph', () => {
    it('should generate 60 schema nodes', () => {
      const result = generateSchemaGraph();
      expect(result.nodes).toHaveLength(60);
    });

    it('should generate schema arcs from RelationRegistry', () => {
      const result = generateSchemaGraph();
      // RelationRegistry has arc types, expanded due to multi-type relations
      expect(result.arcs.length).toBeGreaterThanOrEqual(50);
    });

    it('should include all required node properties', () => {
      const result = generateSchemaGraph();
      const projectNode = result.nodes.find(n => n.nodeType === 'Project');

      expect(projectNode).toBeDefined();
      expect(projectNode?.id).toBe('schema-Project');
      expect(projectNode?.realm).toBe('org');
      expect(projectNode?.layer).toBe('foundation');
      expect(projectNode?.label).toBe('Project');
      expect(projectNode?.description).toBeDefined();
      expect(projectNode?.trait).toBe('defined');
    });

    it('should include all required arc properties', () => {
      const result = generateSchemaGraph();
      const hasPageArc = result.arcs.find(e => e.relationType === 'HAS_PAGE');

      expect(hasPageArc).toBeDefined();
      expect(hasPageArc?.sourceType).toBe('Project');
      expect(hasPageArc?.targetType).toBe('Page');
      expect(hasPageArc?.label).toBeDefined();
      expect(hasPageArc?.description).toBeDefined();
      expect(hasPageArc?.cardinality).toBeDefined();
    });

    it('should map all 57 node types', () => {
      const result = generateSchemaGraph();
      const nodeTypes = result.nodes.map(n => n.nodeType);

      // Every NODE_TYPE should be represented
      for (const nodeType of NODE_TYPES) {
        expect(nodeTypes).toContain(nodeType);
      }
    });

    it('should validate arc node types exist before creating arcs (P0 fix)', () => {
      const result = generateSchemaGraph();

      // All arcs should reference valid node types
      const validNodeTypes = new Set(NODE_TYPES);

      for (const arc of result.arcs) {
        const sourceTypes = Array.isArray(arc.sourceType) ? arc.sourceType : [arc.sourceType];
        const targetTypes = Array.isArray(arc.targetType) ? arc.targetType : [arc.targetType];

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

      // FOR_LOCALE has multiple source types going to Locale (1 target)
      const forLocaleArcs = result.arcs.filter(e => e.relationType === 'FOR_LOCALE');
      expect(forLocaleArcs.length).toBeGreaterThanOrEqual(4);
    });
  });

  describe('getSchemaHierarchy', () => {
    it('should return hierarchical data with all 2 realms', () => {
      const result = getSchemaHierarchy();
      expect(Object.keys(result.realms)).toHaveLength(2);
      expect(result.realms.org).toBeDefined();
      expect(result.realms.shared).toBeDefined();
    });

    it('should include stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalNodes).toBe(60);
      expect(result.stats.nodesByRealm.org).toBe(24);     // v0.19.0: 24 org nodes (+3 enrichment)
      expect(result.stats.nodesByRealm.shared).toBe(36);  // v0.17.0: 36 shared nodes (Market, TermSet, Term, SEOKeywordMetrics removed)
    });

    it('should include all nodes', () => {
      const result = getSchemaHierarchy();
      expect(result.nodes).toHaveLength(60);
    });

    it('should include arcs', () => {
      const result = getSchemaHierarchy();
      expect(result.arcs.length).toBeGreaterThanOrEqual(50);
    });

    it('should have correct realm definitions', () => {
      const result = getSchemaHierarchy();

      // v11.5: Org realm (6 layers - seo/geo removed)
      expect(result.realms.org.label).toBe('ORG');
      expect(result.realms.org.icon).toBe('◎');  // v11.7: Unicode icons (no emoji)
      expect(Object.keys(result.realms.org.layers)).toHaveLength(6);

      // v11.5: Shared realm (4 layers - config, locale, geography, knowledge)
      expect(result.realms.shared.label).toBe('SHARED');
      expect(result.realms.shared.icon).toBe('◉');  // v11.7: Unicode icons (no emoji)
      expect(Object.keys(result.realms.shared.layers)).toHaveLength(4);
    });

    it('should have totalArcs in stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalArcs).toBe(result.arcs.length);
    });
  });
});
