'use client';

/**
 * AiSearchOverlay - Command palette style AI search (⌘J)
 *
 * Centered modal overlay for natural language → Cypher queries.
 * Follows the same design patterns as CommandPalette (⌘K).
 *
 * Features:
 * - Large search input with sparkle icon
 * - AI suggestion buttons (quick prompts)
 * - Chat messages with Cypher results
 * - Keyboard navigation (Enter, Escape)
 * - Uses chatStore for message persistence
 * - Glassmorphism design matching CommandPalette
 */

import { useState, useRef, useCallback, useEffect, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { postJSON, getErrorMessage } from '@/lib/fetchClient';
import { useChatStore } from '@/stores/chatStore';
import { CypherViewer } from '@/components/dx/CodeViewer';
import { useCopyFeedback, useAutoFocus } from '@/hooks';
import { useGridNavigation } from '@/hooks/useGridNavigation';
import { iconSizes, gapTokens, overlayClasses } from '@/design/tokens';
import { KeyboardKey } from '@/components/ui/KeyboardKey';
import { Kbd } from '@/components/ui/Kbd';
import { Modal } from '@/components/ui/Modal';
import { Sparkles } from 'lucide-react';
import {
  ACTION_ICONS,
  STATUS_ICONS,
} from '@/config/iconSystem';

// Design system icons
const SparklesIcon = Sparkles;
const SendIcon = ACTION_ICONS.send;
const ResetIcon = ACTION_ICONS.reset;
const LoaderIcon = STATUS_ICONS.loading;
const CopyIcon = ACTION_ICONS.copy;
const CheckIcon = STATUS_ICONS.success;
const PlayIcon = ACTION_ICONS.execute;

// =============================================================================
// Types
// =============================================================================

export interface AiSearchOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  onExecuteQuery?: (query: string) => void;
}

// =============================================================================
// Suggestions
// =============================================================================

const AI_SUGGESTIONS = [
  { text: 'Show all French locale components', query: 'Show me all French locale components' },
  { text: 'Find concepts linked to QR codes', query: 'Find concepts linked to QR codes' },
  { text: 'Count expressions per locale', query: 'How many expressions per locale?' },
  { text: 'List all BlockType nodes', query: 'List all BlockType nodes with their properties' },
  { text: 'Show Project → Locale connections', query: 'Show how Projects connect to Locales' },
  { text: 'Find nodes without relationships', query: 'Find any orphan nodes without relationships' },
] as const;

// =============================================================================
// Component
// =============================================================================

