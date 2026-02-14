'use client';

/**
 * SharedLayerNode - Unified card design for shared realm nodes (v0.12.4)
 *
 * Handles nodes from 4 shared realm layers:
 * - config: Locale, EntityCategory
 * - locale: Culture, Style, Formatting, etc.
 * - geography: Region, Country, Continent, etc.
 * - knowledge: Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
 * Plus containers: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 *
 * Uses CardShell + StructuralCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getSharedKnowledgeColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, StructuralCardContent } from './card';

export type SharedLayerNodeType = Node<BaseNodeData>;

/**
 * Get card width based on node type
 * Slightly smaller than StructuralNode since these are knowledge atoms
 */
function getCardWidth(type: string): number {
  switch (type) {
    // Containers (larger)
    case 'TermSet':
    case 'ExpressionSet':
    case 'PatternSet':
    case 'CultureSet':
    case 'TabooSet':
    case 'AudienceSet':
    case 'CategorySet':
      return 175;
    // Config/locale/geography nodes
    case 'Locale':
    case 'EntityCategory':
    case 'Culture':
    case 'Style':
    case 'Region':
    case 'Country':
    case 'Continent':
      return 170;
    // Knowledge atoms (smaller)
    case 'Term':
    case 'Expression':
    case 'Pattern':
    case 'CultureRef':
    case 'Taboo':
    case 'AudienceTrait':
      return 155;
    default:
      return 160;
  }
}

/**
 * SharedLayerNode - Uses unified CardShell + StructuralCardContent
 */
export const SharedLayerNode = memo(function SharedLayerNode(props: NodeProps<SharedLayerNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getSharedKnowledgeColors(data.type), [data.type]);
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
