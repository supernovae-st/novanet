/**
 * Database Schema API Route
 *
 * Returns Neo4j database schema information: node labels, relationship types,
 * counts, and property keys. Used by DatabaseInfoPanel.
 *
 * @example GET /api/graph/schema
 */

import { NextResponse } from 'next/server';
import { getDriver } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';

interface NodeLabel {
  label: string;
  count: number;
}

interface RelationType {
  type: string;
  count: number;
}

interface SchemaData {
  nodeLabels: NodeLabel[];
  relationshipTypes: RelationType[];
  propertyKeys: string[];
  totalNodes: number;
  totalRelationships: number;
}

// Helper to extract number from Neo4j Integer
function toNumber(value: unknown): number {
  if (value === null || value === undefined) return 0;
  if (typeof value === 'number') return value;
  if (typeof value === 'object' && 'low' in (value as object)) {
    return (value as { low: number }).low;
  }
  return 0;
}

// =============================================================================
// GET /api/graph/schema - Fetch database schema from Neo4j
// =============================================================================

export async function GET() {
  const driver = getDriver();
  const session = driver.session();

  try {
    // Get node labels
    const labelsResult = await session.run('CALL db.labels() YIELD label RETURN label');
    const labels = labelsResult.records.map((r) => r.get('label') as string);

    // Get counts for each label (separate queries to avoid APOC dependency)
    const nodeLabels: NodeLabel[] = [];
    for (const label of labels) {
      const countResult = await session.run(
        `MATCH (n:\`${label}\`) RETURN count(n) as count`
      );
      const count = toNumber(countResult.records[0]?.get('count'));
      nodeLabels.push({ label, count });
    }
    // Sort by count descending
    nodeLabels.sort((a, b) => b.count - a.count);

    // Get relationship types
    const relTypesResult = await session.run('CALL db.relationshipTypes() YIELD relationshipType RETURN relationshipType');
    const relTypes = relTypesResult.records.map((r) => r.get('relationshipType') as string);

    // Get counts for each relationship type
    const relationshipTypes: RelationType[] = [];
    for (const type of relTypes) {
      const countResult = await session.run(
        `MATCH ()-[r:\`${type}\`]->() RETURN count(r) as count`
      );
      const count = toNumber(countResult.records[0]?.get('count'));
      relationshipTypes.push({ type, count });
    }
    // Sort by count descending
    relationshipTypes.sort((a, b) => b.count - a.count);

    // Get all property keys
    const propsResult = await session.run('CALL db.propertyKeys() YIELD propertyKey RETURN propertyKey');
    const propertyKeys: string[] = propsResult.records.map((r) => r.get('propertyKey') as string);

    // Calculate totals
    const totalNodes = nodeLabels.reduce((sum, label) => sum + label.count, 0);
    const totalRelationships = relationshipTypes.reduce((sum, rel) => sum + rel.count, 0);

    const schemaData: SchemaData = {
      nodeLabels,
      relationshipTypes,
      propertyKeys,
      totalNodes,
      totalRelationships,
    };

    return NextResponse.json({
      success: true,
      data: schemaData,
    });
  } catch (error) {
    return handleApiError(error, '/graph/schema GET');
  } finally {
    await session.close();
  }
}
