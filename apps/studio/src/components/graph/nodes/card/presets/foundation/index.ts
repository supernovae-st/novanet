/**
 * Foundation Layer Card Components
 *
 * Layer color: Violet #8b5cf6
 * 6 nodes: Project, Brand, BrandDesign, BrandPrinciples, PromptStyle, ProjectNative
 *
 * All components are performance-aware and support Framer Motion animations.
 */

// Helper components
export {
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
} from './FoundationHelpers';

// Card content components
export {
  BrandCardContent,
  type BrandCardContentProps,
  type BrandNodeData,
} from './BrandCardContent';

export {
  BrandDesignCardContent,
  type BrandDesignCardContentProps,
  type BrandDesignNodeData,
} from './BrandDesignCardContent';

export {
  PromptStyleCardContent,
  type PromptStyleCardContentProps,
  type PromptStyleNodeData,
} from './PromptStyleCardContent';
