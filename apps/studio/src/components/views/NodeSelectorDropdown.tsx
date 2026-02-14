'use client';

/**
 * NodeSelectorDropdown - Select a node for contextual views
 *
 * Appears when clicking a contextual view without a node selected.
 * Fetches nodes by type from Neo4j and displays them in a searchable list.
 */

import { useState, useEffect, useCallback, useRef, memo } from 'react';
import { createPortal } from 'react-dom';
import { motion, AnimatePresence } from 'motion/react';
import { Search, Loader2, X, Database } from 'lucide-react';
import { cn } from '@/lib/utils';
import { glassClasses, gapTokens } from '@/design/tokens';
import { useOutsideClick } from '@/hooks';

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
  /** Position anchor element */
  anchorRect: DOMRect | null;
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
  anchorRect,
  viewName,
}: NodeSelectorDropdownProps) {
  const [nodes, setNodes] = useState<NodeOption[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [focusedIndex, setFocusedIndex] = useState(0);

  const dropdownRef = useRef<HTMLDivElement>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Close on outside click
  useOutsideClick(dropdownRef, onClose);

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
    searchInputRef.current?.focus();
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
        case 'Escape':
          e.preventDefault();
          onClose();
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [filteredNodes, focusedIndex, onSelect, onClose]);

  // Reset focused index when search changes
  useEffect(() => {
    setFocusedIndex(0);
  }, [searchQuery]);

  // Calculate dropdown position
  const dropdownStyle: React.CSSProperties = anchorRect
    ? {
        position: 'fixed',
        top: anchorRect.bottom + 8,
        left: anchorRect.left,
        minWidth: Math.max(anchorRect.width, 280),
        maxWidth: 400,
      }
    : {
        position: 'fixed',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
      };

  const content = (
    <AnimatePresence>
      <motion.div
        ref={dropdownRef}
        initial={{ opacity: 0, y: -8, scale: 0.95 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        exit={{ opacity: 0, y: -8, scale: 0.95 }}
        transition={{ duration: 0.15 }}
        style={dropdownStyle}
        className={cn(
          'z-[100] overflow-hidden rounded-xl',
          glassClasses.heavy,
          'shadow-2xl shadow-black/50',
          'flex flex-col max-h-[400px]'
        )}
      >
        {/* Header */}
        <div className={cn('px-3 py-2 border-b border-white/10', 'flex items-center justify-between')}>
          <span className="text-xs font-medium text-white/70">
            Select a {applicableTypes.join(' or ')}
          </span>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-white/10 transition-colors"
          >
            <X className="w-3.5 h-3.5 text-white/50" />
          </button>
        </div>

        {/* Search */}
        <div className={cn('px-3 py-2 border-b border-white/10')}>
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/40" />
            <input
              ref={searchInputRef}
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search nodes..."
              className={cn(
                'w-full pl-8 pr-3 py-1.5 rounded-lg',
                'bg-white/5 border border-white/10',
                'text-sm text-white placeholder:text-white/40',
                'focus:outline-none focus:border-white/20'
              )}
            />
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto">
          {isLoading ? (
            <div className={cn('flex items-center justify-center py-8', gapTokens.default)}>
              <Loader2 className="w-4 h-4 text-white/50 animate-spin" />
              <span className="text-sm text-white/50">Loading nodes...</span>
            </div>
          ) : error ? (
            <div className="px-3 py-6 text-center">
              <p className="text-sm text-red-400">{error}</p>
            </div>
          ) : filteredNodes.length === 0 ? (
            <div className="px-3 py-6 text-center">
              <Database className="w-8 h-8 mx-auto mb-2 text-white/20" />
              <p className="text-sm text-white/50">
                {searchQuery ? 'No matching nodes' : 'No nodes found'}
              </p>
            </div>
          ) : (
            <div className="py-1">
              {filteredNodes.map((node, index) => (
                <button
                  key={node.key}
                  onClick={() => onSelect(node.key)}
                  className={cn(
                    'w-full px-3 py-2 text-left',
                    'flex items-center justify-between',
                    'transition-colors',
                    index === focusedIndex
                      ? 'bg-white/10'
                      : 'hover:bg-white/5'
                  )}
                >
                  <div className="flex flex-col min-w-0">
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
                    'ml-2 px-1.5 py-0.5 rounded text-[10px] font-medium',
                    'bg-white/10 text-white/60'
                  )}>
                    {node.type}
                  </span>
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className={cn('px-3 py-2 border-t border-white/10', 'text-[10px] text-white/40')}>
          {filteredNodes.length} node{filteredNodes.length !== 1 ? 's' : ''} • ↑↓ Navigate • ↵ Select • Esc Close
        </div>
      </motion.div>
    </AnimatePresence>
  );

  return createPortal(content, document.body);
});

export default NodeSelectorDropdown;
