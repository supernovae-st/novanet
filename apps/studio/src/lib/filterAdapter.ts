/**
 * FilterAdapter - novanet-core v8.1.0 compatible filter system
 *
 * Mirrors the NovaNetFilter fluent API and CypherGenerator from novanet-core
 * for use in the visualizer without importing the full library.
 */

import type { NodeType } from '@novanet/core/types';
import { DEFAULT_FETCH_LIMIT } from '@/config/constants';

// =============================================================================
// TYPES (aligned with novanet-core/src/filters/types.ts v8.1.0)
// =============================================================================

export type Priority = 'critical' | 'high' | 'medium' | 'low';
export type Freshness = 'realtime' | 'hourly' | 'daily' | 'static';
export type NodeCategory = 'project' | 'content' | 'locale' | 'generation' | 'seo' | 'geo';
export type RelationDirection = 'outgoing' | 'incoming' | 'both';

/**
 * Node categories with their types (v8.1.0 - 35 nodes across 6 categories)
 */
export const NODE_CATEGORIES: Record<NodeCategory, NodeType[]> = {
  // Project category (3 nodes)
  project: ['Project', 'BrandIdentity', 'ProjectL10n'],
  // Content category (6 nodes)
  content: ['Concept', 'ConceptL10n', 'Page', 'PageType', 'Block', 'BlockType'],
  // Locale category (15 nodes - Locale + 14 LocaleKnowledge)
  locale: [
    'Locale',
    'LocaleIdentity',
    'LocaleVoice',
    'LocaleCulture',
    'LocaleCultureReferences',
    'LocaleMarket',
    'LocaleLexicon',
    'LocaleRulesAdaptation',
    'LocaleRulesFormatting',
    'LocaleRulesSlug',
    'Expression',
    'Reference',
    'Metaphor',
    'Pattern',
    'Constraint',
  ],
  // Generation category (5 nodes)
  generation: ['PagePrompt', 'BlockPrompt', 'BlockRules', 'PageL10n', 'BlockL10n'],
  // SEO category (3 nodes)
  seo: ['SEOKeywordL10n', 'SEOKeywordMetrics', 'SEOMiningRun'],
  // GEO category (3 nodes)
  geo: ['GEOSeedL10n', 'GEOSeedMetrics', 'GEOMiningRun'],
};

export interface FilterCriteria {
  nodeTypes?: NodeType[];
  categories?: NodeCategory[];
  excludeTypes?: NodeType[];
  locale?: string;
  localeFamily?: string;
  priority?: Priority[];
  freshness?: Freshness[];
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
// RELATION MAPPINGS (from novanet-core CypherGenerator v8.1.0)
// =============================================================================

const RELATION_ALIAS_MAP: Record<string, string> = {
  HAS_BLOCK: 'block',
  HAS_PROMPT: 'prompt',
  HAS_RULES: 'rules',
  USES_CONCEPT: 'concept',
  HAS_L10N: 'l10n',
  HAS_OUTPUT: 'output',
  HAS_IDENTITY: 'identity',
  HAS_VOICE: 'voice',
  HAS_CULTURE: 'culture',
  HAS_CULTURE_REFS: 'cultureRefs',
  HAS_MARKET: 'market',
  HAS_LEXICON: 'lexicon',
  HAS_RULES_ADAPTATION: 'adaptationRules',
  HAS_RULES_FORMATTING: 'formattingRules',
  HAS_RULES_SLUG: 'slugRules',
  HAS_SEO_TARGET: 'seoKeyword',
  HAS_GEO_TARGET: 'geoSeed',
  TARGETS_SEO: 'seoKeyword',
  TARGETS_GEO: 'geoSeed',
  HAS_PAGE: 'page',
  HAS_CONCEPT: 'concept',
  SUPPORTS_LOCALE: 'locale',
  FOR_LOCALE: 'locale',
};

const RELATION_TARGET_TYPE_MAP: Record<string, string> = {
  HAS_BLOCK: 'Block',
  HAS_PROMPT: 'PagePrompt',
  HAS_RULES: 'BlockRules',
  USES_CONCEPT: 'Concept',
  HAS_L10N: 'ConceptL10n',
  HAS_OUTPUT: 'PageL10n',
  HAS_IDENTITY: 'LocaleIdentity',
  HAS_VOICE: 'LocaleVoice',
  HAS_CULTURE: 'LocaleCulture',
  HAS_CULTURE_REFS: 'LocaleCultureReferences',
  HAS_MARKET: 'LocaleMarket',
  HAS_LEXICON: 'LocaleLexicon',
  HAS_RULES_ADAPTATION: 'LocaleRulesAdaptation',
  HAS_RULES_FORMATTING: 'LocaleRulesFormatting',
  HAS_RULES_SLUG: 'LocaleRulesSlug',
  HAS_SEO_TARGET: 'SEOKeywordL10n',
  HAS_GEO_TARGET: 'GEOSeedL10n',
  TARGETS_SEO: 'SEOKeywordL10n',
  TARGETS_GEO: 'GEOSeedL10n',
  HAS_PAGE: 'Page',
  HAS_CONCEPT: 'Concept',
  SUPPORTS_LOCALE: 'Locale',
  FOR_LOCALE: 'Locale',
};

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

