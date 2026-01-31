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

import { memo, useCallback } from 'react';
import {
  Building2,
  Link,
  Globe,
  Network,
  FileText,
  Search,
  AlertCircle,
  Zap,
  Grid3x3,
  type LucideIcon,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { VIEW_PRESETS, type ViewPreset } from '@/lib/filterAdapter';
import { useFilterStore } from '@/stores/filterStore';

// Map preset IDs to Lucide icons for consistent SVG rendering
const PRESET_ICONS: Record<string, LucideIcon> = {
  'project-structure': Building2,
  'generation-chain': Link,
  'locale-knowledge': Globe,
  'concept-network': Network,
  'prompts-rules': FileText,
  'seo-geo': Search,
  'high-priority': AlertCircle,
  'realtime': Zap,
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
  const applyViewPreset = useFilterStore((state) => state.applyViewPreset);
  const storeActiveId = useFilterStore((state) => state.activePresetId);
  const activeId = activePresetId ?? storeActiveId;

  const handleSelect = useCallback(
    (preset: ViewPreset) => {
      applyViewPreset(preset.id);
      onSelect?.(preset.id);
    },
    [applyViewPreset, onSelect]
  );

  return (
    <div className={cn('space-y-1', className)}>
      {/* Header */}
      <div className="flex items-center gap-2 px-3 py-2 text-xs text-white/40">
        <Grid3x3 className={iconSizes.sm} />
        <span className="uppercase tracking-wider font-medium">Quick Views</span>
      </div>

      {/* 3-column grid of presets */}
      <div className="grid grid-cols-3 gap-1 px-2">
        {VIEW_PRESETS.map((preset) => (
          <button
            key={preset.id}
            onClick={() => handleSelect(preset)}
            aria-pressed={activeId === preset.id}
            aria-label={`${preset.name}: ${preset.description}`}
            className={cn(
              'flex flex-col items-center gap-1.5 px-2 py-2.5 rounded-lg text-center',
              'transition-all duration-200',
              'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
              activeId === preset.id
                ? 'bg-white/[0.1] border border-white/[0.15] text-white'
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
                    activeId === preset.id && 'scale-110'
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
        ))}
      </div>
    </div>
  );
});
