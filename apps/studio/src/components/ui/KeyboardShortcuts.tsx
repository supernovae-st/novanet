'use client';

/**
 * KeyboardShortcuts - Nika-style shortcuts overlay
 *
 * Features:
 * - Category tabs for organization
 * - Beautiful glass panel with animations
 * - Keyboard key styling
 * - Portal-based rendering
 * - Escape to close
 */

import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { cn } from '@/lib/utils';
import { modalClasses } from '@/design/tokens';
import { ACTION_ICONS, NAV_ICONS, CONTENT_ICONS, ICON_SIZES } from '@/config/iconSystem';
import { Kbd } from './Kbd';

// Design system icons
const CloseIcon = ACTION_ICONS.close;
const KeyboardIcon = NAV_ICONS.keyboard;
const CommandIcon = NAV_ICONS.command;
const ShowIcon = ACTION_ICONS.show;
const HashIcon = CONTENT_ICONS.id;
const MouseIcon = NAV_ICONS.mouse;

interface Shortcut {
  keys: string[];
  description: string;
}

interface ShortcutCategory {
  id: string;
  title: string;
  icon: React.ReactNode;
  shortcuts: Shortcut[];
}

const SHORTCUT_CATEGORIES: ShortcutCategory[] = [
  {
    id: 'navigation',
    title: 'Navigation',
    icon: <CommandIcon className={ICON_SIZES.md} />,
    shortcuts: [
      { keys: ['⌘', 'K'], description: 'Open command palette' },
      { keys: ['⌘', 'J'], description: 'Open AI chat' },
      { keys: ['F'], description: 'Fit view to content' },
      { keys: ['='], description: 'Zoom in' },
      { keys: ['-'], description: 'Zoom out' },
      { keys: ['Esc'], description: 'Close dialog / Clear selection' },
      { keys: ['/'], description: 'Show keyboard shortcuts' },
    ],
  },
  {
    id: 'view',
    title: 'View',
    icon: <ShowIcon className={ICON_SIZES.md} />,
    shortcuts: [
      { keys: ['V'], description: 'Toggle 2D/3D view' },
      { keys: ['G'], description: 'Toggle focus mode' },
      { keys: ['M'], description: 'Toggle minimap' },
      { keys: ['L'], description: 'Toggle edge labels' },
      { keys: ['⇧', 'L'], description: 'Cycle locale filter' },
      { keys: ['⇧', 'E'], description: 'Cycle animation effects (Full/Reduced/Off)' },
      { keys: ['['], description: 'Toggle left sidebar' },
      { keys: [']'], description: 'Toggle right panel' },
    ],
  },
  {
    id: 'presets',
    title: 'Quick Views',
    icon: <HashIcon className={ICON_SIZES.md} />,
    shortcuts: [
      { keys: ['1'], description: 'Project Structure' },
      { keys: ['2'], description: 'Generation Chain' },
      { keys: ['3'], description: 'Locale Knowledge' },
      { keys: ['4'], description: 'Concept Network' },
      { keys: ['5'], description: 'Prompts & Rules' },
      { keys: ['6'], description: 'SEO & GEO' },
      { keys: ['7'], description: 'High Priority' },
      { keys: ['8'], description: 'Realtime Content' },
      { keys: ['0'], description: 'All Nodes' },
    ],
  },
  {
    id: 'layout',
    title: 'Layout',
    icon: <CommandIcon className={ICON_SIZES.md} />,
    shortcuts: [
      { keys: ['⇧', 'H'], description: 'Horizontal layout' },
      { keys: ['⇧', 'V'], description: 'Vertical layout' },
      { keys: ['⇧', 'D'], description: 'Dagre layout (hierarchical)' },
      { keys: ['⇧', 'R'], description: 'Radial layout' },
      { keys: ['⇧', 'F'], description: 'Force-directed layout' },
    ],
  },
  {
    id: 'graph',
    title: 'Graph',
    icon: <MouseIcon className={ICON_SIZES.md} />,
    shortcuts: [
      { keys: ['Click'], description: 'Select node' },
      { keys: ['Drag'], description: 'Move node' },
      { keys: ['Scroll'], description: 'Zoom in/out' },
      { keys: ['⌘', 'Scroll'], description: 'Pan canvas' },
      { keys: ['Double-click'], description: 'Expand node neighbors' },
      { keys: ['Tab'], description: 'Next connected node' },
      { keys: ['⇧', 'Tab'], description: 'Previous connected node' },
    ],
  },
];

