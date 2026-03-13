'use client';

/**
 * ContentBadge - Visual display of content + triggers for graph nodes
 *
 * v0.20.0: Replaces LLMContextBadge. Content and triggers are now
 * direct properties (no parsing needed).
 *
 * Display modes:
 * - compact: Content summary + trigger count only
 * - expanded: Full content with trigger chips
 *
 * Layout (compact):
 * ┌─────────────────────────────────────────────────────┐
 * │  ⚡ what this node does              │  5 triggers  │
 * └─────────────────────────────────────────────────────┘
 *
 * Layout (expanded):
 * ┌─────────────────────────────────────────────────────┐
 * │  ⚡ what this node does              │  5 triggers  │
 * │  ─────────────────────────────────────────────────  │
 * │  TRIGGERS: content • native • locale • l10n        │
 * └─────────────────────────────────────────────────────┘
 */

import { memo, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@/lib/utils';
import { Zap, ChevronDown } from 'lucide-react';
import { gapTokens } from '@/design/tokens';

// =============================================================================
// Types
// =============================================================================

export interface ContentBadgeProps {
  /** Content description (what this node IS) */
  content?: string;
  /** Trigger keywords for RAG activation */
  triggers?: string[];
  /** Display mode */
  mode?: 'compact' | 'expanded';
  /** Color for styling */
  color?: string;
  /** Whether parent is selected */
  selected?: boolean;
  /** Whether parent is hovered */
  isHovered?: boolean;
  /** Allow toggling between compact/expanded */
  expandable?: boolean;
}

// =============================================================================
// Subcomponents
// =============================================================================

const TriggerChip = memo(function TriggerChip({
  keyword,
  color,
}: {
  keyword: string;
  color: string;
}) {
  return (
    <span
      className="inline-flex items-center px-1.5 py-0.5 rounded text-[9px] font-mono"
      style={{
        background: `${color}15`,
        color: color,
        border: `1px solid ${color}30`,
      }}
    >
      {keyword}
    </span>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const ContentBadge = memo(function ContentBadge({
  content,
  triggers,
  mode: initialMode = 'compact',
  color = '#8b5cf6',
  selected = false,
  isHovered = false,
  expandable = true,
}: ContentBadgeProps) {
  const [isExpanded, setIsExpanded] = useState(initialMode === 'expanded');

  // Nothing to display
  if (!content && (!triggers || triggers.length === 0)) {
    return null;
  }

  const triggerCount = triggers?.length ?? 0;
  const showExpandToggle = expandable && triggerCount > 0;

  return (
    <motion.div
      className={cn(
        'rounded-lg overflow-hidden transition-all duration-200',
        (selected || isHovered) && 'ring-1'
      )}
      style={{
        background: `${color}08`,
        border: `1px solid ${color}20`,
        boxShadow: (selected || isHovered) ? `0 0 0 1px ${color}30` : undefined,
      }}
      layout
    >
      {/* Compact header (always visible) */}
      <div
        className={cn(
          'flex items-center justify-between px-2 py-1.5',
          showExpandToggle && 'cursor-pointer hover:bg-white/5'
        )}
        onClick={() => showExpandToggle && setIsExpanded(!isExpanded)}
      >
        <div className={cn('flex items-center', gapTokens.compact)}>
          <Zap
            size={12}
            style={{
              color,
              filter: selected ? `drop-shadow(0 0 4px ${color})` : undefined,
            }}
          />
          {content && (
            <span className="text-[10px] text-white/80 truncate max-w-[180px]">
              {content}
            </span>
          )}
        </div>

        <div className={cn('flex items-center', gapTokens.compact)}>
          {triggerCount > 0 && (
            <span
              className="text-[9px] px-1.5 py-0.5 rounded-full font-mono"
              style={{
                background: `${color}20`,
                color,
              }}
            >
              {triggerCount} trigger{triggerCount > 1 ? 's' : ''}
            </span>
          )}

          {showExpandToggle && (
            <motion.div
              animate={{ rotate: isExpanded ? 180 : 0 }}
              transition={{ duration: 0.2 }}
            >
              <ChevronDown size={12} className="text-white/40" />
            </motion.div>
          )}
        </div>
      </div>

      {/* Expanded content: trigger chips */}
      <AnimatePresence>
        {isExpanded && triggers && triggers.length > 0 && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="overflow-hidden"
          >
            <div
              className="px-2 py-2 border-t"
              style={{ borderColor: `${color}15` }}
            >
              <div className="space-y-1">
                <div className="flex items-center gap-1.5">
                  <Zap size={10} color={color} />
                  <span className="text-[9px] font-semibold uppercase tracking-wider" style={{ color }}>
                    Triggers
                  </span>
                </div>
                <div className={cn('flex flex-wrap pl-4', gapTokens.compact)}>
                  {triggers.map((keyword, i) => (
                    <TriggerChip key={i} keyword={keyword} color={color} />
                  ))}
                </div>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
});

// Backward-compatible exports
/** @deprecated Use ContentBadge instead */
export const LLMContextBadge = ContentBadge;
export default ContentBadge;
