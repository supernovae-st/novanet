'use client';

/**
 * ArcOverviewTab - Arc overview with direction and connected nodes
 *
 * Features:
 * - Arc type badge with ArcFamily color
 * - Animated direction indicator (source → target)
 * - 2x NodeNavigationCard (clickable)
 * - Key properties section
 *
 * v11.7 — Enhanced arc experience
 */

import { memo } from 'react';
import { motion } from 'motion/react';
import { ArrowRight, Hash, Clock } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { CopyButton } from '@/components/dx/CopyButton';
import { NodeNavigationCard } from '@/components/ui/detail-panel';
import { useUIStore } from '@/stores/uiStore';
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

export const ArcOverviewTab = memo(function ArcOverviewTab({
  arc,
  sourceNode,
  targetNode,
  colors,
}: ArcOverviewTabProps) {
  const { copied, copy } = useCopyFeedback();
  const setSelectedNode = useUIStore((state) => state.setSelectedNode);

  const arcType = arc.type || (arc.data?.relationType as string | undefined) || 'UNKNOWN';
  const arcId = arc.id;

  // Extract any timestamps if available
  const createdAt = arc.data?.createdAt as string | undefined;
  const updatedAt = arc.data?.updatedAt as string | undefined;

  return (
    <div className="p-4 space-y-6">
      {/* Arc type header */}
      <div
        className="p-4 rounded-xl"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.glow}08)`,
          border: `1px solid ${colors.primary}25`,
        }}
      >
        {/* Arc type badge */}
        <div
          className={cn(
            'inline-flex items-center px-3 py-1.5 rounded-lg text-xs font-mono font-semibold uppercase tracking-wider mb-3',
            gapTokens.tight
          )}
          style={{
            background: `linear-gradient(135deg, ${colors.primary}35, ${colors.glow}25)`,
            color: colors.primary,
            boxShadow: `0 0 12px ${colors.glow}30`,
          }}
        >
          <ArrowRight className="w-3.5 h-3.5" />
          <span>{String(arcType).replace(/_/g, ' ')}</span>
        </div>

        {/* Arc ID with copy */}
        <div className={cn('flex items-center text-sm', gapTokens.default)}>
          <Hash className="w-3.5 h-3.5 text-white/30" />
          <span className="font-mono text-white/50 flex-1 truncate">{String(arcId)}</span>
          <CopyButton
            onCopy={() => copy(String(arcId))}
            isCopied={copied}
            label="Copy ID"
            size="sm"
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
