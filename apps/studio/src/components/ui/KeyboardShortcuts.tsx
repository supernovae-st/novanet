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
import { isInputFocused } from '@/lib/keyboard';
import { ACTION_ICONS, NAV_ICONS, CONTENT_ICONS, ICON_SIZES } from '@/config/iconSystem';

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
      { keys: ['?'], description: 'Show keyboard shortcuts' },
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
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/70 backdrop-blur-sm animate-in fade-in duration-200"
        onClick={onClose}
      />

      {/* Modal */}
      <div className="relative w-full max-w-xl max-h-[80vh] m-4 glass-floating animate-scale-in overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-5 border-b border-white/12">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-primary to-primary/70 flex items-center justify-center shadow-lg shadow-primary/30">
              <KeyboardIcon className={ICON_SIZES.lg} />
            </div>
            <div>
              <h2 className="text-lg font-bold text-white">Keyboard Shortcuts</h2>
              <p className="text-xs text-white/55">Quick actions and navigation</p>
            </div>
          </div>
          <button
            onClick={onClose}
            aria-label="Close keyboard shortcuts"
            className="p-2 hover:bg-white/12 rounded-xl transition-colors text-white/60 hover:text-white/90 border border-transparent hover:border-white/15"
          >
            <CloseIcon className={ICON_SIZES.lg} />
          </button>
        </div>

        {/* Category Tabs */}
        <div className="flex gap-1 p-3 border-b border-white/12 bg-[hsl(240,8%,4%)]">
          {SHORTCUT_CATEGORIES.map((category) => (
            <button
              key={category.id}
              onClick={() => setActiveCategory(category.id)}
              className={cn(
                'flex items-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-200',
                activeCategory === category.id
                  ? 'bg-primary/15 text-white border border-primary/30'
                  : 'text-white/60 hover:text-white/80 hover:bg-white/8 border border-transparent hover:border-white/12'
              )}
            >
              <span
                className={cn(
                  'transition-colors',
                  activeCategory === category.id ? 'text-primary' : 'text-white/50'
                )}
              >
                {category.icon}
              </span>
              {category.title}
            </button>
          ))}
        </div>

        {/* Shortcuts List */}
        <div className="p-4 overflow-y-auto max-h-[calc(80vh-200px)] scrollbar-thin">
          {currentCategory && (
            <div className="space-y-1">
              {currentCategory.shortcuts.map((shortcut, idx) => (
                <div
                  key={shortcut.description}
                  className={cn(
                    'flex items-center justify-between py-3 px-4 rounded-xl',
                    'hover:bg-white/[0.04] transition-colors group'
                  )}
                  style={{
                    animationDelay: `${idx * 30}ms`,
                  }}
                >
                  <span className="text-sm text-white/70 group-hover:text-white/90 transition-colors">
                    {shortcut.description}
                  </span>
                  <div className="flex items-center gap-1">
                    {shortcut.keys.map((key, keyIdx) => (
                      <span key={keyIdx} className="flex items-center">
                        <kbd
                          className={cn(
                            'inline-flex items-center justify-center min-w-[28px] h-7 px-2',
                            'bg-white/10 border border-white/18 rounded-lg',
                            'text-xs font-mono text-white/85',
                            'group-hover:bg-white/15 group-hover:border-white/25 transition-colors',
                            'shadow-sm shadow-black/30'
                          )}
                        >
                          {key}
                        </kbd>
                        {keyIdx < shortcut.keys.length - 1 && (
                          <span className="text-white/35 mx-1 text-xs">+</span>
                        )}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-white/12 bg-[hsl(240,8%,4%)]">
          <div className="flex items-center justify-center gap-2 text-xs text-white/50">
            <span>Press</span>
            <kbd className="px-2 py-1 bg-white/10 rounded-lg text-[10px] font-mono text-white/70 border border-white/15 shadow-sm shadow-black/30">
              ?
            </kbd>
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

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (isInputFocused()) return;

      if (e.key === '?' || (e.shiftKey && e.key === '/')) {
        e.preventDefault();
        setIsOpen(true);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  return {
    isOpen,
    open: () => setIsOpen(true),
    close: () => setIsOpen(false),
    toggle: () => setIsOpen((prev) => !prev),
  };
}
