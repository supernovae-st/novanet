'use client';

/**
 * CommandPalette - VS Code/Linear style command palette (⌘K)
 *
 * Features:
 * - Fuzzy search across all commands
 * - Keyboard navigation (↑/↓, Enter, Escape)
 * - Categorized commands
 * - Uses Modal compound component
 * - Focus trap for accessibility (WCAG 2.1 AA)
 * - Auto-focus search input
 */

import { useEffect, useState, useCallback, useMemo, useRef } from 'react';
import {
  X,
  Search,
  Command,
  Eye,
  Hash,
  RefreshCw,
  Maximize2,
  Map,
  Tag,
  LayoutGrid,
  Globe,
  Keyboard,
  PanelLeft,
  PanelRight,
  Layers,
  Box,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { iconSizes } from '@/design/tokens';
import { fuzzyMatch } from '@/lib/fuzzySearch';
import { useAutoFocus, useDebouncedValue } from '@/hooks';
import { KeyboardKey } from './KeyboardKey';
import { Kbd } from './Kbd';
import { Modal } from './Modal';

// =============================================================================
// Types
// =============================================================================

interface CommandItem {
  id: string;
  title: string;
  description?: string;
  shortcut?: string[];
  icon: React.ReactNode;
  category: string;
  action: () => void;
}

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  commands: CommandItem[];
}

// =============================================================================
// Component
// =============================================================================

