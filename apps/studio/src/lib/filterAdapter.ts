/**
 * FilterAdapter - novanet-core v10.0.0 compatible filter system
 *
 * Mirrors the NovaNetFilter fluent API and CypherGenerator from novanet-core
 * for use in the visualizer without importing the full library.
 *
 * v10: Uses tiered knowledge model with 10 knowledge nodes.
 */

import type { NodeType, Layer } from '@novanet/core/types';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';
import { NODE_LAYERS } from '@/config/nodeTypes';

// =============================================================================
// TYPES (aligned with novanet-core v9.0.0)
// =============================================================================

export type RelationDirection = 'outgoing' | 'incoming' | 'both';

export interface FilterCriteria {
  nodeTypes?: NodeType[];
  layers?: Layer[];
  excludeTypes?: NodeType[];
  locale?: string;
  localeFamily?: string;
  active?: boolean;
  searchQuery?: string;
  searchFields?: string[];
  maxDepth?: number;
}

export interface IncludeRule {
  relation: string;
  direction: RelationDirection;
  depth?: number;
  targetTypes?: NodeType[];
  filters?: FilterCriteria;
  include?: IncludeRule[];
}

export interface NovaNetFilterCriteria {
  root?: { type: NodeType; key: string };
  includes: IncludeRule[];
  filters: FilterCriteria;
}

export interface CypherQuery {
  query: string;
  params: Record<string, unknown>;
}

// =============================================================================
// RELATION MAPPINGS (v10.0.0 - tiered knowledge model)
// =============================================================================

const RELATION_ALIAS_MAP: Record<string, string> = {
  // Core structure (v11.6)
  HAS_PAGE: 'page',
  HAS_BLOCK: 'block',
  HAS_ENTITY: 'entity',
  // v0.12.4: HAS_BRAND_IDENTITY → HAS_BRAND
  HAS_BRAND: 'brand',
  HAS_PROJECT: 'project',
  OF_TYPE: 'blockType',
  OF_CLASS: 'class', // v0.12.0 ADR-023: was OF_KIND
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
  HAS_AUTHORED_CONTENT: 'authoredContent',
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
  // Metrics
  HAS_METRICS: 'metrics',
  // Legacy mappings
  HAS_ADAPTATION: 'adaptation',
  HAS_STYLE: 'style',
  HAS_TERMS: 'terms',
  HAS_PATTERNS: 'patterns',
  HAS_TABOOS: 'taboos',
  HAS_AUDIENCE: 'audience',
};

// v11.6: Target type labels are now OPTIONAL
// The same relation can target different node types depending on context:
// - HAS_CONTENT: Entity → EntityContent, Project → ProjectContent
// - HAS_ENTITY: Project → Entity
// Rather than trying to map every context, we DON'T specify the target label
// and let Neo4j return whatever node is connected.

// =============================================================================
// NOVANET FILTER (fluent API - mirrors novanet-core/src/filters/NovaNetFilter.ts)
// =============================================================================

export class NovaNetFilter {
  private state: NovaNetFilterCriteria = {
    includes: [],
    filters: {},
  };

  // ---------------------------------------------------------------------------
  // STATIC FACTORY
  // ---------------------------------------------------------------------------

  static create(): NovaNetFilter {
    return new NovaNetFilter();
  }

  // ---------------------------------------------------------------------------
  // ROOT SELECTION
  // ---------------------------------------------------------------------------

  fromPage(key: string): this {
    this.state.root = { type: 'Page', key };
    return this;
  }

  fromBlock(key: string): this {
    this.state.root = { type: 'Block', key };
    return this;
  }

  fromEntity(key: string): this {
    this.state.root = { type: 'Entity', key };
    return this;
  }

  fromLocale(key: string): this {
    this.state.root = { type: 'Locale', key };
    return this;
  }

  fromProject(key: string): this {
    this.state.root = { type: 'Project', key };
    return this;
  }

