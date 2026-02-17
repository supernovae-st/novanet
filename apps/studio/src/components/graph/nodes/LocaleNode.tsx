'use client';

/**
 * LocaleNode - "Passport Élégant" design for Locale nodes
 *
 * Features:
 * - Large flag in dedicated left zone with radial glow
 * - BCP-47 code as hero element (28px mono bold)
 * - Vertical glowing separator
 * - Display name and region context
 * - Layer badge with pulse effect
 *
 * Uses CardShell + LocaleCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import { getSharedKnowledgeColors } from '@/design/nodeColors';
import type { BaseNodeData } from './BaseNodeWrapper';
import { CardShell, LocaleCardContent } from './card';

export type LocaleNodeType = Node<BaseNodeData>;

// Locale nodes get a wider card for the passport layout
const LOCALE_CARD_WIDTH = 240;

/**
 * LocaleNode - Uses unified CardShell + LocaleCardContent
 */
export const LocaleNode = memo(function LocaleNode(props: NodeProps<LocaleNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getSharedKnowledgeColors(data.type), [data.type]);

  // Prepare data for LocaleCardContent
  const contentData = useMemo(() => ({
    id: data.id,
    type: data.type,
    key: data.key,
    displayName: data.displayName,
    // Region context could come from associated nodes (Continent, GeoRegion)
    // For now, leave empty - can be enhanced with graph traversal
    region: undefined,
  }), [data.id, data.type, data.key, data.displayName]);

  return (
    <CardShell
      colors={colors}
      selected={selected}
      width={LOCALE_CARD_WIDTH}
      isDimmed={data.dimmed === true}
      isHoverDimmed={data.hoverDimmed === true}
      isSchemaMode={data.isSchemaMode === true}
      ariaLabel={`Locale node: ${data.displayName}`}
      renderContent={(ctx) => (
        <LocaleCardContent data={contentData} {...ctx} />
      )}
    />
  );
});
