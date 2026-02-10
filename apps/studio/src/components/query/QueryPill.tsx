'use client';

/**
 * QueryPill - Modern floating query editor
 *
 * Design system: Linear-dark (#0d0d12 base, white/10 borders)
 * - No focus rings (clean input)
 * - Unified glass style
 * - Integrated controls via IconButton
 * - Expandable modal mode for complex queries
 * - Matrix-style animation when executing
 */

import { useState, useCallback, useRef, useEffect, memo } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { Play, Copy, X, Check, Loader2, Expand, Zap } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useQueryStore } from '@/stores/queryStore';
import { useViewStore } from '@/stores/viewStore';
import { useCopyFeedback, useAutoFocus } from '@/hooks';
import { FOCUS_DELAY_MS } from '@/config/constants';
import { gapTokens } from '@/design/tokens';
import { IconButton } from '@/components/ui';
import { CypherEditorModal } from './CypherEditorModal';

// Matrix-style characters for animation
const MATRIX_CHARS = 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789ABCDEF';
const MATRIX_SYMBOLS = '░▒▓█▀▄▌▐│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌';

// Matrix animation hook - more intense
function useMatrixAnimation(isActive: boolean, length: number = 8) {
  const [chars, setChars] = useState('');

  useEffect(() => {
    if (!isActive) {
      setChars('');
      return;
    }

    const interval = setInterval(() => {
      const allChars = MATRIX_CHARS + MATRIX_SYMBOLS;
      setChars(
        Array.from({ length }, () =>
          allChars[Math.floor(Math.random() * allChars.length)]
        ).join('')
      );
    }, 50); // Faster for more intense effect

    return () => clearInterval(interval);
  }, [isActive, length]);

  return chars;
}

// Matrix text scramble effect for query text
function useMatrixTextScramble(text: string, isActive: boolean) {
  const [scrambledText, setScrambledText] = useState(text);
  const frameRef = useRef(0);

  useEffect(() => {
    if (!isActive || !text) {
      setScrambledText(text);
      return;
    }

    const chars = text.split('');
    const revealed = new Array(chars.length).fill(false);
    let revealIndex = 0;

    const interval = setInterval(() => {
      frameRef.current++;

      // Gradually reveal characters
      if (frameRef.current % 3 === 0 && revealIndex < chars.length) {
        revealed[revealIndex] = true;
        revealIndex++;
      }

      // Generate scrambled text
      const result = chars.map((char, i) => {
        if (revealed[i]) return char;
        if (char === ' ') return ' ';
        // Random Matrix character
        const pool = MATRIX_CHARS + MATRIX_SYMBOLS;
        return pool[Math.floor(Math.random() * pool.length)];
      }).join('');

      setScrambledText(result);

      // Reset when all revealed
      if (revealIndex >= chars.length) {
        frameRef.current = 0;
        revealIndex = 0;
        revealed.fill(false);
      }
    }, 40);

    return () => {
      clearInterval(interval);
      frameRef.current = 0;
    };
  }, [text, isActive]);

  return scrambledText;
}

// Matrix rain component - intensified
const MatrixRain = memo(function MatrixRain() {
  const [columns, setColumns] = useState<string[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setColumns(
        Array.from({ length: 24 }, () =>
          Array.from({ length: 4 }, () =>
            MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
          ).join('')
        )
      );
    }, 70);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-40" aria-hidden="true">
      <div className="flex justify-around h-full">
        {columns.map((col, i) => (
          <span
            key={i}
            className="font-mono text-[9px] text-emerald-400 writing-mode-vertical animate-pulse"
            style={{
              animationDelay: `${i * 60}ms`,
              writingMode: 'vertical-rl',
              textOrientation: 'upright',
              textShadow: '0 0 8px rgba(52,211,153,0.6)',
            }}
          >
            {col}
          </span>
        ))}
      </div>
    </div>
  );
});

