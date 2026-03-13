/**
 * Card Content Selector - Routes node types to specialized card components
 *
 * This module provides a unified way to select the correct card content
 * component based on node type. Each node type can have a specialized
 * card design that displays relevant information for that type.
 *
 * Visual Encoding (ADR-005):
 * - Fill gradient → Layer
 * - Border color → Realm
 * - Border style → Trait
 * - Icon → Class
 *
 * Coverage: 60 node types across 10 layers (SHARED + ORG realms)
 * v0.18.0: Term/TermSet removed (YAGNI), LocaleMarket removed (external API data)
 */

'use client';

import { type ComponentType } from 'react';

// Types
import type { CardContext } from './types';

// Default fallback
import { StructuralCardContent, type StructuralNodeData } from './presets/StructuralCardContent';

// =============================================================================
// TAXONOMY LEVEL (Meta-meta: Realm, Layer, Trait, ArcFamily)
// =============================================================================
import { TaxonomyCardContent, type TaxonomyNodeData } from './presets/TaxonomyCardContent';

// =============================================================================
// SCHEMA LEVEL (Meta: NodeClass, ArcClass)
// =============================================================================
import { ClassCardContent, type ClassNodeData } from './presets/ClassCardContent';

// =====================================================================
// SHARED REALM IMPORTS
// =====================================================================

// Geography layer (shared)
import { GeographyCardContent, type GeographyNodeData } from './presets/GeographyCardContent';

// Knowledge layer - Containers & Atoms (shared)
// v0.18.0: Term/TermSet removed (YAGNI cleanup)
import {
  KnowledgeSetCardContent,
  type KnowledgeSetNodeData,
  ExpressionCardContent,
  type ExpressionNodeData,
  PatternCardContent,
  type PatternNodeData,
  CultureRefCardContent,
  type CultureRefNodeData,
  TabooCardContent,
  type TabooNodeData,
  AudienceTraitCardContent,
  type AudienceTraitNodeData,
  SEOKeywordCardContent,
  type SEOKeywordNodeData,
} from './presets/knowledge';

// Locale layer (shared)
import {
  LocaleSettingsCardContent,
  type LocaleSettingsNodeData,
} from './presets/locale';

// Config layer (shared) - EntityCategory
import {
  EntityCategoryCardContent,
  type EntityCategoryNodeData,
} from './presets/config';

// =====================================================================
// ORG REALM IMPORTS
// =====================================================================

// Foundation layer (org)
import {
  BrandCardContent,
  type BrandNodeData,
  BrandDesignCardContent,
  type BrandDesignNodeData,
  PromptStyleCardContent,
  type PromptStyleNodeData,
} from './presets/foundation';

// Project card (special - uses ProjectCardContent)
import { ProjectCardContent, type ProjectNodeData } from './presets/ProjectCardContent';

// Structure layer (org)
import {
  PageCardContent,
  type PageNodeData,
  BlockCardContent,
  type BlockNodeData,
  ContentSlotCardContent,
  type ContentSlotNodeData,
} from './presets/structure';

// Semantic layer (org)
import {
  EntityCardContent,
  type EntityNodeData,
  EntityNativeCardContent,
  type EntityNativeNodeData,
} from './presets/semantic';

// Instruction layer (org)
import {
  BlockInstructionCardContent,
  type BlockInstructionNodeData,
  BlockTypeCardContent,
  type BlockTypeNodeData,
  BlockRulesCardContent,
  type BlockRulesNodeData,
  PromptArtifactCardContent,
  type PromptArtifactNodeData,
} from './presets/instruction';

// Output layer (org)
import {
  PageNativeCardContent,
  type PageNativeNodeData,
  BlockNativeCardContent,
  type BlockNativeNodeData,
  OutputArtifactCardContent,
  type OutputArtifactNodeData,
} from './presets/output';

// =============================================================================
// TYPES
// =============================================================================

