/**
 * Organizing Principles API Route
 *
 * Returns Scope, Subcategory nodes, and nodeType→subcategory mapping
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
    // Fetch scopes with full display metadata
    const scopeResult = await session.run(`
      MATCH (s:Scope)
      RETURN
        s.key AS key,
        s.display_name AS display_name,
        s.emoji AS emoji,
        s.color AS color,
        s.llm_context AS llm_context
      ORDER BY s.key
    `);

    const scopes = scopeResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      emoji: record.get('emoji') as string,
      color: record.get('color') as string,
      llm_context: record.get('llm_context') as string | null,
    }));

    // Fetch subcategories with full display metadata
    const subResult = await session.run(`
      MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)
      RETURN
        sub.key AS key,
        sub.display_name AS display_name,
        sub.emoji AS emoji,
        sub.llm_context AS llm_context,
        s.key AS scope_key
      ORDER BY s.key, sub.key
    `);

    const subcategories = subResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      emoji: record.get('emoji') as string,
      llm_context: record.get('llm_context') as string | null,
      scope_key: record.get('scope_key') as string,
    }));

    // TODO(v9): Rename Subcategory->Layer, NodeTypeMeta->Kind, DEFINES_TYPE->HAS_KIND
    // TODO(v9): Response shape: { realms, layers, nodeTypeMapping } (not scopes/subcategories)
    // Fetch nodeType → subcategory mapping from DEFINES_TYPE relationships
    const mappingResult = await session.run(`
      MATCH (sub:Subcategory)-[:DEFINES_TYPE]->(ntm:NodeTypeMeta)
      RETURN ntm.label AS node_type, sub.key AS subcategory
      ORDER BY ntm.label
    `);

    const nodeTypeMapping: Record<string, string> = {};
    for (const record of mappingResult.records) {
      nodeTypeMapping[record.get('node_type') as string] = record.get('subcategory') as string;
    }

    return NextResponse.json({
      success: true,
      data: { scopes, subcategories, nodeTypeMapping },
    });
  } catch (error) {
    return handleApiError(error, '/graph/organizing-principles GET');
  } finally {
    await session.close();
  }
}
