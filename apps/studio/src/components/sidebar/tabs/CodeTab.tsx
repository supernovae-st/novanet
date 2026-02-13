'use client';

/**
 * CodeTab - Code representations of the node
 *
 * Features:
 * - Format switcher: JSON, YAML, Cypher, TypeScript
 * - Syntax-highlighted code viewer
 * - Copy to clipboard
 */

import { memo, useState, useMemo, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Copy, Check } from 'lucide-react';
import { KIND_META } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import type { GraphNode } from '@/types';

// Code format types
type CodeFormat = 'json' | 'yaml' | 'cypher' | 'typescript';

interface CodeTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
}

interface FormatButtonProps {
  format: CodeFormat;
  label: string;
  isActive: boolean;
  onClick: () => void;
}

/**
 * Format switcher button
 */
function FormatButton({ format: _format, label, isActive, onClick }: FormatButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'px-3 py-1.5 rounded-lg text-xs font-medium transition-all',
        isActive
          ? 'bg-white/10 text-white'
          : 'text-white/50 hover:text-white/70 hover:bg-white/5'
      )}
    >
      {label}
    </button>
  );
}

/**
 * Generate JSON representation
 */
function toJson(node: GraphNode): string {
  const data = {
    id: node.id,
    type: node.type,
    key: node.key,
    displayName: node.displayName,
    description: node.description,
    llmContext: node.llmContext,
    data: node.data,
    createdAt: node.createdAt,
    updatedAt: node.updatedAt,
  };
  return JSON.stringify(data, null, 2);
}

/**
 * Generate YAML representation
 */
function toYaml(node: GraphNode): string {
  const config = NODE_TYPE_CONFIG[node.type];
  const kindMeta = KIND_META[node.type];
  const realm = kindMeta?.realm ?? 'org';
  const layer = config?.layer ?? 'foundation';
  const trait = kindMeta?.trait ?? 'defined'; // v11.8: ADR-024

  const lines: string[] = [
    `# ${node.type} Node`,
    `node:`,
    `  key: ${node.key}`,
    `  type: ${node.type}`,
    `  display_name: "${node.displayName}"`,
  ];

  if (node.description) {
    lines.push(`  description: "${node.description}"`);
  }

  lines.push(`  realm: ${realm}`);
  lines.push(`  layer: ${layer}`);
  lines.push(`  trait: ${trait}`);

  if (node.llmContext) {
    lines.push(`  llm_context: |`);
    lines.push(`    ${node.llmContext}`);
  }

  if (node.data && Object.keys(node.data).length > 0) {
    lines.push(`  properties:`);
    for (const [key, value] of Object.entries(node.data)) {
      const formatted = typeof value === 'object'
        ? JSON.stringify(value)
        : String(value);
      lines.push(`    ${key}: ${formatted}`);
    }
  }

  return lines.join('\n');
}

/**
 * Generate Cypher CREATE statement
 */
function toCypher(node: GraphNode): string {
  const props: string[] = [
    `key: "${node.key}"`,
    `displayName: "${node.displayName}"`,
  ];

  if (node.description) {
    props.push(`description: "${node.description.replace(/"/g, '\\"')}"`);
  }

  if (node.llmContext) {
    props.push(`llmContext: "${node.llmContext.replace(/"/g, '\\"')}"`);
  }

  if (node.data) {
    for (const [key, value] of Object.entries(node.data)) {
      const formatted = typeof value === 'string'
        ? `"${value.replace(/"/g, '\\"')}"`
        : JSON.stringify(value);
      props.push(`${key}: ${formatted}`);
    }
  }

  return `// Create ${node.type} node
CREATE (n:${node.type} {
  ${props.join(',\n  ')}
})
RETURN n;

// Query this node
MATCH (n:${node.type} {key: "${node.key}"})
RETURN n;

// Find related nodes
MATCH (n:${node.type} {key: "${node.key}"})-[r]-(related)
RETURN n, r, related;`;
}

/**
 * Generate TypeScript interface
 */
