'use client';

/**
 * AiSearchInput - Clickable trigger that opens AiSearchOverlay (⌘J)
 *
 * Self-contained: directly calls uiStore.openModal('ai-chat').
 * Looks like an input but functions as a button.
 */

import { Sparkles } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { glassClasses, iconSizes, gapTokens } from '@/design/tokens';
import { Kbd } from '@/components/ui/Kbd';

interface AiSearchInputProps {
  placeholder?: string;
  className?: string;
}

export function AiSearchInput({
  placeholder = 'Ask AI…',
  className,
}: AiSearchInputProps) {
  const openModal = useUIStore((s) => s.openModal);

  return (
    <button
      type="button"
      onClick={() => openModal('ai-chat')}
      aria-label="Open AI search (⌘J)"
      className={cn(
        'relative w-full flex items-center px-3 py-2.5',
        gapTokens.default,
        'rounded-xl border transition-all duration-200 text-left',
        glassClasses.light,
        'border-white/[0.08] hover:border-white/[0.15]',
        'hover:shadow-lg hover:shadow-novanet-500/5',
        'group cursor-pointer',
        className,
      )}
    >
      {/* Sparkle icon */}
      <div className="relative flex-shrink-0">
        <Sparkles
          className={cn(
            iconSizes.md,
            'text-white/40 group-hover:text-novanet-400 transition-colors duration-200',
          )}
        />
      </div>

      {/* Placeholder text */}
      <span className="flex-1 text-sm text-white/40 group-hover:text-white/50 transition-colors duration-200 truncate">
        {placeholder}
      </span>

      {/* Shortcut badge */}
      <Kbd className="flex-shrink-0">
        ⌘J
      </Kbd>
    </button>
  );
}
