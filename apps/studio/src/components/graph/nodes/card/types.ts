/**
 * Card Type System - 3-Level Architecture
 *
 * Unified type definitions for the NovaNet card system:
 * - Level 1: Taxonomy (21 nodes) - Realm, Layer, Trait, ArcFamily
 * - Level 2: Schema (239 nodes) - 61 NodeClass + 178 ArcClass
 * - Level 3: Data (∞ instances) - Runtime instances per layer
 *
 * Visual Encoding (ADR-005):
 * - Fill color → Layer
 * - Border color → Realm
 * - Border style → Trait
 * - Arc stroke → ArcFamily
 */

import type { PerformanceTier, PerformanceConfig } from '@/contexts/PerformanceContext';
import type {
  RealmKey,
  LayerKey,
  TraitKey,
  ArcFamilyKey,
} from '@/design/colors/generated';

// Re-export for convenience
export type { RealmKey, LayerKey, TraitKey, ArcFamilyKey };
export type { PerformanceTier, PerformanceConfig };

// =============================================================================
// Visual Encoding - Trait Border Styles (ADR-005)
// =============================================================================

export interface TraitBorderStyle {
  style: 'solid' | 'dashed' | 'double' | 'dotted';
  width: number;
}

export const TRAIT_BORDERS: Record<TraitKey, TraitBorderStyle> = {
  defined: { style: 'solid', width: 2 },
  authored: { style: 'dashed', width: 2 },
  imported: { style: 'double', width: 3 },
  generated: { style: 'dotted', width: 2 },
  retrieved: { style: 'dotted', width: 1 },
};

// =============================================================================
// Base Node Data
// =============================================================================

export interface BaseNodeData {
  id: string;
  type: string;
  key: string;
  displayName: string;
  dimmed?: boolean;
  hoverDimmed?: boolean;
}

// =============================================================================
// Card Context (passed to CardContent via render props)
// =============================================================================

export interface CardColors {
  primary: string;    // Layer color (fill, accents)
  secondary: string;  // Realm color (border)
  accent?: string;    // Optional tertiary color
}

export interface CardContext {
  colors: CardColors;
  selected: boolean;
  isHovered: boolean;
  width?: number;
  /** Current performance tier (ULTRA/HIGH/MEDIUM/LOW/MINIMAL) - defaults to HIGH */
  performanceTier?: PerformanceTier;
  /** Performance configuration with enabled effects - defaults to HIGH config */
  performanceConfig?: PerformanceConfig;
}

// =============================================================================
// Node Level Types (3-Level Architecture)
// =============================================================================

export type NodeLevel = 'taxonomy' | 'schema' | 'data';

// -----------------------------------------------------------------------------
// Level 1: Taxonomy (21 nodes)
// -----------------------------------------------------------------------------

export type TaxonomyVariant = 'realm' | 'layer' | 'trait' | 'arcFamily';

export interface TaxonomyNodeData extends BaseNodeData {
  level: 'taxonomy';
  variant: TaxonomyVariant;
  /** Number of nodes/arcs in this category */
  count: number;
  /** Taxonomy-specific color */
  color: string;
  /** For layer: which realm does it belong to */
  parentRealm?: RealmKey;
}

// Specific taxonomy data types
export interface RealmTaxonomyData extends TaxonomyNodeData {
  variant: 'realm';
  realmKey: RealmKey;
  layerCount: number;
  nodeClassCount: number;
}

export interface LayerTaxonomyData extends TaxonomyNodeData {
  variant: 'layer';
  layerKey: LayerKey;
  parentRealm: RealmKey;
  nodeClassCount: number;
}

export interface TraitTaxonomyData extends TaxonomyNodeData {
  variant: 'trait';
  traitKey: TraitKey;
  borderStyle: TraitBorderStyle;
  nodeClassCount: number;
}

export interface ArcFamilyTaxonomyData extends TaxonomyNodeData {
  variant: 'arcFamily';
  familyKey: ArcFamilyKey;
  arcClassCount: number;
}

// -----------------------------------------------------------------------------
// Level 2: Schema (239 nodes = 61 NodeClass + 178 ArcClass)
// -----------------------------------------------------------------------------

export type SchemaVariant = 'nodeClass' | 'arcClass';

