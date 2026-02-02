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
        'flex items-center rounded-lg border border-white/10 bg-[#0d0d12]/90 backdrop-blur-sm',
        gapTokens.compact,
        'p-0.5',
        className
      )}
    >
      {MODE_CONFIG.map(({ mode: m, label, icon: Icon, activeClass, description }) => {
        const isActive = m === mode;
        return (
          <Tooltip key={m}>
            <TooltipTrigger asChild>
              <button
                type="button"
                onClick={() => handleModeClick(m)}
                className={cn(
                  'flex items-center rounded-md px-2.5 py-1.5 text-xs font-medium transition-all duration-150',
                  gapTokens.compact,
                  isActive
                    ? cn(activeClass, 'border')
                    : 'text-white/40 hover:text-white/70 hover:bg-white/[0.05] border border-transparent'
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
