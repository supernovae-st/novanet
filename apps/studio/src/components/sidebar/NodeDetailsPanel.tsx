'use client';

/**
 * NodeDetailsPanel - Clean JSON-ordered property display
 *
 * Features:
 * - Shows all properties in JSON order
 * - Clean key-value layout
 * - Copy functionality for values
 * - Consistent spacing aligned with rest of UI
 */

import { useState, useMemo } from 'react';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { getCategoryColors } from '@/config/categoryColors';
import { useGraphStore } from '@/stores/graphStore';
import { useUIStore } from '@/stores/uiStore';
import { useCopyFieldFeedback } from '@/hooks';
import {
  CollapsibleSection,
  PropertyRow,
  RelationNavigationCard,
  CopyButton,
  JsonView,
} from '@/components/ui/detail-panel';
import { CategoryIcon } from '@/components/ui/CategoryIcon';
import type { GraphNode } from '@/types';

interface NodeDetailsPanelProps {
  node: GraphNode | null;
}

export function NodeDetailsPanel({ node }: NodeDetailsPanelProps) {
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    new Set(['main', 'data', 'relations'])
  );

  const { edges, nodes: allNodes } = useGraphStore();
  const { setSelectedNode } = useUIStore();
  const { copiedField, copyField } = useCopyFieldFeedback();

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

  // Get related edges and nodes
  const { relatedEdges, relatedNodes } = useMemo(() => {
    if (!node) return { relatedEdges: [], relatedNodes: [] };

    const nodeEdges = edges.filter(
      (e) => e.source === node.id || e.target === node.id
    );
    const relatedIds = new Set(
      nodeEdges
        .flatMap((e) => [e.source, e.target])
        .filter((id) => id !== node.id)
    );
    const relNodes = allNodes.filter((n) => relatedIds.has(n.id));

    return { relatedEdges: nodeEdges, relatedNodes: relNodes };
  }, [node, edges, allNodes]);

  if (!node) {
    return (
      <div className="h-full flex items-center justify-center p-8">
        <p className="text-sm text-white/40">No node selected</p>
      </div>
    );
  }

  const config = NODE_TYPE_CONFIG[node.type] || NODE_TYPE_CONFIG.Project;
  const colors = getCategoryColors(config.category);

  // Build all properties in JSON order
  // Note: icon, priority, freshness removed in v8.2.0 (YAML v7.11.0 alignment)
  const mainProps = [
    { key: 'id', value: node.id },
    { key: 'type', value: node.type },
    { key: 'key', value: node.key },
    { key: 'displayName', value: node.displayName },
    { key: 'description', value: node.description },
    { key: 'llmContext', value: node.llmContext },
    { key: 'createdAt', value: node.createdAt },
    { key: 'updatedAt', value: node.updatedAt },
  ].filter((p) => p.value !== undefined);

  const dataProps = node.data
    ? Object.entries(node.data).map(([key, value]) => ({
        key,
        value,
      }))
    : [];

  return (
    <div className="h-full flex flex-col overflow-hidden">
      {/* Header Card */}
      <div
        className="p-5 border-b border-white/12"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.secondary}08)`,
        }}
      >
        {/* Type badge */}
        <div
          className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-bold mb-4 border"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}35, ${colors.secondary}25)`,
            borderColor: `${colors.primary}50`,
            color: colors.primary,
            boxShadow: `0 0 12px ${colors.primary}30`,
          }}
        >
          <CategoryIcon
            category={config.category}
            size={14}
            strokeWidth={2}
            style={{ color: colors.primary }}
          />
          {config.label}
        </div>

        {/* Title */}
        <h2 className="text-xl font-bold text-white mb-2">{node.displayName}</h2>

        {/* Key with copy */}
        <div className="flex items-center gap-2 text-sm">
          <span className="text-white/40">#</span>
          <span className="font-mono text-white/60 flex-1 truncate">
            {node.key}
          </span>
          <CopyButton
            onCopy={() => copyField(node.key, 'key')}
            isCopied={copiedField === 'key'}
            label="Copy key to clipboard"
            size="sm"
          />
        </div>
      </div>

      {/* Scrollable Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin scroll-smooth">
        {/* Main Properties Section */}
        <CollapsibleSection
          title="Properties"
          count={mainProps.length}
          isExpanded={expandedSections.has('main')}
          onToggle={() => toggleSection('main')}
        >
          <div className="space-y-0">
            {mainProps.map(({ key, value }) => (
              <PropertyRow
                key={key}
                label={key}
                value={value}
                onCopy={() => copyField(String(value), key)}
                isCopied={copiedField === key}
              />
            ))}
          </div>
        </CollapsibleSection>

        {/* Data Properties Section */}
        {dataProps.length > 0 && (
          <CollapsibleSection
            title="Data"
            count={dataProps.length}
            isExpanded={expandedSections.has('data')}
            onToggle={() => toggleSection('data')}
          >
            <div className="space-y-0">
              {dataProps.map(({ key, value }) => (
                <PropertyRow
                  key={key}
                  label={key}
                  value={value}
                  onCopy={() => copyField(JSON.stringify(value), `data.${key}`)}
                  isCopied={copiedField === `data.${key}`}
                />
              ))}
            </div>
          </CollapsibleSection>
        )}

        {/* Relations Section */}
        {relatedEdges.length > 0 && (
          <CollapsibleSection
            title="Relations"
            count={relatedEdges.length}
            isExpanded={expandedSections.has('relations')}
            onToggle={() => toggleSection('relations')}
          >
            <div className="space-y-1.5">
              {relatedEdges.map((edge) => {
                const isSource = edge.source === node.id;
                const relatedId = isSource ? edge.target : edge.source;
                const relatedNode =
                  relatedNodes.find((n) => n.id === relatedId) || null;

                return (
                  <RelationNavigationCard
                    key={edge.id}
                    relatedNode={relatedNode}
                    edgeType={edge.type}
                    isSource={isSource}
                    onClick={() => setSelectedNode(relatedId)}
                  />
                );
              })}
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
            data={node}
            onCopy={() => copyField(JSON.stringify(node, null, 2), 'json')}
            isCopied={copiedField === 'json'}
          />
        </CollapsibleSection>
      </div>
    </div>
  );
}
