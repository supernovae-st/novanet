'use client';

/**
 * useAnimationShortcut - Keyboard shortcut for animation mode toggle
 *
 * Handles: ⇧E (Shift+E) to cycle through Full → Reduced → Off → Full
 *
 * @example
 * useAnimationShortcut(); // In main app component
 */

import { useCallback, useEffect } from 'react';
import { useAnimationStore } from '@/stores/animationStore';
import { isInputFocused } from '@/lib/keyboard';

interface UseAnimationShortcutOptions {
  /** Whether the shortcut is enabled (default: true) */
  enabled?: boolean;
  /** Callback when mode changes */
  onModeChange?: (mode: 'full' | 'reduced' | 'off') => void;
}

/**
 * Hook to handle Shift+E keyboard shortcut for animation mode cycling
 */
export function useAnimationShortcut(options: UseAnimationShortcutOptions = {}) {
  const { enabled = true, onModeChange } = options;

  const cycleMode = useAnimationStore((state) => state.cycleMode);
  const currentMode = useAnimationStore((state) => state.settings.mode);

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Skip if in input element
      if (isInputFocused()) return;

      // Check for Shift+E
      if (event.shiftKey && event.key.toLowerCase() === 'e') {
        event.preventDefault();
        cycleMode();

        // Get the new mode after cycling
        const modes: ('full' | 'reduced' | 'off')[] = ['full', 'reduced', 'off'];
        const currentIndex = modes.indexOf(currentMode);
        const nextIndex = (currentIndex + 1) % modes.length;
        const newMode = modes[nextIndex];

        onModeChange?.(newMode);
      }
    },
    [cycleMode, currentMode, onModeChange]
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [enabled, handleKeyDown]);

  return {
    currentMode,
    cycleMode,
  };
}

/**
 * Mode labels for display
 */
export const ANIMATION_MODE_LABELS: Record<'full' | 'reduced' | 'off', string> = {
  full: 'Full Effects',
  reduced: 'Reduced',
  off: 'Off',
};

/**
 * Get a toast message for mode change
 */
export function getAnimationModeToast(mode: 'full' | 'reduced' | 'off'): string {
  switch (mode) {
    case 'full':
      return '✨ Animation Effects: Full';
    case 'reduced':
      return '🌙 Animation Effects: Reduced';
    case 'off':
      return '⏸️ Animation Effects: Off';
  }
}
