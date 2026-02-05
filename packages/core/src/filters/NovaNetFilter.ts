// src/filters/NovaNetFilter.ts
import type {
  NodeType,
  FilterCriteria,
  IncludeRule,
} from './types.js';

/**
 * State for the NovaNetFilter builder.
 * Exported for type-safe testing and extensions.
 */
export interface FilterState {
  root?: { type: NodeType; key: string };
  includes: IncludeRule[];
  filters: FilterCriteria;
}

/**
 * NovaNetFilter - Fluent API for building Neo4j filter criteria.
 *
 * Provides a chainable builder pattern for constructing filter criteria
 * that can be converted to Cypher queries via CypherGenerator.
 *
 * @example
 * ```typescript
 * const filter = NovaNetFilter.create()
 *   .fromPage('page-pricing')
 *   .includeBlocks()
 *   .includeEntities({ spreading: true })
 *   .includePrompts({ activeOnly: true })
 *   .forLocale('fr-FR')
 *   .maxDepth(2);
 *
 * const { query, params } = CypherGenerator.generate(filter);
 * ```
 */
export class NovaNetFilter {
  private state: FilterState = {
    includes: [],
    filters: {},
  };

  // =============================================================================
  // STATIC FACTORY
  // =============================================================================

  /**
   * Creates a new NovaNetFilter instance.
   *
   * @returns A new NovaNetFilter ready for method chaining
   */
  static create(): NovaNetFilter {
    return new NovaNetFilter();
  }

  // =============================================================================
  // ROOT SELECTION
  // =============================================================================

  /**
   * Sets the root node to a Page.
   * @param key - The Page key to query from
   */
  fromPage(key: string): this {
    this.state.root = { type: 'Page', key };
    return this;
  }

  /**
   * Sets the root node to a Block.
   * @param key - The Block key to query from
   */
  fromBlock(key: string): this {
    this.state.root = { type: 'Block', key };
    return this;
  }

  /**
   * Sets the root node to an Entity (v10.3).
   * @param key - The Entity key to query from
   */
  fromEntity(key: string): this {
    this.state.root = { type: 'Entity', key };
    return this;
  }

  /**
   * Sets the root node to a Locale.
   * @param key - The Locale key (e.g., 'fr-FR', 'en-US')
   */
  fromLocale(key: string): this {
    this.state.root = { type: 'Locale', key };
    return this;
  }

  /**
   * Sets the root node to a Project.
   * @param key - The Project key to query from
   */
  fromProject(key: string): this {
    this.state.root = { type: 'Project', key };
    return this;
  }

  // =============================================================================
  // INCLUDE METHODS
  // =============================================================================

