'use client';

/**
 * FilterPanel - Combined filter panel with AI search, ViewPresetSelector and LabelFilter
 *
 * Features:
 * - AI Search Input for natural language → Cypher queries
 * - ViewPresetSelector for quick preset selection with keyboard shortcuts
 * - LabelFilter for granular type filtering
 * - Glassmorphism design matching existing sidebar panels
 */

import { memo, useCallback } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { SlidersHorizontal } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useAiQueryStore } from '@/stores/aiQueryStore';
import { AiSearchInput } from './AiSearchInput';
import { ViewPresetSelector } from './ViewPresetSelector';
import { LabelFilter } from './LabelFilter';

export interface FilterPanelProps {
  className?: string;
}

export const FilterPanel = memo(function FilterPanel({ className }: FilterPanelProps) {
  const { submitAiQuery, isProcessing } = useAiQueryStore(
    useShallow((state) => ({
      submitAiQuery: state.submitAiQuery,
      isProcessing: state.isProcessing,
    }))
  );

  const handleAiSubmit = useCallback(
    async (question: string) => {
      await submitAiQuery(question);
    },
    [submitAiQuery]
  );

  return (
    <div
      className={cn(
        'h-full flex flex-col bg-gradient-to-b from-black/60 to-black/40 backdrop-blur-xl',
        className
      )}
      data-testid="filter-panel"
    >
      {/* Header - Premium Glassmorphism */}
      <div className="relative px-4 py-5 border-b border-white/[0.08]">
        {/* Background glow effect */}
        <div
          className={cn(
            'absolute inset-0 bg-gradient-to-br pointer-events-none',
            'from-indigo-500/5 via-transparent to-purple-500/5'
          )}
        />

        <div className="relative flex items-center gap-3">
          {/* Icon with animated gradient */}
          <div className="relative">
            <div
              className={cn(
                'absolute inset-0 rounded-2xl bg-gradient-to-br',
                'from-indigo-400 to-purple-500 opacity-20 blur-lg'
              )}
            />
            <div
              className={cn(
                'relative w-11 h-11 rounded-2xl bg-gradient-to-br flex items-center justify-center',
                'from-indigo-500/20 to-purple-500/20 border border-white/10',
                'shadow-lg shadow-black/20'
              )}
            >
              <SlidersHorizontal className="w-5 h-5 text-indigo-400" />
            </div>
          </div>

          <div className="flex-1">
            <h2 className="text-[15px] font-semibold text-white tracking-tight">
              Filter Panel
            </h2>
            <p className="text-[11px] text-white/40 mt-0.5">
              Presets &amp; label filters
            </p>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto scrollbar-thin">
        <div className="p-3 space-y-6">
          {/* AI Search Input */}
          <AiSearchInput
            onSubmit={handleAiSubmit}
            isLoading={isProcessing}
            placeholder="Ask AI to query the graph..."
          />

          {/* Divider */}
          <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent" />

          {/* View Preset Selector - Quick Views with keyboard shortcuts */}
          <ViewPresetSelector />

          {/* Divider */}
          <div className="h-px bg-gradient-to-r from-transparent via-white/10 to-transparent" />

          {/* Label Filter Section */}
          <LabelFilter />
        </div>
      </div>
    </div>
  );
});
