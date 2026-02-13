'use client';

/**
 * ViewPresetSelector - Quick view presets from novanet-core v7.2.3 filterAdapter
 *
 * Features:
 * - 3-column grid layout for compact display
 * - Keyboard shortcut badges (1-8, 0)
 * - Uses VIEW_PRESETS from filterAdapter
 * - Connects to filterStore.applyViewPreset
 *
 * @see VIEW_PRESETS in src/lib/filterAdapter.ts
 */

import { memo, useCallback, useRef } from 'react';
import {
  Building2,
  Link,
  Globe,
  Network,
  FileText,
  Search,
  Lock,
  Languages,
  Grid3x3,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { iconSizes, gapTokens } from '@/design/tokens';
import { VIEW_PRESETS, type ViewPreset } from '@/lib/filterAdapter';
import { useFilterStore } from '@/stores/filterStore';
import { useGridNavigation } from '@/hooks/useGridNavigation';

// Map preset IDs to Lucide icons for consistent SVG rendering
// v11.8: Renamed per ADR-024 Data Origin semantics
const PRESET_ICONS: Record<string, LucideIcon> = {
  'project-structure': Building2,
  'generation-chain': Link,
  'locale-knowledge': Globe,
  'concept-network': Network,
  'prompts-rules': FileText,
  'seo-geo': Search,
  'defined-types': Lock,       // was: invariant-types
  'authored-content': Languages, // was: localized-content
  'all-nodes': Grid3x3,
};

interface ViewPresetSelectorProps {
  className?: string;
  onSelect?: (presetId: string) => void;
  activePresetId?: string;
}

export const ViewPresetSelector = memo(function ViewPresetSelector({
  className,
  onSelect,
  activePresetId,
}: ViewPresetSelectorProps) {
  const { applyViewPreset, activePresetId: storeActiveId } = useFilterStore(
    useShallow((state) => ({
      applyViewPreset: state.applyViewPreset,
      activePresetId: state.activePresetId,
    }))
  );
  const activeId = activePresetId ?? storeActiveId;

  const handleSelect = useCallback(
    (preset: ViewPreset) => {
      applyViewPreset(preset.id);
      onSelect?.(preset.id);
    },
    [applyViewPreset, onSelect]
  );

  // Grid keyboard navigation (3 columns, 9 presets)
  const gridRef = useRef<HTMLDivElement>(null);
  const { focusedIndex, handleKeyDown } = useGridNavigation({
    columns: 3,
    totalItems: VIEW_PRESETS.length,
    gridRef,
    onSelect: (index) => handleSelect(VIEW_PRESETS[index]),
    onEscape: () => gridRef.current?.blur(),
  });

  return (
    <div className={cn('space-y-1', className)}>
      {/* Header */}
      <div className={cn('flex items-center px-3 py-2 text-xs text-white/40', gapTokens.default)}>
        <Grid3x3 className={iconSizes.sm} />
        <span className="uppercase tracking-wider font-medium">Quick Views</span>
      </div>

      {/* 3-column grid of presets with arrow key navigation */}
      <div
        ref={gridRef}
        role="grid"
        aria-label="Quick view presets"
        onKeyDown={handleKeyDown}
        className={cn('grid grid-cols-3 px-2', gapTokens.tight)}
      >
        {VIEW_PRESETS.map((preset, index) => {
          const isFocused = focusedIndex === index;
          return (
            <button
              key={preset.id}
              onClick={() => handleSelect(preset)}
              tabIndex={isFocused ? 0 : -1}
              aria-pressed={activeId === preset.id}
              aria-label={`${preset.name}: ${preset.description}`}
              className={cn(
                'flex flex-col items-center px-2 py-2.5 rounded-lg text-center',
                gapTokens.compact,
                'transition-colors duration-200',
                'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
                activeId === preset.id
                  ? 'bg-white/[0.1] border border-white/[0.15] text-white'
                  : isFocused
                    ? 'bg-white/[0.06] border border-white/[0.12] text-white/80'
                    : 'bg-white/[0.03] border border-white/[0.06] text-white/60 hover:bg-white/[0.06] hover:text-white/80'
              )}
            >
              {/* Icon */}
              {(() => {
                const IconComponent = PRESET_ICONS[preset.id] || Grid3x3;
                return (
                  <IconComponent
                    className={cn(
                      iconSizes.xl,
                      'transition-transform duration-200',
                      (activeId === preset.id || isFocused) && 'scale-110'
                    )}
                    strokeWidth={2}
                  />
                );
              })()}

              {/* First word of preset name (compact display) */}
              <span className="text-[10px] font-medium leading-tight truncate w-full">
                {preset.name.split(' ')[0]}
              </span>

              {/* Keyboard shortcut badge */}
              {preset.shortcut && (
                <kbd className="text-[9px] px-1.5 py-0.5 bg-white/[0.08] rounded text-white/40 font-mono">
                  {preset.shortcut}
                </kbd>
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
});