// Cypher syntax highlighting (optimized)
function highlightCypher(query: string): React.ReactNode[] {
  const patterns: [RegExp, string][] = [
    [/'[^']*'|"[^"]*"/g, 'text-orange-300'],
    [/\b(MATCH|RETURN|WHERE|WITH|OPTIONAL|CREATE|MERGE|DELETE|SET|REMOVE|LIMIT|SKIP|ORDER BY|ASC|DESC|AND|OR|NOT|IN|AS|DISTINCT|CALL|YIELD|UNWIND|FOREACH)\b/gi, 'text-cyan-400 font-medium'],
    [/:[A-Z][a-zA-Z0-9_]*/g, 'text-emerald-400'],
    [/\$[a-zA-Z_][a-zA-Z0-9_]*/g, 'text-amber-400'],
    [/\b\d+\.?\d*\b/g, 'text-purple-400'],
  ];

  const tokens: { start: number; end: number; className: string; text: string }[] = [];

  for (const [regex, className] of patterns) {
    let match;
    regex.lastIndex = 0;
    while ((match = regex.exec(query)) !== null) {
      tokens.push({ start: match.index, end: match.index + match[0].length, className, text: match[0] });
    }
  }

  tokens.sort((a, b) => a.start - b.start);
  const filtered = tokens.filter((t, i) => i === 0 || t.start >= tokens[i - 1].end);

  const parts: React.ReactNode[] = [];
  let lastIndex = 0;

  for (const { start, end, className, text } of filtered) {
    if (start > lastIndex) {
      parts.push(<span key={`t${lastIndex}`} className="text-white/70">{query.slice(lastIndex, start)}</span>);
    }
    parts.push(<span key={`k${start}`} className={className}>{text}</span>);
    lastIndex = end;
  }

  if (lastIndex < query.length) {
    parts.push(<span key={`t${lastIndex}`} className="text-white/70">{query.slice(lastIndex)}</span>);
  }

  return parts.length > 0 ? parts : [<span key="e" className="text-white/70">{query}</span>];
}

interface QueryPillProps {
  className?: string;
  onRun?: () => void;
}

