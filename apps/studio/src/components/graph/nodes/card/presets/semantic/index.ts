/**
 * Semantic Layer Card Components
 *
 * Layer color: Orange #f97316
 * 4 nodes: Entity, EntityNative, AudiencePersona, ChannelSurface
 *
 * All components are performance-aware and support Framer Motion animations.
 */

// Helper components
export {
  PillarBadge,
  type PillarBadgeProps,
  SchemaOrgBadge,
  type SchemaOrgBadgeProps,
  CurationBadge,
  type CurationBadgeProps,
  LocaleBadge,
  type LocaleBadgeProps,
  LocaleFlagRibbon,
  type LocaleFlagRibbonProps,
  getLocaleFlag,
  getLocaleAccentColor,
  SemanticLinkCounter,
  type SemanticLinkCounterProps,
  StatusBadge,
  type StatusBadgeProps,
  BenefitsList,
  type BenefitsListProps,
  ContentStats,
  type ContentStatsProps,
} from './SemanticHelpers';

// Card content components
export {
  EntityCardContent,
  type EntityCardContentProps,
  type EntityNodeData,
} from './EntityCardContent';

export {
  EntityNativeCardContent,
  type EntityNativeCardContentProps,
  type EntityNativeNodeData,
} from './EntityNativeCardContent';
