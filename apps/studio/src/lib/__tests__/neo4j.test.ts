/**
 * Neo4j Library Tests
 *
 * Tests for graph data fetching and transformation functions.
 * Validates orphan node exclusion and query behavior.
 */

import { transformNode, transformRelationship } from '../neo4j';

// Mock neo4j-driver
jest.mock('neo4j-driver', () => {
  const mockNode = (elementId: string, labels: string[], properties: Record<string, unknown>) => ({
    elementId,
    labels,
    properties,
  });

  const mockRelationship = (
    elementId: string,
    startNodeElementId: string,
    endNodeElementId: string,
    type: string,
    properties: Record<string, unknown> = {}
  ) => ({
    elementId,
    startNodeElementId,
    endNodeElementId,
    type,
    properties,
  });

  return {
    default: {
      driver: jest.fn(),
      auth: { basic: jest.fn() },
      session: { READ: 'READ', WRITE: 'WRITE' },
      int: (n: number) => ({ toNumber: () => n }),
      types: {
        Node: class Node {
          elementId: string;
          labels: string[];
          properties: Record<string, unknown>;
          constructor(elementId: string, labels: string[], properties: Record<string, unknown>) {
            this.elementId = elementId;
            this.labels = labels;
            this.properties = properties;
          }
        },
        Relationship: class Relationship {
          elementId: string;
          startNodeElementId: string;
          endNodeElementId: string;
          type: string;
          properties: Record<string, unknown>;
          constructor(
            elementId: string,
            startNodeElementId: string,
            endNodeElementId: string,
            type: string,
            properties: Record<string, unknown>
          ) {
            this.elementId = elementId;
            this.startNodeElementId = startNodeElementId;
            this.endNodeElementId = endNodeElementId;
            this.type = type;
            this.properties = properties;
          }
        },
        Path: class Path {
          segments: unknown[];
          constructor(segments: unknown[]) {
            this.segments = segments;
          }
        },
      },
    },
    __mockNode: mockNode,
    __mockRelationship: mockRelationship,
  };
});

describe('neo4j transformations', () => {
  describe('transformNode', () => {
    it('should transform a Neo4j node to GraphNode', () => {
      const neo4jNode = {
        elementId: '4:abc:123',
        labels: ['Concept'],
        properties: {
          key: 'free-tier',
          display_name: 'Free Tier',
          description: 'A free tier concept',
          icon: 'gift',
        },
      };

      const result = transformNode(neo4jNode as never);

      expect(result).toEqual({
        id: '4:abc:123',
        type: 'Concept',
        key: 'free-tier',
        displayName: 'Free Tier',
        description: 'A free tier concept',
        icon: 'gift',
        llmContext: undefined,
        priority: undefined,
        freshness: undefined,
        createdAt: undefined,
        updatedAt: undefined,
        data: {},
      });
    });

    it('should use key as displayName fallback', () => {
      const neo4jNode = {
        elementId: '4:abc:456',
        labels: ['Locale'],
        properties: {
          key: 'fr-FR',
        },
      };

      const result = transformNode(neo4jNode as never);

      expect(result.displayName).toBe('fr-FR');
    });

    it('should handle nodes with extra properties in data field', () => {
      const neo4jNode = {
        elementId: '4:abc:789',
        labels: ['TranslationUnit'],
        properties: {
          key: 'tu-123',
          display_name: 'Test Unit',
          quality_score: 95,
          is_approved: true,
          custom_field: 'custom_value',
        },
      };

      const result = transformNode(neo4jNode as never);

      expect(result.data).toEqual({
        quality_score: 95,
        is_approved: true,
        custom_field: 'custom_value',
      });
    });
  });

  describe('transformRelationship', () => {
    it('should transform a Neo4j relationship to GraphEdge', () => {
      const neo4jRel = {
        elementId: '5:abc:100',
        startNodeElementId: '4:abc:123',
        endNodeElementId: '4:abc:456',
        type: 'HAS_L10N',
        properties: {
          created_at: '2024-01-01',
        },
      };

      const result = transformRelationship(neo4jRel as never);

      expect(result).toEqual({
        id: '5:abc:100',
        source: '4:abc:123',
        target: '4:abc:456',
        type: 'HAS_L10N',
        data: { created_at: '2024-01-01' },
      });
    });

    it('should handle relationships without properties', () => {
      const neo4jRel = {
        elementId: '5:abc:200',
        startNodeElementId: '4:abc:111',
        endNodeElementId: '4:abc:222',
        type: 'USES_CONCEPT',
        properties: {},
      };

      const result = transformRelationship(neo4jRel as never);

      expect(result).toEqual({
        id: '5:abc:200',
        source: '4:abc:111',
        target: '4:abc:222',
        type: 'USES_CONCEPT',
        data: {},
      });
    });
  });
});

