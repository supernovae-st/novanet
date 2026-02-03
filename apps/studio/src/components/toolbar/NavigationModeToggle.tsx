'use client';

/**
 * NavigationModeToggle - 4-mode segmented toggle
 *
 * Order: Meta | Data | Overlay | Query
 * Design: Matches Pill component (solid dark, rounded-2xl)
 * Typography: Monospace code style
 */

import { memo, useCallback } from 'react';
import { Boxes, Database, Layers, Search } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { NavigationMode } from '@/stores/uiStore';
import { useAnimationStore } from '@/stores/animationStore';
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipShortcut,
} from '@/components/ui/tooltip';

interface NavigationModeToggleProps {
  mode: NavigationMode;
  onModeChange: (mode: NavigationMode) => void;
  className?: string;
}

// Mode order: Meta first (schema exploration), then Data, Overlay, Query
const MODES: {
  id: NavigationMode;
  label: string;
  icon: typeof Database;
  color: string;
  bg: string;
  border: string;
  glow: string;
  desc: string;
  shortcut: string;
}[] = [
  {
    id: 'meta',
    label: 'meta',
    icon: Boxes,
    color: 'text-blue-400',
    bg: 'bg-blue-500/15',
    border: 'border-blue-500/40',
    glow: 'shadow-[0_0_12px_rgba(59,130,246,0.3)]',
    desc: 'Schema meta-graph',
    shortcut: '1',
  },
  {
    id: 'data',
    label: 'data',
    icon: Database,
    color: 'text-emerald-400',
    bg: 'bg-emerald-500/15',
    border: 'border-emerald-500/40',
    glow: 'shadow-[0_0_12px_rgba(16,185,129,0.3)]',
    desc: 'Data instances',
    shortcut: '2',
  },
  {
    id: 'overlay',
    label: 'overlay',
    icon: Layers,
    color: 'text-violet-400',
    bg: 'bg-violet-500/15',
    border: 'border-violet-500/40',
    glow: 'shadow-[0_0_12px_rgba(139,92,246,0.3)]',
    desc: 'Data + schema',
    shortcut: '3',
  },
  {
    id: 'query',
    label: 'query',
    icon: Search,
    color: 'text-amber-400',
    bg: 'bg-amber-500/15',
    border: 'border-amber-500/40',
    glow: 'shadow-[0_0_12px_rgba(245,158,11,0.3)]',
    desc: 'Faceted query',
    shortcut: '4',
  },
];

export const NavigationModeToggle = memo(function NavigationModeToggle({
  mode,
  onModeChange: _onModeChange,
  className,
}: NavigationModeToggleProps) {
  const startTransition = useAnimationStore((s) => s.startTransition);

  const handleClick = useCallback(
    (target: NavigationMode) => {
      if (target === mode) return;
      startTransition(target);
    },
    [mode, startTransition]
  );

  return (
    <div
      className={cn(
        // Container - matches Pill design
        'flex items-center gap-1 p-1.5 rounded-2xl',
        'bg-[#0a0a0f]',
        'border border-white/10',
        'shadow-2xl shadow-black/60',
        'ring-1 ring-white/[0.03] ring-inset',
        className
      )}
    >
      {MODES.map(({ id, label, icon: Icon, color, bg, border, glow, desc, shortcut }) => {
        const isActive = id === mode;

        return (
          <Tooltip key={id}>
            <TooltipTrigger asChild>
              <button
                type="button"
                onClick={() => handleClick(id)}
                className={cn(
                  // Base
                  'flex items-center gap-1.5 px-3 py-1.5 rounded-xl',
                  'font-mono text-[11px] tracking-tight',
                  'transition-all duration-200',
                  // Active state
                  isActive && [
                    color,
                    bg,
                    'border',
                    border,
                    glow,
                  ],
                  // Inactive state
                  !isActive && [
                    'text-white/30',
                    'border border-transparent',
                    'hover:text-white/50',
                    'hover:bg-white/[0.02]',
                  ]
                )}
              >
                <Icon className="w-3.5 h-3.5" strokeWidth={isActive ? 2 : 1.5} />
                <span>{label}</span>
              </button>
            </TooltipTrigger>
            <TooltipContent sideOffset={12}>
              {desc}
              <TooltipShortcut>{shortcut}</TooltipShortcut>
            </TooltipContent>
          </Tooltip>
        );
      })}
    </div>
  );
});
