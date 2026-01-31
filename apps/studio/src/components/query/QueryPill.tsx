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
import { Play, Copy, X, Check, Loader2, Expand, Minimize2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useQueryStore } from '@/stores/queryStore';
import { useCopyFeedback, useAutoFocus } from '@/hooks';
import { FOCUS_DELAY_MS } from '@/config/constants';
import { glassClasses, modalClasses, gapTokens } from '@/design/tokens';
import { IconButton, Kbd } from '@/components/ui';

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

// Matrix rain component
const MatrixRain = memo(function MatrixRain() {
  const [columns, setColumns] = useState<string[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setColumns(
        Array.from({ length: 12 }, () =>
          Array.from({ length: 3 }, () =>
            MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
          ).join('')
        )
      );
    }, 100);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-30" aria-hidden="true">
      <div className="flex justify-around h-full">
        {columns.map((col, i) => (
          <span
            key={i}
            className="font-mono text-[8px] text-emerald-500 writing-mode-vertical animate-pulse"
            style={{
              animationDelay: `${i * 100}ms`,
              writingMode: 'vertical-rl',
              textOrientation: 'upright',
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
  const { currentQuery, isExecuting, setQuery, clear } = useQueryStore(
    useShallow((state) => ({
      currentQuery: state.currentQuery,
      isExecuting: state.isExecuting,
      setQuery: state.setQuery,
      clear: state.clear,
    }))
  );
  const { copied, copy } = useCopyFeedback();

  // Matrix animations when executing
  const matrixChars = useMatrixAnimation(isExecuting, 10);
  const scrambledQuery = useMatrixTextScramble(currentQuery || '', isExecuting);

  const [isEditing, setIsEditing] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);
  const [editValue, setEditValue] = useState(currentQuery || '');
  const inputRef = useRef<HTMLInputElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // Ref for editValue to avoid event listener re-registration on every keystroke
  const editValueRef = useRef(editValue);
  editValueRef.current = editValue;

  useEffect(() => {
    if (!isEditing && !isExpanded) setEditValue(currentQuery || '');
  }, [currentQuery, isEditing, isExpanded]);

  // Focus textarea when expanding (using hook for cleanup)
  useAutoFocus(textareaRef, isExpanded);

  // Focus input when editing (using hook for cleanup)
  useAutoFocus(inputRef, isEditing, FOCUS_DELAY_MS);

  // Handle Escape to close expanded mode
  // Uses ref for editValue to avoid re-registering listener on every keystroke
  useEffect(() => {
    if (!isExpanded) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        e.preventDefault();
        e.stopPropagation();
        // Use ref for latest value without deps change
        if (editValueRef.current.trim()) {
          setQuery(editValueRef.current.trim());
        }
        setIsExpanded(false);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isExpanded, setQuery]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      if (editValue.trim()) {
        setQuery(editValue.trim());
        setIsEditing(false);
        if (e.metaKey || e.ctrlKey) onRun?.();
      }
    } else if (e.key === 'Escape') {
      setIsEditing(false);
      setEditValue(currentQuery || '');
    }
  }, [editValue, currentQuery, setQuery, onRun]);

  const handleExpandedKeyDown = useCallback((e: React.KeyboardEvent) => {
    // Cmd/Ctrl + Enter to run
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      if (editValue.trim()) {
        setQuery(editValue.trim());
        setIsExpanded(false);
        onRun?.();
      }
    }
  }, [editValue, setQuery, onRun]);

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

  const hasQuery = !!currentQuery;

  return (
    <>
      {/* Compact Pill */}
      <div
        className={cn(
          // Glass morphism with tokens
          'relative flex items-center gap-5 px-6 h-20 rounded-2xl',
          glassClasses.floating,
          'transition-all duration-300',
          // Executing state - intense Matrix glow
          isExecuting && [
            'border-emerald-400/60',
            'shadow-[0_0_40px_rgba(52,211,153,0.25),0_0_80px_rgba(52,211,153,0.1),inset_0_0_20px_rgba(52,211,153,0.05)]',
          ],
          className
        )}
      >
        {/* Matrix effects when executing */}
        {isExecuting && (
          <>
            {/* Matrix rain background */}
            <MatrixRain />
            {/* Scan line effect */}
            <div
              className="absolute inset-0 rounded-2xl overflow-hidden pointer-events-none"
              aria-hidden="true"
            >
              <div className="absolute inset-0 bg-gradient-to-r from-transparent via-emerald-500/15 to-transparent animate-[shimmer_1.5s_infinite]" />
              <div className="absolute inset-0 bg-gradient-to-b from-emerald-500/5 via-transparent to-emerald-500/5" />
            </div>
            {/* Glowing border pulse */}
            <div className="absolute inset-0 rounded-2xl border-2 border-emerald-400/50 animate-pulse pointer-events-none" />
            {/* Ping effect for feedback */}
            <div className="absolute inset-0 rounded-2xl border border-emerald-400/40 animate-ping pointer-events-none" style={{ animationDuration: '1.5s' }} />
          </>
        )}
        {/* Prompt with Matrix animation */}
        <div className={cn('flex items-center shrink-0 select-none relative z-10', gapTokens.default)}>
          <span className={cn(
            'font-mono text-xs font-bold transition-all duration-300',
            isExecuting
              ? 'text-emerald-400 drop-shadow-[0_0_12px_rgba(52,211,153,0.8)] animate-pulse'
              : 'text-emerald-500/50'
          )}>
            {isExecuting ? '>>>' : 'neo4j$'}
          </span>
          {isExecuting && (
            <span className="font-mono text-[9px] text-emerald-400/80 tracking-[0.2em] drop-shadow-[0_0_6px_rgba(52,211,153,0.5)]">
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
                'font-mono text-sm cursor-text truncate transition-all duration-200',
                hasQuery ? 'text-white/80' : 'text-white/40',
                // Matrix glow effect when executing
                isExecuting && 'text-emerald-300 drop-shadow-[0_0_10px_rgba(52,211,153,0.6)]'
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
            onClick={() => hasQuery && !isExecuting && onRun?.()}
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
            onClick={() => { clear(); setEditValue(''); }}
            disabled={!hasQuery}
            title="Clear"
            variant="danger"
            size="md"
          />
        </div>
      </div>

      {/* Expanded Modal */}
      {isExpanded && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center animate-in fade-in duration-200"
          onClick={(e) => {
            if (e.target === e.currentTarget) closeExpanded();
          }}
        >
          {/* Backdrop */}
          <div className={modalClasses.backdrop} />

          {/* Modal Content */}
          <div
            className={cn(
              'relative w-[90vw] max-w-4xl',
              glassClasses.modal,
              'animate-in zoom-in-95 slide-in-from-bottom-4 duration-300'
            )}
          >
            {/* Header */}
            <div className="flex items-center justify-between px-6 py-4 border-b border-white/10">
              <div className={cn('flex items-center', gapTokens.spacious)}>
                <span className={cn(
                  'font-mono text-sm font-medium select-none transition-colors',
                  isExecuting ? 'text-emerald-400' : 'text-emerald-500/70'
                )}>
                  neo4j$
                </span>
                <span className="text-white/40 text-sm">Cypher Editor</span>
              </div>
              <div className={cn('flex items-center', gapTokens.tight)}>
                <IconButton
                  icon={isExecuting ? Loader2 : Play}
                  onClick={() => {
                    if (editValue.trim() && !isExecuting) {
                      setQuery(editValue.trim());
                      setIsExpanded(false);
                      onRun?.();
                    }
                  }}
                  disabled={!editValue.trim() || isExecuting}
                  loading={isExecuting}
                  title="Run (Cmd+Enter)"
                  variant="success"
                  size="md"
                />
                <IconButton
                  icon={Copy}
                  onClick={() => editValue.trim() && copy(editValue)}
                  disabled={!editValue.trim()}
                  title="Copy"
                  active={copied}
                  activeIcon={Check}
                  size="md"
                />
                <IconButton
                  icon={X}
                  onClick={() => { setEditValue(''); }}
                  disabled={!editValue.trim()}
                  title="Clear"
                  variant="danger"
                  size="md"
                />
                <div className="w-px h-5 bg-white/10 mx-1" />
                <IconButton
                  icon={Minimize2}
                  onClick={closeExpanded}
                  title="Collapse (Esc)"
                  size="md"
                />
              </div>
            </div>

            {/* Editor */}
            <div className="p-6">
              <textarea
                ref={textareaRef}
                value={editValue}
                onChange={(e) => setEditValue(e.target.value)}
                onKeyDown={handleExpandedKeyDown}
                className={cn(
                  'w-full h-64 resize-none',
                  'bg-[#111118] rounded-xl p-4',
                  'font-mono text-sm text-white/90 leading-relaxed',
                  'placeholder:text-white/40',
                  'border border-white/10 focus:border-white/20',
                  'outline-none ring-0 focus:ring-0',
                  'transition-colors duration-200'
                )}
                placeholder="Enter your Cypher query here...&#10;&#10;Examples:&#10;MATCH (n:Project) RETURN n LIMIT 10&#10;MATCH (p:Project)-[:HAS_LOCALE]->(l:Locale) RETURN p, l"
                spellCheck={false}
                autoComplete="off"
              />
            </div>

            {/* Footer hint */}
            <div className="px-6 pb-4 flex items-center justify-between text-xs text-white/40">
              <span>Press <Kbd>Esc</Kbd> to close</span>
              <span>Press <Kbd>Cmd+Enter</Kbd> to run</span>
            </div>
          </div>
        </div>
      )}
    </>
  );
});
