'use client';

/**
 * NavigationModeToggle - 4-mode segmented toggle for navigation modes
 *
 * Modes: Data | Meta | Overlay | Query
 * Each mode has a distinct icon and accent color.
 * Pressing N cycles through modes. Clicking sets directly.
 * Triggers Matrix transition animation via animationStore.
 */

import { memo, useCallback } from 'react';
import { Database, Boxes, Layers, Search } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { NavigationMode } from '@/stores/uiStore';
import { useAnimationStore } from '@/stores/animationStore';
import { gapTokens } from '@/design/tokens';
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipShortcut,
} from '@/components/ui/tooltip';

interface NavigationModeToggleProps {
  /** Current navigation mode */
  mode: NavigationMode;
  /** Called when mode changes */
  onModeChange: (mode: NavigationMode) => void;
  /** Additional CSS classes */
  className?: string;
}

const MODE_CONFIG: {
  mode: NavigationMode;
  label: string;
  icon: typeof Database;
  accent: string;
  activeClass: string;
  description: string;
}[] = [
  {
    mode: 'data',
    label: 'Data',
    icon: Database,
    accent: 'emerald',
    activeClass: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/40',
    description: 'Browse real data instances',
  },
  {
    mode: 'meta',
    label: 'Meta',
    icon: Boxes,
    accent: 'blue',
    activeClass: 'bg-blue-500/20 text-blue-400 border-blue-500/40',
    description: 'View schema meta-graph (35 types)',
  },
  {
    mode: 'overlay',
    label: 'Overlay',
    icon: Layers,
    accent: 'violet',
    activeClass: 'bg-violet-500/20 text-violet-400 border-violet-500/40',
    description: 'Data + schema combined',
  },
  {
    mode: 'query',
    label: 'Query',
    icon: Search,
    accent: 'amber',
    activeClass: 'bg-amber-500/20 text-amber-400 border-amber-500/40',
    description: 'Faceted filter query',
  },
];

export const NavigationModeToggle = memo(function NavigationModeToggle({
  mode,
  onModeChange: _onModeChange,
  className,
}: NavigationModeToggleProps) {
  const startTransition = useAnimationStore((s) => s.startTransition);

  const handleModeClick = useCallback(
    (targetMode: NavigationMode) => {
      if (targetMode === mode) return;
      // Trigger Matrix transition animation, then mode switch happens in page.tsx
      startTransition(targetMode);
    },
    [mode, startTransition]
  );

  return (
    <div
      className={cn(
        // Outer container - bold skeuomorphic bezel
        'relative flex items-center rounded-xl p-[3px]',
        // Deep multi-layer background
        'bg-gradient-to-b from-[#252530] via-[#18181f] to-[#0a0a0f]',
        // Pronounced outer border with top highlight
        'border border-black/60',
        'ring-1 ring-inset ring-white/[0.06]',
        // Heavy inner shadow for deep inset
        'shadow-[inset_0_2px_4px_rgba(0,0,0,0.8),inset_0_-1px_0_rgba(255,255,255,0.04)]',
        // Bold outer shadow
        'shadow-xl shadow-black/60',
        gapTokens.compact,
        className
      )}
    >
      {/* Inner track - deeply recessed groove */}
      <div
        className={cn(
          'absolute inset-[3px] rounded-lg',
          'bg-gradient-to-b from-black/70 to-black/50',
          'shadow-[inset_0_3px_6px_rgba(0,0,0,0.9),inset_0_1px_2px_rgba(0,0,0,0.5)]'
        )}
      />

      {MODE_CONFIG.map(({ mode: m, label, icon: Icon, activeClass, description }) => {
        const isActive = m === mode;
        return (
          <Tooltip key={m}>
            <TooltipTrigger asChild>
              <button
                type="button"
                onClick={() => handleModeClick(m)}
                className={cn(
                  'relative z-10 flex items-center rounded-lg px-2.5 py-1.5 text-[11px] font-semibold tracking-wide transition-all duration-150',
                  gapTokens.compact,
                  isActive
                    ? cn(
                        activeClass,
                        'border',
                        // Bold raised button - 3D pop effect
                        'shadow-[0_3px_8px_rgba(0,0,0,0.5),0_1px_3px_rgba(0,0,0,0.3),inset_0_1px_0_rgba(255,255,255,0.15),inset_0_-1px_0_rgba(0,0,0,0.2)]',
                        // Slight transform for depth
                        'translate-y-[-1px]'
                      )
                    : cn(
                        'text-white/35 hover:text-white/55 border border-transparent',
                        'hover:bg-white/[0.03]',
                        // Pressed-in look for inactive
                        'shadow-[inset_0_1px_2px_rgba(0,0,0,0.3)]'
                      )
                )}
              >
                <Icon className="w-3.5 h-3.5" />
                <span>{label}</span>
              </button>
            </TooltipTrigger>
            <TooltipContent>
              {description}
              <TooltipShortcut>N</TooltipShortcut>
            </TooltipContent>
          </Tooltip>
        );
      })}
    </div>
  );
});
