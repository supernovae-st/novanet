'use client';

import { useMemo, memo, useEffect } from 'react';
import { Highlight, themes, Prism } from 'prism-react-renderer';
import { cn } from '@/lib/utils';
import { CopyButton } from './CopyButton';
import { registerCypher } from '@/lib/prism/cypher';

// Register Cypher language grammar once
registerCypher(Prism);

export interface CodeViewerProps {
  /** Code content */
  code: string;
  /** Language for syntax highlighting */
  language?: 'json' | 'typescript' | 'cypher' | 'yaml' | 'javascript';
  /** Maximum height */
  maxHeight?: string;
  /** Show line numbers */
  showLineNumbers?: boolean;
  /** Show copy button */
  showCopy?: boolean;
  /** Title/label */
  title?: string;
  /** Additional class names */
  className?: string;
  /** ID for ARIA labelling */
  id?: string;
}

/**
 * Code viewer with syntax highlighting and copy functionality
 *
 * Accessibility:
 * - Focusable container for keyboard users
 * - Copy button visible on hover AND focus-within
 * - Proper ARIA labels
 */
export const CodeViewer = memo(function CodeViewer({
  code,
  language = 'json',
  maxHeight = 'max-h-64',
  showLineNumbers = false,
  showCopy = true,
  title,
  className,
  id,
}: CodeViewerProps) {
  const codeId = id || `code-viewer-${Math.random().toString(36).slice(2, 9)}`;

  // Format JSON if needed
  const formattedCode = useMemo(() => {
    if (language === 'json') {
      try {
        const parsed = JSON.parse(code);
        return JSON.stringify(parsed, null, 2);
      } catch {
        return code;
      }
    }
    return code;
  }, [code, language]);

  // Map language to Prism language
  // Note: Cypher grammar is registered in module scope above
  const prismLanguage = useMemo(() => {
    switch (language) {
      case 'cypher':
        return 'cypher'; // Custom Cypher grammar registered above
      case 'typescript':
        return 'typescript';
      case 'javascript':
        return 'javascript';
      case 'yaml':
        return 'yaml';
      default:
        return 'json';
    }
  }, [language]);

  return (
    <div
      className={cn(
        'relative rounded-lg overflow-hidden',
        // Focus-within and hover both show copy button
        'group focus-within:ring-2 focus-within:ring-novanet-accent/50',
        className
      )}
    >
      {/* Header */}
      {title && (
        <div className="flex items-center justify-between px-3 py-2 bg-white/5 border-b border-white/10">
          <span id={`${codeId}-title`} className="text-xs font-medium text-white/60">
            {title}
          </span>
          {showCopy && (
            <CopyButton
              text={formattedCode}
              label={`Copy ${title || language} code`}
              className="opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity"
            />
          )}
        </div>
      )}

      {/* Code block - focusable for keyboard navigation */}
      <div
        className={cn('overflow-auto scrollbar-thin', maxHeight)}
        tabIndex={0}
        role="region"
        aria-label={title ? `${title} code block` : `${language} code block`}
        aria-describedby={title ? `${codeId}-title` : undefined}
      >
        <Highlight
          theme={themes.nightOwl}
          code={formattedCode}
          language={prismLanguage}
        >
          {({ className: highlightClass, style, tokens, getLineProps, getTokenProps }) => (
            <pre
              id={codeId}
              className={cn(
                highlightClass,
                'text-xs p-3 bg-black/40 m-0',
                !title && 'rounded-lg'
              )}
              style={{ ...style, background: 'transparent' }}
            >
              {tokens.map((line, i) => (
                <div key={i} {...getLineProps({ line })}>
                  {showLineNumbers && (
                    <span className="inline-block w-8 text-white/40 select-none text-right mr-4">
                      {i + 1}
                    </span>
                  )}
                  {line.map((token, key) => (
                    <span key={key} {...getTokenProps({ token })} />
                  ))}
                </div>
              ))}
            </pre>
          )}
        </Highlight>
      </div>

      {/* Floating copy button if no title - visible on hover OR focus-within */}
      {!title && showCopy && (
        <CopyButton
          text={formattedCode}
          label={`Copy ${language} code`}
          className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 transition-opacity"
        />
      )}
    </div>
  );
});

/**
 * Cypher Query Viewer
 */
export interface CypherViewerProps {
  query: string;
  title?: string;
  className?: string;
}

export const CypherViewer = memo(function CypherViewer({
  query,
  title = 'Cypher Query',
  className,
}: CypherViewerProps) {
  return (
    <CodeViewer
      code={query}
      language="cypher"
      title={title}
      showLineNumbers
      className={className}
    />
  );
});
