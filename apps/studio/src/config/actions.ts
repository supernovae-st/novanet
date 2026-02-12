/**
 * Action Presets for Macropad Key Binding
 *
 * Action-centric approach: users think "Navigate Down" not "KC_J"
 * Organized by category with visual encoding (colors, icons)
 */

// =============================================================================
// Types
// =============================================================================

export type ActionCategory = 'navigation' | 'modes' | 'scroll' | 'yaml' | 'actions' | 'system';

export interface ActionPreset {
  id: string;
  category: ActionCategory;
  label: string;        // Full label: "Navigate Down"
  shortLabel: string;   // Compact: "↓ Down"
  key: string;          // Display key: "J"
  keycode: string;      // QMK keycode: "KC_J"
  icon: string;         // Lucide icon name
}

// =============================================================================
// Category Colors (matching layer colors)
// =============================================================================

export const CATEGORY_COLORS: Record<ActionCategory, string> = {
  navigation: '#00FFFF',  // Cyan
  modes: '#9945FF',       // Purple
  scroll: '#22C55E',      // Green
  yaml: '#3B82F6',        // Blue
  actions: '#F97316',     // Orange
  system: '#FF4545',      // Red
};

export const CATEGORY_LABELS: Record<ActionCategory, string> = {
  navigation: 'Navigation',
  modes: 'View Modes',
  scroll: 'Scrolling',
  yaml: 'YAML Panel',
  actions: 'Actions',
  system: 'System',
};

// =============================================================================
// Action Presets
// =============================================================================

