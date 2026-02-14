/**
 * Custom Node Types Index (v7.2.5)
 *
 * Exports all differentiated node components for graph visualization
 */

export { BaseNodeWrapper, type BaseNodeData } from './BaseNodeWrapper';
export type { BaseNodeWrapperProps } from './BaseNodeWrapper';

// Project node - Premium social network style card
export { ProjectNode, type ProjectNodeType } from './ProjectNode';

// Structural nodes (project + content categories)
export { StructuralNode, type StructuralNodeType } from './StructuralNode';

// Shared layer nodes (shared realm: config, locale, geography, knowledge)
export { SharedLayerNode, type SharedLayerNodeType } from './SharedLayerNode';

// TurboNode for default/generic display
export { TurboNode, type TurboNodeData, type TurboNodeType } from '../TurboNode';

// Blueprint overlay for schema mode
export { BlueprintOverlay, type BlueprintOverlayProps } from './BlueprintOverlay';

// Attractor nodes for magnetic grouping layout
export { RealmAttractorNode, type RealmAttractorData, type RealmAttractorNodeType } from './RealmAttractorNode';
export { LayerAttractorNode, type LayerAttractorData, type LayerAttractorNodeType } from './LayerAttractorNode';
