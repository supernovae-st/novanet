'use client';

/**
 * GraphTab - Graph visualizations and relations
 *
 * Features:
 * - View switcher: Ego, Arcs, Flow, Context
 * - Mermaid diagram rendering
 * - Relations list with navigation
 * - Context view dropdown (type-specific)
 */

import { memo, useState, useMemo, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { RefreshCw, Maximize2, Copy, ChevronDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { useUIStore } from '@/stores/uiStore';
import { RelationNavigationCard } from '@/components/ui/detail-panel';
import { gapTokens } from '@/design/tokens';
import type { GraphNode } from '@/types';
import type { Edge } from '@xyflow/react';

// View types
type GraphView = 'ego' | 'arcs' | 'flow' | 'context';

interface GraphTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
  relatedEdges: Edge[];
  relatedNodes: GraphNode[];
}

interface ViewButtonProps {
  view: GraphView;
  label: string;
  isActive: boolean;
  onClick: () => void;
  hasDropdown?: boolean;
}

/**
 * View switcher button
 */
function ViewButton({ view, label, isActive, onClick, hasDropdown }: ViewButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium transition-all',
        isActive
          ? 'bg-white/10 text-white'
          : 'text-white/50 hover:text-white/70 hover:bg-white/5'
      )}
    >
      {label}
      {hasDropdown && <ChevronDown className="w-3 h-3" />}
    </button>
  );
}

/**
 * Generate Mermaid diagram for Ego view
 */
function generateEgoDiagram(node: GraphNode, edges: Edge[], nodes: GraphNode[]): string {
  const lines: string[] = ['flowchart LR'];
  const nodeId = node.id.replace(/[^a-zA-Z0-9]/g, '_');

  // Classify edges
  const incoming: { id: string; label: string; type: string }[] = [];
  const outgoing: { id: string; label: string; type: string }[] = [];

  for (const edge of edges) {
    const relatedId = edge.source === node.id ? edge.target : edge.source;
    const relatedNode = nodes.find(n => n.id === relatedId);
    const label = relatedNode?.displayName || relatedId;
    const safeId = relatedId.replace(/[^a-zA-Z0-9]/g, '_');

    if (edge.target === node.id) {
      incoming.push({ id: safeId, label, type: edge.type || 'RELATES_TO' });
    } else {
      outgoing.push({ id: safeId, label, type: edge.type || 'RELATES_TO' });
    }
  }

  // Add incoming subgraph
  if (incoming.length > 0) {
    lines.push('  subgraph incoming[" "]');
    incoming.forEach(({ id, label }) => {
      lines.push(`    ${id}["${label}"]`);
    });
    lines.push('  end');
  }

  // Add center node
  lines.push(`  ${nodeId}["${node.displayName}"]:::selected`);

  // Add outgoing subgraph
  if (outgoing.length > 0) {
    lines.push('  subgraph outgoing[" "]');
    outgoing.forEach(({ id, label }) => {
      lines.push(`    ${id}["${label}"]`);
    });
    lines.push('  end');
  }

  // Add edges
  incoming.forEach(({ id, type }) => {
    lines.push(`  ${id} -->|${type}| ${nodeId}`);
  });
  outgoing.forEach(({ id, type }) => {
    lines.push(`  ${nodeId} -->|${type}| ${id}`);
  });

  // Add styling
  lines.push('  classDef selected fill:#10b981,stroke:#059669,stroke-width:2px,color:#fff');

  return lines.join('\n');
}

/**
 * Mermaid diagram component with loading state
 */
function MermaidDiagram({ code, isLoading }: { code: string; isLoading: boolean }) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-48 text-white/40">
        <RefreshCw className="w-5 h-5 animate-spin" />
      </div>
    );
  }

  return (
    <div className="p-4 bg-black/30 rounded-lg overflow-x-auto">
      <pre className="font-mono text-xs text-white/60 whitespace-pre-wrap">
        {code}
      </pre>
      <p className="mt-2 text-[10px] text-white/30 text-center">
        Mermaid preview placeholder — full rendering coming soon
      </p>
    </div>
  );
}

/**
 * Action bar with refresh, expand, copy
 */