export function CommandPalette({ isOpen, onClose, commands }: CommandPaletteProps) {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  // Auto-focus input when opened
  useAutoFocus(inputRef, isOpen);

  // Reset state when opened
  useEffect(() => {
    if (isOpen) {
      setQuery('');
      setSelectedIndex(0);
    }
  }, [isOpen]);

  // Debounce search query to avoid filtering on every keystroke
  const debouncedQuery = useDebouncedValue(query, 100);

  // Filter and sort commands using debounced query
  const filteredCommands = useMemo(() => {
    if (!debouncedQuery) return commands;

    return commands
      .map((cmd) => {
        const titleMatch = fuzzyMatch(debouncedQuery, cmd.title);
        const descMatch = fuzzyMatch(debouncedQuery, cmd.description || '');
        const categoryMatch = fuzzyMatch(debouncedQuery, cmd.category);

        const bestScore = Math.max(titleMatch.score * 2, descMatch.score, categoryMatch.score);
        const isMatch = titleMatch.match || descMatch.match || categoryMatch.match;

        return { ...cmd, score: bestScore, isMatch };
      })
      .filter((cmd) => cmd.isMatch)
      .sort((a, b) => b.score - a.score);
  }, [commands, debouncedQuery]);

  // Group by category
  const groupedCommands = useMemo(() => {
    const groups: Record<string, typeof filteredCommands> = {};
    for (const cmd of filteredCommands) {
      if (!groups[cmd.category]) {
        groups[cmd.category] = [];
      }
      groups[cmd.category].push(cmd);
    }
    return groups;
  }, [filteredCommands]);

  // Flatten for keyboard navigation
  const flatCommands = useMemo(() => {
    return Object.values(groupedCommands).flat();
  }, [groupedCommands]);

  // Handle keyboard navigation
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          setSelectedIndex((prev) => Math.min(prev + 1, flatCommands.length - 1));
          break;
        case 'ArrowUp':
          e.preventDefault();
          setSelectedIndex((prev) => Math.max(prev - 1, 0));
          break;
        case 'Enter':
          e.preventDefault();
          if (flatCommands[selectedIndex]) {
            flatCommands[selectedIndex].action();
            onClose();
          }
          break;
        case 'Escape':
          e.preventDefault();
          onClose();
          break;
      }
    },
    [flatCommands, selectedIndex, onClose]
  );

  // Scroll selected item into view
  useEffect(() => {
    if (listRef.current && flatCommands.length > 0) {
      const selectedEl = listRef.current.querySelector(`[data-index="${selectedIndex}"]`);
      selectedEl?.scrollIntoView({ block: 'nearest' });
    }
  }, [selectedIndex, flatCommands.length]);

  // Reset selected index when filtered results change
  useEffect(() => {
    setSelectedIndex(0);
  }, [debouncedQuery]);

  let globalIndex = 0;

  return (
    <Modal.Root
      isOpen={isOpen}
      onClose={onClose}
      closeOnEscape={false} // We handle Escape in handleKeyDown
      containerClassName="items-start pt-[15vh]"
    >
      <Modal.Content
        size="md"
        ariaLabel="Command palette"
        className="animate-scale-in"
      >
        {/* Search Header - opacity.border.light (0.08) */}
        <div
          className="flex items-center gap-3 p-4 border-b border-white/[0.08]"
          onKeyDown={handleKeyDown}
        >
          <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Search commands..."
            className="flex-1 bg-transparent text-white placeholder-white/40 text-base outline-none border-none ring-0 focus:outline-none focus:ring-0"
            autoComplete="off"
            spellCheck={false}
          />
          {query && (
            <button
              onClick={() => setQuery('')}
              aria-label="Clear search"
              className="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white/60"
            >
              <X className={iconSizes.md} />
            </button>
          )}
          <KeyboardKey size="md" className="hidden sm:inline-flex">⌘K</KeyboardKey>
        </div>

        {/* Commands List */}
        <Modal.Body maxHeight="50vh">
          <div ref={listRef} className="p-2">
            {flatCommands.length === 0 ? (
              <div className="py-8 text-center text-white/40 text-sm">No commands found</div>
            ) : (
              Object.entries(groupedCommands).map(([category, cmds]) => (
                <div key={category} className="mb-2">
                  {/* Category Header */}
                  <div className="px-3 py-2 text-xs font-medium text-white/40 uppercase tracking-wider">
                    {category}
                  </div>
                  {/* Commands */}
                  {cmds.map((cmd) => {
                    const currentIndex = globalIndex++;
                    const isSelected = currentIndex === selectedIndex;

                    return (
                      <button
                        key={cmd.id}
                        data-index={currentIndex}
                        onClick={() => {
                          cmd.action();
                          onClose();
                        }}
                        onMouseEnter={() => setSelectedIndex(currentIndex)}
                        className={cn(
                          'w-full flex items-center gap-3 px-3 py-2.5 rounded-xl transition-all',
                          'outline-none ring-0 focus:outline-none focus:ring-0',
                          isSelected
                            ? 'bg-novanet-500/20 border border-novanet-500/30'
                            : 'hover:bg-white/[0.04] border border-transparent' // opacity.bg.light (0.04) on hover
                        )}
                      >
                        {/* Icon - opacity.bg.medium (0.06) when not selected */}
                        <div
                          className={cn(
                            'w-8 h-8 rounded-lg flex items-center justify-center shrink-0',
                            isSelected ? 'bg-novanet-500/30 text-novanet-400' : 'bg-white/[0.06] text-white/50'
                          )}
                        >
                          {cmd.icon}
                        </div>

                        {/* Text */}
                        <div className="flex-1 text-left min-w-0">
                          <div
                            className={cn(
                              'text-sm font-medium truncate',
                              isSelected ? 'text-white' : 'text-white/80'
                            )}
                          >
                            {cmd.title}
                          </div>
                          {cmd.description && (
                            <div className="text-xs text-white/40 truncate">{cmd.description}</div>
                          )}
                        </div>

                        {/* Shortcut */}
                        {cmd.shortcut && (
                          <div className="flex items-center gap-0.5 shrink-0">
                            {cmd.shortcut.map((key, keyIdx) => (
                              <span key={`${cmd.id}-key-${keyIdx}`} className="flex items-center">
                                <KeyboardKey size="sm">{key}</KeyboardKey>
                                {cmd.shortcut && keyIdx < cmd.shortcut.length - 1 && (
                                  <span className="text-white/20 mx-0.5 text-[10px]">+</span>
                                )}
                              </span>
                            ))}
                          </div>
                        )}
                      </button>
                    );
                  })}
                </div>
              ))
            )}
          </div>
        </Modal.Body>

        {/* Footer */}
        <Modal.Footer className="p-3 bg-black/20">
          <div className="flex items-center justify-center gap-4 text-xs text-white/50">
            <span className="flex items-center gap-1.5">
              <Kbd>↑↓</Kbd>
              <span>Navigate</span>
            </span>
            <span className="flex items-center gap-1.5">
              <Kbd>↵</Kbd>
              <span>Select</span>
            </span>
            <span className="flex items-center gap-1.5">
              <Kbd>Esc</Kbd>
              <span>Close</span>
            </span>
          </div>
        </Modal.Footer>
      </Modal.Content>
    </Modal.Root>
  );
}

// =============================================================================
// Hook - Creates command list from app state
// =============================================================================

interface UseCommandPaletteOptions {
  toggleViewMode: () => void;
  toggleFocusMode: () => void;
  toggleMinimap: () => void;
  toggleEdgeLabels: () => void;
  toggleSidebar: () => void;
  openShortcuts: () => void;
  applyViewPresetByShortcut: (key: string) => void;
  onRefresh: () => void;
  clearSelection: () => void;
}