export interface SchemaNodeData extends BaseNodeData {
  level: 'schema';
  variant: SchemaVariant;
  realm: RealmKey;
  layer: LayerKey;
  trait: TraitKey;
  propCount?: number;
  description?: string;
}

// NodeClass specific data
export interface NodeClassData extends SchemaNodeData {
  variant: 'nodeClass';
  instanceCount?: number;
}

// ArcClass specific data
export interface ArcClassData extends SchemaNodeData {
  variant: 'arcClass';
  source: string;
  target: string;
  family: ArcFamilyKey;
  cardinality: '1:1' | '1:N' | 'N:M';
  scope: 'intra_realm' | 'cross_realm';
}

// -----------------------------------------------------------------------------
// Level 3: Data (∞ instances)
// -----------------------------------------------------------------------------

export interface DataNodeData extends BaseNodeData {
  level: 'data';
  realm: RealmKey;
  layer: LayerKey;
  trait: TraitKey;
  /** Class name (e.g., 'Page', 'Entity', 'Block') */
  className: string;
  /** Layer-specific data passed via layerData */
  layerData?: Record<string, unknown>;
}

// Layer-specific data instance types
export interface FoundationInstanceData extends DataNodeData {
  layer: 'foundation';
  layerData: {
    projectName?: string;
    brandName?: string;
  };
}

export interface StructureInstanceData extends DataNodeData {
  layer: 'structure';
  layerData: {
    slug?: string;
    blockCount?: number;
    parentPage?: string;
  };
}

export interface SemanticInstanceData extends DataNodeData {
  layer: 'semantic';
  layerData: {
    entityCategory?: string;
    localeCount?: number;
  };
}

export interface OutputInstanceData extends DataNodeData {
  layer: 'output';
  layerData: {
    locale?: string;
    generatedAt?: string;
    version?: number;
  };
}

// =============================================================================
// Union Types
// =============================================================================

export type TaxonomyNodeDataUnion =
  | RealmTaxonomyData
  | LayerTaxonomyData
  | TraitTaxonomyData
  | ArcFamilyTaxonomyData;

export type SchemaNodeDataUnion = NodeClassData | ArcClassData;

export type DataNodeDataUnion =
  | FoundationInstanceData
  | StructureInstanceData
  | SemanticInstanceData
  | OutputInstanceData
  | DataNodeData; // Generic fallback

export type CardNodeData =
  | TaxonomyNodeDataUnion
  | SchemaNodeDataUnion
  | DataNodeDataUnion;

// =============================================================================
// Card Shell Props
// =============================================================================

export interface CardShellProps {
  data: CardNodeData;
  children: (context: CardContext) => React.ReactNode;
  width?: number;
  className?: string;
}

// =============================================================================
// Animation Variants (Framer Motion)
// =============================================================================

export const CARD_ANIMATION_VARIANTS = {
  hidden: { opacity: 0, scale: 0.95, y: 10 },
  visible: {
    opacity: 1,
    scale: 1,
    y: 0,
    transition: {
      type: 'spring',
      stiffness: 500,
      damping: 30,
      mass: 1,
    },
  },
  selected: {
    scale: 1.02,
    transition: { duration: 0.2, ease: [0.175, 0.885, 0.32, 1.275] },
  },
  hover: {
    y: -2,
    transition: { duration: 0.2 },
  },
} as const;

// =============================================================================
// Helper Functions
// =============================================================================

export function isNodeLevel(data: CardNodeData, level: NodeLevel): boolean {
  return data.level === level;
}

export function isTaxonomyNode(data: CardNodeData): data is TaxonomyNodeDataUnion {
  return data.level === 'taxonomy';
}

export function isSchemaNode(data: CardNodeData): data is SchemaNodeDataUnion {
  return data.level === 'schema';
}

export function isDataNode(data: CardNodeData): data is DataNodeDataUnion {
  return data.level === 'data';
}

export function isTaxonomyVariant(
  data: CardNodeData,
  variant: TaxonomyVariant
): boolean {
  return isTaxonomyNode(data) && data.variant === variant;
}

export function isSchemaVariant(
  data: CardNodeData,
  variant: SchemaVariant
): boolean {
  return isSchemaNode(data) && data.variant === variant;
}
