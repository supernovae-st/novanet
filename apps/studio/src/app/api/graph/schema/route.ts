/**
 * Graph Schema API Route
 *
 * Returns database schema information:
 * - Node labels with counts
 * - Relationship types with counts
 * - Property keys
 *
 * Similar to Neo4j Browser's "Database Information" panel.
 *
 * @example GET /api/graph/schema
 */

import { NextResponse } from 'next/server';
import { getDriver, verifyConnection } from '@/lib/neo4j';
import { logger } from '@/lib/logger';

export interface SchemaResponse {
  success: boolean;
  data?: {
    nodeLabels: { label: string; count: number }[];
    relationshipTypes: { type: string; count: number }[];
    propertyKeys: string[];
    totalNodes: number;
    totalRelationships: number;
  };
  meta?: {
    lastUpdate: string;
    requestDuration: number;
  };
  error?: string;
}

export async function GET() {
  const startTime = Date.now();

  try {
    // Verify connection first
    const isConnected = await verifyConnection();
    if (!isConnected) {
      return NextResponse.json(
        {
          success: false,
          error: 'Unable to connect to Neo4j database',
        } as SchemaResponse,
        { status: 503 }
      );
    }

    const driver = getDriver();
    const session = driver.session();

    try {
      // Fetch node labels with counts (sorted alphabetically like Neo4j Browser)
      const labelsResult = await session.run(`
        CALL db.labels() YIELD label
        CALL {
          WITH label
          MATCH (n)
          WHERE label IN labels(n)
          RETURN count(n) AS count
        }
        RETURN label, count
        ORDER BY label ASC
      `);

      const nodeLabels = labelsResult.records.map((record) => ({
        label: record.get('label') as string,
        count: (record.get('count') as { toNumber: () => number }).toNumber(),
      }));

      // Fetch relationship types with counts (sorted alphabetically like Neo4j Browser)
      const relTypesResult = await session.run(`
        CALL db.relationshipTypes() YIELD relationshipType
        CALL {
          WITH relationshipType
          MATCH ()-[r]->()
          WHERE type(r) = relationshipType
          RETURN count(r) AS count
        }
        RETURN relationshipType, count
        ORDER BY relationshipType ASC
      `);

      const relationshipTypes = relTypesResult.records.map((record) => ({
        type: record.get('relationshipType') as string,
        count: (record.get('count') as { toNumber: () => number }).toNumber(),
      }));

      // Fetch property keys
      const propsResult = await session.run(`
        CALL db.propertyKeys() YIELD propertyKey
        RETURN propertyKey
        ORDER BY propertyKey
      `);

      const propertyKeys = propsResult.records.map(
        (record) => record.get('propertyKey') as string
      );

      // Calculate totals
      const totalNodes = nodeLabels.reduce((sum, item) => sum + item.count, 0);
      const totalRelationships = relationshipTypes.reduce(
        (sum, item) => sum + item.count,
        0
      );

      return NextResponse.json({
        success: true,
        data: {
          nodeLabels,
          relationshipTypes,
          propertyKeys,
          totalNodes,
          totalRelationships,
        },
        meta: {
          lastUpdate: new Date().toISOString(),
          requestDuration: Date.now() - startTime,
        },
      } as SchemaResponse);
    } finally {
      await session.close();
    }
  } catch (error) {
    logger.error('API', '/graph/schema error', error);

    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      } as SchemaResponse,
      { status: 500 }
    );
  }
}
