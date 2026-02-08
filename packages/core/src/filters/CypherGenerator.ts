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
  // Core structure
  HAS_PAGE: 'page',
  HAS_BLOCK: 'block',
  HAS_BRAND_IDENTITY: 'brandIdentity',
  OF_TYPE: 'blockType',
  // v10.3: Entity replaces Concept
  USES_ENTITY: 'entity',
  SEMANTIC_LINK: 'relatedEntity',
  // Prompts
  HAS_PROMPT: 'prompt',
  HAS_RULES: 'rules',
  // Output
  HAS_GENERATED: 'output',
  HAS_CONTENT: 'l10n',
  // Locale knowledge
  HAS_IDENTITY: 'identity',
  HAS_VOICE: 'voice',
  HAS_CULTURE: 'culture',
  HAS_MARKET: 'market',
  HAS_LEXICON: 'lexicon',
  HAS_EXPRESSION: 'expression',
  // SEO (v10.3: EXPRESSES replaces TARGETS_SEO, GEO removed)
  EXPRESSES: 'seoKeyword',
  // Locale
  SUPPORTS_LOCALE: 'supportedLocale',
  FALLBACK_TO: 'fallbackLocale',
  FOR_LOCALE: 'locale',
  // Metrics & Assembly
  HAS_METRICS: 'metrics',
  ASSEMBLES: 'assembledBlock',
  // Provenance
  GENERATED: 'generatedOutput',
  INFLUENCED_BY: 'influencingEntity',  // v10.3: renamed from influencingConcept
  GENERATED_FROM: 'generatedFromType',
  BELONGS_TO_PROJECT_L10N: 'projectL10n',
  // SEO Mining
  SEO_MINES: 'minedSeoKeyword',
  // REMOVED v10.3: HAS_CONCEPT, USES_CONCEPT, TARGETS_SEO, TARGETS_GEO, GEO_MINES (GEO removed)
};

const RELATION_TARGET_TYPE_MAP: Record<string, string> = {
  // Core structure
  HAS_PAGE: 'Page',
  HAS_BLOCK: 'Block',
  HAS_BRAND_IDENTITY: 'BrandIdentity',
  OF_TYPE: 'BlockType',
  // v10.3: Entity replaces Concept
  USES_ENTITY: 'Entity',
  SEMANTIC_LINK: 'Entity',
  // Prompts
  HAS_PROMPT: 'PagePrompt',
  HAS_RULES: 'BlockRules',
  // Output
  HAS_GENERATED: 'PageGenerated',
  HAS_CONTENT: 'EntityContent',
  // Locale knowledge
  HAS_IDENTITY: 'LocaleIdentity',
  HAS_VOICE: 'LocaleVoice',
  HAS_CULTURE: 'LocaleCulture',
  HAS_MARKET: 'LocaleMarket',
  HAS_LEXICON: 'LocaleLexicon',
  HAS_EXPRESSION: 'Expression',
  // SEO (v10.3: EXPRESSES replaces TARGETS_SEO, GEO removed)
  EXPRESSES: 'SEOKeyword',
  // Locale
  SUPPORTS_LOCALE: 'Locale',
  FALLBACK_TO: 'Locale',
  FOR_LOCALE: 'Locale',
  // Metrics & Assembly
  HAS_METRICS: 'PageMetrics',
  ASSEMBLES: 'BlockGenerated',
  // Provenance
  GENERATED: 'PageGenerated',
  INFLUENCED_BY: 'EntityContent',  // v10.3: was ConceptL10n
  GENERATED_FROM: 'BlockType',
  BELONGS_TO_PROJECT_L10N: 'ProjectL10n',
  // SEO Mining
  SEO_MINES: 'SEOKeyword',
  // REMOVED v10.3: HAS_CONCEPT, USES_CONCEPT, TARGETS_SEO, TARGETS_GEO, GEO_MINES (GEO removed)
};

// =============================================================================
// CYPHER GENERATOR
// =============================================================================

export class CypherGenerator {
  /**
   * Generate a Cypher query from a NovaNetFilter
   */
  static generate(filter: NovaNetFilter): CypherQuery {
    const criteria = filter.getCriteria();
    const params: Record<string, unknown> = {};
    const lines: string[] = [];
    const aliases = new Set<string>(['root']);

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

    // 2. OPTIONAL MATCH for includes
    for (const include of criteria.includes) {
      const alias = this.relationToAlias(include.relation);
      aliases.add(alias);

      const targetType = this.relationToTargetType(include.relation);
      const arrow = this.directionToArrow(include.direction);
      const activeFilter = include.filters?.active ? ' {active: true}' : '';
      const matchLine = `OPTIONAL MATCH (root)${arrow.left}[:${include.relation}]${arrow.right}(${alias}:${targetType}${activeFilter})`;

      lines.push(matchLine);

      // Handle spreading activation for entities (v10.3: USES_ENTITY replaces USES_CONCEPT)
      if (include.relation === 'USES_ENTITY' && include.depth && include.depth > 1) {
        const relatedAlias = `related${this.capitalize(alias)}`;
        lines.push(`OPTIONAL MATCH (${alias})-[:SEMANTIC_LINK*1..${include.depth - 1}]->(${relatedAlias}:Entity)`);
        aliases.add(relatedAlias);
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

    // 4. RETURN
    const returns = Array.from(aliases).map(a => {
      if (a === 'root') return 'root';
      return `collect(DISTINCT ${a}) AS ${a}s`;
    });
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

  private static relationToTargetType(relation: string): string {
    return RELATION_TARGET_TYPE_MAP[relation] || 'Node';
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
