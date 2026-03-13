/**
 * Card Component System
 *
 * Unified card wrapper and content components for graph nodes.
 *
 * 3-Level Architecture:
 * - Level 1: Taxonomy (21 nodes) - Realm, Layer, Trait, ArcFamily
 * - Level 2: Schema (202 nodes) - 57 NodeClass + 145 ArcClass (v0.18.0)
 * - Level 3: Data (∞ instances) - Runtime instances per layer
 *
 * Visual Encoding (ADR-005):
 * - Fill color → Layer
 * - Border color → Realm
 * - Border style → Trait
 *
 * Usage:
 * ```tsx
 * import { CardShell, StructuralCardContent, type CardContext } from './card';
 *
 * const MyNode = (props) => (
 *   <CardShell
 *     colors={colors}
 *     selected={selected}
 *     renderContent={(ctx) => <StructuralCardContent data={data} {...ctx} />}
 *   />
 * );
 * ```
 */

// Types (3-level architecture) - excluding TRAIT_BORDERS/TraitBorderStyle (defined in taxonomyColors)
export {
  // Re-exported types
  type RealmKey,
  type LayerKey,
  type TraitKey,
  type ArcFamilyKey,
  type PerformanceTier,
  type PerformanceConfig,
  // Base types
  type BaseNodeData,
  type CardColors,
  type CardContext,
  type NodeLevel,
  // Taxonomy types
  type TaxonomyVariant,
  type TaxonomyNodeData,
  type RealmTaxonomyData,
  type LayerTaxonomyData,
  type TraitTaxonomyData,
  type ArcFamilyTaxonomyData,
  // Schema types
  type SchemaVariant,
  type SchemaNodeData,
  type NodeClassData,
  type ArcClassData,
  // Data types
  type DataNodeData,
  type FoundationInstanceData,
  type StructureInstanceData,
  type SemanticInstanceData,
  type OutputInstanceData,
  // Union types
  type TaxonomyNodeDataUnion,
  type SchemaNodeDataUnion,
  type DataNodeDataUnion,
  type CardNodeData,
  type CardShellProps as TypesCardShellProps,
  // Animation variants
  CARD_ANIMATION_VARIANTS,
  // Helper functions
  isNodeLevel,
  isTaxonomyNode,
  isSchemaNode,
  isDataNode,
  isTaxonomyVariant,
  isSchemaVariant,
} from './types';

// Animation presets
export * from './animationPresets';

// Premium effects (MagicUI inspired)
export * from './effects';

// Taxonomy visual encoding (ADR-005) - includes TRAIT_BORDERS and TraitBorderStyle
export * from './taxonomyColors';
export {
  TaxonomyBadge,
  TaxonomyBadgeCompact,
  type TaxonomyBadgeProps,
  type TaxonomyBadgeCompactProps,
} from './TaxonomyBadge';

// CardShell component
export { CardShell, type CardShellProps } from './CardShell';
// Re-export legacy CardContext/CardColors from CardShell for backward compatibility
export type { CardContext as LegacyCardContext, CardColors as LegacyCardColors } from './CardShell';

// LayerCardWrapper (semantic layer-based styling)
export {
  LayerCardWrapper,
  type LayerCardWrapperProps,
  type LayerCardContext,
  type TaxonomyInfo,
} from './LayerCardWrapper';

// Card variants (type-safe styling)
export * from './variants';

// Content presets
export {
  StructuralCardContent,
  type StructuralCardContentProps,
  type StructuralNodeData,
  type TaxonomyProps,
} from './presets/StructuralCardContent';

export {
  AttractorCardContent,
  type AttractorCardContentProps,
  type AttractorNodeData,
} from './presets/AttractorCardContent';

export {
  ProjectCardContent,
  type ProjectCardContentProps,
  type ProjectNodeData,
  type ProjectTaxonomyProps,
} from './presets/ProjectCardContent';

export {
  LocaleCardContent,
  type LocaleCardContentProps,
  type LocaleNodeData,
  type LocaleTaxonomyProps,
} from './presets/LocaleCardContent';

