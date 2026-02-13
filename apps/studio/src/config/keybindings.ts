/**
 * Centralized Keybindings Configuration
 *
 * Three categories:
 * 1. Studio - NovaNet Studio web app shortcuts
 * 2. Pad - Work Louder macropad physical keybindings
 * 3. TUI - NovaNet TUI terminal keybindings (what pad sends to TUI)
 */

// =============================================================================
// Types
// =============================================================================

export interface KeyBinding {
  id: string;
  key: string;
  label: string;
  description?: string;
  category: string;
  icon?: string;
}

export interface PadKeyBinding {
  position: string; // "row,col" format
  key: string;
  label: string;
  action: string;
}

export interface PadLayer {
  id: number;
  name: string;
  color: string;
  description: string;
  keys: PadKeyBinding[];
  encoder?: {
    cw: { key: string; label: string; action: string };
    ccw: { key: string; label: string; action: string };
  };
}

// =============================================================================
// TUI Keybindings (what the macropad sends to NovaNet TUI)
// =============================================================================

export const TUI_KEYBINDINGS: KeyBinding[] = [
  // Navigation
  { id: 'tui-nav-up', key: 'k', label: 'Navigate Up', description: 'Move cursor up in tree', category: 'navigation', icon: 'ArrowUp' },
  { id: 'tui-nav-down', key: 'j', label: 'Navigate Down', description: 'Move cursor down in tree', category: 'navigation', icon: 'ArrowDown' },
  { id: 'tui-nav-left', key: 'h', label: 'Collapse / Back', description: 'Collapse node or go back', category: 'navigation', icon: 'ArrowLeft' },
  { id: 'tui-nav-right', key: 'l', label: 'Expand / Enter', description: 'Expand node or enter', category: 'navigation', icon: 'ArrowRight' },
  { id: 'tui-toggle', key: 'space', label: 'Toggle', description: 'Toggle expand/collapse', category: 'navigation', icon: 'ToggleLeft' },

  // Modes (v11.7: unified to 2 modes - Graph/Nexus; legacy view categories for macropad)
  { id: 'tui-mode-graph', key: '1', label: 'Graph Mode', description: 'Unified tree with Classes and Instances', category: 'mode', icon: 'Database' },
  { id: 'tui-mode-nexus', key: '2', label: 'Nexus Mode', description: 'Hub for Quiz, Audit, Stats, Help', category: 'mode', icon: 'Compass' },
  { id: 'tui-mode-filter', key: '3', label: 'Filter Mode', description: 'Apply faceted filters to graph', category: 'mode', icon: 'Filter' },
  { id: 'tui-mode-search', key: '4', label: 'Search Mode', description: 'Search across nodes', category: 'mode', icon: 'Search' },

  // Scrolling
  { id: 'tui-page-up', key: 'u', label: 'Page Up', description: 'Scroll page up (Ctrl+U)', category: 'scroll', icon: 'ChevronsUp' },
  { id: 'tui-page-down', key: 'd', label: 'Page Down', description: 'Scroll page down (Ctrl+D)', category: 'scroll', icon: 'ChevronsDown' },
  { id: 'tui-goto-first', key: 'g', label: 'Go to First', description: 'Jump to first item (gg)', category: 'scroll', icon: 'ArrowUpToLine' },
  { id: 'tui-goto-last', key: 'G', label: 'Go to Last', description: 'Jump to last item (G)', category: 'scroll', icon: 'ArrowDownToLine' },

  // YAML Panel
  { id: 'tui-yaml-up', key: '[', label: 'YAML Line Up', description: 'Scroll YAML up one line', category: 'yaml', icon: 'ChevronUp' },
  { id: 'tui-yaml-down', key: ']', label: 'YAML Line Down', description: 'Scroll YAML down one line', category: 'yaml', icon: 'ChevronDown' },

  // Search & Help
  { id: 'tui-search', key: '/', label: 'Search', description: 'Open search dialog', category: 'action', icon: 'Search' },
  { id: 'tui-help', key: '?', label: 'Help', description: 'Show help overlay', category: 'action', icon: 'HelpCircle' },
  { id: 'tui-refresh', key: 'r', label: 'Refresh', description: 'Refresh data from Neo4j', category: 'action', icon: 'RefreshCw' },
  { id: 'tui-quit', key: 'q', label: 'Quit', description: 'Exit the TUI', category: 'action', icon: 'X' },
];

// =============================================================================
// Pad Layers (Work Louder Creator Micro configuration)
// =============================================================================

