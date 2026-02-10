'use client';

/**
 * TabbedArcPanel - Main container for tabbed arc (edge) details
 *
 * Features:
 * - 4 tabs: Overview, Context, Data, Cypher
 * - Motion animations for tab transitions
 * - Synced with uiStore for tab persistence
 * - Glassmorphism design matching TabbedDetailPanel
 *
 * v11.7 — Enhanced arc experience (like nodes)
 */

import React, { memo, useMemo, useCallback, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { ArrowRight } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { useGraphStore } from '@/stores/graphStore';
import { panelClasses } from '@/design/tokens';
import { getArcPalette } from '@/design/colors/palette';
import { ElementIdentityCard } from '@/components/ui/detail-panel';
import { ArcOverviewTab } from './tabs/ArcOverviewTab';
import { ArcContextTab } from './tabs/ArcContextTab';
import { ArcDataTab } from './tabs/ArcDataTab';
import { ArcCypherTab } from './tabs/ArcCypherTab';
import type { GraphEdge } from '@/types';

// Arc panel tab type
type ArcPanelTab = 'overview' | 'context' | 'data' | 'cypher';

interface TabbedArcPanelProps {
  arc: GraphEdge | null;
  className?: string;
}

interface TabConfig {
  id: ArcPanelTab;
  label: string;
  shortcut: string;
}

const TABS: TabConfig[] = [
  { id: 'overview', label: 'Overview', shortcut: '1' },
  { id: 'context', label: 'Context', shortcut: '2' },
  { id: 'data', label: 'Data', shortcut: '3' },
  { id: 'cypher', label: 'Cypher', shortcut: '4' },
];

/**
 * Tab bar with active indicator animation
 */
const TabBar = memo(function TabBar({
  activeTab,
  onTabChange,
  colors,
}: {
  activeTab: ArcPanelTab;
  onTabChange: (tab: ArcPanelTab) => void;
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
                layoutId="activeArcTabIndicator"
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
 * Empty state when no arc is selected
 */
function EmptyState() {
  return (
    <div className="h-full flex flex-col items-center justify-center p-8 text-center">
      <div className="w-16 h-16 mb-4 rounded-full bg-white/5 flex items-center justify-center">
        <ArrowRight className="w-8 h-8 text-white/30" />
      </div>
      <p className="text-sm text-white/40 mb-1">No arc selected</p>
      <p className="text-xs text-white/25">Click an arc in the graph to view details</p>
    </div>
  );
}

export const TabbedArcPanel = memo(function TabbedArcPanel({
  arc,
  className,
}: TabbedArcPanelProps) {
  // Local tab state for arc panel (separate from node panel)
  const [activeTab, setActiveTab] = useState<ArcPanelTab>('overview');

  const { clearSelection } = useUIStore(
    useShallow((state) => ({
      clearSelection: state.clearSelection,
    }))
  );

  const { getNodeById } = useGraphStore(
    useShallow((state) => ({
      getNodeById: state.getNodeById,
    }))
  );

  // Get source and target nodes
  const { sourceNode, targetNode } = useMemo(() => {
    if (!arc) return { sourceNode: null, targetNode: null };
    return {
      sourceNode: getNodeById(arc.source) || null,
      targetNode: getNodeById(arc.target) || null,
    };
  }, [arc, getNodeById]);

  // Handle close
  const handleClose = useCallback(() => {
    clearSelection();
  }, [clearSelection]);

  // Get arc colors based on relation type (from unified palette)
  const arcType = arc?.type || (arc?.data?.relationType as string | undefined) || 'UNKNOWN';
  const palette = getArcPalette(arcType);
  const colors = { primary: palette.primary, secondary: palette.secondary };

  if (!arc) {
    return (
      <div className={cn(panelClasses.container, className)}>
        <EmptyState />
      </div>
    );
  }

  return (
    <div className={cn(panelClasses.container, 'flex flex-col', className)}>
      {/* Header with arc identity */}
      <ElementIdentityCard
        elementType="arc"
        variant="header"
        arcType={arcType}
        arcId={arc.id}
        colors={colors}
        onClose={handleClose}
      />

      {/* Tab Bar */}
      <TabBar activeTab={activeTab} onTabChange={setActiveTab} colors={colors} />

      {/* Tab Content */}
      <div className="flex-1 overflow-hidden">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeTab}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2 }}
            className="h-full overflow-y-auto"
          >
            {activeTab === 'overview' && (
              <ArcOverviewTab
                arc={arc}
                sourceNode={sourceNode}
                targetNode={targetNode}
                colors={{ primary: colors.primary, glow: colors.secondary }}
              />
            )}
            {activeTab === 'context' && (
              <ArcContextTab
                arc={arc}
                sourceNode={sourceNode}
                targetNode={targetNode}
              />
            )}
            {activeTab === 'data' && (
              <ArcDataTab arc={arc} />
            )}
            {activeTab === 'cypher' && (
              <ArcCypherTab
                arc={arc}
                sourceNode={sourceNode}
                targetNode={targetNode}
              />
            )}
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  );
});
