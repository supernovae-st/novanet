'use client';

/**
 * ArcCypherTab - Cypher queries for arc operations
 *
 * Features:
 * - MATCH query to find the arc
 * - CREATE query to recreate the arc
 * - DELETE query to remove the arc
 * - Copy to clipboard functionality
 *
 * v11.7 — Enhanced arc experience
 */

import { memo, useMemo, useState, useCallback } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Copy, Check, Search, Plus, Trash2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useCopyFeedback } from '@/hooks';
import { gapTokens } from '@/design/tokens';
import type { GraphEdge, GraphNode } from '@/types';

interface ArcCypherTabProps {
  arc: GraphEdge;
  sourceNode: GraphNode | null;
  targetNode: GraphNode | null;
}

type QueryType = 'match' | 'create' | 'delete';

interface QueryConfig {
  id: QueryType;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  description: string;
}

const QUERY_TYPES: QueryConfig[] = [
  { id: 'match', label: 'MATCH', icon: Search, description: 'Find this relationship' },
  { id: 'create', label: 'CREATE', icon: Plus, description: 'Recreate this relationship' },
  { id: 'delete', label: 'DELETE', icon: Trash2, description: 'Remove this relationship' },
];

/**
 * Generate MATCH query for the arc
 */
function generateMatchQuery(
  arc: GraphEdge,
  sourceNode: GraphNode | null,
  targetNode: GraphNode | null
): string {
  const sourceType = sourceNode?.type || 'Node';
  const sourceKey = sourceNode?.key || arc.source;
  const targetType = targetNode?.type || 'Node';
  const targetKey = targetNode?.key || arc.target;
  const arcType = arc.type || 'RELATES_TO';

  return `// Find this specific relationship
MATCH (source:${sourceType} {key: "${sourceKey}"})
      -[r:${arcType}]->
      (target:${targetType} {key: "${targetKey}"})
RETURN source, r, target;

// Find all relationships of this type
MATCH (s)-[r:${arcType}]->(t)
RETURN s, r, t
LIMIT 25;

// Find relationships between these nodes
MATCH (source:${sourceType} {key: "${sourceKey}"})
      -[r]->
      (target:${targetType} {key: "${targetKey}"})
RETURN type(r) AS relationType, r;`;
}

/**
 * Generate CREATE query for the arc
 */
function generateCreateQuery(
  arc: GraphEdge,
  sourceNode: GraphNode | null,
  targetNode: GraphNode | null
): string {
  const sourceType = sourceNode?.type || 'Node';
  const sourceKey = sourceNode?.key || arc.source;
  const targetType = targetNode?.type || 'Node';
  const targetKey = targetNode?.key || arc.target;
  const arcType = arc.type || 'RELATES_TO';

  // Build properties string if data exists
  const dataProps = arc.data ? Object.entries(arc.data) : [];
  const propsString = dataProps.length > 0
    ? ` {${dataProps.map(([k, v]) => `${k}: ${JSON.stringify(v)}`).join(', ')}}`
    : '';

  return `// Create this relationship (if nodes exist)
MATCH (source:${sourceType} {key: "${sourceKey}"})
MATCH (target:${targetType} {key: "${targetKey}"})
CREATE (source)-[r:${arcType}${propsString}]->(target)
RETURN source, r, target;

// Create with MERGE (idempotent)
MATCH (source:${sourceType} {key: "${sourceKey}"})
MATCH (target:${targetType} {key: "${targetKey}"})
MERGE (source)-[r:${arcType}]->(target)
${dataProps.length > 0 ? `SET ${dataProps.map(([k, v]) => `r.${k} = ${JSON.stringify(v)}`).join(', ')}` : ''}
RETURN source, r, target;`;
}

/**
 * Generate DELETE query for the arc
 */
