'use client';

/**
 * TaxonomyCard - Level 1 node cards for the NovaNet unified tree
 *
 * Four variants for the 21 taxonomy nodes:
 * - Realm (2): shared, org
 * - Layer (10): config, locale, geography, knowledge, foundation, structure, semantic, instruction, output
 * - Trait (5): defined, authored, imported, generated, retrieved
 * - ArcFamily (5): ownership, localization, semantic, generation, mining
 *
 * Visual identity per variant:
 * - Realm: Orbital rings + cosmic glow
 * - Layer: Stacked planes + depth effect
 * - Trait: Border morphing + style preview
 * - ArcFamily: Radiating pulse + connection lines
 *
 * @example
 * ```tsx
 * <TaxonomyCard data={realmData} />
 * ```
 */

import { memo, useMemo } from 'react';
import { CardShell } from '../card/CardShell';
import type {
  TaxonomyNodeDataUnion,
  RealmTaxonomyData,
  LayerTaxonomyData,
  TraitTaxonomyData,
  ArcFamilyTaxonomyData,
  CardContext,
  TaxonomyVariant,
} from '../card/types';
import {
  OrbitalRings,
  StackedPlanes,
  BorderMorph,
  RadiatingPulse,
  MotionTechCorners,
} from '../effects';

// =============================================================================
// Types
// =============================================================================

// React Flow requires Record<string, unknown> for node data
// We use a more specific type internally but accept the generic for props
export interface TaxonomyCardProps {
  data: TaxonomyNodeDataUnion;
  selected?: boolean;
}

interface TaxonomyContentProps {
  data: TaxonomyNodeDataUnion;
  context: CardContext;
}

// =============================================================================
// Variant Icons (Unicode for consistency with TUI)
// =============================================================================

const VARIANT_ICONS: Record<TaxonomyVariant, string> = {
  realm: '\u25C9',     // ◉
  layer: '\u25C8',     // ◈
  trait: '\u25C6',     // ◆
  arcFamily: '\u2192', // →
};

const VARIANT_LABELS: Record<TaxonomyVariant, string> = {
  realm: 'REALM',
  layer: 'LAYER',
  trait: 'TRAIT',
  arcFamily: 'ARC FAMILY',
};

// =============================================================================
// Realm Content
// =============================================================================

const RealmContent = memo(function RealmContent({ data, context }: TaxonomyContentProps) {
  const realmData = data as RealmTaxonomyData;

  return (
    <>
      {/* Orbital rings effect */}
      <OrbitalRings
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        size={context.width ?? 200}
        performanceConfig={context.performanceConfig}
      />

      {/* Tech corners */}
      <MotionTechCorners
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        performanceConfig={context.performanceConfig}
      />

      {/* Content */}
      <div className="relative z-10 p-4 text-center">
        <div
          className="text-[10px] font-mono uppercase tracking-widest mb-2"
          style={{ color: `${context.colors.primary}80` }}
        >
          {VARIANT_ICONS.realm} {VARIANT_LABELS.realm}
        </div>
        <h3 className="text-lg font-bold text-white mb-2">{data.displayName}</h3>
        <div className="flex justify-center gap-2 text-xs">
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
          >
            {realmData.layerCount} layers
          </span>
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.secondary}20`, color: context.colors.secondary }}
          >
            {realmData.nodeClassCount} classes
          </span>
        </div>
      </div>
    </>
  );
});

// =============================================================================
// Layer Content
// =============================================================================

const LayerContent = memo(function LayerContent({ data, context }: TaxonomyContentProps) {
  const layerData = data as LayerTaxonomyData;

  return (
    <>
      {/* Stacked planes effect */}
      <StackedPlanes
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        width={context.width ?? 200}
        height={80}
        performanceConfig={context.performanceConfig}
      />

      {/* Tech corners */}
      <MotionTechCorners
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        performanceConfig={context.performanceConfig}
      />

      {/* Content */}
      <div className="relative z-10 p-4 text-center">
        <div
          className="text-[10px] font-mono uppercase tracking-widest mb-2"
          style={{ color: `${context.colors.primary}80` }}
        >
          {VARIANT_ICONS.layer} {VARIANT_LABELS.layer}
        </div>
        <h3 className="text-lg font-bold text-white mb-2">{data.displayName}</h3>
        <div className="flex justify-center gap-2 text-xs">
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.secondary}20`, color: context.colors.secondary }}
          >
            {layerData.parentRealm}
          </span>
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
          >
            {layerData.nodeClassCount} classes
          </span>
        </div>
      </div>
    </>
  );
});

