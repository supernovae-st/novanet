/**
 * Organizing Principles API Route
 *
 * Returns Realm, Layer nodes, and Class→Layer mapping
 * from Neo4j. All display metadata (emoji, color) comes from the database,
 * which is seeded from organizing-principles.yaml (the source of truth).
 *
 * @example GET /api/graph/organizing-principles
 */

import { NextResponse } from 'next/server';
import { getDriver } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';

// =============================================================================
// GET /api/graph/organizing-principles
// =============================================================================

export async function GET() {
  const driver = getDriver();
  const session = driver.session();

  try {
    // Fetch realms with full display metadata
    const realmResult = await session.run(`
      MATCH (r:Realm)
      RETURN
        r.key AS key,
        r.display_name AS display_name,
        r.emoji AS emoji,
        r.color AS color,
        r.content AS content
      ORDER BY r.key
    `);

    const realms = realmResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      emoji: record.get('emoji') as string,
      color: record.get('color') as string,
      content: record.get('content') as string | null,
    }));

    // Fetch layers with full display metadata
    const layerResult = await session.run(`
      MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)
      RETURN
        l.key AS key,
        l.display_name AS display_name,
        l.emoji AS emoji,
        l.content AS content,
        r.key AS realm_key
      ORDER BY r.key, l.key
    `);

    const layers = layerResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      emoji: record.get('emoji') as string,
      content: record.get('content') as string | null,
      realm_key: record.get('realm_key') as string,
    }));

    // Fetch Class → Layer mapping from HAS_CLASS relationships (v11.8 ADR-023)
    const mappingResult = await session.run(`
      MATCH (l:Layer)-[:HAS_CLASS]->(c:Schema:Class)
      RETURN c.label AS node_type, l.key AS layer
      ORDER BY c.label
    `);

    const nodeTypeMapping: Record<string, string> = {};
    for (const record of mappingResult.records) {
      nodeTypeMapping[record.get('node_type') as string] = record.get('layer') as string;
    }

    return NextResponse.json({
      success: true,
      data: { realms, layers, nodeTypeMapping },
    });
  } catch (error) {
    return handleApiError(error, '/graph/organizing-principles GET');
  } finally {
    await session.close();
  }
}
