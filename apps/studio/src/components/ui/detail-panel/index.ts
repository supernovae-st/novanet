/**
 * Detail Panel Components
 *
 * Shared components for detail panels (NodeDetailsPanel, EdgeDetailsPanel)
 * Reduces code duplication and ensures consistent styling.
 */

export { CollapsibleSection, type CollapsibleSectionProps } from './CollapsibleSection';
export { PropertyRow, formatValue, formatValueString, type PropertyRowProps } from './PropertyRow';
export {
  NodeNavigationCard,
  RelationNavigationCard,
  type NodeNavigationCardProps,
  type RelationNavigationCardProps,
} from './NodeNavigationCard';
export { CopyButton, type CopyButtonProps } from './CopyButton';
export { JsonView, JsonToggleSection, type JsonViewProps, type JsonToggleSectionProps } from './JsonView';
