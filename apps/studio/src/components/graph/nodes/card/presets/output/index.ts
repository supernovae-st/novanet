/**
 * Output Layer Card Components
 *
 * Layer color: Green #22c55e
 * 3 nodes: PageNative, BlockNative, OutputArtifact
 * Trait: generated (dotted borders)
 *
 * All components are performance-aware and support Framer Motion animations.
 */

// Helper components
export {
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
  ContentPreview,
  type ContentPreviewProps,
  BundleStats,
  type BundleStatsProps,
  ChecksumBadge,
  type ChecksumBadgeProps,
  AnchorSlugBadge,
  type AnchorSlugBadgeProps,
} from './OutputHelpers';

// Card content components
export {
  PageNativeCardContent,
  type PageNativeCardContentProps,
  type PageNativeNodeData,
} from './PageNativeCardContent';

export {
  BlockNativeCardContent,
  type BlockNativeCardContentProps,
  type BlockNativeNodeData,
} from './BlockNativeCardContent';

export {
  OutputArtifactCardContent,
  type OutputArtifactCardContentProps,
  type OutputArtifactNodeData,
} from './OutputArtifactCardContent';
