/**
 * Config Layer Card Components
 *
 * Specialized cards for shared/config layer nodes:
 * - EntityCategory: Classification system for Entity nodes (ADR-017)
 * - Locale: BCP-47 locale definitions (handled by LocaleCardContent)
 * - SEOKeywordFormat: SEO keyword formatting rules
 */

export {
  EntityCategoryCardContent,
  type EntityCategoryCardContentProps,
  type EntityCategoryNodeData,
  type EntityCategoryTaxonomyProps,
} from './EntityCategoryCardContent';
