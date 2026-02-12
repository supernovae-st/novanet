'use client';

/**
 * ActionPickerDrawer - Bottom drawer for selecting key actions
 *
 * Gaming-style UX:
 * - Slides up from bottom
 * - Grid of action presets by category
 * - "Press a key" capture mode
 * - Hover preview feedback
 */

import { memo, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  X,
  Keyboard,
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ToggleLeft,
  Database,
  FileText,
  Layers,
  Search,
  ChevronsUp,
  ChevronsDown,
  ArrowUpToLine,
  ArrowDownToLine,
  ChevronUp,
  ChevronDown,
  HelpCircle,
  RefreshCw,
  Lightbulb,
  Palette,
  SunMedium,
  SunDim,
  Cpu,
  Trash2,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import {
  ACTION_PRESETS,
  CATEGORY_COLORS,
  CATEGORY_LABELS,
  CATEGORY_ORDER,
  type ActionPreset,
  type ActionCategory,
} from '@/config/actions';

// =============================================================================
// Icon Mapping
// =============================================================================

const ICONS: Record<string, React.ComponentType<{ className?: string }>> = {
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ToggleLeft,
  Database,
  FileText,
  Layers,
  Search,
  ChevronsUp,
  ChevronsDown,
  ArrowUpToLine,
  ArrowDownToLine,
  ChevronUp,
  ChevronDown,
  HelpCircle,
  RefreshCw,
  X,
  Lightbulb,
  Palette,
  SunMedium,
  SunDim,
  Cpu,
};

// =============================================================================
// Types
// =============================================================================

interface ActionPickerDrawerProps {
  isOpen: boolean;
  selectedKey: string | null;
  currentBinding?: { label: string; key: string } | null;
  isCapturing: boolean;
  isConnected: boolean;
  onSelectAction: (action: ActionPreset) => void;
  onStartCapture: () => void;
  onCancelCapture: () => void;
  onClear: () => void;
  onClose: () => void;
  onHoverAction: (action: ActionPreset | null) => void;
}

// =============================================================================
// Action Button Component
// =============================================================================

interface ActionButtonProps {
  action: ActionPreset;
  isSelected: boolean;
  onSelect: () => void;
  onHover: (hovering: boolean) => void;
}

const ActionButton = memo(function ActionButton({
  action,
  isSelected,
  onSelect,
  onHover,
}: ActionButtonProps) {
  const Icon = ICONS[action.icon] || Keyboard;
  const color = CATEGORY_COLORS[action.category];

  return (
    <button
      onClick={onSelect}
      onMouseEnter={() => onHover(true)}
      onMouseLeave={() => onHover(false)}
      className={cn(
        'flex flex-col items-center justify-center gap-1 p-3 rounded-xl border transition-all',
        'hover:scale-105 active:scale-95',
        isSelected
          ? 'border-white/40 bg-white/10'
          : 'border-white/[0.08] bg-white/[0.02] hover:bg-white/[0.06] hover:border-white/[0.15]'
      )}
      style={{
        boxShadow: isSelected ? `0 0 20px ${color}40` : undefined,
      }}
    >
      <span style={{ color }}><Icon className="w-5 h-5" /></span>
      <span className="text-xs font-medium text-white/90">{action.shortLabel}</span>
      <span className="text-[10px] text-white/40 font-mono">{action.key}</span>
    </button>
  );
});

// =============================================================================
// Category Section Component
// =============================================================================

interface CategorySectionProps {
  category: ActionCategory;
  actions: ActionPreset[];
  currentKeycode?: string;
  onSelectAction: (action: ActionPreset) => void;
  onHoverAction: (action: ActionPreset | null) => void;
}

const CategorySection = memo(function CategorySection({
  category,
  actions,
  currentKeycode,
  onSelectAction,
  onHoverAction,
}: CategorySectionProps) {
  const color = CATEGORY_COLORS[category];
  const label = CATEGORY_LABELS[category];

  return (
    <div>
      <div className="flex items-center gap-2 mb-2">
        <div
          className="w-2 h-2 rounded-full"
          style={{ backgroundColor: color }}
        />
        <span className="text-xs font-medium text-white/60 uppercase tracking-wider">
          {label}
        </span>
      </div>
      <div className="grid grid-cols-5 gap-2">
        {actions.map((action) => (
          <ActionButton
            key={action.id}
            action={action}
            isSelected={action.keycode === currentKeycode}
            onSelect={() => onSelectAction(action)}
            onHover={(hovering) => onHoverAction(hovering ? action : null)}
          />
        ))}
      </div>
    </div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const ActionPickerDrawer = memo(function ActionPickerDrawer({
  isOpen,
  selectedKey,
  currentBinding,
  isCapturing,
  isConnected,
  onSelectAction,
  onStartCapture,
  onCancelCapture,
  onClear,
  onClose,
  onHoverAction,
}: ActionPickerDrawerProps) {
  // Escape to close
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (isCapturing) {
          onCancelCapture();
        } else {
          onClose();
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, isCapturing, onClose, onCancelCapture]);

  const handleBackdropClick = useCallback(() => {
    if (isCapturing) {
      onCancelCapture();
    } else {
      onClose();
    }
  }, [isCapturing, onCancelCapture, onClose]);

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.15 }}
            className="absolute inset-0 bg-black/40 backdrop-blur-sm z-20"
            onClick={handleBackdropClick}
          />

          {/* Drawer */}
          <motion.div
            initial={{ y: '100%' }}
            animate={{ y: 0 }}
            exit={{ y: '100%' }}
            transition={{ duration: 0.2, ease: 'easeOut' }}
            className="absolute bottom-0 left-0 right-0 z-30 bg-[#0d0d12]/95 border-t border-white/[0.1] rounded-t-2xl backdrop-blur-xl"
          >
            {/* Header */}
            <div className="flex items-center justify-between px-6 py-4 border-b border-white/[0.06]">
              <div>
                <h3 className="text-sm font-semibold text-white">
                  What should this key do?
                </h3>
                <p className="text-xs text-white/40 mt-0.5">
                  Key position: {selectedKey || '—'}
                  {currentBinding && (
                    <span className="ml-2 text-white/60">
                      Current: {currentBinding.label}
                    </span>
                  )}
                </p>
              </div>
              <button
                onClick={onClose}
                className="p-2 rounded-lg hover:bg-white/[0.06] text-white/40 hover:text-white transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Content */}
            <div className="px-6 py-4 max-h-[50vh] overflow-y-auto">
              {/* Capture Button */}
              <button
                onClick={isCapturing ? onCancelCapture : onStartCapture}
                className={cn(
                  'w-full mb-6 py-4 rounded-xl border-2 border-dashed transition-all flex items-center justify-center gap-3',
                  isCapturing
                    ? 'border-cyan-500 bg-cyan-500/10 animate-pulse'
                    : 'border-white/[0.15] hover:border-cyan-500/50 hover:bg-cyan-500/5'
                )}
              >
                <Keyboard className={cn(
                  'w-6 h-6',
                  isCapturing ? 'text-cyan-400' : 'text-white/50'
                )} />
                <span className={cn(
                  'text-sm font-medium',
                  isCapturing ? 'text-cyan-400' : 'text-white/70'
                )}>
                  {isCapturing
                    ? isConnected
                      ? 'Press a key on your pad...'
                      : 'Press any key on your keyboard...'
                    : isConnected
                      ? '🎮 Press a key on your pad'
                      : '⌨️ Press any key to bind'
                  }
                </span>
                {isCapturing && (
                  <span className="text-xs text-cyan-400/60">(ESC to cancel)</span>
                )}
              </button>

              {/* Presets Grid */}
              <div className="space-y-5">
                {CATEGORY_ORDER.map((category) => (
                  <CategorySection
                    key={category}
                    category={category}
                    actions={ACTION_PRESETS[category]}
                    currentKeycode={currentBinding?.key}
                    onSelectAction={onSelectAction}
                    onHoverAction={onHoverAction}
                  />
                ))}
              </div>
            </div>

            {/* Footer */}
            <div className="flex items-center justify-between px-6 py-3 border-t border-white/[0.06]">
              <button
                onClick={onClear}
                className="px-4 py-2 rounded-lg text-sm text-red-400/80 hover:text-red-400 hover:bg-red-500/10 transition-colors flex items-center gap-2"
              >
                <Trash2 className="w-4 h-4" />
                Clear binding
              </button>
              <button
                onClick={onClose}
                className="px-4 py-2 rounded-lg text-sm bg-white/[0.06] text-white/70 hover:bg-white/[0.1] transition-colors"
              >
                Cancel
              </button>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
});

export default ActionPickerDrawer;
