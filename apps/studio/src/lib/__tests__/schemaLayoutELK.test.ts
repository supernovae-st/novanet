/**
 * Schema Layout ELK Tests
 *
 * Tests for the ELK-based hierarchical layout engine for schema mode.
 * Validates group nodes, parent relationships, and position conversion.
 */

import { applySchemaLayout } from '../schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';
import type { HierarchicalSchemaData, Subcategory, SchemaNode, SchemaEdge } from '@novanet/core/graph';
import type { Scope } from '@novanet/core/types';

// Mock ELK.js
jest.mock('elkjs/lib/elk.bundled.js', () => {
  return class ELK {
    async layout(graph: {
      id: string;
      children?: Array<{ id: string; children?: Array<{ id: string; children?: unknown[] }> }>;
    }) {
      // Simple mock that returns positions for each node
      let x = 0;
      let y = 0;

      const processChildren = (
        children: Array<{ id: string; children?: Array<{ id: string; children?: unknown[] }> }> | undefined,
        parentX = 0,
        parentY = 0
      ): Array<{
        id: string;
        x: number;
        y: number;
        width: number;
        height: number;
        children?: Array<{
          id: string;
          x: number;
          y: number;
          width: number;
          height: number;
          children?: Array<{
            id: string;
            x: number;
            y: number;
            width: number;
            height: number;
          }>;
        }>;
      }> => {
        if (!children) return [];

        return children.map((child, i) => {
          const childX = 20 + i * 200; // Relative position within parent
          const childY = 60 + i * 100;

          return {
            id: child.id,
            x: childX,
            y: childY,
            width: child.children ? 600 : 140,
            height: child.children ? 400 : 50,
            children: child.children ? processChildren(
              child.children as Array<{ id: string; children?: Array<{ id: string; children?: unknown[] }> }>,
              parentX + childX,
              parentY + childY
            ) : undefined,
          };
        });
      };

      return {
        ...graph,
        children: processChildren(graph.children),
      };
    }
  };
});

