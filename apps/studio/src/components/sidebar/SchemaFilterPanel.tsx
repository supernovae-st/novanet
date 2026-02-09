'use client';

/**
 * SchemaFilterPanel - Schema Browser with NodeCard display
 *
 * v11.0: Unified card-based schema browser similar to TUI meta view.
 * Uses NodeCard components for consistent styling across the app.
 */

import { memo, useMemo, useState } from 'react';
import { RelationType } from '@novanet/core/schemas';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { Sidebar } from './SidebarContent';
import { SchemaCardView } from './SchemaCardView';

// Tab definitions
type SchemaTabId = 'types' | 'rels';

// Realm to Lucide icon mapping (v10.6: 2 realms)
const REALM_ICONS: Record<Realm, LucideIcon> = {
  global: Globe,
  tenant: Building2,
};

// Layer to Lucide icon mapping
const LAYER_ICONS: Record<Layer, LucideIcon> = {
  config: Settings,
  'locale-knowledge': Brain,
  seo: Search,
  foundation: Landmark,
  structure: Layers,
  semantic: Lightbulb,
  instruction: FileText,
  output: FileOutput,
};

// Ordered realms for consistent rendering (v10.6: 2 realms)
const REALM_ORDER: Realm[] = ['global', 'tenant'];

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
      const accent = REALM_COLORS[realm];
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
