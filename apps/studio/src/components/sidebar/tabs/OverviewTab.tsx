'use client';

/**
 * OverviewTab - Node overview with classification and content
 *
 * Features:
 * - Side-by-side layout: info left, 3D preview right
 * - Type badge with layer icon
 * - Classification grid (realm, layer)
 * - Content, triggers, and provenance (v0.20.0)
 * - Copy key functionality
 */

import { memo } from 'react';
import dynamic from 'next/dynamic';
import { Hash, MapPin, Layers, Tag, Database } from 'lucide-react';
import { CLASS_TAXONOMY } from '@novanet/core/types';
import type { Layer, Realm } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { CopyButton } from '@/components/dx/CopyButton';
import {
  gapTokens,
  REALM_DISPLAY_NAMES,
  LAYER_DISPLAY_NAMES,
  getRealmColor,
  getLayerColor,
  type RealmKey,
  type LayerKey,
} from '@/design/tokens';
import type { GraphNode } from '@/types';
import type { NodeTypeConfig } from '@/config/nodeTypes';

// Dynamic import for React Three Fiber component (SSR disabled)
const NodePreview3DSimple = dynamic(
  () => import('./NodePreview3DSimple').then((mod) => mod.NodePreview3DSimple),
  {
    ssr: false,
    loading: () => (
      <div className="w-[100px] h-[100px] rounded-xl bg-white/5 border border-white/10 flex items-center justify-center">
        <div className="w-6 h-6 border-2 border-white/20 border-t-white/60 rounded-full animate-spin" />
      </div>
    ),
  }
);

interface OverviewTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
  config: NodeTypeConfig | null;
}

/**
 * Classification chip - shows realm or layer
 */
function ClassificationChip({
  icon: Icon,
  label,
  value,
  color,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  value: string;
  color: string;
}) {
  return (
    <div className="flex flex-col gap-1.5 p-3 rounded-lg bg-white/[0.03] border border-white/[0.06]">
      <div className="flex items-center gap-1.5 text-xs text-white/40">
        <Icon className="w-3 h-3" />
        {label}
      </div>
      <div
        className="text-sm font-medium"
        style={{ color }}
      >
        {value}
      </div>
    </div>
  );
}

/**
 * Content block with triggers and provenance (v0.20.0)
 */
function ContentBlock({
  content,
  triggers,
  provenance,
}: {
  content?: string;
  triggers?: string[];
  provenance?: string;
}) {
  if (!content && (!triggers || triggers.length === 0) && !provenance) return null;

  return (
    <div className="space-y-4">
      {content && (
        <div>
          <h4 className="text-xs font-medium text-white/40 mb-2">Content</h4>
          <p className="text-sm text-white/70 leading-relaxed">{content}</p>
        </div>
      )}
      {triggers && triggers.length > 0 && (
        <div>
          <div className="flex items-center gap-1.5 text-xs text-white/40 mb-2">
            <Tag className="w-3 h-3" />
            Triggers
          </div>
          <div className="flex flex-wrap gap-1.5">
            {triggers.map((trigger) => (
              <span
                key={trigger}
                className="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium bg-purple-500/15 text-purple-300 border border-purple-500/20"
              >
                {trigger}
              </span>
            ))}
          </div>
        </div>
      )}
      {provenance && (
        <div className="flex items-center gap-1.5">
          <Database className="w-3 h-3 text-white/30" />
          <span className="text-xs text-white/40">Origin:</span>
          <span className="text-xs font-mono text-white/60">{provenance}</span>
        </div>
      )}
    </div>
  );
}

export const OverviewTab = memo(function OverviewTab({
  node,
  colors,
  config,
}: OverviewTabProps) {
  const { copied, copy } = useCopyFeedback();

  // Get classification from CLASS_TAXONOMY (v11.8: ADR-023)
  const classification = CLASS_TAXONOMY[node.type];
  const realm = (classification?.realm ?? 'org') as RealmKey;
  const layer = (config?.layer ?? 'foundation') as LayerKey;
  return (
    <div className="p-4 space-y-6">
      {/* Header card with side-by-side layout: Info left, 3D right */}
      <div
        className="relative p-4 rounded-xl overflow-hidden"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}12, ${colors.secondary}06)`,
          border: `1px solid ${colors.primary}20`,
        }}
      >
        <div className="flex items-start gap-4">
          {/* Left side: Text content */}
          <div className="flex-1 min-w-0">
            {/* Type badge */}
            <div
              className={cn(
                'inline-flex items-center px-2.5 py-1 rounded-full text-xs font-bold mb-3',
                gapTokens.tight
              )}
              style={{
                background: `linear-gradient(135deg, ${colors.primary}30, ${colors.secondary}20)`,
                color: colors.primary,
                boxShadow: `0 0 8px ${colors.primary}25`,
              }}
            >
              {config?.label || node.type}
            </div>

            {/* Display name */}
            <h3 className="text-base font-semibold text-white mb-1.5 leading-tight">
              {node.displayName}
            </h3>

            {/* Key with copy button */}
            <div className={cn('flex items-center text-sm', gapTokens.tight)}>
              <Hash className="w-3 h-3 text-white/25 flex-shrink-0" />
              <span className="font-mono text-white/40 truncate text-xs">
                {node.key}
              </span>
              <CopyButton
                onCopy={() => copy(node.key)}
                isCopied={copied}
                label="Copy"
                size="sm"
              />
            </div>
          </div>

          {/* Right side: 3D Preview (rotating) */}
          <div className="flex-shrink-0">
            <NodePreview3DSimple
              layer={layer as Layer}
              realm={realm as Realm}
              size={100}
            />
          </div>
        </div>
      </div>

      {/* Classification grid */}
      <div>
        <h4 className="text-xs font-medium text-white/40 mb-3">Classification</h4>
        <div className="grid grid-cols-2 gap-2">
          <ClassificationChip
            icon={MapPin}
            label="Realm"
            value={REALM_DISPLAY_NAMES[realm] || realm}
            color={getRealmColor(realm).color}
          />
          <ClassificationChip
            icon={Layers}
            label="Layer"
            value={LAYER_DISPLAY_NAMES[layer] || layer}
            color={getLayerColor(layer).color}
          />
        </div>
      </div>

      {/* Content section (v0.20.0) */}
      <ContentBlock
        content={node.content}
        triggers={node.triggers}
        provenance={node.provenance}
      />

      {/* Timestamps */}
      {(node.createdAt || node.updatedAt) && (
        <div className="pt-4 border-t border-white/[0.06]">
          <h4 className="text-xs font-medium text-white/40 mb-2">Timestamps</h4>
          <div className="space-y-1 text-xs text-white/50">
            {node.createdAt && (
              <div className="flex justify-between">
                <span>Created</span>
                <span className="font-mono">{new Date(node.createdAt).toLocaleString()}</span>
              </div>
            )}
            {node.updatedAt && (
              <div className="flex justify-between">
                <span>Updated</span>
                <span className="font-mono">{new Date(node.updatedAt).toLocaleString()}</span>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
});

export default OverviewTab;
