/**
 * NovaNet Configuration Module (v8.2.0)
 */

export {
  loadSpreadingActivationConfig,
  getTaskModifier,
  getSemanticLinkDefault,
  calculateBoostedTemperature,
  getPriorityScore,
  matchesPriorityFilter,
  PRIORITY_SCORES,
  type TaskType,
  type Priority,
  type SemanticBoosts,
  type TaskModifier,
  type SpreadingActivationConfig,
} from './spreading-activation.js';

export { NODE_ICONS, getNodeIcon } from './nodeIcons.js';
