// src/filters/CypherGenerator.ts
/**
 * CypherGenerator - Converts NovaNetFilter criteria to executable Cypher queries.
 *
 * @example
 * ```typescript
 * const filter = NovaNetFilter.create()
 *   .fromPage('page-pricing')
 *   .includeBlocks()
 *   .forLocale('fr-FR');
 *
 * const { query, params } = CypherGenerator.generate(filter);
 * // query: "MATCH (root:Page {key: $rootKey})..."
 * // params: { rootKey: 'page-pricing', locale: 'fr-FR' }
 * ```
 */
import type { CypherQuery } from './types.js';
import { NovaNetFilter } from './NovaNetFilter.js';

// =============================================================================
// RELATION TO ALIAS/TARGET TYPE MAPPINGS
// =============================================================================

const RELATION_ALIAS_MAP: Record<string, string> = {
  // Core structure (v11.6)
  HAS_PAGE: 'page',
  HAS_BLOCK: 'block',
  HAS_ENTITY: 'entity',
  HAS_BRAND_IDENTITY: 'brandIdentity',
  HAS_PROJECT: 'project',
  OF_TYPE: 'blockType',
  OF_KIND: 'kind',
  // Entity relationships
  USES_ENTITY: 'usedEntity',
  SEMANTIC_LINK: 'relatedEntity',
  BELONGS_TO: 'category',
  INCLUDES: 'includedEntity',
  APPLIES_TO: 'applicableEntity',
  ENABLES: 'enabledEntity',
  SIMILAR_TO: 'similarEntity',
  REQUIRES: 'requiredEntity',
  TYPE_OF: 'typeEntity',
  VARIANT_OF: 'variantEntity',
  ALTERNATIVE_TO: 'alternativeEntity',
  // Instructions
  HAS_INSTRUCTION: 'instruction',
  HAS_RULES: 'rules',
  // Output
  HAS_GENERATED: 'generated',
  HAS_CONTENT: 'content',
  CONTENT_OF: 'contentParent',
  HAS_LOCALIZED_CONTENT: 'localizedContent',
  // Locale knowledge (v11.5 schema)
  HAS_CULTURE: 'culture',
  HAS_MARKET: 'market',
  HAS_FORMATTING: 'formatting',
  HAS_SLUGIFICATION: 'slugification',
  HAS_EXPRESSIONS: 'expressionSet',
  HAS_EXPRESSION: 'expression',
  CONTAINS: 'contained',
  FOLLOWS_RULE: 'slugRule',
  // Geographic
  HAS_INCOME_LEVEL: 'incomeLevel',
  IN_CULTURAL_SUBREALM: 'culturalSubrealm',
  SPEAKS_BRANCH: 'languageBranch',
  IN_SUBREGION: 'subregion',
  HAS_PRIMARY_POPULATION: 'population',
  IN_CONTINENT: 'continent',
  IN_REGION: 'region',
  PART_OF_REALM: 'culturalRealm',
  BRANCH_OF: 'languageFamily',
  CLUSTER_OF: 'populationCluster',
  // SEO
  EXPRESSES: 'seoKeyword',
  TARGETS: 'target',
  SEO_MINES: 'minedSeoKeyword',
  // Locale
  SUPPORTS_LOCALE: 'supportedLocale',
  DEFAULT_LOCALE: 'defaultLocale',
  FALLBACK_TO: 'fallbackLocale',
  FOR_LOCALE: 'forLocale',
  // Metrics & Assembly
  HAS_METRICS: 'metrics',
  ASSEMBLES: 'assembledBlock',
  // Provenance
  GENERATED: 'generatedOutput',
  INFLUENCED_BY: 'influencingEntity',
  GENERATED_FROM: 'generatedFromType',
  BELONGS_TO_PROJECT_CONTENT: 'projectContent',
  // Schema-graph (Class/ArcClass) - v11.8 ADR-023
  HAS_CLASS: 'hasClass',
  HAS_ARC_CLASS: 'arcClass',
  IN_REALM: 'inRealm',
  IN_LAYER: 'inLayer',
  EXHIBITS: 'exhibits',
  IN_FAMILY: 'inFamily',
  FROM_CLASS: 'fromClass',
  TO_CLASS: 'toClass',
  HAS_LAYER: 'layer',
};

// v11.6: Target type labels are now OPTIONAL
// The same relation can target different node types depending on context:
// - HAS_CONTENT: Entity → EntityContent, Project → ProjectContent
// - HAS_ENTITY: Project → Entity
// Rather than trying to map every context, we omit the target label and let Neo4j
// return whatever node is connected. This is more flexible and schema-independent.

// =============================================================================
// CYPHER GENERATOR
// =============================================================================

