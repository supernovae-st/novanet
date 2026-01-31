'use client';

/**
 * AiSearchInput - Natural language query input with AI sparkle
 *
 * Features:
 * - Sparkle icon with glow animation
 * - Keyboard shortcut indicator (⌘J)
 * - Animated icon transition (sparkle → send)
 * - Loading state with spinner
 * - Glassmorphism design
 */

import { useState, useRef, useCallback } from 'react';
import { Sparkles, Send, Loader2 } from 'lucide-react';
import { logger } from '@/lib/logger';
import { cn } from '@/lib/utils';
import { glassClasses } from '@/design/tokens';
import { Kbd } from '@/components/ui/Kbd';

interface AiSearchInputProps {
  onSubmit: (query: string) => Promise<void>;
  isLoading?: boolean;
  placeholder?: string;
  className?: string;
}

export function AiSearchInput({
  onSubmit,
  isLoading = false,
  placeholder = 'Ask AI...',
  className,
}: AiSearchInputProps) {
  const [query, setQuery] = useState('');
  const [isFocused, setIsFocused] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = useCallback(async () => {
    const trimmed = query.trim();
    if (!trimmed || isLoading) return;

    try {
      await onSubmit(trimmed);
      setQuery('');
    } catch (error) {
      logger.error('AiSearchInput', 'AI query failed', error);
    }
  }, [query, isLoading, onSubmit]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  const hasQuery = query.trim().length > 0;

  return (
    <div className={cn('relative', className)}>
      {/* Input container with glassmorphism */}
      <div
        className={cn(
          'relative flex items-center gap-2 px-3 py-2.5',
          'rounded-xl border transition-all duration-300',
          glassClasses.light,
          isFocused
            ? 'border-novanet-500/50 shadow-lg shadow-novanet-500/10'
            : 'border-white/[0.08] hover:border-white/[0.12]'
        )}
      >
        {/* Sparkle icon with glow */}
        <div className="relative flex-shrink-0">
          <div
            className={cn(
              'absolute -inset-1 rounded-full blur-md transition-opacity duration-300',
              isFocused || hasQuery ? 'opacity-60' : 'opacity-0'
            )}
            style={{ background: 'linear-gradient(135deg, #8b5cf6, #06b6d4)' }}
          />
          <Sparkles
            className={cn(
              'relative w-4 h-4 transition-all duration-300',
              isFocused || hasQuery
                ? 'text-novanet-400 ai-sparkle-glow'
                : 'text-white/40'
            )}
          />
        </div>

        {/* Input field */}
        <input
          ref={inputRef}
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          disabled={isLoading}
          className={cn(
            'flex-1 bg-transparent text-sm text-white/90 placeholder:text-white/30',
            'outline-none border-none ring-0 focus:outline-none focus:ring-0',
            'disabled:opacity-50 disabled:cursor-not-allowed'
          )}
        />

        {/* Right side: Loading / Send / Shortcut */}
        <div className="flex items-center gap-1.5 flex-shrink-0">
          {isLoading ? (
            <Loader2 className="w-4 h-4 text-novanet-400 animate-spin" />
          ) : hasQuery ? (
            <button
              onClick={handleSubmit}
              className={cn(
                'p-1 rounded-md transition-all duration-200',
                'hover:bg-novanet-500/20 active:scale-95'
              )}
              aria-label="Send query"
            >
              <Send className="w-3.5 h-3.5 text-novanet-400" />
            </button>
          ) : (
            <Kbd className={cn('transition-opacity duration-200', isFocused ? 'opacity-0' : 'opacity-100')}>
              ⌘J
            </Kbd>
          )}
        </div>
      </div>

      {/* CSS Animations */}
      <style jsx>{`
        .ai-sparkle-glow {
          filter: drop-shadow(0 0 6px rgba(139, 92, 246, 0.6));
          animation: sparkleGlow 2s ease-in-out infinite;
        }

        @keyframes sparkleGlow {
          0%, 100% {
            filter: drop-shadow(0 0 6px rgba(139, 92, 246, 0.6));
          }
          50% {
            filter: drop-shadow(0 0 10px rgba(139, 92, 246, 0.8));
          }
        }
      `}</style>
    </div>
  );
}