function toTypeScript(node: GraphNode): string {
  const config = NODE_TYPE_CONFIG[node.type];
  const kindMeta = KIND_META[node.type];
  const realm = kindMeta?.realm ?? 'org';
  const layer = config?.layer ?? 'foundation';
  const trait = kindMeta?.trait ?? 'defined'; // v11.8: ADR-024
  const properties = node.data || {};

  const propTypes = Object.entries(properties)
    .map(([key, value]) => {
      const type = Array.isArray(value)
        ? 'unknown[]'
        : typeof value === 'object'
          ? 'Record<string, unknown>'
          : typeof value;
      return `  ${key}?: ${type};`;
    })
    .join('\n');

  return `// ${node.type} Node Type Definition

import type { NodeKind } from '@novanet/core';

interface ${node.type}Data {
  key: string;
  displayName: string;
  description?: string;
  llmContext?: string;
${propTypes}
}

// Node kind metadata
const ${node.type.toLowerCase()}Kind: NodeKind = {
  name: '${node.type}',
  realm: '${realm}',
  layer: '${layer}',
  trait: '${trait}',
  display_name: '${node.displayName}',
};

// Example instance
const ${node.key.replace(/-/g, '_')}: ${node.type}Data = ${JSON.stringify({
    key: node.key,
    displayName: node.displayName,
    description: node.description,
    ...node.data,
  }, null, 2)};`;
}

/**
 * Code viewer with copy button
 */
function CodeViewer({
  code,
  format,
  onCopy,
  isCopied,
}: {
  code: string;
  format: CodeFormat;
  onCopy: () => void;
  isCopied: boolean;
}) {
  // Simple syntax highlighting colors by format
  const getLanguageClass = (fmt: CodeFormat) => {
    switch (fmt) {
      case 'json': return 'text-emerald-400/80';
      case 'yaml': return 'text-blue-400/80';
      case 'cypher': return 'text-purple-400/80';
      case 'typescript': return 'text-amber-400/80';
      default: return 'text-white/70';
    }
  };

  return (
    <div className="relative group">
      {/* Copy button */}
      <button
        onClick={onCopy}
        className={cn(
          'absolute top-3 right-3 p-2 rounded-lg transition-all',
          'opacity-0 group-hover:opacity-100',
          isCopied
            ? 'bg-emerald-500/20 text-emerald-400'
            : 'bg-white/10 text-white/60 hover:text-white hover:bg-white/20'
        )}
        title={isCopied ? 'Copied!' : 'Copy code'}
      >
        {isCopied ? (
          <Check className="w-4 h-4" />
        ) : (
          <Copy className="w-4 h-4" />
        )}
      </button>

      {/* Code block */}
      <pre className="p-4 bg-black/40 rounded-lg overflow-x-auto">
        <code className={cn('font-mono text-xs leading-relaxed whitespace-pre-wrap', getLanguageClass(format))}>
          {code}
        </code>
      </pre>
    </div>
  );
}

export const CodeTab = memo(function CodeTab({ node, colors: _colors }: CodeTabProps) {
  const [activeFormat, setActiveFormat] = useState<CodeFormat>('json');
  const { copied, copy } = useCopyFeedback();

  // Generate code for active format
  const code = useMemo(() => {
    switch (activeFormat) {
      case 'json': return toJson(node);
      case 'yaml': return toYaml(node);
      case 'cypher': return toCypher(node);
      case 'typescript': return toTypeScript(node);
      default: return '';
    }
  }, [activeFormat, node]);

  const handleCopy = useCallback(() => {
    copy(code);
  }, [copy, code]);

  return (
    <div className="flex flex-col h-full">
      {/* Format switcher */}
      <div className="flex items-center gap-1 p-2 border-b border-white/[0.06]">
        <FormatButton
          format="json"
          label="JSON"
          isActive={activeFormat === 'json'}
          onClick={() => setActiveFormat('json')}
        />
        <FormatButton
          format="yaml"
          label="YAML"
          isActive={activeFormat === 'yaml'}
          onClick={() => setActiveFormat('yaml')}
        />
        <FormatButton
          format="cypher"
          label="Cypher"
          isActive={activeFormat === 'cypher'}
          onClick={() => setActiveFormat('cypher')}
        />
        <FormatButton
          format="typescript"
          label="TypeScript"
          isActive={activeFormat === 'typescript'}
          onClick={() => setActiveFormat('typescript')}
        />
      </div>

      {/* Code viewer */}
      <div className="flex-1 overflow-y-auto scrollbar-thin p-4">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeFormat}
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
          >
            <CodeViewer
              code={code}
              format={activeFormat}
              onCopy={handleCopy}
              isCopied={copied}
            />
          </motion.div>
        </AnimatePresence>
      </div>

      {/* Footer with format info */}
      <div className="px-4 py-2 border-t border-white/[0.06] bg-black/20">
        <p className="text-[10px] text-white/30 text-center">
          {activeFormat === 'json' && 'Standard JSON representation'}
          {activeFormat === 'yaml' && 'NovaNet YAML node definition format'}
          {activeFormat === 'cypher' && 'Neo4j Cypher query statements'}
          {activeFormat === 'typescript' && 'TypeScript interface and example'}
        </p>
      </div>
    </div>
  );
});

export default CodeTab;
