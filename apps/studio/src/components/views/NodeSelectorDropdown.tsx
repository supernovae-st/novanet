'use client';

/**
 * NodeSelectorDropdown - Select a node for contextual views
 *
 * Appears when clicking a contextual view without a node selected.
 * Fetches nodes by type from Neo4j and displays them in a searchable list.
 *
 * Uses the unified Modal component for consistent styling.
 */

import { useState, useEffect, useRef, memo } from 'react';
import { Search, Loader2, Database, Layout } from 'lucide-react';
import { cn } from '@/lib/utils';
import { modalClasses, gapTokens, iconSizes } from '@/design/tokens';
import { Modal } from '@/components/ui/Modal';

// =============================================================================
// Types
// =============================================================================

interface NodeOption {
  key: string;
  displayName: string;
  type: string;
}

interface NodeSelectorDropdownProps {
  /** Types of nodes to show (e.g., ['Page', 'Block']) */
  applicableTypes: string[];
  /** Called when a node is selected */
  onSelect: (nodeKey: string) => void;
  /** Called when dropdown is closed without selection */
  onClose: () => void;
  /** View name for context */
  viewName?: string;
}

// =============================================================================
// Component
// =============================================================================

export const NodeSelectorDropdown = memo(function NodeSelectorDropdown({
  applicableTypes,
  onSelect,
  onClose,
  viewName,
}: NodeSelectorDropdownProps) {
  const [nodes, setNodes] = useState<NodeOption[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [focusedIndex, setFocusedIndex] = useState(0);

  const searchInputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  // Fetch nodes on mount
  useEffect(() => {
    const fetchNodes = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const typesParam = applicableTypes.join(',');
        const res = await fetch(`/api/graph/nodes-by-types?types=${typesParam}&limit=50`);
        const json = await res.json();

        if (json.success) {
          setNodes(json.data.nodes);
        } else {
          setError(json.error || 'Failed to load nodes');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load nodes');
      } finally {
        setIsLoading(false);
      }
    };

    fetchNodes();
  }, [applicableTypes]);

  // Focus search input on mount
  useEffect(() => {
    const timer = setTimeout(() => {
      searchInputRef.current?.focus();
    }, 50);
    return () => clearTimeout(timer);
  }, []);

  // Filter nodes by search
  const filteredNodes = nodes.filter((node) => {
    if (!searchQuery) return true;
    const query = searchQuery.toLowerCase();
    return (
      node.key.toLowerCase().includes(query) ||
      node.displayName.toLowerCase().includes(query)
    );
  });

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          setFocusedIndex((i) => Math.min(i + 1, filteredNodes.length - 1));
          break;
        case 'ArrowUp':
          e.preventDefault();
          setFocusedIndex((i) => Math.max(i - 1, 0));
          break;
        case 'Enter':
          e.preventDefault();
          if (filteredNodes[focusedIndex]) {
            onSelect(filteredNodes[focusedIndex].key);
          }
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [filteredNodes, focusedIndex, onSelect]);

  // Reset focused index when search changes
  useEffect(() => {
    setFocusedIndex(0);
  }, [searchQuery]);

  // Scroll focused item into view
  useEffect(() => {
    if (listRef.current && filteredNodes.length > 0) {
      const focusedElement = listRef.current.children[focusedIndex] as HTMLElement;
      focusedElement?.scrollIntoView({ block: 'nearest' });
    }
  }, [focusedIndex, filteredNodes.length]);

  return (
    <Modal.Root isOpen={true} onClose={onClose} closeOnEscape={true} closeOnOutsideClick={true}>
      <Modal.Content size="sm" ariaLabel={`Select a ${applicableTypes.join(' or ')}`}>
        {/* Header */}
        <div className={modalClasses.header}>
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className="w-8 h-8 rounded-lg bg-primary/20 flex items-center justify-center shrink-0">
              <Layout className={cn(iconSizes.sm, 'text-primary')} />
            </div>
            <div>
              <h2 className="text-sm font-semibold text-white">
                Select a {applicableTypes.join(' or ')}
              </h2>
              {viewName && (
                <p className="text-xs text-white/40">for {viewName}</p>
              )}
            </div>
          </div>
          <button
            onClick={onClose}
            aria-label="Close"
            className={modalClasses.closeButton}
          >
            <svg className={iconSizes.md} fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Search */}
        <div className="px-4 py-3 border-b border-white/[0.08]">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-white/40" />
            <input
              ref={searchInputRef}
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search nodes..."
              className={cn(
                'w-full pl-10 pr-4 py-2.5 rounded-xl',
                'bg-white/[0.04] border border-white/[0.08]',
                'text-sm text-white placeholder:text-white/40',
                'focus:outline-none focus:border-white/20 focus:bg-white/[0.06]',
                'transition-colors duration-150'
              )}
            />
          </div>
        </div>

        {/* Content */}
        <Modal.Body maxHeight="300px">
          {isLoading ? (
            <div className={cn('flex items-center justify-center py-12', gapTokens.default)}>
              <Loader2 className="w-5 h-5 text-white/50 animate-spin" />
              <span className="text-sm text-white/50">Loading nodes...</span>
            </div>
          ) : error ? (
            <div className="px-4 py-12 text-center">
              <p className="text-sm text-red-400">{error}</p>
            </div>
          ) : filteredNodes.length === 0 ? (
            <div className="px-4 py-12 text-center">
              <Database className="w-10 h-10 mx-auto mb-3 text-white/20" />
              <p className="text-sm text-white/50">
                {searchQuery ? 'No matching nodes' : 'No nodes found'}
              </p>
            </div>
          ) : (
            <div ref={listRef} className="py-2">
              {filteredNodes.map((node, index) => (
                <button
                  key={node.key}
                  onClick={() => onSelect(node.key)}
                  className={cn(
                    'w-full px-4 py-2.5 text-left',
                    'flex items-center justify-between',
                    'transition-colors duration-100',
                    index === focusedIndex
                      ? 'bg-white/[0.08]'
                      : 'hover:bg-white/[0.04]'
                  )}
                >
                  <div className="flex flex-col min-w-0 mr-3">
                    <span className="text-sm text-white/90 truncate">
                      {node.displayName || node.key}
                    </span>
                    {node.displayName && node.displayName !== node.key && (
                      <span className="text-xs text-white/40 font-mono truncate">
                        {node.key}
                      </span>
                    )}
                  </div>
                  <span className={cn(
                    'shrink-0 px-2 py-0.5 rounded-md text-[10px] font-medium uppercase tracking-wide',
                    'bg-white/[0.06] text-white/50 border border-white/[0.06]'
                  )}>
                    {node.type}
                  </span>
                </button>
              ))}
            </div>
          )}
        </Modal.Body>

        {/* Footer */}
        <Modal.Footer>
          <div className="flex items-center justify-between text-xs text-white/40">
            <span>{filteredNodes.length} node{filteredNodes.length !== 1 ? 's' : ''}</span>
            <div className={cn('flex items-center', gapTokens.spacious)}>
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 rounded bg-white/[0.06] text-[10px] font-mono">↑↓</kbd>
                Navigate
              </span>
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 rounded bg-white/[0.06] text-[10px] font-mono">↵</kbd>
                Select
              </span>
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 rounded bg-white/[0.06] text-[10px] font-mono">Esc</kbd>
                Close
              </span>
            </div>
          </div>
        </Modal.Footer>
      </Modal.Content>
    </Modal.Root>
  );
});

export default NodeSelectorDropdown;
