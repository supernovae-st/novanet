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
import type { CypherQuery, NodeCategory, NodeType } from './types.js';
import { NODE_CATEGORIES } from './types.js';
import { NovaNetFilter } from './NovaNetFilter.js';

// =============================================================================
// RELATION TO ALIAS/TARGET TYPE MAPPINGS
// =============================================================================

const RELATION_ALIAS_MAP: Record<string, string> = {
  // Core structure
  HAS_PAGE: 'page',
  HAS_BLOCK: 'block',
  HAS_CONCEPT: 'projectConcept',
  HAS_BRAND_IDENTITY: 'brandIdentity',
  OF_TYPE: 'blockType',
  USES_CONCEPT: 'concept',
  SEMANTIC_LINK: 'relatedConcept',
  // Prompts
  HAS_PROMPT: 'prompt',
  HAS_RULES: 'rules',
  // Output
  HAS_OUTPUT: 'output',
  HAS_L10N: 'l10n',
  // Locale knowledge
  HAS_IDENTITY: 'identity',
  HAS_VOICE: 'voice',
  HAS_CULTURE: 'culture',
  HAS_MARKET: 'market',
  HAS_LEXICON: 'lexicon',
  HAS_EXPRESSION: 'expression',
  // SEO/GEO
  TARGETS_SEO: 'seoKeyword',
  TARGETS_GEO: 'geoSeed',
  // REMOVED v7.8.1: PAGE_TARGETS_SEO, PAGE_TARGETS_GEO (bypasses semantic grouping)
  // Locale
  SUPPORTS_LOCALE: 'supportedLocale',
  FALLBACK_TO: 'fallbackLocale',
  FOR_LOCALE: 'locale',
  // Metrics & Assembly
  HAS_METRICS: 'metrics',
  ASSEMBLES: 'assembledBlock',
  // Provenance
  GENERATED: 'generatedOutput',
  INFLUENCED_BY: 'influencingConcept',
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at ConceptL10n level)
  GENERATED_FROM: 'generatedFromType',
  BELONGS_TO_PROJECT_L10N: 'projectL10n',
  // SEO Mining (v7.8.5: HAS_SNAPSHOT removed, use HAS_METRICS)
  SEO_MINES: 'minedSeoKeyword',
  // REMOVED v7.8.5: SEO_DISCOVERED_BY, HAS_VARIATION, HAS_SNAPSHOT, VARIATES
  // GEO Mining (v7.8.5: HAS_CITATION removed, use HAS_METRICS)
  GEO_MINES: 'minedGeoSeed',
  // REMOVED v7.8.5: GEO_DISCOVERED_BY, HAS_REFORMULATION, HAS_CITATION, REFORMULATES
};

const RELATION_TARGET_TYPE_MAP: Record<string, string> = {
  // Core structure
  HAS_PAGE: 'Page',
  HAS_BLOCK: 'Block',
  HAS_CONCEPT: 'Concept',
  HAS_BRAND_IDENTITY: 'BrandIdentity',
  OF_TYPE: 'BlockType',
  USES_CONCEPT: 'Concept',
  SEMANTIC_LINK: 'Concept',
  // Prompts
  HAS_PROMPT: 'PagePrompt',
  HAS_RULES: 'BlockRules',
  // Output
  HAS_OUTPUT: 'PageL10n',
  HAS_L10N: 'ConceptL10n',
  // Locale knowledge
  HAS_IDENTITY: 'LocaleIdentity',
  HAS_VOICE: 'LocaleVoice',
  HAS_CULTURE: 'LocaleCulture',
  HAS_MARKET: 'LocaleMarket',
  HAS_LEXICON: 'LocaleLexicon',
  HAS_EXPRESSION: 'Expression',
  // SEO/GEO (v7.8.2: SEOKeyword → SEOKeywordL10n, v7.8.3: GEOSeed → GEOSeedL10n)
  TARGETS_SEO: 'SEOKeywordL10n',
  TARGETS_GEO: 'GEOSeedL10n',
  // REMOVED v7.8.1: PAGE_TARGETS_SEO, PAGE_TARGETS_GEO (bypasses semantic grouping)
  // Locale
  SUPPORTS_LOCALE: 'Locale',
  FALLBACK_TO: 'Locale',
  FOR_LOCALE: 'Locale',
  // Metrics & Assembly
  HAS_METRICS: 'PageMetrics',
  ASSEMBLES: 'BlockL10n',
  // Provenance
  GENERATED: 'PageL10n',
  INFLUENCED_BY: 'ConceptL10n',
  // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED
  GENERATED_FROM: 'BlockType',
  BELONGS_TO_PROJECT_L10N: 'ProjectL10n',
  // SEO Mining (v7.8.5: HAS_SNAPSHOT removed, use HAS_METRICS)
  SEO_MINES: 'SEOKeywordL10n',
  // REMOVED v7.8.5: HAS_SNAPSHOT (use HAS_METRICS: SEOKeywordL10n → SEOKeywordMetrics)
  // GEO Mining (v7.8.5: HAS_CITATION removed, use HAS_METRICS)
  GEO_MINES: 'GEOSeedL10n',
  // REMOVED v7.8.5: HAS_CITATION (use HAS_METRICS: GEOSeedL10n → GEOSeedMetrics)
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
      lines.push(`MATCH (root:${criteria.root.type} {key: $rootKey})`);
      params.rootKey = criteria.root.key;
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

      // Handle spreading activation for concepts
      if (include.relation === 'USES_CONCEPT' && include.depth && include.depth > 1) {
        const relatedAlias = `related${this.capitalize(alias)}`;
        lines.push(`OPTIONAL MATCH (${alias})-[:SEMANTIC_LINK*1..${include.depth - 1}]->(${relatedAlias}:Concept)`);
        aliases.add(relatedAlias);
      }
    }

    // 3. WHERE clauses
    const whereConditions: string[] = [];

    // REMOVED v8.2.0: priority and freshness filtering (YAML v7.11.0 alignment)
    // if (criteria.filters.priority?.length) {
    //   whereConditions.push('root.priority IN $priorities');
    //   params.priorities = criteria.filters.priority;
    // }
    // if (criteria.filters.freshness?.length) {
    //   whereConditions.push('root.freshness IN $freshness');
    //   params.freshness = criteria.filters.freshness;
    // }

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

    // Category filtering (expand to node types)
    if (criteria.filters.categories?.length) {
      const categoryTypes = this.expandCategories(criteria.filters.categories);
      if (categoryTypes.length > 0) {
        const typeConditions = categoryTypes.map(t => `root:${t}`).join(' OR ');
        whereConditions.push(`(${typeConditions})`);
      }
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

  private static expandCategories(categories: NodeCategory[]): NodeType[] {
    const types: NodeType[] = [];
    for (const category of categories) {
      const categoryTypes = NODE_CATEGORIES[category];
      if (categoryTypes) {
        types.push(...categoryTypes);
      }
    }
    return types;
  }
}