/** Union of all card content data types */
// v0.18.0: TermNodeData removed (Term node type removed in YAGNI cleanup)
export type CardContentData =
  // Fallback
  | StructuralNodeData
  // Taxonomy level (M2: classification system)
  | TaxonomyNodeData
  // Schema level (M1: definitions)
  | ClassNodeData
  // Shared realm
  | GeographyNodeData
  | KnowledgeSetNodeData
  | ExpressionNodeData
  | PatternNodeData
  | CultureRefNodeData
  | TabooNodeData
  | AudienceTraitNodeData
  | SEOKeywordNodeData
  | LocaleSettingsNodeData
  | EntityCategoryNodeData
  // Org realm - Foundation
  | ProjectNodeData
  | BrandNodeData
  | BrandDesignNodeData
  | PromptStyleNodeData
  // Org realm - Structure
  | PageNodeData
  | BlockNodeData
  | ContentSlotNodeData
  // Org realm - Semantic
  | EntityNodeData
  | EntityNativeNodeData
  // Org realm - Instruction
  | BlockInstructionNodeData
  | BlockTypeNodeData
  | BlockRulesNodeData
  | PromptArtifactNodeData
  // Org realm - Output
  | PageNativeNodeData
  | BlockNativeNodeData
  | OutputArtifactNodeData;

/** Props for any card content component */
export interface CardContentProps extends CardContext {
  data: CardContentData;
}

/** Card content component type */
export type CardContentComponent = ComponentType<CardContentProps>;

// =============================================================================
// REGISTRY
// =============================================================================

/**
 * Map node types to their specialized card content components
 *
 * Structure:
 * - key: Node type name (e.g., 'Continent', 'TermSet')
 * - value: Card content component to render
 */
