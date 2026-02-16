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
    // v0.13.0: 2 realms (shared, org), 10 layers (4 shared + 6 org)
    mockHierarchy = {
      realms: {
        shared: {
          realm: 'shared' as Realm,
          label: 'SHARED',
          icon: '🌍',
          description: 'Universal knowledge',
          layers: {
            locale: {
              label: 'Locale',
              description: 'Locale configuration',
              icon: '🌍',
              nodeTypes: ['Locale', 'Formatting', 'Style'] as never[],
            },
            geography: {
              label: 'Geography',
              description: 'Geographic classifications',
              icon: '🗺️',
              nodeTypes: ['Continent', 'GeoRegion'] as never[],
            },
            knowledge: {
              label: 'Knowledge',
              description: 'Knowledge sets and atoms',
              icon: '🧠',
              nodeTypes: ['TermSet', 'Term'] as never[],
            },
          } as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
        org: {
          realm: 'org' as Realm,
          label: 'ORG',
          icon: '🏢',
          description: 'Organization-specific content',
          layers: {
            config: {
              label: 'Configuration',
              description: 'Org config',
              icon: '⚙️',
              nodeTypes: ['OrgConfig'] as never[],
            },
            foundation: {
              label: 'Foundation',
              description: 'Core project identity',
              icon: '🏛️',
              // v0.12.4: BrandIdentity → Brand + BrandDesign + BrandPrinciples + PromptStyle
              nodeTypes: ['Project', 'Brand', 'BrandDesign', 'BrandPrinciples', 'PromptStyle', 'ProjectNative'] as never[],
            },
            structure: {
              label: 'Structure',
              description: 'Page and block organization',
              icon: '🧱',
              nodeTypes: ['Page', 'Block'] as never[],
            },
          } as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
        },
      } as Record<Realm, typeof mockHierarchy.realms.org>,
      // v0.12.0: trait renames per ADR-024 Data Origin
      nodes: [
        // Shared realm - locale (3)
        { id: 'schema-Locale', nodeType: 'Locale', realm: 'shared', layer: 'locale', label: 'Locale', description: '', trait: 'defined' },
        { id: 'schema-Formatting', nodeType: 'Formatting', realm: 'shared', layer: 'locale', label: 'Formatting', description: '', trait: 'imported' },
        { id: 'schema-Style', nodeType: 'Style', realm: 'shared', layer: 'locale', label: 'Style', description: '', trait: 'imported' },
        // Shared realm - geography (2)
        { id: 'schema-Continent', nodeType: 'Continent', realm: 'shared', layer: 'geography', label: 'Continent', description: '', trait: 'defined' },
        { id: 'schema-GeoRegion', nodeType: 'GeoRegion', realm: 'shared', layer: 'geography', label: 'Geo Region', description: '', trait: 'defined' },
        // Shared realm - knowledge (2)
        { id: 'schema-TermSet', nodeType: 'TermSet', realm: 'shared', layer: 'knowledge', label: 'Term Set', description: '', trait: 'defined' },
        { id: 'schema-Term', nodeType: 'Term', realm: 'shared', layer: 'knowledge', label: 'Term', description: '', trait: 'imported' },
        // Org realm - config (1)
        { id: 'schema-OrgConfig', nodeType: 'OrgConfig', realm: 'org', layer: 'config', label: 'Org Config', description: '', trait: 'defined' },
        // Org realm - foundation (6) — v0.12.4: Brand Architecture (ADR-028)
        { id: 'schema-Project', nodeType: 'Project', realm: 'org', layer: 'foundation', label: 'Project', description: '', trait: 'defined' },
        { id: 'schema-Brand', nodeType: 'Brand', realm: 'org', layer: 'foundation', label: 'Brand', description: '', trait: 'defined' },
        { id: 'schema-BrandDesign', nodeType: 'BrandDesign', realm: 'org', layer: 'foundation', label: 'Brand Design', description: '', trait: 'defined' },
        { id: 'schema-BrandPrinciples', nodeType: 'BrandPrinciples', realm: 'org', layer: 'foundation', label: 'Brand Principles', description: '', trait: 'defined' },
        { id: 'schema-PromptStyle', nodeType: 'PromptStyle', realm: 'org', layer: 'foundation', label: 'Prompt Style', description: '', trait: 'defined' },
        { id: 'schema-ProjectNative', nodeType: 'ProjectNative', realm: 'org', layer: 'foundation', label: 'Project Native', description: '', trait: 'authored' },
        // Org realm - structure (2)
        { id: 'schema-Page', nodeType: 'Page', realm: 'org', layer: 'structure', label: 'Page', description: '', trait: 'defined' },
        { id: 'schema-Block', nodeType: 'Block', realm: 'org', layer: 'structure', label: 'Block', description: '', trait: 'defined' },
      ] as SchemaNode[],
      arcs: [
        { id: 'schema-arc-0', relationType: 'HAS_PAGE', sourceType: 'Project', targetType: 'Page', label: 'HAS_PAGE', description: '', cardinality: '1:N' },
        { id: 'schema-arc-1', relationType: 'HAS_BLOCK', sourceType: 'Page', targetType: 'Block', label: 'HAS_BLOCK', description: '', cardinality: '1:N' },
      ] as SchemaArc[],
      stats: {
        totalNodes: 16,  // v0.12.4: +3 Brand Architecture nodes (ADR-028)
        totalArcs: 2,
        nodesByRealm: { shared: 7, org: 9 },
      },
    };
  });

  describe('applySchemaLayout', () => {
    // v9.5: Layout changed from ELK containers to Dagre hierarchical graph
    // - Realm and Layer are now schemaBadge nodes (not container groups)
    // - No parent/child relationships - flat graph with edges
    // - HAS_LAYER and HAS_CLASS edges connect the hierarchy (v11.8 ADR-023)

    it('should layout schema nodes with Dagre', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // Should have schema badge nodes + schema class nodes
      // v0.12.4: 2 realm badges + 6 layer badges + 16 schema nodes = 24 (Brand Architecture)
      expect(result.nodes.length).toBeGreaterThan(16);

      // All nodes should have positions
      for (const node of result.nodes) {
        expect(node.position).toBeDefined();
        expect(typeof node.position.x).toBe('number');
        expect(typeof node.position.y).toBe('number');
      }
    });

    it('should create realm schema badge nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // v11.3: Realms are schemaBadge nodes with metaType: 'realm' (2 realms: shared, org)
      const realmBadges = result.nodes.filter(n =>
        n.type === 'schemaBadge' && n.data.metaType === 'realm'
      );
      expect(realmBadges).toHaveLength(2);

      // Verify realm badge data
      const orgRealm = realmBadges.find(n => n.data.realmKey === 'org');
      expect(orgRealm).toBeDefined();
      expect(orgRealm?.data.label).toBe('Org');
    });

    it('should create layer schema badge nodes', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      // v11.3: Layers are schemaBadge nodes with metaType: 'layer'
      const layerBadges = result.nodes.filter(n =>
        n.type === 'schemaBadge' && n.data.metaType === 'layer'
      );
      // v11.3: 3 shared (locale, geography, knowledge) + 3 org (config, foundation, structure) = 6
      expect(layerBadges).toHaveLength(6);

      // v11.3: No parent relationships - connected by HAS_LAYER edges
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      expect(hasLayerEdges.length).toBe(6);
    });

    it('should create schema nodes with layer connections', async () => {
      const result = await applySchemaLayout(mockHierarchy);

      const schemaNodes = result.nodes.filter(n => n.type === 'schemaNode');
      expect(schemaNodes).toHaveLength(16);  // v0.12.4: +3 Brand Architecture nodes

      // v11.8 ADR-023: Connected by HAS_CLASS edges (not parent relationships)
      const hasClassEdges = result.edges.filter(e => e.data?.relationType === 'HAS_CLASS');
      expect(hasClassEdges.length).toBe(16);  // v0.12.4: +3 Brand Architecture nodes
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

      // v11.8 ADR-023: Total edges = HAS_LAYER + HAS_CLASS + business edges
      // v0.12.4: 6 HAS_LAYER + 16 HAS_CLASS + 2 business = 24 (Brand Architecture added)
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      const hasClassEdges = result.edges.filter(e => e.data?.relationType === 'HAS_CLASS');
      const businessEdges = result.edges.filter(e => !e.data?.isMetaEdge);

      expect(hasLayerEdges.length).toBe(6);
      expect(hasClassEdges.length).toBe(16);  // v0.12.4: +3 Brand Architecture nodes
      expect(businessEdges.length).toBe(2); // Original mock edges
    });

    it('should skip empty layers (P1 fix)', async () => {
      // Add an empty layer to org realm
      const hierarchyWithEmpty: HierarchicalSchemaData = {
        ...mockHierarchy,
        realms: {
          ...mockHierarchy.realms,
          org: {
            ...mockHierarchy.realms.org,
            layers: {
              ...mockHierarchy.realms.org.layers,
              empty: {
                label: 'Empty',
                description: 'Empty layer',
                icon: '❌',
                nodeTypes: [], // No nodes!
              },
            } as unknown as Record<Layer, { label: string; description: string; icon: string; nodeTypes: never[] }>,
          },
        } as Record<Realm, typeof mockHierarchy.realms.org>,
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

      // v11.8: Uses schemaBadge for Realm and Layer, schemaNode for Class
      // Should have 2 realm schema badges (shared, org)
      const realmBadges = result.nodes.filter(n =>
        n.type === 'schemaBadge' && n.data.metaType === 'realm'
      );
      expect(realmBadges).toHaveLength(2);

      // Should have layer schema badges (varies by active layers)
      const layerBadges = result.nodes.filter(n =>
        n.type === 'schemaBadge' && n.data.metaType === 'layer'
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

      // v11.8 ADR-023: Has HAS_LAYER, HAS_CLASS, and business edges
      const hasLayerEdges = result.edges.filter(e => e.data?.relationType === 'HAS_LAYER');
      const hasClassEdges = result.edges.filter(e => e.data?.relationType === 'HAS_CLASS');
      const businessEdges = result.edges.filter(e => !e.data?.isMetaEdge);

      expect(hasLayerEdges.length).toBeGreaterThan(0);
      expect(hasClassEdges.length).toBeGreaterThan(0);
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
        stats: { totalNodes: 0, totalArcs: 0, nodesByRealm: { shared: 0, org: 0 } },
      };

      // Should not throw
      const result = await applySchemaLayout(emptyHierarchy);

      // Empty input = empty output
      expect(result.nodes).toBeDefined();
      expect(result.edges).toBeDefined();
    });
  });
});