// =============================================================================
// Trait Content
// =============================================================================

const TraitContent = memo(function TraitContent({ data, context }: TaxonomyContentProps) {
  const traitData = data as TraitTaxonomyData;

  return (
    <>
      {/* Border morph effect showing the trait's border style */}
      <BorderMorph
        color={context.colors.primary}
        borderStyle={traitData.borderStyle.style}
        selected={context.selected}
        isHovered={context.isHovered}
        performanceConfig={context.performanceConfig}
      />

      {/* Tech corners */}
      <MotionTechCorners
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        performanceConfig={context.performanceConfig}
      />

      {/* Content */}
      <div className="relative z-10 p-4 text-center">
        <div
          className="text-[10px] font-mono uppercase tracking-widest mb-2"
          style={{ color: `${context.colors.primary}80` }}
        >
          {VARIANT_ICONS.trait} {VARIANT_LABELS.trait}
        </div>
        <h3 className="text-lg font-bold text-white mb-2">{data.displayName}</h3>
        <div className="flex justify-center gap-2 text-xs">
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
          >
            {traitData.borderStyle.style}
          </span>
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.secondary}20`, color: context.colors.secondary }}
          >
            {traitData.nodeClassCount} classes
          </span>
        </div>
      </div>
    </>
  );
});

// =============================================================================
// ArcFamily Content
// =============================================================================

const ArcFamilyContent = memo(function ArcFamilyContent({ data, context }: TaxonomyContentProps) {
  const arcFamilyData = data as ArcFamilyTaxonomyData;

  return (
    <>
      {/* Radiating pulse effect */}
      <RadiatingPulse
        color={context.colors.primary}
        active={context.selected || context.isHovered}
        size={context.width ?? 200}
        performanceConfig={context.performanceConfig}
      />

      {/* Tech corners */}
      <MotionTechCorners
        color={context.colors.primary}
        selected={context.selected}
        isHovered={context.isHovered}
        performanceConfig={context.performanceConfig}
      />

      {/* Content */}
      <div className="relative z-10 p-4 text-center">
        <div
          className="text-[10px] font-mono uppercase tracking-widest mb-2"
          style={{ color: `${context.colors.primary}80` }}
        >
          {VARIANT_ICONS.arcFamily} {VARIANT_LABELS.arcFamily}
        </div>
        <h3 className="text-lg font-bold text-white mb-2">{data.displayName}</h3>
        <div className="flex justify-center gap-2 text-xs">
          <span
            className="px-2 py-0.5 rounded-full font-mono"
            style={{ backgroundColor: `${context.colors.primary}20`, color: context.colors.primary }}
          >
            {arcFamilyData.arcClassCount} arcs
          </span>
        </div>
      </div>
    </>
  );
});

// =============================================================================
// Content Renderer
// =============================================================================

const TaxonomyContent = memo(function TaxonomyContent({ data, context }: TaxonomyContentProps) {
  switch (data.variant) {
    case 'realm':
      return <RealmContent data={data} context={context} />;
    case 'layer':
      return <LayerContent data={data} context={context} />;
    case 'trait':
      return <TraitContent data={data} context={context} />;
    case 'arcFamily':
      return <ArcFamilyContent data={data} context={context} />;
    default:
      return null;
  }
});

// =============================================================================
// TaxonomyCard Component
// =============================================================================

/**
 * Unified card component for all Level 1 (Taxonomy) nodes.
 *
 * Automatically renders the appropriate content based on the variant prop.
 */
export const TaxonomyCard = memo(function TaxonomyCard({ data, selected }: TaxonomyCardProps) {
  // Extract colors from the data
  const colors = useMemo(
    () => ({
      primary: data.color,
      secondary: data.parentRealm
        ? data.color // Layer uses its own color
        : data.color, // Others use primary
    }),
    [data.color, data.parentRealm]
  );

  return (
    <CardShell
      colors={colors}
      selected={selected ?? false}
      width={220}
      showPulseRing={true}
      showHandles={true}
      showGlassmorphism={true}
      borderRadius={16}
      renderContent={(context) => <TaxonomyContent data={data} context={context} />}
    />
  );
});
