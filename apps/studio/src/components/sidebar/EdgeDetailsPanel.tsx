'use client';

/**
 * EdgeDetailsPanel - Details display for selected edges/relations
 *
 * Features:
 * - Relation type with color coding
 * - Source and target node navigation
 * - Edge properties display
 * - Copy functionality
 */

import { useState, useMemo } from 'react';
import { cn } from '@/lib/utils';
import { getRelationColors } from '@/config/categoryColors';
import { ACTION_ICONS, NAV_ICONS, CONTENT_ICONS, GRAPH_ICONS, ICON_COLORS } from '@/config/iconSystem';
import { iconSizes, glassClasses, gapTokens } from '@/design/tokens';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';
import { useCopyFieldFeedback } from '@/hooks';
import {
  CollapsibleSection,
  PropertyRow,
  NodeNavigationCard,
  CopyButton,
  JsonToggleSection,
  formatValueString,
} from '@/components/ui/detail-panel';
import type { GraphEdge } from '@/types';

// Design system icons
const CloseIcon = ACTION_ICONS.close;
const HashIcon = CONTENT_ICONS.id;
const ArrowRightIcon = NAV_ICONS.arrowRight;
const LinkIcon = GRAPH_ICONS.link;

interface EdgeDetailsPanelProps {
  edge: GraphEdge | null;
  onClose?: () => void;
}

