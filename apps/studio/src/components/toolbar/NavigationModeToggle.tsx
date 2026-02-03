'use client';

/**
 * NavigationModeToggle - 4-mode segmented toggle
 *
 * Order: Meta | Data | Overlay | Query
 * Shortcuts: 1, 2, 3, 4
 * Design: Matches Pill component (solid dark, rounded-2xl)
 */

import { memo, useCallback, useEffect } from 'react';
import { Boxes, Database, Layers, Search } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { NavigationMode } from '@/stores/uiStore';
import { useAnimationStore } from '@/stores/animationStore';

interface NavigationModeToggleProps {
  mode: NavigationMode;
  onModeChange: (mode: NavigationMode) => void;
  className?: string;
}

const MODES: {
  id: NavigationMode;
  label: string;
  icon: typeof Database;
  color: string;
  bg: string;
  border: string;
  glow: string;
  key: string;
}[] = [
  {
    id: 'meta',
    label: 'meta',
    icon: Boxes,
    color: 'text-blue-400',
    bg: 'bg-blue-500/15',
    border: 'border-blue-500/40',
    glow: 'shadow-[0_0_12px_rgba(59,130,246,0.3)]',
    key: '1',
  },
  {
    id: 'data',
    label: 'data',
    icon: Database,
    color: 'text-emerald-400',
    bg: 'bg-emerald-500/15',
    border: 'border-emerald-500/40',
    glow: 'shadow-[0_0_12px_rgba(16,185,129,0.3)]',
    key: '2',
  },
  {
    id: 'overlay',
    label: 'overlay',
    icon: Layers,
    color: 'text-violet-400',
    bg: 'bg-violet-500/15',
    border: 'border-violet-500/40',
    glow: 'shadow-[0_0_12px_rgba(139,92,246,0.3)]',
    key: '3',
  },
  {
    id: 'query',
    label: 'query',
    icon: Search,
    color: 'text-amber-400',
    bg: 'bg-amber-500/15',
    border: 'border-amber-500/40',
    glow: 'shadow-[0_0_12px_rgba(245,158,11,0.3)]',
    key: '4',
  },
];

export const NavigationModeToggle = memo(function NavigationModeToggle({
  mode,
  onModeChange: _onModeChange,
  className,
}: NavigationModeToggleProps) {
  const startTransition = useAnimationStore((s) => s.startTransition);

  const handleSwitch = useCallback(
    (target: NavigationMode) => {
      if (target === mode) return;
      startTransition(target);
    },
    [mode, startTransition]
  );

  // Keyboard shortcuts: 1, 2, 3, 4
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if typing in input/textarea
      if (
        e.target instanceof HTMLInputElement ||
        e.target instanceof HTMLTextAreaElement ||
        (e.target as HTMLElement)?.isContentEditable
      ) {
        return;
      }

      const keyMap: Record<string, NavigationMode> = {
        '1': 'meta',
        '2': 'data',
        '3': 'overlay',
        '4': 'query',
      };

      const targetMode = keyMap[e.key];
      if (targetMode) {
        e.preventDefault();
        handleSwitch(targetMode);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleSwitch]);

  return (
    <div
      role="group"
      aria-label="Navigation mode selector"
      className={cn(
        'flex items-center gap-1 p-1.5 rounded-2xl',
        'bg-[#0a0a0f]',
        'border border-white/10',
        'shadow-2xl shadow-black/60',
        'ring-1 ring-white/[0.03] ring-inset',
        className
      )}
    >
      {MODES.map(({ id, label, icon: Icon, color, bg, border, glow, key }) => {
        const isActive = id === mode;

        return (
          <button
            key={id}
            type="button"
            aria-current={isActive ? 'true' : undefined}
            aria-label={`Switch to ${label} mode (keyboard shortcut: ${key})`}
            onClick={() => handleSwitch(id)}
            className={cn(
              'flex items-center gap-1.5 px-2.5 py-1.5 rounded-xl',
              'font-mono text-[11px] tracking-tight',
              'transition-all duration-200',
              isActive && [color, bg, 'border', border, glow],
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
            <kbd
              className={cn(
                'ml-0.5 px-1 py-0.5 rounded text-[9px]',
                'bg-white/5 border border-white/10',
                isActive ? 'text-white/50' : 'text-white/20'
              )}
            >
              {key}
            </kbd>
          </button>
        );
      })}
    </div>
  );
});
