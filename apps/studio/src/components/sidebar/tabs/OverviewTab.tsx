'use client';

/**
 * OverviewTab - Node overview with classification and description
 *
 * Features:
 * - Type badge with layer icon
 * - Classification grid (realm, layer, trait)
 * - Description and LLM context
 * - Copy key functionality
 */

import { memo } from 'react';
import { Hash, MapPin, Layers, Sparkles } from 'lucide-react';
import { KIND_META } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { CopyButton } from '@/components/dx/CopyButton';
import { gapTokens } from '@/design/tokens';
import {
  REALM_DISPLAY_NAMES,
  LAYER_DISPLAY_NAMES,
  TRAIT_DISPLAY_NAMES,
  getRealmColor,
  getLayerColor,
  getTraitColor,
  type RealmKey,
  type LayerKey,
  type TraitKey,
} from '@/design/tokens';
import type { GraphNode } from '@/types';
import type { NodeTypeConfig } from '@/config/nodeTypes';

interface OverviewTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
  config: NodeTypeConfig | null;
}

/**
 * Classification chip - shows realm, layer, or trait
 */
function ClassificationChip({
  icon: Icon,
  label,
  value,
  color,
}: {
  icon: React.ElementType;
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
 * Description block with optional LLM context
 */
function DescriptionBlock({
  description,
  llmContext,
}: {
  description?: string;
  llmContext?: string;
}) {
  if (!description && !llmContext) return null;

  return (
    <div className="space-y-4">
      {description && (
        <div>
          <h4 className="text-xs font-medium text-white/40 mb-2">Description</h4>
          <p className="text-sm text-white/70 leading-relaxed">{description}</p>
        </div>
      )}
      {llmContext && (
        <div className="p-3 rounded-lg bg-purple-500/10 border border-purple-500/20">
          <div className="flex items-center gap-1.5 text-xs text-purple-400 mb-2">
            <Sparkles className="w-3 h-3" />
            LLM Context
          </div>
          <p className="text-sm text-white/60 leading-relaxed font-mono">
            {llmContext}
          </p>
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

  // Get classification from KIND_META
  const kindMeta = KIND_META[node.type];
  const realm = (kindMeta?.realm ?? 'org') as RealmKey;
  const layer = (config?.layer ?? 'foundation') as LayerKey;
  const trait = (kindMeta?.trait ?? 'invariant') as TraitKey;

  return (
    <div className="p-4 space-y-6">
      {/* Header with type badge and key */}
      <div
        className="p-4 rounded-xl"
        style={{
          background: `linear-gradient(135deg, ${colors.primary}15, ${colors.secondary}08)`,
          border: `1px solid ${colors.primary}25`,
        }}
      >
        {/* Type badge */}
        <div
          className={cn(
            'inline-flex items-center px-3 py-1.5 rounded-full text-xs font-bold mb-3',
            gapTokens.tight
          )}
          style={{
            background: `linear-gradient(135deg, ${colors.primary}35, ${colors.secondary}25)`,
            color: colors.primary,
            boxShadow: `0 0 12px ${colors.primary}30`,
          }}
        >
          {config?.label || node.type}
        </div>

        {/* Display name */}
        <h3 className="text-lg font-semibold text-white mb-2">
          {node.displayName}
        </h3>

        {/* Key with copy */}
        <div className={cn('flex items-center text-sm', gapTokens.default)}>
          <Hash className="w-3.5 h-3.5 text-white/30" />
          <span className="font-mono text-white/50 flex-1 truncate">
            {node.key}
          </span>
          <CopyButton
            onCopy={() => copy(node.key)}
            isCopied={copied}
            label="Copy key"
            size="sm"
          />
        </div>
      </div>

      {/* Classification grid */}
      <div>
        <h4 className="text-xs font-medium text-white/40 mb-3">Classification</h4>
        <div className="grid grid-cols-3 gap-2">
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
          <ClassificationChip
            icon={Sparkles}
            label="Trait"
            value={TRAIT_DISPLAY_NAMES[trait] || trait}
            color={getTraitColor(trait).color}
          />
        </div>
      </div>

      {/* Description section */}
      <DescriptionBlock
        description={node.description}
        llmContext={node.llmContext}
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