export const QueryPill = memo(function QueryPill({ className, onRun }: QueryPillProps) {
  // Query store - for display and editing
  const { currentQuery, isExecuting: queryExecuting, setQuery, clear } = useQueryStore(
    useShallow((state) => ({
      currentQuery: state.currentQuery,
      isExecuting: state.isExecuting,
      setQuery: state.setQuery,
      clear: state.clear,
    }))
  );

  // View store - v12: for custom query execution and tracking
  const {
    isCustomQuery,
    isExecuting: viewExecuting,
    executeCustomQuery,
    activeViewId,
    executeView,
  } = useViewStore(
    useShallow((state) => ({
      isCustomQuery: state.isCustomQuery,
      isExecuting: state.isExecuting,
      executeCustomQuery: state.executeCustomQuery,
      activeViewId: state.activeViewId,
      executeView: state.executeView,
    }))
  );

  // Combined executing state
  const isExecuting = queryExecuting || viewExecuting;

  const { copied, copy } = useCopyFeedback();

  // Matrix animations when executing
  const matrixChars = useMatrixAnimation(isExecuting, 10);
  const scrambledQuery = useMatrixTextScramble(currentQuery || '', isExecuting);

  const [isEditing, setIsEditing] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);
  const [editValue, setEditValue] = useState(currentQuery || '');
  const inputRef = useRef<HTMLInputElement>(null);

  // Ref for editValue to avoid event listener re-registration on every keystroke
  const editValueRef = useRef(editValue);
  editValueRef.current = editValue;

  useEffect(() => {
    if (!isEditing && !isExpanded) setEditValue(currentQuery || '');
  }, [currentQuery, isEditing, isExpanded]);

  // Focus input when editing (using hook for cleanup)
  useAutoFocus(inputRef, isEditing, FOCUS_DELAY_MS);

  // v12: Run custom query via viewStore
  const handleRunQuery = useCallback(async (query: string) => {
    if (!query.trim()) return;
    // Execute via viewStore - this tracks it as a custom query
    await executeCustomQuery(query.trim());
    onRun?.();
  }, [executeCustomQuery, onRun]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      if (editValue.trim()) {
        setQuery(editValue.trim());
        setIsEditing(false);
        // v12: Execute via viewStore on Cmd+Enter
        if (e.metaKey || e.ctrlKey) {
          handleRunQuery(editValue.trim());
        }
      }
    } else if (e.key === 'Escape') {
      setIsEditing(false);
      setEditValue(currentQuery || '');
    }
  }, [editValue, currentQuery, setQuery, handleRunQuery]);

  const startEditing = useCallback(() => {
    setIsEditing(true);
    // Focus handled by useAutoFocus hook
  }, []);

  const openExpanded = useCallback(() => {
    setEditValue(currentQuery || '');
    setIsExpanded(true);
  }, [currentQuery]);

  const closeExpanded = useCallback(() => {
    if (editValue.trim()) setQuery(editValue.trim());
    setIsExpanded(false);
  }, [editValue, setQuery]);

  const handleModalRun = useCallback(() => {
    if (editValue.trim()) {
      setQuery(editValue.trim());
      setIsExpanded(false);
      handleRunQuery(editValue.trim());
    }
  }, [editValue, setQuery, handleRunQuery]);

  // v12: Handle clear - if in custom query mode, re-execute the current view
  const handleClear = useCallback(() => {
    clear();
    setEditValue('');
    // If we were showing a custom query, go back to the active view
    if (isCustomQuery) {
      executeView(activeViewId);
    }
  }, [clear, isCustomQuery, activeViewId, executeView]);

  // v12: Back to view button when in custom query mode
  const handleBackToView = useCallback(() => {
    executeView(activeViewId);
    clear();
    setEditValue('');
  }, [activeViewId, executeView, clear]);

  const hasQuery = !!currentQuery;

  return (
    <>
      {/* Compact Pill */}
      <div
        className={cn(
          // Solid dark - matches Pill component
          'relative flex items-center gap-5 px-6 h-20 rounded-2xl font-mono',
          'bg-[#0a0a0f] border border-white/10',
          'shadow-2xl shadow-black/60',
          'ring-1 ring-white/[0.03] ring-inset',
          'hover:border-white/[0.18]',
          'transition duration-300 ease-out',
          // Executing state - ultra Matrix glow
          isExecuting && [
            'border-emerald-400/80',
            'shadow-[0_0_80px_rgba(52,211,153,0.4),0_0_160px_rgba(52,211,153,0.2),inset_0_0_40px_rgba(52,211,153,0.12)]',
          ],
          className
        )}
      >
        {/* Matrix effects when executing */}
        {isExecuting && (
          <>
            {/* Emerald ambient background */}
            <div className="absolute inset-0 rounded-2xl bg-emerald-950/30 pointer-events-none" aria-hidden="true" />
            {/* Matrix rain background */}
            <MatrixRain />
            {/* Horizontal shimmer sweep */}
            <div
              className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none"
              aria-hidden="true"
            >
              <div className="absolute inset-0 bg-gradient-to-r from-transparent via-emerald-400/20 to-transparent animate-[shimmer_1.2s_infinite]" />
              <div className="absolute inset-0 bg-gradient-to-b from-emerald-500/8 via-transparent to-emerald-500/8" />
            </div>
            {/* Vertical scan line */}
            <div className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none" aria-hidden="true">
              <div
                className="absolute top-0 bottom-0 w-[2px] bg-gradient-to-b from-transparent via-emerald-400/60 to-transparent animate-[shimmer_2s_infinite]"
                style={{ filter: 'blur(1px)' }}
              />
            </div>
            {/* Glowing border pulse */}
            <div className="absolute inset-0 rounded-2xl border-2 border-emerald-400/60 animate-pulse pointer-events-none" />
            {/* Ping ripple effect */}
            <div className="absolute inset-0 rounded-2xl border border-emerald-400/40 animate-ping pointer-events-none" style={{ animationDuration: '2s' }} />
          </>
        )}
        {/* Prompt with Matrix animation */}
        <div className={cn('flex items-center shrink-0 select-none relative z-10', gapTokens.default)}>
          {/* v12: Custom query indicator */}
          {isCustomQuery && !isExecuting && (
            <span className="flex items-center gap-1 px-2 py-0.5 rounded bg-amber-500/15 border border-amber-500/25">
              <Zap className="w-3 h-3 text-amber-400" />
              <span className="text-[10px] font-medium text-amber-400">Custom</span>
            </span>
          )}
          <span className={cn(
            'font-mono text-xs font-bold transition duration-300',
            isExecuting
              ? 'text-emerald-300 drop-shadow-[0_0_16px_rgba(52,211,153,0.9)] animate-pulse'
              : isCustomQuery
                ? 'text-amber-400/70'
                : 'text-emerald-500/50'
          )}>
            {isExecuting ? '>>>' : 'neo4j$'}
          </span>
          {isExecuting && (
            <span className="font-mono text-[9px] text-emerald-300/90 tracking-[0.2em] drop-shadow-[0_0_10px_rgba(52,211,153,0.7)]">
              {matrixChars}
            </span>
          )}
        </div>

        {/* Input */}
        <div className="flex-1 min-w-0">
          {isEditing ? (
            <input
              ref={inputRef}
              type="text"
              value={editValue}
              onChange={(e) => setEditValue(e.target.value)}
              onKeyDown={handleKeyDown}
              onBlur={() => {
                if (editValue.trim()) setQuery(editValue.trim());
                setIsEditing(false);
              }}
              className="w-full bg-transparent font-mono text-sm text-white/90 placeholder:text-white/40 outline-none border-none ring-0 focus:ring-0 focus:outline-none"
              placeholder="Enter Cypher query..."
              spellCheck={false}
              autoComplete="off"
            />
          ) : (
            <div
              role="button"
              tabIndex={0}
              aria-label="Edit Cypher query"
              onClick={startEditing}
              onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && startEditing()}
              className={cn(
                'font-mono text-sm cursor-text truncate transition duration-200',
                hasQuery ? 'text-white/80' : 'text-white/40',
                // Matrix glow effect when executing
                isExecuting && 'text-emerald-200 drop-shadow-[0_0_14px_rgba(52,211,153,0.8)]'
              )}
            >
              {hasQuery
                ? isExecuting
                  ? scrambledQuery
                  : highlightCypher(currentQuery)
                : 'Click to enter Cypher query…'}
            </div>
          )}
        </div>

        {/* Expand button */}
        <IconButton
          icon={Expand}
          onClick={openExpanded}
          title="Expand editor (for complex queries)"
          size="md"
        />

        {/* Divider */}
        <div className="w-px h-6 bg-white/10 shrink-0" />

        {/* Controls */}
        <div className="flex items-center gap-0.5 shrink-0">
          <IconButton
            icon={isExecuting ? Loader2 : Play}
            onClick={() => hasQuery && !isExecuting && handleRunQuery(currentQuery)}
            disabled={!hasQuery || isExecuting}
            loading={isExecuting}
            title="Run (Cmd+Enter)"
            variant="success"
            size="md"
          />
          <IconButton
            icon={Copy}
            onClick={() => hasQuery && copy(currentQuery)}
            disabled={!hasQuery}
            title="Copy"
            active={copied}
            activeIcon={Check}
            size="md"
          />
          <IconButton
            icon={X}
            onClick={handleClear}
            disabled={!hasQuery}
            title={isCustomQuery ? "Clear and return to view" : "Clear"}
            variant="danger"
            size="md"
          />
        </div>
      </div>

      {/* Matrix Terminal Cypher Editor Modal */}
      <CypherEditorModal
        isOpen={isExpanded}
        onClose={closeExpanded}
        value={editValue}
        onChange={setEditValue}
        onRun={handleModalRun}
        isExecuting={isExecuting}
      />
    </>
  );
});