export {
  ClassCardContent,
  type ClassCardContentProps,
  type ClassNodeData,
  type ClassTaxonomyProps,
} from './presets/ClassCardContent';

// Taxonomy cards (Premium TAXONOMY level design - v0.13.1)
export {
  TaxonomyCardContent,
  type TaxonomyCardContentProps,
  type TaxonomyNodeData as TaxonomyCardNodeData, // Aliased to avoid conflict with types
  type TaxonomyVariant as TaxonomyCardVariant,   // Aliased to avoid conflict with types
} from './presets/TaxonomyCardContent';

export {
  RealmOrbitalCardContent,
  type RealmOrbitalCardProps,
  type RealmNodeData,
} from './presets/RealmOrbitalCardContent';

// Geography Layer cards (Continent, GeoRegion, GeoSubRegion, Country, etc.)
export {
  GeographyCardContent,
  type GeographyCardContentProps,
  type GeographyNodeData,
  type GeographyNodeType,
  type GeographyTaxonomyProps,
} from './presets/GeographyCardContent';

// Foundation Layer cards (Project, Brand, BrandDesign, PromptStyle)
export {
  // Helper components
  KPIGauge,
  type KPIGaugeProps,
  ColorSwatch,
  type ColorSwatchProps,
  TypographyPreview,
  type TypographyPreviewProps,
  TraitBadge,
  type TraitBadgeProps,
  SectionLabel,
  type SectionLabelProps,
  PlatformBadge,
  type PlatformBadgeProps,
  // Card content components
  BrandCardContent,
  type BrandCardContentProps,
  type BrandNodeData,
  BrandDesignCardContent,
  type BrandDesignCardContentProps,
  type BrandDesignNodeData,
  PromptStyleCardContent,
  type PromptStyleCardContentProps,
  type PromptStyleNodeData,
} from './presets/foundation';

// Structure Layer cards (Page, Block, ContentSlot)
export {
  // Helper components
  URLPathDisplay,
  type URLPathDisplayProps,
  type LocalePath,
  BlockOrderIndicator,
  type BlockOrderIndicatorProps,
  SEOScoreDisplay,
  type SEOScoreDisplayProps,
  PillarBadge as StructurePillarBadge,
  type PillarBadgeProps as StructurePillarBadgeProps,
  StatCounter,
  type StatCounterProps,
  // Card content components
  PageCardContent,
  type PageCardContentProps,
  type PageNodeData,
  BlockCardContent,
  type BlockCardContentProps,
  type BlockNodeData,
  ContentSlotCardContent,
  type ContentSlotCardContentProps,
  type ContentSlotNodeData,
} from './presets/structure';

// Semantic Layer cards (Entity, EntityNative)
export {
  // Helper components
  PillarBadge as SemanticPillarBadge,
  type PillarBadgeProps as SemanticPillarBadgeProps,
  SchemaOrgBadge,
  type SchemaOrgBadgeProps,
  CurationBadge,
  type CurationBadgeProps,
  LocaleBadge,
  type LocaleBadgeProps,
  SemanticLinkCounter,
  type SemanticLinkCounterProps,
  StatusBadge,
  type StatusBadgeProps,
  BenefitsList,
  type BenefitsListProps,
  ContentStats,
  type ContentStatsProps,
  // Card content components
  EntityCardContent,
  type EntityCardContentProps,
  type EntityNodeData,
  EntityNativeCardContent,
  type EntityNativeCardContentProps,
  type EntityNativeNodeData,
} from './presets/semantic';

// Instruction Layer cards (BlockInstruction, BlockType, PromptArtifact)
export {
  // Helper components
  CategoryBadge,
  type CategoryBadgeProps,
  DirectiveBadge,
  type DirectiveBadgeProps,
  ReferenceCounter,
  type ReferenceCounterProps,
  VersionBadge,
  type VersionBadgeProps,
  TokenCounter,
  type TokenCounterProps,
  ContentPreview as InstructionContentPreview,
  type ContentPreviewProps as InstructionContentPreviewProps,
  OrderBadge,
  type OrderBadgeProps,
  InclusionFlags,
  type InclusionFlagsProps,
  SchemaPropertyList,
  type SchemaPropertyListProps,
  CompilationStatus,
  type CompilationStatusProps,
  // Card content components
  BlockInstructionCardContent,
  type BlockInstructionCardContentProps,
  type BlockInstructionNodeData,
  BlockTypeCardContent,
  type BlockTypeCardContentProps,
  type BlockTypeNodeData,
  PromptArtifactCardContent,
  type PromptArtifactCardContentProps,
  type PromptArtifactNodeData,
} from './presets/instruction';

