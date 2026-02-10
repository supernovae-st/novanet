'use client';

/**
 * FacetFilterPanel - Faceted filter panel for Query mode
 *
 * Four facet sections:
 * - Realms (2): shared, org
 * - Layers (8): config, locale-knowledge, foundation, structure, semantic, instruction, output, seo
 * - Traits (5): invariant, localized, knowledge, generated, aggregated
 * - Arc Families (5): ownership, localization, semantic, generation, mining
 *
 * Reads/writes filterStore facet state.
 * Uses Sidebar compound component for consistent styling.
 */

import { memo, useCallback, useMemo } from 'react';
import {
  Globe,
  Building2,
  Target,
  Landmark,
  Layers,
  Lightbulb,
  FileText,
  FileOutput,
  Settings,
  Brain,
  Search,
  Globe2,
  Lock,
  Languages,
  BookOpen,
  Sparkles,
  Cpu,
  Link,
  ArrowRightLeft,
  Waypoints,
  Wand2,
  Pickaxe,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import type { Realm, Layer, Trait } from '@novanet/core/types';
import { useFilterStore } from '@/stores/filterStore';
import { iconSizes } from '@/design/tokens';
import {
  REALM_COLORS,
  LAYER_COLORS,
  TRAIT_COLORS,
  ARC_FAMILY_COLORS,
} from '@/design/colors';
import { Sidebar } from './SidebarContent';

// =============================================================================
// CONSTANTS
// =============================================================================

const REALMS: { key: Realm; label: string; icon: LucideIcon }[] = [
  { key: 'shared', label: 'Global', icon: Globe },
  { key: 'org', label: 'Tenant', icon: Building2 },
];

const LAYERS: { key: Layer; label: string; icon: LucideIcon }[] = [
  { key: 'config', label: 'Configuration', icon: Settings },
  { key: 'locale-knowledge', label: 'Locale Knowledge', icon: Brain },
  { key: 'foundation', label: 'Foundation', icon: Landmark },
  { key: 'structure', label: 'Structure', icon: Layers },
  { key: 'semantic', label: 'Semantic', icon: Lightbulb },
  { key: 'instruction', label: 'Instructions', icon: FileText },
  { key: 'output', label: 'Generated Output', icon: FileOutput },
  { key: 'seo', label: 'SEO Intelligence', icon: Search },
];

const TRAITS: { key: Trait; label: string; icon: LucideIcon }[] = [
  { key: 'invariant', label: 'Invariant', icon: Lock },
  { key: 'localized', label: 'Localized', icon: Languages },
  { key: 'knowledge', label: 'Knowledge', icon: BookOpen },
  { key: 'generated', label: 'Generated', icon: Sparkles },
  { key: 'aggregated', label: 'Aggregated', icon: Cpu },
];

const ARC_FAMILIES: { key: string; label: string; icon: LucideIcon }[] = [
  { key: 'ownership', label: 'Ownership', icon: Link },
  { key: 'localization', label: 'Localization', icon: ArrowRightLeft },
  { key: 'semantic', label: 'Semantic', icon: Waypoints },
  { key: 'generation', label: 'Generation', icon: Wand2 },
  { key: 'mining', label: 'Mining', icon: Pickaxe },
];

// =============================================================================
// COMPONENT
// =============================================================================

export interface FacetFilterPanelProps {
  className?: string;
}

export const FacetFilterPanel = memo(function FacetFilterPanel({
  className,
}: FacetFilterPanelProps) {
  const {
    realmFilter,
    traitFilter,
    layerFilter,
    arcFamilyFilter,
    toggleRealm,
    toggleTrait,
    toggleLayer,
    toggleArcFamily,
  } = useFilterStore(
    useShallow((s) => ({
      realmFilter: s.realmFilter,
      traitFilter: s.traitFilter,
      layerFilter: s.layerFilter,
      arcFamilyFilter: s.arcFamilyFilter,
      toggleRealm: s.toggleRealm,
      toggleTrait: s.toggleTrait,
      toggleLayer: s.toggleLayer,
      toggleArcFamily: s.toggleArcFamily,
    }))
  );

  const realmSet = useMemo(() => new Set(realmFilter), [realmFilter]);
  const layerSet = useMemo(() => new Set(layerFilter), [layerFilter]);
  const traitSet = useMemo(() => new Set(traitFilter), [traitFilter]);
  const arcFamilySet = useMemo(() => new Set(arcFamilyFilter), [arcFamilyFilter]);

  // Section-level tri-state: all checked, some, none
  const realmCheckboxState = useMemo(() => {
    if (realmSet.size === 0) return 'none' as const;
    if (realmSet.size === REALMS.length) return 'all' as const;
    return 'partial' as const;
  }, [realmSet]);

  const layerCheckboxState = useMemo(() => {
    if (layerSet.size === 0) return 'none' as const;
    if (layerSet.size === LAYERS.length) return 'all' as const;
    return 'partial' as const;
  }, [layerSet]);

  const traitCheckboxState = useMemo(() => {
    if (traitSet.size === 0) return 'none' as const;
    if (traitSet.size === TRAITS.length) return 'all' as const;
    return 'partial' as const;
  }, [traitSet]);

  const arcFamilyCheckboxState = useMemo(() => {
    if (arcFamilySet.size === 0) return 'none' as const;
    if (arcFamilySet.size === ARC_FAMILIES.length) return 'all' as const;
    return 'partial' as const;
  }, [arcFamilySet]);

  // Toggle all realms
  const handleRealmSectionClick = useCallback(() => {
    const allKeys = REALMS.map((r) => r.key);
    const store = useFilterStore.getState();
    if (realmCheckboxState !== 'none') {
      store.setRealmFilter([]);
    } else {
      store.setRealmFilter(allKeys);
    }
  }, [realmCheckboxState]);

  // Toggle all layers (facet filter layers, not schema collapse layers)
  const handleLayerSectionClick = useCallback(() => {
    const allKeys = LAYERS.map((l) => l.key);
    const store = useFilterStore.getState();
    if (layerCheckboxState !== 'none') {
      store.setLayerFilter([]);
    } else {
      store.setLayerFilter(allKeys);
    }
  }, [layerCheckboxState]);

  // Toggle all traits
  const handleTraitSectionClick = useCallback(() => {
    const allKeys = TRAITS.map((t) => t.key);
    const store = useFilterStore.getState();
    if (traitCheckboxState !== 'none') {
      store.setTraitFilter([]);
    } else {
      store.setTraitFilter(allKeys);
    }
  }, [traitCheckboxState]);

  // Toggle all arc families
  const handleArcFamilySectionClick = useCallback(() => {
    const allKeys = ARC_FAMILIES.map((f) => f.key);
    const store = useFilterStore.getState();
    if (arcFamilyCheckboxState !== 'none') {
      store.setArcFamilyFilter([]);
    } else {
      store.setArcFamilyFilter(allKeys);
    }
  }, [arcFamilyCheckboxState]);

  // Active facet count
  const activeFacetCount = realmSet.size + layerSet.size + traitSet.size + arcFamilySet.size;

  return (
    <Sidebar.Content
      testId="facet-filter-panel"
      className={className}
      toolbar={
        <div className="px-3 py-2.5">
          <div className="flex items-center justify-between">
            <span className="text-xs font-medium text-white/70">Faceted Query</span>
            {activeFacetCount > 0 && (
              <span className="text-[10px] text-amber-400/80 tabular-nums">
                {activeFacetCount} active
              </span>
            )}
          </div>
        </div>
      }
      footer={
        activeFacetCount === 0 ? (
          <div className="px-3 py-2 text-[10px] text-white/30 text-center">
            Select facets to filter the graph
          </div>
        ) : undefined
      }
    >
      <Sidebar.Tree showProgressBars={false} maxCount={0}>
        {/* Realms Section */}
        <Sidebar.Section
          id="facet-realms"
          label="Realms"
          icon={<Globe className={iconSizes.sm} />}
          color={REALM_COLORS.shared.color}
          checkboxState={realmCheckboxState}
          onCheckboxClick={handleRealmSectionClick}
          count={REALMS.length}
          defaultExpanded
        >
          {REALMS.map(({ key, label, icon: Icon }) => (
            <Sidebar.Row
              key={key}
              id={`facet-realm-${key}`}
              label={label}
              icon={<Icon className={iconSizes.sm} />}
              color={REALM_COLORS[key].color}
              isSelected={realmSet.has(key)}
              onToggle={() => toggleRealm(key)}
            />
          ))}
        </Sidebar.Section>

        {/* Layers Section */}
        <Sidebar.Section
          id="facet-layers"
          label="Layers"
          icon={<Layers className={iconSizes.sm} />}
          color={LAYER_COLORS.foundation.color}
          checkboxState={layerCheckboxState}
          onCheckboxClick={handleLayerSectionClick}
          count={LAYERS.length}
          defaultExpanded
        >
          {LAYERS.map(({ key, label, icon: Icon }) => (
            <Sidebar.Row
              key={key}
              id={`facet-layer-${key}`}
              label={label}
              icon={<Icon className={iconSizes.sm} />}
              color={LAYER_COLORS[key].color}
              isSelected={layerSet.has(key)}
              onToggle={() => toggleLayer(key)}
            />
          ))}
        </Sidebar.Section>

        {/* Traits Section */}
        <Sidebar.Section
          id="facet-traits"
          label="Traits"
          icon={<Sparkles className={iconSizes.sm} />}
          color={TRAIT_COLORS.invariant.color}
          checkboxState={traitCheckboxState}
          onCheckboxClick={handleTraitSectionClick}
          count={TRAITS.length}
          defaultExpanded
        >
          {TRAITS.map(({ key, label, icon: Icon }) => (
            <Sidebar.Row
              key={key}
              id={`facet-trait-${key}`}
              label={label}
              icon={<Icon className={iconSizes.sm} />}
              color={TRAIT_COLORS[key].color}
              isSelected={traitSet.has(key)}
              onToggle={() => toggleTrait(key)}
            />
          ))}
        </Sidebar.Section>

        {/* Arc Families Section */}
        <Sidebar.Section
          id="facet-arc-families"
          label="Arc Families"
          icon={<Link className={iconSizes.sm} />}
          color={ARC_FAMILY_COLORS.ownership.color}
          checkboxState={arcFamilyCheckboxState}
          onCheckboxClick={handleArcFamilySectionClick}
          count={ARC_FAMILIES.length}
          defaultExpanded
        >
          {ARC_FAMILIES.map(({ key, label, icon: Icon }) => (
            <Sidebar.Row
              key={key}
              id={`facet-arc-family-${key}`}
              label={label}
              icon={<Icon className={iconSizes.sm} />}
              color={ARC_FAMILY_COLORS[key as keyof typeof ARC_FAMILY_COLORS].color}
              isSelected={arcFamilySet.has(key)}
              onToggle={() => toggleArcFamily(key)}
            />
          ))}
        </Sidebar.Section>
      </Sidebar.Tree>
    </Sidebar.Content>
  );
});
