'use client';

/**
 * ProjectNode - Premium card for Project nodes
 *
 * Features:
 * - Logo + bold typography
 * - Premium violet theme with gradient border
 * - Grid background pattern (Project-specific decoration via content)
 *
 * Uses CardShell + ProjectCardContent for consistent design system.
 */

import { memo, useMemo } from 'react';
import { type Node, type NodeProps } from '@xyflow/react';
import type { BaseNodeData } from './BaseNodeWrapper';
import { getStructuralColors } from '@/design/nodeColors';
import { CardShell, ProjectCardContent } from './card';
import { getNodeConfig } from './NodeConfig';

export type ProjectNodeType = Node<BaseNodeData>;

/**
 * ProjectNode - Uses unified CardShell + ProjectCardContent
 */
export const ProjectNode = memo(function ProjectNode(props: NodeProps<ProjectNodeType>) {
  const { data, selected = false } = props;
  const colors = useMemo(() => getStructuralColors('Project'), []);

  // Prepare data for ProjectCardContent
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
      width={280}
      isDimmed={data.dimmed === true}
      isHoverDimmed={data.hoverDimmed === true}
      isSchemaMode={data.isSchemaMode === true}
      ariaLabel={`Project node: ${data.displayName}`}
      renderContent={(ctx) => (
        <ProjectCardContent data={contentData} {...ctx} />
      )}
    />
  );
});