// Output Layer cards (PageNative, BlockNative, OutputArtifact)
export {
  // Helper components
  GeneratedBadge,
  type GeneratedBadgeProps,
  OutputStatusBadge,
  type OutputStatusBadgeProps,
  OutputLocaleBadge,
  type OutputLocaleBadgeProps,
  VersionHistory,
  type VersionHistoryProps,
  AssemblyInfo,
  type AssemblyInfoProps,
  ContentPreview as OutputContentPreview,
  type ContentPreviewProps as OutputContentPreviewProps,
  BundleStats,
  type BundleStatsProps,
  ChecksumBadge,
  type ChecksumBadgeProps,
  AnchorSlugBadge,
  type AnchorSlugBadgeProps,
  // Card content components
  PageNativeCardContent,
  type PageNativeCardContentProps,
  type PageNativeNodeData,
  BlockNativeCardContent,
  type BlockNativeCardContentProps,
  type BlockNativeNodeData,
  OutputArtifactCardContent,
  type OutputArtifactCardContentProps,
  type OutputArtifactNodeData,
} from './presets/output';

// Knowledge Layer cards (Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait, Sets, SEOKeyword)
export {
  // Helper components
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
  // Card content components - Atoms (v0.18.0: TermCardContent removed - YAGNI cleanup)
  ExpressionCardContent,
  type ExpressionCardContentProps,
  type ExpressionNodeData,
  PatternCardContent,
  type PatternCardContentProps,
  type PatternNodeData,
  type PatternTaxonomyProps,
  CultureRefCardContent,
  type CultureRefCardContentProps,
  type CultureRefNodeData,
  type CultureRefTaxonomyProps,
  type CultureCategory,
  type SensitivityLevel,
  TabooCardContent,
  type TabooCardContentProps,
  type TabooNodeData,
  type TabooTaxonomyProps,
  type TabooCategory,
  type TabooSeverity,
  AudienceTraitCardContent,
  type AudienceTraitCardContentProps,
  type AudienceTraitNodeData,
  type AudienceTraitTaxonomyProps,
  type TraitCategory,
  type ImportanceLevel,
  // Card content components - Containers (Sets)
  KnowledgeSetCardContent,
  type KnowledgeSetCardContentProps,
  type KnowledgeSetNodeData,
  type KnowledgeSetTaxonomyProps,
  type KnowledgeSetType,
  // Card content components - SEO
  SEOKeywordCardContent,
  type SEOKeywordCardContentProps,
  type SEOKeywordNodeData,
} from './presets/knowledge';

// Locale Layer cards (Culture, Style, Formatting, Adaptation, Slugification, Market)
export {
  LocaleSettingsCardContent,
  type LocaleSettingsCardContentProps,
  type LocaleSettingsNodeData,
  type LocaleSettingsTaxonomyProps,
  type LocaleSettingsType,
  // Individual type data
  type CultureNodeData,
  type StyleNodeData,
  type FormattingNodeData,
  type AdaptationNodeData,
  type SlugificationNodeData,
  type MarketNodeData,
} from './presets/locale';

// Config Layer cards (EntityCategory - Crystal Badge design)
export {
  EntityCategoryCardContent,
  type EntityCategoryCardContentProps,
  type EntityCategoryNodeData,
  type EntityCategoryTaxonomyProps,
} from './presets/config';

// Card Content Selector - Routes node types to specialized cards
export {
  getCardContentComponent,
  hasSpecializedCard,
  getSpecializedNodeTypes,
  type CardContentData,
  type CardContentProps,
  type CardContentComponent,
} from './cardContentSelector';
