'use client';

/**
 * LocaleKnowledgeNode - Unified card design for locale/knowledge nodes
 *
 * Types: v11.5 knowledge atoms - Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait
 * Plus containers: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
 * And locale/geography nodes: Locale, Culture, Style, Region, Country, etc.
 *
 * Uses CardShell + StructuralCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getLocaleKnowledgeColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, StructuralCardContent } from './card';

export type LocaleKnowledgeNodeType = Node<BaseNodeData>;

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
    // Locale/geography nodes
    case 'Locale':
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
 * LocaleKnowledgeNode - Uses unified CardShell + StructuralCardContent
 */
export const LocaleKnowledgeNode = memo(function LocaleKnowledgeNode(props: NodeProps<LocaleKnowledgeNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getLocaleKnowledgeColors(data.type), [data.type]);
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
