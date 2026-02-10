'use client';

/**
 * SchemaCardView - Grid display of schema nodes using NodeCard
 *
 * v11.0: Unified card-based schema browser similar to TUI meta view.
 * Displays node types grouped by realm and layer in a card grid layout.
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
import { REALM_HIERARCHY } from '@novanet/core/graph';
import type { Layer } from '@novanet/core/graph';
import { Realm } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { iconSizes, gapTokens } from '@/design/tokens';
import { REALM_COLORS } from '@/design/colors';
import { NodeCard, NodeCardGrid } from '@/components/ui/NodeCard';
import type { GraphNode, NodeType } from '@/types';

// Realm icons
const REALM_ICONS: Record<Realm, LucideIcon> = {
  shared: Globe,
  org: Building2,
};

// Ordered realms for consistent rendering
const REALM_ORDER: Realm[] = ['shared', 'org'];

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

    return REALM_ORDER.map((realm) => {
      const realmDef = REALM_HIERARCHY[realm];
      const accent = REALM_COLORS[realm];
      const layers = Object.entries(realmDef.layers) as [
        Layer,
        (typeof realmDef.layers)[Layer],
      ][];

      // Filter layers and their node types by search query
      const filteredLayers = layers.map(([layerName, layerMeta]) => {
        const filteredTypes = query
          ? layerMeta.nodeTypes.filter((type) => {
              const config = NODE_TYPE_CONFIG[type];
              return (
                type.toLowerCase().includes(query) ||
                config?.label?.toLowerCase().includes(query)
              );
            })
          : layerMeta.nodeTypes;

        return {
          name: layerName,
          label: layerMeta.label,
          nodeTypes: filteredTypes,
        };
      }).filter((layer) => layer.nodeTypes.length > 0);

      const nodeCount = filteredLayers.reduce(
        (sum, layer) => sum + layer.nodeTypes.length,
        0
      );

      return {
        realm,
        label: realmDef.label,
        accent,
        layers: filteredLayers,
        nodeCount,
      };
    }).filter((realm) => realm.nodeCount > 0);
  }, [searchQuery]);

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
                <div key={realm} className="flex flex-col">
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
                          <div key={layer.name} className="flex flex-col">
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
                                  {layer.nodeTypes.map((nodeType) => {
                                    const graphNode = nodeTypeToGraphNode(nodeType as NodeType);
                                    return (
                                      <NodeCard
                                        key={nodeType}
                                        node={graphNode}
                                        compact
                                        isSelected={selectedNodeType === nodeType}
                                        onClick={() => onNodeSelect?.(nodeType as NodeType)}
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
