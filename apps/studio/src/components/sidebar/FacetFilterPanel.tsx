'use client';

/**
 * FacetFilterPanel - Faceted filter panel for Query mode
 *
 * Three facet sections:
 * - Realms (3): global, project, shared
 * - Layers (9): config, knowledge, foundation, ...
 * - Traits (5): invariant, localized, knowledge, derived, job
 *
 * Reads/writes filterStore facet state.
 * Uses Sidebar compound component for consistent styling.
 */

import { memo, useCallback, useMemo } from 'react';
import {
  Globe,
  Package,
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
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import type { Realm, Layer, Trait } from '@novanet/core/types';
import { useFilterStore } from '@/stores/filterStore';
import { realmAccents, iconSizes } from '@/design/tokens';
import { Sidebar } from './SidebarContent';

// =============================================================================
// CONSTANTS
// =============================================================================

const REALMS: { key: Realm; label: string; icon: LucideIcon }[] = [
  { key: 'global', label: 'Global', icon: Globe },
  { key: 'project', label: 'Project', icon: Package },
  { key: 'shared', label: 'Shared', icon: Target },
];

const LAYERS: { key: Layer; label: string; icon: LucideIcon }[] = [
  { key: 'config', label: 'Configuration', icon: Settings },
  { key: 'knowledge', label: 'Locale Knowledge', icon: Brain },
  { key: 'foundation', label: 'Foundation', icon: Landmark },
  { key: 'structure', label: 'Structure', icon: Layers },
  { key: 'semantic', label: 'Semantic', icon: Lightbulb },
  { key: 'instruction', label: 'Instructions', icon: FileText },
  { key: 'output', label: 'Generated Output', icon: FileOutput },
  { key: 'seo', label: 'SEO Intelligence', icon: Search },
  { key: 'geo', label: 'GEO Intelligence', icon: Globe2 },
];

const TRAITS: { key: Trait; label: string; icon: LucideIcon }[] = [
  { key: 'invariant', label: 'Invariant', icon: Lock },
  { key: 'localized', label: 'Localized', icon: Languages },
  { key: 'knowledge', label: 'Knowledge', icon: BookOpen },
  { key: 'derived', label: 'Derived', icon: Sparkles },
  { key: 'job', label: 'Job', icon: Cpu },
];

const REALM_COLORS: Record<Realm, string> = {
  global: realmAccents.global.color,
  project: realmAccents.project.color,
  shared: realmAccents.shared.color,
};

// Consistent accent for layer and trait sections
const LAYER_SECTION_COLOR = '#6c71c4'; // violet
const TRAIT_SECTION_COLOR = '#d33682'; // magenta

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
    toggleRealm,
    toggleTrait,
    toggleLayer,
  } = useFilterStore(
    useShallow((s) => ({
      realmFilter: s.realmFilter,
      traitFilter: s.traitFilter,
      layerFilter: s.layerFilter,
      toggleRealm: s.toggleRealm,
      toggleTrait: s.toggleTrait,
      toggleLayer: s.toggleLayer,
    }))
  );

  const realmSet = useMemo(() => new Set(realmFilter), [realmFilter]);
  const layerSet = useMemo(() => new Set(layerFilter), [layerFilter]);
  const traitSet = useMemo(() => new Set(traitFilter), [traitFilter]);

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

  // Active facet count
  const activeFacetCount = realmSet.size + layerSet.size + traitSet.size;

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
          color={realmAccents.global.color}
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
              color={REALM_COLORS[key]}
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
          color={LAYER_SECTION_COLOR}
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
              color={LAYER_SECTION_COLOR}
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
          color={TRAIT_SECTION_COLOR}
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
              color={TRAIT_SECTION_COLOR}
              isSelected={traitSet.has(key)}
              onToggle={() => toggleTrait(key)}
            />
          ))}
        </Sidebar.Section>
      </Sidebar.Tree>
    </Sidebar.Content>
  );
});
