'use client';

/**
 * NodeNavigationCard - Card for navigating to related nodes
 *
 * Features:
 * - Displays node info with icon
 * - Click to navigate to node
 * - Optional label (Source/Target/etc.)
 * - Hover effects with scale animation
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { ACTION_ICONS } from '@/config/iconSystem';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import { iconSizes } from '@/design/tokens';
import type { GraphNode } from '@/types';

const TargetIcon = ACTION_ICONS.target;

export interface NodeNavigationCardProps {
  node: GraphNode | null;
  label?: string;
  labelColor?: string;
  onClick: () => void;
  variant?: 'default' | 'compact';
  className?: string;
}

export const NodeNavigationCard = memo(function NodeNavigationCard({
  node,
  label,
  labelColor = '#3b82f6',
  onClick,
  variant = 'default',
  className,
}: NodeNavigationCardProps) {
  if (!node) {
    return (
      <div
        className={cn(
          'p-3 rounded-xl bg-white/[0.04] border border-white/[0.08] text-center',
          className
        )}
      >
        <span className="text-xs text-white/50">Node not found</span>
      </div>
    );
  }

  const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;

  if (variant === 'compact') {
    return (
      <button
        onClick={onClick}
        className={cn(
          'w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm',
          'bg-white/6 hover:bg-white/10 border border-white/10 hover:border-white/18',
          'transition-all text-left group',
          className
        )}
      >
        <CategoryIcon
          category={config.category}
          size={16}
          strokeWidth={2}
          className="group-hover:scale-110 transition-transform"
          style={{ color: config.color }}
        />
        <div className="flex-1 min-w-0">
          <div className="text-white/95 font-medium truncate">
            {node.displayName || node.key?.slice(0, 12)}
          </div>
          <div className="text-xs text-white/60 font-mono truncate mt-0.5">
            {node.type}
          </div>
        </div>
        <TargetIcon className={cn(iconSizes.md, 'text-white/40 group-hover:text-white/70 transition-colors')} />
      </button>
    );
  }

  return (
    <button
      onClick={onClick}
      className={cn(
        'w-full flex items-center gap-3 p-3 rounded-xl text-left',
        'bg-white/6 hover:bg-white/10 border border-white/10 hover:border-white/18',
        'transition-all duration-200 group',
        className
      )}
    >
      {/* Icon */}
      <div
        className="w-10 h-10 rounded-xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-105"
        style={{
          background: `linear-gradient(135deg, ${config.color}25, ${config.color}12)`,
        }}
      >
        <CategoryIcon
          category={config.category}
          size={20}
          strokeWidth={2}
          style={{ color: config.color }}
        />
      </div>

      {/* Content */}
      <div className="flex-1 min-w-0">
        {label && (
          <div className="flex items-center gap-2 mb-0.5">
            <span
              className="text-[10px] font-bold uppercase tracking-wider"
              style={{ color: labelColor }}
            >
              {label}
            </span>
            <span
              className="text-[9px] px-1.5 py-0.5 rounded-md"
              style={{
                background: `${config.color}35`,
                color: config.color,
              }}
            >
              {config.label}
            </span>
          </div>
        )}
        <div className="text-sm text-white/95 font-medium truncate">
          {node.displayName}
        </div>
        <div className="text-[10px] text-white/60 font-mono truncate">
          {node.key}
        </div>
      </div>

      {/* Arrow */}
      <TargetIcon className={cn(iconSizes.md, 'text-white/40 group-hover:text-white/70 transition-colors shrink-0')} />
    </button>
  );
});

// =============================================================================
// Relation Navigation Card - For edges in NodeDetailsPanel
// =============================================================================

export interface RelationNavigationCardProps {
  relatedNode: GraphNode | null;
  edgeType: string;
  isSource: boolean;
  onClick: () => void;
  className?: string;
}

export const RelationNavigationCard = memo(function RelationNavigationCard({
  relatedNode,
  edgeType,
  isSource,
  onClick,
  className,
}: RelationNavigationCardProps) {
  const config = relatedNode
    ? NODE_TYPE_CONFIG[relatedNode.type] || NODE_TYPE_CONFIG.Project
    : null;

  return (
    <button
      onClick={onClick}
      className={cn(
        'w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm',
        'bg-white/6 hover:bg-white/10 border border-white/10 hover:border-white/18',
        'transition-all text-left group',
        className
      )}
    >
      <CategoryIcon
        category={config?.category || 'project'}
        size={16}
        strokeWidth={2}
        className="group-hover:scale-110 transition-transform"
        style={{ color: config?.color || '#8b5cf6' }}
      />
      <div className="flex-1 min-w-0">
        <div className="text-white/95 font-medium truncate">
          {relatedNode?.displayName || 'Unknown'}
        </div>
        <div className="text-xs text-white/60 flex items-center gap-1.5 mt-0.5">
          <span className={isSource ? 'text-primary' : 'text-orange-400'}>
            {isSource ? '→' : '←'}
          </span>
          <span className="font-mono">{edgeType}</span>
        </div>
      </div>
      <TargetIcon className={cn(iconSizes.md, 'text-white/40 group-hover:text-white/70 transition-colors')} />
    </button>
  );
});
