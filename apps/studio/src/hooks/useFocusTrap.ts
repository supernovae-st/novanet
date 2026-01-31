'use client';

/**
 * useFocusTrap - Trap focus within a container (WCAG 2.1 AA requirement)
 *
 * Features:
 * - Tab key cycles through focusable elements
 * - Shift+Tab cycles backwards
 * - Auto-focus first focusable element on enable
 * - Restores focus to previously active element on disable
 * - Filters out hidden/disabled elements
 *
 * Based on:
 * - Radix UI Dialog focus management
 * - WAI-ARIA Modal Dialog pattern
 * - WCAG 2.1 Success Criterion 2.4.3 (Focus Order)
 */

import { useEffect, useRef, type RefObject } from 'react';

/** Selector for all focusable elements */
const FOCUSABLE_SELECTOR = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',');

/**
 * Get all visible focusable elements within a container
 */
function getFocusableElements(container: HTMLElement): HTMLElement[] {
  const elements = container.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR);

  return Array.from(elements).filter((el) => {
    // Filter out hidden elements (display: none, visibility: hidden, etc.)
    return el.offsetParent !== null && !el.hasAttribute('inert');
  });
}

/**
 * Trap focus within a container element
 *
 * @param containerRef - Ref to the container element
 * @param enabled - Whether focus trap is active
 *
 * @example
 * ```tsx
 * function Modal({ isOpen, children }) {
 *   const modalRef = useRef<HTMLDivElement>(null);
 *   useFocusTrap(modalRef, isOpen);
 *
 *   return (
 *     <div ref={modalRef} role="dialog" aria-modal="true">
 *       {children}
 *     </div>
 *   );
 * }
 * ```
 */
export function useFocusTrap(
  containerRef: RefObject<HTMLElement | null>,
  enabled: boolean
): void {
  // Store the element that was focused before the trap was enabled
  const previousActiveElementRef = useRef<HTMLElement | null>(null);

  useEffect(() => {
    if (!enabled || !containerRef.current) return;

    const container = containerRef.current;

    // Store currently focused element for restoration
    previousActiveElementRef.current = document.activeElement as HTMLElement;

    // Focus first focusable element
    const focusableElements = getFocusableElements(container);
    if (focusableElements.length > 0) {
      // Small delay to ensure modal is fully rendered
      requestAnimationFrame(() => {
        focusableElements[0]?.focus();
      });
    }

    // Handle Tab key to cycle focus within container
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Tab') return;

      const elements = getFocusableElements(container);
      if (elements.length === 0) return;

      const firstElement = elements[0];
      const lastElement = elements[elements.length - 1];
      const activeElement = document.activeElement;

      if (event.shiftKey) {
        // Shift+Tab: if on first element, jump to last
        if (activeElement === firstElement) {
          event.preventDefault();
          lastElement.focus();
        }
      } else {
        // Tab: if on last element, jump to first
        if (activeElement === lastElement) {
          event.preventDefault();
          firstElement.focus();
        }
      }
    };

    // Handle focus escaping the container (e.g., via click outside)
    const handleFocusIn = (event: FocusEvent) => {
      const target = event.target as HTMLElement;

      // If focus moved outside container, bring it back
      if (!container.contains(target)) {
        const elements = getFocusableElements(container);
        if (elements.length > 0) {
          elements[0].focus();
        }
      }
    };

    container.addEventListener('keydown', handleKeyDown);
    document.addEventListener('focusin', handleFocusIn);

    // Cleanup
    return () => {
      container.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('focusin', handleFocusIn);

      // Restore focus to previously active element
      if (previousActiveElementRef.current?.focus) {
        previousActiveElementRef.current.focus();
      }
    };
  }, [containerRef, enabled]);
}