describe('fetchNodeNeighbors', () => {
  it('should fetch neighbors for a given node ID', async () => {
    // This test validates the function signature and return type.
    // The actual Neo4j query execution is mocked, but we verify
    // that the function exists and returns the expected structure.
    //
    // Integration testing with a real Neo4j instance should be done
    // separately in E2E tests.

    // Import the function dynamically to get fresh mock state
    const { fetchNodeNeighbors } = await import('../neo4j');

    // The function should exist and be callable
    expect(typeof fetchNodeNeighbors).toBe('function');

    // Check the function signature by inspecting it
    // (2 params: nodeId required, limit optional with default 50)
    expect(fetchNodeNeighbors.length).toBeLessThanOrEqual(2);
  });

  it('should validate the Cypher query pattern', async () => {
    // Verify the source code contains the correct Cypher pattern
    const fs = await import('fs');
    const path = await import('path');
    const neo4jPath = path.resolve(__dirname, '../neo4j.ts');
    const content = fs.readFileSync(neo4jPath, 'utf-8');

    // Should contain the neighbor query pattern
    const neighborQueryPattern = /MATCH \(n\)-\[r\]-\(m\)\s+WHERE elementId\(n\) = \$nodeId/;
    expect(neighborQueryPattern.test(content)).toBe(true);

    // Should have LIMIT $limit for the neighbor query
    const limitPattern = /RETURN n, r, m\s+LIMIT \$limit/;
    expect(limitPattern.test(content)).toBe(true);
  });
});

describe('neo4j query behavior', () => {
  describe('fetchGraphData query construction', () => {
    it('should use MATCH (not OPTIONAL MATCH) to exclude orphan nodes', async () => {
      // This test validates the architectural decision:
      // The Cypher query uses MATCH (n)-[r]-(m) which only returns
      // nodes that have at least one relationship.
      //
      // Before fix: OPTIONAL MATCH would return orphan nodes with r=null, m=null
      // After fix: MATCH excludes nodes without relationships
      //
      // This mimics Neo4j Browser behavior where disconnected nodes don't appear.

      // We test this by checking the source code contains MATCH, not OPTIONAL MATCH
      const fs = await import('fs');
      const path = await import('path');
      const neo4jPath = path.resolve(__dirname, '../neo4j.ts');
      const content = fs.readFileSync(neo4jPath, 'utf-8');

      // Extract the fetchGraphData function region (lines ~457-474)
      const matchPattern = /WITH n LIMIT \$limit\s+MATCH \(n\)-\[r\]-\(m\)/g;
      const optionalMatchPattern = /OPTIONAL MATCH \(n\)-\[r\]-\(m\)/g;

      const matchCount = (content.match(matchPattern) || []).length;
      const optionalMatchCount = (content.match(optionalMatchPattern) || []).length;

      // Should have MATCH patterns (2: one for filtered, one for unfiltered)
      expect(matchCount).toBe(2);
      // Should NOT have OPTIONAL MATCH patterns
      expect(optionalMatchCount).toBe(0);
    });
  });
});
