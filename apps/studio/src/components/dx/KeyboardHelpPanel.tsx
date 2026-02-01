'use client';

/**
 * KeyboardHelpPanel - Modal showing all keyboard shortcuts (?)
 *
 * Uses Modal compound component + overlayClasses for unified design
 * with CommandPalette (⌘K) and AiSearchOverlay (⌘J).
 *
 * Features:
 * - Inline search header (same pattern as ⌘K/⌘J)
 * - Grouped by category with unified section headers
 * - Keyboard navigation (Escape to close)
 * - Focus trap via Modal.Root
 * - Raycast-style spring animation (animate-overlay-enter)
 */

import { memo, useState, useEffect, useRef, useMemo } from 'react';
import { Search, X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { SHORTCUTS } from '@/config/shortcuts';
import { KeyboardKey, KeyboardShortcut } from '@/components/ui/KeyboardKey';
import { Kbd } from '@/components/ui/Kbd';
import { Modal } from '@/components/ui/Modal';
import { overlayClasses, gapTokens, iconSizes } from '@/design/tokens';
import { useAutoFocus } from '@/hooks';
import type { Shortcut } from '@/lib/keyboard';

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
 * Shortcut row - matches overlayClasses.rowBase styling
 */
const ShortcutRow = memo(function ShortcutRow({ shortcut }: { shortcut: Shortcut }) {
  const keyParts = formatKeyCombo(shortcut.keys);

  return (
    <div className={cn(
      overlayClasses.rowBase,
      overlayClasses.rowIdle,
      'justify-between',
      gapTokens.spacious,
    )}>
      <div className="flex flex-col gap-0.5 min-w-0">
        <span className="text-sm font-medium text-white/80 truncate">{shortcut.label}</span>
        {shortcut.description && (
          <span className="text-xs text-white/40 truncate">{shortcut.description}</span>
        )}
      </div>
      <KeyboardShortcut keys={keyParts} size="sm" />
    </div>
  );
});

/**
 * Category section - uses unified overlayClasses.sectionHeader
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
    <div className="mb-2">
      <div className={overlayClasses.sectionHeader}>
        {config.label}
      </div>
      <div className="space-y-0.5">
        {shortcuts.map((shortcut) => (
          <ShortcutRow key={shortcut.id} shortcut={shortcut} />
        ))}
      </div>
    </div>
  );
});

/**
 * Keyboard Help Panel - unified overlay modal
 */
export const KeyboardHelpPanel = memo(function KeyboardHelpPanel({
  isOpen,
  onClose,
}: KeyboardHelpPanelProps) {
  const [search, setSearch] = useState('');
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Auto-focus search input when opened
  useAutoFocus(searchInputRef, isOpen);

  // Reset search when opened
  useEffect(() => {
    if (isOpen) {
      setSearch('');
    }
  }, [isOpen]);

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

  // Handle key press in search
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
  };

  return (
    <Modal.Root
      isOpen={isOpen}
      onClose={onClose}
      closeOnEscape={false}
      containerClassName={overlayClasses.position}
    >
      <Modal.Content
        size={overlayClasses.size}
        ariaLabel="Keyboard shortcuts"
        className={overlayClasses.animation}
      >
        {/* Search Header - unified inline search */}
        <div className={cn(overlayClasses.searchHeader, gapTokens.spacious)}>
          <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
          <input
            ref={searchInputRef}
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Search shortcuts…"
            aria-label="Search shortcuts"
            className={overlayClasses.searchInput}
            autoComplete="off"
            spellCheck={false}
          />
          {search && (
            <button
              onClick={() => setSearch('')}
              aria-label="Clear search"
              className="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white/60"
            >
              <X className={iconSizes.md} />
            </button>
          )}
          <KeyboardKey size="md" className="hidden sm:inline-flex">?</KeyboardKey>
        </div>

        {/* Shortcuts List */}
        <Modal.Body maxHeight={overlayClasses.bodyMaxHeight}>
          <div className={cn('p-2', overlayClasses.contentAnimation)}>
            {groupedShortcuts.length === 0 ? (
              <div className="py-8 text-center text-white/40 text-sm">
                No shortcuts found for &ldquo;{search}&rdquo;
              </div>
            ) : (
              groupedShortcuts.map(([category, shortcuts]) => (
                <CategorySection key={category} category={category} shortcuts={shortcuts} />
              ))
            )}
          </div>
        </Modal.Body>

        {/* Footer - unified dark bar with keyboard hints */}
        <Modal.Footer className={overlayClasses.footer}>
          <div className={cn(overlayClasses.footerContent, gapTokens.large)}>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>↑↓</Kbd>
              <span>Navigate</span>
            </span>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>Esc</Kbd>
              <span>Close</span>
            </span>
          </div>
        </Modal.Footer>
      </Modal.Content>
    </Modal.Root>
  );
});
