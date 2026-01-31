'use client';

/**
 * RelationshipsSection - Relationship types explorer
 *
 * Features:
 * - List of relationship types with counts
 * - Tri-state checkbox for bulk selection
 * - Progress bars showing counts
 * - Execute query button
 */

import { memo, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { ACTION_ICONS, STATUS_ICONS } from '@/config/iconSystem';
import { iconSizes } from '@/design/tokens';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';
import { ProgressBar } from '@/components/ui/ProgressBar';
import { getRelationshipColor } from '@/config/relationshipColors';
import { calculateCheckboxState } from '@/hooks';
import type { RelationType } from '@/hooks';

// Design system icons
const LoaderIcon = STATUS_ICONS.loading;
const PlayIcon = ACTION_ICONS.execute;
const CheckIcon = STATUS_ICONS.success;

// =============================================================================
// RELATIONSHIP ROW
// =============================================================================

interface RelationshipRowProps {
  type: string;
  count: number;
  maxCount: number;
  isSelected: boolean;
  onToggle: () => void;
  disabled?: boolean;
}

const RelationshipRow = memo(function RelationshipRow({
  type,
  count,
  maxCount,
  isSelected,
  onToggle,
  disabled,
}: RelationshipRowProps) {
  const color = getRelationshipColor(type);

  return (
    <button
      onClick={onToggle}
      disabled={disabled}
      role="checkbox"
      aria-checked={isSelected}
      aria-label={`${type} (${count} relationships)`}
      className={cn(
        'group w-full flex items-center gap-3 py-1.5 px-2 -mx-2 rounded-lg transition-all duration-200',
        'hover:bg-white/[0.04] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
        isSelected && 'bg-white/[0.06]',
        disabled && 'opacity-50 cursor-not-allowed'
      )}
    >
      {/* Checkbox */}
      <div
        className={cn(
          'w-3.5 h-3.5 rounded border-[1.5px] flex items-center justify-center transition-all duration-200',
          isSelected ? 'border-transparent' : 'border-white/[0.06]'
        )}
        style={{
          backgroundColor: isSelected ? `${color}30` : 'transparent',
          borderColor: isSelected ? color : undefined,
        }}
      >
        {isSelected && <CheckIcon className={iconSizes.xs} style={{ color }} />}
      </div>

      {/* Label */}
      <span
        className={cn(
          'text-[11px] font-mono transition-colors duration-200 min-w-[120px] text-left truncate',
          isSelected ? 'text-white' : 'text-white/60 group-hover:text-white/80'
        )}
      >
        {type}
      </span>

      {/* Progress bar */}
      <ProgressBar value={count} max={maxCount} color={color} />

      {/* Count */}
      <span
        className={cn(
          'text-[11px] font-mono w-8 text-right transition-colors duration-200',
          isSelected ? 'text-white/80' : 'text-white/40'
        )}
      >
        {count > 999 ? `${(count / 1000).toFixed(1)}k` : count}
      </span>
    </button>
  );
});

// =============================================================================
// MAIN SECTION COMPONENT
// =============================================================================

export interface RelationshipsSectionProps {
  /** Total relationships in database */
  totalRelationships: number;
  /** Relationship types with counts */
  relationshipTypes: RelationType[];
  /** Maximum count for progress bars */
  maxCount: number;
  /** Currently selected relationship types */
  selectedRelTypes: Set<string>;
  /** Callback when relationship type is toggled */
  onToggleRelType: (type: string) => void;
  /** Callback when all relationships toggled */
  onToggleAllRelTypes: () => void;
  /** Callback to execute query */
  onExecuteQuery: () => void;
  /** Whether query is executing */
  isExecuting?: boolean;
}

export const RelationshipsSection = memo(function RelationshipsSection({
  relationshipTypes,
  maxCount,
  selectedRelTypes,
  onToggleRelType,
  onToggleAllRelTypes,
  onExecuteQuery,
  isExecuting = false,
}: RelationshipsSectionProps) {
  // Calculate checkbox state
  const checkboxState = useMemo((): CheckboxState => {
    if (!relationshipTypes.length) return 'none';
    return calculateCheckboxState(
      relationshipTypes.map((r) => r.type),
      selectedRelTypes
    );
  }, [relationshipTypes, selectedRelTypes]);

  // Execute button component
  const executeButton = (
    <button
      onClick={onExecuteQuery}
      disabled={selectedRelTypes.size === 0 || isExecuting}
      aria-label={`Execute query for ${selectedRelTypes.size} selected relationship types`}
      className={cn(
        'p-1.5 rounded-lg transition-all duration-200',
        selectedRelTypes.size > 0
          ? 'text-violet-400 hover:text-violet-300 hover:bg-violet-500/10 hover:scale-110'
          : 'text-white/20 cursor-not-allowed'
      )}
      title="Execute query for selected relationships"
    >
      {isExecuting ? (
        <LoaderIcon className={`${iconSizes.md} animate-spin`} />
      ) : (
        <PlayIcon className={iconSizes.md} />
      )}
    </button>
  );

  return (
    <section data-testid="relationships-container">
      {/* Compact action bar */}
      <div className="flex items-center justify-between px-1 py-1.5 mb-2">
        <button
          onClick={onToggleAllRelTypes}
          className="text-[10px] text-white/40 hover:text-white/60 transition-colors"
        >
          {checkboxState === 'all' ? 'Deselect all' : 'Select all'}
        </button>
        <div className="flex items-center gap-2">
          {selectedRelTypes.size > 0 && (
            <span className="text-[10px] text-white/30">
              {selectedRelTypes.size} selected
            </span>
          )}
          {executeButton}
        </div>
      </div>

      {/* Relationship List */}
      <div className="space-y-0.5">
        {relationshipTypes.map((item) => (
          <RelationshipRow
            key={item.type}
            type={item.type}
            count={item.count}
            maxCount={maxCount}
            isSelected={selectedRelTypes.has(item.type)}
            onToggle={() => onToggleRelType(item.type)}
            disabled={isExecuting}
          />
        ))}
      </div>
    </section>
  );
});
