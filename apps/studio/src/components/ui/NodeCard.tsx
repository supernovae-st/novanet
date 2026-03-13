'use client';

/**
 * NodeCard - Unified card component for displaying graph nodes
 *
 * Features:
 * - Layer-colored border with glow effect
 * - Badge showing layer/trait information
 * - Title and subtitle
 * - Consistent styling across Meta and Data views
 * - Interactive hover states
 *
 * Based on the knowledge card design with glassmorphism effects.
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLayerGradientColors } from '@/design/nodeColors';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import type { GraphNode } from '@/types';
import type { Layer } from '@novanet/core/types';

export interface NodeCardProps {
  /** Node data to display */
  node: GraphNode;
  /** Whether the card is selected */
  isSelected?: boolean;
  /** Click handler */
  onClick?: () => void;
  /** Compact variant (smaller) */
  compact?: boolean;
  /** Show instance count badge */
  instanceCount?: number;
  /** Additional className */
  className?: string;
}

/**
 * Get layer badge label (uppercase for visual emphasis)
 */
function getLayerBadgeLabel(layer: Layer): string {
  const labels: Record<Layer, string> = {
    // Shared realm (4) — v11.4: includes config
    config: 'CONFIG',
    locale: 'LOCALE',
    geography: 'GEOGRAPHY',
    knowledge: 'KNOWLEDGE',  // v11.4: includes SEO/GEO nodes
    // Org realm (6) — v11.4: seo/geo removed
    foundation: 'FOUNDATION',
    structure: 'STRUCTURE',
    semantic: 'SEMANTIC',
    instruction: 'INSTRUCTION',
    output: 'OUTPUT',
  };
  return labels[layer] || layer.toUpperCase();
}

export const NodeCard = memo(function NodeCard({
  node,
  isSelected = false,
  onClick,
  compact = false,
  instanceCount,
  className,
}: NodeCardProps) {
  const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;
  const colors = getLayerGradientColors(config.layer);

  // Memoize glow style for performance
  const glowStyle = useMemo(() => ({
    boxShadow: isSelected
      ? `0 0 30px ${colors.primary}50, 0 0 60px ${colors.primary}25, inset 0 0 20px ${colors.primary}10`
      : `0 0 20px ${colors.primary}30, 0 0 40px ${colors.primary}15`,
    borderColor: isSelected ? colors.primary : `${colors.primary}60`,
  }), [colors.primary, isSelected]);

  // Memoize badge style
  const badgeStyle = useMemo(() => ({
    background: `linear-gradient(135deg, ${colors.primary}35, ${colors.secondary}25)`,
    borderColor: `${colors.primary}50`,
    color: colors.primary,
    boxShadow: `0 0 12px ${colors.primary}30`,
  }), [colors.primary, colors.secondary]);

  return (
    <div
      onClick={onClick}
      className={cn(
        // Base layout
        'relative flex flex-col rounded-xl border-2 transition-all duration-300',
        // Background - glassmorphism
        'bg-[#0d0d12]/90 backdrop-blur-sm',
        // Padding based on variant
        compact ? 'p-3' : 'p-4',
        // Interactive states
        onClick && 'cursor-pointer',
        // Hover effect
        !isSelected && onClick && 'hover:scale-[1.02] hover:border-opacity-100',
        // Selected state
        isSelected && 'ring-2 ring-offset-2 ring-offset-[#0d0d12]',
        className
      )}
      style={glowStyle}
    >
      {/* Badge - Layer indicator */}
      <div
        className={cn(
          'inline-flex items-center self-start px-2.5 py-1 rounded-full text-[10px] font-bold border',
          compact ? 'mb-2' : 'mb-3',
          'gap-1.5'
        )}
        style={badgeStyle}
      >
        <LayerIcon
          layer={config.layer}
          size={compact ? 10 : 12}
          strokeWidth={2.5}
          style={{ color: colors.primary }}
        />
        {getLayerBadgeLabel(config.layer)}
      </div>

      {/* Title */}
      <h3
        className={cn(
          'font-semibold text-white truncate',
          compact ? 'text-sm mb-0.5' : 'text-base mb-1'
        )}
        title={node.displayName}
      >
        {node.displayName}
      </h3>

      {/* Subtitle - description or key */}
      <p
        className={cn(
          'text-white/50 truncate',
          compact ? 'text-[11px]' : 'text-xs'
        )}
        title={node.content || node.key}
      >
        {node.content || node.key}
      </p>

      {/* Instance count badge (for meta view) */}
      {typeof instanceCount === 'number' && instanceCount > 0 && (
        <div
          className={cn(
            'absolute top-2 right-2 px-2 py-0.5 rounded-full text-[10px] font-medium',
            'bg-white/10 text-white/70 border border-white/10'
          )}
        >
          {instanceCount}
        </div>
      )}
    </div>
  );
});

/**
 * NodeCardGrid - Grid layout for NodeCard components
 */
export interface NodeCardGridProps {
  children: React.ReactNode;
  columns?: 2 | 3 | 4;
  className?: string;
}

export const NodeCardGrid = memo(function NodeCardGrid({
  children,
  columns = 2,
  className,
}: NodeCardGridProps) {
  const gridCols = {
    2: 'grid-cols-2',
    3: 'grid-cols-3',
    4: 'grid-cols-4',
  };

  return (
    <div className={cn('grid gap-3', gridCols[columns], className)}>
      {children}
    </div>
  );
});
