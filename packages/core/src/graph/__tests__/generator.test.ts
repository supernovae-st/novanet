// packages/core/src/graph/__tests__/generator.test.ts
// Tests for schema graph generator — v11.2.0 (62 nodes, 2 realms, 3 job nodes removed)
// TDD: Write tests first, then implementation

import { describe, it, expect } from 'vitest';
import { generateSchemaGraph, getSchemaHierarchy } from '../generator.js';
import { NODE_TYPES } from '../../types/nodes.js';

describe('graph/generator', () => {
  describe('generateSchemaGraph', () => {
    it('should generate 62 schema nodes', () => {
      const result = generateSchemaGraph();
      expect(result.nodes).toHaveLength(62);
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
      expect(projectNode?.trait).toBe('invariant');
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

    it('should map all 62 node types', () => {
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
      expect(result.stats.totalNodes).toBe(62);
      expect(result.stats.nodesByRealm.org).toBe(30);   // v11.2: 30 org nodes (3 job nodes removed)
      expect(result.stats.nodesByRealm.shared).toBe(32);   // v11.2: 32 shared nodes
    });

    it('should include all nodes', () => {
      const result = getSchemaHierarchy();
      expect(result.nodes).toHaveLength(62);
    });

    it('should include arcs', () => {
      const result = getSchemaHierarchy();
      expect(result.arcs.length).toBeGreaterThanOrEqual(50);
    });

    it('should have correct realm definitions', () => {
      const result = getSchemaHierarchy();

      // v11.2: Org realm (7 layers - includes SEO)
      expect(result.realms.org.label).toBe('ORG');
      expect(result.realms.org.icon).toBe('🏢');
      expect(Object.keys(result.realms.org.layers)).toHaveLength(7);

      // v11.2: Shared realm (2 layers - config, locale-knowledge)
      expect(result.realms.shared.label).toBe('SHARED');
      expect(result.realms.shared.icon).toBe('🌍');
      expect(Object.keys(result.realms.shared.layers)).toHaveLength(2);
    });

    it('should have totalArcs in stats', () => {
      const result = getSchemaHierarchy();
      expect(result.stats.totalArcs).toBe(result.arcs.length);
    });
  });
});