interface KeyboardShortcutsProps {
  isOpen: boolean;
  onClose: () => void;
}

export function KeyboardShortcuts({ isOpen, onClose }: KeyboardShortcutsProps) {
  const [mounted, setMounted] = useState(false);
  const [activeCategory, setActiveCategory] = useState('navigation');

  useEffect(() => {
    setMounted(true);
  }, []);

  // Handle escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  if (!mounted || !isOpen) return null;

  const currentCategory = SHORTCUT_CATEGORIES.find((c) => c.id === activeCategory);

  return createPortal(
    <div className={modalClasses.container}>
      {/* Backdrop */}
      <div
        className={cn(modalClasses.backdrop, 'animate-in fade-in duration-200')}
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Modal - unified design token */}
      <div
        className={cn(
          'relative w-full max-w-2xl max-h-[80vh] m-4 overflow-hidden',
          modalClasses.content,
          'animate-in zoom-in-95 fade-in duration-200'
        )}
        role="dialog"
        aria-modal="true"
        aria-labelledby="keyboard-shortcuts-title"
      >
        {/* Header */}
        <div className={modalClasses.header}>
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-primary/20 flex items-center justify-center">
              <KeyboardIcon className="w-4 h-4 text-primary" />
            </div>
            <h2 id="keyboard-shortcuts-title" className="text-sm font-semibold text-white">
              Keyboard Shortcuts
            </h2>
          </div>
          <button
            onClick={onClose}
            aria-label="Close keyboard shortcuts"
            className={modalClasses.closeButton}
          >
            <CloseIcon className="w-4 h-4" />
          </button>
        </div>

        {/* Category Tabs - horizontal scroll, compact */}
        <div className="flex gap-1 px-3 py-2 border-b border-white/[0.08] overflow-x-auto scrollbar-thin">
          {SHORTCUT_CATEGORIES.map((category) => (
            <button
              key={category.id}
              onClick={() => setActiveCategory(category.id)}
              className={cn(
                'flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap',
                'transition-all duration-150',
                activeCategory === category.id
                  ? 'bg-primary/15 text-white border border-primary/30'
                  : 'text-white/50 hover:text-white/70 hover:bg-white/[0.04] border border-transparent'
              )}
            >
              <span
                className={cn(
                  'transition-colors',
                  activeCategory === category.id ? 'text-primary' : 'text-white/40'
                )}
              >
                {category.icon}
              </span>
              {category.title}
            </button>
          ))}
        </div>

        {/* Shortcuts List - clean rows */}
        <div className="p-2 overflow-y-auto max-h-[calc(80vh-160px)] scrollbar-thin">
          {currentCategory && (
            <div className="space-y-0.5">
              {currentCategory.shortcuts.map((shortcut) => (
                <div
                  key={shortcut.description}
                  className={cn(
                    'flex items-center justify-between py-2.5 px-3 rounded-lg',
                    'hover:bg-white/[0.04] transition-colors group'
                  )}
                >
                  <span className="text-sm text-white/60 group-hover:text-white/80 transition-colors">
                    {shortcut.description}
                  </span>
                  <div className="flex items-center gap-1">
                    {shortcut.keys.map((key, keyIdx) => (
                      <span key={keyIdx} className="flex items-center">
                        <kbd
                          className={cn(
                            'inline-flex items-center justify-center min-w-[26px] h-6 px-1.5',
                            'bg-white/[0.06] border border-white/[0.08] rounded-md',
                            'text-[11px] font-mono text-white/70',
                            'group-hover:bg-white/[0.10] group-hover:border-white/[0.15] transition-colors'
                          )}
                        >
                          {key}
                        </kbd>
                        {keyIdx < shortcut.keys.length - 1 && (
                          <span className="text-white/30 mx-0.5 text-[10px]">+</span>
                        )}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Footer - subtle hint */}
        <div className="px-4 py-2.5 border-t border-white/[0.08]">
          <div className="flex items-center justify-center gap-2 text-[11px] text-white/40">
            <span>Press</span>
            <Kbd>/</Kbd>
            <span>anytime to show this dialog</span>
          </div>
        </div>
      </div>
    </div>,
    document.body
  );
}

/**
 * Hook to toggle keyboard shortcuts visibility
 */
export function useKeyboardShortcuts() {
  const [isOpen, setIsOpen] = useState(false);


  return {
    isOpen,
    open: () => setIsOpen(true),
    close: () => setIsOpen(false),
    toggle: () => setIsOpen((prev) => !prev),
  };
}
