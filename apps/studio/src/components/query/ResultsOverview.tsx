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
import { useFilteredGraph, type ScopeCounts, type SubcategoryCounts } from '@/hooks/useFilteredGraph';
import { NODE_TYPE_CONFIG, type NodeCategory } from '@/config/nodeTypes';
import { NODE_SCOPES } from '@novanet/core/types';
import { NODE_SUBCATEGORIES, type Subcategory } from '@novanet/core/graph';
import { relationshipTypeConfigs, type RelationshipCategory } from '@/config/relationshipTypes';
import { CategoryIcon, ProgressBar } from '@/components/ui';
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
  category: NodeCategory | RelationshipCategory;
}

/** Scope config for breakdown display - colors match Neo4j organizing-principles */
const SCOPE_CONFIG = {
  Global: { emoji: '🌍', color: '#2aa198' },   // solarized cyan
  Project: { emoji: '📦', color: '#6c71c4' },  // solarized violet
  Shared: { emoji: '🎯', color: '#cb4b16' },   // solarized orange
} as const;

/** Subcategory config - color inherited from parent scope */
const SUBCATEGORY_CONFIG: Record<Subcategory, { emoji: string; scope: keyof typeof SCOPE_CONFIG }> = {
  // Project scope (5 subcategories)
  foundation: { emoji: '🏛️', scope: 'Project' },
  structure: { emoji: '🧱', scope: 'Project' },
  semantic: { emoji: '💡', scope: 'Project' },
  instruction: { emoji: '📋', scope: 'Project' },
  output: { emoji: '📤', scope: 'Project' },
  // Global scope (2 subcategories)
  config: { emoji: '⚙️', scope: 'Global' },
  knowledge: { emoji: '🧠', scope: 'Global' },
  // Shared scope (2 subcategories)
  seo: { emoji: '🔍', scope: 'Shared' },
  geo: { emoji: '🌐', scope: 'Shared' },
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
      <CategoryIcon
        category={item.category as NodeCategory}
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
      <CategoryIcon
        category={item.category as NodeCategory}
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
/*  Scope hero card (schema mode only)                                        */
/* ═══════════════════════════════════════════════════════════════════════════ */

function ScopeHeroCard({ scope, typeCount, loadedCount, maxLoaded, index }: {
  scope: keyof typeof SCOPE_CONFIG;
  typeCount: number;
  loadedCount: number;
  maxLoaded: number;
  index: number;
}) {
  const config = SCOPE_CONFIG[scope];
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
/*  Subcategory card (schema mode only)                                       */
/* ═══════════════════════════════════════════════════════════════════════════ */

function SubcategoryCard({ subcategory, typeCount, loadedCount, maxLoaded, index }: {
  subcategory: Subcategory;
  typeCount: number;
  loadedCount: number;
  maxLoaded: number;
  index: number;
}) {
  const config = SUBCATEGORY_CONFIG[subcategory];
  const scopeColor = SCOPE_CONFIG[config.scope].color;
  const displayName = subcategory.charAt(0).toUpperCase() + subcategory.slice(1);
  return (
    <div
      className={cn(
        'flex items-center gap-2.5 rounded-lg h-14',
        'animate-[badgeIn_200ms_ease-out_both]'
      )}
      style={{
        animationDelay: `${index * 30}ms`,
        background: `${scopeColor}08`,
        borderWidth: '2px 1px 1px 1px',
        borderStyle: 'solid',
        borderColor: `${scopeColor}15`,
        borderTopColor: `${scopeColor}80`,
        paddingLeft: '12px',
        paddingRight: '12px',
      }}
    >
      <span className="text-xl shrink-0">{config.emoji}</span>
      <div className="flex-1 min-w-0">
        <span className="text-xs font-semibold block" style={{ color: scopeColor }}>
          {displayName}
        </span>
        <div className="flex items-center gap-2 mt-0.5">
          <span className="text-[10px] text-white/40 shrink-0">
            {typeCount} types &middot; {loadedCount}
          </span>
          <ProgressBar value={loadedCount} max={maxLoaded} color={scopeColor} size="sm" className="w-12" />
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
  const { nodes, edges, isSchemaMode, scopeCounts, subcategoryCounts } = useFilteredGraph();

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
          category: 'project' as NodeCategory,
        };
        return { type, count, color: config.color, category: config.category };
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
    const counts: Record<string, number> = { Global: 0, Project: 0, Shared: 0 };
    for (const scope of Object.values(NODE_SCOPES)) {
      if (scope in counts) counts[scope]++;
    }
    return counts;
  }, []);

  // Type counts per subcategory (static: how many distinct node types per subcategory)
  const subcategoryTypeCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const subcat of Object.values(NODE_SUBCATEGORIES)) {
      counts[subcat] = (counts[subcat] || 0) + 1;
    }
    return counts;
  }, []);

  // Max loaded counts for progress bar scaling
  const maxScopeLoaded = Math.max(scopeCounts.Global, scopeCounts.Project, scopeCounts.Shared, 1);
  const maxSubcategoryLoaded = Math.max(...Object.values(subcategoryCounts), 1);

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
                {/* Level 1: Scope Hero Cards - full-width, stacked */}
                <SectionHeader icon={NodeIcon} label="scopes" count={3} />
                <div className="flex flex-col gap-2 mb-3">
                  <ScopeHeroCard scope="Global" typeCount={scopeTypeCounts.Global} loadedCount={scopeCounts.Global} maxLoaded={maxScopeLoaded} index={0} />
                  <ScopeHeroCard scope="Project" typeCount={scopeTypeCounts.Project} loadedCount={scopeCounts.Project} maxLoaded={maxScopeLoaded} index={1} />
                  <ScopeHeroCard scope="Shared" typeCount={scopeTypeCounts.Shared} loadedCount={scopeCounts.Shared} maxLoaded={maxScopeLoaded} index={2} />
                </div>

                {/* Level 2: Subcategory Cards - 2-col grid */}
                <div className="h-px bg-white/[0.04] mb-2.5" />
                <SectionHeader icon={NodeIcon} label="subcategories" count={9} />
                <div className="grid grid-cols-2 gap-1.5 mb-3">
                  {(['config', 'knowledge', 'foundation', 'structure', 'semantic', 'instruction', 'output', 'seo', 'geo'] as const).map((subcat, i) => (
                    <SubcategoryCard
                      key={subcat}
                      subcategory={subcat}
                      typeCount={subcategoryTypeCounts[subcat] || 0}
                      loadedCount={subcategoryCounts[subcat]}
                      maxLoaded={maxSubcategoryLoaded}
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
          category: 'project' as NodeCategory,
        };
        return { type, count, color: config.color, category: config.category };
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
