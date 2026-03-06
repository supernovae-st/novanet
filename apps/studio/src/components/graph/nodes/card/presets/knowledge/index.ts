/**
 * Knowledge Layer Card Components
 *
 * Layer color: Purple #8b5cf6
 *
 * Knowledge layer contains:
 * - Containers (6): TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 * - Atoms (6): Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
 * - SEO (5): SEOKeyword, SEOKeywordMetrics, SEOCluster, SEOPillar, SEOKeywordFormat
 * - GEO (2): GEOQuery, GEOAnswer
 *
 * Trait: imported (double borders) for most nodes
 *
 * All components are performance-aware and support Motion animations.
 */

// Helper components
export {
  DomainBadge,
  type DomainBadgeProps,
  RegisterBadge,
  type RegisterBadgeProps,
  ToneBadge,
  type ToneBadgeProps,
  FormalityIndicator,
  type FormalityIndicatorProps,
  UseCaseBadge,
  type UseCaseBadgeProps,
  VolumeDisplay,
  type VolumeDisplayProps,
  DifficultyBadge,
  type DifficultyBadgeProps,
  TrafficPotential,
  type TrafficPotentialProps,
  IntentBadge,
  type IntentBadgeProps,
  SerpFeatures,
  type SerpFeaturesProps,
  TrendBadge,
  type TrendBadgeProps,
  SynonymsList,
  type SynonymsListProps,
  ChannelBadges,
  type ChannelBadgesProps,
  PartOfSpeechBadge,
  type PartOfSpeechBadgeProps,
} from './KnowledgeHelpers';

// Card content components - Atoms
export {
  TermCardContent,
  type TermCardContentProps,
  type TermNodeData,
} from './TermCardContent';

export {
  ExpressionCardContent,
  type ExpressionCardContentProps,
  type ExpressionNodeData,
} from './ExpressionCardContent';

export {
  PatternCardContent,
  type PatternCardContentProps,
  type PatternNodeData,
  type PatternTaxonomyProps,
} from './PatternCardContent';

export {
  CultureRefCardContent,
  type CultureRefCardContentProps,
  type CultureRefNodeData,
  type CultureRefTaxonomyProps,
  type CultureCategory,
  type SensitivityLevel,
} from './CultureRefCardContent';

export {
  TabooCardContent,
  type TabooCardContentProps,
  type TabooNodeData,
  type TabooTaxonomyProps,
  type TabooCategory,
  type TabooSeverity,
} from './TabooCardContent';

export {
  AudienceTraitCardContent,
  type AudienceTraitCardContentProps,
  type AudienceTraitNodeData,
  type AudienceTraitTaxonomyProps,
  type TraitCategory,
  type ImportanceLevel,
} from './AudienceTraitCardContent';

// Card content components - Containers (Sets)
export {
  KnowledgeSetCardContent,
  type KnowledgeSetCardContentProps,
  type KnowledgeSetNodeData,
  type KnowledgeSetTaxonomyProps,
  type KnowledgeSetType,
} from './KnowledgeSetCardContent';

// Card content components - SEO
export {
  SEOKeywordCardContent,
  type SEOKeywordCardContentProps,
  type SEOKeywordNodeData,
} from './SEOKeywordCardContent';