export const AiSearchOverlay = memo(function AiSearchOverlay({
  isOpen,
  onClose,
  onExecuteQuery,
}: AiSearchOverlayProps) {
  const [input, setInput] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const suggestionsRef = useRef<HTMLDivElement>(null);
  const isMountedRef = useRef(true);

  // Chat store
  const {
    messages,
    isLoading,
    error,
    addMessage,
    setLoading,
    setError,
    clearMessages,
  } = useChatStore(
    useShallow((state) => ({
      messages: state.messages,
      isLoading: state.isLoading,
      error: state.error,
      addMessage: state.addMessage,
      setLoading: state.setLoading,
      setError: state.setError,
      clearMessages: state.clearMessages,
    }))
  );

  // Track mounted state
  useEffect(() => {
    isMountedRef.current = true;
    return () => { isMountedRef.current = false; };
  }, []);

  // Auto-focus input when opened
  useAutoFocus(inputRef, isOpen);

  // Reset input when opened
  useEffect(() => {
    if (isOpen) {
      setInput('');
    }
  }, [isOpen]);

  // Scroll to bottom on new messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Handle send
  const handleSend = useCallback(async (queryOverride?: string) => {
    const userMessage = (queryOverride ?? input).trim();
    if (!userMessage || isLoading) return;

    setInput('');
    addMessage('user', userMessage);
    setLoading(true);

    try {
      interface ChatResponse {
        response: string;
        cypherQuery?: string;
        nodeCount?: number;
        duration?: number;
      }

      const data = await postJSON<ChatResponse>('/api/chat', { message: userMessage });
      if (!isMountedRef.current) return;

      addMessage('assistant', data.response, {
        cypherQuery: data.cypherQuery,
        nodeCount: data.nodeCount,
        duration: data.duration,
      });
      setLoading(false);
    } catch (err) {
      if (!isMountedRef.current) return;
      setError(getErrorMessage(err));
      setLoading(false);
    }
  }, [input, isLoading, addMessage, setLoading, setError]);

  // Handle key press
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSend();
      }
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
      }
    },
    [handleSend, onClose]
  );

  const hasMessages = messages.length > 0;

  // Grid navigation for suggestion buttons (2 columns, 6 items)
  const showSuggestions = !hasMessages && !isLoading;
  const { focusedIndex: sugFocused, handleKeyDown: sugKeyDown } = useGridNavigation({
    columns: 2,
    totalItems: AI_SUGGESTIONS.length,
    gridRef: suggestionsRef,
    onSelect: (index) => handleSend(AI_SUGGESTIONS[index].query),
    onEscape: () => inputRef.current?.focus(),
    enabled: showSuggestions && isOpen,
  });

  return (
    <Modal.Root
      isOpen={isOpen}
      onClose={onClose}
      closeOnEscape={false}
      containerClassName={overlayClasses.position}
    >
      <Modal.Content
        size={overlayClasses.size}
        ariaLabel="AI Search"
        className={overlayClasses.animation}
      >
        {/* Search Header */}
        <div className={cn(
          overlayClasses.searchHeader,
          gapTokens.spacious,
        )}>
          {/* Sparkle icon with gradient glow */}
          <div className="relative flex-shrink-0">
            <div
              className="absolute -inset-1.5 rounded-full blur-lg opacity-50"
              style={{ background: 'linear-gradient(135deg, #8b5cf6, #06b6d4)' }}
            />
            <div className="relative w-9 h-9 rounded-xl bg-gradient-to-br from-novanet-400 to-novanet-600 flex items-center justify-center shadow-lg shadow-novanet-500/20">
              <SparklesIcon className={cn(iconSizes.md, 'text-white')} />
            </div>
          </div>

          {/* Input */}
          <input
            ref={inputRef}
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask AI about the graph…"
            aria-label="AI search query"
            className={overlayClasses.searchInput}
            autoComplete="off"
            spellCheck={false}
          />

          {/* Right side: loading / send / shortcut */}
          <div className={cn('flex items-center flex-shrink-0', gapTokens.default)}>
            {isLoading ? (
              <LoaderIcon className={cn(iconSizes.md, 'text-novanet-400 animate-spin')} />
            ) : input.trim() ? (
              <button
                onClick={() => handleSend()}
                className="p-1.5 rounded-lg hover:bg-novanet-500/20 transition-colors"
                aria-label="Send query"
              >
                <SendIcon className={cn(iconSizes.md, 'text-novanet-400')} />
              </button>
            ) : (
              <KeyboardKey size="md" className="hidden sm:inline-flex">⌘J</KeyboardKey>
            )}
          </div>
        </div>

        {/* Body */}
        <Modal.Body maxHeight={overlayClasses.bodyMaxHeight}>
          <div className={cn('p-3', overlayClasses.contentAnimation)}>
            {/* Empty state: suggestions */}
            {!hasMessages && !isLoading && (
              <div className="space-y-3">
                {/* Suggestions header */}
                <div className={overlayClasses.sectionHeader}>
                  Suggestions
                </div>

                {/* Suggestion grid with arrow key navigation */}
                <div
                  ref={suggestionsRef}
                  role="grid"
                  aria-label="AI query suggestions"
                  onKeyDown={sugKeyDown}
                  className="grid grid-cols-2 gap-2"
                >
                  {AI_SUGGESTIONS.map((suggestion, index) => {
                    const isFocused = sugFocused === index;
                    return (
                      <button
                        key={suggestion.text}
                        tabIndex={isFocused ? 0 : -1}
                        onClick={() => handleSend(suggestion.query)}
                        className={cn(
                          'text-left text-sm px-3.5 py-3 rounded-xl',
                          'transition-colors duration-150',
                          'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-novanet-500/50',
                          isFocused
                            ? 'bg-novanet-500/10 text-novanet-300 border border-novanet-500/20'
                            : 'bg-white/[0.03] text-white/60 border border-white/[0.06] hover:bg-novanet-500/10 hover:text-novanet-300 hover:border-novanet-500/20',
                        )}
                      >
                        <span className="text-white/30 mr-1.5">→</span>
                        {suggestion.text}
                      </button>
                    );
                  })}
                </div>
              </div>
            )}

            {/* Messages */}
            {hasMessages && (
              <div className="space-y-3">
                {/* Clear button */}
                <div className="flex items-center justify-between px-2">
                  <span className={overlayClasses.sectionHeader}>
                    Conversation
                  </span>
                  <button
                    onClick={clearMessages}
                    className={cn(
                      'flex items-center text-xs text-white/40 hover:text-white/70 px-2 py-1 rounded-lg',
                      'hover:bg-white/[0.04] transition-colors',
                      gapTokens.compact,
                    )}
                    aria-label="Clear conversation"
                  >
                    <ResetIcon className={iconSizes.xs} />
                    Clear
                  </button>
                </div>

                {/* Message list */}
                {messages.map((message) => (
                  <OverlayMessage
                    key={message.id}
                    message={message}
                    onExecuteQuery={onExecuteQuery}
                  />
                ))}

                {/* Loading indicator */}
                {isLoading && (
                  <div
                    role="status"
                    aria-live="polite"
                    className={cn(
                      'flex items-center text-novanet-400 animate-in fade-in duration-200 px-2 py-2',
                      gapTokens.comfortable,
                    )}
                  >
                    <LoaderIcon className={cn(iconSizes.md, 'animate-spin')} />
                    <span className="text-sm">Generating query…</span>
                  </div>
                )}

                {/* Error */}
                {error && (
                  <div role="alert" className="text-red-400 text-sm bg-red-500/10 border border-red-500/20 px-3 py-2 rounded-lg animate-in fade-in slide-in-from-bottom-2 duration-200">
                    {error}
                  </div>
                )}

                <div ref={messagesEndRef} />
              </div>
            )}
          </div>
        </Modal.Body>

        {/* Footer */}
        <Modal.Footer className={overlayClasses.footer}>
          <div className={cn('flex items-center justify-between text-xs text-white/50')}>
            <span className="text-white/30">
              Natural language → Cypher
            </span>
            <div className={cn('flex items-center', gapTokens.large)}>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>↵</Kbd>
                <span>Send</span>
              </span>
              <span className={cn('flex items-center', gapTokens.compact)}>
                <Kbd>Esc</Kbd>
                <span>Close</span>
              </span>
            </div>
          </div>
        </Modal.Footer>
      </Modal.Content>
    </Modal.Root>
  );
});

