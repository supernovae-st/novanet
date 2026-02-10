'use client';

/**
 * TabbedDetailPanel - Main container for tabbed node details
 *
 * Features:
 * - 4 tabs: Overview, Data, Graph, Code
 * - Motion animations for tab transitions
 * - Synced with uiStore for tab persistence
 * - Glassmorphism design with Magic UI effects
 * - Context Views footer with Action Cards (v11.6)
 */

import { memo, useMemo, useState, useCallback, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { X, Loader2 } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getLayerGradientColors } from '@/design/nodeColors';
import { useUIStore, type DetailPanelTab } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
import { panelClasses } from '@/design/tokens';
import { LayerIcon } from '@/components/ui/CategoryIcon';
import { OverviewTab } from './tabs/OverviewTab';
import { DataTab } from './tabs/DataTab';
import { GraphTab } from './tabs/GraphTab';
import { CodeTab } from './tabs/CodeTab';
import { ContextViewFooter } from './footer/ContextViewFooter';
import { MatrixTransition } from '@/components/graph/effects/MatrixTransition';
import { useContextViews, useViewDetails } from '@/hooks/useContextViews';
import type { ViewId } from '@/config/viewTypes';
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
  const { activeTab, setActiveTab, clearSelection, openModal } = useUIStore(
    useShallow((state) => ({
      activeTab: state.detailPanelTab,
      setActiveTab: state.setDetailPanelTab,
      clearSelection: state.clearSelection,
      openModal: state.openModal,
    }))
  );

  const { edges, nodes: allNodes, setGraphData } = useGraphStore(
    useShallow((state) => ({
      edges: state.edges,
      nodes: state.nodes,
      setGraphData: state.setGraphData,
    }))
  );

  // Active context view state
  const [activeViewId, setActiveViewId] = useState<ViewId | null>(null);
  const [isTransitioning, setIsTransitioning] = useState(false);
  const [pendingViewId, setPendingViewId] = useState<ViewId | null>(null);

  // Fetch view data when a view is selected
  const { data: viewData, isLoading: isViewLoading, error: viewError } = useViewDetails(
    node,
    pendingViewId
  );

  // Handle view data loaded - apply to graph after transition
  useEffect(() => {
    if (viewData && !isViewLoading && pendingViewId) {
      // Data is ready, the transition will call onComplete which loads the data
    }
  }, [viewData, isViewLoading, pendingViewId]);

  // Handle transition complete - load data into graph
  const handleTransitionComplete = useCallback(() => {
    setIsTransitioning(false);

    if (viewData) {
      // Load the view data into the graph
      setGraphData({
        nodes: viewData.nodes,
        edges: viewData.edges,
      });
      setActiveViewId(pendingViewId);
    }
    setPendingViewId(null);
  }, [viewData, pendingViewId, setGraphData]);

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

  // Get context views for this node
  const { views: contextViews } = useContextViews(
    node,
    relatedData.relatedEdges,
    allNodes
  );

  // Handle view selection
  const handleViewSelect = useCallback((viewId: string) => {
    const vid = viewId as ViewId;

    // Don't re-trigger if already loading this view
    if (vid === pendingViewId || vid === activeViewId) {
      return;
    }

    // Start loading and transition
    setPendingViewId(vid);
    setIsTransitioning(true);
  }, [pendingViewId, activeViewId]);

  // Handle "More Views" button
  const handleMoreViews = useCallback(() => {
    // Open the view selector modal
    // For now, just log - will integrate with existing modal system
    console.log('Open more views modal');
  }, []);

  // Get node config and colors
  const config = node ? NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project : null;
  const colors = config ? getLayerGradientColors(config.layer) : { primary: '#888', secondary: '#666' };

  if (!node) {
    return (
      <div className={cn(panelClasses.container, className)}>
        <EmptyState />
      </div>
    );
  }

  return (
    <div className={cn(panelClasses.container, 'flex flex-col relative', className)}>
      {/* Header with node title and close button */}
      <div
        className="flex items-center justify-between px-4 py-3 border-b border-white/[0.06]"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}12, ${colors.secondary}08)`,
        }}
      >
        <div className="flex items-center gap-2 min-w-0">
          <div
            className="flex items-center justify-center w-6 h-6 rounded"
            style={{
              background: `linear-gradient(135deg, ${colors.primary}30, ${colors.secondary}20)`,
            }}
          >
            <LayerIcon
              layer={config?.layer || 'foundation'}
              size={14}
              style={{ color: colors.primary }}
            />
          </div>
          <div className="min-w-0">
            <h2 className="text-sm font-semibold text-white truncate">
              {node.displayName}
            </h2>
            <p className="text-xs text-white/40 font-mono truncate">
              {node.key}
            </p>
          </div>
        </div>
        <button
          onClick={clearSelection}
          className="p-1.5 rounded hover:bg-white/10 text-white/40 hover:text-white transition-colors"
          title="Close panel (Esc)"
          aria-label="Close details panel"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

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

      {/* Context Views Footer (v11.6) */}
      {contextViews.length > 0 && (
        <ContextViewFooter
          views={contextViews}
          nodeKey={node.key}
          activeViewId={activeViewId}
          onViewSelect={handleViewSelect}
          onMoreViews={handleMoreViews}
        />
      )}

      {/* Loading indicator for view data */}
      {isViewLoading && pendingViewId && (
        <div className="absolute bottom-20 left-1/2 -translate-x-1/2 flex items-center gap-2 bg-black/80 px-3 py-2 rounded-full text-xs text-white/70">
          <Loader2 className="w-3 h-3 animate-spin" />
          Loading view...
        </div>
      )}

      {/* View error notification */}
      {viewError && (
        <div className="absolute bottom-20 left-1/2 -translate-x-1/2 bg-red-500/80 px-3 py-2 rounded text-xs text-white">
          {viewError.message}
        </div>
      )}

      {/* Matrix Transition Effect (v11.6) */}
      <MatrixTransition
        isActive={isTransitioning}
        viewId={pendingViewId}
        onComplete={handleTransitionComplete}
      />
    </div>
  );
});

export default TabbedDetailPanel;
