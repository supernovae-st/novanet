'use client';

/**
 * ResultsOverview - Compact node type badges + expandable breakdown
 *
 * Compact badge: Icon + Count. Hover → type name slides in.
 * +N hover: pill expands showing BOTH node + relation breakdowns.
 * "35 nodes" hover: nodes only. "89 relations" hover: relations only.
 */

import { useMemo, memo } from 'react';
import { ArrowRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { useFilteredGraph, type RealmCounts, type LayerCounts } from '@/hooks/useFilteredGraph';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { NODE_REALMS } from '@novanet/core/types';
import { NODE_LAYERS, type Layer } from '@novanet/core/graph';
import { relationshipTypeConfigs, type RelationshipCategory } from '@/config/relationshipTypes';
import { LayerIcon, ProgressBar } from '@/components/ui';
import { GRAPH_ICONS } from '@/config/iconSystem';
import type { NodeType } from '@/types';

export type ExpandedViewType = 'nodes' | 'relations' | 'all' | null;

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Types                                                                     */
/* ═══════════════════════════════════════════════════════════════════════════ */

interface TypeCountItem {
  type: string;
  count: number;
  color: string;
  category: Layer | RelationshipCategory;
}

/** Realm config for breakdown display - colors match Neo4j organizing-principles */
const REALM_CONFIG = {
  global: { emoji: '🌍', color: '#2aa198' },   // solarized cyan
  project: { emoji: '📦', color: '#6c71c4' },  // solarized violet
  shared: { emoji: '🎯', color: '#cb4b16' },   // solarized orange
} as const;

/** Layer config - color inherited from parent realm */
const LAYER_CONFIG: Record<Layer, { emoji: string; realm: keyof typeof REALM_CONFIG }> = {
  // Project realm (5 layers)
  foundation: { emoji: '🏛️', realm: 'project' },
  structure: { emoji: '🧱', realm: 'project' },
  semantic: { emoji: '💡', realm: 'project' },
  instruction: { emoji: '📋', realm: 'project' },
  output: { emoji: '📤', realm: 'project' },
  // Global realm (2 layers)
  config: { emoji: '⚙️', realm: 'global' },
  knowledge: { emoji: '🧠', realm: 'global' },
  // Shared realm (2 layers)
  seo: { emoji: '🔍', realm: 'shared' },
  geo: { emoji: '🌐', realm: 'shared' },
} as const;

interface ResultsOverviewProps {
  className?: string;
  maxTypes?: number;
  onHoverOverflow?: () => void;
  onHoverLeave?: () => void;
  expandedView?: ExpandedViewType;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Compact badge (top row): Icon + Count, hover → name slides in            */
/* ═══════════════════════════════════════════════════════════════════════════ */

function TypeBadge({ item }: { item: TypeCountItem }) {
  return (
    <span
      className={cn(
        'group/badge flex items-center gap-1.5 pl-1.5 pr-2 py-1 rounded-lg',
        'whitespace-nowrap cursor-default',
        'bg-white/[0.05] border border-white/[0.06]',
        'hover:bg-white/[0.10] hover:border-white/[0.15]',
        'transition-all duration-150'
      )}
      title={`${item.type}: ${item.count} nodes`}
    >
      <LayerIcon
        layer={item.category as Layer}
        size={13}
        strokeWidth={2.5}
        className="shrink-0 transition-transform duration-150 group-hover/badge:scale-110"
        style={{
          color: item.color,
          filter: `drop-shadow(0 0 4px ${item.color}60)`,
        }}
      />
      {/* Type name - slides in on hover */}
      <span className={cn(
        'text-[11px] text-white/50 overflow-hidden',
        'max-w-0 group-hover/badge:max-w-[120px]',
        'transition-all duration-150 ease-out'
      )}>
        {item.type}
      </span>
      <span
        className="text-sm font-bold tabular-nums"
        style={{
          color: item.color,
          textShadow: `0 0 8px ${item.color}40`,
        }}
      >
        {item.count}
      </span>
    </span>
  );
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Node type pill: Icon + Name + Count (tiny, inline)                        */
/* ═══════════════════════════════════════════════════════════════════════════ */

function NodeTypePill({ item, index }: { item: TypeCountItem; index: number }) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1 px-2 py-0.5 rounded-full h-7',
        'bg-white/[0.04]',
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{ animationDelay: `${index * 15}ms` }}
    >
      <LayerIcon
        layer={item.category as Layer}
        size={11}
        strokeWidth={2.5}
        className="shrink-0"
        style={{ color: item.color }}
      />
      <span className="text-[11px] text-white/50 truncate max-w-[80px]">{item.type}</span>
      <span
        className="text-[10px] font-semibold tabular-nums"
        style={{ color: item.color }}
      >
        {item.count}
      </span>
    </span>
  );
}