const CARD_CONTENT_REGISTRY: Record<string, CardContentComponent> = {
  // =========================================================================
  // TAXONOMY LEVEL (M2) - Classification system (21 types: Realm, Layer, Trait, ArcFamily)
  // Premium visual treatment: 4px border, triple glow, always-animated
  // =========================================================================
  // Realms (2)
  Realm: TaxonomyCardContent as CardContentComponent,
  NodeRealm: TaxonomyCardContent as CardContentComponent,
  // Layers (10 - includes both Layer node and specific layer attractors)
  Layer: TaxonomyCardContent as CardContentComponent,
  NodeLayer: TaxonomyCardContent as CardContentComponent,
  ConfigLayer: TaxonomyCardContent as CardContentComponent,
  LocaleLayer: TaxonomyCardContent as CardContentComponent,
  GeographyLayer: TaxonomyCardContent as CardContentComponent,
  KnowledgeLayer: TaxonomyCardContent as CardContentComponent,
  FoundationLayer: TaxonomyCardContent as CardContentComponent,
  StructureLayer: TaxonomyCardContent as CardContentComponent,
  SemanticLayer: TaxonomyCardContent as CardContentComponent,
  InstructionLayer: TaxonomyCardContent as CardContentComponent,
  OutputLayer: TaxonomyCardContent as CardContentComponent,
  // Traits (5)
  Trait: TaxonomyCardContent as CardContentComponent,
  NodeTrait: TaxonomyCardContent as CardContentComponent,
  // ArcFamilies (5)
  ArcFamily: TaxonomyCardContent as CardContentComponent,

  // =========================================================================
  // SCHEMA LEVEL (M1) - Definitions (57 NodeClass + 145 ArcClass)
  // Elevated visual treatment: 2px border, single glow, interaction-based animation
  // v0.18.0: Updated counts after YAGNI cleanup
  // =========================================================================
  NodeClass: ClassCardContent as CardContentComponent,
  ArcClass: ClassCardContent as CardContentComponent,
  Class: ClassCardContent as CardContentComponent,
  Schema: ClassCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Geography layer (7 types)
  // =========================================================================
  Continent: GeographyCardContent as CardContentComponent,
  GeoRegion: GeographyCardContent as CardContentComponent,
  GeoSubRegion: GeographyCardContent as CardContentComponent,
  Country: GeographyCardContent as CardContentComponent,
  City: GeographyCardContent as CardContentComponent,
  StateProvince: GeographyCardContent as CardContentComponent,
  PostalCode: GeographyCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Config layer (EntityCategory - ADR-017)
  // Crystal Badge design: glassmorphism + BorderBeam for classification nodes
  // =========================================================================
  EntityCategory: EntityCategoryCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Locale layer settings (5 types, v0.18.0: Market removed)
  // =========================================================================
  Culture: LocaleSettingsCardContent as CardContentComponent,
  Style: LocaleSettingsCardContent as CardContentComponent,
  Formatting: LocaleSettingsCardContent as CardContentComponent,
  Adaptation: LocaleSettingsCardContent as CardContentComponent,
  Slugification: LocaleSettingsCardContent as CardContentComponent,
  // v0.18.0: Market removed (market data from external APIs, not static graph)

  // =========================================================================
  // SHARED REALM - Knowledge layer - Containers (5 types, v0.18.0: TermSet removed)
  // =========================================================================
  ExpressionSet: KnowledgeSetCardContent as CardContentComponent,
  PatternSet: KnowledgeSetCardContent as CardContentComponent,
  CultureSet: KnowledgeSetCardContent as CardContentComponent,
  TabooSet: KnowledgeSetCardContent as CardContentComponent,
  AudienceSet: KnowledgeSetCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Knowledge layer - Atoms (5 types, v0.18.0: Term removed)
  // =========================================================================
  Expression: ExpressionCardContent as CardContentComponent,
  Pattern: PatternCardContent as CardContentComponent,
  CultureRef: CultureRefCardContent as CardContentComponent,
  Taboo: TabooCardContent as CardContentComponent,
  AudienceTrait: AudienceTraitCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Knowledge layer - SEO/GEO
  // =========================================================================
  SEOKeyword: SEOKeywordCardContent as CardContentComponent,

  // =========================================================================
  // ORG REALM - Foundation layer (4 types)
  // =========================================================================
  Project: ProjectCardContent as CardContentComponent,
  Brand: BrandCardContent as CardContentComponent,
  BrandDesign: BrandDesignCardContent as CardContentComponent,
  PromptStyle: PromptStyleCardContent as CardContentComponent,

  // =========================================================================
  // ORG REALM - Structure layer (3 types)
  // =========================================================================
  Page: PageCardContent as CardContentComponent,
  Block: BlockCardContent as CardContentComponent,
  ContentSlot: ContentSlotCardContent as CardContentComponent,

  // =========================================================================
  // ORG REALM - Semantic layer (2 types with specialized cards)
  // =========================================================================
  Entity: EntityCardContent as CardContentComponent,
  EntityNative: EntityNativeCardContent as CardContentComponent,

  // =========================================================================
  // ORG REALM - Instruction layer (4 types)
  // =========================================================================
  BlockInstruction: BlockInstructionCardContent as CardContentComponent,
  BlockType: BlockTypeCardContent as CardContentComponent,
  BlockRules: BlockRulesCardContent as CardContentComponent,
  PromptArtifact: PromptArtifactCardContent as CardContentComponent,

  // =========================================================================
  // ORG REALM - Output layer (3 types)
  // =========================================================================
  PageNative: PageNativeCardContent as CardContentComponent,
  BlockNative: BlockNativeCardContent as CardContentComponent,
  OutputArtifact: OutputArtifactCardContent as CardContentComponent,
};

// =============================================================================
// SELECTOR FUNCTION
// =============================================================================

/**
 * Get the appropriate card content component for a node type
 *
 * @param nodeType - The type of node (e.g., 'Continent', 'Term')
 * @returns The card content component to use, or StructuralCardContent as fallback
 */
export function getCardContentComponent(nodeType: string): CardContentComponent {
  return CARD_CONTENT_REGISTRY[nodeType] ?? (StructuralCardContent as CardContentComponent);
}

/**
 * Check if a node type has a specialized card component
 *
 * @param nodeType - The type of node
 * @returns true if the node type has a specialized card
 */
export function hasSpecializedCard(nodeType: string): boolean {
  return nodeType in CARD_CONTENT_REGISTRY;
}

/**
 * Get all node types that have specialized cards
 *
 * @returns Array of node type names
 */
export function getSpecializedNodeTypes(): string[] {
  return Object.keys(CARD_CONTENT_REGISTRY);
}
