/**
 * Organizing Principles API Route
 *
 * Returns Scope and Subcategory nodes with their relationships
 * for the magnetic layout visualization.
 *
 * @example GET /api/graph/organizing-principles
 */

import { NextResponse } from 'next/server';
import { getDriver } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';

interface Subcategory {
  key: string;
  displayName: string;
  emoji: string;
  llmContext: string | null;
  nodeTypes: string[];
}

interface Scope {
  key: string;
  displayName: string;
  emoji: string;
  color: string;
  llmContext: string | null;
  subcategories: Subcategory[];
}

// =============================================================================
// GET /api/graph/organizing-principles - Fetch scopes and subcategories
// =============================================================================

export async function GET() {
  const driver = getDriver();
  const session = driver.session();

  try {
    const result = await session.run(`
      MATCH (s:Scope)-[:HAS_SUBCATEGORY]->(sub:Subcategory)
      OPTIONAL MATCH (sub)-[:DEFINES_TYPE]->(ntm:NodeTypeMeta)
      RETURN
        s.key AS scopeKey,
        s.display_name AS scopeDisplayName,
        s.emoji AS scopeEmoji,
        s.color AS scopeColor,
        s.llm_context AS scopeLlmContext,
        collect(DISTINCT {
          key: sub.key,
          displayName: sub.display_name,
          emoji: sub.emoji,
          llmContext: sub.llm_context,
          nodeTypes: collect(ntm.label)
        }) AS subcategories
      ORDER BY s.key
    `);

    const scopes: Scope[] = result.records.map((record) => ({
      key: record.get('scopeKey') as string,
      displayName: record.get('scopeDisplayName') as string,
      emoji: record.get('scopeEmoji') as string,
      color: record.get('scopeColor') as string,
      llmContext: record.get('scopeLlmContext') as string | null,
      subcategories: record.get('subcategories') as Subcategory[],
    }));

    return NextResponse.json({
      success: true,
      data: { scopes },
    });
  } catch (error) {
    return handleApiError(error, '/graph/organizing-principles GET');
  } finally {
    await session.close();
  }
}