function ExpandedRelationBadge({ item, index, maxCount }: { item: TypeCountItem; index: number; maxCount: number }) {
  return (
    <div
      className={cn(
        'flex items-center gap-2 px-2.5 py-1.5 rounded-lg min-w-[200px]',
        'bg-white/[0.04] border border-white/[0.06]',
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{ animationDelay: `${index * 20}ms` }}
    >
      <ArrowRight
        size={12}
        strokeWidth={2.5}
        className="shrink-0"
        style={{
          color: item.color,
          filter: `drop-shadow(0 0 3px ${item.color}50)`,
        }}
      />
      <span className="text-[11px] text-white/50 w-32 truncate">{item.type}</span>
      <ProgressBar value={item.count} max={maxCount} color={item.color} size="sm" className="w-12" />
      <span
        className="text-xs font-semibold tabular-nums min-w-[28px] text-right"
        style={{ color: item.color, textShadow: `0 0 6px ${item.color}30` }}
      >
        {item.count}
      </span>
    </div>
  );
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Realm hero card (schema mode only)                                        */
/* ═══════════════════════════════════════════════════════════════════════════ */

function RealmHeroCard({ scope, typeCount, loadedCount, maxLoaded, index }: {
  scope: keyof typeof REALM_CONFIG;
  typeCount: number;
  loadedCount: number;
  maxLoaded: number;
  index: number;
}) {
  const config = REALM_CONFIG[scope];
  return (
    <div
      className={cn(
        'flex items-center gap-4 rounded-xl h-20',
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{
        animationDelay: `${index * 40}ms`,
        background: `linear-gradient(135deg, ${config.color}15, ${config.color}08)`,
        borderWidth: '1px 1px 1px 4px',
        borderStyle: 'solid',
        borderColor: `${config.color}25`,
        borderLeftColor: config.color,
        boxShadow: `0 0 30px ${config.color}15, 0 4px 12px rgba(0,0,0,0.3)`,
        paddingLeft: '16px',
        paddingRight: '16px',
      }}
    >
      <span className="text-3xl shrink-0">{config.emoji}</span>
      <div className="flex-1 min-w-0">
        <span className="text-sm font-bold block" style={{ color: config.color }}>
          {scope}
        </span>
        <span className="text-xs text-white/50 block mt-0.5">
          {typeCount} types &middot; {loadedCount} loaded
        </span>
        <ProgressBar
          value={loadedCount}
          max={maxLoaded}
          color={config.color}
          size="default"
          className="mt-1.5"
        />
      </div>
    </div>
  );
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Layer card (schema mode only)                                       */
/* ═══════════════════════════════════════════════════════════════════════════ */

function LayerCard({ layer, typeCount, loadedCount, maxLoaded, index }: {
  layer: Layer;
  typeCount: number;
  loadedCount: number;
  maxLoaded: number;
  index: number;
}) {
  const config = LAYER_CONFIG[layer];
  const realmColor = REALM_CONFIG[config.realm].color;
  const displayName = layer.charAt(0).toUpperCase() + layer.slice(1);
  return (
    <div
      className={cn(
        'flex items-center gap-2.5 rounded-lg h-14',
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{
        animationDelay: `${index * 30}ms`,
        background: `${realmColor}08`,
        borderWidth: '2px 1px 1px 1px',
        borderStyle: 'solid',
        borderColor: `${realmColor}15`,
        borderTopColor: `${realmColor}80`,
        paddingLeft: '12px',
        paddingRight: '12px',
      }}
    >
      <span className="text-xl shrink-0">{config.emoji}</span>
      <div className="flex-1 min-w-0">
        <span className="text-xs font-semibold block" style={{ color: realmColor }}>
          {displayName}
        </span>
        <div className="flex items-center gap-2 mt-0.5">
          <span className="text-[10px] text-white/40 shrink-0">
            {typeCount} types &middot; {loadedCount}
          </span>
          <ProgressBar value={loadedCount} max={maxLoaded} color={realmColor} size="sm" className="w-12" />
        </div>
      </div>
    </div>
  );
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Section header with icon                                                  */
/* ═══════════════════════════════════════════════════════════════════════════ */

function SectionHeader({ icon: Icon, label, count }: {
  icon: React.ComponentType<{ size?: number; className?: string }>;
  label: string;
  count: number;
}) {
  return (
    <div className="flex items-center gap-2 mb-2 px-0.5">
      <Icon size={12} className="text-white/25" />
      <span className="text-[10px] font-medium uppercase tracking-wider text-white/30">
        {count} {label}
      </span>
    </div>
  );
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Expanded breakdown area (rendered inside Pill)                            */
/* ═══════════════════════════════════════════════════════════════════════════ */

export const ExpandedBreakdown = memo(function ExpandedBreakdown({
  view,
  onMouseEnter,
  onMouseLeave,
}: {
  view: ExpandedViewType;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
}) {
  const { nodes, edges, isSchemaMode, realmCounts, layerCounts } = useFilteredGraph();

  const nodeTypeCounts = useMemo(() => {
    const counts = new Map<string, number>();
    for (const node of nodes) {
      counts.set(node.type, (counts.get(node.type) || 0) + 1);
    }
    return Array.from(counts.entries())
      .sort((a, b) => b[1] - a[1])
      .map(([type, count]): TypeCountItem => {
        const config = NODE_TYPE_CONFIG[type as NodeType] || {
          color: '#6b7280',
          layer: 'foundation' as Layer,
        };
        return { type, count, color: config.color, category: config.layer };
      });
  }, [nodes]);

  const relationTypeCounts = useMemo(() => {
    const counts = new Map<string, number>();
    for (const edge of edges) {
      counts.set(edge.type, (counts.get(edge.type) || 0) + 1);
    }
    return Array.from(counts.entries())
      .sort((a, b) => b[1] - a[1])
      .map(([type, count]): TypeCountItem => {
        const config = relationshipTypeConfigs[type as keyof typeof relationshipTypeConfigs];
        return {
          type,
          count,
          color: config?.color || '#6b7280',
          category: config?.category || 'ownership',
        };
      });
  }, [edges]);

  // Compute max counts for progress bars (relative to max)
  const maxRelCount = relationTypeCounts.length > 0 ? relationTypeCounts[0].count : 1;

  // Type counts per scope (static: how many distinct node types belong to each scope)
  const scopeTypeCounts = useMemo(() => {
    const counts: Record<string, number> = { global: 0, project: 0, shared: 0 };
    for (const scope of Object.values(NODE_REALMS)) {
      if (scope in counts) counts[scope]++;
    }
    return counts;
  }, []);

  // Type counts per subcategory (static: how many distinct node types per subcategory)
  const layerTypeCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const subcat of Object.values(NODE_LAYERS)) {
      counts[subcat] = (counts[subcat] || 0) + 1;
    }
    return counts;
  }, []);

  // Max loaded counts for progress bar scaling
  const maxRealmLoaded = Math.max(realmCounts.global, realmCounts.project, realmCounts.shared, 1);
  const maxLayerLoaded = Math.max(...Object.values(layerCounts), 1);

  const showNodes = view === 'nodes' || view === 'all';
  const showRelations = view === 'relations' || view === 'all';
  const NodeIcon = GRAPH_ICONS.node;
  const RelIcon = GRAPH_ICONS.relationship;

  return (
    <div
      className="grid transition-[grid-template-rows] duration-200 ease-out"
      style={{ gridTemplateRows: view ? '1fr' : '0fr' }}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
    >
      <div className="overflow-hidden">
        {view && (
          <div className="pt-2.5 pb-1 max-h-[60vh] overflow-y-auto scrollbar-thin">
            {/* Subtle separator */}
            <div className="h-px bg-white/[0.06] mb-2.5" />

            {/* Schema mode: 3-tier visual hierarchy */}
            {isSchemaMode && showNodes && (
              <div className={showRelations ? 'mb-3' : ''}>
                {/* Level 1: Realm Hero Cards - full-width, stacked */}
                <SectionHeader icon={NodeIcon} label="realms" count={3} />
                <div className="flex flex-col gap-2 mb-3">
                  <RealmHeroCard scope="global" typeCount={scopeTypeCounts.global} loadedCount={realmCounts.global} maxLoaded={maxRealmLoaded} index={0} />
                  <RealmHeroCard scope="project" typeCount={scopeTypeCounts.project} loadedCount={realmCounts.project} maxLoaded={maxRealmLoaded} index={1} />
                  <RealmHeroCard scope="shared" typeCount={scopeTypeCounts.shared} loadedCount={realmCounts.shared} maxLoaded={maxRealmLoaded} index={2} />
                </div>

                {/* Level 2: Layer Cards - 2-col grid */}
                <div className="h-px bg-white/[0.04] mb-2.5" />
                <SectionHeader icon={NodeIcon} label="layers" count={9} />
                <div className="grid grid-cols-2 gap-1.5 mb-3">
                  {(['config', 'knowledge', 'foundation', 'structure', 'semantic', 'instruction', 'output', 'seo', 'geo'] as const).map((subcat, i) => (
                    <LayerCard
                      key={subcat}
                      layer={subcat}
                      typeCount={layerTypeCounts[subcat] || 0}
                      loadedCount={layerCounts[subcat]}
                      maxLoaded={maxLayerLoaded}
                      index={i}
                    />
                  ))}
                </div>

                {/* Level 3: Node Type Pills - inline flex-wrap */}
                <div className="h-px bg-white/[0.04] mb-2.5" />
                <SectionHeader icon={NodeIcon} label="node types" count={nodeTypeCounts.length} />
                <div className="flex flex-wrap gap-1.5">
                  {nodeTypeCounts.map((item, i) => (
                    <NodeTypePill key={item.type} item={item} index={i} />
                  ))}
                </div>
              </div>
            )}

            {/* Data mode: node type pills */}
            {!isSchemaMode && showNodes && (
              <div className={showRelations ? 'mb-3' : ''}>
                <SectionHeader icon={NodeIcon} label="node types" count={nodeTypeCounts.length} />
                <div className="flex flex-wrap gap-1.5">
                  {nodeTypeCounts.map((item, i) => (
                    <NodeTypePill key={item.type} item={item} index={i} />
                  ))}
                </div>
              </div>
            )}

            {/* Relations section (both modes) */}
            {showRelations && (
              <div>
                {showNodes && <div className="h-px bg-white/[0.04] mb-2.5" />}
                <SectionHeader icon={RelIcon} label="relation types" count={relationTypeCounts.length} />
                <div className="flex flex-wrap gap-1.5">
                  {relationTypeCounts.map((item, i) => (
                    <ExpandedRelationBadge
                      key={item.type}
                      item={item}
                      index={showNodes ? i + nodeTypeCounts.length : i}
                      maxCount={maxRelCount}
                    />
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
});

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  Main compact badges row                                                   */
/* ═══════════════════════════════════════════════════════════════════════════ */

export const ResultsOverview = memo(function ResultsOverview({
  className,
  maxTypes = 6,
  onHoverOverflow,
  onHoverLeave,
  expandedView,
}: ResultsOverviewProps) {
  const { nodes, visibleNodeCount: totalNodes } = useFilteredGraph();

  const { visibleCounts, overflowCount } = useMemo(() => {
    const counts = new Map<NodeType, number>();
    for (const node of nodes) {
      counts.set(node.type, (counts.get(node.type) || 0) + 1);
    }

    const allSorted = Array.from(counts.entries())
      .sort((a, b) => b[1] - a[1])
      .map(([type, count]) => {
        const config = NODE_TYPE_CONFIG[type] || {
          color: '#6b7280',
          layer: 'foundation' as Layer,
        };
        return { type, count, color: config.color, category: config.layer };
      });

    return {
      visibleCounts: allSorted.slice(0, maxTypes),
      overflowCount: Math.max(0, counts.size - maxTypes),
    };
  }, [nodes, maxTypes]);

  if (totalNodes === 0) {
    return (
      <div className={cn('flex items-center text-white/40 text-xs', gapTokens.default, className)}>
        <span>No results</span>
      </div>
    );
  }

  return (
    <div className={cn('flex items-center', gapTokens.compact, className)}>
      {visibleCounts.map((item) => (
        <TypeBadge key={item.type} item={item} />
      ))}

      {overflowCount > 0 && (
        <span
          className={cn(
            'flex items-center px-2 py-1 rounded-lg cursor-default',
            'text-[11px] font-medium tabular-nums',
            'transition-all duration-150',
            expandedView === 'all'
              ? 'text-white/60 bg-white/[0.08]'
              : 'text-white/30 hover:text-white/60 hover:bg-white/[0.08]'
          )}
          onMouseEnter={onHoverOverflow}
          onMouseLeave={onHoverLeave}
        >
          +{overflowCount}
        </span>
      )}
    </div>
  );
});
