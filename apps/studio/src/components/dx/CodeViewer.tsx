'use client';

import { useMemo, memo } from 'react';
import { Highlight, themes } from 'prism-react-renderer';
import { cn } from '@/lib/utils';
import { CopyButton } from './CopyButton';

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
}

/**
 * Code viewer with syntax highlighting and copy functionality
 */
export const CodeViewer = memo(function CodeViewer({
  code,
  language = 'json',
  maxHeight = 'max-h-64',
  showLineNumbers = false,
  showCopy = true,
  title,
  className,
}: CodeViewerProps) {
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
  const prismLanguage = useMemo(() => {
    switch (language) {
      case 'cypher':
        return 'sql'; // Cypher is similar to SQL
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
    <div className={cn('relative group rounded-lg overflow-hidden', className)}>
      {/* Header */}
      {title && (
        <div className="flex items-center justify-between px-3 py-2 bg-white/5 border-b border-white/10">
          <span className="text-xs font-medium text-white/60">{title}</span>
          {showCopy && (
            <CopyButton text={formattedCode} className="opacity-0 group-hover:opacity-100 transition-all" />
          )}
        </div>
      )}

      {/* Code block */}
      <div className={cn('overflow-auto scrollbar-thin', maxHeight)}>
        <Highlight
          theme={themes.nightOwl}
          code={formattedCode}
          language={prismLanguage}
        >
          {({ className: highlightClass, style, tokens, getLineProps, getTokenProps }) => (
            <pre
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

      {/* Floating copy button if no title */}
      {!title && showCopy && (
        <CopyButton
          text={formattedCode}
          className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-all"
        />
      )}
    </div>
  );
});

/**
 * JSON Viewer - convenience wrapper
 */
export interface JsonViewerProps {
  data: unknown;
  maxHeight?: string;
  title?: string;
  className?: string;
}

export const JsonViewer = memo(function JsonViewer({
  data,
  maxHeight = 'max-h-64',
  title,
  className,
}: JsonViewerProps) {
  const code = useMemo(() => {
    try {
      return JSON.stringify(data, null, 2);
    } catch {
      return String(data);
    }
  }, [data]);

  return (
    <CodeViewer
      code={code}
      language="json"
      maxHeight={maxHeight}
      title={title}
      className={className}
    />
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
