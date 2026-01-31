'use client';

/**
 * Unified Tooltip System
 *
 * Design System:
 * - Glassmorphism: bg-black/90 + backdrop-blur + border
 * - Spring animations with scale + fade
 * - Keyboard shortcut support via <kbd>
 * - Accessible: Radix primitives + aria-labels
 *
 * @version 1.0.0
 */

import * as React from 'react';
import * as TooltipPrimitive from '@radix-ui/react-tooltip';
import { cn } from '@/lib/utils';
import { glassClasses, opacity } from '@/design/tokens';

// =============================================================================
// Provider - Wrap app root for global tooltip config
// =============================================================================

const TooltipProvider = TooltipPrimitive.Provider;

// =============================================================================
// Core Components
// =============================================================================

const Tooltip = TooltipPrimitive.Root;
const TooltipTrigger = TooltipPrimitive.Trigger;

// =============================================================================
// TooltipContent - Glassmorphism styled
// =============================================================================

const TooltipContent = React.forwardRef<
  React.ElementRef<typeof TooltipPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Content>
>(({ className, sideOffset = 6, ...props }, ref) => (
  <TooltipPrimitive.Portal>
    <TooltipPrimitive.Content
      ref={ref}
      sideOffset={sideOffset}
      className={cn(
        // Glassmorphism with tokens
        'z-50 overflow-hidden rounded-lg',
        glassClasses.heavy,
        'px-2.5 py-1.5',
        // Typography
        'text-[11px] text-white/90 font-medium',
        // Animations
        'animate-in fade-in-0 zoom-in-95',
        'data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95',
        'data-[side=bottom]:slide-in-from-top-2',
        'data-[side=left]:slide-in-from-right-2',
        'data-[side=right]:slide-in-from-left-2',
        'data-[side=top]:slide-in-from-bottom-2',
        'origin-[--radix-tooltip-content-transform-origin]',
        className
      )}
      {...props}
    />
  </TooltipPrimitive.Portal>
));
TooltipContent.displayName = TooltipPrimitive.Content.displayName;

// =============================================================================
// TooltipShortcut - Keyboard shortcut badge
// =============================================================================

interface TooltipShortcutProps {
  children: React.ReactNode;
}

const TooltipShortcut = ({ children }: TooltipShortcutProps) => (
  <kbd
    className={cn(
      'ml-1.5 px-1 py-0.5 rounded',
      `bg-${opacity.bg.heavy} border border-${opacity.bg.heavy}`,
      'text-[9px] text-white/60 font-mono'
    )}
  >
    {children}
  </kbd>
);

export {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
  TooltipProvider,
  TooltipShortcut,
};