function generateDeleteQuery(
  arc: GraphEdge,
  sourceNode: GraphNode | null,
  targetNode: GraphNode | null
): string {
  const sourceType = sourceNode?.type || 'Node';
  const sourceKey = sourceNode?.key || arc.source;
  const targetType = targetNode?.type || 'Node';
  const targetKey = targetNode?.key || arc.target;
  const arcType = arc.type || 'RELATES_TO';

  return `// Delete this specific relationship
MATCH (source:${sourceType} {key: "${sourceKey}"})
      -[r:${arcType}]->
      (target:${targetType} {key: "${targetKey}"})
DELETE r
RETURN count(r) AS deleted;

// Delete all relationships of this type between these nodes
MATCH (source:${sourceType} {key: "${sourceKey}"})
      -[r]->
      (target:${targetType} {key: "${targetKey}"})
DELETE r
RETURN count(r) AS deleted;

// WARNING: Delete ALL relationships of this type
// MATCH ()-[r:${arcType}]->()
// DELETE r
// RETURN count(r) AS deleted;`;
}

/**
 * Query type selector button
 */
function QueryTypeButton({
  config,
  isActive,
  onClick,
}: {
  config: QueryConfig;
  isActive: boolean;
  onClick: () => void;
}) {
  const Icon = config.icon;
  return (
    <button
      onClick={onClick}
      className={cn(
        'flex items-center px-3 py-1.5 rounded-lg text-xs font-medium transition-all',
        gapTokens.tight,
        isActive
          ? 'bg-white/10 text-white'
          : 'text-white/50 hover:text-white/70 hover:bg-white/5'
      )}
    >
      <Icon className="w-3.5 h-3.5" />
      {config.label}
    </button>
  );
}

/**
 * Code viewer with copy button
 */
function CodeViewer({
  code,
  onCopy,
  isCopied,
}: {
  code: string;
  onCopy: () => void;
  isCopied: boolean;
}) {
  return (
    <div className="relative group">
      {/* Copy button */}
      <button
        onClick={onCopy}
        className={cn(
          'absolute top-3 right-3 p-2 rounded-lg transition-all z-10',
          'opacity-0 group-hover:opacity-100',
          isCopied
            ? 'bg-emerald-500/20 text-emerald-400'
            : 'bg-white/10 text-white/60 hover:text-white hover:bg-white/20'
        )}
        title={isCopied ? 'Copied!' : 'Copy query'}
      >
        {isCopied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
      </button>

      {/* Code block */}
      <pre className="p-4 bg-black/40 rounded-lg overflow-x-auto">
        <code className="font-mono text-xs leading-relaxed whitespace-pre-wrap text-purple-400/80">
          {code}
        </code>
      </pre>
    </div>
  );
}

export const ArcCypherTab = memo(function ArcCypherTab({
  arc,
  sourceNode,
  targetNode,
}: ArcCypherTabProps) {
  const [queryType, setQueryType] = useState<QueryType>('match');
  const { copied, copy } = useCopyFeedback();

  // Generate query based on type
  const query = useMemo(() => {
    switch (queryType) {
      case 'match':
        return generateMatchQuery(arc, sourceNode, targetNode);
      case 'create':
        return generateCreateQuery(arc, sourceNode, targetNode);
      case 'delete':
        return generateDeleteQuery(arc, sourceNode, targetNode);
      default:
        return '';
    }
  }, [queryType, arc, sourceNode, targetNode]);

  const handleCopy = useCallback(() => {
    copy(query);
  }, [copy, query]);

  const activeConfig = QUERY_TYPES.find((q) => q.id === queryType);

  return (
    <div className="flex flex-col h-full">
      {/* Query type selector */}
      <div className={cn('flex items-center p-2 border-b border-white/[0.06]', gapTokens.tight)}>
        {QUERY_TYPES.map((config) => (
          <QueryTypeButton
            key={config.id}
            config={config}
            isActive={queryType === config.id}
            onClick={() => setQueryType(config.id)}
          />
        ))}
      </div>

      {/* Code viewer */}
      <div className="flex-1 overflow-y-auto scrollbar-thin p-4">
        <AnimatePresence mode="wait">
          <motion.div
            key={queryType}
            initial={{ opacity: 0, y: 4 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.1 }}
          >
            <CodeViewer code={query} onCopy={handleCopy} isCopied={copied} />
          </motion.div>
        </AnimatePresence>
      </div>

      {/* Footer with description */}
      <div className="px-4 py-2 border-t border-white/[0.06] bg-black/20">
        <p className="text-[10px] text-white/30 text-center">
          {activeConfig?.description}
        </p>
      </div>
    </div>
  );
});

export default ArcCypherTab;
