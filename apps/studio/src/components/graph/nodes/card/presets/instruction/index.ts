/**
 * Instruction Layer Card Components
 *
 * Layer color: Yellow #eab308
 * 4 nodes: BlockInstruction, BlockType, BlockRules, PromptArtifact
 *
 * All components are performance-aware and support Framer Motion animations.
 */

// Helper components
export {
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
  ContentPreview,
  type ContentPreviewProps,
  OrderBadge,
  type OrderBadgeProps,
  InclusionFlags,
  type InclusionFlagsProps,
  SchemaPropertyList,
  type SchemaPropertyListProps,
  CompilationStatus,
  type CompilationStatusProps,
} from './InstructionHelpers';

// Card content components
export {
  BlockInstructionCardContent,
  type BlockInstructionCardContentProps,
  type BlockInstructionNodeData,
} from './BlockInstructionCardContent';

export {
  BlockTypeCardContent,
  type BlockTypeCardContentProps,
  type BlockTypeNodeData,
} from './BlockTypeCardContent';

export {
  BlockRulesCardContent,
  type BlockRulesCardContentProps,
  type BlockRulesNodeData,
} from './BlockRulesCardContent';

export {
  PromptArtifactCardContent,
  type PromptArtifactCardContentProps,
  type PromptArtifactNodeData,
} from './PromptArtifactCardContent';
