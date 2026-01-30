/**
 * NovaNet Configuration Module (v7.8.0)
 */

export {
  loadSpreadingActivationConfig,
  getTaskModifier,
  getSemanticLinkDefault,
  calculateBoostedTemperature,
  clearConfigCache,
  getPriorityScore,
  matchesPriorityFilter,
  TaskTypes,
  PRIORITY_SCORES,
  type TaskType,
  type Priority,
  type SemanticBoosts,
  type TaskModifier,
  type SpreadingActivationConfig,
} from './spreading-activation.js';

export { NODE_ICONS, getNodeIcon } from './nodeIcons.js';
