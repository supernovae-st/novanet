'use client';

/**
 * TabbedDetailPanel - Premium detail panel with glassmorphism design
 *
 * Features:
 * - 4 tabs: Overview, Data, Graph, Code
 * - Premium tab bar with animated underline indicator
 * - Glassmorphism panel background with subtle noise texture
 * - Motion animations for tab transitions
 * - Synced with uiStore for tab persistence
 *
 * Design System:
 * - Uses glass.surface hierarchy (0-4)
 * - Animated gradient underline on active tab
 * - Subtle inner glow on panel container
 * - Keyboard shortcuts displayed in tab tooltips
 */

import { memo, useMemo } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useShallow } from 'zustand/react/shallow';
import { Eye, Database, GitBranch, Code } from 'lucide-react';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLayerPalette } from '@/design/colors/palette';
import { useUIStore, type DetailPanelTab } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
// panelClasses, glassClasses reserved for future use
import { /* panelClasses, */ glass /* , glassClasses */ } from '@/design/tokens';
import { ElementIdentityCard } from '@/components/ui/detail-panel';
import { OverviewTab } from './tabs/OverviewTab';
import { DataTab } from './tabs/DataTab';
import { GraphTab } from './tabs/GraphTab';
import { CodeTab } from './tabs/CodeTab';
import type { GraphNode } from '@/types';

interface TabbedDetailPanelProps {
  node: GraphNode | null;
  className?: string;
}

interface TabConfig {
  id: DetailPanelTab;
  label: string;
  shortcut: string;
  icon: React.ComponentType<{ className?: string }>;
}

const TABS: TabConfig[] = [
  { id: 'overview', label: 'Overview', shortcut: '1', icon: Eye },
  { id: 'data', label: 'Data', shortcut: '2', icon: Database },
  { id: 'graph', label: 'Graph', shortcut: '3', icon: GitBranch },
  { id: 'code', label: 'Code', shortcut: '4', icon: Code },
];

/**
 * Premium tab bar with animated underline indicator and icons
 */
