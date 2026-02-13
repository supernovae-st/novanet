'use client';

/**
 * KeyboardHelpPanel - Modal showing all keyboard shortcuts (?)
 *
 * Uses Modal compound component + overlayClasses for unified design
 * with CommandPalette (⌘K) and AiSearchOverlay (⌘J).
 *
 * Features:
 * - Tabs for Studio, Pad, TUI keybindings
 * - Inline search header (same pattern as ⌘K/⌘J)
 * - Grouped by category with unified section headers
 * - Keyboard navigation (Escape to close)
 * - Focus trap via Modal.Root
 * - Raycast-style spring animation (animate-overlay-enter)
 * - Visual encoding reference for 2D/3D views
 */

import { memo, useState, useEffect, useRef, useMemo } from 'react';
import {
  Search,
  X,
  Circle,
  Square,
  Triangle,
  Hexagon,
  Monitor,
  Keyboard,
  Terminal,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { SHORTCUTS } from '@/config/shortcuts';
import { TUI_KEYBINDINGS, PAD_LAYERS, TUI_CATEGORIES, type KeyBinding } from '@/config/keybindings';
import { KeyboardKey, KeyboardShortcut } from '@/components/ui/KeyboardKey';
import { Kbd } from '@/components/ui/Kbd';
import { Modal } from '@/components/ui/Modal';
import { overlayClasses, gapTokens, iconSizes } from '@/design/tokens';
import { useAutoFocus } from '@/hooks';
import type { Shortcut } from '@/lib/keyboard';

// =============================================================================
// Tab Types
// =============================================================================

type TabType = 'studio' | 'pad' | 'tui';

const TABS: { id: TabType; label: string; icon: React.ReactNode }[] = [
  { id: 'studio', label: 'Studio', icon: <Monitor className="w-4 h-4" /> },
  { id: 'pad', label: 'Pad', icon: <Keyboard className="w-4 h-4" /> },
  { id: 'tui', label: 'TUI', icon: <Terminal className="w-4 h-4" /> },
];

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

// =============================================================================
// TUI Keybinding Components
// =============================================================================

/**
 * TUI Shortcut row
 */
const TuiShortcutRow = memo(function TuiShortcutRow({ binding }: { binding: KeyBinding }) {
  return (
    <div className={cn(
      overlayClasses.rowBase,
      overlayClasses.rowIdle,
      'justify-between',
      gapTokens.spacious,
    )}>
      <div className="flex flex-col gap-0.5 min-w-0">
        <span className="text-sm font-medium text-white/80 truncate">{binding.label}</span>
        {binding.description && (
          <span className="text-xs text-white/40 truncate">{binding.description}</span>
        )}
      </div>
      <KeyboardShortcut keys={[binding.key.toUpperCase()]} size="sm" />
    </div>
  );
});

/**
 * Group TUI bindings by category
 */
function groupTuiByCategory(bindings: KeyBinding[]): Map<string, KeyBinding[]> {
  const groups = new Map<string, KeyBinding[]>();
  for (const binding of bindings) {
    if (!groups.has(binding.category)) {
      groups.set(binding.category, []);
    }
    groups.get(binding.category)!.push(binding);
  }
  return groups;
}

/**
 * TUI Category section
 */
const TuiCategorySection = memo(function TuiCategorySection({
  category,
  bindings,
}: {
  category: string;
  bindings: KeyBinding[];
}) {
  const config = TUI_CATEGORIES[category] || { label: category, order: 99 };

  return (
    <div className="mb-2">
      <div className={overlayClasses.sectionHeader}>
        {config.label}
      </div>
      <div className="space-y-0.5">
        {bindings.map((binding) => (
          <TuiShortcutRow key={binding.id} binding={binding} />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// Pad Keybinding Components
// =============================================================================

/**
 * Pad Layer Section - shows keys in a visual grid
 */
const PadLayerSection = memo(function PadLayerSection({
  layer,
}: {
  layer: typeof PAD_LAYERS[0];
}) {
  return (
    <div className="mb-4">
      <div className={cn(overlayClasses.sectionHeader, 'flex items-center gap-2')}>
        <span
          className="w-2.5 h-2.5 rounded-full"
          style={{ backgroundColor: layer.color }}
        />
        <span>Layer {layer.id}: {layer.name}</span>
      </div>
      <p className="text-xs text-white/40 mb-3 px-2">{layer.description}</p>

      {/* 3x4 Grid */}
      <div className="grid grid-cols-4 gap-1.5 px-2">
        {layer.keys.map((key) => (
          <div
            key={key.position}
            className={cn(
              'p-2 rounded-lg border text-center',
              key.action === 'NONE'
                ? 'bg-white/[0.02] border-white/[0.05] text-white/20'
                : 'bg-white/[0.04] border-white/[0.1]'
            )}
          >
            <div className="text-xs font-medium text-white/80">{key.label || '—'}</div>
            <div className="text-[10px] font-mono text-white/40 mt-0.5">{key.key}</div>
          </div>
        ))}
      </div>

      {/* Encoder info */}
      {layer.encoder && (
        <div className="mt-2 px-2 flex items-center gap-3 text-xs text-white/50">
          <span className="flex items-center gap-1">
            <span className="text-white/30">↻</span>
            <span>{layer.encoder.cw.label}</span>
          </span>
          <span className="flex items-center gap-1">
            <span className="text-white/30">↺</span>
            <span>{layer.encoder.ccw.label}</span>
          </span>
        </div>
      )}
    </div>
  );
});

// =============================================================================
// Visual Encoding Section
// =============================================================================

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
          maps="Trait (defined, authored, imported, generated, retrieved)"
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
          maps="Trait (glass=defined, metal=authored, glow=imported, pulse=generated)"
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
  const [activeTab, setActiveTab] = useState<TabType>('studio');
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Auto-focus search input when opened
  useAutoFocus(searchInputRef, isOpen);

  // Reset search and tab when opened
  useEffect(() => {
    if (isOpen) {
      setSearch('');
    }
  }, [isOpen]);

  // Filter Studio shortcuts by search
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

  // Group filtered Studio shortcuts by category
  const groupedShortcuts = useMemo(() => {
    const groups = groupByCategory(filteredShortcuts);
    // Sort by category order
    return Array.from(groups.entries()).sort((a, b) => {
      const orderA = CATEGORY_CONFIG[a[0]]?.order ?? 99;
      const orderB = CATEGORY_CONFIG[b[0]]?.order ?? 99;
      return orderA - orderB;
    });
  }, [filteredShortcuts]);

  // Filter TUI bindings by search
  const filteredTuiBindings = useMemo(() => {
    if (!search.trim()) return TUI_KEYBINDINGS;

    const searchLower = search.toLowerCase();
    return TUI_KEYBINDINGS.filter(
      (b) =>
        b.label.toLowerCase().includes(searchLower) ||
        b.description?.toLowerCase().includes(searchLower) ||
        b.key.toLowerCase().includes(searchLower)
    );
  }, [search]);

  // Group filtered TUI bindings by category
  const groupedTuiBindings = useMemo(() => {
    const groups = groupTuiByCategory(filteredTuiBindings);
    return Array.from(groups.entries()).sort((a, b) => {
      const orderA = TUI_CATEGORIES[a[0]]?.order ?? 99;
      const orderB = TUI_CATEGORIES[b[0]]?.order ?? 99;
      return orderA - orderB;
    });
  }, [filteredTuiBindings]);

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
        {/* Tabs + Search Header */}
        <div className="border-b border-white/[0.08]">
          {/* Tabs */}
          <div className="flex gap-1 px-3 pt-3">
            {TABS.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={cn(
                  'flex items-center gap-1.5 px-3 py-2 rounded-t-lg text-sm font-medium transition-colors',
                  activeTab === tab.id
                    ? 'bg-white/[0.08] text-white border-b-2 border-novanet-500'
                    : 'text-white/50 hover:text-white hover:bg-white/[0.04]'
                )}
              >
                {tab.icon}
                {tab.label}
              </button>
            ))}
          </div>

          {/* Search Header - unified inline search */}
          <div className={cn(overlayClasses.searchHeader, gapTokens.spacious)}>
            <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
            <input
              ref={searchInputRef}
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder={
                activeTab === 'studio' ? 'Search shortcuts…' :
                activeTab === 'tui' ? 'Search TUI bindings…' :
                'Search pad keys…'
              }
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
        </div>

        {/* Content based on active tab */}
        <Modal.Body maxHeight={overlayClasses.bodyMaxHeight}>
          <div className={cn('p-2', overlayClasses.contentAnimation)}>
            {/* Studio Tab */}
            {activeTab === 'studio' && (
              <>
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
              </>
            )}

            {/* Pad Tab */}
            {activeTab === 'pad' && (
              <>
                <div className="mb-3 p-3 bg-novanet-500/10 border border-novanet-500/20 rounded-xl">
                  <p className="text-xs text-novanet-300">
                    Work Louder Creator Micro — 3×4 matrix + encoder
                  </p>
                  <p className="text-[11px] text-white/40 mt-1">
                    Press <Kbd>P</Kbd> to open the full pad configurator
                  </p>
                </div>
                {PAD_LAYERS.map((layer) => (
                  <PadLayerSection key={layer.id} layer={layer} />
                ))}
              </>
            )}

            {/* TUI Tab */}
            {activeTab === 'tui' && (
              <>
                {groupedTuiBindings.length === 0 ? (
                  <div className="py-8 text-center text-white/40 text-sm">
                    No TUI bindings found for &ldquo;{search}&rdquo;
                  </div>
                ) : (
                  <>
                    <div className="mb-3 p-3 bg-emerald-500/10 border border-emerald-500/20 rounded-xl">
                      <p className="text-xs text-emerald-300">
                        NovaNet TUI — Terminal interface keybindings
                      </p>
                      <p className="text-[11px] text-white/40 mt-1">
                        Run <code className="font-mono bg-white/10 px-1 rounded">cargo run -- tui</code> in tools/novanet
                      </p>
                    </div>
                    {groupedTuiBindings.map(([category, bindings]) => (
                      <TuiCategorySection key={category} category={category} bindings={bindings} />
                    ))}
                  </>
                )}
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
