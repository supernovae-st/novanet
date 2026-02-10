/**
 * Visual Encoding Utilities
 * Maps node layers to colors and icons from visual-encoding.yaml
 */

import type { NodeLayer, VisualProps } from '../types';

/**
 * Layer icons from visual-encoding.yaml
 */
export const LAYER_ICONS: Record<NodeLayer, string> = {
  config: '⚙',
  locale: '⊕',
  geography: '⊙',
  knowledge: '◈',
  foundation: '▣',
  structure: '▤',
  semantic: '◆',
  instruction: '▧',
  output: '●',
};

/**
 * Layer colors from visual-encoding.yaml
 * These are the default colors - actual colors may vary based on node type
 */
export const LAYER_COLORS: Record<NodeLayer, string> = {
  config: '#ef4444',     // red-500
  locale: '#a855f7',     // purple-500
  geography: '#22c55e',  // green-500
  knowledge: '#06b6d4',  // cyan-500
  foundation: '#f97316', // orange-500
  structure: '#f5f5f5',  // neutral-100
  semantic: '#3b82f6',   // blue-500
  instruction: '#eab308', // yellow-500
  output: '#ec4899',     // pink-500
};

/**
 * Get visual encoding for a layer
 */
export function getVisualEncoding(layer: NodeLayer): VisualProps {
  return {
    color: LAYER_COLORS[layer] ?? '#ffffff',
    icon: LAYER_ICONS[layer] ?? '●',
  };
}

/**
 * Default key bindings for the 9-key pad
 * Each key is assigned to a layer in order
 */
export const DEFAULT_BINDINGS: Array<{ layer: NodeLayer; action: string }> = [
  { layer: 'config', action: 'toggle-settings' },
  { layer: 'locale', action: 'switch-locale' },
  { layer: 'geography', action: 'navigate-geo' },
  { layer: 'knowledge', action: 'open-knowledge' },
  { layer: 'foundation', action: 'view-foundation' },
  { layer: 'structure', action: 'show-structure' },
  { layer: 'semantic', action: 'semantic-search' },
  { layer: 'instruction', action: 'add-instruction' },
  { layer: 'output', action: 'generate-output' },
];
