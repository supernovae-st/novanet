// src/filters/NovaNetFilter.ts
// v0.13.0 ADR-029: Added includeNative() for unified *Native pattern
// Deprecated: includeContent(), includeOutputs() - use includeNative()
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
 * // v0.13.0: Use includeNative() for all *Native nodes
 * const filter = NovaNetFilter.create()
 *   .fromPage('page-pricing')
 *   .includeBlocks()
 *   .includeNative()   // EntityNative, ProjectNative, PageNative, BlockNative
 *   .includeEntities({ spreading: true })
 *   .includeInstructions({ activeOnly: true })
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
   * Includes Entity nodes related to Project via HAS_ENTITY (v11.6).
   * Used when Project directly owns entities (vs Page/Block using entities).
   * @param opts - Optional configuration
   * @param opts.depth - Traversal depth (default: 1)
   */
  includeProjectEntities(opts?: { depth?: number }): this {
    this.state.includes.push({
      relation: 'HAS_ENTITY',
      direction: 'outgoing',
      depth: opts?.depth ?? 1,
    });
    return this;
  }

  /**
   * Includes BlockInstruction nodes via HAS_INSTRUCTION.
   * v0.12.4: PageInstruction deleted (ADR-028) - page instructions composed from BlockInstructions
   * @param opts - Optional configuration
   * @param opts.activeOnly - Only include active instructions
   */
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
   * Includes all locale knowledge nodes (Culture, Market, Formatting, Slugification, ExpressionSet).
   * Typically used with fromLocale().
   * Safe to call multiple times - prevents duplicate relations.
   * v11.5: Updated to match current schema.
   */
  includeKnowledge(): this {
    const knowledgeRelations = ['HAS_CULTURE', 'HAS_MARKET', 'HAS_FORMATTING', 'HAS_SLUGIFICATION', 'HAS_EXPRESSIONS'];
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
   * Includes *Native nodes (EntityNative, ProjectNative, PageNative, BlockNative) via HAS_NATIVE.
   * v0.13.0 ADR-029: Unified method replacing includeOutputs() and includeContent()
   * @param _opts - Reserved for future options (e.g., latestOnly)
   */
  includeNative(_opts?: { latestOnly?: boolean }): this {
    this.state.includes.push({
      relation: 'HAS_NATIVE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes parent structure node (Entity, Project, Page, Block) via NATIVE_OF.
   * v0.13.0 ADR-029: Inverse of includeNative(), used when starting from *Native nodes.
   * Typically used with fromEntityNative(), fromPageNative(), etc.
   */
  includeNativeParent(): this {
    this.state.includes.push({
      relation: 'NATIVE_OF',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes PageNative/BlockNative nodes via HAS_NATIVE.
   * @deprecated Use includeNative() instead (v0.13.0 ADR-029)
   * @param _opts - Reserved for future options (e.g., latestOnly)
   */
  includeOutputs(_opts?: { latestOnly?: boolean }): this {
    this.state.includes.push({
      relation: 'HAS_NATIVE',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes EntityNative/ProjectNative nodes via HAS_NATIVE.
   * @deprecated Use includeNative() instead (v0.13.0 ADR-029)
   * v11.6: renamed from includeL10n() (ADR-014)
   */
  includeContent(): this {
    this.state.includes.push({
      relation: 'HAS_NATIVE',
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
   * Includes Brand node via HAS_BRAND.
   * Typically used with fromProject().
   * v0.12.4: ADR-028 Brand Architecture
   */
  includeBrand(): this {
    this.state.includes.push({
      relation: 'HAS_BRAND',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes BrandDesign node via HAS_DESIGN.
   * Typically used after includeBrand().
   * v0.12.4: new method (ADR-028)
   */
  includeDesign(): this {
    this.state.includes.push({
      relation: 'HAS_DESIGN',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes BrandPrinciples node via HAS_PRINCIPLES.
   * Typically used after includeBrand().
   * v0.12.4: new method (ADR-028)
   */
  includePrinciples(): this {
    this.state.includes.push({
      relation: 'HAS_PRINCIPLES',
      direction: 'outgoing',
    });
    return this;
  }

  /**
   * Includes PromptStyle nodes via HAS_PROMPT_STYLE.
   * Typically used after includeBrand().
   * v0.12.4: new method (ADR-028)
   */
  includePromptStyle(): this {
    this.state.includes.push({
      relation: 'HAS_PROMPT_STYLE',
      direction: 'outgoing',
    });
    return this;
  }

  // REMOVED v10.3: includeProjectConcepts() - HAS_CONCEPT arc removed
  // Entity is now in shared realm, accessed via USES_ENTITY from Page/Block

  /**
   * Includes *Native nodes via FOR_LOCALE relation.
   * v0.13.0 ADR-029: Used to connect *Native nodes to their Locale.
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
