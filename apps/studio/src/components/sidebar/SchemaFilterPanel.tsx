'use client';

/**
 * SchemaFilterPanel - Schema Browser using unified Sidebar components
 *
 * Uses Sidebar.Content for consistent skeleton across all tabs:
 * - Same header structure
 * - Same body padding (p-3)
 * - Same row heights and spacing
 *
 * Now unified with Data Explorer: AI Search + Tabs (Types | Rels)
 */

import { memo, useCallback, useMemo, useState } from 'react';
import {
  Landmark,
  Layers,
  Lightbulb,
  FileText,
  FileOutput,
  Settings,
  Brain,
  Search,
  Globe2,
  Package,
  Globe,
  Target,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { REALM_HIERARCHY } from '@novanet/core/graph';
import type { Layer } from '@novanet/core/graph';
import { Realm } from '@novanet/core/types';
import { RelationType } from '@novanet/core/schemas';
import { useFilterStore } from '@/stores/filterStore';
import { realmAccents, iconSizes } from '@/design/tokens';
import { calculateCheckboxState } from '@/hooks';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { Sidebar } from './SidebarContent';
import { AiSearchInput } from './AiSearchInput';

// Tab definitions
type SchemaTabId = 'types' | 'rels';

// Realm to Lucide icon mapping
const REALM_ICONS: Record<Realm, LucideIcon> = {
  project: Package,
  global: Globe,
  shared: Target,
};

// Layer to Lucide icon mapping
const LAYER_ICONS: Record<Layer, LucideIcon> = {
  foundation: Landmark,
  structure: Layers,
  semantic: Lightbulb,
  instruction: FileText,
  output: FileOutput,
  config: Settings,
  knowledge: Brain,
  seo: Search,
  geo: Globe2,
};

// Ordered realms for consistent rendering
const REALM_ORDER: Realm[] = ['project', 'global', 'shared'];

// Map realm names to accent keys
const REALM_ACCENT_MAP: Record<Realm, keyof typeof realmAccents> = {
  project: 'project',
  global: 'global',
  shared: 'shared',
};

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const [activeTab, setActiveTab] = useState<SchemaTabId>('types');

  const {
    toggleLayerCollapsed,
    isLayerCollapsed,
    setLayerCollapsed,
  } = useFilterStore(
    useShallow((state) => ({
      toggleLayerCollapsed: state.toggleLayerCollapsed,
      isLayerCollapsed: state.isLayerCollapsed,
      setLayerCollapsed: state.setLayerCollapsed,
    }))
  );

  // Relationship count
  const relCount = useMemo(() => {
    return Object.keys(RelationType).length;
  }, []);

  // Tab definitions
  const tabs = useMemo(() => [
    { id: 'types' as const, label: 'Types', count: 35 },
    { id: 'rels' as const, label: 'Rels', count: relCount },
  ], [relCount]);

  // Memoize realm data
  const realmData = useMemo(() => {
    return REALM_ORDER.map((realm) => {
      const scopeDef = REALM_HIERARCHY[realm];
      const accentKey = REALM_ACCENT_MAP[realm];
      const accent = realmAccents[accentKey];
      const layers = Object.entries(scopeDef.layers) as [
        Layer,
        (typeof scopeDef.layers)[Layer],
      ][];
      const nodeCount = layers.reduce(
        (sum, [, subcat]) => sum + subcat.nodeTypes.length,
        0
      );

      return {
        realm,
        realmDef: scopeDef,
        accent,
        layers,
        nodeCount,
      };
    });
  }, []);

  // Get visible layers as a Set for checkbox state calculation
  const getVisibleLayers = useCallback(
    (realm: Realm): Set<string> => {
      const realmDef = REALM_HIERARCHY[realm];
      const layerNames = Object.keys(realmDef.layers) as Layer[];
      const visible = new Set<string>();
      layerNames.forEach((name) => {
        if (!isLayerCollapsed(realm, name)) {
          visible.add(name);
        }
      });
      return visible;
    },
    [isLayerCollapsed]
  );

  // Calculate checkbox state for a realm
  const getRealmCheckboxState = useCallback(
    (realm: Realm): CheckboxState => {
      const realmDef = REALM_HIERARCHY[realm];
      const layerNames = Object.keys(realmDef.layers) as Layer[];
      const visible = getVisibleLayers(realm);
      return calculateCheckboxState(layerNames, visible);
    },
    [getVisibleLayers]
  );

  // Handle realm checkbox click
  const handleRealmCheckboxClick = useCallback(
    (realm: Realm) => {
      const realmDef = REALM_HIERARCHY[realm];
      const layerNames = Object.keys(realmDef.layers) as Layer[];
      const currentState = getRealmCheckboxState(realm);

      const shouldCollapse = currentState !== 'none';
      layerNames.forEach((name) => {
        setLayerCollapsed(realm, name, shouldCollapse);
      });
    },
    [getRealmCheckboxState, setLayerCollapsed]
  );

  // Render Types tab content
  const renderTypesContent = () => (
    <Sidebar.Tree showProgressBars={false} maxCount={35}>
      {realmData.map(({ realm, realmDef, accent, layers, nodeCount }) => {
        const RealmIcon = REALM_ICONS[realm];
        return (
          <Sidebar.Section
            key={realm}
            id={realm}
            label={realmDef.label}
            icon={<RealmIcon className={iconSizes.sm} />}
            color={accent.color}
            checkboxState={getRealmCheckboxState(realm)}
            onCheckboxClick={() => handleRealmCheckboxClick(realm)}
            count={nodeCount}
            defaultExpanded
          >
            {layers.map(([layerName, layerMeta]) => {
              const isVisible = !isLayerCollapsed(realm, layerName);
              const LayerIcon = LAYER_ICONS[layerName];

              return (
                <Sidebar.Row
                  key={layerName}
                  id={`${realm}-${layerName}`}
                  label={layerMeta.label}
                  icon={<LayerIcon className={iconSizes.sm} />}
                  color={accent.color}
                  isSelected={isVisible}
                  onToggle={() => toggleLayerCollapsed(realm, layerName)}
                  count={layerMeta.nodeTypes.length}
                />
              );
            })}
          </Sidebar.Section>
        );
      })}
    </Sidebar.Tree>
  );

  // Render Rels tab content (placeholder for now)
  const renderRelsContent = () => (
    <div className="flex items-center justify-center h-32 text-white/40 text-sm">
      Relationships browser coming soon
    </div>
  );

  return (
    <Sidebar.Content
      testId="schema-filter-panel"
      className={className}
      toolbar={
        <div className="flex flex-col gap-2 px-3 py-2.5">
          <AiSearchInput placeholder="Ask AI about the schema…" />
          <SegmentedTabs
            tabs={tabs}
            activeTab={activeTab}
            onTabChange={(id) => setActiveTab(id as SchemaTabId)}
          />
        </div>
      }
    >
      {activeTab === 'types' ? renderTypesContent() : renderRelsContent()}
    </Sidebar.Content>
  );
});
