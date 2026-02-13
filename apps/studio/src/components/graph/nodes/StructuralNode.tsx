'use client';

/**
 * StructuralNode - Gradient Edge design for structural nodes
 *
 * Categories: project, content, locale, generation
 * Uses CardShell + StructuralCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getStructuralColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, StructuralCardContent } from './card';

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
 * StructuralNode - Uses unified CardShell + StructuralCardContent
 */
export const StructuralNode = memo(function StructuralNode(props: NodeProps<StructuralNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getStructuralColors(data.type), [data.type]);
  const width = getCardWidth(data.type);

  // Prepare data for StructuralCardContent
  const contentData = useMemo(() => ({
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
  }), [data.id, data.type, data.key, data.displayName]);

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
        <StructuralCardContent data={contentData} {...ctx} />
      )}
    />
  );
});
