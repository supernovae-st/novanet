// =============================================================================
// VIEW CYPHER QUERIES (v11.6)
// =============================================================================
// Cypher query builders for each context view type
//
// NOTE: Cypher queries are now embedded directly in packages/core/models/views.yaml (v0.12.5).
// Use ViewLoader.getCypher(viewId) from @novanet/core/filters instead.

import type { ViewId } from '@/config/viewTypes';

// =============================================================================
// TYPES
// =============================================================================

export interface ViewQueryParams {
  nodeId: string;
  nodeKey: string;
  nodeType: string;
  limit?: number;
}

export interface ViewQuery {
  cypher: string;
  params: Record<string, unknown>;
  description: string;
}

// =============================================================================
// QUERY BUILDERS
// =============================================================================

/**
 * Get Cypher query for a specific view
 */
export function getViewQuery(viewId: ViewId, params: ViewQueryParams): ViewQuery {
  const { nodeKey, limit = 50 } = params;

  switch (viewId) {
    // -------------------------------------------------------------------------
    // TREE STYLE VIEWS
    // -------------------------------------------------------------------------
    case 'composition':
      return {
        cypher: `
          MATCH (root {key: $nodeKey})
          WHERE root:Page OR root:Block
          OPTIONAL MATCH path = (root)-[:HAS_BLOCK*1..3]->(block:Block)
          WITH root, collect(DISTINCT block) AS blocks
          UNWIND ([root] + blocks) AS n
          WITH collect(DISTINCT n) AS nodes
          UNWIND nodes AS n
          OPTIONAL MATCH (n)-[r:HAS_BLOCK]->(m)
          WHERE m IN nodes
          RETURN nodes, collect(DISTINCT r) AS relationships
        `,
        params: { nodeKey },
        description: 'Page/Block composition hierarchy',
      };

    case 'knowledge':
      return {
        cypher: `
          MATCH (locale:Locale {key: $nodeKey})
          OPTIONAL MATCH (locale)-[r1:HAS_TERMS|HAS_EXPRESSIONS|HAS_PATTERNS|HAS_CULTURE|HAS_TABOOS|HAS_AUDIENCE]->(container)
          OPTIONAL MATCH (container)-[r2:CONTAINS_TERM|CONTAINS_EXPRESSION|CONTAINS_PATTERN|CONTAINS_CULTURE_REF|CONTAINS_TABOO|CONTAINS_AUDIENCE_TRAIT]->(atom)
          WITH locale,
               collect(DISTINCT container) AS containers,
               collect(DISTINCT atom)[0..$limit] AS atoms,
               collect(DISTINCT r1) AS r1s,
               collect(DISTINCT r2) AS r2s
          RETURN [locale] + containers + atoms AS nodes,
                 r1s + r2s AS relationships
        `,
        params: { nodeKey, limit },
        description: 'Locale knowledge atoms (Terms, Expressions, Patterns)',
      };

    case 'geographic':
      return {
        cypher: `
          MATCH (locale:Locale {key: $nodeKey})
          OPTIONAL MATCH (locale)-[r1:FOR_COUNTRY]->(country)
          OPTIONAL MATCH (country)-[r2:IN_REGION]->(region)
          OPTIONAL MATCH (region)-[r3:IN_CONTINENT]->(continent)
          WITH locale, country, region, continent,
               collect(DISTINCT r1) + collect(DISTINCT r2) + collect(DISTINCT r3) AS rels
          RETURN [locale, country, region, continent] AS nodes, rels AS relationships
        `,
        params: { nodeKey },
        description: 'Geographic hierarchy (Country → Region → Continent)',
      };

    case 'project':
      return {
        cypher: `
          MATCH (project:Project {key: $nodeKey})
          OPTIONAL MATCH (project)-[r1:HAS_PAGE]->(page:Page)
          OPTIONAL MATCH (project)-[r2:HAS_ENTITY]->(entity:Entity)
          OPTIONAL MATCH (project)-[r3:HAS_BRAND]->(brand:Brand)
          WITH project,
               collect(DISTINCT page)[0..$limit] AS pages,
               collect(DISTINCT entity)[0..$limit] AS entities,
               collect(DISTINCT brand) AS brands,
               collect(DISTINCT r1) + collect(DISTINCT r2) + collect(DISTINCT r3) AS rels
          RETURN [project] + pages + entities + brands AS nodes, rels AS relationships
        `,
        params: { nodeKey, limit },
        description: 'Project structure (Pages, Entities, Brand)',
      };

    case 'brand':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Project OR n:Brand
          OPTIONAL MATCH (n)-[r:HAS_BRAND|BRAND_OF]-(brand:Brand)
          OPTIONAL MATCH (brand)-[r2:HAS_VOICE]->(voice)
          RETURN [n, brand, voice] AS nodes,
                 collect(DISTINCT r) + collect(DISTINCT r2) AS relationships
        `,
        params: { nodeKey },
        description: 'Brand identity configuration',
      };

    // -------------------------------------------------------------------------
    // FLOW STYLE VIEWS
    // -------------------------------------------------------------------------
    case 'entities':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Page OR n:Block OR n:Entity
          OPTIONAL MATCH (n)-[r:USES_ENTITY|HAS_ENTITY|TARGETS]->(entity:Entity)
          OPTIONAL MATCH (n)<-[r2:USES_ENTITY|HAS_ENTITY|TARGETS]-(other)
          WITH n,
               collect(DISTINCT entity) AS outEntities,
               collect(DISTINCT other) AS inNodes,
               collect(DISTINCT r) AS outRels,
               collect(DISTINCT r2) AS inRels
          RETURN [n] + outEntities + inNodes AS nodes,
                 outRels + inRels AS relationships
        `,
        params: { nodeKey },
        description: 'Connected entities and semantic links',
      };

    case 'seo-intel':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Entity OR n:Page OR n:SEOKeyword
          OPTIONAL MATCH (n)-[r1:TARGETS]->(keyword:SEOKeyword)
          OPTIONAL MATCH (keyword)-[r2:IN_CLUSTER]->(cluster:SEOKeywordSet)
          OPTIONAL MATCH (keyword)-[r3:HAS_METRICS]->(metrics:SEOKeywordMetrics)
          WITH n,
               collect(DISTINCT keyword)[0..$limit] AS keywords,
               collect(DISTINCT cluster) AS clusters,
               collect(DISTINCT metrics) AS metrics,
               collect(DISTINCT r1) + collect(DISTINCT r2) + collect(DISTINCT r3) AS rels
          RETURN [n] + keywords + clusters + metrics AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey, limit },
        description: 'SEO keywords, clusters, and metrics',
      };

    case 'geo-intel':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Entity OR n:GEOQuery
          OPTIONAL MATCH (n)-[r1:MONITORS_GEO]->(query:GEOQuery)
          OPTIONAL MATCH (query)-[r2:HAS_ANSWER]->(answer:GEOAnswer)
          OPTIONAL MATCH (query)-[r3:IN_QUERY_SET]->(querySet:GEOQuerySet)
          WITH n,
               collect(DISTINCT query)[0..$limit] AS queries,
               collect(DISTINCT answer) AS answers,
               collect(DISTINCT querySet) AS querySets,
               collect(DISTINCT r1) + collect(DISTINCT r2) + collect(DISTINCT r3) AS rels
          RETURN [n] + queries + answers + querySets AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey, limit },
        description: 'GEO queries and AI-generated answers',
      };

    case 'generation':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Page OR n:Block OR n:PageNative OR n:BlockNative
          OPTIONAL MATCH (n)-[r1:HAS_NATIVE]->(generated)
          OPTIONAL MATCH (n)<-[r2:NATIVE_OF]-(generated2)
          OPTIONAL MATCH (generated)-[r3:FOR_LOCALE]->(locale:Locale)
          WITH n,
               collect(DISTINCT generated) AS gen1,
               collect(DISTINCT generated2) AS gen2,
               collect(DISTINCT locale) AS locales,
               collect(DISTINCT r1) + collect(DISTINCT r2) + collect(DISTINCT r3) AS rels
          RETURN [n] + gen1 + gen2 + locales AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey },
        description: 'Generation pipeline and outputs',
      };

    case 'categories':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Entity OR n:EntityCategory
          OPTIONAL MATCH (n)-[r1:BELONGS_TO]->(category:EntityCategory)
          OPTIONAL MATCH (n)<-[r2:BELONGS_TO]-(entity:Entity)
          WITH n,
               collect(DISTINCT category) AS categories,
               collect(DISTINCT entity)[0..$limit] AS entities,
               collect(DISTINCT r1) + collect(DISTINCT r2) AS rels
          RETURN [n] + categories + entities AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey, limit },
        description: 'Entity category classification',
      };

    case 'cross-realm':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          OPTIONAL MATCH (n)-[r]->(shared)
          WHERE r.scope = 'cross_realm' OR type(r) IN ['BELONGS_TO', 'TARGETS', 'FOR_LOCALE', 'MONITORS_GEO', 'FOR_COUNTRY']
          WITH n, collect(DISTINCT shared) AS sharedNodes, collect(DISTINCT r) AS rels
          RETURN [n] + sharedNodes AS nodes, rels AS relationships
        `,
        params: { nodeKey },
        description: 'Cross-realm connections (org ↔ shared)',
      };

    // -------------------------------------------------------------------------
    // COMPACT STYLE VIEWS
    // -------------------------------------------------------------------------
    case 'locales':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Page OR n:Entity OR n:Project OR n:Block
          OPTIONAL MATCH (n)-[r:HAS_NATIVE|HAS_NATIVE]->(content)
          OPTIONAL MATCH (content)-[r2:FOR_LOCALE]->(locale:Locale)
          WITH n,
               collect(DISTINCT content) AS contents,
               collect(DISTINCT locale) AS locales,
               collect(DISTINCT r) + collect(DISTINCT r2) AS rels
          RETURN [n] + contents + locales AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey },
        description: 'Locale coverage and content status',
      };

    case 'content':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:Entity OR n:Project
          OPTIONAL MATCH (n)-[r:HAS_NATIVE]->(content)
          OPTIONAL MATCH (content)-[r2:FOR_LOCALE]->(locale:Locale)
          WITH n,
               collect(DISTINCT content) AS contents,
               collect(DISTINCT locale) AS locales,
               collect(DISTINCT r) + collect(DISTINCT r2) AS rels
          RETURN [n] + contents + locales AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey },
        description: 'Content per locale',
      };

    case 'metrics':
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          WHERE n:SEOKeyword OR n:SEOKeywordMetrics OR n:GEOQuery OR n:GEOAnswer
          OPTIONAL MATCH (n)-[r:HAS_METRICS]->(metrics)
          OPTIONAL MATCH (n)<-[r2:HAS_METRICS]-(parent)
          WITH n,
               collect(DISTINCT metrics) AS metricNodes,
               collect(DISTINCT parent) AS parents,
               collect(DISTINCT r) + collect(DISTINCT r2) AS rels
          RETURN [n] + metricNodes + parents AS nodes,
                 rels AS relationships
        `,
        params: { nodeKey },
        description: 'Performance metrics and analytics',
      };

    default:
      return {
        cypher: `
          MATCH (n {key: $nodeKey})
          OPTIONAL MATCH (n)-[r]-(connected)
          RETURN [n] + collect(DISTINCT connected)[0..$limit] AS nodes,
                 collect(DISTINCT r) AS relationships
        `,
        params: { nodeKey, limit },
        description: 'Default view - all connections',
      };
  }
}

/**
 * Get stats query for a view (lighter weight, counts only)
 */
export function getViewStatsQuery(viewId: ViewId, params: ViewQueryParams): ViewQuery {
  const { nodeKey } = params;

  return {
    cypher: `
      MATCH (n {key: $nodeKey})
      OPTIONAL MATCH (n)-[r]-(connected)
      RETURN count(DISTINCT connected) AS nodeCount,
             count(DISTINCT r) AS arcCount,
             collect(DISTINCT labels(connected)) AS nodeTypes,
             collect(DISTINCT type(r)) AS arcTypes
    `,
    params: { nodeKey },
    description: `Stats for ${viewId} view`,
  };
}

export default getViewQuery;
