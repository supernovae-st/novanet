'use client';

/**
 * GraphToolbar - Unified glass design system
 *
 * Design:
 * - Glass containers matching minimap style
 * - Vertical layouts panel (collapsible)
 * - Horizontal zoom/fit bar (always visible)
 * - Connected by glowing line
 *
 * @version 7.5.0
 */

import { memo, useCallback, useState } from 'react';
import { useReactFlow } from '@xyflow/react';
import { useSmartFitView } from '@/hooks';
import {
  Plus,
  Minus,
  Maximize2,
  Rows3,
  Layers,
  LayoutGrid,
  Target,
  Atom,
  ChevronUp,
  ChevronDown,
  Magnet,
  Box,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores';
import type { LayoutDirection } from '@/stores/uiStore';
import { MINIMAP_WIDTH } from '@/config/layoutConstants';
import { controls, easing, durations, gapTokens } from '@/design/tokens';
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipShortcut,
} from '@/components/ui/Tooltip';

// =============================================================================
// Design System Tokens (from centralized tokens.ts)
// =============================================================================

const { buttonSize: BUTTON_SIZE, iconSize: ICON_SIZE, gap: GAP } = controls;
const SPRING_EASE = easing.spring;
const TRANSITION_DURATION = durations.faster; // 100ms for micro-interactions

/** Haptic feedback for button press */
function triggerHaptic() {
  if (typeof navigator !== 'undefined' && navigator.vibrate) {
    navigator.vibrate(8);
  }
}

// =============================================================================
// ToolbarButton
// =============================================================================

interface ToolbarButtonProps {
  icon: React.ReactNode;
  label: string;
  shortcut?: string;
  onClick: () => void;
  isActive?: boolean;
}

const ToolbarButton = memo(function ToolbarButton({
  icon,
  label,
  shortcut,
  onClick,
  isActive = false,
}: ToolbarButtonProps) {
  const handleClick = useCallback(() => {
    triggerHaptic();
    onClick();
  }, [onClick]);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <button
          onClick={handleClick}
          className={cn(
            'group relative flex items-center justify-center',
            'rounded-xl',
            'transition', // Duration + easing set via style for token consistency
            // opacity.bg.light = white/[0.04]
            // opacity.border.light = white/10
            'bg-white/[0.04] border border-white/10',
            'hover:bg-white/[0.10] hover:border-white/[0.15] hover:scale-105',
            isActive && [
              'bg-accent-blue/20 border-accent-blue/30',
              'shadow-[0_0_20px_-4px_rgba(94,106,210,0.6)]',
            ],
            'active:scale-90 active:bg-white/[0.15]',
            'focus:outline-none focus-visible:ring-2 focus-visible:ring-accent-blue/50'
          )}
          style={{
            width: BUTTON_SIZE,
            height: BUTTON_SIZE,
            transitionDuration: `${TRANSITION_DURATION}ms`,
            transitionTimingFunction: SPRING_EASE,
          }}
          aria-label={shortcut ? `${label} (${shortcut})` : label}
        >
          <span
            className={cn(
              'transition-colors', // Duration + easing set via style
              isActive ? 'text-accent-blue' : 'text-white/50',
              'group-hover:text-white/90',
              'group-active:text-white'
            )}
            style={{
              transitionDuration: `${TRANSITION_DURATION}ms`,
              transitionTimingFunction: SPRING_EASE,
            }}
          >
            {icon}
          </span>
        </button>
      </TooltipTrigger>
      <TooltipContent side="top">
        {label}
        {shortcut && <TooltipShortcut>{shortcut}</TooltipShortcut>}
      </TooltipContent>
    </Tooltip>
  );
});

// =============================================================================
// LayoutButton - For vertical panel, tooltip on left, NO active state
// =============================================================================

interface LayoutButtonProps {
  icon: React.ReactNode;
  label: string;
  shortcut?: string;
  onClick: () => void;
}