export function EdgeDetailsPanel({ edge, onClose }: EdgeDetailsPanelProps) {
  const [showJson, setShowJson] = useState(false);
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set(['nodes', 'properties'])
  );

  const getNodeById = useGraphStore((state) => state.getNodeById);
  const { setSelectedNode } = useUIStore();
  const { copiedField, copyField: copyToClipboard } = useCopyFieldFeedback();

  const toggleSection = (section: string) => {
    setExpandedSections((prev) => {
      const next = new Set(prev);
      if (next.has(section)) {
        next.delete(section);
      } else {
        next.add(section);
      }
      return next;
    });
  };

  // Get source and target nodes
  const { sourceNode, targetNode } = useMemo(() => {
    if (!edge) return { sourceNode: null, targetNode: null };
    return {
      sourceNode: getNodeById(edge.source) || null,
      targetNode: getNodeById(edge.target) || null,
    };
  }, [edge, getNodeById]);

  if (!edge) {
    return (
      <div className={`h-full flex flex-col ${glassClasses.floating} animate-slide-in-right`}>
        <div className="flex-1 flex flex-col items-center justify-center p-8 text-center">
          <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-white/[0.08] to-white/[0.02] border border-white/[0.08] flex items-center justify-center mb-4 animate-float">
            <LinkIcon className="w-8 h-8 text-white/40" />
          </div>
          <h3 className="text-sm font-semibold text-white/70 mb-2">
            No Relation Selected
          </h3>
          <p className="text-xs text-white/40 max-w-[200px] leading-relaxed">
            Click on a relation line in the graph to view its details
          </p>
        </div>
      </div>
    );
  }

  const colors = getRelationColors(edge.type);
  const dataEntries = edge.data ? Object.entries(edge.data) : [];

  return (
    <div className={`h-full flex flex-col ${glassClasses.floating} animate-slide-in-right`}>
      {/* Header with gradient */}
      <div className="relative overflow-hidden">
        {/* Gradient background */}
        <div
          className="absolute inset-0 opacity-25"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}, ${colors.secondary})`,
          }}
        />

        {/* Glow effect */}
        <div
          className="absolute inset-0 opacity-20 blur-2xl"
          style={{
            background: `radial-gradient(circle at 30% 30%, ${colors.primary}, transparent 60%)`,
          }}
        />

        {/* Content */}
        <div className="relative p-4">
          {/* Close button */}
          {onClose && (
            <button
              onClick={onClose}
              className="absolute top-3 right-3 p-1.5 rounded-lg bg-black/30 hover:bg-black/50 text-white/50 hover:text-white/80 transition-colors"
              aria-label="Close edge details panel"
            >
              <CloseIcon className={iconSizes.md} />
            </button>
          )}

          {/* Type badge */}
          <div
            className={cn('inline-flex items-center px-3 py-1.5 rounded-full text-xs font-bold mb-3 shadow-lg', gapTokens.default)}
            style={{
              background: `linear-gradient(135deg, ${colors.primary}40, ${colors.secondary}30)`,
              color: colors.primary,
              boxShadow: `0 4px 12px ${colors.primary}20`,
            }}
          >
            <LinkIcon className={iconSizes.sm} />
            Relation
          </div>

          {/* Relation Type */}
          <h2 className="text-lg font-bold text-white mb-2 pr-8">
            {formatRelationType(edge.type)}
          </h2>

          {/* ID with copy */}
          <div className={cn('flex items-center text-xs', gapTokens.default)}>
            <HashIcon className={cn(iconSizes.xs, 'text-white/40')} />
            <span className="font-mono text-white/60 truncate flex-1">
              {edge.id}
            </span>
            <CopyButton
              onCopy={() => copyToClipboard(edge.id, 'id')}
              isCopied={copiedField === 'id'}
              label="Copy ID to clipboard"
              size="sm"
            />
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        {/* Connected Nodes Section */}
        <CollapsibleSection
          title="Connected Nodes"
          icon={<ArrowRightIcon className={iconSizes.md} />}
          isExpanded={expandedSections.has('nodes')}
          onToggle={() => toggleSection('nodes')}
        >
          <div className="space-y-2">
            {/* Source Node */}
            <NodeNavigationCard
              label="Source"
              node={sourceNode}
              labelColor={ICON_COLORS.system.primary}
              onClick={() => sourceNode && setSelectedNode(sourceNode.id)}
            />

            {/* Arrow */}
            <div className="flex items-center justify-center py-2">
              <div
                className={cn('flex items-center px-3 py-1.5 rounded-full text-[10px] font-semibold', gapTokens.default)}
                style={{
                  background: `linear-gradient(135deg, ${colors.primary}20, ${colors.secondary}20)`,
                  color: colors.primary,
                }}
              >
                <ArrowRightIcon className={iconSizes.xs} />
                {edge.type}
              </div>
            </div>

            {/* Target Node */}
            <NodeNavigationCard
              label="Target"
              node={targetNode}
              labelColor={ICON_COLORS.node.primary}
              onClick={() => targetNode && setSelectedNode(targetNode.id)}
            />
          </div>
        </CollapsibleSection>

        {/* Properties Section */}
        {dataEntries.length > 0 && (
          <CollapsibleSection
            title="Properties"
            icon={<HashIcon className={iconSizes.md} />}
            count={dataEntries.length}
            isExpanded={expandedSections.has('properties')}
            onToggle={() => toggleSection('properties')}
          >
            <div className="space-y-0">
              {dataEntries.map(([key, value]) => (
                <PropertyRow
                  key={key}
                  label={key}
                  value={value}
                  onCopy={() =>
                    copyToClipboard(formatValueString(value), `prop.${key}`)
                  }
                  isCopied={copiedField === `prop.${key}`}
                />
              ))}
            </div>
          </CollapsibleSection>
        )}
      </div>

      {/* Footer - JSON Toggle */}
      <JsonToggleSection
        data={edge}
        isOpen={showJson}
        onToggle={() => setShowJson(!showJson)}
        onCopy={() => copyToClipboard(JSON.stringify(edge, null, 2), 'json')}
        isCopied={copiedField === 'json'}
      />
    </div>
  );
}

// =============================================================================
// Helpers
// =============================================================================

function formatRelationType(type: string): string {
  return type
    .replace(/_/g, ' ')
    .toLowerCase()
    .split(' ')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}
