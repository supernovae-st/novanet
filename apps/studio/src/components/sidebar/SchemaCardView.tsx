'use client';

/**
 * SchemaCardView - Grid display of schema nodes using NodeCard
 *
 * v11.6.1: Unified card-based schema browser using schemaStore.
 * Displays node types grouped by realm and layer with counts from current query.
 */

import { memo, useMemo, useState, useCallback } from 'react';
import {
  Search,
  ChevronDown,
  ChevronRight,
  Globe,
  Building2,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { Realm } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { iconSizes, gapTokens } from '@/design/tokens';
import { NodeCard, NodeCardGrid } from '@/components/ui/NodeCard';
import type { GraphNode, NodeType } from '@/types';
import { useSchemaStore, selectRealmGroups, selectIsSchemaLoaded } from '@/stores/schemaStore';

// Realm icons
const REALM_ICONS: Record<Realm, LucideIcon> = {
  shared: Globe,
  org: Building2,
};

export interface SchemaCardViewProps {
  className?: string;
  onNodeSelect?: (nodeType: NodeType) => void;
  selectedNodeType?: NodeType | null;
}

/**
 * Convert a node type to a pseudo GraphNode for display
 */
function nodeTypeToGraphNode(nodeType: NodeType): GraphNode {
  const config = NODE_TYPE_CONFIG[nodeType] || NODE_TYPE_CONFIG.Project;
  return {
    id: `schema-${nodeType}`,
    type: nodeType,
    key: nodeType,
    displayName: config.label,
    description: `${config.layer} layer node`,
  };
}

export const SchemaCardView = memo(function SchemaCardView({
  className,
  onNodeSelect,
  selectedNodeType,
}: SchemaCardViewProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [collapsedSections, setCollapsedSections] = useState<Set<string>>(new Set());

  // Get realm groups from schema store
  const { realmGroups, isSchemaLoaded } = useSchemaStore(
    useShallow((state) => ({
      realmGroups: selectRealmGroups(state),
      isSchemaLoaded: selectIsSchemaLoaded(state),
    }))
  );

  // Toggle section collapse
  const toggleSection = useCallback((key: string) => {
    setCollapsedSections((prev) => {
      const next = new Set(prev);
      if (next.has(key)) {
        next.delete(key);
      } else {
        next.add(key);
      }
      return next;
    });
  }, []);

  // Memoize realm data with filtered node types
  const realmData = useMemo(() => {
    const query = searchQuery.toLowerCase().trim();

    return realmGroups.map((realmGroup) => {
      // Filter layers and their node types by search query
      const filteredLayers = realmGroup.layers.map((layerGroup) => {
        const filteredTypes = query
          ? layerGroup.nodeTypes.filter((nodeType) => {
              const config = NODE_TYPE_CONFIG[nodeType.name];
              return (
                nodeType.name.toLowerCase().includes(query) ||
                config?.label?.toLowerCase().includes(query)
              );
            })
          : layerGroup.nodeTypes;

        return {
          name: layerGroup.layer,
          label: layerGroup.displayName,
          color: layerGroup.color,
          nodeTypes: filteredTypes,
          totalCount: filteredTypes.reduce((sum, t) => sum + t.count, 0),
        };
      }).filter((layer) => layer.nodeTypes.length > 0);

      const nodeCount = filteredLayers.reduce(
        (sum, layer) => sum + layer.nodeTypes.length,
        0
      );
      const totalCount = filteredLayers.reduce(
        (sum, layer) => sum + layer.totalCount,
        0
      );

      return {
        realm: realmGroup.realm,
        label: realmGroup.displayName,
        accent: { color: realmGroup.color },
        layers: filteredLayers,
        nodeCount,
        totalCount,
      };
    }).filter((realm) => realm.nodeCount > 0);
  }, [searchQuery, realmGroups]);

  return (
    <div className={cn('flex flex-col h-full', className)}>
      {/* Search Bar */}
      <div className="px-3 py-2.5 border-b border-white/[0.06]">
        <div className="relative">
          <Search className={cn('absolute left-3 top-1/2 -translate-y-1/2 text-white/30', iconSizes.sm)} />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search node types..."
            className="w-full pl-9 pr-3 py-2 bg-white/[0.04] border border-white/[0.08] rounded-lg text-sm text-white placeholder-white/30 outline-none focus:border-white/20 transition-colors"
          />
        </div>
      </div>

      {/* Card Grid */}
      <div className="flex-1 overflow-y-auto scrollbar-thin px-3 py-3">
        {realmData.length === 0 ? (
          <div className="flex items-center justify-center h-32 text-white/40 text-sm">
            No node types match your search
          </div>
        ) : (
          <div className={cn('flex flex-col', gapTokens.large)}>
            {realmData.map(({ realm, label, accent, layers }) => {
              const RealmIcon = REALM_ICONS[realm];
              const realmKey = `realm-${realm}`;
              const isCollapsed = collapsedSections.has(realmKey);

              return (
                <div key={realmKey} className="flex flex-col">
                  {/* Realm Header */}
                  <button
                    onClick={() => toggleSection(realmKey)}
                    className={cn(
                      'flex items-center px-2 py-2 rounded-lg transition-colors',
                      'hover:bg-white/[0.04]',
                      gapTokens.default
                    )}
                  >
                    {isCollapsed ? (
                      <ChevronRight className={iconSizes.sm} style={{ color: accent.color }} />
                    ) : (
                      <ChevronDown className={iconSizes.sm} style={{ color: accent.color }} />
                    )}
                    <RealmIcon className={iconSizes.md} style={{ color: accent.color }} />
                    <span
                      className="text-sm font-semibold"
                      style={{ color: accent.color }}
                    >
                      {label}
                    </span>
                  </button>

                  {/* Layers */}
                  {!isCollapsed && (
                    <div className={cn('flex flex-col pl-4', gapTokens.spacious)}>
                      {layers.map((layer) => {
                        const layerKey = `${realm}-${layer.name}`;
                        const isLayerCollapsed = collapsedSections.has(layerKey);

                        return (
                          <div key={layerKey} className="flex flex-col">
                            {/* Layer Header */}
                            <button
                              onClick={() => toggleSection(layerKey)}
                              className={cn(
                                'flex items-center px-2 py-1.5 rounded-lg transition-colors',
                                'hover:bg-white/[0.04] text-white/60',
                                gapTokens.compact
                              )}
                            >
                              {isLayerCollapsed ? (
                                <ChevronRight className={iconSizes.xs} />
                              ) : (
                                <ChevronDown className={iconSizes.xs} />
                              )}
                              <span className="text-xs font-medium">
                                {layer.label}
                              </span>
                              <span className="text-[10px] text-white/30 ml-auto">
                                {layer.nodeTypes.length}
                              </span>
                            </button>

                            {/* Node Cards Grid */}
                            {!isLayerCollapsed && (
                              <div className="pl-4 pt-2">
                                <NodeCardGrid columns={2}>
                                  {layer.nodeTypes.map((enrichedType) => {
                                    const graphNode = nodeTypeToGraphNode(enrichedType.name);
                                    return (
                                      <NodeCard
                                        key={enrichedType.name}
                                        node={graphNode}
                                        compact
                                        isSelected={selectedNodeType === enrichedType.name}
                                        onClick={() => onNodeSelect?.(enrichedType.name)}
                                        instanceCount={enrichedType.count}
                                        className={enrichedType.count === 0 ? 'opacity-50' : undefined}
                                      />
                                    );
                                  })}
                                </NodeCardGrid>
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
});