const LayoutButton = memo(function LayoutButton({
  icon,
  label,
  shortcut,
  onClick,
}: LayoutButtonProps) {
  const handleClick = useCallback(() => {
    triggerHaptic();
    onClick();
  }, [onClick]);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <button
          onClick={handleClick}
          className={cn(
            'group relative flex items-center',
            gapTokens.default,
            'rounded-xl px-2.5',
            'transition',
            // opacity.bg.light = white/[0.04]
            // opacity.border.light = white/10
            'bg-white/[0.04] border border-white/10',
            'hover:bg-white/[0.10] hover:border-white/[0.15] hover:scale-105',
            'active:scale-95 active:bg-white/[0.15]',
            'focus:outline-none focus-visible:ring-2 focus-visible:ring-accent-blue/50'
          )}
          style={{
            height: BUTTON_SIZE,
            transitionDuration: `${TRANSITION_DURATION}ms`,
            transitionTimingFunction: SPRING_EASE,
          }}
          aria-label={shortcut ? `${label} (${shortcut})` : label}
        >
          {/* Icon */}
          <span
            className={cn(
              'transition-colors',
              'text-white/50',
              'group-hover:text-white/90',
              'group-active:text-white'
            )}
            style={{
              transitionDuration: `${TRANSITION_DURATION}ms`,
              transitionTimingFunction: SPRING_EASE,
            }}
          >
            {icon}
          </span>
          {/* Shortcut kbd - prominent styling */}
          {shortcut && (
            <kbd
              className={cn(
                'inline-flex items-center justify-center',
                'min-w-[28px] h-5 px-1.5',
                // opacity.bg.medium = white/[0.06]
                // opacity.border.light = white/10
                'bg-white/[0.06] border border-white/10 rounded',
                'text-[10px] font-mono text-white/50',
                'group-hover:bg-white/[0.12] group-hover:text-white/80 group-hover:border-white/20',
                'transition-colors'
              )}
            >
              {shortcut}
            </kbd>
          )}
        </button>
      </TooltipTrigger>
      <TooltipContent side="left">
        {label}
      </TooltipContent>
    </Tooltip>
  );
});

// =============================================================================
// GraphToolbar
// =============================================================================

// =============================================================================
// Layout Labels - Dynamic based on navigation mode
// =============================================================================

const META_LABELS = {
  LR: 'Swimlanes',
  TB: 'Stacked',
  dagre: 'Treemap',
  radial: 'Target',
  force: 'Force',
} as const;

const DATA_LABELS = {
  LR: 'Horizontal',
  TB: 'Vertical',
  dagre: 'Dagre',
  radial: 'Radial',
  force: 'Force',
} as const;

