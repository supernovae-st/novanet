/**
 * DataModeToggle Component
 *
 * Toggle between Data mode (real instances from Neo4j)
 * and Schema mode (ontological schema with 35 node types)
 *
 * The toggle only updates the store - data fetching is handled
 * by an effect in page.tsx that watches dataMode changes.
 */

'use client';

import { Database, GitBranch } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore, selectDataMode } from '@/stores/uiStore';

interface DataModeToggleProps {
  /** Additional class names */
  className?: string;
}

export function DataModeToggle({ className }: DataModeToggleProps) {
  const dataMode = useUIStore(selectDataMode);
  const toggleDataMode = useUIStore((state) => state.toggleDataMode);

  const isSchema = dataMode === 'schema';

  return (
    <button
      onClick={toggleDataMode}
      className={cn(
        'flex items-center gap-1.5 px-2 py-1 rounded-md text-xs font-medium transition-all duration-150',
        'hover:bg-white/8',
        isSchema
          ? 'text-violet-400 bg-violet-500/15 border border-violet-500/30'
          : 'text-emerald-400 bg-emerald-500/15 border border-emerald-500/30',
        className
      )}
      title={isSchema ? 'Schema Mode: Viewing ontology (35 types)' : 'Data Mode: Viewing Neo4j instances'}
    >
      {isSchema ? (
        <>
          <GitBranch className="w-3 h-3" />
          <span>Schema</span>
        </>
      ) : (
        <>
          <Database className="w-3 h-3" />
          <span>Data</span>
        </>
      )}
    </button>
  );
}

export default DataModeToggle;
