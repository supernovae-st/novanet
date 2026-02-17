/**
 * Layer Tokens - Visual encoding for the 10 NovaNet layers
 *
 * Source of truth: visual-encoding.yaml (ADR-005)
 * Layer = primary identification via fill color
 *
 * Usage:
 * ```tsx
 * import { LAYER_TOKENS, getLayerGradient } from '@/design/tokens/layerTokens';
 *
 * const gradient = getLayerGradient('knowledge');
 * // -> "bg-gradient-to-br from-violet-900/80 to-violet-800/60"
 * ```
 */

import type { NodeLayer } from '@/components/graph/nodes/card/taxonomyColors';

// =============================================================================
// Layer Color Tokens
// =============================================================================

export interface LayerToken {
  /** Layer identifier */
  key: NodeLayer;
  /** Display name */
  name: string;
  /** Tailwind color key (e.g., 'slate', 'violet') */
  colorKey: string;
  /** Hex color value */
  hex: string;
  /** Gradient class for card backgrounds */
  gradient: string;
  /** Badge background class */
  badgeBg: string;
  /** Badge text class */
  badgeText: string;
  /** Glow effect for icons */
  iconGlow: string;
  /** Description */
  description: string;
}

export const LAYER_TOKENS: Record<NodeLayer, LayerToken> = {
  // SHARED realm (4 layers)
  config: {
    key: 'config',
    name: 'Config',
    colorKey: 'slate',
    hex: '#64748b',
    gradient: 'bg-gradient-to-br from-slate-900/80 to-slate-800/60',
    badgeBg: 'bg-slate-500/20',
    badgeText: 'text-slate-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(100 116 139 / 0.6))',
    description: 'Configuration and definitions',
  },
  locale: {
    key: 'locale',
    name: 'Locale',
    colorKey: 'violet',
    hex: '#8b5cf6',
    gradient: 'bg-gradient-to-br from-violet-900/80 to-violet-800/60',
    badgeBg: 'bg-violet-500/20',
    badgeText: 'text-violet-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(139 92 246 / 0.6))',
    description: 'Locale settings and formatting',
  },
  geography: {
    key: 'geography',
    name: 'Geography',
    colorKey: 'emerald',
    hex: '#10b981',
    gradient: 'bg-gradient-to-br from-emerald-900/80 to-emerald-800/60',
    badgeBg: 'bg-emerald-500/20',
    badgeText: 'text-emerald-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(16 185 129 / 0.6))',
    description: 'Geographic classifications',
  },
  knowledge: {
    key: 'knowledge',
    name: 'Knowledge',
    colorKey: 'purple',
    hex: '#a855f7',
    gradient: 'bg-gradient-to-br from-purple-900/80 to-purple-800/60',
    badgeBg: 'bg-purple-500/20',
    badgeText: 'text-purple-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(168 85 247 / 0.6))',
    description: 'Locale expertise and knowledge atoms',
  },
  // ORG realm (6 layers)
  foundation: {
    key: 'foundation',
    name: 'Foundation',
    colorKey: 'blue',
    hex: '#3b82f6',
    gradient: 'bg-gradient-to-br from-blue-900/80 to-blue-800/60',
    badgeBg: 'bg-blue-500/20',
    badgeText: 'text-blue-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(59 130 246 / 0.6))',
    description: 'Project identity and branding',
  },
  structure: {
    key: 'structure',
    name: 'Structure',
    colorKey: 'cyan',
    hex: '#06b6d4',
    gradient: 'bg-gradient-to-br from-cyan-900/80 to-cyan-800/60',
    badgeBg: 'bg-cyan-500/20',
    badgeText: 'text-cyan-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(6 182 212 / 0.6))',
    description: 'Information architecture',
  },
  semantic: {
    key: 'semantic',
    name: 'Semantic',
    colorKey: 'orange',
    hex: '#f97316',
    gradient: 'bg-gradient-to-br from-orange-900/80 to-orange-800/60',
    badgeBg: 'bg-orange-500/20',
    badgeText: 'text-orange-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(249 115 22 / 0.6))',
    description: 'Entities and meaning',
  },
  instruction: {
    key: 'instruction',
    name: 'Instruction',
    colorKey: 'yellow',
    hex: '#eab308',
    gradient: 'bg-gradient-to-br from-yellow-900/80 to-yellow-800/60',
    badgeBg: 'bg-yellow-500/20',
    badgeText: 'text-yellow-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(234 179 8 / 0.6))',
    description: 'Generation directives',
  },
  output: {
    key: 'output',
    name: 'Output',
    colorKey: 'green',
    hex: '#22c55e',
    gradient: 'bg-gradient-to-br from-green-900/80 to-green-800/60',
    badgeBg: 'bg-green-500/20',
    badgeText: 'text-green-300',
    iconGlow: 'drop-shadow(0 0 6px rgb(34 197 94 / 0.6))',
    description: 'Generated content',
  },
};

// =============================================================================
// Helper Functions
// =============================================================================

/**
 * Get gradient class for a layer
 */
export function getLayerGradient(layer: NodeLayer): string {
  return LAYER_TOKENS[layer].gradient;
}

/**
 * Get badge classes for a layer
 */
export function getLayerBadgeClasses(layer: NodeLayer): string {
  const token = LAYER_TOKENS[layer];
  return `${token.badgeBg} ${token.badgeText}`;
}

/**
 * Get icon glow filter for a layer
 */
export function getLayerIconGlow(layer: NodeLayer): string {
  return LAYER_TOKENS[layer].iconGlow;
}

/**
 * Get all layer keys
 */
export function getAllLayers(): NodeLayer[] {
  return Object.keys(LAYER_TOKENS) as NodeLayer[];
}

/**
 * Get layers by realm
 */
export function getLayersByRealm(realm: 'shared' | 'org'): NodeLayer[] {
  if (realm === 'shared') {
    return ['config', 'locale', 'geography', 'knowledge'];
  }
  return ['foundation', 'structure', 'semantic', 'instruction', 'output'];
}
