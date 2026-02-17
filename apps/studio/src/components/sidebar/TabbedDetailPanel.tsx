'use client';

/**
 * TabbedDetailPanel - Main container for tabbed node details
 *
 * Features:
 * - 4 tabs: Overview, Data, Graph, Code
 * - Motion animations for tab transitions
 * - Synced with uiStore for tab persistence
 * - Glassmorphism design with Magic UI effects
 */

import { memo, useMemo } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLayerPalette } from '@/design/colors/palette';
import { useUIStore, type DetailPanelTab } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
import { panelClasses } from '@/design/tokens';
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
}

const TABS: TabConfig[] = [
  { id: 'overview', label: 'Overview', shortcut: '1' },
  { id: 'data', label: 'Data', shortcut: '2' },
  { id: 'graph', label: 'Graph', shortcut: '3' },
  { id: 'code', label: 'Code', shortcut: '4' },
];

/**
 * Tab bar with active indicator animation
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
    <div className="flex items-center border-b border-white/[0.06] bg-black/20">
      {TABS.map((tab) => {
        const isActive = tab.id === activeTab;
        return (
          <button
            key={tab.id}
            onClick={() => onTabChange(tab.id)}
            className={cn(
              'relative flex-1 px-4 py-3 text-sm font-medium transition-colors',
              'hover:bg-white/5 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/20',
              isActive ? 'text-white' : 'text-white/50 hover:text-white/70'
            )}
            title={`${tab.label} (${tab.shortcut})`}
          >
            {tab.label}
            {isActive && (
              <motion.div
                layoutId="activeTabIndicator"
                className="absolute bottom-0 left-0 right-0 h-0.5"
                style={{
                  background: `linear-gradient(90deg, ${colors.primary}, ${colors.secondary})`,
                }}
                transition={{ type: 'spring', bounce: 0.2, duration: 0.4 }}
              />
            )}
          </button>
        );
      })}
    </div>
  );
});

/**
 * Empty state when no node is selected
 */
function EmptyState() {
  return (
    <div className="h-full flex flex-col items-center justify-center p-8 text-center">
      <div className="w-16 h-16 mb-4 rounded-full bg-white/5 flex items-center justify-center">
        <span className="text-2xl opacity-50">◎</span>
      </div>
      <p className="text-sm text-white/40 mb-1">No node selected</p>
      <p className="text-xs text-white/25">Click a node in the graph to view details</p>
    </div>
  );
}

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

  if (!node) {
    return (
      <div className={cn(panelClasses.container, className)}>
        <EmptyState />
      </div>
    );
  }

  return (
    <div className={cn(panelClasses.container, 'flex flex-col relative', className)}>
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

      {/* Tab bar */}
      <TabBar activeTab={activeTab} onTabChange={setActiveTab} colors={colors} />

      {/* Tab content with animations */}
      <div className="flex-1 overflow-hidden min-h-0">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeTab}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -8 }}
            transition={{ duration: 0.15, ease: 'easeOut' }}
            className="h-full overflow-y-auto scrollbar-thin"
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
        </AnimatePresence>
      </div>
    </div>
  );
});

export default TabbedDetailPanel;