// =============================================================================
// OverlayMessage - Single message in the overlay
// =============================================================================

interface OverlayMessageProps {
  message: {
    id: string;
    role: 'user' | 'assistant';
    content: string;
    metadata?: {
      cypherQuery?: string;
      nodeCount?: number;
      duration?: number;
    };
  };
  onExecuteQuery?: (query: string) => void;
}

const OverlayMessage = memo(function OverlayMessage({
  message,
  onExecuteQuery,
}: OverlayMessageProps) {
  const { copied: copiedQuery, copy } = useCopyFeedback();

  const handleCopyQuery = async () => {
    if (message.metadata?.cypherQuery) {
      try {
        await copy(message.metadata.cypherQuery);
      } catch (err) {
        logger.error('AiSearchOverlay', 'Failed to copy query', err);
      }
    }
  };

  return (
    <div
      className={cn(
        'flex flex-col animate-stagger-fade-in',
        gapTokens.default,
        message.role === 'user' ? 'items-end' : 'items-start',
      )}
    >
      {/* Message bubble */}
      <div
        className={cn(
          'max-w-[85%] rounded-xl px-4 py-2.5 text-sm',
          message.role === 'user'
            ? 'bg-novanet-500/20 text-novanet-100 border border-novanet-500/30'
            : 'bg-white/[0.03] text-white/80 border border-white/[0.06]',
        )}
      >
        <p className="whitespace-pre-wrap leading-relaxed">{message.content}</p>
      </div>

      {/* Cypher query display */}
      {message.metadata?.cypherQuery && (
        <div className="w-full max-w-[85%] space-y-2">
          <CypherViewer
            query={message.metadata.cypherQuery}
            title="Generated Cypher"
            className="text-xs"
          />

          <div className={cn('flex items-center', gapTokens.default)}>
            <button
              onClick={() => {
                if (message.metadata?.cypherQuery) {
                  onExecuteQuery?.(message.metadata.cypherQuery);
                }
              }}
              aria-label="Execute Cypher query"
              className={cn(
                'flex items-center text-xs px-3 py-1.5 rounded-lg',
                gapTokens.compact,
                'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30',
                'hover:bg-emerald-500/30 transition-colors',
              )}
            >
              <PlayIcon className={iconSizes.xs} />
              Execute
            </button>
            <button
              onClick={handleCopyQuery}
              aria-label={copiedQuery ? 'Query copied' : 'Copy Cypher query'}
              className={cn(
                'flex items-center text-xs px-3 py-1.5 rounded-lg',
                gapTokens.compact,
                'bg-white/5 text-white/50 border border-white/10',
                'hover:bg-white/10 hover:text-white/70 transition-colors',
              )}
            >
              {copiedQuery ? <CheckIcon className={iconSizes.xs} /> : <CopyIcon className={iconSizes.xs} />}
              {copiedQuery ? 'Copied' : 'Copy'}
            </button>
          </div>

          {/* Stats */}
          {(message.metadata.nodeCount !== undefined || message.metadata.duration !== undefined) && (
            <div className={cn('flex items-center text-[10px] text-white/40 px-1', gapTokens.spacious)}>
              {message.metadata.nodeCount !== undefined && (
                <span className={cn('flex items-center', gapTokens.tight)}>
                  <span className="w-1.5 h-1.5 rounded-full bg-indigo-400/50" />
                  {message.metadata.nodeCount} nodes
                </span>
              )}
              {message.metadata.duration !== undefined && (
                <span className={cn('flex items-center', gapTokens.tight)}>
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400/50" />
                  {message.metadata.duration}ms
                </span>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
});
