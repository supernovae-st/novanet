'use client';

/**
 * GraphViewToggle - Segmented control for 2D/3D view switching
 *
 * Features:
 * - Compact pill-shaped toggle
 * - Animated selection indicator
 * - Keyboard shortcut hint (V)
 * - Integrates with uiStore viewMode
 */

import { memo } from 'react';
import { motion } from 'motion/react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';

interface ViewOptionProps {
  label: string;
  isActive: boolean;
  onClick: () => void;
}

/**
 * Individual view option button
 */
function ViewOption({ label, isActive, onClick }: ViewOptionProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'relative z-10 px-3 py-1 text-xs font-medium transition-colors',
        isActive ? 'text-white' : 'text-white/50 hover:text-white/70'
      )}
      title={`Switch to ${label} view (V)`}
    >
      {label}
    </button>
  );
}

export interface GraphViewToggleProps {
  className?: string;
}

export const GraphViewToggle = memo(function GraphViewToggle({
  className,
}: GraphViewToggleProps) {
  const viewMode = useUIStore((state) => state.viewMode);
  const setViewMode = useUIStore((state) => state.setViewMode);

  return (
    <div
      className={cn(
        'relative flex items-center rounded-lg bg-black/40 backdrop-blur-sm border border-white/10',
        className
      )}
    >
      {/* Animated selection indicator */}
      <motion.div
        className="absolute inset-y-0 rounded-md bg-white/20"
        initial={false}
        animate={{
          x: viewMode === '2d' ? 0 : '100%',
          width: '50%',
        }}
        transition={{
          type: 'spring',
          stiffness: 500,
          damping: 30,
        }}
      />

      {/* View options */}
      <ViewOption
        label="2D"
        isActive={viewMode === '2d'}
        onClick={() => setViewMode('2d')}
      />
      <ViewOption
        label="3D"
        isActive={viewMode === '3d'}
        onClick={() => setViewMode('3d')}
      />

      {/* Keyboard hint */}
      <span className="ml-1 mr-2 text-[10px] text-white/30 font-mono">V</span>
    </div>
  );
});

export default GraphViewToggle;
