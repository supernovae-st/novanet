'use client';

/**
 * CodeTab - Premium code representations of the node
 *
 * Features:
 * - Format switcher: JSON, YAML, Cypher, TypeScript
 * - Premium syntax-highlighted code viewer with glow effects
 * - Copy to clipboard with feedback
 * - Language-specific color themes
 * - Line numbers option
 *
 * Design System:
 * - Uses glass.surface hierarchy
 * - Language-specific accent colors
 * - Premium code block with subtle gradients
 */

import { memo, useState, useMemo, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Copy, Check, FileJson, FileCode, Database, FileType } from 'lucide-react';
import { CLASS_TAXONOMY } from '@novanet/core/types';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { NODE_TYPE_CONFIG } from '@/config/nodeTypes';
import { glass } from '@/design/tokens';
import type { GraphNode } from '@/types';

// Code format types with metadata
type CodeFormat = 'json' | 'yaml' | 'cypher' | 'typescript';

interface FormatConfig {
  id: CodeFormat;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  color: string;
  bgColor: string;
  description: string;
}

const FORMAT_CONFIGS: FormatConfig[] = [
  {
    id: 'json',
    label: 'JSON',
    icon: FileJson,
    color: '#10b981',
    bgColor: 'rgba(16, 185, 129, 0.1)',
    description: 'Standard JSON representation',
  },
  {
    id: 'yaml',
    label: 'YAML',
    icon: FileCode,
    color: '#3b82f6',
    bgColor: 'rgba(59, 130, 246, 0.1)',
    description: 'NovaNet YAML node definition',
  },
  {
    id: 'cypher',
    label: 'Cypher',
    icon: Database,
    color: '#a855f7',
    bgColor: 'rgba(168, 85, 247, 0.1)',
    description: 'Neo4j Cypher query statements',
  },
  {
    id: 'typescript',
    label: 'TypeScript',
    icon: FileType,
    color: '#f59e0b',
    bgColor: 'rgba(245, 158, 11, 0.1)',
    description: 'TypeScript interface and example',
  },
];

interface CodeTabProps {
  node: GraphNode;
  colors: { primary: string; secondary: string };
}

/**
 * Premium format switcher with icons and active state
 */
