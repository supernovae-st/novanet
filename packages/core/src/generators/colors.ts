// src/generators/colors.ts
// Runtime color palette for NovaNet view diagram generation
// v8.2.0 - Minimal runtime subset (full palette in @novanet/schema-tools)
//
// NOTE: For build-time Mermaid generation with full locale_behavior colors,
// use @novanet/schema-tools/colors instead.

// =============================================================================
// TAILWIND COLOR VALUES
// =============================================================================

const TAILWIND = {
  blue500: '#3b82f6',
  blue700: '#1d4ed8',
  green500: '#22c55e',
  green600: '#16a34a',
  violet500: '#8b5cf6',
  violet600: '#7c3aed',
  orange500: '#f97316',
  orange600: '#ea580c',
  gray500: '#6b7280',
  gray600: '#4b5563',
  red500: '#ef4444',
  red600: '#dc2626',
  cyan500: '#06b6d4',
  cyan600: '#0891b2',
} as const;

// =============================================================================
// LAYER COLORS (for view diagrams - runtime)
// =============================================================================

export type LayerColor = 'blue' | 'green' | 'orange' | 'purple' | 'red' | 'gray' | 'cyan';

export interface ColorDef {
  fill: string;
  stroke: string;
}

/**
 * Colors for documentation layers in view diagrams.
 * Used by MarkdownGenerator.generateMermaid() at runtime.
 */
export const LAYER_COLORS: Record<LayerColor, ColorDef> = {
  blue: { fill: TAILWIND.blue500, stroke: TAILWIND.blue700 },
  green: { fill: TAILWIND.green500, stroke: TAILWIND.green600 },
  purple: { fill: TAILWIND.violet500, stroke: TAILWIND.violet600 },
  gray: { fill: TAILWIND.gray500, stroke: TAILWIND.gray600 },
  orange: { fill: TAILWIND.orange500, stroke: TAILWIND.orange600 },
  cyan: { fill: TAILWIND.cyan500, stroke: TAILWIND.cyan600 },
  red: { fill: TAILWIND.red500, stroke: TAILWIND.red600 },
};
