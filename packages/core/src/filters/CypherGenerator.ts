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
// v0.13.0 ADR-029: Complete mapping of all 169 arcs from YAML source of truth
// Generated from: packages/core/models/arc-classes/**/*.yaml
// =============================================================================

const RELATION_ALIAS_MAP: Record<string, string> = {
  // ---------------------------------------------------------------------------
  // OWNERSHIP FAMILY (79 arcs) - Parent→Child hierarchy
  // ---------------------------------------------------------------------------
  // Core structure
  HAS_PAGE: 'page',
  PAGE_OF: 'pageParent',
  HAS_BLOCK: 'block',
  BLOCK_OF: 'blockParent',
  HAS_ENTITY: 'entity',
  ENTITY_OF: 'entityParent',
  HAS_PROJECT: 'project',
  PROJECT_OF: 'projectParent',
  HAS_BRAND: 'brand',
  BRAND_OF: 'brandParent',
  HAS_CHILD: 'child',
  CHILD_OF: 'parent',
  // Type relationships
  OF_TYPE: 'blockType',
  TYPE_OF: 'typeTarget',
  HAS_TYPE: 'hasType',
  ACCEPTS_BLOCK_TYPE: 'acceptsBlockType',
  // Instructions & rules
  HAS_INSTRUCTION: 'instruction',
  INSTRUCTION_OF: 'instructionParent',
  HAS_RULES: 'rules',
  HAS_ADAPTATION: 'adaptation',
  ADAPTATION_OF: 'adaptationParent',
  // Native content (v0.13.0 ADR-029)
  HAS_NATIVE: 'native',
  NATIVE_OF: 'nativeParent',
  BELONGS_TO_PROJECT_NATIVE: 'projectNative',
  // Brand architecture (ADR-028)
  HAS_DESIGN: 'design',
  DESIGN_OF: 'designParent',
  HAS_PRINCIPLES: 'principles',
  PRINCIPLES_OF: 'principlesParent',
  HAS_PROMPT_STYLE: 'promptStyle',
  PROMPT_STYLE_OF: 'promptStyleParent',
  // Locale settings
  HAS_STYLE: 'style',
  STYLE_OF: 'styleParent',
  HAS_FORMATTING: 'formatting',
  FORMATTING_OF: 'formattingParent',
  HAS_SLUGIFICATION: 'slugification',
  SLUGIFICATION_OF: 'slugificationParent',
  HAS_CULTURE: 'culture',
  CULTURE_OF: 'cultureParent',
  HAS_CULTURE_SET: 'cultureSet',
  CULTURE_SET_OF: 'cultureSetParent',
  HAS_MARKET: 'market',
  MARKET_OF: 'marketParent',
  HAS_AUDIENCE: 'audience',
  AUDIENCE_OF: 'audienceParent',
  // Knowledge containers
  HAS_TERMS: 'terms',
  TERMS_OF: 'termsParent',
  HAS_EXPRESSIONS: 'expressions',
  EXPRESSIONS_OF: 'expressionsParent',
  HAS_PATTERNS: 'patterns',
  PATTERNS_OF: 'patternsParent',
  HAS_TABOOS: 'taboos',
  TABOOS_OF: 'taboosParent',
  // Container contents
  CONTAINS_TERM: 'containedTerm',
  CONTAINS_EXPRESSION: 'containedExpression',
  CONTAINS_PATTERN: 'containedPattern',
  CONTAINS_TABOO: 'containedTaboo',
  CONTAINS_CULTURE_REF: 'containedCultureRef',
  CONTAINS_AUDIENCE_TRAIT: 'containedAudienceTrait',
  CONTAINS_SEO_KEYWORD: 'containedSeoKeyword',
  CONTAINS_GEO_QUERY: 'containedGeoQuery',
  // SEO/GEO sets
  HAS_SEO_KEYWORDS: 'seoKeywords',
  SEO_KEYWORDS_OF: 'seoKeywordsParent',
  HAS_GEO_QUERIES: 'geoQueries',
  GEO_QUERIES_OF: 'geoQueriesParent',
  HAS_GEO_ANSWERS: 'geoAnswers',
  HAS_KEYWORD: 'keyword',
  KEYWORD_OF: 'keywordParent',
  // Geographic hierarchy
  IN_CONTINENT: 'continent',
  IN_REGION: 'region',
  IN_SUBREGION: 'subregion',
  IN_COUNTRY: 'country',
  IN_CULTURAL_SUBREALM: 'culturalSubrealm',
  IN_ECONOMIC_REGION: 'economicRegion',
  HAS_REGION: 'hasRegion',
  HAS_SUBREGION: 'hasSubregion',
  HAS_SUBREALM: 'subrealm',
  HAS_BRANCH: 'branch',
  BRANCH_OF: 'branchParent',
  HAS_SUBCLUSTER: 'subcluster',
  PART_OF_REALM: 'culturalRealm',
  CLUSTER_OF: 'clusterParent',
  // Population
  HAS_POPULATION: 'population',
  POPULATION_OF: 'populationParent',
  HAS_PRIMARY_POPULATION: 'primaryPopulation',
  // Income/Lending
  HAS_INCOME_LEVEL: 'incomeLevel',
  INCOME_CLASSIFIES: 'incomeClassifies',
  HAS_LENDING_TYPE: 'lendingType',
  LENDING_CLASSIFIES: 'lendingClassifies',
  // Locale support
  SUPPORTS_LOCALE: 'supportedLocale',
  DEFAULT_LOCALE: 'defaultLocale',
  FALLBACK_TO: 'fallbackLocale',
  HAS_LOCALE: 'locale',
  HAS_LOCALE_VARIANT: 'localeVariant',
  LOCALE_VARIANT_OF: 'localeVariantParent',
  // Org structure
  BELONGS_TO_ORG: 'organization',
  HAS_SLOT: 'slot',
  HAS_FORMAT: 'format',
  HAS_VARIANT: 'variant',
  HAS_METRICS: 'metrics',
  METRICS_OF: 'metricsParent',

  // ---------------------------------------------------------------------------
  // LOCALIZATION FAMILY (20 arcs) - Locale relationships
  // ---------------------------------------------------------------------------
  FOR_LOCALE: 'forLocale',
  LOCALE_OF: 'localeTarget',
  FOR_MARKET: 'forMarket',
  FOR_CHANNEL: 'forChannel',
  SPEAKS_BRANCH: 'speaksBranch',
  SPOKEN_BY: 'spokenByLocale',
  POPULAR_IN: 'popularIn',
  PRIMARY_FOR: 'primaryFor',
  CULTURALLY_SIMILAR: 'culturallySimilar',
  INSPIRED_BY_REGION: 'inspiredByRegion',

  // ---------------------------------------------------------------------------
  // SEMANTIC FAMILY (52 arcs) - Meaning relationships
  // ---------------------------------------------------------------------------
  // Entity references
  USES_ENTITY: 'usedEntity',
  USED_BY: 'usedByNode',
  REFERENCES: 'references',
  REFERENCED_BY: 'referencedBy',
  REFERENCES_ENTITY: 'referencesEntity',
  REFERENCES_PAGE: 'referencesPage', // deprecated v0.13.1
  MENTIONS: 'mentions',
  MENTIONS_BRAND: 'mentionsBrand',
  // Page relationships
  LINKS_TO: 'linksTo',
  REPRESENTS: 'represents',
  REPRESENTED_BY: 'representedBy',
  SUBTOPIC_OF: 'subtopic',
  SEO_CLUSTER_OF: 'seoCluster',
  HAS_INTERNAL_LINK: 'internalLink',
  // Entity semantics
  SEMANTIC_LINK: 'semanticLink',
  SIMILAR_TO: 'similarTo',
  ALTERNATIVE_TO: 'alternativeTo',
  COMPETES_WITH: 'competesWith',
  COMPARES_A: 'comparesA',
  COMPARES_B: 'comparesB',
  VARIANT_OF: 'variantOf',
  CATEGORY_OF: 'categoryOf',
  BELONGS_TO: 'belongsTo',
  INCLUDES: 'includes',
  INCLUDED_IN: 'includedIn',
  INCLUDES_ENTITY: 'includesEntity',
  APPLIES_TO: 'appliesTo',
  // Capabilities
  ENABLES: 'enables',
  ENABLED_BY: 'enabledBy',
  ENHANCES: 'enhances',
  ENHANCED_BY: 'enhancedBy',
  REQUIRES: 'requires',
  REQUIRED_BY: 'requiredBy',
  HAS_APPLICATION: 'application',
  USE_CASE_FOR: 'useCaseFor',
  ACTS_ON: 'actsOn',
  OPERATED_BY: 'operatedBy',
  FILLS_SLOT: 'fillsSlot',
  // Targeting
  TARGETS: 'target',
  TARGETS_PERSONA: 'targetsPersona',
  EXPRESSES: 'expresses',

  // ---------------------------------------------------------------------------
  // GENERATION FAMILY (12 arcs) - LLM pipeline
  // ---------------------------------------------------------------------------
  GENERATED: 'generated',
  GENERATED_FROM: 'generatedFrom',
  COMPILED_FROM: 'compiledFrom',
  INFLUENCED_BY: 'influencedBy',
  INCLUDES_STYLE: 'includesStyle',
  ASSEMBLES: 'assembles',
  BUNDLES: 'bundles',
  PREVIOUS_VERSION: 'previousVersion',
  PRODUCED: 'produced',
  PRODUCED_BY: 'producedBy',
  READS: 'reads',
  READ_BY: 'readBy',
  DERIVED_SLUG_FROM: 'derivedSlugFrom',

  // ---------------------------------------------------------------------------
  // MINING FAMILY (6 arcs) - SEO/GEO intelligence
  // ---------------------------------------------------------------------------
  TARGETS_KEYWORD: 'targetsKeyword',
  KEYWORD_TARGETED_BY: 'keywordTargetedBy',
  TARGETS_GEO: 'targetsGeo',
  GEO_TARGETED_BY: 'geoTargetedBy',
  MONITORS_GEO: 'monitorsGeo',

  // ---------------------------------------------------------------------------
  // SCHEMA-GRAPH (internal - v11.8 ADR-023)
  // ---------------------------------------------------------------------------
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
// - HAS_NATIVE: Entity → EntityNative, Project → ProjectNative, Page → PageNative, Block → BlockNative (v0.13.0)
// - HAS_ENTITY: Project → Entity
// Rather than trying to map every context, we omit the target label and let Neo4j
// return whatever node is connected. This is more flexible and schema-independent.

// =============================================================================
// CYPHER GENERATOR
// =============================================================================

// eslint-disable-next-line @typescript-eslint/no-extraneous-class -- Namespace for Cypher generation utilities
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
    if (criteria.root != null) {
      // If a specific key is provided, match that node
      // Otherwise, match all nodes of the specified type
      if (criteria.root.key != null && criteria.root.key !== '') {
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
      // This handles cases where same relation targets different types (HAS_NATIVE, HAS_ENTITY, etc.)
      const matchLine = `OPTIONAL MATCH (root)${arrow.left}[${relAlias}:${include.relation}]${arrow.right}(${alias}${activeFilter})`;

      lines.push(matchLine);

      // Handle spreading activation for entities (v10.3: USES_ENTITY replaces USES_CONCEPT)
      if (include.relation === 'USES_ENTITY' && include.depth != null && include.depth > 1) {
        const relatedAlias = `related${this.capitalize(alias)}`;
        const spreadRelAlias = `r${relIndex++}`;
        relAliases.push(spreadRelAlias);
        lines.push(`OPTIONAL MATCH (${alias})-[${spreadRelAlias}:SEMANTIC_LINK*1..${include.depth - 1}]->(${relatedAlias}:Entity)`);
        nodeAliases.add(relatedAlias);
      }
    }

    // 3. WHERE clauses
    const whereConditions: string[] = [];

    if (criteria.filters.locale != null && criteria.filters.locale !== '') {
      params.locale = criteria.filters.locale;
    }

    // Node type filtering (byTypes)
    if (criteria.filters.nodeTypes != null && criteria.filters.nodeTypes.length > 0) {
      const typeConditions = criteria.filters.nodeTypes.map(t => `root:${t}`).join(' OR ');
      whereConditions.push(`(${typeConditions})`);
    }

    // Exclude types
    if (criteria.filters.excludeTypes != null && criteria.filters.excludeTypes.length > 0) {
      const excludeConditions = criteria.filters.excludeTypes.map(t => `NOT root:${t}`).join(' AND ');
      whereConditions.push(`(${excludeConditions})`);
    }

    // Fulltext search
    if (criteria.filters.searchQuery != null && criteria.filters.searchQuery !== '') {
      const fields = criteria.filters.searchFields ?? ['key', 'display_name', 'description'];
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
    return RELATION_ALIAS_MAP[relation] ?? relation.toLowerCase();
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
