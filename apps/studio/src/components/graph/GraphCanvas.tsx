'use client';

/**
 * GraphCanvas - View switcher wrapper for 2D/3D graph visualization
 *
 * Features:
 * - Conditionally renders Graph2D or Graph3D based on viewMode
 * - Smooth transitions between views
 * - GraphViewToggle positioned in top-right corner
 * - Unified interface for both view modes
 */

import { memo, Suspense, lazy } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Loader2 } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { GraphViewToggle } from './GraphViewToggle';
import { Graph3D, type Graph3DProps } from './Graph3D';

// Lazy load Graph2D to reduce initial bundle
const Graph2D = lazy(() =>
  import('./Graph2D').then((mod) => ({ default: mod.Graph2D }))
);

export interface GraphCanvasProps {
  /** Additional class names */
  className?: string;
  /** Show minimap (2D only) */
  showMinimap?: boolean;
  /** Show controls */
  showControls?: boolean;
  /** Callback when a node is clicked */
  onNodeClick?: (nodeId: string) => void;
  /** Callback when a node is double-clicked (expand) */
  onNodeDoubleClick?: (nodeId: string) => void;
  /** Callback when background is clicked */
  onPaneClick?: () => void;
  /** Show the 2D/3D toggle */
  showViewToggle?: boolean;
}

/**
 * Loading fallback for lazy-loaded graphs
 */
function GraphLoadingFallback() {
  return (
    <div className="absolute inset-0 flex items-center justify-center bg-slate-900/50">
      <div className="flex flex-col items-center gap-3">
        <Loader2 className="w-8 h-8 text-blue-400 animate-spin" />
        <span className="text-sm text-white/50">Loading graph...</span>
      </div>
    </div>
  );
}

export const GraphCanvas = memo(function GraphCanvas({
  className,
  showMinimap = true,
  showControls = true,
  onNodeClick,
  onNodeDoubleClick,
  onPaneClick,
  showViewToggle = true,
}: GraphCanvasProps) {
  const viewMode = useUIStore((state) => state.viewMode);

  // Common props for both graph views
  const graphProps: Graph3DProps = {
    className: 'absolute inset-0',
    showMinimap,
    showControls,
    onNodeClick,
    onNodeDoubleClick,
    onPaneClick,
  };

  return (
    <div className={cn('relative', className)}>
      {/* Graph view with crossfade transition */}
      <AnimatePresence mode="wait">
        {viewMode === '2d' ? (
          <motion.div
            key="2d"
            className="absolute inset-0"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
          >
            <Suspense fallback={<GraphLoadingFallback />}>
              <Graph2D {...graphProps} />
            </Suspense>
          </motion.div>
        ) : (
          <motion.div
            key="3d"
            className="absolute inset-0"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.2 }}
          >
            <Graph3D {...graphProps} />
          </motion.div>
        )}
      </AnimatePresence>

      {/* View toggle overlay */}
      {showViewToggle && (
        <div className="absolute top-4 right-4 z-50">
          <GraphViewToggle />
        </div>
      )}
    </div>
  );
});

export default GraphCanvas;
