/**
 * Structure Layer Card Components
 *
 * Layer color: Blue #3b82f6
 * 3 nodes: Page, Block, ContentSlot
 *
 * All components are performance-aware and support Framer Motion animations.
 */

// Helper components
export {
  URLPathDisplay,
  type URLPathDisplayProps,
  type LocalePath,
  BlockOrderIndicator,
  type BlockOrderIndicatorProps,
  SEOScoreDisplay,
  type SEOScoreDisplayProps,
  PillarBadge,
  type PillarBadgeProps,
  StatCounter,
  type StatCounterProps,
} from './StructureHelpers';

// Card content components
export {
  PageCardContent,
  type PageCardContentProps,
  type PageNodeData,
} from './PageCardContent';

export {
  BlockCardContent,
  type BlockCardContentProps,
  type BlockNodeData,
} from './BlockCardContent';

export {
  ContentSlotCardContent,
  type ContentSlotCardContentProps,
  type ContentSlotNodeData,
} from './ContentSlotCardContent';
