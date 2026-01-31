'use client';

/**
 * SmartHandle - Consistent node connection handles
 *
 * Provides two handle styles:
 * - Solid (target): Filled circle for incoming connections
 * - Hollow (source): Border-only circle for outgoing connections
 *
 * Handles are hidden when node is selected (WOW effect - clean selection state).
 *
 * Used by: StructuralNode, LocaleKnowledgeNode, SchemaNode, ProjectNode
 */

import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import { NODE_DESIGN } from '@/config/constants';

export interface SmartHandleProps {
  /** Handle type: target (incoming) or source (outgoing) */
  type: 'target' | 'source';
  /** Position of the handle */
  position: Position;
  /** Primary color for the handle */
  color: string;
  /** Whether the node is selected (hides handles) */
  selected?: boolean;
  /** Size variant: 'default' or 'small' */
  size?: 'default' | 'small';
  /** Additional className for positioning */
  className?: string;
}

/**
 * SmartHandle - Solid/hollow connection handles
 */
export const SmartHandle = memo(function SmartHandle({
  type,
  position,
  color,
  selected = false,
  size = 'default',
  className,
}: SmartHandleProps) {
  const sizeClasses = size === 'small' ? '!w-2.5 !h-2.5' : '!w-3 !h-3';

  // Solid (filled) for target, hollow (border only) for source
  const isSolid = type === 'target';

  return (
    <Handle
      type={type}
      position={position}
      className={cn(
        sizeClasses,
        '!rounded-full !border-2 transition duration-200',
        selected && '!opacity-0 !pointer-events-none',
        className
      )}
      style={{
        backgroundColor: isSolid ? color : 'transparent',
        borderColor: color,
        boxShadow: selected ? NODE_DESIGN.shadows.handleGlow(color) : undefined,
      }}
    />
  );
});

/**
 * Pre-configured handle pairs for common node layouts
 */

export interface NodeHandlesProps {
  /** Primary color for the handles */
  color: string;
  /** Whether the node is selected */
  selected?: boolean;
  /** Layout: 'vertical' (top/bottom) or 'horizontal' (left/right) */
  layout?: 'vertical' | 'horizontal';
  /** Size variant */
  size?: 'default' | 'small';
}

/**
 * NodeHandles - Pre-configured target + source handle pair
 */
export const NodeHandles = memo(function NodeHandles({
  color,
  selected = false,
  layout = 'vertical',
  size = 'default',
}: NodeHandlesProps) {
  const isVertical = layout === 'vertical';
  const targetPosition = isVertical ? Position.Top : Position.Left;
  const sourcePosition = isVertical ? Position.Bottom : Position.Right;

  // Position classes based on layout and size
  const targetClassName = isVertical
    ? (size === 'small' ? '!-top-1' : '!-top-1.5')
    : undefined;
  const sourceClassName = isVertical
    ? (size === 'small' ? '!-bottom-1' : '!-bottom-1.5')
    : undefined;

  return (
    <>
      <SmartHandle
        type="target"
        position={targetPosition}
        color={color}
        selected={selected}
        size={size}
        className={targetClassName}
      />
      <SmartHandle
        type="source"
        position={sourcePosition}
        color={color}
        selected={selected}
        size={size}
        className={sourceClassName}
      />
    </>
  );
});
