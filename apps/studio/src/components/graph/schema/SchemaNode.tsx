'use client';

/**
 * SchemaNode - Individual node type card in Schema Mode
 *
 * Features:
 * - Node card with scope-colored left border accent
 * - Label and nodeType display
 * - Source and target handles for edge connections
 * - Glassmorphism styling matching NovaNet design
 *
 * Scope Accent Colors:
 * - Project: violet left border
 * - Global: emerald left border
 * - Shared: amber left border
 */

import { memo } from 'react';
import { type NodeProps, type Node, Handle, Position } from '@xyflow/react';
import { cn } from '@/lib/utils';
import type { Scope } from '@novanet/core/types';

/**
 * Data interface for SchemaNode
 */
export interface SchemaNodeData extends Record<string, unknown> {
  nodeType: string;
  label: string;
  description: string;
  scope: Scope;
  subcategory: string;
}

/** Node type for SchemaNode */
export type SchemaNodeType = Node<SchemaNodeData, 'schemaNode'>;

/**
 * Scope accent color mapping for left border
 * Matches ScopeGroupNode color scheme
 */
const SCOPE_ACCENT: Record<Scope, string> = {
  Project: 'border-l-violet-500',
  Global: 'border-l-emerald-500',
  Shared: 'border-l-amber-500',
};

/**
 * SchemaNode - Individual node type representation
 *
 * Used in Schema Mode to display a single node type (e.g., Project, Concept, Locale).
 * Features scope-colored left border accent and connection handles.
 */
export const SchemaNode = memo(function SchemaNode({
  data,
  selected,
}: NodeProps<SchemaNodeType>) {
  const accentClass = SCOPE_ACCENT[data.scope] || 'border-l-gray-500';

  return (
    <div
      className={cn(
        'px-3 py-2 rounded-md bg-black/80 backdrop-blur-sm border border-white/10',
        'border-l-4',
        accentClass,
        selected && 'ring-2 ring-white/30'
      )}
    >
      {/* Target handle - left side (incoming connections) */}
      <Handle
        type="target"
        position={Position.Left}
        className="w-2 h-2 !bg-white/50"
      />

      {/* Node content */}
      <div className="text-sm font-medium text-white/90">{data.label}</div>
      <div className="text-xs text-white/50 truncate max-w-[120px]">
        {data.nodeType}
      </div>

      {/* Source handle - right side (outgoing connections) */}
      <Handle
        type="source"
        position={Position.Right}
        className="w-2 h-2 !bg-white/50"
      />
    </div>
  );
});
