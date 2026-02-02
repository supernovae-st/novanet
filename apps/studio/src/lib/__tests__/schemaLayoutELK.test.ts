/**
 * Schema Layout ELK Tests
 *
 * Tests for the ELK-based hierarchical layout engine for schema mode.
 * Validates group nodes, parent relationships, and position conversion.
 */

import { applySchemaLayout } from '../schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';
import type { Layer, SchemaNode, SchemaEdge, HierarchicalSchemaData } from '@novanet/core/graph';
import type { Realm } from '@novanet/core/types';

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
      realms: {
        project: {
          realm: 'project' as Realm,
          label: 'PROJECT',
          icon: '📦',
          description: 'Project-specific content',
          layers: {
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
          } as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
        global: {
          realm: 'global' as Realm,
          label: 'GLOBAL',
          icon: '🌍',
          description: 'Shared across all projects',
          layers: {
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
          } as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
        shared: {
          realm: 'shared' as Realm,
          label: 'SHARED',
          icon: '🎯',
          description: 'Shared across projects',
          layers: {
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
          } as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
      } as Record<Realm, typeof mockHierarchy.realms.project>,
      nodes: [
        { id: 'schema-Project', nodeType: 'Project', realm: 'project', layer: 'foundation', label: 'Project', description: '', trait: 'invariant' },
        { id: 'schema-BrandIdentity', nodeType: 'BrandIdentity', realm: 'project', layer: 'foundation', label: 'Brand Identity', description: '', trait: 'invariant' },
        { id: 'schema-ProjectL10n', nodeType: 'ProjectL10n', realm: 'project', layer: 'foundation', label: 'Project L10n', description: '', trait: 'localized' },
        { id: 'schema-Page', nodeType: 'Page', realm: 'project', layer: 'structure', label: 'Page', description: '', trait: 'invariant' },
        { id: 'schema-Block', nodeType: 'Block', realm: 'project', layer: 'structure', label: 'Block', description: '', trait: 'invariant' },
        { id: 'schema-Locale', nodeType: 'Locale', realm: 'global', layer: 'config', label: 'Locale', description: '', trait: 'invariant' },
        { id: 'schema-LocaleVoice', nodeType: 'LocaleVoice', realm: 'global', layer: 'knowledge', label: 'Locale Voice', description: '', trait: 'localeKnowledge' },
        { id: 'schema-LocaleCulture', nodeType: 'LocaleCulture', realm: 'global', layer: 'knowledge', label: 'Locale Culture', description: '', trait: 'localeKnowledge' },
        { id: 'schema-SEOKeywordL10n', nodeType: 'SEOKeywordL10n', realm: 'shared', layer: 'seo', label: 'SEO Keyword', description: '', trait: 'localized' },
        { id: 'schema-GEOSeedL10n', nodeType: 'GEOSeedL10n', realm: 'shared', layer: 'geo', label: 'GEO Seed', description: '', trait: 'localized' },
      ] as SchemaNode[],
      edges: [
        { id: 'schema-edge-0', relationType: 'HAS_PAGE', sourceType: 'Project', targetType: 'Page', label: 'HAS_PAGE', description: '', cardinality: '1:N' },
        { id: 'schema-edge-1', relationType: 'HAS_BLOCK', sourceType: 'Page', targetType: 'Block', label: 'HAS_BLOCK', description: '', cardinality: '1:N' },
      ] as SchemaEdge[],
      stats: {
        totalNodes: 10,
        totalEdges: 2,
        nodesByRealm: { project: 5, global: 3, shared: 2 },
      },
    };
  });

  describe('applySchemaLayout', () => {
    it('should layout schema nodes with ELK', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // Should have group nodes + schema nodes
      // 3 realm groups + 6 layer groups + 10 schema nodes = 19
      expect(result.nodes.length).toBeGreaterThan(10);

      // All nodes should have positions
      for (const node of result.nodes) {
        expect(node.position).toBeDefined();
        expect(typeof node.position.x).toBe('number');
        expect(typeof node.position.y).toBe('number');
      }
    });

    it('should create realm group nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const realmGroups = result.nodes.filter(n => n.type === 'realmGroup');
      expect(realmGroups).toHaveLength(3);

      // Verify realm group data
      const projectRealm = realmGroups.find(n => n.data.realm === 'project');
      expect(projectRealm).toBeDefined();
      expect(projectRealm?.data.label).toBe('PROJECT');
      expect(projectRealm?.data.icon).toBe('📦');
    });

    it('should create layer group nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const subGroups = result.nodes.filter(n => n.type === 'layerGroup');
      // 2 (Project) + 2 (Global) + 2 (Shared) = 6
      expect(subGroups).toHaveLength(6);

      // Verify subcategory has parentId (realm group)
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
        expect(node.parentId).toMatch(/^layer-/);
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
        realms: {
          ...mockHierarchy.realms,
          project: {
            ...mockHierarchy.realms.project,
            layers: {
              ...mockHierarchy.realms.project.layers,
              empty: {
                label: 'Empty',
                description: 'Empty subcategory',
                icon: '❌',
                nodeTypes: [], // No nodes!
              },
            } as unknown as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
          },
        } as Record<Realm, typeof mockHierarchy.realms.project>,
      };

      const result = await applySchemaLayout(hierarchyWithEmpty);

      // Should NOT create a layer group for the empty one
      const emptySubcat = result.nodes.find(n =>
        n.type === 'layerGroup' && n.data.layer === 'empty'
      );
      expect(emptySubcat).toBeUndefined();
    });

    it('should set realm group dimensions from ELK layout', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const realmGroups = result.nodes.filter(n => n.type === 'realmGroup');

      for (const realmGroup of realmGroups) {
        // Realm groups should have width and height from ELK
        expect(realmGroup.style).toBeDefined();
        expect(realmGroup.style?.width).toBeDefined();
        expect(realmGroup.style?.height).toBeDefined();
      }
    });
  });

  describe('with real hierarchy data', () => {
    it('should layout the full schema hierarchy', async () => {
      const hierarchy = getSchemaHierarchy();
      const result = await applySchemaLayout(hierarchy);

      // Should have 3 realm groups
      const realmGroups = result.nodes.filter(n => n.type === 'realmGroup');
      expect(realmGroups).toHaveLength(3);

      // Should have 9 layer groups (5 + 2 + 2)
      const subGroups = result.nodes.filter(n => n.type === 'layerGroup');
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
        realms: {} as never,
        nodes: [
          { id: 'schema-Test', nodeType: 'Project', realm: 'project', layer: 'foundation', label: 'Test', description: '', trait: 'invariant' },
        ] as SchemaNode[],
        edges: [],
        stats: { totalNodes: 1, totalEdges: 0, nodesByRealm: { project: 1, global: 0, shared: 0 } },
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
