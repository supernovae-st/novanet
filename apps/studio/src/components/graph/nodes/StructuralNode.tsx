'use client';

/**
 * StructuralNode - Gradient Edge design for structural nodes (v0.12.5)
 *
 * Categories: project, content, locale, generation
 *
 * v0.12.5: Uses card content selector to route to specialized card components
 * based on node type (ADR-005 visual encoding).
 *
 * Handles ORG realm nodes across 6 layers:
 * - foundation: Project, Brand, BrandDesign, PromptStyle
 * - structure: Page, Block, ContentSlot
 * - semantic: Entity, EntityNative
 * - instruction: BlockInstruction, BlockType, BlockRules, PromptArtifact
 * - output: PageNative, BlockNative, OutputArtifact
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getStructuralColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, getCardContentComponent } from './card';

export type StructuralNodeType = Node<BaseNodeData>;

/**
 * Get card width based on node type
 */
function getCardWidth(type: string): number {
  switch (type) {
    case 'Page': return 210;
    case 'Entity': return 195;
    case 'Block': return 175;
    case 'BlockType': return 165;
    case 'Locale': return 200;
    default: return 180;
  }
}

/**
 * StructuralNode - Uses card content selector for specialized rendering
 *
 * The selector routes to specialized card components based on node type:
 * - Foundation types → BrandCardContent, BrandDesignCardContent, etc.
 * - Structure types → PageCardContent, BlockCardContent
 * - Semantic types → EntityCardContent, EntityNativeCardContent
 * - Instruction types → BlockInstructionCardContent, BlockTypeCardContent, etc.
 * - Output types → PageNativeCardContent, BlockNativeCardContent
 * - Fallback → StructuralCardContent
 */
export const StructuralNode = memo(function StructuralNode(props: NodeProps<StructuralNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getStructuralColors(data.type), [data.type]);
  const width = getCardWidth(data.type);

  // Get the specialized card content component for this node type
  const CardContent = useMemo(() => getCardContentComponent(data.type), [data.type]);

  // Prepare data for card content
  // BaseNodeData extends Record<string, unknown>, so all Neo4j properties
  // are available directly on data.
  const contentData = useMemo(() => ({
    ...data, // Spread all properties from Neo4j
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
    locale: data.locale,
  }), [data]);

  return (
    <CardShell
      colors={colors}
      selected={selected}
      width={width}
      isDimmed={data.dimmed === true}
      isHoverDimmed={data.hoverDimmed === true}
      isSchemaMode={data.isSchemaMode === true}
      ariaLabel={`${data.type} node: ${data.displayName}`}
      renderContent={(ctx) => (
        <CardContent data={contentData} {...ctx} />
      )}
    />
  );
});