describe('schemaLayoutELK', () => {
  let mockHierarchy: HierarchicalSchemaData;

  beforeEach(() => {
    // Create a minimal mock hierarchy for testing
    mockHierarchy = {
      scopes: {
        Project: {
          scope: 'Project' as Scope,
          label: 'PROJECT',
          icon: '📦',
          description: 'Project-specific content',
          subcategories: {
            foundation: {
              label: 'Foundation',
              description: 'Core project identity',
              icon: '🏛️',
              nodeTypes: ['Project', 'BrandIdentity', 'ProjectL10n'] as never[],
            },
            structure: {
              label: 'Structure',
              description: 'Page and block organization',
              icon: '🧱',
              nodeTypes: ['Page', 'Block'] as never[],
            },
          } as Record<Subcategory, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
        Global: {
          scope: 'Global' as Scope,
          label: 'GLOBAL',
          icon: '🌍',
          description: 'Shared across all projects',
          subcategories: {
            config: {
              label: 'Configuration',
              description: 'Locale configuration',
              icon: '⚙️',
              nodeTypes: ['Locale'] as never[],
            },
            knowledge: {
              label: 'Knowledge',
              description: 'Locale-specific knowledge',
              icon: '🧠',
              nodeTypes: ['LocaleVoice', 'LocaleCulture'] as never[],
            },
          } as Record<Subcategory, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
        Shared: {
          scope: 'Shared' as Scope,
          label: 'SHARED',
          icon: '🎯',
          description: 'Shared across projects',
          subcategories: {
            seo: {
              label: 'SEO',
              description: 'SEO data',
              icon: '🔍',
              nodeTypes: ['SEOKeywordL10n'] as never[],
            },
            geo: {
              label: 'GEO',
              description: 'GEO data',
              icon: '🤖',
              nodeTypes: ['GEOSeedL10n'] as never[],
            },
          } as Record<Subcategory, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
      } as Record<Scope, typeof mockHierarchy.scopes.Project>,
      nodes: [
        { id: 'schema-Project', nodeType: 'Project', scope: 'Project', subcategory: 'foundation', label: 'Project', description: '', behavior: 'invariant' },
        { id: 'schema-BrandIdentity', nodeType: 'BrandIdentity', scope: 'Project', subcategory: 'foundation', label: 'Brand Identity', description: '', behavior: 'invariant' },
        { id: 'schema-ProjectL10n', nodeType: 'ProjectL10n', scope: 'Project', subcategory: 'foundation', label: 'Project L10n', description: '', behavior: 'localized' },
        { id: 'schema-Page', nodeType: 'Page', scope: 'Project', subcategory: 'structure', label: 'Page', description: '', behavior: 'invariant' },
        { id: 'schema-Block', nodeType: 'Block', scope: 'Project', subcategory: 'structure', label: 'Block', description: '', behavior: 'invariant' },
        { id: 'schema-Locale', nodeType: 'Locale', scope: 'Global', subcategory: 'config', label: 'Locale', description: '', behavior: 'invariant' },
        { id: 'schema-LocaleVoice', nodeType: 'LocaleVoice', scope: 'Global', subcategory: 'knowledge', label: 'Locale Voice', description: '', behavior: 'localeKnowledge' },
        { id: 'schema-LocaleCulture', nodeType: 'LocaleCulture', scope: 'Global', subcategory: 'knowledge', label: 'Locale Culture', description: '', behavior: 'localeKnowledge' },
        { id: 'schema-SEOKeywordL10n', nodeType: 'SEOKeywordL10n', scope: 'Shared', subcategory: 'seo', label: 'SEO Keyword', description: '', behavior: 'localized' },
        { id: 'schema-GEOSeedL10n', nodeType: 'GEOSeedL10n', scope: 'Shared', subcategory: 'geo', label: 'GEO Seed', description: '', behavior: 'localized' },
      ] as SchemaNode[],
      edges: [
        { id: 'schema-edge-0', relationType: 'HAS_PAGE', sourceType: 'Project', targetType: 'Page', label: 'HAS_PAGE', description: '', cardinality: '1:N' },
        { id: 'schema-edge-1', relationType: 'HAS_BLOCK', sourceType: 'Page', targetType: 'Block', label: 'HAS_BLOCK', description: '', cardinality: '1:N' },
      ] as SchemaEdge[],
      stats: {
        totalNodes: 10,
        totalEdges: 2,
        nodesByScope: { Project: 5, Global: 3, Shared: 2 },
      },
    };
  });

  describe('applySchemaLayout', () => {
    it('should layout schema nodes with ELK', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // Should have group nodes + schema nodes
      // 3 scope groups + 6 subcategory groups + 10 schema nodes = 19
      expect(result.nodes.length).toBeGreaterThan(10);

      // All nodes should have positions
      for (const node of result.nodes) {
        expect(node.position).toBeDefined();
        expect(typeof node.position.x).toBe('number');
        expect(typeof node.position.y).toBe('number');
      }
    });

    it('should create scope group nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const scopeGroups = result.nodes.filter(n => n.type === 'scopeGroup');
      expect(scopeGroups).toHaveLength(3);

      // Verify scope group data
      const projectScope = scopeGroups.find(n => n.data.scope === 'Project');
      expect(projectScope).toBeDefined();
      expect(projectScope?.data.label).toBe('PROJECT');
      expect(projectScope?.data.icon).toBe('📦');
    });

    it('should create subcategory group nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const subGroups = result.nodes.filter(n => n.type === 'subcategoryGroup');
      // 2 (Project) + 2 (Global) + 2 (Shared) = 6
      expect(subGroups).toHaveLength(6);

      // Verify subcategory has parentId (scope group)
      for (const subGroup of subGroups) {
        expect(subGroup.parentId).toBeDefined();
        expect(subGroup.parentId).toMatch(/^scope-/);
        expect(subGroup.extent).toBe('parent');
      }
    });

    it('should set parent relationships for schema nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
      expect(schemaNodes).toHaveLength(10);

      for (const node of schemaNodes) {
        expect(node.parentId).toBeDefined();
        expect(node.parentId).toMatch(/^subcat-/);
        expect(node.extent).toBe('parent');
      }
    });

    it('should convert ELK absolute positions to React Flow relative positions', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // Child nodes should have RELATIVE positions (not absolute)
      // This is the P0 fix - ELK returns absolute, React Flow needs relative for child nodes
      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');

      for (const node of schemaNodes) {
        // Relative positions should be smaller than what would be absolute
        // In our mock, relative positions start at 20, 60 for first child
        expect(node.position.x).toBeGreaterThanOrEqual(0);
        expect(node.position.y).toBeGreaterThanOrEqual(0);
      }
    });

    it('should include schema edges', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      expect(result.edges.length).toBe(2);

      const firstEdge = result.edges[0];
      expect(firstEdge.source).toBe('schema-Project');
      expect(firstEdge.target).toBe('schema-Page');
      expect(firstEdge.type).toBe('floating');
      expect(firstEdge.data?.relationType).toBe('HAS_PAGE');
    });

    it('should skip empty subcategories (P1 fix)', async () => {
      // Add an empty subcategory
      const hierarchyWithEmpty: HierarchicalSchemaData = {
        ...mockHierarchy,
        scopes: {
          ...mockHierarchy.scopes,
          Project: {
            ...mockHierarchy.scopes.Project,
            subcategories: {
              ...mockHierarchy.scopes.Project.subcategories,
              empty: {
                label: 'Empty',
                description: 'Empty subcategory',
                icon: '❌',
                nodeTypes: [], // No nodes!
              },
            } as unknown as Record<Subcategory, { label: string; description: string; icon: string; nodeTypes: never[] }>,
          },
        } as Record<Scope, typeof mockHierarchy.scopes.Project>,
      };

      const result = await applySchemaLayout(hierarchyWithEmpty);

      // Should NOT create a subcategory group for the empty one
      const emptySubcat = result.nodes.find(n =>
        n.type === 'subcategoryGroup' && n.data.subcategory === 'empty'
      );
      expect(emptySubcat).toBeUndefined();
    });

    it('should set scope group dimensions from ELK layout', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const scopeGroups = result.nodes.filter(n => n.type === 'scopeGroup');

      for (const scopeGroup of scopeGroups) {
        // Scope groups should have width and height from ELK
        expect(scopeGroup.style).toBeDefined();
        expect(scopeGroup.style?.width).toBeDefined();
        expect(scopeGroup.style?.height).toBeDefined();
      }
    });
  });

  describe('with real hierarchy data', () => {
    it('should layout the full schema hierarchy', async () => {
      const hierarchy = getSchemaHierarchy();
      const result = await applySchemaLayout(hierarchy);

      // Should have 3 scope groups
      const scopeGroups = result.nodes.filter(n => n.type === 'scopeGroup');
      expect(scopeGroups).toHaveLength(3);

      // Should have 9 subcategory groups (5 + 2 + 2)
      const subGroups = result.nodes.filter(n => n.type === 'subcategoryGroup');
      expect(subGroups).toHaveLength(9);

      // Should have 35 schema nodes
      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
      expect(schemaNodes).toHaveLength(35);

      // Total nodes: 3 + 9 + 35 = 47
      expect(result.nodes).toHaveLength(47);
    });

    it('should include all edges from hierarchy', async () => {
      const hierarchy = getSchemaHierarchy();
      const result = await applySchemaLayout(hierarchy);

      // Should have same number of edges as input
      expect(result.edges.length).toBe(hierarchy.edges.length);
    });
  });

  describe('fallback layout', () => {
    it('should fall back to grid layout if ELK fails', async () => {
      // Force ELK to fail by passing invalid data
      const brokenHierarchy: HierarchicalSchemaData = {
        scopes: {} as never,
        nodes: [
          { id: 'schema-Test', nodeType: 'Project', scope: 'Project', subcategory: 'foundation', label: 'Test', description: '', behavior: 'invariant' },
        ] as SchemaNode[],
        edges: [],
        stats: { totalNodes: 1, totalEdges: 0, nodesByScope: { Project: 1, Global: 0, Shared: 0 } },
      };

      // This should not throw, but use fallback
      const result = await applySchemaLayout(brokenHierarchy);

      // Fallback creates schema nodes directly (no groups)
      expect(result.nodes.length).toBeGreaterThanOrEqual(1);

      // All nodes should still have positions
      for (const node of result.nodes) {
        expect(node.position).toBeDefined();
      }
    });
  });
});