  /**
   * Includes Block nodes related to the root via HAS_BLOCK.
   * @param opts - Optional configuration
   * @param opts.depth - Traversal depth (default: 1)
   */
  includeBlocks(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'HAS_BLOCK',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  /**
   * Includes Entity nodes related to the root via USES_ENTITY (v10.3).
   * @param opts - Optional configuration
   * @param opts.depth - Traversal depth (default: 1)
   * @param opts.spreading - Enable spreading activation (sets depth to 2)
   */
  includeEntities(opts?: { depth?: number; spreading?: boolean }): this {
    this.state.includes.push({
      relation: 'USES_ENTITY',
      direction: 'outgoing',
      depth: opts?.spreading ? 2 : (opts?.depth ?? 1),
    });
    return this;
  }

  /**
   * Includes PagePrompt/BlockPrompt nodes via HAS_PROMPT.
   * @param opts - Optional configuration
   * @param opts.activeOnly - Only include active prompts
   */
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

  /**
   * Includes BlockRules nodes via HAS_RULES.
   * @param opts - Optional configuration
   * @param opts.activeOnly - Only include active rules
   */
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

  /**
   * Includes all locale knowledge nodes (Identity, Voice, Culture, Market, Lexicon).
   * Typically used with fromLocale().
   * Safe to call multiple times - prevents duplicate relations.
   */
  includeKnowledge(): this {
    const knowledgeRelations = ['HAS_IDENTITY', 'HAS_VOICE', 'HAS_CULTURE', 'HAS_MARKET', 'HAS_LEXICON'];
    // Prevent duplicates if called multiple times
    const hasKnowledge = this.state.includes.some(i => knowledgeRelations.includes(i.relation));
    if (!hasKnowledge) {
      for (const relation of knowledgeRelations) {
        this.state.includes.push({
          relation,
          direction: 'outgoing',
        });
      }
    }
    return this;
  }

  /**
   * Includes PageL10n/BlockL10n nodes via HAS_OUTPUT.
   * @param _opts - Reserved for future options (e.g., latestOnly)
   */
  includeOutputs(_opts?: { latestOnly?: boolean }): this {
    this.state.includes.push({
      relation: 'HAS_OUTPUT',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes localized content nodes via HAS_L10N.
   */
  includeL10n(): this {
    this.state.includes.push({
      relation: 'HAS_L10N',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes SEO keyword nodes via EXPRESSES (v10.3: replaces TARGETS_SEO).
   */
  includeSEO(): this {
    this.state.includes.push({
      relation: 'EXPRESSES',
      direction: 'outgoing',
    });
    return this;
  }

  // REMOVED v10.3: includeGEO() - GEO layer removed

  /**
   * Includes BlockType node via OF_TYPE.
   * Typically used with fromBlock().
   */
  includeBlockType(): this {
    this.state.includes.push({
      relation: 'OF_TYPE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes semantic links between Concepts via SEMANTIC_LINK.
   * @param opts - Optional configuration
   * @param opts.depth - Traversal depth for spreading activation (default: 1)
   */
  includeSemanticLinks(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'SEMANTIC_LINK',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  /**
   * Includes Page nodes via HAS_PAGE.
   * Typically used with fromProject().
   */
  includePages(): this {
    this.state.includes.push({
      relation: 'HAS_PAGE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes BrandIdentity node via HAS_BRAND_IDENTITY.
   * Typically used with fromProject().
   */
  includeBrandIdentity(): this {
    this.state.includes.push({
      relation: 'HAS_BRAND_IDENTITY',
      direction: 'outgoing',
    });
    return this;
  }

  // REMOVED v10.3: includeProjectConcepts() - HAS_CONCEPT arc removed
  // Entity is now in global realm, accessed via USES_ENTITY from Page/Block

  /**
   * Includes localized content via FOR_LOCALE relation.
   * Used to connect L10n nodes to their Locale.
   */
  includeForLocale(): this {
    this.state.includes.push({
      relation: 'FOR_LOCALE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes supported locales via SUPPORTS_LOCALE.
   * Typically used with fromProject().
   */
  includeSupportedLocales(): this {
    this.state.includes.push({
      relation: 'SUPPORTS_LOCALE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes default locale via DEFAULT_LOCALE.
   * Typically used with fromProject().
   */
  includeDefaultLocale(): this {
    this.state.includes.push({
      relation: 'DEFAULT_LOCALE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes fallback locale via FALLBACK_TO.
   * Typically used with fromLocale().
   */
  includeFallbackLocale(): this {
    this.state.includes.push({
      relation: 'FALLBACK_TO',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes metrics via HAS_METRICS.
   * Used for SEOKeywordMetrics (v10.3: GEO removed).
   */
  includeMetrics(): this {
    this.state.includes.push({
      relation: 'HAS_METRICS',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes page links via LINKS_TO.
   * Typically used with fromPage().
   */
  includePageLinks(): this {
    this.state.includes.push({
      relation: 'LINKS_TO',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes subtopic hierarchy via SUBTOPIC_OF.
   * Used for pillar-cluster page relationships.
   */
  includeSubtopics(): this {
    this.state.includes.push({
      relation: 'SUBTOPIC_OF',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes all locale rules nodes (Adaptation, Formatting, Slug).
   * Typically used with fromLocale().
   */
  includeLocaleRules(): this {
    const rulesRelations = ['HAS_RULES_ADAPTATION', 'HAS_RULES_FORMATTING', 'HAS_RULES_SLUG'];
    for (const relation of rulesRelations) {
      this.state.includes.push({
        relation,
        direction: 'outgoing',
      });
    }
    return this;
  }

  /**
   * Includes lexicon details (Expression nodes) via HAS_EXPRESSION.
   * Typically used after includeKnowledge() with a Locale root.
   */
  includeExpressions(): this {
    this.state.includes.push({
      relation: 'HAS_EXPRESSION',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes culture references via HAS_REFERENCE.
   */
  includeCultureReferences(): this {
    this.state.includes.push({
      relation: 'HAS_REFERENCE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes metaphors via HAS_METAPHOR.
   */
  includeMetaphors(): this {
    this.state.includes.push({
      relation: 'HAS_METAPHOR',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes formatting patterns via HAS_PATTERN.
   */
  includePatterns(): this {
    this.state.includes.push({
      relation: 'HAS_PATTERN',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes cultural constraints via HAS_CONSTRAINT.
   */
  includeConstraints(): this {
    this.state.includes.push({
      relation: 'HAS_CONSTRAINT',
      direction: 'outgoing',
    });
    return this;
  }

  // =============================================================================
  // FILTERING
  // =============================================================================

  /**
   * Filters results by locale.
   * @param locale - BCP 47 locale code (e.g., 'fr-FR', 'en-US')
   */
  forLocale(locale: string): this {
    this.state.filters.locale = locale;
    return this;
  }

  /**
   * Filters by specific node types.
   * @param types - One or more node types to include
   */
  byTypes(...types: NodeType[]): this {
    this.state.filters.nodeTypes = types;
    return this;
  }

  /**
   * Excludes specific node types from results.
   * @param types - One or more node types to exclude
   */
  excludeTypes(...types: NodeType[]): this {
    this.state.filters.excludeTypes = types;
    return this;
  }

  /**
   * Adds a fulltext search filter.
   * @param query - Search query string
   * @param fields - Optional specific fields to search in
   */
  search(query: string, fields?: string[]): this {
    this.state.filters.searchQuery = query;
    this.state.filters.searchFields = fields;
    return this;
  }

  /**
   * Sets the maximum traversal depth.
   * @param depth - Maximum depth for relationship traversal
   */
  maxDepth(depth: number): this {
    this.state.filters.maxDepth = depth;
    return this;
  }

  // =============================================================================
  // OUTPUT
  // =============================================================================

  /**
   * Returns the current filter state.
   * Used by CypherGenerator to build the Cypher query.
   * @returns A copy of the internal filter state
   */
  getCriteria(): FilterState {
    return { ...this.state };
  }
}
