'use client';

/**
 * Toaster Component
 *
 * Sonner-based toast provider with NovaNet glassmorphism styling.
 * Place in RootLayout to enable toasts throughout the app.
 *
 * @example
 * // In layout.tsx
 * import { Toaster } from '@/components/ui/Toaster';
 *
 * export default function RootLayout({ children }) {
 *   return (
 *     <html>
 *       <body>
 *         {children}
 *         <Toaster />
 *       </body>
 *     </html>
 *   );
 * }
 */

import { Toaster as SonnerToaster } from 'sonner';
import { zIndex, glassClasses } from '@/design/tokens';

export interface ToasterProps {
  /** Position of toast container */
  position?: 'top-left' | 'top-center' | 'top-right' | 'bottom-left' | 'bottom-center' | 'bottom-right';
  /** Whether to expand toasts on hover */
  expand?: boolean;
  /** Whether to show close button */
  closeButton?: boolean;
  /** Duration before auto-dismiss (ms) */
  duration?: number;
  /** Maximum number of visible toasts */
  visibleToasts?: number;
}

export function Toaster({
  position = 'bottom-right',
  expand = false,
  closeButton = true,
  duration = 3000,
  visibleToasts = 4,
}: ToasterProps) {
  return (
    <SonnerToaster
      position={position}
      expand={expand}
      closeButton={closeButton}
      duration={duration}
      visibleToasts={visibleToasts}
      gap={8}
      offset={16}
      theme="dark"
      className="toaster-group"
      toastOptions={{
        unstyled: false,
        classNames: {
          toast: [
            // Glass morphism base
            'group toast',
            glassClasses.heavy,
            'rounded-xl',
            'shadow-2xl shadow-black/50',
            'ring-1 ring-white/5 ring-inset',
            // Text
            'text-white/90',
            'font-sans',
          ].join(' '),
          title: 'text-white/95 font-medium text-sm',
          description: 'text-white/65 text-xs',
          actionButton: [
            'bg-white/10',
            'text-white/90',
            'hover:bg-white/20',
            'border border-white/20',
            'rounded-lg',
            'text-xs font-medium',
          ].join(' '),
          cancelButton: [
            'bg-transparent',
            'text-white/50',
            'hover:text-white/80',
            'text-xs',
          ].join(' '),
          closeButton: [
            '!bg-white/10',
            '!border-white/20',
            '!text-white/60',
            'hover:!bg-white/20',
            'hover:!text-white',
          ].join(' '),
          // Type-specific styles
          success: '!border-emerald-500/30 [&>svg]:text-emerald-400',
          error: '!border-red-500/30 [&>svg]:text-red-400',
          warning: '!border-amber-500/30 [&>svg]:text-amber-400',
          info: '!border-cyan-500/30 [&>svg]:text-cyan-400',
          loading: '!border-white/20 [&>svg]:text-white/60',
        },
        style: {
          // Ensure toasts appear above everything
          zIndex: zIndex.toast,
        },
      }}
    />
  );
}

export default Toaster;
