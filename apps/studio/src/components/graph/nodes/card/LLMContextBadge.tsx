'use client';

/**
 * LLMContextBadge - Visual display of llm_context from node YAML
 *
 * Parses and displays the USE/TRIGGERS/NOT/RELATES pattern (ADR-027)
 * for AI-friendly graph context loading.
 *
 * Display modes:
 * - compact: Purpose badge + trigger count only
 * - expanded: Full context with all sections
 *
 * Layout (compact):
 * ┌─────────────────────────────────────────────────────┐
 * │  ⚡ load locale-specific content  │  5 triggers    │
 * └─────────────────────────────────────────────────────┘
 *
 * Layout (expanded):
 * ┌─────────────────────────────────────────────────────┐
 * │  USE: when loading locale-specific content...      │
 * │  ─────────────────────────────────────────────────  │
 * │  TRIGGERS: content • native • locale • l10n        │
 * │  ─────────────────────────────────────────────────  │
 * │  NOT: for structure (use HAS_BLOCK)                │
 * │  ─────────────────────────────────────────────────  │
 * │  RELATES: Entity (parent) → EntityNative (content) │
 * └─────────────────────────────────────────────────────┘
 */

import { memo, useMemo, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@/lib/utils';
import { Zap, ChevronDown, AlertTriangle, Link2 } from 'lucide-react';
import { gapTokens } from '@/design/tokens';

// =============================================================================
// Types
// =============================================================================

export interface ParsedLLMContext {
  /** USE: when to load this node into context */
  use?: string;
  /** TRIGGERS: keywords for RAG activation */
  triggers?: string[];
  /** NOT: disambiguation - what NOT to use this for */
  not?: string;
  /** RELATES: graph topology with roles */
  relates?: string;
}

export interface LLMContextBadgeProps {
  /** Raw llm_context string from YAML */
  llmContext?: string;
  /** Pre-parsed context (if already parsed) */
  parsedContext?: ParsedLLMContext;
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
// Parser
// =============================================================================

/**
 * Parse llm_context string into structured object
 *
 * Expected format:
 * ```
 * USE: when [primary use case].
 * TRIGGERS: "keyword1", "keyword2", "keyword3".
 * NOT: for [disambiguation] (use [alternative] instead).
 * RELATES: [Source] (role), [Target] (role).
 * ```
 */
export function parseLLMContext(raw: string): ParsedLLMContext {
  const result: ParsedLLMContext = {};

  // Extract USE clause
  const useMatch = raw.match(/USE:\s*(.+?)(?=\n|TRIGGERS:|NOT:|RELATES:|$)/is);
  if (useMatch) {
    result.use = useMatch[1].trim().replace(/\.$/, '');
  }

  // Extract TRIGGERS
  const triggersMatch = raw.match(/TRIGGERS:\s*(.+?)(?=\n|USE:|NOT:|RELATES:|$)/is);
  if (triggersMatch) {
    // Parse quoted keywords or comma-separated
    const triggersStr = triggersMatch[1].trim();
    const keywords = triggersStr
      .split(/[,،]/) // Support both comma types
      .map(k => k.trim().replace(/^["']|["']$/g, '').replace(/\.$/, ''))
      .filter(k => k.length > 0);
    result.triggers = keywords;
  }

  // Extract NOT clause
  const notMatch = raw.match(/NOT:\s*(.+?)(?=\n|USE:|TRIGGERS:|RELATES:|$)/is);
  if (notMatch) {
    result.not = notMatch[1].trim().replace(/\.$/, '');
  }

  // Extract RELATES clause
  const relatesMatch = raw.match(/RELATES:\s*(.+?)(?=\n|USE:|TRIGGERS:|NOT:|$)/is);
  if (relatesMatch) {
    result.relates = relatesMatch[1].trim().replace(/\.$/, '');
  }

  return result;
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

const ContextSection = memo(function ContextSection({
  label,
  icon: Icon,
  content,
  color,
}: {
  label: string;
  icon: typeof Zap;
  content: React.ReactNode;
  color: string;
}) {
  return (
    <div className="space-y-1">
      <div className="flex items-center gap-1.5">
        <Icon size={10} color={color} />
        <span className="text-[9px] font-semibold uppercase tracking-wider" style={{ color }}>
          {label}
        </span>
      </div>
      <div className="text-[10px] text-white/70 pl-4">
        {content}
      </div>
    </div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const LLMContextBadge = memo(function LLMContextBadge({
  llmContext,
  parsedContext: providedContext,
  mode: initialMode = 'compact',
  color = '#8b5cf6',
  selected = false,
  isHovered = false,
  expandable = true,
}: LLMContextBadgeProps) {
  const [isExpanded, setIsExpanded] = useState(initialMode === 'expanded');

  // Parse context if not provided
  const context = useMemo(() => {
    if (providedContext) return providedContext;
    if (!llmContext) return {};
    return parseLLMContext(llmContext);
  }, [llmContext, providedContext]);

  // Nothing to display
  if (!context.use && !context.triggers?.length) {
    return null;
  }

  const triggerCount = context.triggers?.length ?? 0;
  const showExpandToggle = expandable && (context.not || context.relates || triggerCount > 3);

  return (
    <motion.div
      className={cn(
        'rounded-lg overflow-hidden transition-all duration-200',
        (selected || isHovered) && 'ring-1'
      )}
      style={{
        background: `${color}08`,
        border: `1px solid ${color}20`,
        // Ring color via box-shadow since ringColor isn't a valid CSS prop
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
          {context.use && (
            <span className="text-[10px] text-white/80 truncate max-w-[180px]">
              {context.use}
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

      {/* Expanded content */}
      <AnimatePresence>
        {isExpanded && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2 }}
            className="overflow-hidden"
          >
            <div
              className="px-2 py-2 space-y-2 border-t"
              style={{ borderColor: `${color}15` }}
            >
              {/* TRIGGERS as chips */}
              {context.triggers && context.triggers.length > 0 && (
                <ContextSection
                  label="Triggers"
                  icon={Zap}
                  color={color}
                  content={
                    <div className={cn('flex flex-wrap', gapTokens.compact)}>
                      {context.triggers.map((keyword, i) => (
                        <TriggerChip key={i} keyword={keyword} color={color} />
                      ))}
                    </div>
                  }
                />
              )}

              {/* NOT clause */}
              {context.not && (
                <ContextSection
                  label="Not For"
                  icon={AlertTriangle}
                  color="#f59e0b"
                  content={context.not}
                />
              )}

              {/* RELATES clause */}
              {context.relates && (
                <ContextSection
                  label="Relates"
                  icon={Link2}
                  color="#22c55e"
                  content={context.relates}
                />
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
});

export default LLMContextBadge;