function FormatSwitcher({
  activeFormat,
  onFormatChange,
}: {
  activeFormat: CodeFormat;
  onFormatChange: (format: CodeFormat) => void;
}) {
  return (
    <div
      className="flex items-center gap-1 p-1.5"
      style={{
        background: `linear-gradient(180deg, ${glass.surface[2]}, ${glass.surface[1]})`,
        borderBottom: `1px solid ${glass.border.subtle}`,
      }}
    >
      {FORMAT_CONFIGS.map((config) => {
        const isActive = config.id === activeFormat;
        const Icon = config.icon;

        return (
          <button
            key={config.id}
            onClick={() => onFormatChange(config.id)}
            className={cn(
              'relative flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium',
              'transition-all duration-200 ease-out',
              'focus:outline-none focus-visible:ring-2 focus-visible:ring-white/20',
              isActive ? 'text-white' : 'text-white/40 hover:text-white/60'
            )}
            style={isActive ? {
              background: config.bgColor,
              boxShadow: `0 0 12px ${config.color}20`,
            } : undefined}
            title={config.description}
          >
            <span style={isActive ? { color: config.color } : undefined}>
              <Icon className="w-3.5 h-3.5" />
            </span>
            <span>{config.label}</span>

            {/* Active indicator dot */}
            {isActive && (
              <motion.div
                layoutId="codeFormatIndicator"
                className="absolute -bottom-1.5 left-1/2 -translate-x-1/2 w-1 h-1 rounded-full"
                style={{ background: config.color }}
                transition={{ type: 'spring', stiffness: 400, damping: 30 }}
              />
            )}
          </button>
        );
      })}
    </div>
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
  const classification = CLASS_TAXONOMY[node.type];
  const realm = classification?.realm ?? 'org';
  const layer = config?.layer ?? 'foundation';
  const trait = classification?.trait ?? 'defined'; // v11.8: ADR-024

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
  const classification = CLASS_TAXONOMY[node.type];
  const realm = classification?.realm ?? 'org';
  const layer = config?.layer ?? 'foundation';
  const trait = classification?.trait ?? 'defined'; // v11.8: ADR-024
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

import type { NodeClass } from '@novanet/core';

interface ${node.type}Data {
  key: string;
  displayName: string;
  description?: string;
  llmContext?: string;
${propTypes}
}

// Node class metadata
const ${node.type.toLowerCase()}Class: NodeClass = {
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
 * Premium code viewer with syntax highlighting and glow effects
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
  const formatConfig = FORMAT_CONFIGS.find((c) => c.id === format)!;
  const lines = code.split('\n');

  return (
    <div className="relative group">
      {/* Premium code container with glass effect */}
      <div
        className="relative rounded-xl overflow-hidden"
        style={{
          background: `linear-gradient(135deg, ${glass.surface[0]}, ${glass.surface[1]})`,
          border: `1px solid ${glass.border.subtle}`,
          boxShadow: `
            inset 0 1px 0 ${glass.highlight.subtle},
            0 4px 24px rgba(0, 0, 0, 0.4)
          `,
        }}
      >
        {/* Header bar with format indicator */}
        <div
          className="flex items-center justify-between px-4 py-2"
          style={{
            background: `linear-gradient(90deg, ${formatConfig.bgColor}, transparent)`,
            borderBottom: `1px solid ${glass.border.subtle}`,
          }}
        >
          <div className="flex items-center gap-2">
            {/* Traffic light dots */}
            <div className="flex items-center gap-1.5">
              <div className="w-2.5 h-2.5 rounded-full bg-red-500/60" />
              <div className="w-2.5 h-2.5 rounded-full bg-yellow-500/60" />
              <div className="w-2.5 h-2.5 rounded-full bg-green-500/60" />
            </div>
            <span
              className="text-[10px] font-mono uppercase tracking-wider"
              style={{ color: formatConfig.color }}
            >
              {format}
            </span>
          </div>

          {/* Copy button */}
          <motion.button
            onClick={onCopy}
            className={cn(
              'flex items-center gap-1.5 px-2.5 py-1 rounded-md text-xs font-medium',
              'transition-all duration-200',
              isCopied
                ? 'bg-emerald-500/20 text-emerald-400'
                : 'bg-white/[0.06] text-white/50 hover:text-white hover:bg-white/10'
            )}
            whileTap={{ scale: 0.95 }}
          >
            {isCopied ? (
              <>
                <Check className="w-3 h-3" />
                <span>Copied</span>
              </>
            ) : (
              <>
                <Copy className="w-3 h-3" />
                <span>Copy</span>
              </>
            )}
          </motion.button>
        </div>

        {/* Code content with line numbers */}
        <div className="overflow-x-auto">
          <pre className="p-4 text-xs leading-relaxed">
            <code className="flex">
              {/* Line numbers column */}
              <div
                className="flex-shrink-0 pr-4 select-none text-right font-mono"
                style={{ color: `${formatConfig.color}40` }}
              >
                {lines.map((_, i) => (
                  <div key={i} className="leading-relaxed">
                    {i + 1}
                  </div>
                ))}
              </div>

              {/* Code column */}
              <div
                className="flex-1 font-mono whitespace-pre"
                style={{ color: `${formatConfig.color}cc` }}
              >
                {lines.map((line, i) => (
                  <div
                    key={i}
                    className="leading-relaxed hover:bg-white/[0.02] -mx-2 px-2 rounded transition-colors"
                  >
                    {line || ' '}
                  </div>
                ))}
              </div>
            </code>
          </pre>
        </div>

        {/* Bottom glow accent */}
        <div
          className="absolute bottom-0 left-0 right-0 h-px"
          style={{
            background: `linear-gradient(90deg, transparent, ${formatConfig.color}30, transparent)`,
          }}
        />
      </div>
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

  const formatConfig = FORMAT_CONFIGS.find((c) => c.id === activeFormat)!;
  const lineCount = code.split('\n').length;

  return (
    <div className="flex flex-col h-full">
      {/* Premium format switcher */}
      <FormatSwitcher
        activeFormat={activeFormat}
        onFormatChange={setActiveFormat}
      />

      {/* Code viewer with animations */}
      <div className="flex-1 overflow-y-auto scrollbar-thin scrollbar-track-transparent scrollbar-thumb-white/10 p-4">
        <AnimatePresence mode="wait">
          <motion.div
            key={activeFormat}
            initial={{ opacity: 0, y: 8, scale: 0.98 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -8, scale: 0.98 }}
            transition={{
              duration: 0.15,
              ease: [0.25, 0.46, 0.45, 0.94],
            }}
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

      {/* Premium footer with format info and stats */}
      <div
        className="px-4 py-2.5 flex items-center justify-between"
        style={{
          background: `linear-gradient(180deg, ${glass.surface[1]}, ${glass.surface[0]})`,
          borderTop: `1px solid ${glass.border.subtle}`,
        }}
      >
        <div className="flex items-center gap-2">
          <div
            className="w-2 h-2 rounded-full"
            style={{ background: formatConfig.color }}
          />
          <span className="text-[10px] text-white/40">
            {formatConfig.description}
          </span>
        </div>
        <div className="flex items-center gap-3 text-[10px] text-white/30">
          <span>{lineCount} lines</span>
          <span>{code.length} chars</span>
        </div>
      </div>
    </div>
  );
});

export default CodeTab;
