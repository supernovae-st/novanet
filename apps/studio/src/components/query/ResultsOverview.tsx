'use client';

/**
 * ResultsOverview - Compact node type badges + expandable breakdown
 *
 * Compact badge: Icon + Count. Hover → type name slides in.
 * +N hover: pill expands showing BOTH node + relation breakdowns.
 * "46 nodes" hover: nodes only. "89 relations" hover: relations only.
 */

import { useMemo, memo, useCallback } from 'react';
import { ArrowRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { useFilteredGraph } from '@/hooks/useFilteredGraph';
import { useFilterStore } from '@/stores/filterStore';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import type { Layer } from '@novanet/core/graph';
import { relationshipTypeConfigs, type RelationshipCategory } from '@/config/relationshipTypes';
import { LayerIcon } from '@/components/ui';
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

/** Realm config for breakdown display - colors match Neo4j organizing-principles (v10.6: 2 realms) */
const REALM_CONFIG = {
  global: { emoji: '🌍', color: '#2aa198' },   // solarized cyan
  tenant: { emoji: '🏢', color: '#0ea5e9' },   // sky blue
} as const;

/** Layer config - color inherited from parent realm (v10.6: 2 realms) */
const LAYER_CONFIG: Record<Layer, { emoji: string; realm: keyof typeof REALM_CONFIG }> = {
  // Tenant realm (5 layers)
  foundation: { emoji: '🏛️', realm: 'tenant' },
  structure: { emoji: '🧱', realm: 'tenant' },
  semantic: { emoji: '💡', realm: 'tenant' },
  instruction: { emoji: '📋', realm: 'tenant' },
  output: { emoji: '📤', realm: 'tenant' },
  // Global realm (3 layers) — v10.6: locale-knowledge
  config: { emoji: '⚙️', realm: 'global' },
  'locale-knowledge': { emoji: '🧠', realm: 'global' },
  seo: { emoji: '🔍', realm: 'global' },
} as const;

interface ResultsOverviewProps {
  className?: string;
  maxTypes?: number;
  onHoverOverflow?: () => void;
  onHoverLeave?: () => void;
  expandedView?: ExpandedViewType;
}

/* ═══════════════════════════════════════════════════════════════════════════ */
/*  UnifiedBadge - Single consistent style for all badges                     */
/*                                                                            */
/*  Variants:                                                                 */
/*  - node: LayerIcon (●) + label + count                                     */
/*  - relation: ArrowRight (→) + label + count                                */
/*  - realm: emoji + label + count                                            */
/*  - layer: emoji + label + count                                            */
/*                                                                            */
/*  Consistent dimensions: h-7, px-2.5, rounded-lg, border-white/10           */
/* ═══════════════════════════════════════════════════════════════════════════ */

interface UnifiedBadgeProps {
  item: TypeCountItem;
  variant: 'node' | 'relation' | 'realm' | 'layer';
  index?: number;
  onClick?: () => void;
  isEnabled?: boolean;
  showLabel?: boolean;
  /** Emoji for realm/layer variants */
  emoji?: string;
}

function UnifiedBadge({
  item,
  variant,
  index = 0,
  onClick,
  isEnabled = true,
  showLabel = true,
  emoji,
}: UnifiedBadgeProps) {
  const isClickable = !!onClick;
  const Component = isClickable ? 'button' : 'span';

  // Get title based on variant
  const getTitle = () => {
    const clickHint = isClickable ? ` (click to ${isEnabled ? 'hide' : 'show'})` : '';
    switch (variant) {
      case 'node': return `${item.type}: ${item.count} nodes${clickHint}`;
      case 'relation': return `${item.type}: ${item.count} relationships`;
      case 'realm': return `${item.type}: ${item.count} loaded${clickHint}`;
      case 'layer': return `${item.type}: ${item.count} loaded${clickHint}`;
    }
  };

  return (
    <Component
      type={isClickable ? 'button' : undefined}
      onClick={onClick}
      className={cn(
        // Unified dimensions
        'inline-flex items-center gap-1.5 h-7 px-2.5 rounded-lg',
        'whitespace-nowrap',
        // Border and background
        'border transition-all duration-150',
        isEnabled
          ? 'bg-white/[0.03] border-white/10 hover:bg-white/[0.06] hover:border-white/[0.15]'
          : 'bg-white/[0.02] border-white/[0.06] opacity-50 hover:opacity-80',
        // Cursor
        isClickable ? 'cursor-pointer' : 'cursor-default',
        // Animation
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{ animationDelay: `${index * 15}ms` }}
      title={getTitle()}
    >
      {/* Icon - varies by variant */}
      {variant === 'node' && (
        <LayerIcon
          layer={item.category as Layer}
          size={12}
          strokeWidth={2.5}
          className="shrink-0"
          style={{
            color: item.color,
            filter: isEnabled ? `drop-shadow(0 0 3px ${item.color}50)` : 'none',
          }}
        />
      )}
      {variant === 'relation' && (
        <ArrowRight
          size={12}
          strokeWidth={2.5}
          className="shrink-0"
          style={{
            color: item.color,
            filter: `drop-shadow(0 0 3px ${item.color}50)`,
          }}
        />
      )}
      {(variant === 'realm' || variant === 'layer') && emoji && (
        <span className="text-xs shrink-0">{emoji}</span>
      )}

      {/* Label */}
      {showLabel && (
        <span className={cn(
          'text-[11px] truncate max-w-[72px]',
          isEnabled ? 'text-white/50' : 'text-white/30'
        )}>
          {item.type}
        </span>
      )}

      {/* Count */}
      <span
        className="text-[11px] font-semibold tabular-nums"
        style={{
          color: isEnabled ? item.color : `${item.color}80`,
          textShadow: isEnabled ? `0 0 6px ${item.color}30` : 'none',
        }}
      >
        {item.count}
      </span>
    </Component>
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
  const { nodes, edges, isMetaMode, realmCounts, layerCounts } = useFilteredGraph();

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

            {/* Schema mode: unified badge hierarchy */}
            {isMetaMode && showNodes && (
              <div className={showRelations ? 'mb-3' : ''}>
                {/* Realms - inline badges (v10.6: 2 realms) */}
                <SectionHeader icon={NodeIcon} label="realms" count={2} />
                <div className="flex flex-wrap gap-1.5 mb-3">
                  {(['global', 'tenant'] as const).map((scope, i) => (
                    <UnifiedBadge
                      key={scope}
                      item={{
                        type: scope,
                        count: realmCounts[scope],
                        color: REALM_CONFIG[scope].color,
                        category: 'foundation' as Layer,
                      }}
                      variant="realm"
                      emoji={REALM_CONFIG[scope].emoji}
                      index={i}
                    />
                  ))}
                </div>

                {/* Layers - inline badges (v10.6: 8 layers) */}
                <div className="h-px bg-white/[0.04] mb-2.5" />
                <SectionHeader icon={NodeIcon} label="layers" count={8} />
                <div className="flex flex-wrap gap-1.5 mb-3">
                  {(['config', 'locale-knowledge', 'foundation', 'structure', 'semantic', 'instruction', 'output', 'seo'] as const).map((layer, i) => {
                    const config = LAYER_CONFIG[layer];
                    const realmColor = REALM_CONFIG[config.realm].color;
                    return (
                      <UnifiedBadge
                        key={layer}
                        item={{
                          type: layer.charAt(0).toUpperCase() + layer.slice(1),
                          count: layerCounts[layer],
                          color: realmColor,
                          category: layer,
                        }}
                        variant="layer"
                        emoji={config.emoji}
                        index={i + 2}
                      />
                    );
                  })}
                </div>

                {/* Node Types - inline badges */}
                <div className="h-px bg-white/[0.04] mb-2.5" />
                <SectionHeader icon={NodeIcon} label="node types" count={nodeTypeCounts.length} />
                <div className="flex flex-wrap gap-1.5">
                  {nodeTypeCounts.map((item, i) => (
                    <UnifiedBadge key={item.type} item={item} variant="node" index={i + 12} />
                  ))}
                </div>
              </div>
            )}

            {/* Data mode: node type badges */}
            {!isMetaMode && showNodes && (
              <div className={showRelations ? 'mb-3' : ''}>
                <SectionHeader icon={NodeIcon} label="node types" count={nodeTypeCounts.length} />
                <div className="flex flex-wrap gap-1.5">
                  {nodeTypeCounts.map((item, i) => (
                    <UnifiedBadge key={item.type} item={item} variant="node" index={i} />
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
                    <UnifiedBadge
                      key={item.type}
                      item={item}
                      variant="relation"
                      index={showNodes ? i + nodeTypeCounts.length : i}
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
  const toggleNodeType = useFilterStore((s) => s.toggleNodeType);
  const enabledNodeTypes = useFilterStore((s) => s.enabledNodeTypes);

  const handleToggleType = useCallback((type: NodeType) => {
    toggleNodeType(type);
  }, [toggleNodeType]);

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
      {visibleCounts.map((item, i) => (
        <UnifiedBadge
          key={item.type}
          item={item}
          variant="node"
          index={i}
          onClick={() => handleToggleType(item.type as NodeType)}
          isEnabled={enabledNodeTypes.has(item.type as NodeType)}
          showLabel={false}
        />
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
