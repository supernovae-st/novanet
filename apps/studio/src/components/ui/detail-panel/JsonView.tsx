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
import { CopyButton } from '@/components/dx/CopyButton';

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

