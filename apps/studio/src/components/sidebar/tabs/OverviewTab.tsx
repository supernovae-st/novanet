'use client';

/**
 * OverviewTab - Node overview with classification and description
 *
 * Features:
 * - 3D node preview (centered)
 * - Classification grid (realm, layer, trait)
 * - Description and LLM context
 *
 * Note: Identity (badge + title + key) is shown in panel header via ElementIdentityCard
 */

import { memo } from 'react';
import { MapPin, Layers, Sparkles } from 'lucide-react';
import { KIND_META } from '@novanet/core/types';
import type { Layer, Realm, Trait } from '@novanet/core/types';
import { NodePreview3D } from '@/components/graph/NodePreview3D';
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
  // Get classification from KIND_META
  const kindMeta = KIND_META[node.type];
  const realm = (kindMeta?.realm ?? 'org') as RealmKey;
  const layer = (config?.layer ?? 'foundation') as LayerKey;
  const trait = (kindMeta?.trait ?? 'invariant') as TraitKey;

  return (
    <div className="p-4 space-y-6">
      {/* 3D Preview (centered) */}
      <div className="flex justify-center">
        <div
          className="p-4 rounded-xl"
          style={{
            background: `linear-gradient(135deg, ${colors.primary}08, ${colors.secondary}04)`,
            border: `1px solid ${colors.primary}15`,
          }}
        >
          <NodePreview3D
            layer={layer as Layer}
            realm={realm as Realm}
            trait={trait as Trait}
            size={120}
            autoRotate
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
