'use client';

/**
 * SubcategoryGroupNode - Container node for subcategory groups in Schema Mode
 *
 * Features:
 * - Smaller container than ScopeGroupNode
 * - Subtle border with glassmorphism
 * - Icon + label + count badge
 * - Nested inside ScopeGroupNode
 *
 * Subcategories per Scope:
 * - Project: foundation, structure, semantic, instruction, output
 * - Global: config, knowledge
 * - Shared: seo, geo
 */

import { memo } from 'react';
import { type NodeProps } from '@xyflow/react';
import { cn } from '@/lib/utils';

/**
 * Data interface for SubcategoryGroupNode
 */
export interface SubcategoryGroupData {
  subcategory: string;
  label: string;
  icon: string;
  nodeCount: number;
}

/**
 * SubcategoryGroupNode - Nested container within a scope
 *
 * Used in Schema Mode to group related node types by subcategory
 * (e.g., foundation, structure, semantic within Project scope).
 */
export const SubcategoryGroupNode = memo(function SubcategoryGroupNode({
  data,
  selected,
}: NodeProps<SubcategoryGroupData>) {
  return (
    <div
      className={cn(
        'w-full h-full rounded-lg border border-white/10 bg-white/5',
        selected && 'ring-1 ring-white/20'
      )}
    >
      {/* Label badge positioned above the container */}
      <div className="absolute -top-5 left-2 flex items-center gap-1.5 px-1.5 py-0.5 rounded bg-black/60 backdrop-blur-sm">
        <span className="text-xs text-white/70">
          {data.icon} {data.label}
        </span>
        <span className="text-xs text-white/40">({data.nodeCount})</span>
      </div>
    </div>
  );
});
