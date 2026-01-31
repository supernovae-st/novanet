// @novanet/schema-tools
// Build-time generators and validators for NovaNet schema
// v8.2.0

// =============================================================================
// PUBLIC API - GENERATORS
// =============================================================================

export { MermaidGenerator, type MermaidGeneratorConfig } from './generators/MermaidGenerator.js';
export { SubcategoryGenerator, type SubcategoryGeneratorConfig } from './generators/SubcategoryGenerator.js';
export { OrganizingPrinciplesGenerator, type OrganizingPrinciplesGeneratorConfig } from './generators/OrganizingPrinciplesGenerator.js';

// =============================================================================
// PUBLIC API - PARSERS
// =============================================================================

export { RelationsParser, type RelationEdge } from './parsers/RelationsParser.js';

// =============================================================================
// PUBLIC API - UTILITIES
// =============================================================================

export { filePathToNodeName } from './utils/filePathToNodeName.js';

// =============================================================================
// PUBLIC API - TYPES (re-exported from config/colors.ts)
// =============================================================================

// Types needed by consumers of MermaidGenerator
export type { LocaleBehavior, LayerColor, ColorDef, EdgeCategory } from './config/colors.js';

// =============================================================================
// INTERNAL - Colors and constants (use sparingly, implementation details)
// =============================================================================

export {
  BEHAVIOR_COLORS,
  BEHAVIOR_STYLE,
  BEHAVIOR_EMOJI,
  SCOPE_EMOJI,
  LAYER_COLORS,
  LAYER_STYLE,
  BEHAVIOR_TO_LAYER,
  LAYER_TO_BEHAVIOR,
  EDGE_ARROWS,
  EDGE_COLORS,
  EDGE_TO_CATEGORY,
} from './config/colors.js';