export const GraphToolbar = memo(function GraphToolbar() {
  const [isExpanded, setIsExpanded] = useState(false);
  const { zoomIn, zoomOut } = useReactFlow();
  const { smartFitView } = useSmartFitView();

  const { triggerLayout, navigationMode, layoutMode, toggleLayoutMode } = useUIStore(
    useShallow((state) => ({
      triggerLayout: state.triggerLayout,
      navigationMode: state.navigationMode,
      layoutMode: state.layoutMode,
      toggleLayoutMode: state.toggleLayoutMode,
    }))
  );

  // Pick labels based on current mode
  const labels = navigationMode === 'meta' ? META_LABELS : DATA_LABELS;

  const toggleExpanded = useCallback(() => {
    setIsExpanded((prev) => !prev);
  }, []);

  const handleZoomIn = useCallback(() => {
    zoomIn({ duration: 200 });
  }, [zoomIn]);

  const handleZoomOut = useCallback(() => {
    zoomOut({ duration: 200 });
  }, [zoomOut]);

  const handleFitView = useCallback(() => {
    smartFitView({ duration: 400 });
  }, [smartFitView]);

  const handleSetLayout = useCallback(
    (layout: LayoutDirection) => {
      triggerLayout(layout);
    },
    [triggerLayout]
  );

  const handleToggleLayoutMode = useCallback(() => {
    triggerHaptic();
    toggleLayoutMode();
  }, [toggleLayoutMode]);

  return (
    <div className="flex flex-col items-end" style={{ gap: GAP }}>
      {/* ═══════════════════════════════════════════════════════════════════════
          LAYOUTS PANEL - Vertical column, aligned RIGHT (above collapse button)
          ═══════════════════════════════════════════════════════════════════════ */}
      <div
        className={cn(
          'flex flex-col items-end',
          'transition-all duration-200',
          isExpanded
            ? 'max-h-[400px] opacity-100 overflow-visible'
            : 'max-h-0 opacity-0 overflow-hidden'
        )}
        style={{
          gap: GAP,
          transitionTimingFunction: SPRING_EASE,
        }}
      >
        <LayoutButton
          icon={<Rows3 size={ICON_SIZE} />}
          label={labels.LR}
          shortcut="⇧H"
          onClick={() => handleSetLayout('LR')}
        />
        <LayoutButton
          icon={<Layers size={ICON_SIZE} />}
          label={labels.TB}
          shortcut="⇧V"
          onClick={() => handleSetLayout('TB')}
        />
        <LayoutButton
          icon={<LayoutGrid size={ICON_SIZE} />}
          label={labels.dagre}
          shortcut="⇧D"
          onClick={() => handleSetLayout('dagre')}
        />
        <LayoutButton
          icon={<Target size={ICON_SIZE} />}
          label={labels.radial}
          shortcut="⇧R"
          onClick={() => handleSetLayout('radial')}
        />
        <LayoutButton
          icon={<Atom size={ICON_SIZE} />}
          label={labels.force}
          shortcut="⇧F"
          onClick={() => handleSetLayout('force')}
        />
        {/* Separator */}
        <div className="w-full h-px bg-white/10 my-1" />
        {/* Layout Mode Toggle */}
        <Tooltip>
          <TooltipTrigger asChild>
            <button
              onClick={handleToggleLayoutMode}
              className={cn(
                'group relative flex items-center',
                gapTokens.default,
                'rounded-xl px-2.5',
                'transition',
                layoutMode === 'magnetic'
                  ? 'bg-violet-500/20 text-violet-300 border border-violet-500/40'
                  : 'bg-white/[0.04] text-white/60 border border-white/10',
                'hover:bg-white/[0.10] hover:border-white/[0.15] hover:scale-105',
                'active:scale-95 active:bg-white/[0.15]',
                'focus:outline-none focus-visible:ring-2 focus-visible:ring-violet-500/50'
              )}
              style={{
                height: BUTTON_SIZE,
                transitionDuration: `${TRANSITION_DURATION}ms`,
                transitionTimingFunction: SPRING_EASE,
              }}
              aria-label="Toggle magnetic grouping (Shift+M)"
            >
              <span
                className={cn(
                  'transition-colors',
                  layoutMode === 'magnetic' ? 'text-violet-400' : 'text-white/50',
                  'group-hover:text-white/90'
                )}
                style={{
                  transitionDuration: `${TRANSITION_DURATION}ms`,
                  transitionTimingFunction: SPRING_EASE,
                }}
              >
                {layoutMode === 'magnetic' ? (
                  <Magnet size={ICON_SIZE} />
                ) : (
                  <Box size={ICON_SIZE} />
                )}
              </span>
              <kbd
                className={cn(
                  'inline-flex items-center justify-center',
                  'min-w-[28px] h-5 px-1.5',
                  layoutMode === 'magnetic'
                    ? 'bg-violet-500/20 border border-violet-500/30 text-violet-300'
                    : 'bg-white/[0.06] border border-white/10 text-white/50',
                  'rounded text-[10px] font-mono',
                  'group-hover:bg-white/[0.12] group-hover:text-white/80 group-hover:border-white/20',
                  'transition-colors'
                )}
              >
                ⇧M
              </kbd>
            </button>
          </TooltipTrigger>
          <TooltipContent side="left">
            {layoutMode === 'magnetic' ? 'Magnetic' : 'Containers'}
          </TooltipContent>
        </Tooltip>
      </div>

      {/* ═══════════════════════════════════════════════════════════════════════
          MAIN BAR - Horizontal, same width as minimap
          [+] [−] [↗] [∧] - all gaps uniform (8px)
          ═══════════════════════════════════════════════════════════════════════ */}
      <div
        className="flex items-center"
        style={{ width: MINIMAP_WIDTH, gap: GAP }}
      >
        <ToolbarButton
          icon={<Plus size={ICON_SIZE} strokeWidth={2.5} />}
          label="Zoom In"
          shortcut="+"
          onClick={handleZoomIn}
        />
        <ToolbarButton
          icon={<Minus size={ICON_SIZE} strokeWidth={2.5} />}
          label="Zoom Out"
          shortcut="-"
          onClick={handleZoomOut}
        />
        <ToolbarButton
          icon={<Maximize2 size={ICON_SIZE} />}
          label="Fit View"
          shortcut="F"
          onClick={handleFitView}
        />
        <ToolbarButton
          icon={isExpanded ? <ChevronDown size={ICON_SIZE} /> : <ChevronUp size={ICON_SIZE} />}
          label={isExpanded ? 'Hide layouts' : 'Show layouts'}
          onClick={toggleExpanded}
          isActive={isExpanded}
        />
      </div>
    </div>
  );
});
