'use client';

import { ChevronRight, ChevronDown, Loader2, type LucideIcon } from 'lucide-react';
import * as Icons from 'lucide-react';
import type { UnifiedNode } from '@novanet/core';
import { useTreeStore } from '@/stores/treeStore';
import { cn } from '@/lib/utils';

/**
 * Extended node props for tree rendering.
 * The base UnifiedNode from @novanet/core is intersected with optional
 * children and badges that are managed by the tree store.
 */
type TreeNode = UnifiedNode & {
  /** Child nodes or 'lazy' for lazy loading */
  children?: TreeNode[] | 'lazy';
  /** Optional badges to display */
  badges?: Array<{ text: string; color: string }>;
};

interface UnifiedTreeNodeProps {
  node: TreeNode;
  depth: number;
}

export function UnifiedTreeNode({ node, depth }: UnifiedTreeNodeProps) {
  const { expanded, selectedId, loading, toggleExpand, selectNode, loadChildren } = useTreeStore();
  const isExpanded = expanded.has(node.id);
  const isSelected = selectedId === node.id;
  const isLoading = loading.has(node.id);

  const hasChildren = node.children && node.children !== 'lazy';
  const canExpand = hasChildren || node.children === 'lazy';

  // Get the Lucide icon component dynamically
  const IconName = node.icon?.web || 'file';
  const IconComponent = (Icons as unknown as Record<string, LucideIcon>)[pascalCase(IconName)] || Icons.File;

  const handleClick = () => {
    selectNode(node.id);
    if (canExpand) {
      toggleExpand(node.id);
      // If children are lazy and we're expanding, load them
      if (node.children === 'lazy' && !isExpanded) {
        loadChildren(node.id);
      }
    }
  };

  return (
    <div>
      <div
        className={cn(
          'flex items-center gap-1 px-2 py-1 cursor-pointer hover:bg-muted rounded-sm',
          isSelected && 'bg-accent'
        )}
        style={{ paddingLeft: `${depth * 16 + 8}px` }}
        onClick={handleClick}
      >
        {/* Expand/collapse chevron */}
        {canExpand ? (
          isLoading ? (
            <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
          ) : isExpanded ? (
            <ChevronDown className="h-4 w-4 text-muted-foreground" />
          ) : (
            <ChevronRight className="h-4 w-4 text-muted-foreground" />
          )
        ) : (
          <span className="w-4" /> // Spacer for alignment
        )}

        {/* Node icon */}
        <IconComponent className="h-4 w-4" />

        {/* Label */}
        <span className="flex-1 truncate">{node.label}</span>

        {/* Badges */}
        {node.badges?.map((badge, i) => (
          <span
            key={i}
            className="text-xs px-1.5 py-0.5 rounded-full bg-muted"
            style={{ color: badge.color }}
          >
            {badge.text}
          </span>
        ))}
      </div>

      {/* Children */}
      {isExpanded && Array.isArray(node.children) && (
        <div>
          {node.children.map((child) => (
            <UnifiedTreeNode key={child.id} node={child} depth={depth + 1} />
          ))}
        </div>
      )}
    </div>
  );
}

// Helper to convert kebab-case to PascalCase for Lucide icons
function pascalCase(str: string): string {
  return str
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join('');
}
