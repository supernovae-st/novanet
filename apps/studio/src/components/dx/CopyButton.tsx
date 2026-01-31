'use client';

import { useState, useCallback, useRef, memo, useEffect } from 'react';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { copyToClipboard, copyNodeProperties } from '@/lib/clipboard';
import { COPY_FEEDBACK_MS } from '@/config/constants';
import { ACTION_ICONS, STATUS_ICONS, CONTENT_ICONS } from '@/config/iconSystem';
import { iconSizes, gapTokens } from '@/design/tokens';

// Design system icons
const CheckIcon = STATUS_ICONS.success;
const CopyIcon = ACTION_ICONS.copy;
const CodeIcon = CONTENT_ICONS.code;
const JsonIcon = CONTENT_ICONS.json;

export interface CopyButtonProps {
  /** Text to copy (self-contained mode) */
  text?: string;
  /** Object to copy as JSON (self-contained mode) */
  json?: Record<string, unknown>;
  /** Callback when copy is clicked (controlled mode) */
  onCopy?: () => void;
  /** External copied state (controlled mode) */
  isCopied?: boolean;
  /** Format for copying */
  format?: 'text' | 'json' | 'typescript' | 'yaml';
  /** Size variant */
  size?: 'sm' | 'md' | 'lg';
  /** Visual variant */
  variant?: 'ghost' | 'filled';
  /** Additional class names */
  className?: string;
  /** Label to show */
  label?: string;
  /** Show label */
  showLabel?: boolean;
}

// Touch target sizes (WCAG 2.1 AA: minimum 44×44px)
const sizeClasses = {
  sm: 'p-2.5 min-w-[44px] min-h-[44px]',   // 40px padding → 44px touch
  md: 'p-3 min-w-[48px] min-h-[48px]',     // 48px
  lg: 'p-3.5 min-w-[52px] min-h-[52px]',   // 52px
};

const sizeIconClasses = {
  sm: iconSizes.md,
  md: iconSizes.lg,
  lg: iconSizes.xl,
};

/**
 * Copy button with visual feedback
 *
 * Supports two modes:
 * 1. Self-contained: Pass `text` or `json` - manages copying internally
 * 2. Controlled: Pass `onCopy` and `isCopied` - for external state management
 *
 * @example Self-contained
 * ```tsx
 * <CopyButton text="Hello" />
 * <CopyButton json={{ foo: 'bar' }} format="json" />
 * ```
 *
 * @example Controlled
 * ```tsx
 * <CopyButton onCopy={() => copyField('key')} isCopied={copiedField === 'key'} />
 * ```
 */
export const CopyButton = memo(function CopyButton({
  text,
  json,
  onCopy,
  isCopied: externalIsCopied,
  format = 'text',
  size = 'sm',
  variant = 'ghost',
  className,
  label,
  showLabel = false,
}: CopyButtonProps) {
  // Internal state for self-contained mode
  const [internalCopied, setInternalCopied] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Use external state if provided (controlled mode), otherwise internal
  const isControlled = onCopy !== undefined;
  const copied = isControlled ? (externalIsCopied ?? false) : internalCopied;

  // Cleanup timer on unmount to prevent memory leak
  useEffect(() => {
    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }
    };
  }, []);

  const handleCopy = useCallback(async () => {
    // Controlled mode: just call the callback
    if (isControlled) {
      onCopy?.();
      return;
    }

    // Self-contained mode: handle copying internally
    try {
      let success = false;

      if (json) {
        if (format === 'text' || format === 'json') {
          success = await copyToClipboard(JSON.stringify(json, null, 2));
        } else {
          success = await copyNodeProperties(json, format as 'typescript' | 'yaml');
        }
      } else if (text) {
        success = await copyToClipboard(text);
      }

      if (success) {
        setInternalCopied(true);

        // Clear previous timer
        if (timerRef.current) {
          clearTimeout(timerRef.current);
        }

        // Reset after feedback duration
        timerRef.current = setTimeout(() => {
          setInternalCopied(false);
        }, COPY_FEEDBACK_MS);
      }
    } catch (error) {
      logger.error('Clipboard', 'Copy failed', error);
    }
  }, [text, json, format, isControlled, onCopy]);

  const Icon = format === 'typescript' ? CodeIcon : format === 'json' ? JsonIcon : CopyIcon;

  // Variant styling
  const variantClasses = copied
    ? 'bg-emerald-500/20 text-emerald-400 scale-105'
    : variant === 'ghost'
      ? 'text-white/50 hover:text-white hover:bg-white/10 focus-visible:bg-white/10'
      : 'bg-white/10 hover:bg-white/20 text-white/50 hover:text-white/70';

  return (
    <>
      <button
        type="button"
        onClick={handleCopy}
        className={cn(
          // Base styles
          'inline-flex items-center justify-center rounded-lg transition-all duration-200',
          // Focus visible ring (keyboard navigation)
          'focus:outline-none focus-visible:ring-2 focus-visible:ring-novanet-accent focus-visible:ring-offset-2 focus-visible:ring-offset-black',
          gapTokens.compact,
          sizeClasses[size],
          variantClasses,
          className
        )}
        title={label || (copied ? 'Copied!' : `Copy as ${format}`)}
        aria-label={label || `Copy as ${format}`}
      >
        {copied ? (
          <CheckIcon className={sizeIconClasses[size]} aria-hidden="true" />
        ) : (
          <Icon className={sizeIconClasses[size]} aria-hidden="true" />
        )}
        {showLabel && (
          <span className="text-xs">
            {copied ? 'Copied!' : label || 'Copy'}
          </span>
        )}
      </button>
      {/* ARIA live region for screen reader announcements */}
      <span
        role="status"
        aria-live="polite"
        aria-atomic="true"
        className="sr-only"
      >
        {copied ? 'Copied to clipboard' : ''}
      </span>
    </>
  );
});

