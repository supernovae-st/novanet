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
  /** Text to copy */
  text?: string;
  /** Object to copy as JSON */
  json?: Record<string, unknown>;
  /** Format for copying */
  format?: 'text' | 'json' | 'typescript' | 'yaml';
  /** Size variant */
  size?: 'sm' | 'md';
  /** Additional class names */
  className?: string;
  /** Label to show */
  label?: string;
  /** Show label */
  showLabel?: boolean;
}

/**
 * Copy button with visual feedback
 * Adapted from Nika Studio pattern
 */
export const CopyButton = memo(function CopyButton({
  text,
  json,
  format = 'text',
  size = 'sm',
  className,
  label,
  showLabel = false,
}: CopyButtonProps) {
  const [copied, setCopied] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Cleanup timer on unmount to prevent memory leak
  useEffect(() => {
    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }
    };
  }, []);

  const handleCopy = useCallback(async () => {
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
        setCopied(true);

        // Clear previous timer
        if (timerRef.current) {
          clearTimeout(timerRef.current);
        }

        // Reset after feedback duration
        timerRef.current = setTimeout(() => {
          setCopied(false);
        }, COPY_FEEDBACK_MS);
      }
    } catch (error) {
      // Logger handles environment-aware logging
      logger.error('Clipboard', 'Copy failed', error);
    }
  }, [text, json, format]);

  const iconSize = size === 'sm' ? iconSizes.md : iconSizes.xl;
  const buttonSize = size === 'sm' ? 'p-2' : 'p-2.5'; // Min 44px touch target

  const Icon = format === 'typescript' ? CodeIcon : format === 'json' ? JsonIcon : CopyIcon;

  return (
    <button
      onClick={handleCopy}
      className={cn(
        'inline-flex items-center rounded transition-all',
        gapTokens.compact,
        buttonSize,
        copied
          ? 'bg-emerald-500/20 text-emerald-400'
          : 'text-white/50 hover:text-white hover:bg-white/10',
        className
      )}
      title={label || (copied ? 'Copied!' : `Copy as ${format}`)}
      aria-label={label || (copied ? 'Copied!' : `Copy as ${format}`)}
    >
      {copied ? (
        <CheckIcon className={iconSize} />
      ) : (
        <Icon className={iconSize} />
      )}
      {showLabel && (
        <span className="text-xs">
          {copied ? 'Copied!' : label || 'Copy'}
        </span>
      )}
    </button>
  );
});

/**
 * Copy button group with multiple format options
 */
export interface CopyButtonGroupProps {
  properties: Record<string, unknown>;
  className?: string;
}

export const CopyButtonGroup = memo(function CopyButtonGroup({
  properties,
  className,
}: CopyButtonGroupProps) {
  return (
    <div className={cn('flex items-center', gapTokens.tight, className)}>
      <CopyButton
        json={properties}
        format="json"
        label="JSON"
        size="sm"
      />
      <CopyButton
        json={properties}
        format="typescript"
        label="TypeScript"
        size="sm"
      />
      <CopyButton
        json={properties}
        format="yaml"
        label="YAML"
        size="sm"
      />
    </div>
  );
});
