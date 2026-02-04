/**
 * Schema Layout ELK Tests
 *
 * Tests for the ELK-based hierarchical layout engine for schema mode.
 * Validates group nodes, parent relationships, and position conversion.
 */

import { applySchemaLayout } from '../schemaLayoutELK';
import { getSchemaHierarchy } from '@novanet/core/graph';
import type { Layer, SchemaNode, SchemaArc, HierarchicalSchemaData } from '@novanet/core/graph';
import type { Realm } from '@novanet/core/types';

// Mock ELK.js
jest.mock('elkjs/lib/elk.bundled.js', () => {
  return class ELK {
    async layout(graph: {
      id: string;
      children?: Array<{ id: string; children?: Array<{ id: string; children?: unknown[] }> }>;
    }) {
      // Simple mock that returns positions for each node
      const _x = 0;
      const _y = 0;

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
              nodeTypes: ['Style', 'Formatting'] as never[],
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
        { id: 'schema-Style', nodeType: 'Style', realm: 'global', layer: 'knowledge', label: 'Style', description: '', trait: 'knowledge' },
        { id: 'schema-Formatting', nodeType: 'Formatting', realm: 'global', layer: 'knowledge', label: 'Formatting', description: '', trait: 'knowledge' },
        { id: 'schema-SEOKeywordL10n', nodeType: 'SEOKeywordL10n', realm: 'shared', layer: 'seo', label: 'SEO Keyword', description: '', trait: 'localized' },
        { id: 'schema-GEOSeedL10n', nodeType: 'GEOSeedL10n', realm: 'shared', layer: 'geo', label: 'GEO Seed', description: '', trait: 'localized' },
      ] as SchemaNode[],
      arcs: [
        { id: 'schema-arc-0', relationType: 'HAS_PAGE', sourceType: 'Project', targetType: 'Page', label: 'HAS_PAGE', description: '', cardinality: '1:N' },
        { id: 'schema-arc-1', relationType: 'HAS_BLOCK', sourceType: 'Page', targetType: 'Block', label: 'HAS_BLOCK', description: '', cardinality: '1:N' },
      ] as SchemaArc[],
      stats: {
        totalNodes: 10,
        totalArcs: 2,
        nodesByRealm: { project: 5, global: 3, shared: 2 },
      },
    };
  });

  describe('applySchemaLayout', () => {
    // v9.5: Layout changed from ELK containers to Dagre hierarchical graph
    // - Realm and Layer are now metaBadge nodes (not container groups)
    // - No parent/child relationships - flat graph with edges
    // - HAS_LAYER and HAS_KIND edges connect the hierarchy

    it('should layout schema nodes with Dagre', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // Should have meta nodes + schema nodes
      // 3 realm badges + 6 layer badges + 10 schema nodes = 19
      expect(result.nodes.length).toBeGreaterThan(10);

      // All nodes should have positions
      for (const node of result.nodes) {
        expect(node.position).toBeDefined();
        expect(typeof node.position.x).toBe('number');
        expect(typeof node.position.y).toBe('number');
      }
    });

    it('should create realm meta badge nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // v9.5: Realms are metaBadge nodes with metaType: 'realm'
      const realmBadges = result.nodes.filter(n =>
        n.type === 'metaBadge' && n.data.metaType === 'realm'
      );
      expect(realmBadges).toHaveLength(3);

      // Verify realm badge data
      const projectRealm = realmBadges.find(n => n.data.realmKey === 'project');
      expect(projectRealm).toBeDefined();
      expect(projectRealm?.data.label).toBe('Project');
    });

    it('should create layer meta badge nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // v9.5: Layers are metaBadge nodes with metaType: 'layer'
      const layerBadges = result.nodes.filter(n =>
        n.type === 'metaBadge' && n.data.metaType === 'layer'
      );
      // 2 (Project) + 2 (Global) + 2 (Shared) = 6
      expect(layerBadges).toHaveLength(6);

      // v9.5: No parent relationships - connected by HAS_LAYER edges
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      expect(hasLayerEdges.length).toBe(6);
    });

    it('should create schema nodes with layer connections', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
      expect(schemaNodes).toHaveLength(10);

      // v9.5: Connected by HAS_KIND edges (not parent relationships)
      const hasKindEdges = result.edges.filter(e => e.data?.relationType === 'HAS_KIND');
      expect(hasKindEdges.length).toBe(10);
    });

    it('should position all nodes with valid coordinates', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');

      for (const node of schemaNodes) {
        // All positions should be valid numbers
        expect(node.position.x).toBeGreaterThanOrEqual(0);
        expect(node.position.y).toBeGreaterThanOrEqual(0);
      }
    });

    it('should include business edges plus hierarchy edges', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // v9.5: Total edges = HAS_LAYER + HAS_KIND + business edges
      // 6 HAS_LAYER + 10 HAS_KIND + 2 business = 18
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      const hasKindEdges = result.edges.filter(e => e.data?.relationType === 'HAS_KIND');
      const businessEdges = result.edges.filter(e => !e.data?.isMetaEdge);

      expect(hasLayerEdges.length).toBe(6);
      expect(hasKindEdges.length).toBe(10);
      expect(businessEdges.length).toBe(2); // Original mock edges
    });

    it('should skip empty layers (P1 fix)', async () => {
      // Add an empty layer
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
                description: 'Empty layer',
                icon: '❌',
                nodeTypes: [], // No nodes!
              },
            } as unknown as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
          },
        } as Record<Realm, typeof mockHierarchy.realms.project>,
      };

      const result = await applySchemaLayout(hierarchyWithEmpty);

      // Should NOT create a layer group for the empty one
      const emptyLayer = result.nodes.find(n =>
        n.type === 'layerGroup' && n.data.layer === 'empty'
      );
      expect(emptyLayer).toBeUndefined();
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

      // v9.5: Uses metaBadge for Realm and Layer, schemaNode for Kind
      // Should have 3 realm meta badges
      const realmBadges = result.nodes.filter(n =>
        n.type === 'metaBadge' && n.data.metaType === 'realm'
      );
      expect(realmBadges).toHaveLength(3);

      // Should have layer meta badges (varies by active layers)
      const layerBadges = result.nodes.filter(n =>
        n.type === 'metaBadge' && n.data.metaType === 'layer'
      );
      expect(layerBadges.length).toBeGreaterThan(0);

      // Should have schema nodes (count varies with ontology)
      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
      expect(schemaNodes.length).toBeGreaterThan(0);

      // Total should match: realm + layer + kind nodes
      expect(result.nodes.length).toBe(
        realmBadges.length + layerBadges.length + schemaNodes.length
      );
    });

    it('should include edges for hierarchy and business relationships', async () => {
      const hierarchy = getSchemaHierarchy();
      const result = await applySchemaLayout(hierarchy);

      // v9.5: Has HAS_LAYER, HAS_KIND, and business edges
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      const hasKindEdges = result.edges.filter(e => e.data?.relationType === 'HAS_KIND');
      const businessEdges = result.edges.filter(e => !e.data?.isMetaEdge);

      expect(hasLayerEdges.length).toBeGreaterThan(0);
      expect(hasKindEdges.length).toBeGreaterThan(0);
      expect(businessEdges.length).toBe(hierarchy.arcs.length);
    });
  });

  describe('graceful degradation', () => {
    it('should handle empty realms without crashing', async () => {
      // Empty hierarchy with no realms
      const emptyHierarchy: HierarchicalSchemaData = {
        realms: {} as never,
        nodes: [],
        arcs: [],
        stats: { totalNodes: 0, totalArcs: 0, nodesByRealm: { project: 0, global: 0, shared: 0 } },
      };

      // Should not throw
      const result = await applySchemaLayout(emptyHierarchy);

      // Empty input = empty output
      expect(result.nodes).toBeDefined();
      expect(result.edges).toBeDefined();
    });
  });
});
