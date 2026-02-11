'use client';

/**
 * StructuralCardContent - Content preset for structural nodes
 *
 * Used by: Page, Entity, Block, PageType, BlockType, ContentSlot, etc.
 *
 * Layout:
 * ┌─────────────────────────────┐
 * │ [Icon] Type Label    [dot] │
 * │ Display Name               │
 * │ key (if different)         │
 * │ [Layer Badge]              │
 * └─────────────────────────────┘
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { gapTokens } from '@/design/tokens';
import type { CardContext } from '../CardShell';
import type { Layer } from '@novanet/core/types';

// =============================================================================
// Types
// =============================================================================

export interface StructuralNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
}

export interface StructuralCardContentProps extends CardContext {
  data: StructuralNodeData;
}

// =============================================================================
// Component
// =============================================================================

export const StructuralCardContent = memo(function StructuralCardContent({
  data,
  colors,
  selected,
  isHovered,
}: StructuralCardContentProps) {
  // Type-safe config lookup with fallback
  const config = (NODE_TYPE_CONFIG as Record<string, { label: string; layer: string }>)[data.type]
    || { label: data.type, layer: 'foundation' };

  // Memoize icon style
  const iconStyle = useMemo(() => ({
    color: colors.primary,
    filter: `drop-shadow(0 0 ${selected ? '10px' : '6px'} ${colors.primary}80)`,
  }), [colors.primary, selected]);

  return (
    <div className="px-3 py-2.5">
      {/* Header: Icon + Type Label + Status Dot */}
      <div className="flex items-center justify-between mb-1.5">
        <div className={cn('flex items-center', gapTokens.default)}>
          <LayerIcon
            layer={config.layer as Layer}
            size={20}
            strokeWidth={2}
            className={cn(
              'transition-transform duration-200',
              (selected || isHovered) && 'scale-110'
            )}
            style={iconStyle}
          />
          <span
            className="text-[10px] font-bold uppercase tracking-wider"
            style={{ color: colors.primary }}
          >
            {config.label}
          </span>
        </div>

        {/* Status dot */}
        <div
          className={cn('w-2 h-2 rounded-full', selected && 'animate-pulse')}
          style={{
            background: colors.primary,
            boxShadow: `0 0 6px ${colors.primary}`,
          }}
        />
      </div>

      {/* Display Name */}
      <h3 className="text-sm font-bold text-white truncate">
        {data.displayName}
      </h3>

      {/* Key (if different from displayName) */}
      {data.key !== data.displayName && (
        <p
          className="text-[10px] font-mono truncate mt-0.5"
          style={{ color: `${colors.primary}70` }}
        >
          {data.key}
        </p>
      )}

      {/* Layer badge */}
      <div
        className={cn(
          'mt-2 inline-flex items-center px-1.5 py-0.5 rounded-full',
          'text-[8px] font-semibold uppercase tracking-wider border',
          gapTokens.compact
        )}
        style={{
          background: `${colors.primary}15`,
          borderColor: `${colors.primary}35`,
          color: colors.primary,
        }}
      >
        <span
          className={cn('w-1 h-1 rounded-full', selected && 'animate-pulse')}
          style={{
            background: colors.primary,
            boxShadow: `0 0 4px ${colors.primary}`,
          }}
        />
        {config.layer}
      </div>
    </div>
  );
});
