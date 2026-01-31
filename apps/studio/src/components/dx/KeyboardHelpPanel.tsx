'use client';

/**
 * KeyboardHelpPanel - Modal showing all keyboard shortcuts
 *
 * Features:
 * - Grouped by category
 * - Searchable
 * - Accessible (focus trap, escape to close)
 * - Shows platform-specific modifier keys
 */

import { memo, useState, useEffect, useRef, useMemo, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { SHORTCUTS } from '@/config/shortcuts';
import { KeyboardKey, KeyboardShortcut } from '@/components/ui/KeyboardKey';
import { ACTION_ICONS } from '@/config/iconSystem';
import { glassClasses, gapTokens, iconSizes } from '@/design/tokens';
import type { Shortcut } from '@/lib/keyboard';

const CloseIcon = ACTION_ICONS.close;

export interface KeyboardHelpPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

// Category display names and order
const CATEGORY_CONFIG: Record<string, { label: string; order: number }> = {
  navigation: { label: 'Navigation', order: 1 },
  view: { label: 'View', order: 2 },
  filter: { label: 'Search & Filter', order: 3 },
  preset: { label: 'Quick Views (Presets)', order: 4 },
  layout: { label: 'Layout', order: 5 },
  action: { label: 'Actions', order: 6 },
};

/**
 * Format a key combo for display
 * Converts "mod+shift+k" to ["⌘", "⇧", "K"] on Mac
 */
function formatKeyCombo(keys: string): string[] {
  const isMac = typeof navigator !== 'undefined' &&
    navigator.platform.toUpperCase().indexOf('MAC') >= 0;

  return keys.split('+').map((part) => {
    switch (part.toLowerCase()) {
      case 'mod':
        return isMac ? '⌘' : 'Ctrl';
      case 'shift':
        return '⇧';
      case 'alt':
        return isMac ? '⌥' : 'Alt';
      case 'escape':
        return 'Esc';
      case 'enter':
        return '↵';
      case 'tab':
        return '⇥';
      case 'delete':
        return '⌫';
      case 'arrowup':
        return '↑';
      case 'arrowdown':
        return '↓';
      case 'arrowleft':
        return '←';
      case 'arrowright':
        return '→';
      case 'space':
        return '␣';
      default:
        return part.toUpperCase();
    }
  });
}

/**
 * Group shortcuts by category
 */
function groupByCategory(shortcuts: Shortcut[]): Map<string, Shortcut[]> {
  const groups = new Map<string, Shortcut[]>();

  for (const shortcut of shortcuts) {
    const category = shortcut.category;
    if (!groups.has(category)) {
      groups.set(category, []);
    }
    groups.get(category)!.push(shortcut);
  }

  return groups;
}

/**
 * Shortcut row component
 */
const ShortcutRow = memo(function ShortcutRow({ shortcut }: { shortcut: Shortcut }) {
  const keyParts = formatKeyCombo(shortcut.keys);

  return (
    <div className="flex items-center justify-between py-2 px-3 rounded-lg hover:bg-white/5 transition-colors">
      <div className="flex flex-col gap-0.5">
        <span className="text-sm text-white/90">{shortcut.label}</span>
        {shortcut.description && (
          <span className="text-xs text-white/50">{shortcut.description}</span>
        )}
      </div>
      <KeyboardShortcut keys={keyParts} size="sm" />
    </div>
  );
});

/**
 * Category section component
 */
const CategorySection = memo(function CategorySection({
  category,
  shortcuts,
}: {
  category: string;
  shortcuts: Shortcut[];
}) {
  const config = CATEGORY_CONFIG[category] || { label: category, order: 99 };

  return (
    <div className="space-y-1">
      <h3 className="text-xs font-semibold text-white/60 uppercase tracking-wider px-3 py-2">
        {config.label}
      </h3>
      <div className="space-y-0.5">
        {shortcuts.map((shortcut) => (
          <ShortcutRow key={shortcut.id} shortcut={shortcut} />
        ))}
      </div>
    </div>
  );
});

/**
 * Keyboard Help Panel
 */
export const KeyboardHelpPanel = memo(function KeyboardHelpPanel({
  isOpen,
  onClose,
}: KeyboardHelpPanelProps) {
  const [search, setSearch] = useState('');
  const panelRef = useRef<HTMLDivElement>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Focus search input when panel opens
  useEffect(() => {
    if (isOpen && searchInputRef.current) {
      // Small delay to ensure panel is rendered
      const timer = setTimeout(() => {
        searchInputRef.current?.focus();
      }, 50);
      return () => clearTimeout(timer);
    }
  }, [isOpen]);

  // Handle escape key
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  // Filter shortcuts by search
  const filteredShortcuts = useMemo(() => {
    if (!search.trim()) return SHORTCUTS;

    const searchLower = search.toLowerCase();
    return SHORTCUTS.filter(
      (s) =>
        s.label.toLowerCase().includes(searchLower) ||
        s.description?.toLowerCase().includes(searchLower) ||
        s.keys.toLowerCase().includes(searchLower)
    );
  }, [search]);

  // Group filtered shortcuts by category
  const groupedShortcuts = useMemo(() => {
    const groups = groupByCategory(filteredShortcuts);
    // Sort by category order
    return Array.from(groups.entries()).sort((a, b) => {
      const orderA = CATEGORY_CONFIG[a[0]]?.order ?? 99;
      const orderB = CATEGORY_CONFIG[b[0]]?.order ?? 99;
      return orderA - orderB;
    });
  }, [filteredShortcuts]);

  // Handle click outside
  const handleBackdropClick = useCallback(
    (e: React.MouseEvent) => {
      if (e.target === e.currentTarget) {
        onClose();
      }
    },
    [onClose]
  );

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
      onClick={handleBackdropClick}
      role="dialog"
      aria-modal="true"
      aria-labelledby="keyboard-help-title"
    >
      <div
        ref={panelRef}
        className={cn(
          'w-full max-w-2xl max-h-[80vh] m-4 overflow-hidden rounded-2xl',
          glassClasses.modal,
          'border border-white/15 shadow-2xl'
        )}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-white/10">
          <h2 id="keyboard-help-title" className="text-lg font-semibold text-white">
            Keyboard Shortcuts
          </h2>
          <button
            onClick={onClose}
            className={cn(
              'p-2 rounded-lg transition-colors',
              'text-white/50 hover:text-white hover:bg-white/10',
              'focus:outline-none focus-visible:ring-2 focus-visible:ring-novanet-accent'
            )}
            aria-label="Close keyboard shortcuts"
          >
            <CloseIcon className={iconSizes.md} />
          </button>
        </div>

        {/* Search */}
        <div className="p-4 border-b border-white/10">
          <input
            ref={searchInputRef}
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search shortcuts..."
            className={cn(
              'w-full px-4 py-2.5 rounded-xl',
              'bg-white/5 border border-white/10',
              'text-white placeholder:text-white/40',
              'focus:outline-none focus:ring-2 focus:ring-novanet-accent focus:border-transparent',
              'transition-colors'
            )}
          />
        </div>

        {/* Shortcuts list */}
        <div className={cn('overflow-y-auto p-4', gapTokens.spacious, 'max-h-[calc(80vh-140px)]')}>
          {groupedShortcuts.length === 0 ? (
            <div className="text-center py-8 text-white/50">
              No shortcuts found for "{search}"
            </div>
          ) : (
            <div className={cn('space-y-6')}>
              {groupedShortcuts.map(([category, shortcuts]) => (
                <CategorySection key={category} category={category} shortcuts={shortcuts} />
              ))}
            </div>
          )}
        </div>

        {/* Footer hint */}
        <div className="px-4 py-3 border-t border-white/10 text-center">
          <span className="text-xs text-white/40">
            Press <KeyboardKey size="sm">Esc</KeyboardKey> to close
          </span>
        </div>
      </div>
    </div>
  );
});
