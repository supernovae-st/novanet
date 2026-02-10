'use client';

/**
 * ActionCard - Context view action card with ASCII preview
 *
 * Features:
 * - Icon + label header
 * - Node/Arc count stats
 * - ASCII preview (Tree/Flow/Compact)
 * - Arc type summary footer
 * - Hover glow effect with view color
 * - Click triggers view load with Matrix transition
 */

import { memo, useState, useCallback } from 'react';
import { motion } from 'motion/react';
import { cn } from '@/lib/utils';
import { AsciiPreview } from './AsciiPreview';
import type { ContextView } from '@/hooks/useContextViews';

// =============================================================================
// TYPES
// =============================================================================

interface ActionCardProps {
  view: ContextView;
  nodeKey: string;
  isActive?: boolean;
  onClick: (viewId: string) => void;
  className?: string;
}

// =============================================================================
// COMPONENT
// =============================================================================

export const ActionCard = memo(function ActionCard({
  view,
  nodeKey,
  isActive = false,
  onClick,
  className,
}: ActionCardProps) {
  const [isHovered, setIsHovered] = useState(false);

  const handleClick = useCallback(() => {
    onClick(view.id);
  }, [onClick, view.id]);

  const { stats, transitionColor } = view;

  // Format arc summary for footer
  const arcSummary = Object.entries(stats.arcsByType)
    .slice(0, 3)
    .map(([type, count]) => `${type.replace('HAS_', '').replace('_', '')}:${count}`)
    .join(' ');

  return (
    <motion.button
      onClick={handleClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className={cn(
        'relative flex flex-col w-[160px] min-w-[160px] h-[140px]',
        'rounded-lg border transition-all duration-200',
        'bg-black/40 backdrop-blur-sm',
        isActive
          ? 'border-white/30 bg-white/10'
          : 'border-white/10 hover:border-white/20',
        'focus:outline-none focus-visible:ring-2 focus-visible:ring-white/30',
        'overflow-hidden',
        className
      )}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
    >
      {/* Glow effect on hover */}
      {isHovered && (
        <motion.div
          className="absolute inset-0 opacity-20 rounded-lg pointer-events-none"
          initial={{ opacity: 0 }}
          animate={{ opacity: 0.15 }}
          style={{
            background: `radial-gradient(circle at center, ${transitionColor}, transparent 70%)`,
          }}
        />
      )}

      {/* Border beam effect when active */}
      {isActive && (
        <motion.div
          className="absolute inset-0 rounded-lg pointer-events-none"
          style={{
            background: `linear-gradient(90deg, transparent, ${transitionColor}40, transparent)`,
            backgroundSize: '200% 100%',
          }}
          animate={{
            backgroundPosition: ['0% 0%', '200% 0%'],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'linear',
          }}
        />
      )}

      {/* Header: Icon + Label + Stats */}
      <div className="flex items-center justify-between px-2.5 py-1.5 border-b border-white/5">
        <div className="flex items-center gap-1.5 min-w-0">
          <span className="text-sm flex-shrink-0">{view.icon}</span>
          <span className="text-xs font-medium text-white truncate">
            {view.label}
          </span>
        </div>
        <div className="flex items-center gap-1 text-[10px] text-white/50 flex-shrink-0">
          <span>{stats.nodeCount}n</span>
          <span className="text-white/20">·</span>
          <span>{stats.arcCount}a</span>
        </div>
      </div>

      {/* ASCII Preview */}
      <div className="flex-1 px-2 py-1.5 overflow-hidden">
        <AsciiPreview
          viewId={view.id}
          style={view.style}
          stats={stats}
          nodeKey={nodeKey}
        />
      </div>

      {/* Footer: Arc types summary */}
      <div className="px-2.5 py-1 border-t border-white/5 bg-black/20">
        <div className="text-[9px] text-white/40 font-mono truncate">
          {arcSummary || 'No connections'}
        </div>
      </div>

      {/* Depth indicator (for tree views) */}
      {view.style === 'tree' && stats.depth && (
        <div
          className="absolute top-1.5 right-1.5 text-[9px] text-white/30 font-mono"
          title={`${stats.depth} levels deep`}
        >
          {stats.depth}lvl
        </div>
      )}

      {/* Completion indicator (for compact views) */}
      {view.style === 'compact' && stats.completion !== undefined && (
        <div
          className="absolute top-1.5 right-1.5 text-[9px] font-mono"
          style={{
            color: stats.completion === 100
              ? '#22c55e'
              : stats.completion >= 60
                ? '#f59e0b'
                : '#ef4444',
          }}
          title={`${stats.completion}% complete`}
        >
          {stats.completion}%
        </div>
      )}
    </motion.button>
  );
});

export default ActionCard;
