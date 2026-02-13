// src/filters/ViewLoader.ts
// v11.6.1: Unified view system - all views from _registry.yaml with embedded Cypher

import { promises as fs } from 'fs';
import path from 'path';
import yaml from 'js-yaml';
import type { ViewDefinition, ViewRegistry, ViewRegistryEntry, IncludeRule, CypherQuery } from './types.js';
import { NovaNetFilter } from './NovaNetFilter.js';


// ─────────────────────────────────────────────────────────────────────────────
// YAML Registry Loading
// ─────────────────────────────────────────────────────────────────────────────

interface YAMLRegistry {
  version: string;
  description?: string;
  views: ViewRegistryEntry[];
}

let registryCache: YAMLRegistry | null = null;

/**
 * Load the unified _registry.yaml file.
 */
async function loadRegistryYAML(): Promise<YAMLRegistry> {
  if (registryCache) return registryCache;

  // Try multiple paths (for different execution contexts)
  const possiblePaths = [
    path.join(process.cwd(), 'packages/core/models/views/_registry.yaml'),
    path.join(process.cwd(), '../../packages/core/models/views/_registry.yaml'),
    path.join(process.cwd(), '../../../packages/core/models/views/_registry.yaml'),
  ];

  for (const registryPath of possiblePaths) {
    try {
      const content = await fs.readFile(registryPath, 'utf-8');
      registryCache = yaml.load(content) as YAMLRegistry;
      return registryCache;
    } catch {
      // Try next path
    }
  }

  throw new Error('Could not load _registry.yaml from any known path');
}


// ─────────────────────────────────────────────────────────────────────────────
// ViewLoader
// ─────────────────────────────────────────────────────────────────────────────

/**
 * ViewLoader - Loads view definitions from _registry.yaml (unified view system).
 *
 * v11.6.1: All views defined in _registry.yaml with embedded Cypher queries.
 * No more separate YAML files or generated TypeScript.
 *
 * Usage:
 * ```typescript
 * const registry = await ViewLoader.loadRegistry();
 * const view = ViewLoader.getViewById(registry, 'data-complete');
 * if (view.cypher) {
 *   // Execute cypher directly
 * }
 * ```
 */
export class ViewLoader {
  /**
   * Loads the unified view registry from _registry.yaml.
   *
   * @returns The view registry with all views and their Cypher queries
   */
  static async loadRegistry(): Promise<ViewRegistry> {
    const registry = await loadRegistryYAML();
    return {
      version: registry.version,
      description: registry.description || 'NovaNet Unified View System',
      views: registry.views,
    };
  }

  /**
   * Get a view by ID from the registry.
   *
   * @param viewId - The view identifier
   * @returns The view entry or undefined
   */
  static async getViewById(viewId: string): Promise<ViewRegistryEntry | undefined> {
    const registry = await loadRegistryYAML();
    return registry.views.find(v => v.id === viewId);
  }

  /**
   * Get Cypher query for a view with parameter substitution.
   *
   * @param viewId - The view identifier
   * @param params - Parameters to substitute (nodeKey, locale, etc.)
   * @returns CypherQuery with query and params
   */
  static async getCypher(viewId: string, params: Record<string, unknown> = {}): Promise<CypherQuery> {
    const view = await this.getViewById(viewId);
    if (!view) {
      throw new Error(`View '${viewId}' not found`);
    }
    if (!view.cypher) {
      throw new Error(`View '${viewId}' has no Cypher query`);
    }

    return {
      query: view.cypher.trim(),
      params,
    };
  }


  /**
   * Get all available view IDs.
   */
  static async getViewIds(): Promise<string[]> {
    const registry = await loadRegistryYAML();
    return registry.views.map(v => v.id);
  }

  /**
   * Check if a view exists.
   */
  static async hasView(viewId: string): Promise<boolean> {
    const view = await this.getViewById(viewId);
    return view !== undefined;
  }

  /**
   * Get views for a specific node type (contextual views).
   *
   * @param nodeType - The node type to filter by
   * @returns Array of applicable views
   */
  static async getViewsForNodeType(nodeType: string): Promise<ViewRegistryEntry[]> {
    const registry = await loadRegistryYAML();
    return registry.views.filter(v =>
      v.contextual &&
      v.applicable_types &&
      (v.applicable_types.length === 0 || v.applicable_types.includes(nodeType))
    );
  }