  fromNode(type: NodeType, key: string): this {
    this.state.root = { type, key };
    return this;
  }

  // ---------------------------------------------------------------------------
  // INCLUDE METHODS
  // ---------------------------------------------------------------------------

  includeBlocks(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'HAS_BLOCK',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  includePages(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'HAS_PAGE',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  includeEntities(opts?: { depth?: number; spreading?: boolean }): this {
    this.state.includes.push({
      relation: 'USES_ENTITY',
      direction: 'outgoing',
      depth: opts?.spreading ? 2 : (opts?.depth ?? 1),
    });
    return this;
  }

  includeProjectEntities(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'HAS_ENTITY',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  /** @deprecated use includeBrand() */
  includeBrandIdentity(): this {
    return this.includeBrand();
  }

  includeBrand(): this {
    this.state.includes.push({
      relation: 'HAS_BRAND', // v0.12.4 ADR-028: was HAS_BRAND_IDENTITY
      direction: 'outgoing',
    });
    return this;
  }

  includeInstructions(opts?: { activeOnly?: boolean }): this {
    const rule: IncludeRule = {
      relation: 'HAS_INSTRUCTION',
      direction: 'outgoing',
    };
    if (opts?.activeOnly) {
      rule.filters = { active: true };
    }
    this.state.includes.push(rule);
    return this;
  }

  includeRules(opts?: { activeOnly?: boolean }): this {
    const rule: IncludeRule = {
      relation: 'HAS_RULES',
      direction: 'outgoing',
    };
    if (opts?.activeOnly) {
      rule.filters = { active: true };
    }
    this.state.includes.push(rule);
    return this;
  }

  includeKnowledge(): this {
    // v10 tiered knowledge arcs
    const knowledgeRelations = [
      // Technical tier
      'HAS_FORMATTING',
      'HAS_SLUGIFICATION',
      'HAS_ADAPTATION',
      // Style tier
      'HAS_STYLE',
      // Semantic tier
      'HAS_TERMS',
      'HAS_EXPRESSIONS',
      'HAS_PATTERNS',
      'HAS_CULTURE',
      'HAS_TABOOS',
      'HAS_AUDIENCE',
    ];
    for (const relation of knowledgeRelations) {
      this.state.includes.push({
        relation,
        direction: 'outgoing',
      });
    }
    return this;
  }

   
  includeOutputs(_opts?: { latestOnly?: boolean }): this {
    this.state.includes.push({
      relation: 'HAS_GENERATED',
      direction: 'outgoing',
    });
    return this;
  }

  includeContent(): this {
    this.state.includes.push({
      relation: 'HAS_CONTENT',
      direction: 'outgoing',
    });
    return this;
  }

  includeSEO(): this {
    this.state.includes.push({
      relation: 'HAS_SEO_TARGET',
      direction: 'outgoing',
    });
    return this;
  }

  includeGEO(): this {
    this.state.includes.push({
      relation: 'HAS_GEO_TARGET',
      direction: 'outgoing',
    });
    return this;
  }

  includeLocales(): this {
    this.state.includes.push({
      relation: 'SUPPORTS_LOCALE',
      direction: 'outgoing',
    });
    return this;
  }

  // ---------------------------------------------------------------------------
  // FILTERING
  // ---------------------------------------------------------------------------

  forLocale(locale: string): this {
    this.state.filters.locale = locale;
    return this;
  }

  forLocaleFamily(family: string): this {
    this.state.filters.localeFamily = family;
    return this;
  }

  byLayer(...layers: Layer[]): this {
    this.state.filters.layers = layers;
    return this;
  }

  byTypes(...types: NodeType[]): this {
    this.state.filters.nodeTypes = types;
    return this;
  }

  excludeTypes(...types: NodeType[]): this {
    this.state.filters.excludeTypes = types;
    return this;
  }

  search(query: string, fields?: string[]): this {
    this.state.filters.searchQuery = query;
    this.state.filters.searchFields = fields;
    return this;
  }

  maxDepth(depth: number): this {
    this.state.filters.maxDepth = depth;
    return this;
  }

  activeOnly(): this {
    this.state.filters.active = true;
    return this;
  }

  // ---------------------------------------------------------------------------
  // OUTPUT
  // ---------------------------------------------------------------------------

  getCriteria(): NovaNetFilterCriteria {
    return { ...this.state };
  }

  /**
   * Get the resolved node types based on layers
   */
  getResolvedNodeTypes(): NodeType[] {
    const types = new Set<NodeType>();

    // Add explicit types
    if (this.state.filters.nodeTypes) {
      this.state.filters.nodeTypes.forEach(t => types.add(t));
    }

    // Add types from layers
    if (this.state.filters.layers) {
      for (const layer of this.state.filters.layers) {
        const layerTypes = NODE_LAYERS[layer];
        if (layerTypes) {
          layerTypes.forEach(t => types.add(t));
        }
      }
    }

    // Remove excluded types
    if (this.state.filters.excludeTypes) {
      this.state.filters.excludeTypes.forEach(t => types.delete(t));
    }

    return Array.from(types);
  }

  /**
   * Generate Cypher query
   */
  toCypher(): CypherQuery {
    return CypherGenerator.generate(this);
  }
}

// =============================================================================
// CYPHER GENERATOR (mirrors novanet-core/src/filters/CypherGenerator.ts)
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
    } else {
      // No root - match all nodes of specified types
      const types = filter.getResolvedNodeTypes();
      if (types.length > 0) {
        const labels = types.join('|');
        lines.push(`MATCH (root) WHERE root:${labels.replace(/\|/g, ' OR root:')}`);
      } else {
        lines.push('MATCH (root)');
      }
    }

    // 2. OPTIONAL MATCH for includes
    // v11.6: Don't specify target type - let Neo4j return whatever is connected
    // This handles cases where same relation targets different types (HAS_CONTENT, HAS_ENTITY, etc.)
    for (const include of criteria.includes) {
      const alias = this.relationToAlias(include.relation);
      aliases.add(alias);

      // Build match line without target type constraint
      let matchLine = `OPTIONAL MATCH (root)-[:${include.relation}]->(${alias})`;

      // Add filter for active prompts/rules
      if (include.filters?.active) {
        matchLine = `OPTIONAL MATCH (root)-[:${include.relation}]->(${alias} {active: true})`;
      }

      lines.push(matchLine);

      // Handle spreading activation for entities
      if ((include.relation === 'USES_ENTITY' || include.relation === 'HAS_ENTITY') && include.depth && include.depth > 1) {
        const relatedAlias = `related${this.capitalize(alias)}`;
        lines.push(`OPTIONAL MATCH (${alias})-[:SEMANTIC_LINK*1..${include.depth - 1}]->(${relatedAlias}:Entity)`);
        aliases.add(relatedAlias);
      }
    }

    // 3. WHERE clauses
    const whereConditions: string[] = [];

    if (criteria.filters.locale) {
      whereConditions.push('(root.locale = $locale OR root:Locale {key: $locale})');
      params.locale = criteria.filters.locale;
    }

    if (criteria.filters.localeFamily) {
      whereConditions.push('root.locale STARTS WITH $localeFamily');
      params.localeFamily = criteria.filters.localeFamily;
    }

    if (criteria.filters.searchQuery) {
      const fields = criteria.filters.searchFields || ['key', 'display_name', 'description'];
      const searchConditions = fields.map(f => `root.${f} CONTAINS $searchQuery`).join(' OR ');
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

    // 5. LIMIT
    if (criteria.filters.maxDepth) {
      lines.push(`LIMIT ${criteria.filters.maxDepth * 100}`);
    } else {
      lines.push(`LIMIT ${DEFAULT_FETCH_LIMIT}`);
    }

    return {
      query: lines.join('\n'),
      params,
    };
  }

  private static relationToAlias(relation: string): string {
    return RELATION_ALIAS_MAP[relation] || relation.toLowerCase().replace('has_', '');
  }

  private static capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
  }
}

