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
 * - Visual encoding reference for 2D/3D views
 */

import { memo, useState, useEffect, useRef, useMemo } from 'react';
import { Search, X, Circle, Square, Triangle, Hexagon } from 'lucide-react';
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
 * Visual encoding row - for displaying visual channel mappings
 */
const VisualEncodingRow = memo(function VisualEncodingRow({
  channel,
  maps,
  visual,
}: {
  channel: string;
  maps: string;
  visual: React.ReactNode;
}) {
  return (
    <div className={cn(
      overlayClasses.rowBase,
      overlayClasses.rowIdle,
      'justify-between',
      gapTokens.spacious,
    )}>
      <div className="flex flex-col gap-0.5 min-w-0">
        <span className="text-sm font-medium text-white/80">{channel}</span>
        <span className="text-xs text-white/40">{maps}</span>
      </div>
      <div className="flex items-center gap-2">
        {visual}
      </div>
    </div>
  );
});

/**
 * Visual Encoding Section - shows how visual channels map to graph properties
 */
const VisualEncodingSection = memo(function VisualEncodingSection() {
  return (
    <div className="mb-2">
      <div className={overlayClasses.sectionHeader}>
        Visual Encoding (2D)
      </div>
      <div className="space-y-0.5">
        <VisualEncodingRow
          channel="Fill Color"
          maps="Layer (config, locale, geography, knowledge, semantic, output...)"
          visual={
            <div className="flex gap-1">
              <div className="w-3 h-3 rounded-sm bg-[#6366f1]" title="config" />
              <div className="w-3 h-3 rounded-sm bg-[#a855f7]" title="locale" />
              <div className="w-3 h-3 rounded-sm bg-[#14b8a6]" title="knowledge" />
              <div className="w-3 h-3 rounded-sm bg-[#f97316]" title="output" />
            </div>
          }
        />
        <VisualEncodingRow
          channel="Border Color"
          maps="Realm (shared = teal, org = blue)"
          visual={
            <div className="flex gap-1">
              <div className="w-3 h-3 rounded-sm border-2 border-[#2aa198]" title="shared" />
              <div className="w-3 h-3 rounded-sm border-2 border-[#0ea5e9]" title="org" />
            </div>
          }
        />
        <VisualEncodingRow
          channel="Border Style"
          maps="Trait (invariant, localized, knowledge, generated, aggregated)"
          visual={
            <div className="flex gap-1.5 text-[10px] font-mono text-white/60">
              <span className="border-b-2 border-solid border-white/60 px-1">solid</span>
              <span className="border-b-2 border-dashed border-white/60 px-1">dash</span>
              <span className="border-b-2 border-dotted border-white/60 px-1">dot</span>
            </div>
          }
        />
        <VisualEncodingRow
          channel="Arc Color"
          maps="ArcFamily (ownership, localization, semantic, generation, mining)"
          visual={
            <div className="flex gap-1">
              <div className="w-6 h-0.5 bg-[#6366f1]" title="ownership" />
              <div className="w-6 h-0.5 bg-[#06b6d4]" title="localization" />
              <div className="w-6 h-0.5 bg-[#f59e0b]" title="semantic" />
            </div>
          }
        />
      </div>

      <div className={cn(overlayClasses.sectionHeader, 'mt-3')}>
        Visual Encoding (3D)
      </div>
      <div className="space-y-0.5">
        <VisualEncodingRow
          channel="Shape"
          maps="Layer (sphere, box, octahedron, tetrahedron, torus...)"
          visual={
            <div className="flex gap-1.5 text-white/60">
              <Circle className="w-4 h-4" />
              <Square className="w-4 h-4" />
              <Hexagon className="w-4 h-4" />
              <Triangle className="w-4 h-4" />
            </div>
          }
        />
        <VisualEncodingRow
          channel="Material"
          maps="Trait (glass=invariant, metal=localized, glow=knowledge, pulse=generated)"
          visual={
            <div className="flex gap-1.5 text-[10px] font-mono text-white/60">
              <span className="text-cyan-300">glass</span>
              <span className="text-amber-300">metal</span>
              <span className="text-emerald-300">glow</span>
            </div>
          }
        />
        <VisualEncodingRow
          channel="Outline"
          maps="Realm (teal glow = shared, blue glow = org)"
          visual={
            <div className="flex gap-1.5">
              <div className="w-4 h-4 rounded-full bg-[#2aa198]/30 ring-2 ring-[#2aa198]" />
              <div className="w-4 h-4 rounded-full bg-[#0ea5e9]/30 ring-2 ring-[#0ea5e9]" />
            </div>
          }
        />
        <VisualEncodingRow
          channel="Arc Particles"
          maps="ArcFamily (particle color & density match arc family)"
          visual={
            <div className="flex gap-1 text-[10px]">
              <span className="text-indigo-400">●●●</span>
              <span className="text-cyan-400">●●</span>
              <span className="text-amber-400">●</span>
            </div>
          }
        />
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

  return (
    <Modal.Root
      isOpen={isOpen}
      onClose={onClose}
      closeOnEscape={true}
      closeOnOutsideClick={true}
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
              <>
                {groupedShortcuts.map(([category, shortcuts]) => (
                  <CategorySection key={category} category={category} shortcuts={shortcuts} />
                ))}
                {/* Visual Encoding Reference - only show when not searching */}
                {!search.trim() && <VisualEncodingSection />}
              </>
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
