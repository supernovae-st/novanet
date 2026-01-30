// src/generators/index.ts
// Unified View System - Documentation generators
// v8.0.0

// Types
export * from './types.js';

// Schemas
export * from './schemas.js';

// Colors (unified palette) - selective exports to avoid LAYER_COLORS conflict with types.ts
export {
  BEHAVIOR_COLORS,
  BEHAVIOR_STYLE,
  BEHAVIOR_EMOJI,
  SCOPE_EMOJI,
  LAYER_STYLE,
  BEHAVIOR_TO_LAYER,
  LAYER_TO_BEHAVIOR,
  type LocaleBehavior,
  type LayerColor,
  type ColorDef,
} from './colors.js';

// Generators
export { ViewParser } from './ViewParser.js';
export { MarkdownGenerator } from './MarkdownGenerator.js';
export { CypherExporter } from './CypherExporter.js';
export { RelationsParser, type RelationEdge } from './RelationsParser.js';
export { MermaidGenerator, type MermaidGeneratorConfig } from './MermaidGenerator.js';
