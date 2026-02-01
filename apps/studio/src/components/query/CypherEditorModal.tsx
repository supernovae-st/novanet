'use client';

/**
 * CypherEditorModal - Matrix Terminal Style Code Editor
 *
 * Full immersion CRT experience with Monaco editor:
 * - Matrix rain background
 * - CRT scanlines + vignette
 * - Glitching header with animated prompt
 * - Monaco editor with custom Matrix theme
 * - Cypher syntax highlighting
 *
 * Keyboard shortcuts:
 * - Cmd+Enter: Run query
 * - Escape: Close modal
 */

import { memo, useEffect, useState, useCallback, useRef, useMemo } from 'react';
import dynamic from 'next/dynamic';
import { loader } from '@monaco-editor/react';
import { Play, Copy, X, Check, Loader2, Minimize2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback, useFocusTrap, useBodyScrollLock } from '@/hooks';
import { gapTokens, overlayClasses, zIndex } from '@/design/tokens';
import { IconButton } from '@/components/ui/IconButton';
import { Kbd } from '@/components/ui/Kbd';
import {
  MATRIX_THEME_NAME,
  matrixTheme,
  CYPHER_LANGUAGE_ID,
  cypherLanguageConfig,
  cypherTokensProvider,
  matrixEditorOptions,
} from '@/config/monacoConfig';

// Configure Monaco loader to use CDN (required for Next.js Turbopack compatibility)
// Turbopack cannot resolve Monaco's worker files at build time, so we load from CDN.
// See: https://github.com/suren-atoyan/monaco-react/issues/88
loader.config({
  paths: {
    vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.55.1/min/vs',
  },
});

// Lazy load Monaco to reduce initial bundle
const Editor = dynamic(() => import('@monaco-editor/react').then(mod => mod.Editor), {
  ssr: false,
  loading: () => (
    <div className="flex items-center justify-center h-64 text-emerald-400/50">
      <Loader2 className="w-6 h-6 animate-spin mr-2" />
      <span className="font-mono text-sm">Loading editor...</span>
    </div>
  ),
});

// =============================================================================
// Matrix Characters for animations
// =============================================================================

const MATRIX_CHARS = 'アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン0123456789';

// =============================================================================
// Matrix Rain Background
// =============================================================================

const MatrixRainColumn = memo(function MatrixRainColumn({ delay, speed }: { delay: number; speed: number }) {
  const [chars, setChars] = useState('');

  useEffect(() => {
    const generateChars = () => {
      const length = Math.floor(Math.random() * 15) + 8;
      return Array.from({ length }, () =>
        MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
      ).join('\n');
    };

    setChars(generateChars());
    const interval = setInterval(() => setChars(generateChars()), speed);
    return () => clearInterval(interval);
  }, [speed]);

  return (
    <div
      className="absolute top-0 font-mono text-[10px] leading-tight text-emerald-400/30 whitespace-pre animate-matrix-rain"
      style={{
        animationDelay: `${delay}ms`,
        animationDuration: `${speed}ms`,
        textShadow: '0 0 8px rgba(52, 211, 153, 0.5)',
      }}
    >
      {chars}
    </div>
  );
});

