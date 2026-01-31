'use client';

import { useState, useRef, useEffect, useCallback, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { postJSON, getErrorMessage } from '@/lib/fetchClient';
import { useChatStore } from '@/stores/chatStore';
import { CypherViewer } from '@/components/dx/CodeViewer';
import { useCopyFeedback } from '@/hooks';
import { glassClasses, iconSizes, gapTokens } from '@/design/tokens';
import { Kbd } from '@/components/ui/Kbd';
import {
  ACTION_ICONS,
  STATUS_ICONS,
  DOMAIN_ICONS,
} from '@/config/iconSystem';

// Design system icons
const SendIcon = ACTION_ICONS.send;
const SparklesIcon = DOMAIN_ICONS.ai;
const CloseIcon = ACTION_ICONS.close;
const ResetIcon = ACTION_ICONS.reset;
const LoaderIcon = STATUS_ICONS.loading;
const CopyIcon = ACTION_ICONS.copy;
const CheckIcon = STATUS_ICONS.success;
const PlayIcon = ACTION_ICONS.execute;

export interface AiChatProps {
  /** Whether the chat is open */
  isOpen: boolean;
  /** Close handler */
  onClose: () => void;
  /** Execute Cypher query callback */
  onExecuteQuery?: (query: string) => void;
  /** Additional class names */
  className?: string;
}

/**
 * AI Chat interface for natural language graph queries
 * Uses Claude API to translate natural language to Cypher
 */
export const AiChat = memo(function AiChat({
  isOpen,
  onClose,
  onExecuteQuery,
  className,
}: AiChatProps) {
  const [input, setInput] = useState('');
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Track mounted state to avoid updating after unmount
  const isMountedRef = useRef(true);
  useEffect(() => {
    isMountedRef.current = true;
    return () => {
      isMountedRef.current = false;
    };
  }, []);

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

  // Auto-focus input when opened
  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isOpen]);

  // Scroll to bottom on new messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Handle send message
  const handleSend = useCallback(async () => {
    if (!input.trim() || isLoading) return;

    const userMessage = input.trim();
    setInput('');
    addMessage('user', userMessage);
    setLoading(true);

    try {
      // Call AI API to generate Cypher query
      interface ChatResponse {
        response: string;
        cypherQuery?: string;
        nodeCount?: number;
        duration?: number;
      }

      const data = await postJSON<ChatResponse>('/api/chat', { message: userMessage });

      // Check if component is still mounted before updating state
      if (!isMountedRef.current) return;

      addMessage('assistant', data.response, {
        cypherQuery: data.cypherQuery,
        nodeCount: data.nodeCount,
        duration: data.duration,
      });

      setLoading(false);
    } catch (err) {
      // Check if component is still mounted before updating state
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
    },
    [handleSend]
  );

  if (!isOpen) return null;

  return (
    <div
      className={cn(
        'fixed bottom-4 right-4 w-[calc(100vw-2rem)] sm:w-[420px] max-h-[600px]',
        'flex flex-col rounded-2xl',
        glassClasses.floating,
        'animate-in slide-in-from-bottom-4 fade-in duration-300',
        className
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/[0.08]">
        {/* opacity.border.subtle = white/[0.08] */}
        <div className={cn('flex items-center', gapTokens.spacious)}>
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-novanet-400 to-novanet-600 flex items-center justify-center shadow-lg shadow-novanet-500/20">
            <SparklesIcon className={cn(iconSizes.md, 'text-white')} />
          </div>
          <div className="flex flex-col">
            <span className="font-semibold text-white text-sm">AI Search</span>
            <span className="text-[10px] text-white/40">Natural language → Cypher</span>
          </div>
        </div>
        <div className={cn('flex items-center', gapTokens.tight)}>
          <button
            onClick={clearMessages}
            aria-label="Clear chat"
            className={cn(
              'p-2 rounded-lg transition-colors border',
              // opacity.border.light = white/[0.10]
              'text-white/40 hover:text-white/70 hover:bg-white/5 border-transparent hover:border-white/10'
            )}
            title="Clear chat"
          >
            <ResetIcon className={iconSizes.md} />
          </button>
          <button
            onClick={onClose}
            aria-label="Close chat (Escape)"
            className={cn(
              'p-2 rounded-lg transition-colors border',
              'text-white/40 hover:text-red-400 hover:bg-red-500/10 border-transparent hover:border-red-500/20'
            )}
            title="Close (Esc)"
          >
            <CloseIcon className={iconSizes.md} />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 scrollbar-thin min-h-[200px]">
        {messages.length === 0 && (
          <div className="text-center py-6">
            <div className="w-12 h-12 rounded-xl bg-novanet-500/10 border border-novanet-500/20 flex items-center justify-center mx-auto mb-4">
              <SparklesIcon className={cn(iconSizes['2xl'], 'text-novanet-400')} />
            </div>
            <p className="text-white/60 text-sm mb-1">
              Ask me anything about your graph
            </p>
            <p className="text-white/40 text-xs mb-6">
              I&apos;ll translate your question to Cypher
            </p>
            <div className="space-y-2">
              <SuggestionButton
                text="Show all French locale components"
                onClick={() => setInput('Show me all French locale components')}
              />
              <SuggestionButton
                text="Find concepts linked to QR codes"
                onClick={() => setInput('Find concepts linked to QR codes')}
              />
              <SuggestionButton
                text="Count expressions per locale"
                onClick={() => setInput('How many expressions per locale?')}
              />
            </div>
          </div>
        )}

        {messages.map((message, index) => (
          <ChatMessage
            key={message.id}
            message={message}
            onExecuteQuery={onExecuteQuery}
            style={{
              animationDelay: `${Math.min(index * 50, 300)}ms`,
            }}
          />
        ))}

        {isLoading && (
          <div
            role="status"
            aria-live="polite"
            className={cn('flex items-center text-novanet-400 animate-in fade-in duration-200 px-1', gapTokens.comfortable)}
          >
            <LoaderIcon className={cn(iconSizes.md, 'animate-spin')} />
            <span className="text-sm">Generating query...</span>
          </div>
        )}

        {error && (
          <div role="alert" className="text-red-400 text-sm bg-red-500/10 border border-red-500/20 px-3 py-2 rounded-lg animate-in fade-in slide-in-from-bottom-2 duration-200">
            {error}
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="p-4 border-t border-white/[0.08] bg-black/40">
        {/* opacity.border.subtle = white/[0.08] */}
        <div className={cn('flex items-end', gapTokens.default)}>
          <div className="flex-1 relative">
            <textarea
              ref={inputRef}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Describe what you're looking for..."
              rows={1}
              className={cn(
                // opacity.bg.medium = white/[0.05]
                // opacity.border.light = white/[0.10]
                'w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3',
                'text-sm resize-none focus:outline-none focus:border-novanet-500/50 focus:bg-white/[0.08]',
                'placeholder:text-white/40 transition-colors duration-150'
              )}
              style={{ minHeight: '48px', maxHeight: '120px' }}
            />
          </div>
          <button
            onClick={handleSend}
            disabled={!input.trim() || isLoading}
            aria-label="Send message"
            className={cn(
              'p-3 rounded-xl transition-colors duration-150 border',
              input.trim() && !isLoading
                ? 'bg-novanet-500/20 text-novanet-400 border-novanet-500/30 hover:bg-novanet-500/30'
                : 'bg-white/5 text-white/40 border-white/10 cursor-not-allowed'
            )}
          >
            <SendIcon className={iconSizes.md} />
          </button>
        </div>
        <div className="flex items-center justify-between mt-2 px-1">
          <p className="text-[10px] text-white/40">
            Enter to send · Shift+Enter for new line
          </p>
          <Kbd>⌘J</Kbd>
        </div>
      </div>
    </div>
  );
});

/**
 * Individual chat message
 */
interface ChatMessageProps {
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
  style?: React.CSSProperties;
}

const ChatMessage = memo(function ChatMessage({
  message,
  onExecuteQuery,
  style,
}: ChatMessageProps) {
  const { copied: copiedQuery, copy } = useCopyFeedback();

  const handleCopyQuery = async () => {
    if (message.metadata?.cypherQuery) {
      try {
        await copy(message.metadata.cypherQuery);
      } catch (error) {
        logger.error('AiChat', 'Failed to copy query', error);
      }
    }
  };

  return (
    <div
      className={cn(
        'flex flex-col animate-stagger-fade-in',
        gapTokens.default,
        message.role === 'user' ? 'items-end' : 'items-start'
      )}
      style={style}
    >
      <div
        className={cn(
          'max-w-[90%] rounded-xl px-4 py-2.5 text-sm',
          message.role === 'user'
            ? 'bg-novanet-500/20 text-novanet-100 border border-novanet-500/30'
            : 'bg-white/[0.03] text-white/80 border border-white/5'
        )}
      >
        <p className="whitespace-pre-wrap leading-relaxed">{message.content}</p>
      </div>

      {/* Cypher query display */}
      {message.metadata?.cypherQuery && (
        <div className="w-full max-w-[90%] space-y-2.5">
          <CypherViewer
            query={message.metadata.cypherQuery}
            title="Generated Cypher"
            className="text-xs"
          />

          <div className={cn('flex items-center', gapTokens.default)}>
            <button
              onClick={() => {
                // Re-check inside callback (TypeScript can't narrow across closures)
                if (message.metadata?.cypherQuery) {
                  onExecuteQuery?.(message.metadata.cypherQuery);
                }
              }}
              aria-label="Execute Cypher query"
              className={cn(
                'flex items-center text-xs px-3 py-1.5 rounded-lg',
                gapTokens.compact,
                'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30',
                'hover:bg-emerald-500/30 transition-colors'
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
                'hover:bg-white/10 hover:text-white/70 transition-colors'
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

/**
 * Suggestion button - clickable prompt suggestions
 */
interface SuggestionButtonProps {
  text: string;
  onClick: () => void;
}

const SuggestionButton = memo(function SuggestionButton({
  text,
  onClick,
}: SuggestionButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'w-full text-left text-xs px-3.5 py-2.5 rounded-xl',
        // opacity.bg.light = white/[0.03]
        // opacity.border.subtle = white/[0.06]
        'bg-white/[0.03] text-white/50 border border-white/[0.06]',
        'hover:bg-novanet-500/10 hover:text-novanet-300 hover:border-novanet-500/20',
        'transition-colors duration-150'
      )}
    >
      <span className="mr-2 text-white/40">→</span>
      {text}
    </button>
  );
});
