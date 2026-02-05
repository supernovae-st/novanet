/**
 * Taxonomy API Route (v9.5)
 *
 * Returns complete taxonomy data from Neo4j including:
 * - Realms with nested layers
 * - Traits with visual encoding (border styles)
 * - Arc families with visual encoding (stroke colors/styles)
 * - Arc scopes and cardinalities
 *
 * The database is seeded from taxonomy.yaml (the source of truth).
 *
 * @example GET /api/graph/taxonomy
 */

import { NextResponse } from 'next/server';
import { getDriver } from '@/lib/neo4j';
import { handleApiError } from '@/lib/apiErrorHandler';

// =============================================================================
// Types
// =============================================================================

export interface TaxonomyRealm {
  key: string;
  display_name: string;
  emoji: string;
  color: string;
  llm_context: string | null;
  layers: TaxonomyLayer[];
}

export interface TaxonomyLayer {
  key: string;
  display_name: string;
  emoji: string;
  color: string;
  llm_context: string | null;
}

export interface TaxonomyTrait {
  key: string;
  display_name: string;
  color: string;
  border_style: string | null;
  border_width: number | null;
  unicode_border: string | null;
  llm_context: string | null;
}

export interface TaxonomyArcFamily {
  key: string;
  display_name: string;
  color: string;
  stroke_style: string | null;
  stroke_width: number | null;
  arrow_style: string;
  llm_context: string | null;
}

export interface TaxonomyResponse {
  version: string;
  realms: TaxonomyRealm[];
  traits: TaxonomyTrait[];
  arcFamilies: TaxonomyArcFamily[];
  nodeTypeMapping: Record<string, string>;
}

// =============================================================================
// GET /api/graph/taxonomy
// =============================================================================

export async function GET() {
  const driver = getDriver();
  const session = driver.session();

  try {
    // Fetch realms
    const realmResult = await session.run(`
      MATCH (r:Realm)
      RETURN
        r.key AS key,
        r.display_name AS display_name,
        r.emoji AS emoji,
        r.color AS color,
        r.llm_context AS llm_context
      ORDER BY
        CASE r.key
          // v10.6: 2 realms (global + tenant)
          WHEN 'global' THEN 1
          WHEN 'tenant' THEN 2
          ELSE 3
        END
    `);

    // Fetch layers with realm
    const layerResult = await session.run(`
      MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)
      RETURN
        l.key AS key,
        l.display_name AS display_name,
        l.emoji AS emoji,
        l.color AS color,
        l.llm_context AS llm_context,
        r.key AS realm_key
      ORDER BY r.key, l.key
    `);

    // Build realm objects with nested layers
    const layersByRealm: Record<string, TaxonomyLayer[]> = {};
    for (const record of layerResult.records) {
      const realmKey = record.get('realm_key') as string;
      if (!layersByRealm[realmKey]) {
        layersByRealm[realmKey] = [];
      }
      layersByRealm[realmKey].push({
        key: record.get('key') as string,
        display_name: record.get('display_name') as string,
        emoji: record.get('emoji') as string,
        color: record.get('color') as string || '#64748b',
        llm_context: record.get('llm_context') as string | null,
      });
    }

    const realms: TaxonomyRealm[] = realmResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      emoji: record.get('emoji') as string,
      color: record.get('color') as string,
      llm_context: record.get('llm_context') as string | null,
      layers: layersByRealm[record.get('key') as string] || [],
    }));

    // Fetch traits with visual encoding
    const traitResult = await session.run(`
      MATCH (t:Trait)
      RETURN
        t.key AS key,
        t.display_name AS display_name,
        t.color AS color,
        t.border_style AS border_style,
        t.border_width AS border_width,
        t.unicode_border AS unicode_border,
        t.llm_context AS llm_context
      ORDER BY
        CASE t.key
          WHEN 'invariant' THEN 1
          WHEN 'localized' THEN 2
          WHEN 'knowledge' THEN 3
          WHEN 'derived' THEN 4
          WHEN 'job' THEN 5
          ELSE 6
        END
    `);

    const traits: TaxonomyTrait[] = traitResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      color: record.get('color') as string,
      border_style: record.get('border_style') as string | null,
      border_width: record.get('border_width')?.toNumber?.() ?? record.get('border_width') ?? null,
      unicode_border: record.get('unicode_border') as string | null,
      llm_context: record.get('llm_context') as string | null,
    }));

    // Fetch arc families with visual encoding
    const arcFamilyResult = await session.run(`
      MATCH (f:ArcFamily)
      RETURN
        f.key AS key,
        f.display_name AS display_name,
        f.color AS color,
        f.stroke_style AS stroke_style,
        f.stroke_width AS stroke_width,
        f.arrow_style AS arrow_style,
        f.llm_context AS llm_context
      ORDER BY
        CASE f.key
          WHEN 'ownership' THEN 1
          WHEN 'localization' THEN 2
          WHEN 'semantic' THEN 3
          WHEN 'generation' THEN 4
          WHEN 'mining' THEN 5
          ELSE 6
        END
    `);

    const arcFamilies: TaxonomyArcFamily[] = arcFamilyResult.records.map((record) => ({
      key: record.get('key') as string,
      display_name: record.get('display_name') as string,
      color: record.get('color') as string,
      stroke_style: record.get('stroke_style') as string | null,
      stroke_width: record.get('stroke_width')?.toNumber?.() ?? record.get('stroke_width') ?? null,
      arrow_style: record.get('arrow_style') as string || '-->',
      llm_context: record.get('llm_context') as string | null,
    }));

    // Fetch Kind → Layer mapping
    const mappingResult = await session.run(`
      MATCH (l:Layer)-[:HAS_KIND]->(k:Kind)
      RETURN k.label AS node_type, l.key AS layer
      ORDER BY k.label
    `);

    const nodeTypeMapping: Record<string, string> = {};
    for (const record of mappingResult.records) {
      nodeTypeMapping[record.get('node_type') as string] = record.get('layer') as string;
    }

    const response: TaxonomyResponse = {
      version: '9.5.0',
      realms,
      traits,
      arcFamilies,
      nodeTypeMapping,
    };

    return NextResponse.json({
      success: true,
      data: response,
    });
  } catch (error) {
    return handleApiError(error, '/graph/taxonomy GET');
  } finally {
    await session.close();
  }
}
