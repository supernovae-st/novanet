/**
 * Spreading Activation Configuration Loader (v7.8.0)
 *
 * Loads and caches the spreading activation configuration from YAML.
 * Used by GraphTraversalService for task-aware semantic retrieval.
 *
 * TODO(v9): DEAD CODE — GraphTraversalService was deleted with services/.
 * All exports from this file are unused. Keep types (Priority, TaskType,
 * SpreadingActivationConfig) if needed by Rust binary, otherwise delete
 * entire file + config/index.ts re-exports during v9 migration.
 */

import { readFileSync } from 'fs';
import { parse } from 'yaml';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

// ESM __dirname equivalent
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// =============================================================================
// Types
// =============================================================================

export interface SemanticBoosts {
  [semanticType: string]: number;
}

export interface TaskModifier {
  activation_threshold: number;
  propagation_steps: number;
  semantic_boosts: SemanticBoosts;
  priority_filter: string[];
}

export interface SpreadingActivationConfig {
  // Core parameters
  decay_factor: number;
  retention_factor: number;
  propagation_steps: number;
  initial_activation: number;

  // Thresholds
  activation_threshold: number;
  output_threshold: number;

  // Fan effect control
  max_fan_out: number;
  fan_penalty: number;

  // Task modifiers
  task_modifiers: Record<string, TaskModifier>;

  // Semantic link defaults
  semantic_link_defaults: Record<string, number>;
}

// =============================================================================
// Raw YAML structure
// =============================================================================

interface RawConfig {
  default: {
    decay_factor: number;
    retention_factor: number;
    propagation_steps: number;
    initial_activation: number;
    activation_threshold: number;
    output_threshold: number;
    max_fan_out: number;
    fan_penalty: number;
  };
  task_modifiers: Record<string, TaskModifier>;
  semantic_link_defaults: Record<string, number>;
}

// =============================================================================
// Configuration Loading
// =============================================================================

let cachedConfig: SpreadingActivationConfig | null = null;

/**
 * Load the spreading activation configuration from YAML.
 * Configuration is cached after first load.
 */
export function loadSpreadingActivationConfig(): SpreadingActivationConfig {
  if (cachedConfig) {
    return cachedConfig;
  }

  const configPath = join(__dirname, '../../models/config/spreading-activation.yaml');
  const raw = readFileSync(configPath, 'utf-8');
  const parsed = parse(raw) as RawConfig;

  cachedConfig = {
    ...parsed.default,
    task_modifiers: parsed.task_modifiers,
    semantic_link_defaults: parsed.semantic_link_defaults,
  };

  return cachedConfig;
}

/**
 * Get the task modifier for a specific task type.
 * Falls back to DEFAULT if task type is not found.
 */
export function getTaskModifier(taskType: string): TaskModifier {
  const config = loadSpreadingActivationConfig();
  const modifier = config.task_modifiers[taskType];

  if (!modifier) {
    console.warn(`Unknown task type: ${taskType}, falling back to DEFAULT`);
    return config.task_modifiers.DEFAULT;
  }

  return modifier;
}

/**
 * Get the default temperature for a semantic link type.
 */
export function getSemanticLinkDefault(linkType: string): number {
  const config = loadSpreadingActivationConfig();
  return config.semantic_link_defaults[linkType] ?? 0.5;
}

/**
 * Calculate the boosted temperature for a semantic link.
 * Applies task-specific boosts to the base temperature.
 */
export function calculateBoostedTemperature(
  baseTemperature: number,
  semanticType: string,
  taskType: string
): number {
  const modifier = getTaskModifier(taskType);
  const boost = modifier.semantic_boosts[semanticType] ?? 1.0;
  return Math.min(1.0, baseTemperature * boost);
}

/**
 * Clear the cached configuration.
 * Useful for testing or hot-reloading.
 */
export function clearConfigCache(): void {
  cachedConfig = null;
}

// =============================================================================
// Task Type Constants
// =============================================================================

export const TaskTypes = {
  CTA: 'CTA',
  FAQ: 'FAQ',
  HERO: 'HERO',
  PRICING: 'PRICING',
  TESTIMONIAL: 'TESTIMONIAL',
  DEFAULT: 'DEFAULT',
} as const;

export type TaskType = (typeof TaskTypes)[keyof typeof TaskTypes];

// =============================================================================
// Priority Constants and Mapping
// =============================================================================

/**
 * Priority levels as strings (for human readability)
 */
export type Priority = 'critical' | 'high' | 'medium' | 'low';

/**
 * Priority string to numeric score mapping.
 * Higher score = higher priority for spreading activation.
 * Used for consistent numeric comparison in algorithms.
 */
export const PRIORITY_SCORES: Record<Priority, number> = {
  critical: 1.0,
  high: 0.75,
  medium: 0.5,
  low: 0.25,
} as const;

/**
 * Convert priority string to numeric score.
 * Defaults to medium (0.5) if unknown priority.
 */
export function getPriorityScore(priority: string | undefined): number {
  if (!priority) return 0.5;
  return PRIORITY_SCORES[priority as Priority] ?? 0.5;
}

/**
 * Check if a priority string passes a priority filter.
 * Compares numeric scores rather than string matching.
 *
 * @param priority - The priority to check
 * @param filter - Array of acceptable priorities
 * @returns true if priority is in filter or filter is empty
 */
export function matchesPriorityFilter(
  priority: string | undefined,
  filter: string[] | undefined
): boolean {
  if (!filter || filter.length === 0) return true;
  if (!priority) return false;

  const score = getPriorityScore(priority);
  const minFilterScore = Math.min(...filter.map(getPriorityScore));

  return score >= minFilterScore;
}