  /**
   * Converts a ViewDefinition to a NovaNetFilter instance.
   *
   * @param view - The ViewDefinition to convert
   * @param params - Optional parameters (key for root node, locale for filtering)
   * @returns A configured NovaNetFilter instance
   */
  static toFilter(
    view: ViewDefinition,
    params: { key?: string; locale?: string } = {}
  ): NovaNetFilter {
    const filter = NovaNetFilter.create();

    // Set root node
    const rootKey = params.key || view.root.key || '';
    switch (view.root.type) {
      case 'Page':
        filter.fromPage(rootKey);
        break;
      case 'Block':
        filter.fromBlock(rootKey);
        break;
      case 'Entity':
        filter.fromEntity(rootKey);
        break;
      case 'Locale':
        filter.fromLocale(rootKey);
        break;
      case 'Project':
        filter.fromProject(rootKey);
        break;
      default:
        throw new Error(`Unsupported root type: ${view.root.type}`);
    }

    // Apply include rules
    for (const include of view.include) {
      this.applyInclude(filter, include);
    }

    // Apply filters
    if (view.filters) {
      // Handle locale - support $locale placeholder
      if (view.filters.locale === '$locale' && params.locale) {
        filter.forLocale(params.locale);
      } else if (view.filters.locale && view.filters.locale !== '$locale') {
        filter.forLocale(view.filters.locale);
      }

      // Handle maxDepth
      if (view.filters.maxDepth !== undefined) {
        filter.maxDepth(view.filters.maxDepth);
      }
    }

    return filter;
  }

  /**
   * Applies a single include rule to a NovaNetFilter.
   */
  private static applyInclude(filter: NovaNetFilter, include: IncludeRule): void {
    const activeOnly = include.filters?.active === true;

    switch (include.relation) {
      case 'HAS_BLOCK':
        filter.includeBlocks({ depth: include.depth });
        break;

      case 'USES_ENTITY':
        filter.includeEntities({
          depth: include.depth,
          spreading: (include.depth || 1) > 1
        });
        break;

      case 'HAS_INSTRUCTION':
        filter.includeInstructions({ activeOnly });
        break;

      case 'HAS_RULES':
        filter.includeRules({ activeOnly });
        break;

      case 'HAS_GENERATED':
        filter.includeOutputs();
        break;

      case 'HAS_CONTENT':
        filter.includeContent();
        break;

      // v11.5: Locale knowledge relations
      case 'HAS_CULTURE':
      case 'HAS_MARKET':
      case 'HAS_FORMATTING':
      case 'HAS_SLUGIFICATION':
      case 'HAS_EXPRESSIONS': {
        const criteria = filter.getCriteria();
        const knowledgeRelations = ['HAS_CULTURE', 'HAS_MARKET', 'HAS_FORMATTING', 'HAS_SLUGIFICATION', 'HAS_EXPRESSIONS'];
        const hasKnowledge = criteria.includes.some(i => knowledgeRelations.includes(i.relation));
        if (!hasKnowledge) {
          filter.includeKnowledge();
        }
        break;
      }

      case 'EXPRESSES':
        filter.includeSEO();
        break;

      case 'OF_TYPE':
        filter.includeBlockType();
        break;

      case 'SEMANTIC_LINK':
        filter.includeSemanticLinks({ depth: include.depth });
        break;

      case 'HAS_PAGE':
        filter.includePages();
        break;

      case 'HAS_ENTITY':
        filter.includeProjectEntities({ depth: include.depth });
        break;

      case 'HAS_BRAND_IDENTITY':
        filter.includeBrandIdentity();
        break;

      case 'FOR_LOCALE':
        filter.includeForLocale();
        break;

      case 'SUPPORTS_LOCALE':
        filter.includeSupportedLocales();
        break;

      case 'DEFAULT_LOCALE':
        filter.includeDefaultLocale();
        break;

      case 'FALLBACK_TO':
        filter.includeFallbackLocale();
        break;

      case 'HAS_METRICS':
        filter.includeMetrics();
        break;

      case 'LINKS_TO':
        filter.includePageLinks();
        break;

      case 'SUBTOPIC_OF':
        filter.includeSubtopics();
        break;

      case 'HAS_RULES_ADAPTATION':
      case 'HAS_RULES_FORMATTING':
      case 'HAS_RULES_SLUG':
        filter.includeLocaleRules();
        break;

      case 'HAS_EXPRESSION':
        filter.includeExpressions();
        break;

      case 'HAS_REFERENCE':
        filter.includeCultureReferences();
        break;

      case 'HAS_METAPHOR':
        filter.includeMetaphors();
        break;

      case 'HAS_PATTERN':
        filter.includePatterns();
        break;

      case 'HAS_CONSTRAINT':
        filter.includeConstraints();
        break;

      default:
        // v11.6: Pass through unknown relations directly
        // This makes views more flexible and schema-independent
        filter.getCriteria().includes.push({
          relation: include.relation,
          direction: include.direction || 'outgoing',
          depth: include.depth,
          filters: include.filters,
        });
        break;
    }
  }
}
