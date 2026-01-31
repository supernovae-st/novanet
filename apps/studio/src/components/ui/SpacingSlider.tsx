/**
 * SpacingSlider - Real-time graph spacing control
 *
 * Allows users to adjust the spacing between nodes in the graph visualization.
 * Supports both preset selection and continuous value adjustment.
 *
 * Features:
 * - Preset buttons: Compact, Normal, Spacious
 * - Continuous slider for fine-tuning
 * - Real-time preview (value updates immediately)
 * - Keyboard shortcut: Shift+S to cycle presets
 */

'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import { useUIStore, selectSpacingPreset, selectSpacingValue } from '@/stores/uiStore';
import type { SpacingPreset } from '@/lib/forceSimulation';
import { Minimize2, Maximize2, Grid3X3 } from 'lucide-react';

interface SpacingSliderProps {
  className?: string;
  /** Show preset buttons (default: true) */
  showPresets?: boolean;
  /** Show value label (default: true) */
  showLabel?: boolean;
  /** Compact mode - hides labels, shows only slider */
  compact?: boolean;
}

const PRESET_CONFIG: Record<SpacingPreset, { icon: React.ReactNode; label: string; value: number }> = {
  compact: { icon: <Minimize2 className="w-3.5 h-3.5" />, label: 'Compact', value: 0 },
  normal: { icon: <Grid3X3 className="w-3.5 h-3.5" />, label: 'Normal', value: 50 },
  spacious: { icon: <Maximize2 className="w-3.5 h-3.5" />, label: 'Spacious', value: 100 },
};

export function SpacingSlider({
  className,
  showPresets = true,
  showLabel = true,
  compact = false,
}: SpacingSliderProps) {
  const spacingPreset = useUIStore(selectSpacingPreset);
  const spacingValue = useUIStore(selectSpacingValue);
  const setSpacingPreset = useUIStore((s) => s.setSpacingPreset);
  const setSpacingValue = useUIStore((s) => s.setSpacingValue);
  const triggerLayout = useUIStore((s) => s.triggerLayout);
  const layoutDirection = useUIStore((s) => s.layoutDirection);

  const handleSliderChange = React.useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const value = Number(e.target.value);
      setSpacingValue(value);
      // Trigger layout recalculation with current direction
      // Only 'force' and 'dagre' modes use spacing (they use force simulation)
      // For TB/LR/radial, switch to 'dagre' to show the effect
      const effectiveDirection = layoutDirection === 'TB' || layoutDirection === 'LR' || layoutDirection === 'radial'
        ? 'dagre'
        : layoutDirection;
      triggerLayout(effectiveDirection);
    },
    [setSpacingValue, triggerLayout, layoutDirection]
  );

  const handlePresetClick = React.useCallback(
    (preset: SpacingPreset) => {
      setSpacingPreset(preset);
      // Same logic: use dagre if current mode doesn't support spacing
      const effectiveDirection = layoutDirection === 'TB' || layoutDirection === 'LR' || layoutDirection === 'radial'
        ? 'dagre'
        : layoutDirection;
      triggerLayout(effectiveDirection);
    },
    [setSpacingPreset, triggerLayout, layoutDirection]
  );

  if (compact) {
    return (
      <div className={cn('flex items-center gap-2', className)}>
        <input
          type="range"
          min={0}
          max={100}
          value={spacingValue}
          onChange={handleSliderChange}
          className="w-24 h-1.5 bg-white/10 rounded-full appearance-none cursor-pointer
            [&::-webkit-slider-thumb]:appearance-none
            [&::-webkit-slider-thumb]:w-3
            [&::-webkit-slider-thumb]:h-3
            [&::-webkit-slider-thumb]:rounded-full
            [&::-webkit-slider-thumb]:bg-primary
            [&::-webkit-slider-thumb]:shadow-lg
            [&::-webkit-slider-thumb]:shadow-primary/30
            [&::-webkit-slider-thumb]:transition-transform
            [&::-webkit-slider-thumb]:hover:scale-125
            [&::-moz-range-thumb]:w-3
            [&::-moz-range-thumb]:h-3
            [&::-moz-range-thumb]:rounded-full
            [&::-moz-range-thumb]:bg-primary
            [&::-moz-range-thumb]:border-0"
          aria-label="Node spacing"
        />
        <span className="text-xs text-white/50 w-8">{spacingValue}%</span>
      </div>
    );
  }

  return (
    <div className={cn('space-y-2', className)}>
      {showLabel && (
        <div className="flex items-center justify-between">
          <span className="text-xs font-medium text-white/70">Spacing</span>
          <span className="text-xs text-white/50">{spacingValue}%</span>
        </div>
      )}

      {/* Slider track with glow effect */}
      <div className="relative">
        <div
          className="absolute inset-0 rounded-full bg-gradient-to-r from-white/5 via-primary/20 to-white/5"
          style={{
            clipPath: `inset(0 ${100 - spacingValue}% 0 0 round 9999px)`,
          }}
        />
        <input
          type="range"
          min={0}
          max={100}
          value={spacingValue}
          onChange={handleSliderChange}
          className="relative w-full h-2 bg-white/10 rounded-full appearance-none cursor-pointer
            [&::-webkit-slider-thumb]:appearance-none
            [&::-webkit-slider-thumb]:w-4
            [&::-webkit-slider-thumb]:h-4
            [&::-webkit-slider-thumb]:rounded-full
            [&::-webkit-slider-thumb]:bg-primary
            [&::-webkit-slider-thumb]:shadow-lg
            [&::-webkit-slider-thumb]:shadow-primary/40
            [&::-webkit-slider-thumb]:transition-all
            [&::-webkit-slider-thumb]:duration-150
            [&::-webkit-slider-thumb]:hover:scale-110
            [&::-webkit-slider-thumb]:hover:shadow-primary/60
            [&::-moz-range-thumb]:w-4
            [&::-moz-range-thumb]:h-4
            [&::-moz-range-thumb]:rounded-full
            [&::-moz-range-thumb]:bg-primary
            [&::-moz-range-thumb]:border-0
            [&::-moz-range-thumb]:shadow-lg
            [&::-moz-range-thumb]:shadow-primary/40"
          aria-label="Node spacing"
        />
      </div>

      {/* Preset buttons */}
      {showPresets && (
        <div className="flex items-center gap-1">
          {(Object.keys(PRESET_CONFIG) as SpacingPreset[]).map((preset) => {
            const config = PRESET_CONFIG[preset];
            const isActive = spacingPreset === preset;
            return (
              <button
                key={preset}
                onClick={() => handlePresetClick(preset)}
                className={cn(
                  'flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-xs font-medium',
                  'transition-all duration-150',
                  isActive
                    ? 'bg-primary/20 text-primary border border-primary/30 shadow-sm shadow-primary/20'
                    : 'bg-white/5 text-white/60 border border-transparent hover:bg-white/10 hover:text-white/80'
                )}
                title={`${config.label} spacing (${config.value}%)`}
              >
                {config.icon}
                <span className="hidden sm:inline">{config.label}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}

export default SpacingSlider;