// =============================================================================
// PRESET DEFINITIONS (using ViewDefinition-compatible structure) v9.0.0
// =============================================================================

export interface ViewPreset {
  id: string;
  name: string;
  description: string;
  icon: string;
  shortcut?: string;
  filter: () => NovaNetFilter;
}

export const VIEW_PRESETS: ViewPreset[] = [
  {
    id: 'project-structure',
    name: 'Project Structure',
    description: 'Project, Pages, Blocks hierarchy',
    icon: '🏗️',
    shortcut: '1',
    filter: () => NovaNetFilter.create()
      .byLayer('foundation', 'structure', 'semantic')
      .excludeTypes('EntityContent'),
  },
  {
    id: 'generation-chain',
    name: 'Generation Chain',
    description: 'Entities with content outputs',
    icon: '🔗',
    shortcut: '2',
    filter: () => NovaNetFilter.create()
      .byTypes('Entity', 'EntityContent', 'PageGenerated', 'BlockGenerated')
      .byLayer('instruction', 'output'),
  },
  {
    id: 'locale-knowledge',
    name: 'Locale & Knowledge',
    description: 'Locale, geography, and knowledge nodes',
    icon: '🌍',
    shortcut: '3',
    filter: () => NovaNetFilter.create()
      .byLayer('locale', 'geography', 'knowledge'),
  },
  {
    id: 'entity-network',
    name: 'Entity Network',
    description: 'Entities and semantic links',
    icon: '🕸️',
    shortcut: '4',
    filter: () => NovaNetFilter.create()
      .byTypes('Entity', 'EntityContent', 'ExpressionSet'),
  },
  {
    id: 'prompts-rules',
    name: 'Prompts & Rules',
    description: 'AI instructions and validation rules',
    icon: '📝',
    shortcut: '5',
    filter: () => NovaNetFilter.create()
      .byLayer('instruction')
      .byTypes('Page', 'Block'),
  },
  {
    id: 'seo-keywords',
    name: 'SEO & GEO Keywords',
    description: 'Search optimization data (v11.5: in knowledge layer)',
    icon: '🔍',
    shortcut: '6',
    filter: () => NovaNetFilter.create()
      .byTypes('SEOKeyword', 'SEOKeywordMetrics', 'SEOKeywordSet', 'SEOKeywordFormat', 'GEOQuery', 'GEOQuerySet', 'GEOAnswer'),
  },
  {
    id: 'defined-types',
    name: 'Defined Types',
    description: 'Structurally fixed schema definitions',
    icon: '🔒',
    shortcut: '7',
    // v0.12.4: BrandIdentity → Brand, PageStructure/PageInstruction removed
    filter: () => NovaNetFilter.create()
      .byTypes(
        'Locale', 'Project', 'Brand', 'Page', 'Block', 'Entity',
        'BlockType', 'BlockInstruction', 'BlockRules',
      ),
  },
  {
    id: 'authored-content',
    name: 'Authored Content',
    description: 'Human-authored locale-specific content',
    icon: '🌐',
    shortcut: '8',
    filter: () => NovaNetFilter.create()
      .byTypes(
        'ProjectContent', 'EntityContent', 'PageGenerated', 'BlockGenerated',
        'SEOKeyword',
      ),
  },
  {
    id: 'all-nodes',
    name: 'All Nodes',
    description: 'Show everything',
    icon: '🌐',
    shortcut: '0',
    filter: () => NovaNetFilter.create(),
  },
];

/**
 * Get preset by shortcut key
 */
export function getViewPresetByShortcut(shortcut: string): ViewPreset | undefined {
  return VIEW_PRESETS.find(p => p.id === shortcut || p.shortcut === shortcut);
}