export function useCommandPalette(options: UseCommandPaletteOptions): CommandItem[] {
  const {
    toggleViewMode,
    toggleFocusMode,
    toggleMinimap,
    toggleEdgeLabels,
    toggleSidebar,
    openShortcuts,
    applyViewPresetByShortcut,
    onRefresh,
    clearSelection,
  } = options;

  return useMemo(
    () => [
      // Navigation
      {
        id: 'shortcuts',
        title: 'Show Keyboard Shortcuts',
        description: 'View all available shortcuts',
        shortcut: ['/'],
        icon: <Keyboard className={iconSizes.md} />,
        category: 'Navigation',
        action: openShortcuts,
      },
      {
        id: 'refresh',
        title: 'Refresh Data',
        description: 'Reload data from Neo4j',
        icon: <RefreshCw className={iconSizes.md} />,
        category: 'Navigation',
        action: onRefresh,
      },
      {
        id: 'clear-selection',
        title: 'Clear Selection',
        description: 'Deselect current node/edge',
        shortcut: ['Esc'],
        icon: <X className={iconSizes.md} />,
        category: 'Navigation',
        action: clearSelection,
      },

      // View
      {
        id: 'toggle-view',
        title: 'Toggle 2D/3D View',
        description: 'Switch between visualization modes',
        shortcut: ['V'],
        icon: <Box className={iconSizes.md} />,
        category: 'View',
        action: toggleViewMode,
      },
      {
        id: 'focus-mode',
        title: 'Toggle Focus Mode',
        description: 'Hide UI for distraction-free viewing',
        shortcut: ['G'],
        icon: <Maximize2 className={iconSizes.md} />,
        category: 'View',
        action: toggleFocusMode,
      },
      {
        id: 'minimap',
        title: 'Toggle Minimap',
        description: 'Show/hide the navigation minimap',
        shortcut: ['M'],
        icon: <Map className={iconSizes.md} />,
        category: 'View',
        action: toggleMinimap,
      },
      {
        id: 'edge-labels',
        title: 'Toggle Edge Labels',
        description: 'Show/hide relationship labels',
        shortcut: ['L'],
        icon: <Tag className={iconSizes.md} />,
        category: 'View',
        action: toggleEdgeLabels,
      },
      {
        id: 'sidebar',
        title: 'Toggle Sidebar',
        description: 'Show/hide the left sidebar',
        shortcut: ['['],
        icon: <PanelLeft className={iconSizes.md} />,
        category: 'View',
        action: toggleSidebar,
      },

      // Quick Views (Presets)
      {
        id: 'preset-1',
        title: 'Project Structure',
        description: 'Show project organization',
        shortcut: ['1'],
        icon: <LayoutGrid className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('1'),
      },
      {
        id: 'preset-2',
        title: 'Translation Chain',
        description: 'Show translation workflow',
        shortcut: ['2'],
        icon: <Layers className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('2'),
      },
      {
        id: 'preset-3',
        title: 'Locale Knowledge',
        description: 'Show locale-related nodes',
        shortcut: ['3'],
        icon: <Globe className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('3'),
      },
      {
        id: 'preset-4',
        title: 'Concept Network',
        description: 'Show concept relationships',
        shortcut: ['4'],
        icon: <Hash className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('4'),
      },
      {
        id: 'preset-5',
        title: 'Prompts & Rules',
        description: 'Show generation prompts',
        shortcut: ['5'],
        icon: <Command className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('5'),
      },
      {
        id: 'preset-6',
        title: 'SEO & GEO',
        description: 'Show SEO/GEO nodes',
        shortcut: ['6'],
        icon: <Eye className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('6'),
      },
      {
        id: 'preset-0',
        title: 'Show All Nodes',
        description: 'Clear filters, show everything',
        shortcut: ['0'],
        icon: <PanelRight className={iconSizes.md} />,
        category: 'Quick Views',
        action: () => applyViewPresetByShortcut('0'),
      },
    ],
    [
      toggleViewMode,
      toggleFocusMode,
      toggleMinimap,
      toggleEdgeLabels,
      toggleSidebar,
      openShortcuts,
      applyViewPresetByShortcut,
      onRefresh,
      clearSelection,
    ]
  );
}

// =============================================================================
// State Hook
// =============================================================================

export function useCommandPaletteState() {
  const [isOpen, setIsOpen] = useState(false);

  return {
    isOpen,
    open: () => setIsOpen(true),
    close: () => setIsOpen(false),
    toggle: () => setIsOpen((prev) => !prev),
  };
}
