/**
 * Keyboard utilities
 * Adapted from Nika Studio pattern
 */

import { getViewPresetByShortcut } from './filterAdapter';

export interface ParsedKeyCombo {
  mod: boolean;   // ⌘ on Mac, Ctrl on Windows
  shift: boolean;
  alt: boolean;
  key: string;
}

/**
 * Parse a key combo string like "mod+shift+k" into structured format
 */
export function parseKeyCombo(keys: string): ParsedKeyCombo {
  const parts = keys.toLowerCase().split('+');
  const key = parts[parts.length - 1];

  return {
    mod: parts.includes('mod'),
    shift: parts.includes('shift'),
    alt: parts.includes('alt'),
    key,
  };
}

/**
 * Check if a keyboard event matches a key combo
 */
export function matchesKeyCombo(event: KeyboardEvent, keys: string): boolean {
  const combo = parseKeyCombo(keys);
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const modPressed = isMac ? event.metaKey : event.ctrlKey;

  if (combo.mod !== modPressed) return false;
  if (combo.shift !== event.shiftKey) return false;
  if (combo.alt !== event.altKey) return false;

  const eventKey = event.key.toLowerCase();

  // Special key handling
  if (combo.key === 'space' && eventKey === ' ') return true;
  if (combo.key === 'escape' && eventKey === 'escape') return true;
  if (combo.key === 'enter' && eventKey === 'enter') return true;
  if (combo.key === 'backspace' && eventKey === 'backspace') return true;
  if (combo.key === 'delete' && eventKey === 'delete') return true;
  if (combo.key === 'tab' && eventKey === 'tab') return true;
  if (combo.key === 'arrowup' && eventKey === 'arrowup') return true;
  if (combo.key === 'arrowdown' && eventKey === 'arrowdown') return true;
  if (combo.key === 'arrowleft' && eventKey === 'arrowleft') return true;
  if (combo.key === 'arrowright' && eventKey === 'arrowright') return true;

  return eventKey === combo.key;
}

/**
 * Check if input is focused
 */
export function isInputFocused(): boolean {
  const activeElement = document.activeElement;
  if (!activeElement) return false;

  const tagName = activeElement.tagName.toLowerCase();
  if (tagName === 'input' || tagName === 'textarea') return true;
  if (activeElement.getAttribute('contenteditable') === 'true') return true;

  return false;
}

/**
 * Shortcut definition
 */
export interface Shortcut {
  id: string;
  keys: string;
  label: string;
  description?: string;
  category: 'navigation' | 'view' | 'filter' | 'preset' | 'layout' | 'action';
  icon?: string;
  when?: string[];
  action: string;
}

/**
 * Handle VIEW_PRESET keyboard shortcuts (1-8, 0)
 * Returns the preset ID if the key matches a preset shortcut, null otherwise
 *
 * @param key - The keyboard key pressed ('0', '1', '2', etc.)
 * @returns The preset ID or null if not found
 */
export function handleViewPresetShortcut(key: string): string | null {
  const preset = getViewPresetByShortcut(key);
  return preset?.id ?? null;
}
