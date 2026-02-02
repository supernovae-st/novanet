/**
 * Navigation API Route - Faceted graph query
 *
 * Resolves realm/trait/layer/edgeFamily facets to a Cypher query and executes
 * against Neo4j. Same response format as /api/graph.
 *
 * @example GET /api/graph/navigation?realms=global,project&traits=localized&layers=knowledge
 */

import { NextRequest, NextResponse } from 'next/server';
import { fetchGraphData, type QueryOptions } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';
import {
  resolveTypesForRealms,
  resolveTypesForTraits,
} from '@/lib/filterAdapter';
import { NODE_LAYERS } from '@/config/nodeTypes';
import type { Realm, Layer, Trait, NodeType } from '@novanet/core/types';

const VALID_REALMS: Realm[] = ['global', 'project', 'shared'];
const VALID_LAYERS: Layer[] = [
  'config', 'knowledge', 'foundation', 'structure', 'semantic',
  'instruction', 'output', 'seo', 'geo',
];
const VALID_TRAITS: Trait[] = ['invariant', 'localized', 'knowledge', 'derived', 'job'];

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
    const edgeFamilies = searchParams.get('edgeFamilies')?.split(',').filter(Boolean) ?? [];
    const rawLimit = parseInt(searchParams.get('limit') || '500', 10);
    const limit = Math.min(Math.max(1, isNaN(rawLimit) ? 500 : rawLimit), 5000);

    // Resolve facets to node types for the standard fetchGraphData path
    // This avoids raw Cypher execution and reuses the existing Neo4j query pipeline
    const realmTypes = resolveTypesForRealms(realms);
    const traitTypes = resolveTypesForTraits(traits);
    const layerTypes: NodeType[] = [];
    for (const layer of layers) {
      const types = NODE_LAYERS[layer];
      if (types) types.forEach((t) => layerTypes.push(t));
    }

    // Intersect non-empty sets
    const sets = [realmTypes, traitTypes, layerTypes].filter((s) => s.length > 0);
    let resolvedTypes: NodeType[];

    if (sets.length === 0) {
      resolvedTypes = []; // Empty = all types (fetchGraphData default)
    } else if (sets.length === 1) {
      resolvedTypes = sets[0];
    } else {
      const first = new Set(sets[0]);
      for (let i = 1; i < sets.length; i++) {
        const current = new Set(sets[i]);
        for (const t of first) {
          if (!current.has(t)) first.delete(t);
        }
      }
      resolvedTypes = [...first];
    }

    // Use the standard fetchGraphData pipeline with resolved types
    const options: QueryOptions = {
      nodeTypes: resolvedTypes,
      limit,
    };

    const result = await fetchGraphData(options);

    return NextResponse.json({
      success: true,
      data: {
        nodes: result.nodes,
        edges: result.edges,
      },
      meta: {
        totalNodes: result.totalNodes,
        totalEdges: result.totalEdges,
        duration: result.duration,
        requestDuration: Date.now() - startTime,
        mode: 'query',
        facets: { realms, layers, traits, edgeFamilies },
        resolvedTypes,
      },
    });
  } catch (error) {
    return handleApiError(error, '/graph/navigation GET');
  }
}
