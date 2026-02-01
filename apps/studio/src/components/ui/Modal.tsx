'use client';

/**
 * Modal - Base modal component with compound pattern
 *
 * Features:
 * - Portal-based rendering
 * - Focus trap (WCAG 2.1 AA)
 * - Body scroll lock
 * - Escape to close
 * - Outside click to close (optional)
 * - Animated backdrop and content
 *
 * Based on:
 * - Radix UI Dialog patterns
 * - WAI-ARIA Modal Dialog specification
 * - WCAG 2.1 Success Criterion 2.4.3 (Focus Order)
 */

import { createContext, useContext, useRef, type ReactNode, type RefObject } from 'react';
import { createPortal } from 'react-dom';
import { X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { modalClasses, zIndex, iconSizes, gapTokens } from '@/design/tokens';
import { useModal, useFocusTrap } from '@/hooks';

// =============================================================================
// Context
// =============================================================================

interface ModalContextValue {
  onClose: () => void;
  contentRef: RefObject<HTMLDivElement | null>;
}

const ModalContext = createContext<ModalContextValue | null>(null);

function useModalContext() {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error('Modal compound components must be used within Modal.Root');
  }
  return context;
}

// =============================================================================
// Root Component
// =============================================================================

export interface ModalRootProps {
  /** Whether modal is open */
  isOpen: boolean;
  /** Callback to close modal */
  onClose: () => void;
  /** Modal content */
  children: ReactNode;
  /** Close on outside click (default: true) */
  closeOnOutsideClick?: boolean;
  /** Close on escape key (default: true) */
  closeOnEscape?: boolean;
  /** Lock body scroll (default: true) */
  lockScroll?: boolean;
  /** Container class for positioning (default: centered) */
  containerClassName?: string;
}

function ModalRoot({
  isOpen,
  onClose,
  children,
  closeOnOutsideClick = true,
  closeOnEscape = true,
  lockScroll = true,
  containerClassName,
}: ModalRootProps) {
  const contentRef = useRef<HTMLDivElement>(null);

  // Unified modal hook: handles mounting, scroll lock, escape, outside click
  const { shouldRender, modalRef } = useModal(isOpen, onClose, {
    lockScroll,
    closeOnEscape,
    closeOnOutsideClick,
  });

  // Focus trap for accessibility
  useFocusTrap(contentRef, isOpen);

  if (!shouldRender) return null;

  return createPortal(
    <ModalContext.Provider value={{ onClose, contentRef }}>
      <div
        className={cn('fixed inset-0 flex items-center justify-center', containerClassName)}
        style={{ zIndex: zIndex.modal }}
        role="presentation"
      >
        {/* Backdrop - Raycast-style blur ramp */}
        <div
          className={cn(
            'fixed inset-0 bg-black/60 animate-overlay-backdrop',
          )}
          onClick={closeOnOutsideClick ? onClose : undefined}
          aria-hidden="true"
        />

        {/* Content container - spring entrance */}
        <div
          ref={(node) => {
            // Assign to both refs
            (modalRef as React.MutableRefObject<HTMLDivElement | null>).current = node;
            (contentRef as React.MutableRefObject<HTMLDivElement | null>).current = node;
          }}
          className="relative w-full px-4"
          role="dialog"
          aria-modal="true"
        >
          {children}
        </div>
      </div>
    </ModalContext.Provider>,
    document.body
  );
}

// =============================================================================
// Content Component
// =============================================================================

export interface ModalContentProps {
  children: ReactNode;
  className?: string;
  /** Width preset */
  size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | 'full';
  /** ARIA label for dialog */
  ariaLabel?: string;
  /** ARIA labelledby ID */
  ariaLabelledBy?: string;
}

const sizeClasses = {
  sm: 'max-w-sm',
  md: 'max-w-lg',
  lg: 'max-w-2xl',
  xl: 'max-w-4xl',
  '2xl': 'max-w-6xl',
  full: 'max-w-[95vw]',
};

function ModalContent({
  children,
  className,
  size = 'md',
  ariaLabel,
  ariaLabelledBy,
}: ModalContentProps) {
  return (
    <div
      className={cn(
        'w-full mx-auto my-4 overflow-hidden',
        modalClasses.content,
        sizeClasses[size],
        className
      )}
      aria-label={ariaLabel}
      aria-labelledby={ariaLabelledBy}
    >
      {children}
    </div>
  );
}

// =============================================================================
// Header Component
// =============================================================================

export interface ModalHeaderProps {
  children: ReactNode;
  className?: string;
  /** Show close button (default: true) */
  showClose?: boolean;
  /** Title ID for aria-labelledby */
  titleId?: string;
}

function ModalHeader({ children, className, showClose = true, titleId }: ModalHeaderProps) {
  const { onClose } = useModalContext();

  return (
    <div className={cn(modalClasses.header, className)}>
      <div id={titleId} className={cn('flex items-center min-w-0', gapTokens.spacious)}>
        {children}
      </div>
      {showClose && (
        <button
          onClick={onClose}
          aria-label="Close modal"
          className={modalClasses.closeButton}
        >
          <X className={iconSizes.md} />
        </button>
      )}
    </div>
  );
}

// =============================================================================
// Body Component
// =============================================================================

export interface ModalBodyProps {
  children: ReactNode;
  className?: string;
  /** Max height with scroll */
  maxHeight?: string;
}

function ModalBody({ children, className, maxHeight = 'calc(80vh - 160px)' }: ModalBodyProps) {
  return (
    <div
      className={cn(modalClasses.body, 'scrollbar-thin', className)}
      style={{ maxHeight }}
    >
      {children}
    </div>
  );
}

// =============================================================================
// Footer Component
// =============================================================================

export interface ModalFooterProps {
  children: ReactNode;
  className?: string;
}

function ModalFooter({ children, className }: ModalFooterProps) {
  return <div className={cn(modalClasses.footer, className)}>{children}</div>;
}

// =============================================================================
// Title Component (for consistent styling)
// =============================================================================

export interface ModalTitleProps {
  children: ReactNode;
  className?: string;
  id?: string;
  icon?: ReactNode;
}

function ModalTitle({ children, className, id, icon }: ModalTitleProps) {
  return (
    <div className={cn('flex items-center', gapTokens.spacious)}>
      {icon && (
        <div className="w-8 h-8 rounded-lg bg-primary/20 flex items-center justify-center shrink-0">
          {icon}
        </div>
      )}
      <h2 id={id} className={cn('text-sm font-semibold text-white', className)}>
        {children}
      </h2>
    </div>
  );
}

// =============================================================================
// Export Compound Component
// =============================================================================

export const Modal = {
  Root: ModalRoot,
  Content: ModalContent,
  Header: ModalHeader,
  Body: ModalBody,
  Footer: ModalFooter,
  Title: ModalTitle,
};
