'use client';

/**
 * ArcOverviewTab - Arc overview with direction and connected nodes
 *
 * Features:
 * - 3D arc preview (centered)
 * - Animated direction indicator (source → target)
 * - 2x NodeNavigationCard (clickable)
 * - Key properties section
 *
 * Note: Identity (badge + arc ID) is shown in panel header via ElementIdentityCard
 *
 * v11.7 — Enhanced arc experience
 */

import { memo, useMemo } from 'react';
import { motion } from 'motion/react';
import { ArrowRight, Clock } from 'lucide-react';
import { KIND_META } from '@novanet/core/types';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { CopyButton } from '@/components/dx/CopyButton';
import { ArcPreview3D } from '@/components/graph/ArcPreview3D';
import { NodeNavigationCard } from '@/components/ui/detail-panel';
import { useUIStore } from '@/stores/uiStore';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { gapTokens } from '@/design/tokens';
import type { GraphEdge, GraphNode } from '@/types';

interface ArcOverviewTabProps {
  arc: GraphEdge;
  sourceNode: GraphNode | null;
  targetNode: GraphNode | null;
  colors: { primary: string; glow: string };
}

/**
 * Animated arrow pulse between source and target
 */
function DirectionIndicator({ colors }: { colors: { primary: string; glow: string } }) {
  return (
    <div className="flex items-center justify-center py-2">
      <div className="flex items-center gap-2">
        <motion.div
          className="w-8 h-0.5 rounded-full origin-left"
          style={{ background: `linear-gradient(90deg, transparent, ${colors.primary})` }}
          initial={{ scaleX: 0, opacity: 0 }}
          animate={{ scaleX: 1, opacity: 1 }}
          transition={{ duration: 0.3 }}
        />
        <motion.div
          animate={{ x: [0, 4, 0] }}
          transition={{ duration: 1.5, repeat: Infinity, ease: 'easeInOut' }}
        >
          <ArrowRight
            className="w-5 h-5"
            style={{
              color: colors.primary,
              filter: `drop-shadow(0 0 8px ${colors.glow})`,
            }}
          />
        </motion.div>
        <motion.div
          className="w-8 h-0.5 rounded-full origin-right"
          style={{ background: `linear-gradient(90deg, ${colors.primary}, transparent)` }}
          initial={{ scaleX: 0, opacity: 0 }}
          animate={{ scaleX: 1, opacity: 1 }}
          transition={{ duration: 0.3, delay: 0.1 }}
        />
      </div>
    </div>
  );
}

/**
 * Property item display
 */
function PropertyItem({
  label,
  value,
  icon: Icon,
  onCopy,
  isCopied,
}: {
  label: string;
  value: string;
  icon?: React.ComponentType<{ className?: string }>;
  onCopy?: () => void;
  isCopied?: boolean;
}) {
  return (
    <div className="flex items-center justify-between py-2 px-3 rounded-lg hover:bg-white/[0.03] group">
      <div className={cn('flex items-center text-xs', gapTokens.default)}>
        {Icon && <Icon className="w-3.5 h-3.5 text-white/30" />}
        <span className="text-white/40">{label}</span>
      </div>
      <div className={cn('flex items-center', gapTokens.default)}>
        <span className="font-mono text-sm text-white/70 truncate max-w-[200px]">{value}</span>
        {onCopy && (
          <CopyButton
            onCopy={onCopy}
            isCopied={isCopied || false}
            label={`Copy ${label}`}
            size="sm"
            className="opacity-0 group-hover:opacity-100 transition-opacity"
          />
        )}
      </div>
    </div>
  );
}

/**
 * Get node classification for 3D preview
 */
function getNodeClassification(node: GraphNode | null): {
  layer: Layer;
  realm: Realm;
  trait: Trait;
} {
  if (!node) {
    return { layer: 'foundation', realm: 'org', trait: 'invariant' };
  }
  const kindMeta = KIND_META[node.type];
  const config = NODE_TYPE_CONFIG[node.type];
  return {
    layer: (config?.layer ?? 'foundation') as Layer,
    realm: (kindMeta?.realm ?? 'org') as Realm,
    trait: (kindMeta?.trait ?? 'invariant') as Trait,
  };
}

export const ArcOverviewTab = memo(function ArcOverviewTab({
  arc,
  sourceNode,
  targetNode,
  colors,
}: ArcOverviewTabProps) {
  const { copied, copy } = useCopyFeedback();
  const setSelectedNode = useUIStore((state) => state.setSelectedNode);

  const arcType = arc.type || (arc.data?.relationType as string | undefined) || 'UNKNOWN';

  // Extract any timestamps if available
  const createdAt = arc.data?.createdAt as string | undefined;
  const updatedAt = arc.data?.updatedAt as string | undefined;

  // Get node classifications for 3D preview
  const sourceClassification = useMemo(() => getNodeClassification(sourceNode), [sourceNode]);
  const targetClassification = useMemo(() => getNodeClassification(targetNode), [targetNode]);

  return (
    <div className="p-4 space-y-6">
      {/* 3D Arc Preview (centered) */}
      <div className="flex justify-center">
        <div
          className="p-3 rounded-xl"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}08, ${colors.glow}04)`,
            border: `1px solid ${colors.primary}15`,
          }}
        >
          <ArcPreview3D
            arcType={arcType}
            source={sourceClassification}
            target={targetClassification}
            size={140}
          />
        </div>
      </div>

      {/* Direction section: Source → Target */}
      <div>
        <h4 className="text-xs font-medium text-white/40 mb-3">Direction</h4>

        {/* Source node card */}
        <NodeNavigationCard
          node={sourceNode}
          label="Source"
          labelColor={colors.primary}
          onClick={() => sourceNode && setSelectedNode(sourceNode.id)}
        />

        {/* Animated arrow */}
        <DirectionIndicator colors={colors} />

        {/* Target node card */}
        <NodeNavigationCard
          node={targetNode}
          label="Target"
          labelColor={colors.primary}
          onClick={() => targetNode && setSelectedNode(targetNode.id)}
        />
      </div>

      {/* Properties section */}
      <div>
        <h4 className="text-xs font-medium text-white/40 mb-2">Properties</h4>
        <div className="rounded-lg border border-white/[0.06] overflow-hidden">
          <PropertyItem
            label="Type"
            value={arcType}
            onCopy={() => copy(arcType)}
            isCopied={copied}
          />
          {createdAt && (
            <PropertyItem
              label="Created"
              value={new Date(createdAt).toLocaleString()}
              icon={Clock}
            />
          )}
          {updatedAt && (
            <PropertyItem
              label="Updated"
              value={new Date(updatedAt).toLocaleString()}
              icon={Clock}
            />
          )}
        </div>
      </div>
    </div>
  );
});

export default ArcOverviewTab;
