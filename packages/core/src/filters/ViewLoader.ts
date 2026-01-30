// src/filters/ViewLoader.ts
import * as fs from 'fs/promises';
import * as path from 'path';
import { parse as parseYaml } from 'yaml';
import type { ViewDefinition, ViewRegistry, IncludeRule } from './types.js';
import type { Priority } from '../types/index.js';
import { NovaNetFilter } from './NovaNetFilter.js';

/**
 * ViewLoader - Loads YAML view definitions and converts them to NovaNetFilter instances.
 *
 * Usage:
 * ```typescript
 * const view = await ViewLoader.loadView('page-generation-context', viewsDir);
 * const filter = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });
 * const { query, params } = CypherGenerator.generate(filter);
 * ```
 */
export class ViewLoader {
  /**
   * Loads a single view definition from YAML file.
   *
   * @param viewId - The view identifier (without .yaml extension)
   * @param viewsDir - Path to the views directory
   * @returns The parsed ViewDefinition
   * @throws Error if file does not exist or is invalid YAML
   */
  static async loadView(viewId: string, viewsDir: string): Promise<ViewDefinition> {
    const filePath = path.join(viewsDir, `${viewId}.yaml`);
    const content = await fs.readFile(filePath, 'utf-8');
    return parseYaml(content) as ViewDefinition;
  }

  /**
   * Loads the view registry from _registry.yaml.
   *
   * @param viewsDir - Path to the views directory
   * @returns The parsed ViewRegistry
   */
  static async loadRegistry(viewsDir: string): Promise<ViewRegistry> {
    const filePath = path.join(viewsDir, '_registry.yaml');
    const content = await fs.readFile(filePath, 'utf-8');
    return parseYaml(content) as ViewRegistry;
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
      case 'Concept':
        filter.fromConcept(rootKey);
        break;
      case 'Locale':
        filter.fromLocale(rootKey);
        break;
      case 'Project':
        filter.fromProject(rootKey);
        break;
      default:
        // For other types, we'd need to extend NovaNetFilter
        // For now, throw an error
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

      // Handle priority
      if (view.filters.priority && view.filters.priority.length > 0) {
        filter.withPriority(...(view.filters.priority as Priority[]));
      }

      // Handle maxDepth
      if (view.filters.maxDepth !== undefined) {
        filter.maxDepth(view.filters.maxDepth);
      }

      // Handle categories
      if (view.filters.categories && view.filters.categories.length > 0) {
        filter.byCategory(...view.filters.categories);
      }

      // Handle freshness
      if (view.filters.freshness && view.filters.freshness.length > 0) {
        filter.withFreshness(...view.filters.freshness);
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

      case 'USES_CONCEPT':
        filter.includeConcepts({
          depth: include.depth,
          spreading: (include.depth || 1) > 1
        });
        break;

      case 'HAS_PROMPT':
        filter.includePrompts({ activeOnly });
        break;

      case 'HAS_RULES':
        filter.includeRules({ activeOnly });
        break;

      case 'HAS_OUTPUT':
        filter.includeOutputs();
        break;

      case 'HAS_L10N':
        filter.includeL10n();
        break;

      case 'HAS_IDENTITY':
      case 'HAS_VOICE':
      case 'HAS_CULTURE':
      case 'HAS_MARKET':
      case 'HAS_LEXICON': {
        // All knowledge relations use includeKnowledge()
        // Only call once for first knowledge relation encountered
        const criteria = filter.getCriteria();
        const hasKnowledge = criteria.includes.some(i =>
          ['HAS_IDENTITY', 'HAS_VOICE', 'HAS_CULTURE', 'HAS_MARKET', 'HAS_LEXICON'].includes(i.relation)
        );
        if (!hasKnowledge) {
          filter.includeKnowledge();
        }
        break;
      }

      case 'TARGETS_SEO':
        filter.includeSEO();
        break;

      case 'TARGETS_GEO':
        filter.includeGEO();
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

      case 'HAS_BRAND_IDENTITY':
        filter.includeBrandIdentity();
        break;

      case 'HAS_CONCEPT':
        filter.includeProjectConcepts({ depth: include.depth });
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
        // For unknown relations, log a warning but don't fail
        console.warn(`Unknown relation in view include: ${include.relation}`);
    }
  }
}
