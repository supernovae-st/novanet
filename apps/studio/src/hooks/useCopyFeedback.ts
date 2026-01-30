/**
 * useCopyFeedback Hook
 *
 * Handles clipboard copy with visual feedback and toast notifications.
 * Centralizes the copy-to-clipboard + timeout pattern.
 *
 * @example
 * const { copied, copy } = useCopyFeedback();
 * <button onClick={() => copy(text)}>
 *   {copied ? 'Copied!' : 'Copy'}
 * </button>
 *
 * // With toast notification
 * const { copied, copy } = useCopyFeedback({ showToast: true });
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { COPY_FEEDBACK_MS } from '@/config/constants';
import { logger } from '@/lib/logger';
import { toast } from '@/lib/toast';

export interface UseCopyFeedbackOptions {
  /** Duration to show feedback (default: COPY_FEEDBACK_MS) */
  duration?: number;
  /** Callback after successful copy */
  onCopy?: (text: string) => void;
  /** Show toast notification (default: false for backward compat) */
  showToast?: boolean;
  /** Custom toast message (default: 'Copied to clipboard') */
  toastMessage?: string;
}

export interface UseCopyFeedbackReturn {
  /** Whether copy feedback is currently showing */
  copied: boolean;
  /** Copy text to clipboard and show feedback */
  copy: (text: string) => Promise<void>;
  /** Reset copied state manually */
  reset: () => void;
}

export function useCopyFeedback(
  options: UseCopyFeedbackOptions = {}
): UseCopyFeedbackReturn {
  const { duration = COPY_FEEDBACK_MS, onCopy, showToast = false, toastMessage } = options;
  const [copied, setCopied] = useState(false);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  const reset = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    setCopied(false);
  }, []);

  const copy = useCallback(
    async (text: string) => {
      try {
        await navigator.clipboard.writeText(text);
        setCopied(true);
        onCopy?.(text);

        // Show toast notification if enabled
        if (showToast) {
          toast.copied(toastMessage);
        }

        // Clear any existing timeout
        if (timeoutRef.current) {
          clearTimeout(timeoutRef.current);
        }

        // Reset after duration
        timeoutRef.current = setTimeout(() => {
          setCopied(false);
          timeoutRef.current = null;
        }, duration);
      } catch (err) {
        logger.error('Clipboard', 'Failed to copy', err);
        if (showToast) {
          toast.error('Copy failed', 'Could not access clipboard');
        }
      }
    },
    [duration, onCopy, showToast, toastMessage]
  );

  return { copied, copy, reset };
}

/**
 * useCopyFeedback with field tracking
 *
 * For components that need to track which field was copied.
 *
 * @example
 * const { copiedField, copyField } = useCopyFieldFeedback();
 * <button onClick={() => copyField('id', node.id)}>
 *   {copiedField === 'id' ? 'Copied!' : 'Copy'}
 * </button>
 */
export interface UseCopyFieldFeedbackReturn {
  /** Which field was copied (null if none) */
  copiedField: string | null;
  /** Copy a specific field */
  copyField: (field: string, value: string) => Promise<void>;
  /** Reset copied state */
  reset: () => void;
}

export function useCopyFieldFeedback(
  options: UseCopyFeedbackOptions = {}
): UseCopyFieldFeedbackReturn {
  const { duration = COPY_FEEDBACK_MS, onCopy, showToast = false, toastMessage } = options;
  const [copiedField, setCopiedField] = useState<string | null>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  const reset = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    setCopiedField(null);
  }, []);

  const copyField = useCallback(
    async (field: string, value: string) => {
      try {
        await navigator.clipboard.writeText(value);
        setCopiedField(field);
        onCopy?.(value);

        // Show toast notification if enabled
        if (showToast) {
          toast.copied(toastMessage || field);
        }

        if (timeoutRef.current) {
          clearTimeout(timeoutRef.current);
        }

        timeoutRef.current = setTimeout(() => {
          setCopiedField(null);
          timeoutRef.current = null;
        }, duration);
      } catch (err) {
        logger.error('Clipboard', 'Failed to copy', err);
        if (showToast) {
          toast.error('Copy failed', 'Could not access clipboard');
        }
      }
    },
    [duration, onCopy, showToast, toastMessage]
  );

  return { copiedField, copyField, reset };
}
