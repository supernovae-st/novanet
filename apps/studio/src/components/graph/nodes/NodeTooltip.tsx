'use client';

/**
 * NodeTooltip - Hover tooltip for graph nodes
 *
 * Features:
 * - Shows node type and key on hover
 * - Fixed position above center pill
 * - Portal-based rendering
 * - Fade in animation
 */

import { memo, useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { NodeType } from '@/types';

export interface NodeTooltipProps {
  /** Whether the tooltip is visible */
  visible: boolean;
  /** Node type for display */
  nodeType: NodeType;
  /** Node key to display */
  nodeKey: string;
  /** Optional display name */
  displayName?: string;
  /** Primary color for accent */
  color?: string;
  /** Additional class names */
  className?: string;
}

/**
 * NodeTooltip Component
 * Renders a tooltip fixed above the center pill via portal
 */
export const NodeTooltip = memo(function NodeTooltip({
  visible,
  nodeType,
  nodeKey,
  displayName,
  color,
  className,
}: NodeTooltipProps) {
  const [mounted, setMounted] = useState(false);
  const config = NODE_TYPE_CONFIG[nodeType] || NODE_TYPE_CONFIG.Project;
  const accentColor = color || config.color;

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!visible || !mounted) return null;

  const tooltipContent = (
    <div
      className={cn(
        // Fixed position above center pill
        'fixed left-1/2 -translate-x-1/2 bottom-20 z-[100]',
        // Styling
        'node-tooltip animate-tooltip-fade-in',
        'whitespace-nowrap',
        className
      )}
    >
      {/* Type badge */}
      <div className={cn('flex items-center', gapTokens.default)}>
        <span
          className={cn('inline-flex items-center', gapTokens.compact)}
          style={{ color: accentColor }}
        >
          <CategoryIcon
            category={config.category}
            size={14}
            strokeWidth={2}
            style={{ color: accentColor }}
          />
          <span className="font-bold uppercase text-[10px] tracking-wider">
            {config.label}
          </span>
        </span>
      </div>

      {/* Key/Name */}
      <div className="mt-1 text-white/90 font-mono text-[11px] truncate max-w-[280px]">
        {displayName && displayName !== nodeKey ? (
          <>
            <span className="text-white">{displayName}</span>
            <span className="text-white/50 ml-1">({nodeKey})</span>
          </>
        ) : (
          nodeKey
        )}
      </div>
    </div>
  );

  return createPortal(tooltipContent, document.body);
});
