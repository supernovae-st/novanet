/**
 * Relationship type color configuration
 *
 * Colors are categorized by relationship type:
 * - Locale/Identity: emerald/green tones
 * - Content structure: blue tones
 * - Concepts: amber/yellow tones
 * - Derivation: purple tones
 * - Output: orange tones
 */

export const RELATIONSHIP_COLORS: Record<string, string> = {
  // Locale & Identity (emerald/green)
  HAS_LOCALE: '#10b981',
  HAS_IDENTITY: '#22c55e',
  HAS_VOICE: '#4ade80',
  HAS_CULTURE: '#86efac',
  HAS_MARKET: '#6ee7b7',
  HAS_LEXICON: '#34d399',

  // Content Structure (blue)
  HAS_UNIT: '#3b82f6',
  HAS_PAGE: '#3b82f6',
  HAS_BLOCK: '#06b6d4',
  HAS_PROMPT: '#60a5fa',
  HAS_RULES: '#93c5fd',
  HAS_METRICS: '#93c5fd',

  // Concepts & Expressions (amber/yellow/pink)
  HAS_EXPRESSION: '#ec4899',
  HAS_CONCEPT: '#f59e0b',
  USES_CONCEPT: '#fbbf24',

  // Localization & Derivation (purple/violet)
  HAS_L10N: '#a78bfa',
  DERIVES_FROM: '#a855f7',

  // Output & Targets (orange/red)
  HAS_OUTPUT: '#f97316',
  TARGETS: '#ef4444',

  // Containment (teal)
  CONTAINS: '#14b8a6',

  // Default
  DEFAULT: '#6b7280',
};

/**
 * Get color for a relationship type
 */
export function getRelationshipColor(type: string): string {
  return RELATIONSHIP_COLORS[type] || RELATIONSHIP_COLORS.DEFAULT;
}
