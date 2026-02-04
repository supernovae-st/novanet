'use client';

/**
 * ArcDetailsPanel - Details display for selected arcs (relationships)
 *
 * Features:
 * - Arc type with color coding
 * - Source and target node navigation
 * - Arc properties display
 * - Copy functionality
 * - Unified design with NodeDetailsPanel
 *
 * v9.5.0 — Arc terminology
 */

import { useState, useMemo, memo } from 'react';
import { cn } from '@/lib/utils';
import { getRelationColors } from '@/design/nodeColors';
import { ACTION_ICONS, CONTENT_ICONS, GRAPH_ICONS, ICON_COLORS } from '@/config/iconSystem';
import { iconSizes, panelClasses, gapTokens } from '@/design/tokens';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';
import { useCopyFieldFeedback } from '@/hooks';
import {
  CollapsibleSection,
  PropertyRow,
  NodeNavigationCard,
  CopyButton,
  JsonView,
  formatValueString,
} from '@/components/ui/detail-panel';
import type { GraphEdge } from '@/types';

// Design system icons
const HashIcon = CONTENT_ICONS.id;
const ArrowRightIcon = ACTION_ICONS.target;
const LinkIcon = GRAPH_ICONS.link;

interface ArcDetailsPanelProps {
  /** The arc to display (uses GraphEdge for React Flow compatibility) */
  arc: GraphEdge | null;
}


export const ArcDetailsPanel = memo(function ArcDetailsPanel({ arc }: ArcDetailsPanelProps) {
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set(['nodes', 'properties', 'json'])
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
    if (!arc) return { sourceNode: null, targetNode: null };
    return {
      sourceNode: getNodeById(arc.source) || null,
      targetNode: getNodeById(arc.target) || null,
    };
  }, [arc, getNodeById]);

  if (!arc) {
    return (
      <div className="h-full flex items-center justify-center p-8">
        <p className="text-sm text-white/40">No arc selected</p>
      </div>
    );
  }

  const colors = getRelationColors(arc.type);
  const dataEntries = arc.data ? Object.entries(arc.data) : [];

  return (
    <div className={panelClasses.container}>
      {/* Header - matches NodeDetailsPanel structure */}
      <div
        className="p-5 border-b border-white/[0.06]"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.secondary}08)`,
        }}
      >
        {/* Type badge */}
        <div
          className={cn('inline-flex items-center px-3 py-1.5 rounded-full text-xs font-bold mb-4 border', gapTokens.default)}
          style={{
            background: `linear-gradient(135deg, ${colors.primary}35, ${colors.secondary}25)`,
            borderColor: `${colors.primary}50`,
            color: colors.primary,
            boxShadow: `0 0 12px ${colors.primary}30`,
          }}
        >
          <LinkIcon className={iconSizes.sm} />
          Arc
        </div>

        {/* Arc Type */}
        <h2 className="text-xl font-bold text-white mb-2">
          {formatArcType(arc.type)}
        </h2>

        {/* ID with copy */}
        <div className={cn('flex items-center text-sm', gapTokens.default)}>
          <HashIcon className={cn(iconSizes.xs, 'text-white/40')} />
          <span className="font-mono text-white/60 truncate flex-1">
            {arc.id}
          </span>
          <CopyButton
            onCopy={() => copyToClipboard(arc.id, 'id')}
            isCopied={copiedField === 'id'}
            label="Copy ID to clipboard"
            size="sm"
          />
        </div>
      </div>

      {/* Scrollable Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin scroll-smooth">
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
                className={cn('flex items-center px-3 py-1.5 rounded-full text-[10px] font-semibold border', gapTokens.default)}
                style={{
                  background: `linear-gradient(135deg, ${colors.primary}20, ${colors.secondary}20)`,
                  borderColor: `${colors.primary}35`,
                  color: colors.primary,
                }}
              >
                <ArrowRightIcon className={iconSizes.xs} />
                {arc.type}
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

        {/* Raw JSON Section */}
        <CollapsibleSection
          title="JSON"
          isExpanded={expandedSections.has('json')}
          onToggle={() => toggleSection('json')}
        >
          <JsonView
            data={arc}
            onCopy={() => copyToClipboard(JSON.stringify(arc, null, 2), 'json')}
            isCopied={copiedField === 'json'}
          />
        </CollapsibleSection>
      </div>
    </div>
  );
});


// =============================================================================
// Helpers
// =============================================================================

function formatArcType(type: string): string {
  return type
    .replace(/_/g, ' ')
    .toLowerCase()
    .split(' ')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}
