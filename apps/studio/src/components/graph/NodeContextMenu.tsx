'use client';

/**
 * NodeContextMenu - Right-click context menu for graph nodes
 *
 * Neo4j Browser-style context menu providing quick actions:
 * - Expand Neighbors: Fetch and display connected nodes
 * - Hide Node: Temporarily remove node from view
 * - Copy ID: Copy node key/ID to clipboard
 *
 * Features:
 * - Glassmorphism styling (dark glass with blur)
 * - Closes on click outside or after action
 * - Keyboard accessible (Escape to close)
 */

import { useCallback, useEffect, useRef } from 'react';
import { cn } from '@/lib/utils';
import { glassClasses, iconSizes, gapTokens } from '@/design/tokens';
import { ACTION_ICONS } from '@/config/iconSystem';
import { useNodeExpansion, useLatestRef } from '@/hooks';
import { useGraphStore } from '@/stores/graphStore';
import { copyToClipboard } from '@/lib/clipboard';

// Design system icons
const ExpandIcon = ACTION_ICONS.expand;
const HideIcon = ACTION_ICONS.hide;
const CopyIcon = ACTION_ICONS.copy;

interface NodeContextMenuProps {
  /** ID of the node to show menu for */
  nodeId: string;
  /** Screen position to render the menu */
  position: { x: number; y: number };
  /** Callback when menu should close */
  onClose: () => void;
}

/**
 * Context menu item button component
 */
function MenuItem({
  icon: Icon,
  label,
  onClick,
  className,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  onClick: () => void;
  className?: string;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        cn('w-full flex items-center px-3 py-2 text-sm text-left', gapTokens.spacious),
        'text-white/80 hover:bg-white/10 hover:text-white',
        'transition-colors',
        className
      )}
    >
      <Icon className={cn(iconSizes.md, 'text-white/60')} />
      <span>{label}</span>
    </button>
  );
}

export function NodeContextMenu({ nodeId, position, onClose }: NodeContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);
  const { expandNode } = useNodeExpansion();
  const hideNode = useGraphStore((state) => state.hideNode);
  const getNodeById = useGraphStore((state) => state.getNodeById);
  const node = getNodeById(nodeId);

  // Handle expand action
  const handleExpand = useCallback(async () => {
    await expandNode(nodeId);
    onClose();
  }, [expandNode, nodeId, onClose]);

  // Handle hide action
  const handleHide = useCallback(() => {
    hideNode(nodeId);
    onClose();
  }, [hideNode, nodeId, onClose]);

  // Handle copy ID action
  const handleCopyId = useCallback(async () => {
    const textToCopy = node?.key || nodeId;
    await copyToClipboard(textToCopy);
    onClose();
  }, [node, nodeId, onClose]);

  // Use ref for onClose to avoid stale closure in event handlers
  const onCloseRef = useLatestRef(onClose);

  // Close on click outside or Escape key
  useEffect(() => {
    // Use ref object to track listener state (primitives are captured by value in closures)
    // This ensures cleanup reads the current value, not the initial captured value
    const listenersAddedRef = { current: false };

    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onCloseRef.current();
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onCloseRef.current();
      }
    };

    // Add listeners with a small delay to prevent immediate close from the triggering click
    const timeoutId = setTimeout(() => {
      document.addEventListener('mousedown', handleClickOutside);
      document.addEventListener('keydown', handleKeyDown);
      listenersAddedRef.current = true;
    }, 0);

    return () => {
      clearTimeout(timeoutId);
      // Only remove listeners if they were actually added
      if (listenersAddedRef.current) {
        document.removeEventListener('mousedown', handleClickOutside);
        document.removeEventListener('keydown', handleKeyDown);
      }
    };
  }, [onCloseRef]);

  // Adjust position to keep menu within viewport
  const adjustedPosition = {
    x: Math.min(position.x, window.innerWidth - 180),
    y: Math.min(position.y, window.innerHeight - 150),
  };

  return (
    <div
      ref={menuRef}
      className={cn(
        'fixed z-[100] min-w-[160px] py-1 rounded-lg',
        glassClasses.heavy,
        'animate-in fade-in-0 zoom-in-95 duration-100'
      )}
      style={{
        left: adjustedPosition.x,
        top: adjustedPosition.y,
      }}
    >
      <MenuItem
        icon={ExpandIcon}
        label="Expand Neighbors"
        onClick={handleExpand}
      />
      <MenuItem
        icon={HideIcon}
        label="Hide Node"
        onClick={handleHide}
      />
      <div className="h-px bg-white/10 my-1 mx-2" />
      <MenuItem
        icon={CopyIcon}
        label="Copy ID"
        onClick={handleCopyId}
      />
    </div>
  );
}