  fromConcept(key: string): this {
    this.state.root = { type: 'Concept', key };
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

  includeConcepts(opts?: { depth?: number; spreading?: boolean }): this {
    this.state.includes.push({
      relation: 'USES_CONCEPT',
      direction: 'outgoing',
      depth: opts?.spreading ? 2 : (opts?.depth ?? 1),
    });
    return this;
  }

  includePrompts(opts?: { activeOnly?: boolean }): this {
    const rule: IncludeRule = {
      relation: 'HAS_PROMPT',
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
    const knowledgeRelations = [
      'HAS_IDENTITY',
      'HAS_VOICE',
      'HAS_CULTURE',
      'HAS_CULTURE_REFS',
      'HAS_MARKET',
      'HAS_LEXICON',
      'HAS_RULES_ADAPTATION',
      'HAS_RULES_FORMATTING',
      'HAS_RULES_SLUG',
    ];
    for (const relation of knowledgeRelations) {
      this.state.includes.push({
        relation,
        direction: 'outgoing',
      });
    }
    return this;
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars -- reserved for future latestOnly implementation
  includeOutputs(_opts?: { latestOnly?: boolean }): this {
    this.state.includes.push({
      relation: 'HAS_OUTPUT',
      direction: 'outgoing',
    });
    return this;
  }

  includeL10n(): this {
    this.state.includes.push({
      relation: 'HAS_L10N',
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

  withPriority(...priorities: Priority[]): this {
    this.state.filters.priority = priorities;
    return this;
  }

  withFreshness(...freshness: Freshness[]): this {
    this.state.filters.freshness = freshness;
    return this;
  }

  byCategory(...categories: NodeCategory[]): this {
    this.state.filters.categories = categories;
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
   * Get the resolved node types based on categories
   */
  getResolvedNodeTypes(): NodeType[] {
    const types = new Set<NodeType>();

    // Add explicit types
    if (this.state.filters.nodeTypes) {
      this.state.filters.nodeTypes.forEach(t => types.add(t));
    }

    // Add types from categories
    if (this.state.filters.categories) {
      for (const category of this.state.filters.categories) {
        NODE_CATEGORIES[category].forEach(t => types.add(t));
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
    for (const include of criteria.includes) {
      const alias = this.relationToAlias(include.relation);
      aliases.add(alias);

      const targetType = this.relationToTargetType(include.relation);
      let matchLine = `OPTIONAL MATCH (root)-[:${include.relation}]->(${alias}:${targetType})`;

      // Add filter for active prompts/rules
      if (include.filters?.active) {
        matchLine = `OPTIONAL MATCH (root)-[:${include.relation}]->(${alias}:${targetType} {active: true})`;
      }

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

    if (criteria.filters.priority?.length) {
      whereConditions.push('root.priority IN $priorities');
      params.priorities = criteria.filters.priority;
    }

    if (criteria.filters.freshness?.length) {
      whereConditions.push('root.freshness IN $freshness');
      params.freshness = criteria.filters.freshness;
    }

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

  private static relationToTargetType(relation: string): string {
    return RELATION_TARGET_TYPE_MAP[relation] || 'Node';
  }

  private static capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
  }
}

// =============================================================================
// PRESET DEFINITIONS (using ViewDefinition-compatible structure) v8.1.0
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
      .byCategory('project', 'content')
      .excludeTypes('ConceptL10n'),
  },
  {
    id: 'generation-chain',
    name: 'Generation Chain',
    description: 'Concepts with L10n outputs',
    icon: '🔗',
    shortcut: '2',
    filter: () => NovaNetFilter.create()
      .byTypes('Concept', 'ConceptL10n', 'PageL10n', 'BlockL10n')
      .byCategory('generation'),
  },
  {
    id: 'locale-knowledge',
    name: 'Locale Knowledge',
    description: 'Locale with all knowledge nodes',
    icon: '🌍',
    shortcut: '3',
    filter: () => NovaNetFilter.create()
      .byCategory('locale'),
  },
  {
    id: 'concept-network',
    name: 'Concept Network',
    description: 'Concepts and semantic links',
    icon: '🕸️',
    shortcut: '4',
    filter: () => NovaNetFilter.create()
      .byTypes('Concept', 'ConceptL10n', 'Expression'),
  },
  {
    id: 'prompts-rules',
    name: 'Prompts & Rules',
    description: 'AI instructions and validation rules',
    icon: '📝',
    shortcut: '5',
    filter: () => NovaNetFilter.create()
      .byCategory('generation')
      .byTypes('Page', 'Block'),
  },
  {
    id: 'seo-geo',
    name: 'SEO & GEO',
    description: 'Search optimization data',
    icon: '🔍',
    shortcut: '6',
    filter: () => NovaNetFilter.create()
      .byCategory('seo', 'geo'),
  },
  {
    id: 'high-priority',
    name: 'High Priority',
    description: 'Critical and high priority nodes',
    icon: '🔴',
    shortcut: '7',
    filter: () => NovaNetFilter.create()
      .withPriority('critical', 'high'),
  },
  {
    id: 'realtime',
    name: 'Realtime Content',
    description: 'Nodes requiring frequent updates',
    icon: '⚡',
    shortcut: '8',
    filter: () => NovaNetFilter.create()
      .withFreshness('realtime', 'hourly'),
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