export const ACTION_PRESETS: Record<ActionCategory, ActionPreset[]> = {
  navigation: [
    {
      id: 'nav-up',
      category: 'navigation',
      label: 'Navigate Up',
      shortLabel: '↑ Up',
      key: 'K',
      keycode: 'KC_K',
      icon: 'ArrowUp',
    },
    {
      id: 'nav-down',
      category: 'navigation',
      label: 'Navigate Down',
      shortLabel: '↓ Down',
      key: 'J',
      keycode: 'KC_J',
      icon: 'ArrowDown',
    },
    {
      id: 'nav-left',
      category: 'navigation',
      label: 'Collapse / Back',
      shortLabel: '← Back',
      key: 'H',
      keycode: 'KC_H',
      icon: 'ArrowLeft',
    },
    {
      id: 'nav-right',
      category: 'navigation',
      label: 'Expand / Enter',
      shortLabel: '→ Enter',
      key: 'L',
      keycode: 'KC_L',
      icon: 'ArrowRight',
    },
    {
      id: 'nav-toggle',
      category: 'navigation',
      label: 'Toggle Expand',
      shortLabel: '␣ Toggle',
      key: 'SPACE',
      keycode: 'KC_SPC',
      icon: 'ToggleLeft',
    },
  ],

  modes: [
    {
      id: 'mode-meta',
      category: 'modes',
      label: 'Meta Mode',
      shortLabel: 'META',
      key: '1',
      keycode: 'KC_1',
      icon: 'Database',
    },
    {
      id: 'mode-data',
      category: 'modes',
      label: 'Data Mode',
      shortLabel: 'DATA',
      key: '2',
      keycode: 'KC_2',
      icon: 'FileText',
    },
    {
      id: 'mode-overlay',
      category: 'modes',
      label: 'Overlay Mode',
      shortLabel: 'OVERLAY',
      key: '3',
      keycode: 'KC_3',
      icon: 'Layers',
    },
    {
      id: 'mode-query',
      category: 'modes',
      label: 'Query Mode',
      shortLabel: 'QUERY',
      key: '4',
      keycode: 'KC_4',
      icon: 'Search',
    },
  ],

  scroll: [
    {
      id: 'scroll-page-up',
      category: 'scroll',
      label: 'Page Up',
      shortLabel: 'PgUp',
      key: 'U',
      keycode: 'KC_U',
      icon: 'ChevronsUp',
    },
    {
      id: 'scroll-page-down',
      category: 'scroll',
      label: 'Page Down',
      shortLabel: 'PgDn',
      key: 'D',
      keycode: 'KC_D',
      icon: 'ChevronsDown',
    },
    {
      id: 'scroll-top',
      category: 'scroll',
      label: 'Go to Top',
      shortLabel: 'Top',
      key: 'g',
      keycode: 'KC_G',
      icon: 'ArrowUpToLine',
    },
    {
      id: 'scroll-bottom',
      category: 'scroll',
      label: 'Go to Bottom',
      shortLabel: 'Bottom',
      key: 'G',
      keycode: 'S(KC_G)',
      icon: 'ArrowDownToLine',
    },
  ],

  yaml: [
    {
      id: 'yaml-line-up',
      category: 'yaml',
      label: 'YAML Line Up',
      shortLabel: '[ Up',
      key: '[',
      keycode: 'KC_LBRC',
      icon: 'ChevronUp',
    },
    {
      id: 'yaml-line-down',
      category: 'yaml',
      label: 'YAML Line Down',
      shortLabel: '] Down',
      key: ']',
      keycode: 'KC_RBRC',
      icon: 'ChevronDown',
    },
  ],

  actions: [
    {
      id: 'action-search',
      category: 'actions',
      label: 'Search',
      shortLabel: '/ Search',
      key: '/',
      keycode: 'KC_SLSH',
      icon: 'Search',
    },
    {
      id: 'action-help',
      category: 'actions',
      label: 'Help',
      shortLabel: '? Help',
      key: '?',
      keycode: 'S(KC_SLSH)',
      icon: 'HelpCircle',
    },
    {
      id: 'action-refresh',
      category: 'actions',
      label: 'Refresh',
      shortLabel: 'R Refresh',
      key: 'R',
      keycode: 'KC_R',
      icon: 'RefreshCw',
    },
    {
      id: 'action-quit',
      category: 'actions',
      label: 'Quit',
      shortLabel: 'Q Quit',
      key: 'Q',
      keycode: 'KC_Q',
      icon: 'X',
    },
  ],

  system: [
    {
      id: 'sys-rgb-toggle',
      category: 'system',
      label: 'RGB Toggle',
      shortLabel: 'RGB On/Off',
      key: 'RGB_TOG',
      keycode: 'RGB_TOG',
      icon: 'Lightbulb',
    },
    {
      id: 'sys-rgb-mode',
      category: 'system',
      label: 'RGB Mode',
      shortLabel: 'RGB Mode',
      key: 'RGB_MOD',
      keycode: 'RGB_MOD',
      icon: 'Palette',
    },
    {
      id: 'sys-rgb-bright-up',
      category: 'system',
      label: 'RGB Brighter',
      shortLabel: 'RGB +',
      key: 'RGB_VAI',
      keycode: 'RGB_VAI',
      icon: 'SunMedium',
    },
    {
      id: 'sys-rgb-bright-down',
      category: 'system',
      label: 'RGB Dimmer',
      shortLabel: 'RGB -',
      key: 'RGB_VAD',
      keycode: 'RGB_VAD',
      icon: 'SunDim',
    },
    {
      id: 'sys-bootloader',
      category: 'system',
      label: 'Bootloader',
      shortLabel: 'BOOT',
      key: 'QK_BOOT',
      keycode: 'QK_BOOT',
      icon: 'Cpu',
    },
  ],
};

// =============================================================================
// Helpers
// =============================================================================

/**
 * Get all presets as flat array
 */
export function getAllPresets(): ActionPreset[] {
  return Object.values(ACTION_PRESETS).flat();
}

/**
 * Find preset by ID
 */
export function getPresetById(id: string): ActionPreset | undefined {
  return getAllPresets().find((p) => p.id === id);
}

/**
 * Find preset by keycode
 */
export function getPresetByKeycode(keycode: string): ActionPreset | undefined {
  return getAllPresets().find((p) => p.keycode === keycode);
}

/**
 * Find preset by display key
 */
export function getPresetByKey(key: string): ActionPreset | undefined {
  return getAllPresets().find((p) => p.key.toLowerCase() === key.toLowerCase());
}

/**
 * Get category color
 */
export function getCategoryColor(category: ActionCategory): string {
  return CATEGORY_COLORS[category];
}

/**
 * Order of categories for display
 */
export const CATEGORY_ORDER: ActionCategory[] = [
  'navigation',
  'modes',
  'scroll',
  'yaml',
  'actions',
  'system',
];
