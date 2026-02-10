'use client';

/**
 * ViewModeToggle - 2D/3D graph view toggle
 *
 * Replaces meta/data toggle at bottom center
 * Shortcuts: 1 = 2D, 2 = 3D
 * Design: Matches Pill component (solid dark, rounded-2xl)
 */

import { memo, useCallback, useEffect } from 'react';
import { Grid2x2, Box } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { ViewMode } from '@/types';

interface ViewModeToggleProps {
  mode: ViewMode;
  onModeChange: (mode: ViewMode) => void;
  className?: string;
}

const MODES: {
  id: ViewMode;
  label: string;
  icon: typeof Grid2x2;
  color: string;
  bg: string;
  border: string;
  glow: string;
  key: string;
}[] = [
  {
    id: '2d',
    label: '2D graph',
    icon: Grid2x2,
    color: 'text-blue-400',
    bg: 'bg-blue-500/15',
    border: 'border-blue-500/40',
    glow: 'shadow-[0_0_12px_rgba(59,130,246,0.3)]',
    key: '1',
  },
  {
    id: '3d',
    label: '3D graph',
    icon: Box,
    color: 'text-purple-400',
    bg: 'bg-purple-500/15',
    border: 'border-purple-500/40',
    glow: 'shadow-[0_0_12px_rgba(168,85,247,0.3)]',
    key: '2',
  },
];

export const ViewModeToggle = memo(function ViewModeToggle({
  mode,
  onModeChange,
  className,
}: ViewModeToggleProps) {
  const handleSwitch = useCallback(
    (target: ViewMode) => {
      if (target === mode) return;
      onModeChange(target);
    },
    [mode, onModeChange]
  );

  // Keyboard shortcuts: 1 = 2D, 2 = 3D
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

      const keyMap: Record<string, ViewMode> = {
        '1': '2d',
        '2': '3d',
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
      aria-label="View mode selector"
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
            aria-label={`Switch to ${label} view (keyboard shortcut: ${key})`}
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
