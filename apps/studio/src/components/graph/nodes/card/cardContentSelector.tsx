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
 * Coverage: 61 node types across 10 layers (SHARED + ORG realms)
 */

'use client';

import { type ComponentType } from 'react';

// Types
import type { CardContext } from './types';

// Default fallback
import { StructuralCardContent, type StructuralNodeData } from './presets/StructuralCardContent';

// =====================================================================
// SHARED REALM IMPORTS
// =====================================================================

// Geography layer (shared)
import { GeographyCardContent, type GeographyNodeData } from './presets/GeographyCardContent';

// Knowledge layer - Containers & Atoms (shared)
import {
  KnowledgeSetCardContent,
  type KnowledgeSetNodeData,
  TermCardContent,
  type TermNodeData,
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
export type CardContentData =
  // Fallback
  | StructuralNodeData
  // Shared realm
  | GeographyNodeData
  | KnowledgeSetNodeData
  | TermNodeData
  | ExpressionNodeData
  | PatternNodeData
  | CultureRefNodeData
  | TabooNodeData
  | AudienceTraitNodeData
  | SEOKeywordNodeData
  | LocaleSettingsNodeData
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
  // SHARED REALM - Locale layer settings (6 types)
  // =========================================================================
  Culture: LocaleSettingsCardContent as CardContentComponent,
  Style: LocaleSettingsCardContent as CardContentComponent,
  Formatting: LocaleSettingsCardContent as CardContentComponent,
  Adaptation: LocaleSettingsCardContent as CardContentComponent,
  Slugification: LocaleSettingsCardContent as CardContentComponent,
  Market: LocaleSettingsCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Knowledge layer - Containers (6 types)
  // =========================================================================
  TermSet: KnowledgeSetCardContent as CardContentComponent,
  ExpressionSet: KnowledgeSetCardContent as CardContentComponent,
  PatternSet: KnowledgeSetCardContent as CardContentComponent,
  CultureSet: KnowledgeSetCardContent as CardContentComponent,
  TabooSet: KnowledgeSetCardContent as CardContentComponent,
  AudienceSet: KnowledgeSetCardContent as CardContentComponent,

  // =========================================================================
  // SHARED REALM - Knowledge layer - Atoms (6 types)
  // =========================================================================
  Term: TermCardContent as CardContentComponent,
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
