/**
 * Navigation API Route - Faceted graph query via Rust bridge
 *
 * Resolves realm/trait/layer/arcFamily facets to a Cypher query using the
 * `novanet filter build` Rust binary (meta-graph aware), then executes
 * against Neo4j. Same response format as /api/graph.
 *
 * @example GET /api/graph/navigation?realms=global,project&traits=localized&layers=knowledge
 */

import { NextRequest, NextResponse } from 'next/server';
import { executeQuery } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';
import { buildCypherViaRust, type FacetFilter } from '@/lib/novanetBridge';
import type { Realm, Layer, Trait } from '@novanet/core/types';

const VALID_REALMS: Realm[] = ['global', 'project'];
const VALID_LAYERS: Layer[] = [
  'config', 'knowledge', 'foundation', 'structure', 'semantic',
  'instruction', 'output', 'seo',
];
const VALID_TRAITS: Trait[] = ['invariant', 'localized', 'knowledge', 'derived', 'job'];
const VALID_ARC_FAMILIES = ['ownership', 'localization', 'semantic', 'generation', 'mining'];

function parseCSV<T extends string>(param: string | null, valid: T[]): T[] {
  if (!param) return [];
  return param.split(',').filter((v): v is T => valid.includes(v as T));
}

export async function GET(request: NextRequest) {
  const startTime = Date.now();

  try {
    const searchParams = request.nextUrl.searchParams;

    const realms = parseCSV(searchParams.get('realms'), VALID_REALMS);
    const layers = parseCSV(searchParams.get('layers'), VALID_LAYERS);
    const traits = parseCSV(searchParams.get('traits'), VALID_TRAITS);
    const arcFamilies = parseCSV(searchParams.get('arcFamilies'), VALID_ARC_FAMILIES);
    const rawLimit = parseInt(searchParams.get('limit') || '500', 10);
    const limit = Math.min(Math.max(1, isNaN(rawLimit) ? 500 : rawLimit), 5000);

    // Build Cypher via Rust binary (meta-graph aware resolution)
    const facets: FacetFilter = {
      realms,
      layers,
      traits,
      arc_families: arcFamilies,
      kinds: [],
    };

    const cypher = await buildCypherViaRust(facets);

    // Execute generated Cypher + apply limit
    const cypherWithLimit = `${cypher}\nLIMIT ${limit}`;
    const result = await executeQuery(cypherWithLimit, {}, { timeout: 30_000 });

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.totalNodes,
        totalArcs: result.totalArcs,
        duration: result.duration,
        requestDuration: Date.now() - startTime,
        mode: 'query',
        facets: { realms, layers, traits, arcFamilies },
        generatedCypher: cypher,
      },
    });
  } catch (error) {
    return handleApiError(error, '/graph/navigation GET');
  }
}