function ActionBar({
  onRefresh,
  onExpand,
  onCopy,
  isCopied,
}: {
  onRefresh: () => void;
  onExpand: () => void;
  onCopy: () => void;
  isCopied: boolean;
}) {
  return (
    <div className={cn('flex items-center justify-end p-2 border-b border-white/[0.06]', gapTokens.tight)}>
      <button
        onClick={onRefresh}
        className="p-1.5 rounded hover:bg-white/10 text-white/40 hover:text-white/70 transition-colors"
        title="Refresh"
      >
        <RefreshCw className="w-3.5 h-3.5" />
      </button>
      <button
        onClick={onExpand}
        className="p-1.5 rounded hover:bg-white/10 text-white/40 hover:text-white/70 transition-colors"
        title="Expand"
      >
        <Maximize2 className="w-3.5 h-3.5" />
      </button>
      <button
        onClick={onCopy}
        className={cn(
          'p-1.5 rounded hover:bg-white/10 transition-colors',
          isCopied ? 'text-emerald-400' : 'text-white/40 hover:text-white/70'
        )}
        title={isCopied ? 'Copied!' : 'Copy diagram'}
      >
        <Copy className="w-3.5 h-3.5" />
      </button>
    </div>
  );
}

export const GraphTab = memo(function GraphTab({
  node,
  colors,
  relatedEdges,
  relatedNodes,
}: GraphTabProps) {
  const [activeView, setActiveView] = useState<GraphView>('ego');
  const [isLoading, setIsLoading] = useState(false);
  const { copied, copy } = useCopyFeedback();
  const setSelectedNode = useUIStore((state) => state.setSelectedNode);

  // Generate diagram based on active view
  const diagramCode = useMemo(() => {
    switch (activeView) {
      case 'ego':
        return generateEgoDiagram(node, relatedEdges, relatedNodes);
      case 'arcs':
        // Group by arc type
        return `flowchart TB
  subgraph OWNERSHIP
    direction TB
  end
  subgraph SEMANTIC
    direction TB
  end
  note: Arc-type grouping view`;
      case 'flow':
        return `flowchart LR
  foundation --> structure --> semantic --> output
  note: Layer flow position view`;
      case 'context':
        return `flowchart TB
  note: Context view - type-specific`;
      default:
        return '';
    }
  }, [activeView, node, relatedEdges, relatedNodes]);

  const handleRefresh = useCallback(() => {
    setIsLoading(true);
    setTimeout(() => setIsLoading(false), 500);
  }, []);

  const handleExpand = useCallback(() => {
    // TODO: Open in modal
    console.log('Expand diagram');
  }, []);

  const handleCopy = useCallback(() => {
    copy(diagramCode);
  }, [copy, diagramCode]);

  return (
    <div className="flex flex-col h-full">
      {/* View switcher */}
      <div className={cn('flex items-center p-2 border-b border-white/[0.06]', gapTokens.tight)}>
        <ViewButton
          view="ego"
          label="Ego"
          isActive={activeView === 'ego'}
          onClick={() => setActiveView('ego')}
        />
        <ViewButton
          view="arcs"
          label="Arcs"
          isActive={activeView === 'arcs'}
          onClick={() => setActiveView('arcs')}
        />
        <ViewButton
          view="flow"
          label="Flow"
          isActive={activeView === 'flow'}
          onClick={() => setActiveView('flow')}
        />
        <ViewButton
          view="context"
          label="Context"
          isActive={activeView === 'context'}
          onClick={() => setActiveView('context')}
          hasDropdown
        />
      </div>

      {/* Action bar */}
      <ActionBar
        onRefresh={handleRefresh}
        onExpand={handleExpand}
        onCopy={handleCopy}
        isCopied={copied}
      />

      {/* Content area */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeView}
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
            className="p-4"
          >
            {/* Mermaid diagram */}
            <MermaidDiagram code={diagramCode} isLoading={isLoading} />

            {/* Relations list */}
            {activeView === 'ego' && relatedEdges.length > 0 && (
              <div className="mt-4">
                <h4 className="text-xs font-medium text-white/40 mb-2">
                  Relations ({relatedEdges.length})
                </h4>
                <div className="space-y-1.5">
                  {relatedEdges.map((edge) => {
                    const isSource = edge.source === node.id;
                    const relatedId = isSource ? edge.target : edge.source;
                    const relatedNode = relatedNodes.find(n => n.id === relatedId) ?? null;

                    return (
                      <RelationNavigationCard
                        key={edge.id}
                        relatedNode={relatedNode}
                        edgeType={edge.type || 'RELATES_TO'}
                        isSource={isSource}
                        onClick={() => setSelectedNode(relatedId)}
                      />
                    );
                  })}
                </div>
              </div>
            )}
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  );
});

export default GraphTab;
