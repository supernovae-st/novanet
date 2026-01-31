/**
 * Library Utilities Barrel Export
 *
 * Centralizes all utility exports for cleaner imports:
 * import { logger, fetchJSON, cn } from '@/lib';
 */

// =============================================================================
// Core Utilities
// =============================================================================

export { cn } from './utils';
export { logger } from './logger';

// =============================================================================
// Data Fetching
// =============================================================================

export {
  fetchJSON,
  postJSON,
  getErrorMessage,
  FetchError,
  type FetchOptions,
} from './fetchClient';

export { handleApiError } from './apiErrorHandler';

// =============================================================================
// Clipboard
// =============================================================================

export {
  clipboard,
  copyToClipboard,
  copyNodeProperties,
  type ClipboardProvider,
} from './clipboard';

// =============================================================================
// Graph Layout
// =============================================================================

export { applyDagreLayout, type LayoutOptions } from './layout';

export {
  createForceSimulation,
  runSimulationSync,
  applyForcePositions,
  type ForceOptions,
  type ForceNode,
} from './forceSimulation';

// =============================================================================
// Validation & Security
// =============================================================================

export {
  validateCypher,
  getCypherError,
  type ValidationResult,
} from './cypherValidator';

// =============================================================================
// Search
// =============================================================================

export {
  fuzzyMatch,
  fuzzySearch,
  type FuzzyMatch,
  type FuzzySearchOptions,
} from './fuzzySearch';

// =============================================================================
// Formatting
// =============================================================================

export {
  formatDuration,
  formatNumber,
  formatFileSize,
  formatRelativeTime,
  formatDateTime,
  formatCompact,
  formatPercent,
} from './formatters';

export {
  hexToRgb,
  hexToRgba,
  withOpacity,
  coloredBadgeStyle,
  OPACITY,
  type HexColor,
} from './colorUtils';

// =============================================================================
// Keyboard
// =============================================================================

export {
  parseKeyCombo,
  matchesKeyCombo,
  isInputFocused,
  type ParsedKeyCombo,
  type Shortcut,
} from './keyboard';

// =============================================================================
// Database (use sparingly - prefer API routes)
// =============================================================================

export { getDriver } from './neo4j';
export { getSchema, formatSchemaForPrompt } from './schemaCache';

// =============================================================================
// Filter Adapter (Cypher generation)
// =============================================================================

export {
  NovaNetFilter,
  NODE_CATEGORIES,
  type FilterCriteria,
  type NovaNetFilterCriteria,
  type CypherQuery,
  type Priority,
  type Freshness,
  type NodeCategory,
} from './filterAdapter';