const MatrixRainBackground = memo(function MatrixRainBackground() {
  // Stable column config - generated once to honor memoization
  const columns = useMemo(() =>
    Array.from({ length: 30 }, (_, i) => ({
      id: i,
      delay: Math.random() * 4000,
      speed: 3000 + Math.random() * 2000,
    })),
    []
  );

  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none opacity-40" aria-hidden="true">
      <div className="relative w-full h-full flex justify-around">
        {columns.map((col) => (
          <MatrixRainColumn
            key={col.id}
            delay={col.delay}
            speed={col.speed}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// Glitch Header with animated prompt
// =============================================================================

const GlitchHeader = memo(function GlitchHeader({
  isExecuting,
  onRun,
  onCopy,
  onClear,
  onClose,
  hasContent,
  copied,
}: {
  isExecuting: boolean;
  onRun: () => void;
  onCopy: () => void;
  onClear: () => void;
  onClose: () => void;
  hasContent: boolean;
  copied: boolean;
}) {
  const [matrixChars, setMatrixChars] = useState('');

  // Animated Matrix characters
  useEffect(() => {
    const interval = setInterval(() => {
      setMatrixChars(
        Array.from({ length: 8 }, () =>
          MATRIX_CHARS[Math.floor(Math.random() * MATRIX_CHARS.length)]
        ).join('')
      );
    }, 80);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="relative flex items-center justify-between px-5 py-3.5 border-b border-emerald-400/30 bg-black/40">
      {/* Left: Prompt + Title */}
      <div className={cn('flex items-center', gapTokens.spacious)}>
        {/* Animated prompt */}
        <div className="flex items-center gap-1.5">
          <span className="font-mono text-sm font-bold text-emerald-400 matrix-text-glow animate-matrix-glitch">
            {'>>>'}
          </span>
          <span className="font-mono text-xs text-emerald-300/60 tracking-widest">
            {matrixChars}
          </span>
        </div>

        {/* Title with glitch */}
        <span className="font-mono text-sm text-emerald-400/70 animate-matrix-glitch-intense">
          neo4j$
        </span>
        <span className="text-white/40 text-sm">Cypher Editor</span>
      </div>

      {/* Right: Controls */}
      <div className={cn('flex items-center', gapTokens.tight)}>
        <IconButton
          icon={isExecuting ? Loader2 : Play}
          onClick={onRun}
          disabled={!hasContent || isExecuting}
          loading={isExecuting}
          title="Run (Cmd+Enter)"
          variant="success"
          size="md"
        />
        <IconButton
          icon={Copy}
          onClick={onCopy}
          disabled={!hasContent}
          title="Copy"
          active={copied}
          activeIcon={Check}
          size="md"
        />
        <IconButton
          icon={X}
          onClick={onClear}
          disabled={!hasContent}
          title="Clear"
          variant="danger"
          size="md"
        />
        <div className="w-px h-5 bg-emerald-400/20 mx-1" />
        <IconButton
          icon={Minimize2}
          onClick={onClose}
          title="Close (Esc)"
          size="md"
        />
      </div>
    </div>
  );
});

// =============================================================================
// Scanline Sweep Effect
// =============================================================================

const ScanlineSweep = memo(function ScanlineSweep() {
  return (
    <div className="absolute inset-0 overflow-hidden pointer-events-none" aria-hidden="true">
      <div
        className="absolute left-0 right-0 h-[2px] bg-gradient-to-r from-transparent via-emerald-400/40 to-transparent animate-matrix-scanline"
        style={{ filter: 'blur(1px)' }}
      />
    </div>
  );
});

// =============================================================================
// Matrix Footer
// =============================================================================

const MatrixFooter = memo(function MatrixFooter({ lineCount }: { lineCount: number }) {
  return (
    <div className="relative px-5 py-3 border-t border-emerald-400/20 bg-black/40">
      <div className="flex items-center justify-between text-xs">
        <span className={cn('flex items-center text-emerald-400/50', gapTokens.compact)}>
          Press <Kbd className="border-emerald-400/30 text-emerald-400/70">Esc</Kbd> to close
        </span>

        {/* Stats */}
        <div className="flex items-center gap-4">
          <span className="font-mono text-emerald-400/40">
            {lineCount} {lineCount === 1 ? 'line' : 'lines'}
          </span>
        </div>

        <span className={cn('flex items-center text-emerald-400/50', gapTokens.compact)}>
          Press <Kbd className="border-emerald-400/30 text-emerald-400/70">Cmd+Enter</Kbd> to run
        </span>
      </div>
    </div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export interface CypherEditorModalProps {
  isOpen: boolean;
  onClose: () => void;
  value: string;
  onChange: (value: string) => void;
  onRun: () => void;
  isExecuting?: boolean;
}

export const CypherEditorModal = memo(function CypherEditorModal({
  isOpen,
  onClose,
  value,
  onChange,
  onRun,
  isExecuting = false,
}: CypherEditorModalProps) {
  const { copied, copy } = useCopyFeedback();
  const modalRef = useRef<HTMLDivElement>(null);
  const editorRef = useRef<unknown>(null);

  // Ref for value to avoid event listener re-registration on every keystroke
  const valueRef = useRef(value);
  valueRef.current = value;

  // Lock body scroll when open
  useBodyScrollLock(isOpen);

  // Focus trap for accessibility
  useFocusTrap(modalRef, isOpen);

  // Line count for footer
  const lineCount = value.split('\n').length;

  // Handle keyboard shortcuts - uses ref for stable dependencies
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      if (valueRef.current.trim()) {
        onRun();
      }
    }
  }, [onClose, onRun]);

  useEffect(() => {
    if (!isOpen) return;
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, handleKeyDown]);

  // Monaco setup
  const handleEditorWillMount = useCallback((monaco: typeof import('monaco-editor')) => {
    // Register theme
    monaco.editor.defineTheme(MATRIX_THEME_NAME, matrixTheme);

    // Register Cypher language if not already registered
    if (!monaco.languages.getLanguages().some(lang => lang.id === CYPHER_LANGUAGE_ID)) {
      monaco.languages.register({ id: CYPHER_LANGUAGE_ID });
      monaco.languages.setLanguageConfiguration(CYPHER_LANGUAGE_ID, cypherLanguageConfig);
      monaco.languages.setMonarchTokensProvider(CYPHER_LANGUAGE_ID, cypherTokensProvider);
    }
  }, []);

  const handleEditorDidMount = useCallback((editor: unknown) => {
    editorRef.current = editor;
    // Focus editor
    if (editor && typeof (editor as { focus: () => void }).focus === 'function') {
      (editor as { focus: () => void }).focus();
    }
  }, []);

  const handleCopy = useCallback(() => {
    if (value.trim()) {
      copy(value);
    }
  }, [value, copy]);

  const handleClear = useCallback(() => {
    onChange('');
  }, [onChange]);

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 flex items-center justify-center animate-in fade-in duration-200"
      style={{ zIndex: zIndex.modal }}
    >
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/80 backdrop-blur-sm animate-overlay-backdrop"
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Modal */}
      <div
        ref={modalRef}
        role="dialog"
        aria-modal="true"
        aria-label="Cypher Editor"
        className={cn(
          'relative w-full mx-4',
          overlayClasses.maxWidth,
          // Base styling
          'rounded-2xl overflow-hidden',
          'bg-[#0a0a0f] border-2',
          // Matrix glow effects
          'animate-matrix-glow-pulse animate-matrix-border-glow',
          'animate-overlay-enter animate-matrix-flicker',
        )}
      >
        {/* Layer 1: Matrix Rain */}
        <MatrixRainBackground />

        {/* Layer 2: CRT Scanlines */}
        <div className="absolute inset-0 matrix-scanlines opacity-30" aria-hidden="true" />

        {/* Layer 3: Scanline Sweep */}
        <ScanlineSweep />

        {/* Layer 4: Vignette */}
        <div className="absolute inset-0 matrix-vignette" aria-hidden="true" />

        {/* Layer 5: Content */}
        <div className="relative z-10">
          {/* Header */}
          <GlitchHeader
            isExecuting={isExecuting}
            onRun={onRun}
            onCopy={handleCopy}
            onClear={handleClear}
            onClose={onClose}
            hasContent={!!value.trim()}
            copied={copied}
          />

          {/* Editor */}
          <div className="relative min-h-[300px] max-h-[60vh]">
            <Editor
              height="300px"
              language={CYPHER_LANGUAGE_ID}
              theme={MATRIX_THEME_NAME}
              value={value}
              onChange={(val) => onChange(val || '')}
              beforeMount={handleEditorWillMount}
              onMount={handleEditorDidMount}
              options={matrixEditorOptions}
            />
          </div>

          {/* Footer */}
          <MatrixFooter lineCount={lineCount} />
        </div>
      </div>
    </div>
  );
});