export class CypherGenerator {
  /**
   * Generate a Cypher query from a NovaNetFilter
   *
   * Returns both nodes AND relationships so that graphStore receives edges.
   */
  static generate(filter: NovaNetFilter): CypherQuery {
    const criteria = filter.getCriteria();
    const params: Record<string, unknown> = {};
    const lines: string[] = [];
    const nodeAliases = new Set<string>(['root']);
    const relAliases: string[] = [];

    // 1. MATCH root node
    if (criteria.root) {
      // If a specific key is provided, match that node
      // Otherwise, match all nodes of the specified type
      if (criteria.root.key) {
        lines.push(`MATCH (root:${criteria.root.type} {key: $rootKey})`);
        params.rootKey = criteria.root.key;
      } else {
        lines.push(`MATCH (root:${criteria.root.type})`);
      }
    }

    // 2. OPTIONAL MATCH for includes (capture both nodes AND relationships)
    let relIndex = 0;
    for (const include of criteria.includes) {
      const alias = this.relationToAlias(include.relation);
      nodeAliases.add(alias);

      // Create a unique relationship alias
      const relAlias = `r${relIndex++}`;
      relAliases.push(relAlias);

      const arrow = this.directionToArrow(include.direction);
      const activeFilter = include.filters?.active ? ' {active: true}' : '';
      // Capture the relationship with a variable (r0, r1, etc.)
      // v11.6: Don't specify target type - let Neo4j return whatever is connected
      // This handles cases where same relation targets different types (HAS_CONTENT, HAS_ENTITY, etc.)
      const matchLine = `OPTIONAL MATCH (root)${arrow.left}[${relAlias}:${include.relation}]${arrow.right}(${alias}${activeFilter})`;

      lines.push(matchLine);

      // Handle spreading activation for entities (v10.3: USES_ENTITY replaces USES_CONCEPT)
      if (include.relation === 'USES_ENTITY' && include.depth && include.depth > 1) {
        const relatedAlias = `related${this.capitalize(alias)}`;
        const spreadRelAlias = `r${relIndex++}`;
        relAliases.push(spreadRelAlias);
        lines.push(`OPTIONAL MATCH (${alias})-[${spreadRelAlias}:SEMANTIC_LINK*1..${include.depth - 1}]->(${relatedAlias}:Entity)`);
        nodeAliases.add(relatedAlias);
      }
    }

    // 3. WHERE clauses
    const whereConditions: string[] = [];

    if (criteria.filters.locale) {
      params.locale = criteria.filters.locale;
    }

    // Node type filtering (byTypes)
    if (criteria.filters.nodeTypes?.length) {
      const typeConditions = criteria.filters.nodeTypes.map(t => `root:${t}`).join(' OR ');
      whereConditions.push(`(${typeConditions})`);
    }

    // Exclude types
    if (criteria.filters.excludeTypes?.length) {
      const excludeConditions = criteria.filters.excludeTypes.map(t => `NOT root:${t}`).join(' AND ');
      whereConditions.push(`(${excludeConditions})`);
    }

    // Fulltext search
    if (criteria.filters.searchQuery) {
      const fields = criteria.filters.searchFields || ['key', 'display_name', 'description'];
      const searchConditions = fields.map(f => `toLower(root.${f}) CONTAINS toLower($searchQuery)`).join(' OR ');
      whereConditions.push(`(${searchConditions})`);
      params.searchQuery = criteria.filters.searchQuery;
    }

    if (whereConditions.length > 0) {
      lines.push(`WHERE ${whereConditions.join(' AND ')}`);
    }

    // 4. RETURN - include both nodes AND relationships
    const returns: string[] = ['root'];

    // Add node collections
    for (const alias of nodeAliases) {
      if (alias !== 'root') {
        returns.push(`collect(DISTINCT ${alias}) AS ${alias}s`);
      }
    }

    // Add relationship collections (crucial for edges in graphStore!)
    for (const relAlias of relAliases) {
      returns.push(`collect(DISTINCT ${relAlias}) AS ${relAlias}s`);
    }

    lines.push(`RETURN ${returns.join(', ')}`);

    return {
      query: lines.join('\n'),
      params,
    };
  }

  // ===========================================================================
  // PRIVATE HELPERS
  // ===========================================================================

  private static relationToAlias(relation: string): string {
    return RELATION_ALIAS_MAP[relation] || relation.toLowerCase();
  }

  private static directionToArrow(direction?: string): { left: string; right: string } {
    switch (direction) {
      case 'incoming':
        return { left: '<-', right: '-' };
      case 'both':
        return { left: '-', right: '-' };
      case 'outgoing':
      default:
        return { left: '-', right: '->' };
    }
  }

  private static capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
  }
}
