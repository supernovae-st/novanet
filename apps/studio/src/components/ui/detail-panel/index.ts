/**
 * Detail Panel Components
 *
 * Shared components for detail panels (NodeDetailsPanel, ArcDetailsPanel)
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
// Re-export from dx for backwards compatibility
export { CopyButton, type CopyButtonProps } from '@/components/dx/CopyButton';
export { JsonView, type JsonViewProps } from './JsonView';