const TabBar = memo(function TabBar({
  activeTab,
  onTabChange,
  colors,
}: {
  activeTab: DetailPanelTab;
  onTabChange: (tab: DetailPanelTab) => void;
  colors: { primary: string; secondary: string };
}) {
  return (
    <div className="relative">
      {/* Tab container with glass effect */}
      <div
        className="flex items-center px-1 py-1"
        style={{
          background: `linear-gradient(180deg, ${glass.surface[1]}, ${glass.surface[0]})`,
          borderBottom: `1px solid ${glass.border.subtle}`,
        }}
      >
        {TABS.map((tab) => {
          const isActive = tab.id === activeTab;
          const Icon = tab.icon;

          return (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className={cn(
                'group relative flex-1 flex items-center justify-center gap-2',
                'px-3 py-2.5 rounded-lg text-xs font-medium',
                'transition-all duration-200 ease-out',
                'focus:outline-none focus-visible:ring-2 focus-visible:ring-white/20 focus-visible:ring-offset-1 focus-visible:ring-offset-transparent',
                isActive
                  ? 'text-white'
                  : 'text-white/40 hover:text-white/70 hover:bg-white/[0.04]'
              )}
              title={`${tab.label} (${tab.shortcut})`}
              aria-selected={isActive}
              role="tab"
            >
              {/* Icon */}
              <Icon
                className={cn(
                  'w-3.5 h-3.5 transition-all duration-200',
                  isActive ? 'opacity-100' : 'opacity-50 group-hover:opacity-70'
                )}
              />

              {/* Label */}
              <span className="hidden sm:inline">{tab.label}</span>

              {/* Keyboard shortcut badge (visible on hover) */}
              <span
                className={cn(
                  'absolute -top-1 -right-1 px-1 py-0.5 rounded text-[9px] font-mono',
                  'bg-white/10 text-white/40 opacity-0 group-hover:opacity-100',
                  'transition-opacity duration-150 pointer-events-none'
                )}
              >
                {tab.shortcut}
              </span>

              {/* Active indicator - animated gradient underline */}
              {isActive && (
                <motion.div
                  layoutId="activeTabIndicator"
                  className="absolute bottom-0 left-2 right-2 h-[2px] rounded-full"
                  style={{
                    background: `linear-gradient(90deg, ${colors.primary}, ${colors.secondary})`,
                    boxShadow: `0 0 8px ${colors.primary}60, 0 0 16px ${colors.primary}30`,
                  }}
                  transition={{
                    type: 'spring',
                    stiffness: 400,
                    damping: 30,
                  }}
                />
              )}
            </button>
          );
        })}
      </div>

      {/* Subtle separator glow */}
      <div
        className="absolute bottom-0 left-0 right-0 h-px"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}20, transparent)`,
        }}
      />
    </div>
  );
});

/**
 * Empty state when no node is selected - premium design
 */
function EmptyState() {
  return (
    <div className="h-full flex flex-col items-center justify-center p-8 text-center">
      {/* Animated rings */}
      <div className="relative w-20 h-20 mb-6">
        <motion.div
          className="absolute inset-0 rounded-full border border-white/10"
          animate={{ scale: [1, 1.2, 1], opacity: [0.3, 0.1, 0.3] }}
          transition={{ duration: 3, repeat: Infinity, ease: 'easeInOut' }}
        />
        <motion.div
          className="absolute inset-2 rounded-full border border-white/15"
          animate={{ scale: [1, 1.15, 1], opacity: [0.4, 0.15, 0.4] }}
          transition={{ duration: 3, repeat: Infinity, ease: 'easeInOut', delay: 0.2 }}
        />
        <div className="absolute inset-4 rounded-full bg-gradient-to-br from-white/[0.08] to-white/[0.02] border border-white/10 flex items-center justify-center">
          <span className="text-xl text-white/30">◎</span>
        </div>
      </div>

      <p className="text-sm font-medium text-white/50 mb-1.5">No node selected</p>
      <p className="text-xs text-white/30 max-w-[200px] leading-relaxed">
        Click a node in the graph to view its details, properties, and relationships
      </p>

      {/* Keyboard hint */}
      <div className="mt-6 flex items-center gap-2 px-3 py-1.5 rounded-lg bg-white/[0.03] border border-white/[0.06]">
        <span className="text-[10px] text-white/30">Press</span>
        <kbd className="px-1.5 py-0.5 rounded bg-white/[0.06] text-[10px] font-mono text-white/50">
          Tab
        </kbd>
        <span className="text-[10px] text-white/30">to navigate</span>
      </div>
    </div>
  );
}

/**
 * Premium panel container with glassmorphism effect
 */
const PanelContainer = memo(function PanelContainer({
  children,
  colors,
  className,
}: {
  children: React.ReactNode;
  colors: { primary: string; secondary: string };
  className?: string;
}) {
  return (
    <div
      className={cn(
        'relative flex flex-col h-full overflow-hidden',
        'rounded-xl backdrop-blur-xl',
        className
      )}
      style={{
        background: `linear-gradient(180deg, ${glass.surface[1]}, ${glass.surface[0]})`,
        border: `1px solid ${glass.border.light}`,
        boxShadow: `
          0 25px 50px -12px rgba(0, 0, 0, 0.6),
          inset 0 1px 0 ${glass.highlight.subtle},
          0 0 0 1px ${glass.border.subtle}
        `,
      }}
    >
      {/* Top accent line */}
      <div
        className="absolute top-0 left-4 right-4 h-px"
        style={{
          background: `linear-gradient(90deg, transparent, ${colors.primary}40, transparent)`,
        }}
      />

      {/* Content */}
      {children}

      {/* Bottom gradient fade */}
      <div
        className="absolute bottom-0 left-0 right-0 h-8 pointer-events-none"
        style={{
          background: `linear-gradient(to top, ${glass.surface[0]}, transparent)`,
        }}
      />
    </div>
  );
});

/**
 * Content wrapper with custom scrollbar styling
 */
const ContentWrapper = memo(function ContentWrapper({
  children,
  colors: _colors,
}: {
  children: React.ReactNode;
  colors: { primary: string; secondary: string };
}) {
  return (
    <div
      className="flex-1 overflow-hidden min-h-0 relative"
      style={{
        // Custom scrollbar track color matching panel
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ['--scrollbar-track' as any]: glass.surface[1],
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ['--scrollbar-thumb' as any]: glass.border.medium,
      }}
    >
      <AnimatePresence mode="wait">
        {children}
      </AnimatePresence>
    </div>
  );
});

export const TabbedDetailPanel = memo(function TabbedDetailPanel({
  node,
  className,
}: TabbedDetailPanelProps) {
  const { activeTab, setActiveTab, clearSelection } = useUIStore(
    useShallow((state) => ({
      activeTab: state.detailPanelTab,
      setActiveTab: state.setDetailPanelTab,
      clearSelection: state.clearSelection,
    }))
  );

  const { edges, nodes: allNodes } = useGraphStore(
    useShallow((state) => ({
      edges: state.edges,
      nodes: state.nodes,
    }))
  );

  // Get related edges for the selected node
  const relatedData = useMemo(() => {
    if (!node) return { relatedEdges: [], relatedNodes: [] };

    const nodeEdges = edges.filter(
      (e) => e.source === node.id || e.target === node.id
    );
    const relatedIds = new Set(
      nodeEdges
        .flatMap((e) => [e.source, e.target])
        .filter((id) => id !== node.id)
    );
    const relatedNodes = allNodes.filter((n) => relatedIds.has(n.id));

    return { relatedEdges: nodeEdges, relatedNodes };
  }, [node, edges, allNodes]);

  // Get node config and colors
  const config = node ? NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project : null;
  const layerPalette = config ? getLayerPalette(config.layer) : { primary: '#888', secondary: '#666666' };
  const colors = { primary: layerPalette.primary, secondary: layerPalette.secondary };

  // Empty state
  if (!node) {
    return (
      <PanelContainer colors={{ primary: '#666', secondary: '#444' }} className={className}>
        <EmptyState />
      </PanelContainer>
    );
  }

  return (
    <PanelContainer colors={colors} className={className}>
      {/* Header with node identity */}
      <ElementIdentityCard
        elementType="node"
        variant="header"
        layer={config?.layer || 'foundation'}
        colors={colors}
        displayName={node.displayName}
        typeLabel={config?.label || node.type}
        nodeKey={node.key}
        onClose={clearSelection}
      />

      {/* Premium tab bar */}
      <TabBar activeTab={activeTab} onTabChange={setActiveTab} colors={colors} />

      {/* Tab content with premium animations */}
      <ContentWrapper colors={colors}>
        <motion.div
          key={activeTab}
          initial={{ opacity: 0, y: 12, scale: 0.98 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: -8, scale: 0.98 }}
          transition={{
            duration: 0.2,
            ease: [0.25, 0.46, 0.45, 0.94], // Custom ease for premium feel
          }}
          className="h-full overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-white/10 hover:scrollbar-thumb-white/20"
        >
          {activeTab === 'overview' && (
            <OverviewTab node={node} colors={colors} config={config} />
          )}
          {activeTab === 'data' && (
            <DataTab
              node={node}
              colors={colors}
              relatedEdges={relatedData.relatedEdges}
            />
          )}
          {activeTab === 'graph' && (
            <GraphTab
              node={node}
              colors={colors}
              relatedEdges={relatedData.relatedEdges}
              relatedNodes={relatedData.relatedNodes}
            />
          )}
          {activeTab === 'code' && <CodeTab node={node} colors={colors} />}
        </motion.div>
      </ContentWrapper>
    </PanelContainer>
  );
});

export default TabbedDetailPanel;
