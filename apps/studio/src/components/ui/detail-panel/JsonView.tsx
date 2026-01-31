'use client';

/**
 * JsonView - JSON display with copy functionality
 *
 * Features:
 * - Pretty-printed JSON with syntax highlighting colors
 * - Copy button with feedback
 * - Configurable max height
 * - Scrollable with thin scrollbar
 */

import { memo } from 'react';
import { cn } from '@/lib/utils';
import { gapTokens } from '@/design/tokens';
import { CopyButton } from './CopyButton';

export interface JsonViewProps {
  data: unknown;
  onCopy?: () => void;
  isCopied?: boolean;
  maxHeight?: string;
  className?: string;
}

export const JsonView = memo(function JsonView({
  data,
  onCopy,
  isCopied = false,
  maxHeight = '400px',
  className,
}: JsonViewProps) {
  const jsonString = JSON.stringify(data, null, 2);

  return (
    <div className={cn('relative', className)}>
      <pre
        className={cn(
          'text-xs text-white/80 bg-[hsl(240,8%,5%)] p-4 rounded-lg overflow-x-auto',
          'border border-white/12 scrollbar-thin font-mono leading-relaxed'
        )}
        style={{ maxHeight }}
      >
        {jsonString}
      </pre>
      {onCopy && (
        <CopyButton
          onCopy={onCopy}
          isCopied={isCopied}
          label="Copy JSON to clipboard"
          size="sm"
          variant="filled"
          className="absolute top-2 right-2"
        />
      )}
    </div>
  );
});

// =============================================================================
// Compact JSON Toggle Section
// =============================================================================

export interface JsonToggleSectionProps {
  data: unknown;
  isOpen: boolean;
  onToggle: () => void;
  onCopy?: () => void;
  isCopied?: boolean;
  className?: string;
}

export const JsonToggleSection = memo(function JsonToggleSection({
  data,
  isOpen,
  onToggle,
  onCopy,
  isCopied = false,
  className,
}: JsonToggleSectionProps) {
  return (
    <div className={cn('p-4 border-t border-white/12', className)}>
      <button
        onClick={onToggle}
        className={cn(
          cn('w-full flex items-center justify-center px-4 py-2.5 rounded-xl text-xs font-medium', gapTokens.default),
          'transition-all duration-200',
          isOpen
            ? 'bg-primary/20 text-primary border border-primary/35'
            : 'bg-white/6 text-white/80 hover:bg-white/10 border border-white/12'
        )}
      >
        <span className="font-mono">{'{}'}</span>
        {isOpen ? 'Hide JSON' : 'View JSON'}
      </button>

      {isOpen && (
        <div className="mt-3">
          <JsonView
            data={data}
            onCopy={onCopy}
            isCopied={isCopied}
            maxHeight="200px"
          />
        </div>
      )}
    </div>
  );
});