export const PAD_LAYERS: PadLayer[] = [
  {
    id: 0,
    name: 'Navigation',
    color: '#00FFFF',
    description: 'NovaNet TUI navigation - move through the graph tree',
    keys: [
      { position: '0,0', key: '1', label: 'Graph', action: 'MODE_GRAPH' },
      { position: '0,1', key: 'K', label: '↑', action: 'NAV_UP' },
      { position: '0,2', key: 'UP', label: '↑', action: 'ARROW_UP' },
      { position: '0,3', key: '2', label: 'Nexus', action: 'MODE_NEXUS' },
      { position: '1,0', key: 'H', label: '←', action: 'NAV_LEFT' },
      { position: '1,1', key: 'LEFT', label: '←', action: 'ARROW_LEFT' },
      { position: '1,2', key: 'SPACE', label: 'Toggle', action: 'TOGGLE' },
      { position: '1,3', key: 'L', label: '→', action: 'NAV_RIGHT' },
      { position: '2,0', key: '3', label: 'Filter', action: 'MODE_FILTER' },
      { position: '2,1', key: 'J', label: '↓', action: 'NAV_DOWN' },
      { position: '2,2', key: 'DOWN', label: '↓', action: 'ARROW_DOWN' },
      { position: '2,3', key: '4', label: 'Search', action: 'MODE_SEARCH' },
    ],
    encoder: {
      cw: { key: 'K', label: 'Scroll Up', action: 'SCROLL_UP' },
      ccw: { key: 'J', label: 'Scroll Down', action: 'SCROLL_DOWN' },
    },
  },
  {
    id: 1,
    name: 'YAML & Overlays',
    color: '#9945FF',
    description: 'YAML panel scrolling and overlay navigation',
    keys: [
      { position: '0,0', key: 'g', label: 'First', action: 'GOTO_FIRST' },
      { position: '0,1', key: 'u', label: 'PgUp', action: 'PAGE_UP' },
      { position: '0,2', key: 'G', label: 'Last', action: 'GOTO_LAST' },
      { position: '0,3', key: 'G', label: 'Last', action: 'GOTO_LAST' },
      { position: '1,0', key: '[', label: '←Line', action: 'LINE_LEFT' },
      { position: '1,1', key: 'd', label: '½PgDn', action: 'HALF_PAGE_DOWN' },
      { position: '1,2', key: 'u', label: '½PgUp', action: 'HALF_PAGE_UP' },
      { position: '1,3', key: ']', label: '→Line', action: 'LINE_RIGHT' },
      { position: '2,0', key: '[', label: '←Sect', action: 'SECTION_PREV' },
      { position: '2,1', key: '/', label: 'Search', action: 'SEARCH' },
      { position: '2,2', key: '?', label: 'Help', action: 'HELP' },
      { position: '2,3', key: ']', label: '→Sect', action: 'SECTION_NEXT' },
    ],
    encoder: {
      cw: { key: ']', label: 'Line Down', action: 'YAML_LINE_DOWN' },
      ccw: { key: '[', label: 'Line Up', action: 'YAML_LINE_UP' },
    },
  },
  {
    id: 2,
    name: 'System',
    color: '#FF4545',
    description: 'System controls, RGB, and bootloader',
    keys: [
      { position: '0,0', key: 'r', label: 'Refresh', action: 'REFRESH' },
      { position: '0,1', key: 'RGB_VAI', label: 'RGB+', action: 'RGB_BRIGHTNESS_UP' },
      { position: '0,2', key: 'q', label: 'Quit', action: 'QUIT' },
      { position: '0,3', key: '___', label: '', action: 'NONE' },
      { position: '1,0', key: 'RGB_TOG', label: 'Toggle', action: 'RGB_TOGGLE' },
      { position: '1,1', key: 'RGB_MOD', label: 'Mode', action: 'RGB_MODE_NEXT' },
      { position: '1,2', key: 'RGB_HUI', label: 'Hue', action: 'RGB_HUE_UP' },
      { position: '1,3', key: '___', label: '', action: 'NONE' },
      { position: '2,0', key: '___', label: '', action: 'NONE' },
      { position: '2,1', key: '___', label: '', action: 'NONE' },
      { position: '2,2', key: '___', label: '', action: 'NONE' },
      { position: '2,3', key: 'QK_BOOT', label: 'Boot', action: 'BOOTLOADER' },
    ],
    encoder: {
      cw: { key: 'RGB_VAI', label: 'RGB+', action: 'RGB_BRIGHTNESS_UP' },
      ccw: { key: 'RGB_VAD', label: 'RGB-', action: 'RGB_BRIGHTNESS_DOWN' },
    },
  },
];

// =============================================================================
// Category Labels
// =============================================================================

export const TUI_CATEGORIES: Record<string, { label: string; order: number }> = {
  navigation: { label: 'Navigation', order: 1 },
  mode: { label: 'View Modes', order: 2 },
  scroll: { label: 'Scrolling', order: 3 },
  yaml: { label: 'YAML Panel', order: 4 },
  action: { label: 'Actions', order: 5 },
};
