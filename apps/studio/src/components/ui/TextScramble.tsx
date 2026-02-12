'use client';

/**
 * TextScramble - Matrix-style text decode animation
 *
 * When text changes, shows random characters that resolve into the actual text.
 * Inspired by the classic "hacker" text effect.
 *
 * Usage:
 * <TextScramble text={nodeTitle} />
 * <TextScramble text={nodeKey} className="text-sm text-white/50" />
 */

import { memo, useEffect, useState, useRef, useCallback } from 'react';
import { cn } from '@/lib/utils';

// Matrix-style characters for the scramble effect
const CHARS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@#$%&*+-=[]{}|;:,.<>?';

interface TextScrambleProps {
  /** The text to display */
  text: string;
  /** Additional CSS classes */
  className?: string;
  /** Duration in ms (default: 300) */
  duration?: number;
  /** Delay before starting (default: 0) */
  delay?: number;
  /** Whether to trigger on mount (default: true) */
  animateOnMount?: boolean;
  /** Callback when animation completes */
  onComplete?: () => void;
}

export const TextScramble = memo(function TextScramble({
  text,
  className,
  duration = 300,
  delay = 0,
  animateOnMount = true,
  onComplete,
}: TextScrambleProps) {
  const [displayText, setDisplayText] = useState(animateOnMount ? '' : text);
  const [isAnimating, setIsAnimating] = useState(false);
  const previousTextRef = useRef<string>(text);
  const frameRef = useRef<number | undefined>(undefined);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  const animate = useCallback((targetText: string, animDuration: number) => {
    setIsAnimating(true);
    const startTime = performance.now();
    const targetLength = targetText.length;

    // Resolve characters progressively from left to right
    const frame = (currentTime: number) => {
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / animDuration, 1);

      // Number of characters that should be resolved
      const resolvedCount = Math.floor(progress * targetLength);

      // Build the display string
      let result = '';
      for (let i = 0; i < targetLength; i++) {
        if (i < resolvedCount) {
          // This character is resolved - show actual character
          result += targetText[i];
        } else {
          // This character is still scrambling
          result += CHARS[Math.floor(Math.random() * CHARS.length)];
        }
      }

      setDisplayText(result);

      if (progress < 1) {
        frameRef.current = requestAnimationFrame(frame);
      } else {
        setDisplayText(targetText);
        setIsAnimating(false);
        onComplete?.();
      }
    };

    frameRef.current = requestAnimationFrame(frame);
  }, [onComplete]);

  // Handle text changes
  useEffect(() => {
    // Skip if text hasn't changed
    if (text === previousTextRef.current && !animateOnMount) {
      return;
    }

    // Skip animation if text is empty
    if (!text) {
      setDisplayText('');
      previousTextRef.current = text;
      return;
    }

    // Cancel any ongoing animation
    if (frameRef.current) {
      cancelAnimationFrame(frameRef.current);
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    // Apply delay if specified
    if (delay > 0) {
      timeoutRef.current = setTimeout(() => {
        animate(text, duration);
      }, delay);
    } else {
      animate(text, duration);
    }

    previousTextRef.current = text;

    return () => {
      if (frameRef.current) {
        cancelAnimationFrame(frameRef.current);
      }
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [text, duration, delay, animate, animateOnMount]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (frameRef.current) {
        cancelAnimationFrame(frameRef.current);
      }
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return (
    <span
      className={cn(
        'inline-block font-mono',
        isAnimating && 'select-none',
        className
      )}
    >
      {displayText || '\u00A0'} {/* Non-breaking space to maintain height */}
    </span>
  );
});

export default TextScramble;
