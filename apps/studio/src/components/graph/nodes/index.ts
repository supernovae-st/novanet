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
// v0.13.1: Enhanced with hierarchy visualization, depth indicators, premium hover states
export { StructuralNode, type StructuralNodeType, type StructuralNodeData } from './StructuralNode';

// Shared layer nodes (shared realm: config, locale, geography, knowledge)
export { SharedLayerNode, type SharedLayerNodeType } from './SharedLayerNode';

// Locale node - "Passport Élégant" design for BCP-47 locales
export { LocaleNode, type LocaleNodeType } from './LocaleNode';

// Class node - "Holographic Blueprint" design for schema Class nodes
export { ClassNode, type ClassNodeType, type ClassNodeExtendedData } from './ClassNode';

// Realm node - "Orbital Gateway" design for Realm nodes (shared, org)
export { RealmNode, type RealmNodeType, type RealmNodeExtendedData } from './RealmNode';

// TurboNode for default/generic display
export { TurboNode, type TurboNodeData, type TurboNodeType } from '../TurboNode';

// Blueprint overlay for schema mode
export { BlueprintOverlay, type BlueprintOverlayProps } from './BlueprintOverlay';

// Attractor nodes for magnetic grouping layout
export { RealmAttractorNode, type RealmAttractorData, type RealmAttractorNodeType } from './RealmAttractorNode';
export { LayerAttractorNode, type LayerAttractorData, type LayerAttractorNodeType } from './LayerAttractorNode';

// Taxonomy cards (Level 1 unified card system)
export { TaxonomyCard, type TaxonomyCardProps } from './taxonomy';

// Schema cards (Level 2 unified card system)
export { ArcClassCard, type ArcClassCardProps, type ArcClassNodeData } from './schema';
